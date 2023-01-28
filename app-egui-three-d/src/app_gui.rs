use std::{cell::RefCell, rc::Rc};

use three_d::{
    egui::{DragValue, Grid, Ui},
    *,
};

use crate::{
    app_settings::AppSettings, camera_orbit_control::CameraOrbitControl, debug_models::DebugModels,
    depth_model::DepthModel,
};

pub struct AppGui {
    // TODO: maybe make a separate gui wrapper, so that we don't have to use `Rc<RefCell<>>`?
    pub gui: Rc<RefCell<three_d::GUI>>,
    pub enabled: bool,
}

impl AppGui {
    pub fn new(context: &Context) -> Self {
        Self {
            // gui: three_d::GUI::new(context),
            // gui: RefCell::new(three_d::GUI::new(context)),
            gui: Rc::new(RefCell::new(three_d::GUI::new(context))),
            enabled: true,
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
        if self.enabled {
            let gui = self.gui.clone();
            gui.borrow_mut().update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    use three_d::egui::*;
                    Window::new("window").show(gui_context, |ui| {
                        self.gui_update_inner(
                            ui,
                            app_settings,
                            camera,
                            camera_control,
                            depth_model,
                            debug_models,
                            gui_context,
                        );
                    });
                },
            );
        }
    }

    fn gui_update_inner(
        &mut self,
        ui: &mut Ui,
        app_settings: &mut AppSettings,
        camera: &mut Camera,
        camera_control: &mut CameraOrbitControl,
        depth_model: &mut DepthModel,
        debug_models: &mut DebugModels,
        gui_context: &egui::Context,
    ) {
        use three_d::egui::*;
        ui.add(Slider::new(&mut depth_model.angle, 0.01..=10.0).text("angle"));

        let mut orbit_drag_speed = camera_control.get_orbit_drag_speed();
        if ui
            .add(Slider::new(&mut orbit_drag_speed, 0.01..=0.5).text("orbit drag speed"))
            .changed()
        {
            camera_control.set_orbit_drag_speed(orbit_drag_speed);
        }

        ui.collapsing("camera info", |ui| camera_info(ui, camera));
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, _camera: &Camera) {
        if self.enabled {
            render_target.write(|| self.gui.borrow().render());
        }
    }
}

fn camera_info(ui: &mut Ui, camera: &mut Camera) {
    // TODO: also need to update the camera orbit controller, apparently
    Grid::new("camera grid").striped(true).show(ui, |ui| {
        ui.label("position");
        if let Some(position) = vec3_edit(ui, camera.position()) {
            camera.set_view(position, *camera.target(), *camera.up());
        }
        ui.end_row();

        ui.label("target");
        if let Some(target) = vec3_edit(ui, camera.target()) {
            camera.set_view(*camera.position(), target, *camera.up());
        }
        ui.end_row();

        ui.label("up");
        if let Some(up) = vec3_edit(ui, camera.up()) {
            camera.set_view(*camera.position(), *camera.target(), up);
        }
        ui.end_row();
    });
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
