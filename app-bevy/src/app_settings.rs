use std::io::Read;

use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{COLOR_HEIGHT, COLOR_WIDTH};

#[derive(Default, Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum UiMode {
    Game,
    #[default]
    Dock,
    Calibration,
}

#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Resource)]
#[serde(default)]
pub struct AppSettings {
    // window/system settings
    pub ui_mode: UiMode,
    pub window_title: String,
    pub window_width: f32,
    pub window_height: f32,
    pub window_resizable: bool,
    pub window_scale_factor_override: Option<f64>,
    pub min_total_threads: usize,
    pub max_total_threads: usize,
    pub greenscreen: bool,
    pub framerate_limit: Option<u32>,
    // enabled plugins
    pub kinect_enabled: bool,
    pub vr_input_enabled: bool,
    pub camera2_vmc_enabled: bool,
    /// deprecated
    pub history_buffer_size: usize,
    // individual plugin options
    pub fixed_delay_ms: i64,
    pub depth_texture_do_lookback: bool,
    pub depth_texture_do_lookahead: bool,
    pub depth_texture_always_use_player_index: bool,
    pub camera2_settings_folder: String,
    // debugging options
    pub show_debug_axes: bool,
    pub show_debug_entities: bool,
    pub show_debug_skeleton: bool,
    pub show_debug_coordinate_matchup: bool,
    pub kinect_static_frame: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        const DEFAULT_CAMERA2_SETTINGS_FOLDER: &str =
            r#"C:\Program Files (x86)\Steam\steamapps\common\Beat Saber\UserData\Camera2\"#;

        Self {
            ui_mode: default(),
            window_title: "Kinect Reprojection".to_string(),
            window_width: (COLOR_WIDTH as f32) * 2.0,
            window_height: (COLOR_HEIGHT as f32) + 400.0,
            window_resizable: true,
            window_scale_factor_override: None,
            min_total_threads: 0,
            max_total_threads: 0,
            greenscreen: false,
            framerate_limit: None,

            kinect_enabled: true,
            vr_input_enabled: true,
            camera2_vmc_enabled: true,

            history_buffer_size: 2,
            fixed_delay_ms: 500,
            depth_texture_do_lookback: true,
            depth_texture_do_lookahead: true,
            depth_texture_always_use_player_index: false,
            camera2_settings_folder: DEFAULT_CAMERA2_SETTINGS_FOLDER.to_string(),

            show_debug_axes: true,
            show_debug_entities: true,
            show_debug_skeleton: true,
            show_debug_coordinate_matchup: true,
            kinect_static_frame: None,
        }
    }
}

impl AppSettings {
    pub fn read_from_file(config_file_path: impl AsRef<std::path::Path>) -> Self {
        // TODO: create config file (and populate default values) if it doesn't exist
        let mut s = String::new();
        std::fs::File::open(config_file_path)
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        serde_json::from_str(&s).unwrap()
    }
}

pub struct AppSettingsPlugin {
    pub initial_settings: AppSettings,
}
impl Plugin for AppSettingsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(self.initial_settings.clone())
            .register_type::<AppSettings>()
            // .register_type()
            ;
    }
}

pub fn kinect_enabled(settings: Res<AppSettings>) -> bool {
    settings.kinect_enabled
}
pub fn use_kinect_static_frame(settings: Res<AppSettings>) -> bool {
    settings.kinect_static_frame.is_some()
}
pub fn vr_input_enabled(settings: Res<AppSettings>) -> bool {
    settings.vr_input_enabled
}
pub fn camera2_vmc_enabled(settings: Res<AppSettings>) -> bool {
    settings.camera2_vmc_enabled
}
pub fn debug_axes_enabled(settings: Res<AppSettings>) -> bool {
    settings.show_debug_axes
}
pub fn debug_entities_enabled(settings: Res<AppSettings>) -> bool {
    settings.show_debug_entities
}
pub fn debug_skeleton_enabled(settings: Res<AppSettings>) -> bool {
    settings.show_debug_skeleton
}
pub fn debug_coordinate_matchup_enabled(settings: Res<AppSettings>) -> bool {
    settings.show_debug_coordinate_matchup
}
