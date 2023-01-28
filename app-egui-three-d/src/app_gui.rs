use three_d::{
    egui::{DragValue, Grid, Ui},
    *,
};

pub struct AppGui {
    pub gui: three_d::GUI,
    pub enabled: bool,
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
}

impl AppGui {
    pub fn new(context: &Context) -> Self {
        Self {
            gui: three_d::GUI::new(context),
            enabled: true,
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }

    pub(crate) fn gui_update(app: &mut crate::app::App, frame_input: &mut FrameInput) {
        if app.app_gui.enabled {
            let gui = &mut app.app_gui.gui;

            let do_log = frame_input.events.len() > 0;
            if do_log {
                dbg!("before", &frame_input.events);
            }

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

                        ui.add(DragValue::new(&mut app.app_gui.a0));
                        ui.add(DragValue::new(&mut app.app_gui.a1));
                        ui.add(DragValue::new(&mut app.app_gui.a2));

                        ui.collapsing("camera info", |ui| camera_info(ui, &mut app.camera));

                    });
                },
            );

            if do_log {
                dbg!("after", &frame_input.events);
            }
        }
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, _camera: &Camera) {
        if self.enabled {
            render_target.write(|| self.gui.render());
        }
    }
}

fn camera_info(ui: &mut Ui, camera: &mut Camera) {
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
    // ui.label(format!("position: {:?}", camera.position()));
    // ui.label(format!("target: {:?}", camera.target()));
    // ui.label(format!("up: {:?}", camera.up()));

    // ui.label(format!("projection: {:?}", camera.projection()));
    // ui.label(format!("view: {:?}", camera.view()));
    // ui.label(format!("viewport: {:?}", camera.viewport()));
}

pub const DEFAULT_SPEED: f32 = 0.1;

pub fn vec3_edit(ui: &mut Ui, input: &Vec3) -> Option<Vec3> {
    let mut value = *input;
    let mut changed = false;
    ui.horizontal(|ui| {
        let changed0 = ui
            .add(
                DragValue::from_get_set(|v| {
                    if let Some(v) = v {
                        value.x = v as f32;
                    }
                    value.x as f64
                })
                .speed(DEFAULT_SPEED),
            )
            .changed();
        let changed1 = ui
            .add(
                DragValue::from_get_set(|v| {
                    if let Some(v) = v {
                        value.y = v as f32;
                    }
                    value.y as f64
                })
                .speed(DEFAULT_SPEED),
            )
            .changed();
        let changed2 = ui
            .add(
                DragValue::from_get_set(|v| {
                    if let Some(v) = v {
                        value.z = v as f32;
                    }
                    value.z as f64
                })
                .speed(DEFAULT_SPEED),
            )
            .changed();
        // let changed0 = ui.add(DragValue::new(&mut value.x).speed(DEFAULT_SPEED)).changed();
        // let changed1 = ui.add(DragValue::new(&mut value.y).speed(DEFAULT_SPEED)).changed();
        // let changed2 = ui.add(DragValue::new(&mut value.z).speed(DEFAULT_SPEED)).changed();
        changed = changed0 || changed1 || changed2
    });
    // ui.columns(3, |uis| {
    //     // uis[0].id
    //     let changed0 = uis[0].add(DragValue::new(&mut value.x).speed(DEFAULT_SPEED)).changed();
    //     let changed1 = uis[1].add(DragValue::new(&mut value.y).speed(DEFAULT_SPEED)).changed();
    //     let changed2 = uis[2].add(DragValue::new(&mut value.z).speed(DEFAULT_SPEED)).changed();
    //     changed = changed0 || changed1 || changed2
    // });
    if changed {
        dbg!("changed", input, value);
        Some(value)
    } else {
        None
    }
}
