use std::f32::consts::PI;

use array2d::Array2D;
use bevy::{math::Affine3A, prelude::*};

use itertools::Itertools;
use kinect1::{
    skeleton::{SkeletonPositionData, SkeletonPositionTrackingState},
    worker_v2::{start_frame_thread2, FrameMessage, FrameMessageReceiver},
};

use crate::{COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH};

#[derive(Debug)]
pub struct KinectReceiver(pub FrameMessageReceiver);

#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct KinectDepthTransformer {
    pub pixel_width: usize,
    pub width: f32,
    pub height: f32,
    // TODO: add coordinate offset as well
    pub sensor_tilt_angle_deg: f32,
    pub kinect_position: Vec3,
    pub kinect_rot_deg: Vec3,
    pub kinect_scale: Vec3,
    pub point_transform_matrix: Affine3A,
    pub point_transform_matrix_inverse: Affine3A,
    pub point_cloud_skel: bool,
}
fn update_depth_transformer(mut kdt: ResMut<KinectDepthTransformer>) {
    kdt.point_transform_matrix = Affine3A::IDENTITY
        * Affine3A::from_rotation_x(kdt.kinect_rot_deg.x * PI / 180.0)
        * Affine3A::from_rotation_y(kdt.kinect_rot_deg.y * PI / 180.0)
        * Affine3A::from_rotation_z(kdt.kinect_rot_deg.z * PI / 180.0)
        * Affine3A::from_translation(kdt.kinect_position)
        * Affine3A::from_scale(kdt.kinect_scale);
    kdt.point_transform_matrix_inverse = kdt.point_transform_matrix.inverse();
}
impl KinectDepthTransformer {
    pub fn new() -> Self {
        let sensor_tilt_angle_deg = -33.0;
        let point_transform_matrix = Affine3A::from_rotation_x(sensor_tilt_angle_deg * PI / 180.0);
        Self {
            pixel_width: DEPTH_WIDTH,
            width: DEPTH_WIDTH as f32,
            height: DEPTH_HEIGHT as f32,
            sensor_tilt_angle_deg,
            kinect_position: Vec3::new(-0.077, 2.4273, 1.9451),
            kinect_rot_deg: Vec3::new(33.0, 0.0, 0.0),
            kinect_scale: Vec3::new(-1.0, -1.0, -1.0),
            point_transform_matrix,
            point_transform_matrix_inverse: Affine3A::IDENTITY,
            point_cloud_skel: false,
        }
    }
    pub fn skeleton_bone_to_xyz(&self, bone: &[SkeletonPositionData; 2], depth_frame: &[u16]) -> Option<(Vec3, Vec3)> {
        match (
            self.skeleton_position_to_xyz(&bone[0], depth_frame),
            self.skeleton_position_to_xyz(&bone[1], depth_frame),
        ) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
    pub fn skeleton_position_to_xyz(&self, pos: &SkeletonPositionData, depth_frame: &[u16]) -> Option<Vec3> {
        if pos.pixel_index >= depth_frame.len()
            || pos.pixel_index == 0
            || pos.tracking_state == SkeletonPositionTrackingState::NotTracked
        {
            return None;
        }
        let depth = depth_frame[pos.pixel_index];
        if depth == 0 {
            return None;
        }
        Some(self.index_depth_to_xyz(pos.pixel_index, depth))
    }
    pub fn index_depth_to_xyz(&self, pixel_index: usize, depth_in_mm: u16) -> Vec3 {
        let i = pixel_index % self.pixel_width;
        let j = pixel_index / self.pixel_width;
        self.coordinate_depth_to_xyz(i, j, depth_in_mm)
    }
    pub fn coordinate_depth_to_xyz(&self, i: usize, j: usize, depth_in_mm: u16) -> Vec3 {
        let i = i as f32;
        let j = j as f32;
        // ref https://openkinect.org/wiki/Imaging_Information
        let z = depth_in_mm as f32;
        let min_distance = -10.0;
        let scale_factor = 0.0021;
        let x = (i - self.width / 2.0) * (z + min_distance) * scale_factor;
        let y = (j - self.height / 2.0) * (z + min_distance) * scale_factor;
        self.point_transform_matrix_inverse.transform_point3(Vec3::new(x, y, z) / 1_000.0)
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct KinectPostProcessorConfig {
    pub history_buffer_size: usize,
    pub depth_threshold: f32,
    /// deprecated
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
    depth_transformer: Res<KinectDepthTransformer>,
) {
    if let Ok(received_frame) = receiver.0.try_recv() {
        let (config, mut buffers) = current_frame_query.single_mut();
        process_received_frame(received_frame, config, &mut buffers, &depth_transformer);
    }
}

fn process_received_frame(
    received_frame: FrameMessage,
    config: &KinectPostProcessorConfig,
    buffers: &mut KinectFrameBuffers,
    depth_transformer: &KinectDepthTransformer,
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

    for (index, &depth) in buffers.derived_frame.depth.iter().enumerate() {
        let i = index % buffers.derived_frame.width;
        let j = index / buffers.derived_frame.width;
        buffers.point_cloud[(j as usize, i as usize)] = depth_transformer.index_depth_to_xyz(index, depth);
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
    let receiver = start_frame_thread2();
    world.insert_non_send_resource(KinectReceiver(receiver));
    world.insert_resource(KinectDepthTransformer::new());
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
            .add_system(update_depth_transformer)
            .register_type::<KinectPostProcessorConfig>()
            .register_type::<KinectDepthTransformer>();
    }
}
