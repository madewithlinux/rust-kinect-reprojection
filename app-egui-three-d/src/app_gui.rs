use std::{cell::RefCell, rc::Rc};

use three_d::{
    egui::{DragValue, Grid, Slider, Ui},
    *,
};

use crate::{
    app_settings::AppSettings, camera_orbit_control::CameraOrbitControl, debug_models::DebugModels,
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
        app_settings: &mut AppSettings,
        camera: &mut Camera,
        camera_control: &mut CameraOrbitControl,
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
                            app_settings,
                            camera,
                            camera_control,
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
    app_settings: &'a mut AppSettings,
    camera: &'a mut Camera,
    camera_control: &'a mut CameraOrbitControl,
    depth_model: &'a mut DepthModel,
    debug_models: &'a mut DebugModels,
    gui_context: &'a egui::Context,
    changed: &'a mut bool,
}

impl<'a> GuiWindowContent<'a> {
    pub fn main_settings_window(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.depth_model.angle, 0.01..=10.0).text("angle"));

        let mut orbit_drag_speed = self.camera_control.get_orbit_drag_speed();
        if ui
            .add(Slider::new(&mut orbit_drag_speed, 0.01..=0.5).text("orbit drag speed"))
            .changed()
        {
            self.camera_control.set_orbit_drag_speed(orbit_drag_speed);
        }

        ui.collapsing("camera info", |ui| self.camera_info(ui));
    }

    pub fn camera_info(&mut self, ui: &mut Ui) {
        Grid::new("camera grid").striped(true).show(ui, |ui| {
            ui.label("position");
            if let Some(position) = vec3_edit(ui, self.camera.position()) {
                self.camera.set_view(position, *self.camera.target(), *self.camera.up());
                *self.changed = true;
            }
            ui.end_row();

            ui.label("target");
            if let Some(target) = vec3_edit(ui, self.camera.target()) {
                self.camera.set_view(*self.camera.position(), target, *self.camera.up());
                *self.changed = true;
            }
            ui.end_row();

            ui.label("up");
            if let Some(up) = vec3_edit(ui, self.camera.up()) {
                self.camera.set_view(*self.camera.position(), *self.camera.target(), up);
                *self.changed = true;
            }
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
