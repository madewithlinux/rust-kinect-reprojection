use three_d::*;

pub struct DebugModels {
    pub axes: Axes,
}

impl DebugModels {
    pub fn new(context: &Context) -> Self {
        Self {
            axes: Axes::new(&context, 0.05, 0.8),
        }
    }

    pub(crate) fn update_app(app: &mut crate::app::App, frame_input: &mut FrameInput) {
        let this = &mut app.debug_models;
        this.axes
            .set_transformation(Mat4::from_angle_y(radians((app.depth_model.angle) as f32)));
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, camera: &Camera) {
        render_target.render(&camera, &[&self.axes], &[]);
    }
}