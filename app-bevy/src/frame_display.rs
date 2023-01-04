use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use image::RgbImage;
use kinect1::{depth_to_rgb_color, Gray16Image};

use crate::dock_ui::MainCamera;
use crate::receiver::{KinectFrameBufferName, KinectFrameBuffers};
use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Component, Reflect)]
pub struct KinectFrameBufferImageHandle(pub KinectFrameBufferName, pub Handle<Image>);

#[derive(Component, Debug, Reflect)]
pub struct SpritePosition {
    pub relative_scale: Vec3,
    pub relative_translation: Vec3,
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
        KinectFrameBufferImageHandle(KinectFrameBufferName::CurrentColor, color_image_handle.clone()),
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
        KinectFrameBufferImageHandle(KinectFrameBufferName::DerivedDepth, depth_image_handle.clone()),
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
        KinectFrameBufferImageHandle(KinectFrameBufferName::ActiveColor, color_image_subt_handle.clone()),
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
        KinectFrameBufferImageHandle(KinectFrameBufferName::CurrentPlayerIndex, depth_image_subt_handle.clone()),
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

fn update_framebuffer_images(
    buffers: Query<&KinectFrameBuffers>,
    frame_buffer_handle_query: Query<&KinectFrameBufferImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    let buffers = buffers.single();
    if buffers.derived_frame.depth_frame.len() == 0 {
        return;
    }

    for KinectFrameBufferImageHandle(buffer_name, handle) in frame_buffer_handle_query.iter() {
        if let Some(mut image) = images.get_mut(&handle) {
            image.data = buffers.get_buffer(*buffer_name);
        }
    }
}

pub struct FrameDisplayPlugin;
impl Plugin for FrameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_display_frames)
            .add_system(update_framebuffer_images)
            .add_system(update_sprite_transforms2)
            .register_type::<KinectFrameBufferImageHandle>()
            .register_type::<SpritePosition>()
            ;
    }
}
