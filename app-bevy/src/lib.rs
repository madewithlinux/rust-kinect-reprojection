use bevy::prelude::*;
use dock_ui::MainCamera;

pub mod dock_ui;
pub mod frame_display;
pub mod receiver;

pub const COLOR_WIDTH: usize = 640;
pub const COLOR_HEIGHT: usize = 480;
// TODO: use the smaller depth size that isn't interpolated?
pub const DEPTH_WIDTH: usize = 640;
pub const DEPTH_HEIGHT: usize = 480;

pub fn app_main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Kinect Reprojection".to_string(),
                width: (COLOR_WIDTH as f32) * 2.0,
                height: (COLOR_HEIGHT as f32) + 65.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(receiver::KinectReceiverPlugin)
        .add_plugin(frame_display::FrameDisplayPlugin)
        .add_plugin(dock_ui::AppUiDockPlugin)
        .register_type::<MainCamera>()
        .run();
}
