use array2d::Array2D;
use bevy::prelude::*;
use image::{Rgb, RgbImage};
use kinect1::{depth_to_rgb_color, Gray16Image, NuiDepthPixelToPlayerIndex};

use crate::{receiver::KinectFrameBuffers, COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Component, Default, Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameBufferDescriptor {
    #[default]
    CurrentColor,
    CurrentDepth,
    DerivedDepth,
    CurrentPlayerIndex,
    DerivedPlayerIndex,
    DepthBaseline,
    ActiveDepth,
    ActiveColor,
    PointCloud,
}

fn get_buffer(descriptor: &FrameBufferDescriptor, buffers: &KinectFrameBuffers, image_data: &mut [u8]) {
    match descriptor {
        FrameBufferDescriptor::CurrentColor => color_frame_to_pixels(&buffers.current_frame.color_frame, image_data),
        FrameBufferDescriptor::CurrentDepth => depth_frame_to_pixels(&buffers.current_frame.depth_frame, image_data),
        FrameBufferDescriptor::DerivedDepth => depth_frame_to_pixels(&buffers.derived_frame.depth_frame, image_data),
        FrameBufferDescriptor::DepthBaseline => depth_frame_to_pixels(&buffers.depth_baseline_frame, image_data),
        FrameBufferDescriptor::ActiveDepth => depth_frame_to_pixels(&buffers.active_depth, image_data),
        FrameBufferDescriptor::ActiveColor => color_frame_to_pixels(&buffers.active_color, image_data),
        FrameBufferDescriptor::CurrentPlayerIndex => {
            player_index_frame_to_pixels(&buffers.current_frame.depth_frame, image_data)
        }
        FrameBufferDescriptor::DerivedPlayerIndex => {
            player_index_frame_to_pixels(&buffers.derived_frame.depth_frame, image_data)
        }
        FrameBufferDescriptor::PointCloud => point_cloud_to_pixels(&buffers.point_cloud, image_data),
    }
}

#[derive(Component, Reflect)]
pub struct FrameBufferImageHandle(pub FrameBufferDescriptor, pub Handle<Image>);

pub fn update_framebuffer_images(
    buffers: Query<&KinectFrameBuffers>,
    frame_buffer_handle_query: Query<&FrameBufferImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    let buffers = buffers.single();
    if buffers.derived_frame.depth_frame.len() == 0 {
        return;
    }

    for FrameBufferImageHandle(buffer_name, handle) in frame_buffer_handle_query.iter() {
        if let Some(image) = images.get_mut(&handle) {
            // image.data = get_buffer(buffer_name, buffers);
            get_buffer(buffer_name, buffers, &mut image.data);
        }
    }
}

fn color_frame_to_pixels(color_frame: &RgbImage, image_data: &mut [u8]) {
    assert_eq!(image_data.len(), COLOR_HEIGHT * COLOR_WIDTH * 4);
    for (i, &pixel) in color_frame.pixels().enumerate() {
        image_data[4 * i + 0] = pixel.0[0];
        image_data[4 * i + 1] = pixel.0[1];
        image_data[4 * i + 2] = pixel.0[2];
        image_data[4 * i + 3] = 255;
    }
}

fn depth_frame_to_pixels(depth_frame: &Gray16Image, image_data: &mut [u8]) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH * 4);
    for (i, &depth) in depth_frame.iter().enumerate() {
        let Rgb([r, g, b]) = depth_to_rgb_color(depth);
        image_data[4 * i + 0] = r;
        image_data[4 * i + 1] = g;
        image_data[4 * i + 2] = b;
        image_data[4 * i + 3] = 255;
    }
}

fn point_cloud_to_pixels(point_cloud: &Array2D<Vec3>, image_data: &mut [u8]) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH * 4);
    for (i, v) in point_cloud.elements_row_major_iter().enumerate() {
        image_data[4 * i + 0] = (v.x.abs() % 256.0) as u8;
        image_data[4 * i + 1] = (v.y.abs() % 256.0) as u8;
        image_data[4 * i + 2] = (v.z.abs() % 256.0) as u8;
        image_data[4 * i + 3] = 255;
    }
}

fn player_index_frame_to_pixels(depth_frame: &Gray16Image, image_data: &mut [u8]) {
    assert_eq!(image_data.len(), DEPTH_HEIGHT * DEPTH_WIDTH * 4);
    let player_colors = &[
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (0, 255, 255),
        (255, 255, 0),
        (255, 0, 0),
    ];
    for (i, &depth) in depth_frame.iter().enumerate() {
        let player_index = NuiDepthPixelToPlayerIndex(depth);
        let (r, g, b) = if player_index == 0 {
            (0, 0, 0)
        } else {
            player_colors[(player_index as usize) % player_colors.len()]
        };
        image_data[4 * i + 0] = r;
        image_data[4 * i + 1] = g;
        image_data[4 * i + 2] = b;
        image_data[4 * i + 3] = 255;
    }
}
