use bevy::{math::Affine3A, prelude::*};
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, VertexPullingRenderPlugin, COLOR_MODE_RGB};
use bevy_prototype_debug_lines::*;
use bevy_render::primitives::Aabb;

use itertools::{iproduct, Itertools};
use kinect1::skeleton::{SkeletonPositionIndex, SkeletonPositionTrackingState, SkeletonTrackingState};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::{
    dock_ui::MainCamera,
    receiver::{KinectDepthTransformer, KinectFrameBuffers},
    util::draw_debug_axes,
    DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct PointCloudPlugin;
impl Plugin for PointCloudPlugin {
    fn build(&self, app: &mut App) {
        app //
            // .add_plugins(DefaultPlugins)
            .insert_resource(Msaa { samples: 1 })
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(VertexPullingRenderPlugin { outlines: false })
            .add_plugin(LookTransformPlugin)
            // .add_plugin(FpsCameraPlugin::default())
            .add_plugin(OrbitCameraPlugin::default())
            .add_startup_system(setup)
            // .add_system(update_scalar_hue_options)
            .add_system(update_cuboid_position_color)
            .add_system(skeleton_lines)
            .add_system(axis_references)
            .add_system(debug_coordinate_matchup)
            // .register_type::<BufferIndexes>()
            ;
    }
}

#[derive(Clone, Copy, Debug, Reflect)]
pub struct BufferIndex {
    pub x: u32,
    pub y: u32,
    pub flat_index: usize,
}

#[derive(Component, Debug, Default)]
pub struct BufferIndexes {
    pub indexes: Vec<BufferIndex>,
}

fn setup(mut commands: Commands, mut color_options_map: ResMut<ColorOptionsMap>) {
    let color_options_id = color_options_map.push(ColorOptions {
        color_mode: COLOR_MODE_RGB,
        wireframe: 0,
        scalar_hue: Default::default(),
    });

    const PATCH_SIZE: usize = 150;

    for y_batch in (0..DEPTH_HEIGHT).step_by(PATCH_SIZE) {
        for x_batch in (0..DEPTH_WIDTH).step_by(PATCH_SIZE) {
            let mut instances = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            let mut indexes = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            for y in y_batch..(y_batch + PATCH_SIZE).min(DEPTH_HEIGHT) {
                for x in x_batch..(x_batch + PATCH_SIZE).min(DEPTH_WIDTH) {
                    let flat_index = x + y * DEPTH_WIDTH;
                    indexes.push(BufferIndex {
                        x: x as u32,
                        y: y as u32,
                        flat_index,
                    });
                    let depth_jitter = (flat_index & 0xffff) as u16;
                    instances.push(Cuboid::new(Vec3::ZERO, Vec3::ONE, 0, false, depth_jitter));
                }
            }
            let cuboids = Cuboids::new(instances);
            let aabb = cuboids.aabb();
            commands
                .spawn(SpatialBundle::default())
                .insert((cuboids, aabb, color_options_id))
                .insert(BufferIndexes { indexes });
        }
    }

    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0.5, 3.6, 2.6),
            Vec3::new(0.0, 0.0, -0.8),
        ))
        .insert(MainCamera);
}

const POINT_WIDTH_FACTOR: f32 = 0.0014800;

fn update_cuboid_position_color(
    buffers: Res<KinectFrameBuffers>,
    mut cuboids_query: Query<(&BufferIndexes, &mut Cuboids, &mut Aabb)>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    let skeleton_points = &buffers.current_frame.skeleton_points;

    for (buffer_indexes, mut cuboids, mut aabb) in cuboids_query.iter_mut() {
        assert_eq!(buffer_indexes.indexes.len(), cuboids.instances.len());
        for (i, &BufferIndex { x, y, flat_index }) in buffer_indexes.indexes.iter().enumerate() {
            // set color
            let [r, g, b, _a] = buffers.current_frame.rgba[flat_index];
            cuboids.instances[i].color = u32::from_le_bytes([r, g, b, 255]);

            // set position
            let depth = buffers.derived_frame.depth[flat_index];
            if depth == 0 {
                cuboids.instances[i].minimum = Vec3::ZERO;
                cuboids.instances[i].maximum = Vec3::splat(1.0);
                cuboids.instances[i].make_invisible();
            } else {
                let (pixel_pos, point_width) = if depth_transformer.point_cloud_skel {
                    let skeleton_point = skeleton_points[flat_index];
                    let pixel_pos = depth_transformer.skeleton_point_to_world(skeleton_point);
                    let point_width = POINT_WIDTH_FACTOR * skeleton_point.z;
                    (pixel_pos, point_width)
                } else {
                    let pixel_pos = depth_transformer.index_depth_to_world(flat_index, depth);
                    let point_width = POINT_WIDTH_FACTOR * (depth as f32) / 1_000.0;
                    (pixel_pos, point_width)
                };

                // let point_cuboid_depth: f32 = point_width;
                let point_cuboid_depth: f32 =
                    adjacent_depth_difference(&buffers.derived_frame.depth, x as usize, y as usize, point_width, 0.1);
                let min = pixel_pos - Vec3::new(point_width, point_width, point_cuboid_depth);
                let max = pixel_pos + Vec3::new(point_width, point_width, 0.0);
                cuboids.instances[i].minimum = min;
                cuboids.instances[i].maximum = max;
                cuboids.instances[i].make_visible();
            }
        }
        *aabb = cuboids.aabb();
    }
}

fn adjacent_depth_difference(depth_frame: &Vec<u16>, x: usize, y: usize, min: f32, max: f32) -> f32 {
    let depth = depth_frame[x + y * DEPTH_WIDTH] as f32;
    let (min_depth, _max_depth) = {
        let x = x as i32;
        let y = y as i32;
        iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1))
            .filter(|&(i, j)| i >= 0 && i < (DEPTH_WIDTH as i32) && j >= 0 && j < (DEPTH_HEIGHT as i32))
            .map(|(i, j)| depth_frame[(i as usize) + (j as usize) * DEPTH_WIDTH] as f32)
            .filter(|&pd| pd > 0.0)
            .minmax()
            .into_option()
            .unwrap_or((depth, depth))
    };
    let adj_depth = depth - min_depth;
    if adj_depth < min {
        min
    } else if adj_depth > max {
        max
    } else {
        adj_depth
    }
}

fn skeleton_lines(
    buffers: Res<KinectFrameBuffers>,
    mut lines: ResMut<DebugLines>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    for &skeleton in buffers.current_frame.skeleton_frame.skeleton_data.iter() {
        if skeleton.tracking_state == SkeletonTrackingState::NotTracked {
            info!("skip skeleton");
            continue;
        }

        for bone in skeleton.get_skeleton_bones() {
            let Some((start_xyz, end_xyz)) = depth_transformer.skeleton_bone_to_world(&bone, &buffers.derived_frame.depth) else {
                continue;
            };
            let [start_color, end_color] = bone.map(|p| match p.tracking_state {
                SkeletonPositionTrackingState::NotTracked => Color::RED,
                SkeletonPositionTrackingState::Inferred => Color::YELLOW,
                SkeletonPositionTrackingState::Tracked => Color::WHITE,
            });
            let [start_color, end_color] = match bone[1].index {
                SkeletonPositionIndex::HandLeft => [Color::RED, Color::RED],
                SkeletonPositionIndex::HandRight => [Color::BLUE, Color::BLUE],
                _ => [start_color, end_color],
            };
            lines.line_gradient(start_xyz, end_xyz, 0.0, start_color, end_color);
        }
    }
}

fn axis_references(mut lines: ResMut<DebugLines>, dt: Res<KinectDepthTransformer>) {
    let scale = 1.0;
    lines.line_colored(Vec3::ZERO, Vec3::X * scale, 0.0, Color::RED);
    lines.line_colored(Vec3::ZERO, Vec3::Y * scale, 0.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO, Vec3::Z * scale, 0.0, Color::BLUE);

    // references for the kinect itself
    let start = dt.skeleton_point_to_world(Vec3::ZERO);
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::X * scale / 2.0),
        0.0,
        Color::RED,
    );
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::Y * scale / 2.0),
        0.0,
        Color::GREEN,
    );
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::Z * scale / 2.0),
        0.0,
        Color::BLUE,
    );
}

pub const REFERENCE_POINTS: [(Vec3, (usize, usize)); 3] = [
    (Vec3::new(1.2902215, -0.021568049, -0.59659046), (136, 346)),
    (Vec3::new(-1.4719752, 0.45078325, -0.96842957), (570, 278)),
    (Vec3::new(1.1872808, 1.5832841, -0.95948), (134, 89)),
];
pub const MORE_REFERENCE_POINTS: [(Vec3, Vec3); 5] = [
    (
        Vec3::new(-0.44902867, 1.2476542, 0.010715961),
        Vec3::new(0.6713257, 0.042388733, 2.3023503),
    ),
    (
        Vec3::new(-0.66511905, 1.476749, 0.38921714),
        Vec3::new(0.8464379, 0.036796886, 1.826863),
    ),
    (
        Vec3::new(0.39814186, 1.4179764, 0.61133754),
        Vec3::new(-0.29460278, -0.062339563, 1.6987133),
    ),
    (
        Vec3::new(0.61787796, 1.363985, -0.26557398),
        Vec3::new(-0.5433431, 0.36454648, 2.476277),
    ),
    (
        Vec3::new(-0.4848734, 0.6623882, -0.33309472),
        Vec3::new(0.691764, -0.29973462, 2.8916762),
    ),
];

fn debug_coordinate_matchup(
    buffers: Res<KinectFrameBuffers>,
    depth_transformer: Res<KinectDepthTransformer>,
    mut lines: ResMut<DebugLines>,
) {
    let depth = &buffers.current_frame.depth;
    let skeleton_points = &buffers.current_frame.skeleton_points;
    if skeleton_points.len() < 1 {
        return;
    }

    for (vr_point, kinect_image_point) in REFERENCE_POINTS.iter() {
        let vr_point = *vr_point;
        let flat_index = depth_transformer.ij_to_flat_index(kinect_image_point.0, kinect_image_point.1);
        let pixel_pos = if depth_transformer.point_cloud_skel {
            depth_transformer.skeleton_point_to_world(skeleton_points[flat_index])
        } else {
            depth_transformer.index_depth_to_world(flat_index, depth[flat_index])
        };

        lines.line_colored(vr_point, pixel_pos, 0.0, Color::YELLOW);
        draw_debug_axes(&mut lines, &Affine3A::from_translation(vr_point), 0.1);
    }

    for &(vr_point, sk_point) in MORE_REFERENCE_POINTS.iter() {
        let pixel_pos = depth_transformer.skeleton_point_to_world(sk_point);
        lines.line_colored(vr_point, pixel_pos, 0.0, Color::YELLOW);
        draw_debug_axes(&mut lines, &Affine3A::from_translation(vr_point), 0.1);
    }
}
