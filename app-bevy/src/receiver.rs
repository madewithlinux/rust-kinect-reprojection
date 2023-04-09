use std::f32::consts::PI;
use std::io::Write;

use bevy::{math::Affine3A, prelude::*};
use iyes_loopless::prelude::*;

#[cfg(feature = "debug_helpers")]
use bevy_prototype_debug_lines::DebugLines;

use kinect1::worker_v2::ReceiverThreadArgs;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing::instrument;

use kinect1::{
    skeleton::{SkeletonFrame, SkeletonPositionData, SkeletonPositionTrackingState},
    worker_v2::{start_frame_thread2, FrameMessage, FrameMessageReceiver},
};

use crate::app_settings::debug_axes_enabled;
use crate::app_settings::use_kinect_static_frame;
use crate::delay_buffer::query_performance_counter_ms;
use crate::util::read_from_json_file;
use crate::vr_connector::OpenVrPoseData;
use crate::{
    app_settings::{kinect_enabled, AppSettings},
    delay_buffer::DelayBuffer,
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct KinectReceiver(pub FrameMessageReceiver);

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct KinectDepthTransformer {
    pub pixel_width: usize,
    pub width: f32,
    pub height: f32,
    // these values are updated from AppSettings whenever it changes
    // pub kinect_position: Vec3,
    // pub kinect_rot_deg: Vec3,
    // pub kinect_scale: Vec3,
    pub point_transform_matrix: Affine3A,
    point_transform_matrix_inverse: Affine3A,
    pub point_cloud_skel: bool,
}
fn update_depth_transformer(
    mut kdt: ResMut<KinectDepthTransformer>,
    settings: Res<AppSettings>,
    pose_data_opt: Option<Res<OpenVrPoseData>>,
) {
    if let (Some(pose_data), Some(kinect_tracker_offset)) = (pose_data_opt, settings.kinect_tracker_offset) {
        if pose_data.kinect_tracker.is_good {
            kdt.point_transform_matrix = pose_data.kinect_tracker.transform * kinect_tracker_offset;
            kdt.point_transform_matrix_inverse = kdt.point_transform_matrix.inverse();
            return;
        }
    }
    if !settings.is_added() && !settings.is_changed() {
        return;
    }

    // kdt.kinect_position = settings.kinect_transform.position;
    // kdt.kinect_rot_deg = settings.kinect_transform.euler_rotation;
    // kdt.kinect_scale = settings.kinect_transform.scale;

    // kdt.point_transform_matrix = Affine3A::from_scale_rotation_translation(
    //     kdt.kinect_scale,
    //     Quat::from_euler(
    //         EulerRot::XYZ,
    //         kdt.kinect_rot_deg.x * PI / 180.0,
    //         kdt.kinect_rot_deg.y * PI / 180.0,
    //         kdt.kinect_rot_deg.z * PI / 180.0,
    //     ),
    //     kdt.kinect_position,
    // );
    kdt.point_transform_matrix = settings.kinect_transform.to_affine();
    kdt.point_transform_matrix_inverse = kdt.point_transform_matrix.inverse();
}
impl FromWorld for KinectDepthTransformer {
    fn from_world(world: &mut World) -> Self {
        let settings = world.resource::<AppSettings>();
        let point_transform_matrix = settings.kinect_transform.to_affine();
        let point_transform_matrix_inverse = point_transform_matrix.inverse();
        Self {
            // kinect_position: settings.kinect_transform.position,
            // kinect_rot_deg: settings.kinect_transform.euler_rotation,
            // kinect_scale: settings.kinect_transform.scale,
            point_transform_matrix,
            point_transform_matrix_inverse,
            ..Self::new()
        }
    }
}
impl KinectDepthTransformer {
    fn new() -> Self {
        Self {
            pixel_width: DEPTH_WIDTH,
            width: DEPTH_WIDTH as f32,
            height: DEPTH_HEIGHT as f32,
            // kinect_position: Vec3::new(0.18, 2.4273, 1.9451),
            // kinect_rot_deg: Vec3::new(-33.0, 180.0, 0.0),
            // kinect_scale: Vec3::new(1.0, 1.0, 1.0),
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

#[cfg(feature = "debug_helpers")]
fn kinect_view_frustum_debug_lines(kdt: Res<KinectDepthTransformer>, mut lines: ResMut<DebugLines>) {
    // let origin = kdt.coordinate_depth_to_world(DEPTH_WIDTH / 2, DEPTH_HEIGHT / 2, 20);
    let near_depth_mm = 800;
    let far_depth_mm = 4000;

    // let corners = [(0, 0), (DEPTH_WIDTH, 0), (DEPTH_WIDTH, DEPTH_HEIGHT), (0, DEPTH_HEIGHT)];
    // these are the approximate coordinates in RGB space of the limits of the depth sensor
    let corners = [
        (33, 10), //
        (620, 14),
        (627, 459),
        (30, 456),
    ];
    let edges = [
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[3]),
        (corners[3], corners[0]),
    ];

    // too near
    for (i, j) in corners {
        lines.line_colored(
            kdt.coordinate_depth_to_world(i, j, 20),
            kdt.coordinate_depth_to_world(i, j, near_depth_mm),
            0.0,
            Color::RED,
        );
    }

    // sweet spot
    for depth in [near_depth_mm, far_depth_mm] {
        for ((i0, j0), (i1, j1)) in edges {
            lines.line_colored(
                kdt.coordinate_depth_to_world(i0, j0, depth),
                kdt.coordinate_depth_to_world(i1, j1, depth),
                0.0,
                Color::GREEN,
            );
        }
    }
    for (i, j) in corners {
        lines.line_colored(
            kdt.coordinate_depth_to_world(i, j, near_depth_mm),
            kdt.coordinate_depth_to_world(i, j, far_depth_mm),
            0.0,
            Color::GREEN,
        );
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct KinectFrameBuffers {
    pub timestamp: i64,
    // viewable buffers
    pub rgba: Vec<[u8; 4]>,
    pub depth: Vec<u16>,
    pub player_index: Vec<u8>,
    pub skeleton_points: Vec<Vec3>,
    #[cfg(feature = "skeleton_frame")]
    #[serde(skip)]
    pub skeleton_frame: SkeletonFrame,
}

impl Default for KinectFrameBuffers {
    fn default() -> Self {
        let width = COLOR_WIDTH;
        let height = COLOR_HEIGHT;
        Self {
            timestamp: 0,
            rgba: vec![Default::default(); width * height],
            depth: vec![Default::default(); width * height],
            player_index: vec![Default::default(); width * height],
            skeleton_points: vec![Default::default(); width * height],
            #[cfg(feature = "skeleton_frame")]
            skeleton_frame: Default::default(),
        }
    }
}

#[derive(Resource, Clone, Default)]
pub struct KinectFrameDataDelayBufferV2(pub DelayBuffer<KinectFrameBuffers>);

fn receive_kinect_current_frame(
    receiver: NonSend<KinectReceiver>,
    mut buffers: ResMut<KinectFrameBuffers>,
    mut frame_delay_buffer: ResMut<KinectFrameDataDelayBufferV2>,
    settings: Res<AppSettings>,
) {
    while let Ok(received_frame) = receiver.0.try_recv() {
        let frame_data = frame_message_to_frame_data_v2(received_frame);
        frame_delay_buffer
            .0
            .push_for_timestamp(frame_data.timestamp, frame_data);
    }
    if let Some(frame_data) = frame_delay_buffer.0.pop_for_delay(settings.fixed_delay_ms) {
        *buffers = frame_data;
    }
}

#[instrument(skip_all)]
fn frame_message_to_frame_data_v2(received_frame: FrameMessage) -> KinectFrameBuffers {
    KinectFrameBuffers {
        timestamp: received_frame
            .depth_frame_info
            .timestamp
            .max(received_frame.color_frame_info.timestamp),
        rgba: received_frame.rgba,
        depth: received_frame.depth,
        player_index: received_frame.player_index,
        skeleton_points: received_frame.skeleton_points,
        #[cfg(feature = "skeleton_frame")]
        skeleton_frame: received_frame.skeleton_frame,
    }
}

fn setup_kinect_receiver(world: &mut World) {
    #[cfg(feature = "skeleton_frame")]
    let skeleton_stream_enabled = true;
    #[cfg(not(feature = "skeleton"))]
    let skeleton_stream_enabled = false;
    let receiver = start_frame_thread2(ReceiverThreadArgs {
        skeleton_stream_enabled,
        ..default()
    });
    world.insert_non_send_resource(KinectReceiver(receiver));
}

fn static_frame_system(
    mut buffers: ResMut<KinectFrameBuffers>,
    mut frame_delay_buffer: ResMut<KinectFrameDataDelayBufferV2>,
    settings: Res<AppSettings>,
    mut static_frame: Local<Option<KinectFrameBuffers>>,
    mut last_update_ms: Local<i64>,
) {
    let Some(static_frame_path) = &settings.kinect_static_frame else {
        return;
    };
    if static_frame.is_none() {
        info!("reading framebuffers from {:?}", static_frame_path);
        *static_frame = Some(read_from_json_file(static_frame_path));
        info!("finished reading framebuffers from {:?}", static_frame_path);
    }
    let Some(static_frame) = &*static_frame else {
        return;
    };
    let current_ms = query_performance_counter_ms();
    let target_update_delay = (30.0 / 1000.0) as i64;
    if current_ms > *last_update_ms + target_update_delay {
        *buffers = static_frame.clone();
        frame_delay_buffer.0.push_for_timestamp(
            current_ms - settings.fixed_delay_ms + target_update_delay,
            static_frame.clone(),
        );
        *last_update_ms = current_ms;
    }
    // remove old frames to avoid leaking memory
    frame_delay_buffer.0.pop_for_delay(settings.fixed_delay_ms * 2);
}

pub struct KinectReceiverPlugin;
impl Plugin for KinectReceiverPlugin {
    fn build(&self, app: &mut App) {
        app //
            // .insert_resource(KinectDepthTransformer::new())
            .init_resource::<KinectDepthTransformer>()
            .insert_resource(KinectFrameBuffers::default())
            .insert_resource(KinectFrameDataDelayBufferV2::default())
            .add_startup_system(
                setup_kinect_receiver
                    .run_if(kinect_enabled)
                    .run_if_not(use_kinect_static_frame),
            )
            .add_system(
                receive_kinect_current_frame
                    .run_if(kinect_enabled)
                    .run_if_not(use_kinect_static_frame),
            )
            .add_system(
                static_frame_system
                    .run_if(kinect_enabled)
                    .run_if(use_kinect_static_frame),
            )
            .add_system(update_depth_transformer.run_if(kinect_enabled))
            .register_type::<KinectDepthTransformer>();

        #[cfg(feature = "debug_helpers")]
        app //
            .add_system(
                kinect_view_frustum_debug_lines
                    .run_if(kinect_enabled)
                    .run_if(debug_axes_enabled),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of_skeleton_frame() {
        assert_eq!(std::mem::size_of::<SkeletonFrame>(), 72);
        // assert_eq!(std::mem::size_of::<KinectFrameBuffers>(), 176);
    }
}
