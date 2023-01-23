use std::f32::consts::PI;

use bevy::{math::Affine3A, prelude::*};
use iyes_loopless::prelude::*;

use tracing::{info, span};
use tracing::{instrument, Level};

use kinect1::{
    skeleton::{SkeletonFrame, SkeletonPositionData, SkeletonPositionTrackingState},
    worker_v2::{start_frame_thread2, FrameMessage, FrameMessageReceiver},
};

use crate::{
    app_settings::{kinect_enabled, AppSettings},
    delay_buffer::DelayBuffer,
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH, FIXED_DELAY_MS,
};

pub struct KinectReceiver(pub FrameMessageReceiver, pub DelayBuffer<FrameMessage>);

// TODO: put this stuff in AppSettings as well?
#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct KinectDepthTransformer {
    pub pixel_width: usize,
    pub width: f32,
    pub height: f32,
    pub kinect_position: Vec3,
    pub kinect_rot_deg: Vec3,
    pub kinect_scale: Vec3,
    pub point_transform_matrix: Affine3A,
    point_transform_matrix_inverse: Affine3A,
    pub point_cloud_skel: bool,
}
fn update_depth_transformer(mut kdt: ResMut<KinectDepthTransformer>) {
    kdt.point_transform_matrix = Affine3A::from_scale_rotation_translation(
        kdt.kinect_scale,
        Quat::from_euler(
            EulerRot::XYZ,
            kdt.kinect_rot_deg.x * PI / 180.0,
            kdt.kinect_rot_deg.y * PI / 180.0,
            kdt.kinect_rot_deg.z * PI / 180.0,
        ),
        kdt.kinect_position,
    );
    kdt.point_transform_matrix_inverse = kdt.point_transform_matrix.inverse();
}
impl KinectDepthTransformer {
    pub fn new() -> Self {
        Self {
            pixel_width: DEPTH_WIDTH,
            width: DEPTH_WIDTH as f32,
            height: DEPTH_HEIGHT as f32,
            kinect_position: Vec3::new(0.18, 2.4273, 1.9451),
            kinect_rot_deg: Vec3::new(-33.0, 180.0, 0.0),
            kinect_scale: Vec3::new(1.0, 1.0, 1.0),
            point_transform_matrix: Affine3A::IDENTITY,
            point_transform_matrix_inverse: Affine3A::IDENTITY,
            point_cloud_skel: true,
        }
    }
    pub fn skeleton_bone_to_world(
        &self,
        bone: &[SkeletonPositionData; 2],
        depth_frame: &[u16],
    ) -> Option<(Vec3, Vec3)> {
        // TODO: don't require depth frame here
        match (
            self.skeleton_position_to_world(&bone[0], depth_frame),
            self.skeleton_position_to_world(&bone[1], depth_frame),
        ) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
    pub fn skeleton_position_to_world(&self, pos: &SkeletonPositionData, depth_frame: &[u16]) -> Option<Vec3> {
        // TODO: don't require depth frame here
        if pos.pixel_index >= depth_frame.len()
            || pos.pixel_index == 0
            || pos.tracking_state == SkeletonPositionTrackingState::NotTracked
        {
            return None;
        }
        Some(self.skeleton_point_to_world(Vec3::new(pos.position[0].0, pos.position[1].0, pos.position[2].0)))
    }
    pub fn skeleton_point_to_world(&self, skeleton_point: Vec3) -> Vec3 {
        self.point_transform_matrix.transform_point3(skeleton_point)
    }

    pub fn flat_index_to_ij(&self, flat_index: usize) -> (usize, usize) {
        (flat_index % self.pixel_width, flat_index / self.pixel_width)
    }
    pub fn ij_to_flat_index(&self, i: usize, j: usize) -> usize {
        i + j * self.pixel_width
    }

    pub fn index_depth_to_world(&self, flat_index: usize, depth_in_mm: u16) -> Vec3 {
        let (i, j) = self.flat_index_to_ij(flat_index);
        self.coordinate_depth_to_world(i, j, depth_in_mm)
    }
    pub fn coordinate_depth_to_world(&self, i: usize, j: usize, depth_in_mm: u16) -> Vec3 {
        let i = i as f32;
        let j = self.height - 1.0 - (j as f32);
        // ref https://openkinect.org/wiki/Imaging_Information
        let z = depth_in_mm as f32;
        let min_distance = -10.0;
        let scale_factor = 0.0021;
        let x = (i - self.width / 2.0) * (z + min_distance) * scale_factor;
        let y = (j - self.height / 2.0) * (z + min_distance) * scale_factor;
        let world_point = Vec3::new(x, y, z) / 1_000.0;
        let world_point = self.point_transform_matrix.transform_point3(world_point);
        world_point
    }
}

#[derive(Resource)]
pub struct KinectFrameBuffers {
    // viewable buffers
    pub rgba: Vec<[u8; 4]>,
    // these ones may be derived values (if config.history_buffer_size > 1)
    pub depth: Vec<u16>,
    pub player_index: Vec<u8>,
    pub skeleton_points: Vec<Vec3>,
    pub skeleton_frame: SkeletonFrame,
    // non-viewable data
    pub frame_history: std::collections::VecDeque<FrameMessage>,
}

impl Default for KinectFrameBuffers {
    fn default() -> Self {
        let width = COLOR_WIDTH;
        let height = COLOR_HEIGHT;
        Self {
            rgba: vec![Default::default(); width * height],
            depth: vec![Default::default(); width * height],
            player_index: vec![Default::default(); width * height],
            skeleton_points: vec![Default::default(); width * height],
            skeleton_frame: Default::default(),
            frame_history: Default::default(),
        }
    }
}

fn receive_kinect_current_frame(
    mut receiver: NonSendMut<KinectReceiver>,
    mut buffers: ResMut<KinectFrameBuffers>,
    settings: Res<AppSettings>,
) {
    while let Ok(received_frame) = receiver.0.try_recv() {
        receiver.1.push_for_timestamp(
            received_frame
                .depth_frame_info
                .timestamp
                .max(received_frame.color_frame_info.timestamp),
            received_frame,
        );
    }
    if let Some(received_frame) = receiver.1.pop_for_delay(FIXED_DELAY_MS) {
        process_received_frame(received_frame, &mut buffers, &settings);
    }
}

#[instrument(skip_all)]
fn process_received_frame(received_frame: FrameMessage, buffers: &mut KinectFrameBuffers, settings: &AppSettings) {
    {
        let span = span!(Level::INFO, "copy");
        let _enter = span.enter();
        buffers.rgba.copy_from_slice(&received_frame.rgba);
        buffers.depth.copy_from_slice(&received_frame.depth);
        buffers.player_index.copy_from_slice(&received_frame.player_index);
        buffers.skeleton_points.copy_from_slice(&received_frame.skeleton_points);
        buffers.skeleton_frame = received_frame.skeleton_frame.clone();
    }

    buffers.frame_history.push_front(received_frame);
    while buffers.frame_history.len() > settings.history_buffer_size.max(1) {
        buffers.frame_history.pop_back();
    }
    if buffers.frame_history.len() <= 1 {
        return;
    }

    let span = span!(Level::INFO, "historic_frames");
    let _enter = span.enter();
    for historic_frame in buffers.frame_history.iter() {
        for (i, &depth) in historic_frame.depth.iter().enumerate() {
            if depth == 0 {
                continue;
            }
            if buffers.depth[i] == 0 {
                buffers.depth[i] = depth;
                buffers.player_index[i] = historic_frame.player_index[i];
                buffers.skeleton_points[i] = historic_frame.skeleton_points[i];
            }
        }
        // TODO: should we terminate the loop through historic buffers if no changes were made?
    }
}

fn setup_kinect_receiver(world: &mut World) {
    let receiver = start_frame_thread2();
    world.insert_non_send_resource(KinectReceiver(receiver, Default::default()));
}

pub struct KinectReceiverPlugin;
impl Plugin for KinectReceiverPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(KinectDepthTransformer::new())
            .insert_resource(KinectFrameBuffers::default())
            .add_startup_system(setup_kinect_receiver.run_if(kinect_enabled))
            .add_system(receive_kinect_current_frame.run_if(kinect_enabled))
            .add_system(update_depth_transformer.run_if(kinect_enabled))
            .register_type::<KinectDepthTransformer>();
    }
}
