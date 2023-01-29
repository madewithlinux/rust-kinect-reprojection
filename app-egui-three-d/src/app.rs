use crate::app_gui::AppGui;
use crate::app_settings::{AppSettings, AppState};
use crate::camera_orbit_control::OrbitCamera;
use crate::debug_models::DebugModels;
use crate::depth_model::DepthModel;
use crate::util::default;
use three_d::*;


pub struct App {
    pub app_settings: AppSettings,
    pub orbit_camera: OrbitCamera,
    pub app_gui: AppGui,
    pub depth_model: DepthModel,
    pub debug_models: DebugModels,
    pub state: AppState,
}

impl App {
    pub fn new(window: &Window, context: &Context) -> Self {
        let state = AppState {
            camera_position: vec3(0.5, 3.6, 2.6),
            camera_target: vec3(0.0, 0.0, -0.8),
            camera_up: vec3(0.0, 1.0, 0.0),
            camera_fov_deg: 45.0,
            camera_z_near: 0.1,
            camera_z_far: 10.0,
            orbit_drag_speed: 0.05,
        };
        // Create a camera
        let orbit_camera = OrbitCamera::new(
            window.viewport(),
            state.camera_position,
            state.camera_target,
            state.camera_up,
            degrees(state.camera_fov_deg),
            state.camera_z_near,
            state.camera_z_far,
            0.1,
            100.0,
        );

        Self {
            app_settings: default(),
            orbit_camera,
            app_gui: AppGui::new(context),
            depth_model: DepthModel::new(context),
            debug_models: DebugModels::new(context),
            state,
        }
    }

    pub(crate) fn render_loop(&mut self, mut frame_input: FrameInput) -> FrameOutput {
        let frame_input = &mut frame_input;

        // write state to the state object that the gui edits
        if self.app_gui.enabled {
            self.orbit_camera.write_to_state(&mut self.state);
        }
        self.app_gui.gui_update(
            frame_input,
            &mut self.app_settings,
            &mut self.state,
            &mut self.orbit_camera,
            &mut self.depth_model,
            &mut self.debug_models,
        );
        if self.app_gui.changed {
            self.orbit_camera.read_from_state(&self.state);
        }

        self.depth_model.update_self(frame_input);
        self.orbit_camera.handle_events(&mut frame_input.events);

        DebugModels::update_app(self, frame_input);

        // set the viewport every frame so that it responds to window resizing
        self.orbit_camera.camera.set_viewport(frame_input.viewport);

        let render_target = &frame_input.screen();
        let camera = &self.orbit_camera.camera;
        render_target.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        self.depth_model.render(render_target, camera);
        self.debug_models.render(render_target, camera);
        // render gui last, so that it's always on top
        self.app_gui.render(render_target, camera);

        FrameOutput::default()
    }
}
