use image::{Rgb, RgbImage};
use kinect1::{depth_to_rgb_color, Gray16Image, NuiDepthPixelToPlayerIndex};

use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

pub fn color_frame_to_pixels(color_frame: &RgbImage) -> Vec<u8> {
    let mut color_pixels: Vec<u8> = Vec::with_capacity(COLOR_WIDTH * COLOR_HEIGHT * 4);
    for &pixel in color_frame.pixels() {
        color_pixels.push(pixel.0[0]);
        color_pixels.push(pixel.0[1]);
        color_pixels.push(pixel.0[2]);
        color_pixels.push(255); // alpha
    }
    color_pixels
}

pub fn depth_frame_to_pixels(depth_frame: &Gray16Image) -> Vec<u8> {
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

pub fn player_index_frame_to_pixels(depth_frame: &Gray16Image) -> Vec<u8> {
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
