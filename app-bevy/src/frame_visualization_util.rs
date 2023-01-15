use bevy::prelude::*;
use bytemuck::cast_slice_mut;

use crate::{receiver::KinectFrameBuffers, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Component, Default, Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameBufferDescriptor {
    #[default]
    Rgba,
    Depth,
    PlayerIndex,
    SkeletonPointCloud,
}

pub fn get_buffer(descriptor: &FrameBufferDescriptor, buffers: &KinectFrameBuffers, image_data: &mut [u8]) {
    let image_data = cast_slice_mut::<_, [u8; 4]>(image_data);
    match descriptor {
        FrameBufferDescriptor::Rgba => color_frame_to_pixels(&buffers.rgba, image_data),
        FrameBufferDescriptor::Depth => depth_frame_to_pixels(&buffers.depth, image_data),
        FrameBufferDescriptor::PlayerIndex => player_index_frame_to_pixels(&buffers.player_index, image_data),
        FrameBufferDescriptor::SkeletonPointCloud => {
            vec3_cloud_to_pixels(&buffers.skeleton_points, image_data, 1_000.0)
        }
    }
}

#[derive(Component, Reflect)]
pub struct FrameBufferImageHandle(pub FrameBufferDescriptor, pub Handle<Image>);

pub fn update_framebuffer_images(
    buffers: Res<KinectFrameBuffers>,
    frame_buffer_handle_query: Query<&FrameBufferImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    for FrameBufferImageHandle(buffer_name, handle) in frame_buffer_handle_query.iter() {
        if let Some(image) = images.get_mut(&handle) {
            get_buffer(buffer_name, &buffers, &mut image.data);
        }
    }
}

fn color_frame_to_pixels(color_frame: &[[u8; 4]], image_data: &mut [[u8; 4]]) {
    image_data.copy_from_slice(color_frame);
}

fn depth_frame_to_pixels(depth_frame: &[u16], image_data: &mut [[u8; 4]]) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH);
    for (i, &depth) in depth_frame.iter().enumerate() {
        let depth = depth << 3;
        image_data[i] = [(depth % 256) as u8, (depth / 256) as u8, 0, 255];
    }
}

fn vec3_cloud_to_pixels(point_cloud: &[Vec3], image_data: &mut [[u8; 4]], scale: f32) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH);
    for (i, v) in point_cloud.iter().enumerate() {
        image_data[i] = [
            ((v.x.abs() * scale) % 256.0) as u8,
            ((v.y.abs() * scale) % 256.0) as u8,
            ((v.z.abs() * scale) % 256.0) as u8,
            255,
        ];
    }
}

fn player_index_frame_to_pixels(player_index_frame: &[u8], image_data: &mut [[u8; 4]]) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH);
    let player_colors = &[
        [255, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 255, 255],
        [0, 255, 255, 255],
        [255, 255, 0, 255],
        [255, 0, 0, 255],
    ];
    for (i, &player_index) in player_index_frame.iter().enumerate() {
        image_data[i] = if player_index == 0 {
            [0, 0, 0, 255]
        } else {
            player_colors[(player_index as usize) % player_colors.len()]
        };
    }
}
