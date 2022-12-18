use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use array2d::Array2D;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};

use image::RgbImage;
use kinect1::{get_sensor_count, start_frame_thread};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Kinect {
    rgb_receiver: std::sync::mpsc::Receiver<RgbImage>,
}

#[derive(Component)]
struct CurrentRgb {
    rgb_data: RgbImage,
    handle: Handle<Image>,
}

#[derive(Component)]
struct MainCamera;

fn setup_kinect(world: &mut World) {
    let receiver = start_frame_thread();

    world.insert_non_send_resource(Kinect { rgb_receiver: receiver });
}

fn spawn_rgb(mut commands: Commands, mut images: ResMut<Assets<Image>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let image_handle = images.add(Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));

    commands.spawn_empty().insert(CurrentRgb {
        rgb_data: RgbImage::new(WIDTH as u32, HEIGHT as u32),
        handle: image_handle.clone(),
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
                        image: UiImage(image_handle),
                        ..default()
                    });
                });
        });
}

fn read_rgb_data(kinect: NonSend<Kinect>, mut rgb_query: Query<&mut CurrentRgb>) {
    let rgb_res = rgb_query.get_single_mut();

    match rgb_res {
        Ok(mut rgb) => {
            if let Ok(rgb_data) = kinect.rgb_receiver.try_recv() {
                rgb.rgb_data = rgb_data;
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
            if let Some(mut handle) = images.get_mut(&rgb.handle) {
                let mut new_pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);

                // for [r,g,b] in rgb.rgb_data.chunks(3) {
                for pixel in rgb.rgb_data.pixels() {
                    new_pixels.push(pixel.0[0]);
                    new_pixels.push(pixel.0[1]);
                    new_pixels.push(pixel.0[2]);
                    new_pixels.push(255); // alpha

                    // new_pixels.push(0);
                    // new_pixels.push(0);
                    // new_pixels.push(0);
                    // new_pixels.push((measurement / 8) as u8);
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
                width: 640.,
                height: 480.,
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
