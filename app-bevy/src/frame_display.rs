use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use kinect1::depth_to_rgb_color;

use crate::receiver::{KinectCurrentFrame, KinectDerivedFrame};
use crate::{DEPTH_HEIGHT, DEPTH_WIDTH, RGB_HEIGHT, RGB_WIDTH};

#[derive(Component, Reflect)]
pub struct KinectColorImageHandle(pub Handle<Image>);

#[derive(Component, Reflect)]
pub struct KinectDepthImageHandle(pub Handle<Image>);

fn setup_display_frames(mut commands: Commands, mut images: ResMut<Assets<Image>>, _asset_server: Res<AssetServer>) {
    let color_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: RGB_WIDTH as u32,
            height: RGB_HEIGHT as u32,
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

    commands
        .spawn((
            Name::new("UI"),
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..default()
                        },
                        image: UiImage(color_image_handle),
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..default()
                        },
                        image: UiImage(depth_image_handle),
                        ..default()
                    });
                });
        });
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

    let mut color_pixels: Vec<u8> = Vec::with_capacity(RGB_WIDTH * RGB_HEIGHT * 4);
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
            .register_type::<KinectColorImageHandle>()
            .register_type::<KinectDepthImageHandle>();
    }
}
