use app_settings::{AppSettings, UiMode};
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use itertools::Itertools;

pub mod app_settings;
pub mod camera2_vmc_osc_receiver;
pub mod debug_coordinates;
pub mod delay_buffer;
pub mod depth_texture;
pub mod dock_ui;
pub mod frame_visualization_util;
pub mod game_ui;
// pub mod point_cloud;
pub mod receiver;
mod util;
pub mod vr_connector;

pub const COLOR_WIDTH: usize = 640;
pub const COLOR_HEIGHT: usize = 480;
// TODO: use the smaller depth size that isn't interpolated?
pub const DEPTH_WIDTH: usize = 640;
pub const DEPTH_HEIGHT: usize = 480;

#[derive(Component, Reflect)]
pub struct MainCamera;

pub fn app_main() {
    let args = std::env::args().collect_vec();
    let app_settings_path = if args.len() == 2 {
        args[1].clone()
    } else {
        "app_settings.json".to_owned()
    };
    let settings = AppSettings::read_from_file(app_settings_path);

    let task_pool_options = {
        let mut task_pool_options = TaskPoolOptions::default();
        if settings.min_total_threads > 0 {
            task_pool_options.min_total_threads = settings.min_total_threads;
        }
        if settings.max_total_threads > 0 {
            task_pool_options.max_total_threads = settings.max_total_threads;
        }
        task_pool_options
    };

    let clear_color = if settings.greenscreen {
        ClearColor(Color::GREEN)
    } else {
        ClearColor::default()
    };

    let mut app = App::new();
    app //
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: settings.window_title.clone(),
                        width: settings.window_width,
                        height: settings.window_height,
                        resizable: settings.window_resizable,
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .insert_resource(task_pool_options)
        .insert_resource(clear_color)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(bevy_framepace::FramepacePlugin)
        // .add_plugin(app_settings::AppSettingsPlugin::new("app_settings.json"))
        .add_plugin(app_settings::AppSettingsPlugin {
            initial_settings: settings.clone(),
        });
    match settings.ui_mode {
        UiMode::Game => {
            app.add_plugin(game_ui::AppUiGamePlugin)
                .insert_resource(Msaa { samples: 1 });
        }
        UiMode::Dock => {
            app
                // third-party plugins
                // .add_plugin(DebugLinesPlugin::default())
                // app plugins
                .add_plugin(dock_ui::AppUiDockPlugin)
                // .add_plugin(frame_display::FrameDisplayPlugin)
                // .add_plugin(point_cloud::PointCloudPlugin)
                .add_plugin(debug_coordinates::DebugCoordinatesPlugin)
                .add_plugin(vr_connector::VrConnectorPlugin);
        }
    }
    app //
        .add_plugin(receiver::KinectReceiverPlugin)
        .add_plugin(depth_texture::DepthTexturePlugin)
        .add_plugin(camera2_vmc_osc_receiver::OscReceiverPlugin)
        .register_type::<MainCamera>()
        .register_type::<frame_visualization_util::FrameBufferImageHandle>()
        .register_type::<frame_visualization_util::FrameBufferDescriptor>()
        .run();
}
