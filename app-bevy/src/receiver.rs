use std::f32::consts::PI;

use array2d::Array2D;
use bevy::{math::Affine3A, prelude::*};

use itertools::Itertools;
use kinect1::worker_v2::{start_frame_thread2, FrameMessage, FrameMessageReceiver};

use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Debug)]
pub struct KinectReceiver(pub FrameMessageReceiver);

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct KinectPostProcessorConfig {
    pub history_buffer_size: usize,
    pub depth_threshold: f32,
    pub sensor_tilt_angle_deg: f32,
    pub baseline_threshold_background_removal_enabled: bool,
}
impl Default for KinectPostProcessorConfig {
    fn default() -> Self {
        Self {
            history_buffer_size: 2,
            depth_threshold: 100.0,
            // sensor_tilt_angle_deg: -33.0,
            // sensor_tilt_angle_deg: 90.0,
            sensor_tilt_angle_deg: 0.0,
            baseline_threshold_background_removal_enabled: false,
        }
    }
}

#[derive(Component)]
pub struct KinectFrameBuffers {
    // viewable buffers
    pub current_frame: FrameMessage,
    pub derived_frame: FrameMessage,
    pub depth_baseline_frame: Vec<u16>,
    pub active_depth: Vec<u16>,
    pub active_color: Vec<[u8; 4]>,
    pub point_cloud: Array2D<Vec3>,
    // non-viewable data
    pub frame_history: std::collections::VecDeque<FrameMessage>,
}

impl Default for KinectFrameBuffers {
    fn default() -> Self {
        let width = COLOR_WIDTH;
        let height = COLOR_HEIGHT;
        let color_frame = vec![Default::default(); width * height];
        let depth_frame = vec![Default::default(); width * height];
        let player_index = vec![Default::default(); width * height];
        let frame_message = FrameMessage {
            width,
            height,
            rgba: color_frame.clone(),
            depth: depth_frame.clone(),
            player_index: player_index.clone(),
            color_frame_info: Default::default(),
            depth_frame_info: Default::default(),
            ..Default::default()
        };
        Self {
            current_frame: frame_message.clone(),
            derived_frame: frame_message.clone(),
            depth_baseline_frame: depth_frame.clone(),
            active_depth: depth_frame.clone(),
            active_color: color_frame.clone(),
            point_cloud: Array2D::filled_with(Vec3::default(), COLOR_HEIGHT, COLOR_WIDTH),
            frame_history: Default::default(),
        }
    }
}

fn receive_kinect_current_frame(
    receiver: NonSend<KinectReceiver>,
    mut current_frame_query: Query<(&KinectPostProcessorConfig, &mut KinectFrameBuffers)>,
) {
    if let Ok(received_frame) = receiver.0.try_recv() {
        let (config, mut buffers) = current_frame_query.single_mut();
        process_received_frame(received_frame, config, &mut buffers);
    }
}

fn process_received_frame(
    received_frame: FrameMessage,
    config: &KinectPostProcessorConfig,
    buffers: &mut KinectFrameBuffers,
) {
    buffers.current_frame = received_frame.clone();
    buffers.derived_frame = received_frame.clone();
    for historic_frame in buffers.frame_history.iter() {
        for (i, &depth) in historic_frame.depth.iter().enumerate() {
            if depth == 0 {
                continue;
            }
            if buffers.derived_frame.depth[i] == 0 {
                buffers.derived_frame.depth[i] = depth;
                buffers.derived_frame.player_index[i] = historic_frame.player_index[i];
            }
        }
    }
    while buffers.frame_history.len() > config.history_buffer_size {
        buffers.frame_history.pop_back();
    }
    buffers.frame_history.push_front(received_frame.clone());

    if config.baseline_threshold_background_removal_enabled {
        baseline_threshold_background_removal(config, buffers);
    }

    // TODO: use a real correction factor instead of this
    let point_transform = Affine3A::from_rotation_x(config.sensor_tilt_angle_deg * PI / 180.0);
    for (index, &depth) in buffers.derived_frame.depth.iter().enumerate() {
        let i = index % buffers.derived_frame.width;
        let j = index / buffers.derived_frame.width;
        let xyz = convert_depth_to_xyz(
            DEPTH_WIDTH as f32,
            DEPTH_HEIGHT as f32,
            i as f32,
            j as f32,
            depth as f32,
        );
        let xyz = point_transform.transform_vector3(xyz);
        buffers.point_cloud[(j as usize, i as usize)] = xyz;
    }
}

pub fn baseline_threshold_background_removal(config: &KinectPostProcessorConfig, buffers: &mut KinectFrameBuffers) {
    // subtract depth using depth threshold
    let depth_threshold = config.depth_threshold as u16;

    for (i, &current_depth) in buffers.derived_frame.depth.iter().enumerate() {
        let baseline_depth = buffers.depth_baseline_frame[i];
        buffers.active_depth[i] = match (current_depth, baseline_depth) {
            (0, _) => 0u16,
            (value, 0) => value,
            (value, baseline) if value < baseline && (value + depth_threshold) < baseline => value,
            _ => 0u16,
        };
    }

    for (i, &depth) in buffers.active_depth.iter().enumerate() {
        buffers.active_color[i] = if depth != 0 {
            buffers.current_frame.rgba[i]
        } else {
            [0, 0, 0, 255]
        };
    }
}

pub fn convert_depth_to_xyz(w: f32, h: f32, i: f32, j: f32, depth_in_mm: f32) -> Vec3 {
    // ref https://openkinect.org/wiki/Imaging_Information
    let z = depth_in_mm;
    let min_distance = -10.0;
    let scale_factor = 0.0021;
    let x = (i - w / 2.0) * (z + min_distance) * scale_factor;
    let y = (j - h / 2.0) * (z + min_distance) * scale_factor;
    Vec3::new(x, y, z)
}

pub fn load_baseline_frame(path: impl AsRef<std::path::Path>) -> Result<Vec<u16>, image::ImageError> {
    image::open(path).map(|img| img.into_luma16().iter().cloned().collect_vec())
}
pub fn try_load_baseline_frame(path: impl AsRef<std::path::Path>) -> Vec<u16> {
    image::open(path)
        .map(|img| img.into_luma16().iter().cloned().collect_vec())
        .unwrap_or_else(|_| {
            info!("failed to load depth baseline frame");
            vec![0; DEPTH_WIDTH * DEPTH_HEIGHT]
        })
}

fn setup_kinect_receiver(world: &mut World) {
    // let receiver = start_frame_thread();
    let receiver = start_frame_thread2();
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
