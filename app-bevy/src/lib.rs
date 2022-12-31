use bevy::prelude::*;

pub mod frame_display;
pub mod receiver;
pub mod ui;

pub const RGB_WIDTH: usize = 640;
pub const RGB_HEIGHT: usize = 480;
// TODO: use the smaller depth size that isn't interpolated?
pub const DEPTH_WIDTH: usize = 640;
pub const DEPTH_HEIGHT: usize = 480;

#[derive(Component, Reflect)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

pub fn app_main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Kinect Reprojection".to_string(),
                    width: (RGB_WIDTH as f32) * 2.0,
                    height: (RGB_HEIGHT as f32) + 65.0,
                    ..default()
                },
                ..default()
            }),
        )
        .add_plugin(bevy_egui::EguiPlugin)
        // .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(receiver::KinectReceiverPlugin)
        .add_plugin(frame_display::FrameDisplayPlugin)
        .add_startup_system(spawn_camera)
        .register_type::<MainCamera>()
        .run();
}
