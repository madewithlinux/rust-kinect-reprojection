use bevy::{math::Affine3A, prelude::*};
use bevy_prototype_debug_lines::DebugLines;
use openvr::TrackingResult;

use crate::util::draw_debug_axes;

pub struct OpenVrContextSystem(openvr::Context, openvr::System);

/// openvr is scaled in meters, but we are using millimeters
pub const OPENVR_SCALE_FACTOR: f32 = 1_000.0;

#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct TrackedDevicePose {
    pub is_good: bool,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub position: Vec3,
    pub transform: Affine3A,
}
impl From<&openvr::TrackedDevicePose> for TrackedDevicePose {
    fn from(pose: &openvr::TrackedDevicePose) -> Self {
        let device_to_absolute_tracking = *pose.device_to_absolute_tracking();
        Self {
            is_good: pose.pose_is_valid() && pose.device_is_connected() && pose.tracking_result() == TrackingResult::OK,
            velocity: Vec3::from_array(*pose.velocity()),
            angular_velocity: Vec3::from_array(*pose.angular_velocity()),
            position: Vec3::new(
                device_to_absolute_tracking[0][3] * OPENVR_SCALE_FACTOR,
                device_to_absolute_tracking[1][3] * OPENVR_SCALE_FACTOR,
                device_to_absolute_tracking[2][3] * OPENVR_SCALE_FACTOR,
            ),
            transform: Affine3A::from_mat4(
                Mat4::from_cols_array_2d(&[
                    device_to_absolute_tracking[0],
                    device_to_absolute_tracking[1],
                    device_to_absolute_tracking[2],
                    [0.0, 0.0, 0.0, 0.0],
                ])
                .transpose()
                    * OPENVR_SCALE_FACTOR,
            ),
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct OpenVrPoseData {
    pub hmd: TrackedDevicePose,
    pub left_controller: TrackedDevicePose,
    pub right_controller: TrackedDevicePose,
}

fn setup_openvr_connector(world: &mut World) {
    let context = unsafe { openvr::init(openvr::ApplicationType::Utility).unwrap() };
    let system = context.system().unwrap();
    world.insert_non_send_resource(OpenVrContextSystem(context, system));
    world.insert_resource(OpenVrPoseData::default());
}

fn update_pose_data(mut pose_data: ResMut<OpenVrPoseData>, open_vr_context_system: NonSend<OpenVrContextSystem>) {
    let system: &openvr::System = &open_vr_context_system.1;

    let left_controller_index = system
        .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::LeftHand)
        .unwrap_or((openvr::MAX_TRACKED_DEVICE_COUNT + 1) as u32) as usize;
    let right_controller_index = system
        .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::RightHand)
        .unwrap_or((openvr::MAX_TRACKED_DEVICE_COUNT + 1) as u32) as usize;

    let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.0);
    for (i, pose) in poses.iter().enumerate() {
        if !pose.pose_is_valid() {
            continue;
        }
        if !pose.device_is_connected() || pose.tracking_result() != TrackingResult::OK {
            continue;
        }
        if system.tracked_device_class(i as u32) == openvr::TrackedDeviceClass::HMD {
            pose_data.hmd = TrackedDevicePose::from(pose);
        }
        match system.tracked_device_class(i as u32) {
            openvr::TrackedDeviceClass::HMD => {
                pose_data.hmd = TrackedDevicePose::from(pose);
            }
            openvr::TrackedDeviceClass::Controller => {
                if i == left_controller_index {
                    pose_data.left_controller = TrackedDevicePose::from(pose);
                } else if i == right_controller_index {
                    pose_data.right_controller = TrackedDevicePose::from(pose);
                }
            }
            _ => (), // skip other tracking data
        }
    }
}

fn debug_pose_data(pose_data: Res<OpenVrPoseData>, mut lines: ResMut<DebugLines>) {
    draw_debug_axes(&mut lines, &pose_data.hmd.transform, 200.0);
    draw_debug_axes(&mut lines, &pose_data.left_controller.transform, 200.0);
    draw_debug_axes(&mut lines, &pose_data.right_controller.transform, 200.0);

    // // let REFERENCE_POINTS = vec![
    // //     Vec3::new(1.1872808, 1.5832841, -0.95948) * 1_000.0,
    // //     Vec3::new(1.2902215, -0.021568049, -0.59659046) * 1_000.0,
    // //     Vec3::new(-1.4719752, 0.45078325, -0.96842957) * 1_000.0,
    // // ];
    // for (p, _) in REFERENCE_POINTS.iter() {
    //     draw_debug_axes(&mut lines, &Affine3A::from_translation(*p * 1_000.0), 100.0);
    // }
}

pub struct VrConnectorPlugin;
impl Plugin for VrConnectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_openvr_connector)
            .add_system(update_pose_data)
            .add_system(debug_pose_data)
            .register_type::<OpenVrPoseData>();
    }
}
