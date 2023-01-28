use crate::app_gui::AppGui;
use crate::app_settings::AppSettings;
use crate::camera_orbit_control::CameraOrbitControl;
use crate::debug_models::DebugModels;
use crate::depth_model::DepthModel;
use crate::util::default;
use three_d::*;

pub struct App {
    pub app_settings: AppSettings,
    pub camera: Camera,
    pub camera_control: CameraOrbitControl,
    pub app_gui: AppGui,
    pub depth_model: DepthModel,
    pub debug_models: DebugModels,
}

impl App {
    pub fn new(window: &Window, context: &Context) -> Self {
        // Create a camera
        let camera = Camera::new_perspective(
            window.viewport(),
            // vec3(0.0, 0.0, 2.0),
            // vec3(0.0, 0.0, 0.0),
            // vec3(0.0, 1.0, 0.0),
            vec3(0.5, 3.6, 2.6),
            vec3(0.0, 0.0, -0.8),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );
        let mut camera_control = CameraOrbitControl::new(*camera.target(), 0.1, 100.0);
        camera_control.set_orbit_drag_speed(0.05);

        Self {
            app_settings: default(),
            camera,
            camera_control,
            app_gui: AppGui::new(context),
            depth_model: DepthModel::new(context),
            debug_models: DebugModels::new(context),
        }
    }

    pub(crate) fn render_loop(&mut self, mut frame_input: FrameInput) -> FrameOutput {
        let frame_input = &mut frame_input;

        // AppGui::gui_update(self, frame_input);
        self.app_gui.gui_update(
            frame_input,
            &mut self.app_settings,
            &mut self.camera,
            &mut self.camera_control,
            &mut self.depth_model,
            &mut self.debug_models,
        );

        self.depth_model.update_self(frame_input);
        self.camera_control
            .handle_events(&mut self.camera, &mut frame_input.events);

        DebugModels::update_app(self, frame_input);

        // set the viewport every frame so that it responds to window resizing
        self.camera.set_viewport(frame_input.viewport);

        let render_target = &frame_input.screen();
        let camera = &self.camera;
        render_target.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        self.depth_model.render(render_target, camera);
        self.debug_models.render(render_target, camera);
        // render gui last, so that it's always on top
        self.app_gui.render(render_target, camera);

        FrameOutput::default()
    }
}
