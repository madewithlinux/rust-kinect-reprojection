use std::io::Read;

use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum UiMode {
    Game,
    #[default]
    Dock,
}

#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Resource)]
#[serde(default)]
pub struct AppSettings {
    pub ui_mode: UiMode,
    // enabled plugins
    pub kinect_enabled: bool,
    pub vr_input_enabled: bool,
    pub camera2_vmc_enabled: bool,
    // individual plugin options
    /// deprecated
    pub history_buffer_size: usize,
    pub fixed_delay_ms: i64,
    pub depth_texture_do_lookback: bool,
    pub depth_texture_do_lookahead: bool,
    // debugging options
    pub show_debug_axes: bool,
    pub show_debug_entities: bool,
    pub show_debug_skeleton: bool,
    pub show_debug_coordinate_matchup: bool,
    pub kinect_static_frame: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ui_mode: default(),
            kinect_enabled: true,
            vr_input_enabled: true,
            camera2_vmc_enabled: true,
            history_buffer_size: 2,
            fixed_delay_ms: 500,
            depth_texture_do_lookback: true,
            depth_texture_do_lookahead: true,
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
