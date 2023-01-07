use std::f32::consts::PI;

use bevy::{math::Affine3A, prelude::*};
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, VertexPullingRenderPlugin, COLOR_MODE_RGB};
use bevy_render::primitives::Aabb;
use image::{Luma, Rgb};
use itertools::{iproduct, Itertools};
use kinect1::Gray16Image;
use kinect1::NuiDepthPixelToDepth;
use kinect1::NUI_DEPTH_DEPTH_UNKNOWN_VALUE;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::{
    dock_ui::MainCamera,
    receiver::{convert_depth_to_xyz, KinectFrameBuffers, KinectPostProcessorConfig},
    DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct PointCloudPlugin;
impl Plugin for PointCloudPlugin {
    fn build(&self, app: &mut App) {
        app //
            // .add_plugins(DefaultPlugins)
            .insert_resource(Msaa { samples: 1 })
            .add_plugin(VertexPullingRenderPlugin { outlines: false })
            .add_plugin(LookTransformPlugin)
            // .add_plugin(FpsCameraPlugin::default())
            .add_plugin(OrbitCameraPlugin::default())
            .add_startup_system(setup)
            // .add_system(update_scalar_hue_options)
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
    const SCENE_RADIUS: f32 = 1500.0;

    let mut current_flat_index = 0;
    for y_batch in (0..DEPTH_HEIGHT).step_by(PATCH_SIZE) {
        for x_batch in (0..DEPTH_WIDTH).step_by(PATCH_SIZE) {
            let mut instances = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            let mut indexes = Vec::with_capacity(PATCH_SIZE * PATCH_SIZE);
            for y in y_batch..(y_batch + PATCH_SIZE).min(DEPTH_HEIGHT) {
                for x in x_batch..(x_batch + PATCH_SIZE).min(DEPTH_WIDTH) {
                    indexes.push(BufferIndex {
                        x: x as u32,
                        y: y as u32,
                        // FIXME: this flat index is totally wrong
                        flat_index: current_flat_index,
                    });
                    current_flat_index += 1;

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
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::new(100.0, 0.0, 100.0),
        ))
        .insert(MainCamera);
}

// fn update_scalar_hue_options(time: Res<Time>, mut color_options_map: ResMut<ColorOptionsMap>) {
//     let options = color_options_map.get_mut(ColorOptionsId(1));
//     let tv = 1000.0 * (time.elapsed_seconds().sin() + 1.0);
//     // options.scalar_hue.max_visible = tv;
//     options.scalar_hue.clamp_max = tv;
// }

fn update_cuboid_position_color(
    // buffers: Query<&KinectFrameBuffers>,
    data_source_query: Query<(&KinectPostProcessorConfig, &KinectFrameBuffers)>,
    mut cuboids_query: Query<(&BufferIndexes, &mut Cuboids, &mut Aabb)>,
) {
    // let buffers = buffers.single();
    let (config, buffers) = data_source_query.single();

    // TODO: use a real correction factor instead of this
    let point_transform = Affine3A::from_rotation_x(config.sensor_tilt_angle_deg * PI / 180.0);

    for (buffer_indexes, mut cuboids, mut aabb) in cuboids_query.iter_mut() {
        assert_eq!(buffer_indexes.indexes.len(), cuboids.instances.len());
        for (i, &BufferIndex { x, y, flat_index }) in buffer_indexes.indexes.iter().enumerate() {
            // set color
            let &Rgb([r, g, b]) = buffers.current_frame.color_frame.get_pixel(x, y);
            cuboids.instances[i].color = u32::from_le_bytes([r, g, b, 255]);

            // set position
            // let pixel_pos = *buffers.point_cloud.get_row_major(flat_index).unwrap();
            // let pixel_pos = *buffers.point_cloud.get(y as usize, x as usize).unwrap();
            let &Luma([packed_depth]) = buffers.derived_frame.depth_frame.get_pixel(x, y);
            if packed_depth == NUI_DEPTH_DEPTH_UNKNOWN_VALUE {
                cuboids.instances[i].minimum = Vec3::ZERO;
                cuboids.instances[i].maximum = Vec3::splat(1.0);
            } else {
                let pixel_pos = convert_depth_to_xyz(
                    DEPTH_WIDTH as f32,
                    DEPTH_HEIGHT as f32,
                    x as f32,
                    (DEPTH_HEIGHT as u32 - y) as f32,
                    NuiDepthPixelToDepth(packed_depth) as f32,
                );
                let neighbor_pos = convert_depth_to_xyz(
                    DEPTH_WIDTH as f32,
                    DEPTH_HEIGHT as f32,
                    (x + 1) as f32,
                    (DEPTH_HEIGHT as u32 - y + 1) as f32,
                    NuiDepthPixelToDepth(packed_depth) as f32,
                );
                let pixel_pos = point_transform.transform_vector3(pixel_pos);
                let neighbor_pos = point_transform.transform_vector3(neighbor_pos);
                // const POINT_WIDTH: f32 = 1.0;
                let point_width = pixel_pos.distance(neighbor_pos) / 2.0;
                // let point_cuboid_depth: f32 = 50.0;
                let point_cuboid_depth: f32 = adjacent_depth_difference(&buffers.derived_frame.depth_frame, x, y);
                let min = pixel_pos - Vec3::new(point_width, point_width, point_cuboid_depth);
                let max = pixel_pos + Vec3::new(point_width, point_width, 0.0);
                cuboids.instances[i].minimum = min;
                cuboids.instances[i].maximum = max;
            }
        }
        *aabb = cuboids.aabb();
    }
}

fn adjacent_depth_difference(depth_frame: &Gray16Image, x: u32, y: u32) -> f32 {
    let &Luma([packed_depth]) = depth_frame.get_pixel(x, y);
    let depth = NuiDepthPixelToDepth(packed_depth) as f32;
    let (min_depth, _max_depth) = {
        let x = x as i32;
        let y = y as i32;
        iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1))
            .filter(|&(i, j)| i >= 0 && i < (DEPTH_WIDTH as i32) && j >= 0 && j < (DEPTH_HEIGHT as i32))
            .map(|(i, j)| depth_frame.get_pixel(i as u32, j as u32).0[0])
            .filter(|&pd| pd > 0)
            .filter(|&pd| pd != NUI_DEPTH_DEPTH_UNKNOWN_VALUE)
            .map(|pd| NuiDepthPixelToDepth(pd) as f32)
            .minmax()
            .into_option()
            .unwrap_or((depth, depth))
    };
    let adj_depth = depth - min_depth;
    if adj_depth < 5.0 {
        5.0
    } else if adj_depth > 100.0 {
        100.0
    } else {
        adj_depth
    }
}
