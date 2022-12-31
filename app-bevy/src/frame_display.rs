use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use kinect1::depth_to_rgb_color;

use crate::dock_ui::MainCamera;
use crate::receiver::{KinectCurrentFrame, KinectDerivedFrame};
use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Component, Reflect)]
pub struct KinectColorImageHandle(pub Handle<Image>);
#[derive(Component, Reflect)]
pub struct KinectDepthImageHandle(pub Handle<Image>);

#[derive(Component, Reflect)]
pub struct ColorImageSprite;
#[derive(Component, Reflect)]
pub struct DepthImageSprite;

fn update_sprite_transforms(
    cameras: Query<&mut Camera, With<MainCamera>>,
    mut color_transform: Query<&mut Transform, (With<ColorImageSprite>, Without<DepthImageSprite>)>,
    mut depth_transform: Query<&mut Transform, (With<DepthImageSprite>, Without<ColorImageSprite>)>,
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

    let mut color_transform = color_transform.single_mut();
    let mut depth_transform = depth_transform.single_mut();
    *color_transform = Transform::from_scale(Vec3::splat(viewport_width / (COLOR_WIDTH as f32) / 2.0))
        .with_translation(Vec3::new(-viewport_width / 4.0, 0.0, 0.0));
    *depth_transform = Transform::from_scale(Vec3::splat(viewport_width / (DEPTH_WIDTH as f32) / 2.0))
        .with_translation(Vec3::new(viewport_width / 4.0, 0.0, 0.0));
}

fn setup_display_frames(mut commands: Commands, mut images: ResMut<Assets<Image>>, _asset_server: Res<AssetServer>) {
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

    commands.spawn((
        Name::new("color image handle"),
        KinectColorImageHandle(color_image_handle.clone()),
    ));
    commands.spawn((
        Name::new("depth image handle"),
        KinectDepthImageHandle(depth_image_handle.clone()),
    ));

    commands.spawn((
        SpriteBundle {
            texture: color_image_handle,
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(0.5),
                Quat::default(),
                Vec3::new(-0.5, 0.0, 0.0),
            )),
            ..default()
        },
        Name::new("color image"),
        ColorImageSprite,
    ));

    commands.spawn((
        SpriteBundle {
            texture: depth_image_handle,
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(0.5),
                Quat::default(),
                Vec3::new(0.5, 0.0, 0.0),
            )),
            ..default()
        },
        Name::new("depth image"),
        DepthImageSprite,
    ));
}

fn update_color_image(
    current_frame: Query<&KinectCurrentFrame>,
    handle_query: Query<&KinectColorImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    let current_frame = current_frame.single();
    if current_frame.0.color_frame.len() == 0 {
        return;
    }

    let mut color_pixels: Vec<u8> = Vec::with_capacity(COLOR_WIDTH * COLOR_HEIGHT * 4);
    for pixel in current_frame.0.color_frame.pixels() {
        color_pixels.push(pixel.0[0]);
        color_pixels.push(pixel.0[1]);
        color_pixels.push(pixel.0[2]);
        color_pixels.push(255); // alpha
    }

    for color_handle in handle_query.iter() {
        if let Some(mut handle) = images.get_mut(&color_handle.0) {
            handle.data = color_pixels.clone();
        }
    }
}

fn update_depth_image(
    // current_frame: Query<&KinectCurrentFrame>,
    current_frame: Query<&KinectDerivedFrame>,
    handle_query: Query<&KinectDepthImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    let current_frame = current_frame.single();
    if current_frame.0.depth_frame.len() == 0 {
        return;
    }

    let mut depth_pixels: Vec<u8> = Vec::with_capacity(DEPTH_WIDTH * DEPTH_HEIGHT * 4);
    for &depth in current_frame.0.depth_frame.iter() {
        let image::Rgb([r, g, b]) = depth_to_rgb_color(depth);
        depth_pixels.push(r);
        depth_pixels.push(g);
        depth_pixels.push(b);
        depth_pixels.push(255); // alpha
    }

    for depth_handle in handle_query.iter() {
        if let Some(mut handle) = images.get_mut(&depth_handle.0) {
            handle.data = depth_pixels.clone();
        }
    }
}

pub struct FrameDisplayPlugin;
impl Plugin for FrameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_display_frames)
            .add_system(update_color_image)
            .add_system(update_depth_image)
            .add_system(update_sprite_transforms)
            .register_type::<KinectColorImageHandle>()
            .register_type::<KinectDepthImageHandle>();
    }
}
