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

fn get_buffer(descriptor: &FrameBufferDescriptor, buffers: &KinectFrameBuffers) -> Vec<u8> {
    match descriptor {
        FrameBufferDescriptor::CurrentColor => color_frame_to_pixels(&buffers.current_frame.color_frame),
        FrameBufferDescriptor::CurrentDepth => depth_frame_to_pixels(&buffers.current_frame.depth_frame),
        FrameBufferDescriptor::DerivedDepth => depth_frame_to_pixels(&buffers.derived_frame.depth_frame),
        FrameBufferDescriptor::DepthBaseline => depth_frame_to_pixels(&buffers.depth_baseline_frame),
        FrameBufferDescriptor::ActiveDepth => depth_frame_to_pixels(&buffers.active_depth),
        FrameBufferDescriptor::ActiveColor => color_frame_to_pixels(&buffers.active_color),
        FrameBufferDescriptor::CurrentPlayerIndex => player_index_frame_to_pixels(&buffers.current_frame.depth_frame),
        FrameBufferDescriptor::DerivedPlayerIndex => player_index_frame_to_pixels(&buffers.derived_frame.depth_frame),
        FrameBufferDescriptor::PointCloud => buffers
            .point_cloud
            .elements_row_major_iter()
            .flat_map(|v| {
                [
                    (v.x.abs() % 256.0) as u8,
                    (v.y.abs() % 256.0) as u8,
                    (v.z.abs() % 256.0) as u8,
                    255u8,
                ]
            })
            .collect(),
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
        if let Some(mut image) = images.get_mut(&handle) {
            image.data = get_buffer(buffer_name, buffers);
        }
    }
}

fn color_frame_to_pixels(color_frame: &RgbImage) -> Vec<u8> {
    let mut color_pixels: Vec<u8> = Vec::with_capacity(COLOR_WIDTH * COLOR_HEIGHT * 4);
    for &pixel in color_frame.pixels() {
        color_pixels.push(pixel.0[0]);
        color_pixels.push(pixel.0[1]);
        color_pixels.push(pixel.0[2]);
        color_pixels.push(255); // alpha
    }
    color_pixels
}

fn depth_frame_to_pixels(depth_frame: &Gray16Image) -> Vec<u8> {
    let mut depth_pixels: Vec<u8> = Vec::with_capacity(DEPTH_WIDTH * DEPTH_HEIGHT * 4);
    for &depth in depth_frame.iter() {
        let Rgb([r, g, b]) = depth_to_rgb_color(depth);
        depth_pixels.push(r);
        depth_pixels.push(g);
        depth_pixels.push(b);
        depth_pixels.push(255); // alpha
    }
    depth_pixels
}

fn player_index_frame_to_pixels(depth_frame: &Gray16Image) -> Vec<u8> {
    let player_colors = &[
        Rgb([255, 0, 0]),
        Rgb([0, 255, 0]),
        Rgb([0, 0, 255]),
        Rgb([0, 255, 255]),
        Rgb([255, 255, 0]),
        Rgb([255, 0, 0]),
    ];
    let mut depth_pixels: Vec<u8> = Vec::with_capacity(DEPTH_WIDTH * DEPTH_HEIGHT * 4);
    for &depth in depth_frame.iter() {
        let player_index = NuiDepthPixelToPlayerIndex(depth);
        let Rgb([r, g, b]) = if player_index == 0 {
            Rgb([0, 0, 0])
        } else {
            player_colors[(player_index as usize) % player_colors.len()]
        };
        depth_pixels.push(r);
        depth_pixels.push(g);
        depth_pixels.push(b);
        depth_pixels.push(255); // alpha
    }
    depth_pixels
}
