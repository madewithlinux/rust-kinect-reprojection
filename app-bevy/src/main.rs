use anyhow::Result;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::tasks::AsyncComputeTaskPool;
use image::{Rgb, RgbImage};

use kinect1::{depth_to_rgb_color, start_frame_thread, Gray16Image, KinectFrameMessage};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Kinect {
    rgb_receiver: std::sync::mpsc::Receiver<KinectFrameMessage>,
}

#[derive(Component)]
struct CurrentRgb {
    rgb_data: RgbImage,
    depth_data: Gray16Image,
    // TODO: probably should split these out? and make the raw rgb/depth data a separate resource or something?
    rgb_handle: Handle<Image>,
    depth_handle: Handle<Image>,
}

#[derive(Component)]
struct MainCamera;

fn setup_kinect(world: &mut World) {
    let receiver = start_frame_thread();

    world.insert_non_send_resource(Kinect { rgb_receiver: receiver });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn spawn_rgb(mut commands: Commands, mut images: ResMut<Assets<Image>>, _asset_server: Res<AssetServer>) {
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
                    new_pixels.push(255); // alpha
                }
                handle.data = new_pixels;
            }
        }
        Err(_) => {}
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    time: Res<Time>,
    frame_count: Res<FrameCount>,
    // TODO: can't add this, because then the query won't find the button for some reason
    rgb_query: Query<&CurrentRgb>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    let current_rgb = rgb_query.get_single();
    let thread_pool = AsyncComputeTaskPool::get();

    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                println!("pressed {}", frame_count.0);
                // println!("pressed");

                if let Ok(current_rgb) = current_rgb {
                    let rgb_data = current_rgb.rgb_data.clone();
                    let depth_data = current_rgb.depth_data.clone();
                    let i = frame_count.0;
                    thread_pool
                        .spawn(async move {
                            println!("saving frames");
                            let rgb_filename = format!("kinect_rgb_data_{}.png", i);
                            rgb_data.save(rgb_filename).unwrap();
                            let depth_filename = format!("kinect_depth_data_{}.png", i);
                            depth_data.save(depth_filename).unwrap();
                            println!("saved frames");
                        })
                        .detach();
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn spawn_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // // center button
                // margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        // .insert(CurrentRgb {
        //     rgb_data: RgbImage::new(WIDTH as u32, HEIGHT as u32),
        //     depth_data: Gray16Image::new(WIDTH as u32, HEIGHT as u32),
        //     rgb_handle: Default::default(),
        //     depth_handle: Default::default(),
        // })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Down) {
        println!("KeyCode::Down");
    }

    if keys.just_pressed(KeyCode::Up) {
        println!("KeyCode::Up");
    }
}

fn main() -> Result<()> {
    App::new()
        .add_startup_system(setup_kinect)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_rgb)
        .add_startup_system(spawn_button)
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Bevy Kinect".to_string(),
                    width: 640.0 * 2.0,
                    height: 480.0 + 65.0,
                    ..default()
                },
                ..default()
            }), // .add(FrameCountPlugin),
        )
        .add_system(read_rgb_data)
        .add_system(keyboard_input)
        .add_system(update_image_from_rgb_data)
        .add_system(button_system)
        .run();

    Ok(())
}
