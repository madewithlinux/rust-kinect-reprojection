use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use dock_ui::MainCamera;

pub mod debug_coordinates;
pub mod delay_buffer;
pub mod depth_texture;
pub mod dock_ui;
pub mod frame_visualization_util;
pub mod point_cloud;
pub mod receiver;
mod util;
pub mod vr_connector;

pub const COLOR_WIDTH: usize = 640;
pub const COLOR_HEIGHT: usize = 480;
// TODO: use the smaller depth size that isn't interpolated?
pub const DEPTH_WIDTH: usize = 640;
pub const DEPTH_HEIGHT: usize = 480;

pub const FIXED_DELAY_MS: i64 = 500;

pub fn app_main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Kinect Reprojection".to_string(),
                width: (COLOR_WIDTH as f32) * 2.0,
                height: (COLOR_HEIGHT as f32) + 400.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(receiver::KinectReceiverPlugin)
        // .add_plugin(frame_display::FrameDisplayPlugin)
        // .add_plugin(point_cloud::PointCloudPlugin)
        .add_plugin(depth_texture::DepthTexturePlugin)
        .add_plugin(debug_coordinates::DebugCoordinatesPlugin)
        .add_plugin(dock_ui::AppUiDockPlugin)
        .add_plugin(vr_connector::VrConnectorPlugin)
        .register_type::<MainCamera>()
        .register_type::<frame_visualization_util::FrameBufferImageHandle>()
        .register_type::<frame_visualization_util::FrameBufferDescriptor>()
        .run();
}
