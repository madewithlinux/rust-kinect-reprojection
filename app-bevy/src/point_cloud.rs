use bevy::prelude::*;
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, VertexPullingRenderPlugin, COLOR_MODE_RGB};
use bevy_render::primitives::Aabb;

use itertools::{iproduct, Itertools};

use crate::{
    receiver::{KinectDepthTransformer, KinectFrameBuffers},
    DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct PointCloudPlugin;
impl Plugin for PointCloudPlugin {
    fn build(&self, app: &mut App) {
        app //
            // .add_plugins(DefaultPlugins)
            .insert_resource(Msaa { samples: 1 })
            // .add_plugin(DebugLinesPlugin::default())
            .add_plugin(VertexPullingRenderPlugin { outlines: false })
            // .add_plugin(LookTransformPlugin)
            // .add_plugin(OrbitCameraPlugin::default())
            .add_startup_system(setup)
            .add_system(update_cuboid_position_color)
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

    // commands
    //     .spawn(Camera3dBundle::default())
    //     .insert(OrbitCameraBundle::new(
    //         OrbitCameraController::default(),
    //         Vec3::new(0.5, 3.6, 2.6),
    //         Vec3::new(0.0, 0.0, -0.8),
    //     ))
    //     .insert(MainCamera);
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
