use three_d::*;

use crate::per_frame_update::PerFrameUpdate;

pub struct AppGui {
    pub gui: three_d::GUI,
    pub enabled: bool,
}

impl AppGui {
    pub fn new(context: &Context) -> Self {
        Self {
            gui: three_d::GUI::new(context),
            enabled: true,
        }
    }
}

impl PerFrameUpdate for AppGui {
    fn update_self(&mut self, _frame_input: &mut FrameInput) {
    }

    fn update_app(app: &mut crate::app::App, frame_input: &mut FrameInput) {
        if app.app_gui.enabled {
            let gui = &mut app.app_gui.gui;
            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    use three_d::egui::*;
                    Window::new("window").show(gui_context, |ui| {
                        ui.add(Slider::new(&mut app.depth_model.angle, 0.01..=10.0).text("angle"));

                        let mut orbit_drag_speed = app.camera_control.get_orbit_drag_speed();
                        if ui
                            .add(Slider::new(&mut orbit_drag_speed, 0.01..=0.5).text("orbit drag speed"))
                            .changed()
                        {
                            app.camera_control.set_orbit_drag_speed(orbit_drag_speed);
                        }
                    });
                },
            );
        }
    }

    fn render(&mut self, render_target: &RenderTarget, _camera: &Camera) {
        if self.enabled {
            render_target.write(|| self.gui.render());
        }
    }
}
