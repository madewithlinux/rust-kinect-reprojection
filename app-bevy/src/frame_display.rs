use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use image::RgbImage;
use kinect1::{depth_to_rgb_color, Gray16Image};

use crate::dock_ui::MainCamera;
use crate::receiver::{ActiveColor, ActiveDepth, DepthBaselineFrame, KinectCurrentFrame, KinectDerivedFrame};
use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Component, Reflect)]
pub struct KinectColorImageHandle(pub Handle<Image>);
#[derive(Component, Reflect)]
pub struct KinectDepthImageHandle(pub Handle<Image>);

#[derive(Component, Debug, Reflect)]
pub struct ColorImageSprite;
#[derive(Component, Debug, Reflect)]
pub struct DepthImageSprite;

#[derive(Component, Debug, Reflect)]
pub struct SpritePosition {
    pub relative_scale: Vec3,
    pub relative_translation: Vec3,
}

#[derive(Component, Debug, Reflect)]
pub enum ImageSource {
    Raw,
    Derived,
    BackgroundSubtracted,
    Baseline,
}

fn update_sprite_transforms2(
    cameras: Query<&mut Camera, With<MainCamera>>,
    mut sprites_query: Query<(&mut Transform, &SpritePosition)>,
    windows: Res<Windows>,
) {
    let window = windows.primary();
    let scale_factor = window.scale_factor() as f32;

    let cam = cameras.single();
    let physical_size = match &cam.viewport {
        Some(vp) => vp.physical_size,
        None => return,
    };
    let viewport_width = (physical_size.x as f32) / scale_factor;
    // let viewport_height = (physical_size.y as f32) / scale_factor;

    for (mut transform, sp) in sprites_query.iter_mut() {
        *transform = Transform::from_scale(viewport_width * sp.relative_scale)
            .with_translation(viewport_width * sp.relative_translation);
    }
}

fn setup_display_frames(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let color_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));
    let depth_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: DEPTH_WIDTH as u32,
            height: DEPTH_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    ));

    let color_image_subt_handle = images.add(Image::new_fill(
        Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));
    let depth_image_subt_handle = images.add(Image::new_fill(
        Extent3d {
            width: DEPTH_WIDTH as u32,
            height: DEPTH_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    ));

    commands.spawn((
        Name::new("color image handle"),
        KinectColorImageHandle(color_image_handle.clone()),
        ImageSource::Derived,
        SpriteBundle {
            texture: color_image_handle,
            ..default()
        },
        SpritePosition {
            relative_scale: Vec3::splat(1.0 / (COLOR_WIDTH as f32) / 2.0),
            relative_translation: Vec3::new(-1.0 / 4.0, 1.0 / 4.0, 0.0),
        },
    ));
    commands.spawn((
        Name::new("depth image handle"),
        KinectDepthImageHandle(depth_image_handle.clone()),
        ImageSource::Derived,
        SpriteBundle {
            texture: depth_image_handle,
            ..default()
        },
        SpritePosition {
            relative_scale: Vec3::splat(1.0 / (DEPTH_WIDTH as f32) / 2.0),
            relative_translation: Vec3::new(1.0 / 4.0, 1.0 / 4.0, 0.0),
        },
    ));
    commands.spawn((
        Name::new("color subt image handle"),
        KinectColorImageHandle(color_image_subt_handle.clone()),
        ImageSource::BackgroundSubtracted,
        SpriteBundle {
            texture: color_image_subt_handle,
            ..default()
        },
        SpritePosition {
            relative_scale: Vec3::splat(1.0 / (COLOR_WIDTH as f32) / 2.0),
            relative_translation: Vec3::new(-1.0 / 4.0, -1.0 / 4.0, 0.0),
        },
    ));
    commands.spawn((
        Name::new("depth subt image handle"),
        KinectDepthImageHandle(depth_image_subt_handle.clone()),
        ImageSource::BackgroundSubtracted,
        SpriteBundle {
            texture: depth_image_subt_handle,
            ..default()
        },
        SpritePosition {
            relative_scale: Vec3::splat(1.0 / (DEPTH_WIDTH as f32) / 2.0),
            relative_translation: Vec3::new(1.0 / 4.0, -1.0 / 4.0, 0.0),
        },
    ));
}

fn update_color_image2(
    current_frame: Query<&KinectCurrentFrame>,
    derived_frame: Query<&KinectDerivedFrame>,
    active_color: Query<&ActiveColor>,
    handle_query: Query<(&KinectColorImageHandle, &ImageSource)>,
    mut images: ResMut<Assets<Image>>,
) {
    let derived_frame = derived_frame.single();
    if derived_frame.0.depth_frame.len() == 0 {
        return;
    }

    for (color_handle, source) in handle_query.iter() {
        if let Some(mut handle) = images.get_mut(&color_handle.0) {
            handle.data = match source {
                ImageSource::Raw | ImageSource::Baseline => {
                    color_frame_to_pixels(&current_frame.single().0.color_frame)
                }
                ImageSource::Derived => color_frame_to_pixels(&derived_frame.0.color_frame),
                ImageSource::BackgroundSubtracted => color_frame_to_pixels(&active_color.single().0),
            };
        }
    }
}
fn update_depth_image2(
    current_frame: Query<&KinectCurrentFrame>,
    derived_frame: Query<&KinectDerivedFrame>,
    baseline_depth: Query<&DepthBaselineFrame>,
    active_depth: Query<&ActiveDepth>,
    handle_query: Query<(&KinectDepthImageHandle, &ImageSource)>,
    mut images: ResMut<Assets<Image>>,
) {
    let derived_frame = derived_frame.single();
    if derived_frame.0.depth_frame.len() == 0 {
        return;
    }

    for (depth_handle, source) in handle_query.iter() {
        if let Some(mut handle) = images.get_mut(&depth_handle.0) {
            handle.data = match source {
                ImageSource::Raw => depth_frame_to_pixels(&current_frame.single().0.depth_frame),
                ImageSource::Derived => depth_frame_to_pixels(&derived_frame.0.depth_frame),
                ImageSource::Baseline => depth_frame_to_pixels(&baseline_depth.single().0),
                ImageSource::BackgroundSubtracted => depth_frame_to_pixels(&active_depth.single().0),
            };
        }
    }
}

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
        let image::Rgb([r, g, b]) = depth_to_rgb_color(depth);
        depth_pixels.push(r);
        depth_pixels.push(g);
        depth_pixels.push(b);
        depth_pixels.push(255); // alpha
    }
    depth_pixels
}

pub struct FrameDisplayPlugin;
impl Plugin for FrameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_display_frames)
            // .add_system(update_color_image)
            .add_system(update_color_image2)
            .add_system(update_depth_image2)
            // .add_system(update_sprite_transforms)
            .add_system(update_sprite_transforms2)
            .register_type::<ImageSource>()
            .register_type::<KinectColorImageHandle>()
            .register_type::<KinectDepthImageHandle>();
    }
}
