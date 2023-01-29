use three_d::{
    egui::{DragValue, Grid, Slider, Ui},
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
            *self.changed |= vec3_edit_mut(ui, &mut self.state.camera_position);
            ui.end_row();

            ui.label("target");
            *self.changed |= vec3_edit_mut(ui, &mut self.state.camera_target);
            ui.end_row();

            ui.label("up");
            *self.changed |= vec3_edit_mut(ui, &mut self.state.camera_up);
            ui.end_row();

            ui.label("fov");
            *self.changed |= ui
                .add(DragValue::new(&mut self.state.camera_fov_deg).speed(DEFAULT_SPEED))
                .changed();
            ui.end_row();
        });
    }
}

pub const DEFAULT_SPEED: f32 = 0.1;

pub fn vec3_edit(ui: &mut Ui, input: &Vec3) -> Option<Vec3> {
    let mut value = *input;
    let mut changed = false;
    ui.columns(3, |uis| {
        changed |= uis[0].add(DragValue::new(&mut value.x).speed(DEFAULT_SPEED)).changed();
        changed |= uis[1].add(DragValue::new(&mut value.y).speed(DEFAULT_SPEED)).changed();
        changed |= uis[2].add(DragValue::new(&mut value.z).speed(DEFAULT_SPEED)).changed();
    });
    if changed {
        // dbg!("changed", input, value);
        Some(value)
    } else {
        None
    }
}

pub fn vec3_edit_mut(ui: &mut Ui, value: &mut Vec3) -> bool {
    ui.columns(3, |uis| {
        let mut changed = false;
        changed |= uis[0].add(DragValue::new(&mut value.x).speed(DEFAULT_SPEED)).changed();
        changed |= uis[1].add(DragValue::new(&mut value.y).speed(DEFAULT_SPEED)).changed();
        changed |= uis[2].add(DragValue::new(&mut value.z).speed(DEFAULT_SPEED)).changed();
        changed
    })
}
