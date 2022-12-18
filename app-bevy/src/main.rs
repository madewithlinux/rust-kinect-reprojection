use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use array2d::Array2D;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};

use image::{Rgb, RgbImage};
use kinect1::{depth_to_rgb_color, get_sensor_count, start_frame_thread, Gray16Image, KinectFrameMessage};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Kinect {
    rgb_receiver: std::sync::mpsc::Receiver<KinectFrameMessage>,
}

#[derive(Component)]
struct CurrentRgb {
    rgb_data: RgbImage,
    depth_data: Gray16Image,
    rgb_handle: Handle<Image>,
    depth_handle: Handle<Image>,
}

#[derive(Component)]
struct MainCamera;

fn setup_kinect(world: &mut World) {
    let receiver = start_frame_thread();

    world.insert_non_send_resource(Kinect { rgb_receiver: receiver });
}

fn spawn_rgb(mut commands: Commands, mut images: ResMut<Assets<Image>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let rgb_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));
    let depth_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    ));

    commands.spawn_empty().insert(CurrentRgb {
        rgb_data: RgbImage::new(WIDTH as u32, HEIGHT as u32),
        depth_data: Gray16Image::new(WIDTH as u32, HEIGHT as u32),
        rgb_handle: rgb_image_handle.clone(),
        depth_handle: depth_image_handle.clone(),
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
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
                    // bevy logo (image)
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..default()
                        },
                        image: UiImage(rgb_image_handle),
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

fn read_rgb_data(kinect: NonSend<Kinect>, mut rgb_query: Query<&mut CurrentRgb>) {
    let rgb_res = rgb_query.get_single_mut();

    match rgb_res {
        Ok(mut rgb) => {
            if let Ok(frame) = kinect.rgb_receiver.try_recv() {
                rgb.rgb_data = frame.color_frame;
                rgb.depth_data = frame.depth_frame;
                return;
            } else {
                return;
            }
        }
        Err(_) => {}
    }
}

fn update_image_from_rgb_data(rgb_query: Query<&CurrentRgb>, mut images: ResMut<Assets<Image>>) {
    let rgb_res = rgb_query.get_single();

    match rgb_res {
        Ok(rgb) => {
            if rgb.rgb_data.len() == 0 {
                return;
            }
            if let Some(mut handle) = images.get_mut(&rgb.rgb_handle) {
                let mut new_pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);
                for pixel in rgb.rgb_data.pixels() {
                    new_pixels.push(pixel.0[0]);
                    new_pixels.push(pixel.0[1]);
                    new_pixels.push(pixel.0[2]);
                    new_pixels.push(255); // alpha
                }
                handle.data = new_pixels;
            }
            if let Some(mut handle) = images.get_mut(&rgb.depth_handle) {
                let mut new_pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);
                for &depth in rgb.depth_data.iter() {
                    let Rgb([r, g, b]) = depth_to_rgb_color(depth);
                    new_pixels.push(r);
                    new_pixels.push(g);
                    new_pixels.push(b);
                    // new_pixels.push((luma/255) as u8);
                    // new_pixels.push((luma/255) as u8);
                    // new_pixels.push((luma/255) as u8);
                    new_pixels.push(255); // alpha
                }
                handle.data = new_pixels;
            }
        }
        Err(_) => {}
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, kinect: NonSend<Kinect>) {
    if keys.just_pressed(KeyCode::Down) {
        println!("KeyCode::Down");
        // let tilt_degree = kinect.device.get_tilt_degree().unwrap();
        // // kinect.device.set_tilt_degree(tilt_degree - 5.0).unwrap();
    }

    if keys.just_pressed(KeyCode::Up) {
        println!("KeyCode::Up");
        // // let tilt_degree = kinect.device.get_tilt_degree().unwrap();
        // // kinect.device.set_tilt_degree(tilt_degree + 5.0).unwrap();
    }
}

fn main() -> Result<()> {
    App::new()
        .add_startup_system(setup_kinect)
        .add_startup_system(spawn_rgb)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Kinect".to_string(),
                width: 640.0 * 2.0,
                height: 480.0,
                ..default()
            },
            ..default()
        }))
        .add_system(read_rgb_data)
        .add_system(keyboard_input)
        .add_system(update_image_from_rgb_data)
        // .add_system(move_crosshair_to_pos)
        .run();

    Ok(())
}
