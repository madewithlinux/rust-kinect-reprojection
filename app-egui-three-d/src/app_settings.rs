use std::io::Read;
use serde::{Deserialize, Serialize};

use crate::util::default;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum UiMode {
    Game,
    #[default]
    Dock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub ui_mode: UiMode,
    // enabled plugins
    pub kinect_enabled: bool,
    pub vr_input_enabled: bool,
    pub camera2_vmc_enabled: bool,
    // individual plugin options
    pub history_buffer_size: usize,
    pub fixed_delay_ms: i64,
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
