use bevy::prelude::*;
use renderdoc::{RenderDoc, V141, InputButton, V110};
use tracing::instrument;

pub struct RenderDocApiPlugin;
impl Plugin for RenderDocApiPlugin {
    fn build(&self, app: &mut App) {
        let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connect");
        rd.set_focus_toggle_keys(&[InputButton::F]);
        rd.set_capture_keys(&[InputButton::C]);
        app //
            .insert_non_send_resource(rd)
            .add_system(trigger_frame_capture);
    }
}

#[instrument(skip_all)]
fn trigger_frame_capture(mut rd: NonSendMut<RenderDoc<V110>>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::C) {
        info!("calling rd.trigger_capture()");
        rd.trigger_capture();
        match rd.get_capture(0) {
            Some((path, capture_time)) => println!("ID: 0, Path: {:?}, Captured: {:?}", path, capture_time),
            None => println!("No capture found with ID of 0!"),
        }
    }
    // // if keys.just_released(KeyCode::S) {
    // //     info!("calling rd.start_frame_capture()");
    // //     rd.start_frame_capture(std::ptr::null(), std::ptr::null());
    // // }
    // // if keys.just_released(KeyCode::E) {
    // //     info!("calling rd.end_frame_capture()");
    // //     rd.end_frame_capture(std::ptr::null(), std::ptr::null());
    // // }
}
