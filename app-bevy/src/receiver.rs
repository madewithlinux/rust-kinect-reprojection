use bevy::prelude::*;

use image::{GenericImageView, Luma, Rgb, RgbImage};
use kinect1::{start_frame_thread, Gray16Image, KinectFrameMessage, NUI_IMAGE_DEPTH_NO_VALUE};

use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Debug)]
pub struct KinectReceiver(pub std::sync::mpsc::Receiver<KinectFrameMessage>);

#[derive(Component)]
pub struct KinectCurrentFrame(pub KinectFrameMessage);

#[derive(Component)]
pub struct KinectDerivedFrame(pub KinectFrameMessage);

#[derive(Component, Default, Debug, Reflect)]
pub struct KinectFrameHistorySize {
    pub buffer_size: usize,
}
#[derive(Component, Default)]
pub struct KinectFrameHistoryBuffer {
    pub history: std::collections::VecDeque<KinectFrameMessage>,
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

#[derive(Component)]
pub struct DepthBaselineFrame(pub Gray16Image);

#[derive(Component, Default, Debug, Reflect)]
pub struct DepthThreshold(pub f32);

#[derive(Component)]
pub struct ActiveDepth(pub Gray16Image);

#[derive(Component)]
pub struct ActiveColor(pub RgbImage);

// kinect_depth_data_empty.png
impl DepthBaselineFrame {
    pub fn load_from(path: impl AsRef<std::path::Path>) -> Self {
        Self(image::open(path).unwrap().into_luma16())
    }
}

fn subtract_depth(
    mut query: Query<
        (
            &KinectDerivedFrame,
            &DepthBaselineFrame,
            &DepthThreshold,
            &mut ActiveDepth,
            &mut ActiveColor,
        ),
        Changed<KinectCurrentFrame>,
    >,
) {
    let Ok((
        KinectDerivedFrame(derived_frame),
        DepthBaselineFrame(baseline_frame),
        DepthThreshold(depth_threshold),
        mut active_depth,
        mut active_color,
    )) = query.get_single_mut() else {
        return
    };

    let depth_threshold = *depth_threshold as u16;

    for (i, current_depth) in derived_frame.depth_frame.iter().enumerate() {
        let baseline_depth = baseline_frame.get(i).unwrap();
        *active_depth.0.get_mut(i).unwrap() = match (current_depth, baseline_depth) {
            (&NUI_IMAGE_DEPTH_NO_VALUE, _) => 0u16,
            (&value, &NUI_IMAGE_DEPTH_NO_VALUE) => value,
            // (&value, &baseline) if (baseline - value) < depth_threshold => value,
            (&value, &baseline) if (value + depth_threshold) < baseline => value,
            _ => 0u16,
        };
    }

    for (x, y, pixel) in (*active_color).0.enumerate_pixels_mut() {
        let depth = active_depth.0.get_pixel(x, y).0[0];
        *pixel = if depth != NUI_IMAGE_DEPTH_NO_VALUE {
            *derived_frame.color_frame.get_pixel(x, y)
        } else {
            Rgb([0, 0, 0])
        }
    }
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
        // DepthBaselineFrame::default(),
        DepthBaselineFrame::load_from("kinect_depth_data_empty.png"),
        DepthThreshold(5.0),
        ActiveDepth::default(),
        ActiveColor::default(),
    ));
}

pub struct KinectReceiverPlugin;
impl Plugin for KinectReceiverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_kinect_receiver)
            .add_system(receive_kinect_current_frame)
            .add_system(subtract_depth.after(receive_kinect_current_frame))
            .register_type::<KinectFrameHistorySize>()
            .register_type::<DepthThreshold>();
    }
}

impl Default for KinectCurrentFrame {
    fn default() -> Self {
        Self(KinectFrameMessage {
            color_frame: RgbImage::new(COLOR_WIDTH as u32, COLOR_HEIGHT as u32),
            depth_frame: Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32),
            ..Default::default()
        })
    }
}
impl Default for KinectDerivedFrame {
    fn default() -> Self {
        Self(KinectFrameMessage {
            color_frame: RgbImage::new(COLOR_WIDTH as u32, COLOR_HEIGHT as u32),
            depth_frame: Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32),
            ..Default::default()
        })
    }
}
impl Default for DepthBaselineFrame {
    fn default() -> Self {
        Self(Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32))
    }
}
impl Default for ActiveDepth {
    fn default() -> Self {
        Self(Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32))
    }
}
impl Default for ActiveColor {
    fn default() -> Self {
        Self(RgbImage::new(COLOR_WIDTH as u32, COLOR_HEIGHT as u32))
    }
}
