use three_d::{Camera, FrameInput, RenderTarget};

use crate::app::App;

// TODO: this is maybe a bad idea, since it will hide when a function isn't being called?
pub(crate) trait PerFrameUpdate {
    fn update_self(&mut self, frame_input: &mut FrameInput);

    fn update_app(app: &mut App, frame_input: &mut FrameInput);

    #[allow(unused)]
    fn update_before_render(&mut self, frame_input: &mut FrameInput) {}

    fn render(&mut self, render_target: &RenderTarget, camera: &Camera);
}
