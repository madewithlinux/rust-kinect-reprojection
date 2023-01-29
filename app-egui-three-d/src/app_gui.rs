use three_d::{
    egui::{DragValue, Grid, Response, Slider, Ui},
    *,
};

use crate::{
    app_settings::{AppSettings, AppState},
    camera_orbit_control::OrbitCamera,
    debug_models::DebugModels,
    depth_model::DepthModel,
};

pub struct AppGui {
    pub gui: three_d::GUI,
    pub enabled: bool,
    pub changed: bool,
}

impl AppGui {
    pub fn new(context: &Context) -> Self {
        Self {
            gui: three_d::GUI::new(context),
            enabled: true,
            changed: false,
        }
    }

    pub(crate) fn gui_update(
        &mut self,
        frame_input: &mut FrameInput,
        settings: &mut AppSettings,
        state: &mut AppState,
        orbit_camera: &mut OrbitCamera,
        depth_model: &mut DepthModel,
        debug_models: &mut DebugModels,
    ) {
        self.changed = false;
        if self.enabled {
            self.gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    use three_d::egui::*;
                    Window::new("window").show(gui_context, |ui| {
                        GuiWindowContent {
                            settings,
                            state,
                            orbit_camera,
                            depth_model,
                            debug_models,
                            gui_context,
                            changed: &mut self.changed,
                        }
                        .main_settings_window(ui);
                    });
                },
            );
        }
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, _camera: &Camera) {
        if self.enabled {
            render_target.write(|| self.gui.render());
        }
    }
}

struct GuiWindowContent<'a> {
    settings: &'a mut AppSettings,
    state: &'a mut AppState,
    orbit_camera: &'a mut OrbitCamera,
    depth_model: &'a mut DepthModel,
    debug_models: &'a mut DebugModels,
    gui_context: &'a egui::Context,
    changed: &'a mut bool,
}

impl<'a> GuiWindowContent<'a> {
    pub fn main_settings_window(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.depth_model.angle, 0.01..=10.0).text("angle"));

        *self.changed |= ui
            .add(Slider::new(&mut self.state.orbit_drag_speed, 0.01..=0.25).text("orbit drag speed"))
            .changed();

        ui.collapsing("camera info", |ui| self.camera_info(ui));
    }

    pub fn camera_info(&mut self, ui: &mut Ui) {
        Grid::new("camera grid").striped(true).show(ui, |ui| {
            ui.label("position");
            *self.changed |= self.state.camera_position.gui_edit(ui).changed();
            ui.end_row();

            ui.label("target");
            *self.changed |= self.state.camera_target.gui_edit(ui).changed();
            ui.end_row();

            ui.label("up");
            *self.changed |= self.state.camera_up.gui_edit(ui).changed();
            ui.end_row();

            ui.label("fov");
            *self.changed |= self.state.camera_fov_deg.gui_edit(ui).changed();
            ui.end_row();
        });
    }
}

pub const DEFAULT_SPEED: f32 = 0.1;

pub trait GuiEditable {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response;
}

impl GuiEditable for f32 {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self).speed(DEFAULT_SPEED))
    }
}

impl<S: GuiEditable + three_d::egui::emath::Numeric> GuiEditable for Vector3<S> {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response {
        ui.columns(3, |uis| {
            let mut response = self.x.gui_edit(&mut uis[0]);
            response |= self.y.gui_edit(&mut uis[1]);
            response |= self.z.gui_edit(&mut uis[2]);
            response
        })
    }
}
