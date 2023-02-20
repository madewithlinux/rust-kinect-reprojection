use std::time::Duration;

use app_settings::{AppSettings, UiMode};
use bevy::prelude::*;
use itertools::Itertools;

pub mod app_settings;
pub mod camera2_vmc_osc_receiver;
pub mod delay_buffer;
pub mod depth_model2;
pub mod depth_texture;
#[cfg(feature = "calibration")]
pub mod frame_visualization_util;
// pub mod point_cloud;
pub mod receiver;
mod util;

#[cfg(feature = "calibration")]
pub mod calibration_ui;
#[cfg(feature = "calibration")]
pub mod debug_coordinates;
#[cfg(feature = "calibration")]
pub mod dock_ui;
#[cfg(feature = "calibration")]
pub mod game_ui;
#[cfg(feature = "calibration")]
pub mod gui_common;
#[cfg(feature = "calibration")]
pub mod vr_connector;

#[cfg(feature = "renderdoc_api")]
pub mod renderdoc_api;

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
    #[cfg(feature = "calibration")]
    let watch_for_changes = true;
    #[cfg(not(feature = "calibration"))]
    let watch_for_changes = false;
    app //
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: settings.window_title.clone(),
                        width: settings.window_width,
                        height: settings.window_height,
                        resizable: settings.window_resizable,
                        scale_factor_override: settings.window_scale_factor_override,
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes,
                    ..default()
                }),
        )
        .insert_resource(task_pool_options)
        .insert_resource(clear_color)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(app_settings::AppSettingsPlugin {
            initial_settings: settings.clone(),
        });

    if let Some(framerate_limit) = settings.framerate_limit {
        app.insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::Manual(Duration::from_secs(1) / framerate_limit),
        });
    }

    #[cfg(feature = "debug_helpers")]
    app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default());

    match settings.ui_mode {
        UiMode::Game => {
            app.insert_resource(Msaa { samples: 1 })
                // .add_plugin(game_ui::AppUiGamePlugin)
                // we don't need the whole game UI, just a camera
                .add_startup_system(camera2_vmc_osc_receiver::spawn_3d_camera);
        }
        UiMode::Dock => {
            #[cfg(feature = "calibration")]
            app
                // app plugins
                .add_plugin(dock_ui::AppUiDockPlugin)
                // .add_plugin(point_cloud::PointCloudPlugin)
                .add_plugin(debug_coordinates::DebugCoordinatesPlugin)
                .add_plugin(vr_connector::VrConnectorPlugin);
            #[cfg(not(feature = "calibration"))]
            panic!("calibration/debug UI isn't enabled");
        }
        UiMode::Calibration => {
            #[cfg(feature = "calibration")]
            app
                // app plugins
                .add_plugin(calibration_ui::AppCalibrationUiPlugin)
                // .add_plugin(point_cloud::PointCloudPlugin)
                .add_plugin(debug_coordinates::DebugCoordinatesPlugin)
                .add_plugin(vr_connector::VrConnectorPlugin);
            #[cfg(not(feature = "calibration"))]
            panic!("calibration/debug UI isn't enabled");
        }
    }

    #[cfg(feature = "calibration")]
    app.register_type::<frame_visualization_util::FrameBufferImageHandle>()
        .register_type::<frame_visualization_util::FrameBufferDescriptor>();

    #[cfg(feature = "renderdoc_api")]
    app.add_plugin(renderdoc_api::RenderDocApiPlugin);

    app //
        .add_plugin(receiver::KinectReceiverPlugin)
        // .add_plugin(depth_texture::DepthTexturePlugin)
        .add_plugin(depth_model2::DepthModel2Plugin)
        .add_plugin(camera2_vmc_osc_receiver::OscReceiverPlugin)
        .register_type::<MainCamera>()
        .run();
}
