use bevy::prelude::*;

use kinect1::{start_frame_thread, KinectFrameMessage, NUI_IMAGE_DEPTH_NO_VALUE};

#[derive(Debug)]
pub struct KinectReceiver(pub std::sync::mpsc::Receiver<KinectFrameMessage>);

#[derive(Component, Default)]
pub struct KinectCurrentFrame(pub KinectFrameMessage);

#[derive(Component, Default)]
pub struct KinectDerivedFrame(pub KinectFrameMessage);

#[derive(Component, Default, Debug, Reflect)]
pub struct KinectFrameHistorySize {
    pub buffer_size: usize,
}
#[derive(Component, Default)]
pub struct KinectFrameHistoryBuffer {
    pub history: std::collections::VecDeque<KinectFrameMessage>,
}

fn setup_kinect_receiver(world: &mut World) {
    let receiver = start_frame_thread();
    world.insert_non_send_resource(KinectReceiver(receiver));
    world.spawn((
        Name::new("frame and buffer"),
        KinectCurrentFrame::default(),
        KinectFrameHistorySize { buffer_size: 2 },
        KinectFrameHistoryBuffer {
            history: std::collections::VecDeque::with_capacity(2),
        },
        KinectDerivedFrame::default(),
    ));
}

fn receive_kinect_current_frame(
    receiver: NonSend<KinectReceiver>,
    mut current_frame_query: Query<(
        &mut KinectCurrentFrame,
        &KinectFrameHistorySize,
        &mut KinectFrameHistoryBuffer,
        &mut KinectDerivedFrame,
    )>,
) {
    if let Ok(received_frame) = receiver.0.try_recv() {
        let (mut current_frame, history_size, mut history_buf, mut derived_frame) = current_frame_query.single_mut();
        current_frame.0 = received_frame.clone();
        derived_frame.0 = received_frame.clone();
        for historic_frame in history_buf.history.iter() {
            for (i, depth) in historic_frame.depth_frame.iter().enumerate() {
                if depth == &NUI_IMAGE_DEPTH_NO_VALUE {
                    continue;
                }
                if derived_frame.0.depth_frame.get(i) == Some(&NUI_IMAGE_DEPTH_NO_VALUE) {
                    *derived_frame.0.depth_frame.get_mut(i).unwrap() = *depth;
                }
            }
        }
        while history_buf.history.len() > history_size.buffer_size {
            history_buf.history.pop_back();
        }
        history_buf.history.push_front(received_frame.clone());
    }
}

pub struct KinectReceiverPlugin;
impl Plugin for KinectReceiverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_kinect_receiver)
            .add_system(receive_kinect_current_frame)
            .register_type::<KinectFrameHistorySize>();
    }
}
