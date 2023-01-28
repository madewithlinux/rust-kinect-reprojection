pub mod app;
pub mod app_gui;
pub mod app_settings;
pub mod camera_orbit_control;
pub mod debug_models;
pub mod depth_model;
pub mod per_frame_update;
pub mod util;

use app::App;
use three_d::*;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    let mut app = App::new(&window, &context);

    window.render_loop(move |frame_input: FrameInput| app.render_loop(frame_input));
}
