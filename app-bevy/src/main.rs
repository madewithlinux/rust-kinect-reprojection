use std::collections::VecDeque;

use anyhow::Result;
use image::{Rgb, RgbImage};

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::tasks::AsyncComputeTaskPool;
// use bevy_inspector_egui::{prelude::*, DefaultInspectorConfigPlugin};
// use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use std::any::TypeId;

use kinect1::{depth_to_rgb_color, start_frame_thread, Gray16Image, KinectFrameMessage, NUI_IMAGE_DEPTH_NO_VALUE};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

#[derive(Debug)]
struct KinectReceiver(std::sync::mpsc::Receiver<KinectFrameMessage>);

#[derive(Component, Default)]
struct KinectCurrentFrame(KinectFrameMessage);

#[derive(Component, Default)]
struct KinectDerivedFrame(KinectFrameMessage);

#[derive(Component, Default, Debug, Reflect)]
struct KinectFrameHistorySize {
    pub buffer_size: usize,
}
#[derive(Component, Default)]
struct KinectFrameHistoryBuffer {
    pub history: VecDeque<KinectFrameMessage>,
}

#[derive(Component, Reflect)]
struct KinectColorImageHandle(Handle<Image>);

#[derive(Component, Reflect)]
struct KinectDepthImageHandle(Handle<Image>);

#[derive(Component, Reflect)]
struct MainCamera;

fn setup_kinect(world: &mut World) {
    let receiver = start_frame_thread();
    world.insert_non_send_resource(KinectReceiver(receiver));
    world.spawn((
        Name::new("frame and buffer"),
        KinectCurrentFrame::default(),
        KinectFrameHistorySize { buffer_size: 2 },
        KinectFrameHistoryBuffer {
            history: VecDeque::with_capacity(2),
        },
        KinectDerivedFrame::default(),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn spawn_rgb(mut commands: Commands, mut images: ResMut<Assets<Image>>, _asset_server: Res<AssetServer>) {
    let color_image_handle = images.add(Image::new_fill(
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

fn receive_kinect_current_frame(
    receiver: NonSend<KinectReceiver>,
    mut current_frame_query: Query<(
        &mut KinectCurrentFrame,
        &KinectFrameHistorySize,
        &mut KinectFrameHistoryBuffer,
        &mut KinectDerivedFrame,
    )>,
) {
    if let Ok(received_frame) = receiver.0.try_recv() {
        let (mut current_frame, history_size, mut history_buf, mut derived_frame) = current_frame_query.single_mut();
        // info!("history_size={:?}", history_size);
        current_frame.0 = received_frame.clone();
        derived_frame.0 = received_frame.clone();
        for historic_frame in history_buf.history.iter() {
            for (i, depth) in historic_frame.depth_frame.iter().enumerate() {
                if depth == &NUI_IMAGE_DEPTH_NO_VALUE {
                    continue;
                }
                if derived_frame.0.depth_frame.get(i) == Some(&NUI_IMAGE_DEPTH_NO_VALUE) {
                    *derived_frame.0.depth_frame.get_mut(i).unwrap() = *depth;
                }
            }
        }
        while history_buf.history.len() > history_size.buffer_size {
            history_buf.history.pop_back();
        }
        history_buf.history.push_front(received_frame.clone());
    }
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

    let mut color_pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);
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

    let mut depth_pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);
    for &depth in current_frame.0.depth_frame.iter() {
        let Rgb([r, g, b]) = depth_to_rgb_color(depth);
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    // time: Res<Time>,
    frame_count: Res<FrameCount>,
    current_frame: Query<&KinectCurrentFrame>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    let current_frame = current_frame.single();
    let thread_pool = AsyncComputeTaskPool::get();

    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                println!("pressed {}", frame_count.0);
                // println!("pressed");

                let color_frame = current_frame.0.color_frame.clone();
                let depth_frame = current_frame.0.depth_frame.clone();
                let i = frame_count.0;
                thread_pool
                    .spawn(async move {
                        println!("saving frames");
                        let rgb_filename = format!("kinect_rgb_data_{}.png", i);
                        color_frame.save(rgb_filename).unwrap();
                        let depth_filename = format!("kinect_depth_data_{}.png", i);
                        depth_frame.save(depth_filename).unwrap();
                        println!("saved frames");
                    })
                    .detach();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Save Frame Data",
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
        .add_plugin(bevy_egui::EguiPlugin)
        // .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin) // adds default options and `InspectorEguiImpl`s
        // startup systems
        .add_startup_system(setup_kinect)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_rgb)
        .add_startup_system(spawn_button)
        // other systems
        .add_system(receive_kinect_current_frame)
        .add_system(keyboard_input)
        .add_system(update_color_image)
        .add_system(update_depth_image)
        .add_system(button_system)
        // types
        .register_type::<KinectFrameHistorySize>()
        .register_type::<KinectColorImageHandle>()
        .register_type::<KinectDepthImageHandle>()
        .register_type::<MainCamera>()
        .run();

    Ok(())
}
