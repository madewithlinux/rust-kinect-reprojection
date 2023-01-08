use std::f32::consts::PI;

use bevy::{math::Affine3A, prelude::*};
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, VertexPullingRenderPlugin, COLOR_MODE_RGB};
use bevy_prototype_debug_lines::*;
use bevy_render::primitives::Aabb;

use itertools::{iproduct, Itertools};
use kinect1::skeleton::{SkeletonPositionTrackingState, SkeletonTrackingState};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::{
    dock_ui::MainCamera,
    receiver::{KinectDepthTransformer, KinectFrameBuffers, KinectPostProcessorConfig},
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
    const SCENE_RADIUS: f32 = 1500.0;

    for y_batch in (0..DEPTH_HEIGHT).step_by(PATCH_SIZE) {
        for x_batch in (0..DEPTH_WIDTH).step_by(PATCH_SIZE) {
            let mut instances = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            let mut indexes = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            for y in y_batch..(y_batch + PATCH_SIZE).min(DEPTH_HEIGHT) {
                for x in x_batch..(x_batch + PATCH_SIZE).min(DEPTH_WIDTH) {
                    indexes.push(BufferIndex {
                        x: x as u32,
                        y: y as u32,
                        flat_index: x + y * DEPTH_WIDTH,
                    });

                    let x = x as f32 - SCENE_RADIUS;
                    let z = y as f32 - SCENE_RADIUS;
                    let d = (x * x + z * z).sqrt();
                    let amp = 0.2 * d;
                    let y = amp * ((0.05 * x).cos() * (0.05 * z).sin());
                    let c = Vec3::new(x, y, z);
                    let h = 0.01 * d;
                    let min = c - Vec3::new(0.5, h, 0.5);
                    let max = c + Vec3::new(0.5, h, 0.5);
                    let visible = true;
                    let depth_jitter = 0;
                    let scalar_color = u32::from_le_bytes(d.to_le_bytes());

                    instances.push(Cuboid::new(min, max, scalar_color, visible, depth_jitter));
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
            OrbitCameraController {
                mouse_translate_sensitivity: Vec2::splat(100.0),
                ..Default::default()
            },
            Vec3::new(1.0, 1.0, 1.0) * 3_000.0,
            Vec3::new(1.0, 0.0, 1.0) * 1_000.0,
        ))
        .insert(MainCamera);
}

fn update_cuboid_position_color(
    // buffers: Query<&KinectFrameBuffers>,
    data_source_query: Query<(&KinectPostProcessorConfig, &KinectFrameBuffers)>,
    mut cuboids_query: Query<(&BufferIndexes, &mut Cuboids, &mut Aabb)>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    // let buffers = buffers.single();
    let (config, buffers) = data_source_query.single();

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
            } else {
                let pixel_pos = depth_transformer.coordinate_depth_to_xyz(x as usize, y as usize, depth);
                let neighbor_pos = depth_transformer.coordinate_depth_to_xyz((x + 1) as usize, (y + 1) as usize, depth);
                let point_width = pixel_pos.distance(neighbor_pos) / 2.0;
                // let point_cuboid_depth: f32 = 50.0;
                // let point_cuboid_depth: f32 = point_width;
                let point_cuboid_depth: f32 =
                    adjacent_depth_difference(&buffers.derived_frame.depth, x as usize, y as usize, point_width, 250.0);
                let min = pixel_pos - Vec3::new(point_width, point_width, point_cuboid_depth);
                let max = pixel_pos + Vec3::new(point_width, point_width, 0.0);
                cuboids.instances[i].minimum = min;
                cuboids.instances[i].maximum = max;
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
    data_source_query: Query<(&KinectPostProcessorConfig, &KinectFrameBuffers)>,
    mut lines: ResMut<DebugLines>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    let (config, buffers) = data_source_query.single();

    for &skeleton in buffers.current_frame.skeleton_frame.skeleton_data.iter() {
        if skeleton.tracking_state == SkeletonTrackingState::NotTracked {
            info!("skip skeleton");
            continue;
        }

        for bone in skeleton.get_skeleton_bones() {
            let Some((start_xyz, end_xyz)) = depth_transformer.skeleton_bone_to_xyz(&bone, &buffers.derived_frame.depth) else {
                continue;
            };
            let [start_color, end_color] = bone.map(|p| match p.tracking_state {
                SkeletonPositionTrackingState::NotTracked => Color::RED,
                SkeletonPositionTrackingState::Inferred => Color::YELLOW,
                SkeletonPositionTrackingState::Tracked => Color::WHITE,
            });
            lines.line_gradient(start_xyz, end_xyz, 0.0, start_color, end_color);
        }
    }
}

fn axis_references(mut lines: ResMut<DebugLines>, dt: Res<KinectDepthTransformer>) {
    let scale = 1_000.0;
    lines.line_colored(Vec3::ZERO, Vec3::X * scale, 0.0, Color::RED);
    lines.line_colored(Vec3::ZERO, Vec3::Y * scale, 0.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO, Vec3::Z * scale, 0.0, Color::BLUE);

    // references for the kinect itself
    lines.line_colored(
        dt.point_transform_matrix_inverse.transform_point3(Vec3::ZERO),
        dt.point_transform_matrix_inverse
            .transform_point3(Vec3::X * scale / 2.0),
        0.0,
        Color::RED,
    );
    lines.line_colored(
        dt.point_transform_matrix_inverse.transform_point3(Vec3::ZERO),
        dt.point_transform_matrix_inverse
            .transform_point3(Vec3::Y * scale / 2.0),
        0.0,
        Color::GREEN,
    );
    lines.line_colored(
        dt.point_transform_matrix_inverse.transform_point3(Vec3::ZERO),
        dt.point_transform_matrix_inverse
            .transform_point3(Vec3::Z * scale / 2.0),
        0.0,
        Color::BLUE,
    );
}
