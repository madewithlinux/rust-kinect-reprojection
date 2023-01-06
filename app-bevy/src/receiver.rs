use bevy::prelude::*;

use image::{Rgb, RgbImage};
use kinect1::{start_frame_thread, FrameMessageReceiver, Gray16Image, KinectFrameMessage, NUI_IMAGE_DEPTH_NO_VALUE};

use crate::{
    frame_visualization_util::{color_frame_to_pixels, depth_frame_to_pixels, player_index_frame_to_pixels},
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH,
};

#[derive(Debug)]
pub struct KinectReceiver(pub FrameMessageReceiver);

#[derive(Component, Debug, Reflect)]
pub struct KinectPostProcessorConfig {
    pub history_buffer_size: usize,
    pub depth_threshold: f32,
}
impl Default for KinectPostProcessorConfig {
    fn default() -> Self {
        Self {
            history_buffer_size: 2,
            depth_threshold: 100.0,
        }
    }
}

#[derive(Component, Default, Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KinectFrameBufferName {
    #[default]
    CurrentColor,
    CurrentDepth,
    DerivedDepth,
    CurrentPlayerIndex,
    DerivedPlayerIndex,
    DepthBaseline,
    ActiveDepth,
    ActiveColor,
}

#[derive(Component)]
pub struct KinectFrameBuffers {
    // viewable buffers
    pub current_frame: KinectFrameMessage,
    pub derived_frame: KinectFrameMessage,
    pub depth_baseline_frame: Gray16Image,
    pub active_depth: Gray16Image,
    pub active_color: RgbImage,
    // non-viewable data
    pub frame_history: std::collections::VecDeque<KinectFrameMessage>,
}

impl Default for KinectFrameBuffers {
    fn default() -> Self {
        let color_frame = RgbImage::new(COLOR_WIDTH as u32, COLOR_HEIGHT as u32);
        let depth_frame = Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32);
        let frame_message = KinectFrameMessage {
            color_frame: color_frame.clone(),
            depth_frame: depth_frame.clone(),
            ..Default::default()
        };
        Self {
            current_frame: frame_message.clone(),
            derived_frame: frame_message.clone(),
            depth_baseline_frame: depth_frame.clone(),
            active_depth: depth_frame.clone(),
            active_color: color_frame.clone(),
            frame_history: Default::default(),
        }
    }
}

impl KinectFrameBuffers {
    pub fn get_buffer(&self, buffer_name: KinectFrameBufferName) -> Vec<u8> {
        match buffer_name {
            KinectFrameBufferName::CurrentColor => color_frame_to_pixels(&self.current_frame.color_frame),
            KinectFrameBufferName::CurrentDepth => depth_frame_to_pixels(&self.current_frame.depth_frame),
            KinectFrameBufferName::DerivedDepth => depth_frame_to_pixels(&self.derived_frame.depth_frame),
            KinectFrameBufferName::DepthBaseline => depth_frame_to_pixels(&self.depth_baseline_frame),
            KinectFrameBufferName::ActiveDepth => depth_frame_to_pixels(&self.active_depth),
            KinectFrameBufferName::ActiveColor => color_frame_to_pixels(&self.active_color),
            KinectFrameBufferName::CurrentPlayerIndex => player_index_frame_to_pixels(&self.current_frame.depth_frame),
            KinectFrameBufferName::DerivedPlayerIndex => player_index_frame_to_pixels(&self.derived_frame.depth_frame),
        }
    }
}

fn receive_kinect_current_frame(
    receiver: NonSend<KinectReceiver>,
    mut current_frame_query: Query<(&KinectPostProcessorConfig, &mut KinectFrameBuffers)>,
) {
    if let Ok(received_frame) = receiver.0.try_recv() {
        let (config, mut buffers) = current_frame_query.single_mut();
        receive_and_process_frame(received_frame, config, &mut buffers);
    }
}

fn receive_and_process_frame(
    received_frame: KinectFrameMessage,
    config: &KinectPostProcessorConfig,
    buffers: &mut KinectFrameBuffers,
) {
    buffers.current_frame = received_frame.clone();
    buffers.derived_frame = received_frame.clone();
    for historic_frame in buffers.frame_history.iter() {
        for (i, depth) in historic_frame.depth_frame.iter().enumerate() {
            if depth == &NUI_IMAGE_DEPTH_NO_VALUE {
                continue;
            }
            if buffers.derived_frame.depth_frame.get(i) == Some(&NUI_IMAGE_DEPTH_NO_VALUE) {
                *buffers.derived_frame.depth_frame.get_mut(i).unwrap() = *depth;
            }
        }
    }
    while buffers.frame_history.len() > config.history_buffer_size {
        buffers.frame_history.pop_back();
    }
    buffers.frame_history.push_front(received_frame.clone());

    // subtract depth using depth threshold
    let depth_threshold = config.depth_threshold as u16;

    for (i, current_depth) in buffers.derived_frame.depth_frame.iter().enumerate() {
        let baseline_depth = buffers.depth_baseline_frame.get(i).unwrap();
        *buffers.active_depth.get_mut(i).unwrap() = match (current_depth, baseline_depth) {
            (&NUI_IMAGE_DEPTH_NO_VALUE, _) => 0u16,
            (&value, &NUI_IMAGE_DEPTH_NO_VALUE) => value,
            (&value, &baseline) if value < baseline && (value + depth_threshold) < baseline => value,
            _ => 0u16,
        };
    }

    for (x, y, pixel) in buffers.active_color.enumerate_pixels_mut() {
        let depth = buffers.active_depth.get_pixel(x, y).0[0];
        *pixel = if depth != NUI_IMAGE_DEPTH_NO_VALUE {
            *buffers.derived_frame.color_frame.get_pixel(x, y)
        } else {
            Rgb([0, 0, 0])
        }
    }
}

pub fn load_baseline_frame(path: impl AsRef<std::path::Path>) -> Result<Gray16Image, image::ImageError> {
    image::open(path).map(|img| img.into_luma16())
}
pub fn try_load_baseline_frame(path: impl AsRef<std::path::Path>) -> Gray16Image {
    image::open(path).map(|img| img.into_luma16()).unwrap_or_else(|_| {
        info!("failed to load depth baseline frame");
        Gray16Image::new(DEPTH_WIDTH as u32, DEPTH_HEIGHT as u32)
    })
}

fn setup_kinect_receiver(world: &mut World) {
    let receiver = start_frame_thread();
    world.insert_non_send_resource(KinectReceiver(receiver));
    world.spawn((
        Name::new("frame and buffer"),
        KinectPostProcessorConfig::default(),
        KinectFrameBuffers {
            depth_baseline_frame: try_load_baseline_frame("kinect_depth_data_empty.png"),
            ..Default::default()
        },
    ));
}

pub struct KinectReceiverPlugin;
impl Plugin for KinectReceiverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_kinect_receiver)
            .add_system(receive_kinect_current_frame)
            .register_type::<KinectPostProcessorConfig>();
    }
}
