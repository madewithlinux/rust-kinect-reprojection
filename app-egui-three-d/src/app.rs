use crate::app_gui::AppGui;
use crate::app_settings::AppSettings;
use crate::camera_orbit_control::CameraOrbitControl;
use crate::debug_models::DebugModels;
use crate::depth_model::DepthModel;
use crate::per_frame_update::PerFrameUpdate;
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

impl PerFrameUpdate for App {
    fn update_self(&mut self, frame_input: &mut FrameInput) {
        self.app_gui.update_self(frame_input);
        self.depth_model.update_self(frame_input);
        self.debug_models.update_self(frame_input);
        self.camera_control
            .handle_events(&mut self.camera, &mut frame_input.events);
    }

    fn update_app(app: &mut crate::app::App, frame_input: &mut FrameInput) {
        AppGui::update_app(app, frame_input);
        DepthModel::update_app(app, frame_input);
        DebugModels::update_app(app, frame_input);
    }

    fn update_before_render(&mut self, frame_input: &mut FrameInput) {
        // TODO: do I really need to set the camera viewport every frame?
        self.camera.set_viewport(frame_input.viewport);
        self.app_gui.update_before_render(frame_input);
        self.depth_model.update_before_render(frame_input);
        self.debug_models.update_before_render(frame_input);
    }

    fn render(&mut self, render_target: &RenderTarget, camera: &Camera) {
        // render_target.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        // self.depth_model.render(render_target, camera);
        // self.debug_models.render(render_target, camera);
        // // render gui last, so that it's always on top
        // self.app_gui.render(render_target, camera);
    }
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
        self.update_self(&mut frame_input);
        App::update_app(self, &mut frame_input);
        self.update_before_render(&mut frame_input);



        let render_target = &frame_input.screen();
        let camera = &self.camera;
        // we can't call self.render because it would double borrow the camera
        // self.render(render_target, camera);

        render_target.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        self.depth_model.render(render_target, camera);
        self.debug_models.render(render_target, camera);
        // render gui last, so that it's always on top
        self.app_gui.render(render_target, camera);

        FrameOutput::default()
    }
}
