use std::f32::consts::PI;

use bevy::{math::Affine3A, prelude::*};
use iyes_loopless::prelude::*;

use bevy_prototype_debug_lines::DebugLines;
use openvr::TrackingResult;

use crate::{
    app_settings::{debug_axes_enabled, vr_input_enabled},
    delay_buffer::{query_performance_counter_ms, DelayBuffer},
    util::draw_debug_axes,
    FIXED_DELAY_MS,
};

pub struct OpenVrContextSystem(openvr::Context, openvr::System);

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
                device_to_absolute_tracking[0][3],
                device_to_absolute_tracking[1][3],
                device_to_absolute_tracking[2][3],
            ),
            transform: Affine3A::from_mat4(
                Mat4::from_cols_array_2d(&[
                    device_to_absolute_tracking[0],
                    device_to_absolute_tracking[1],
                    device_to_absolute_tracking[2],
                    [0.0, 0.0, 0.0, 0.0],
                ])
                .transpose(),
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
}

fn update_pose_data(
    mut delayed_pose_data: ResMut<OpenVrPoseData>,
    open_vr_context_system: NonSend<OpenVrContextSystem>,
    mut pose_data_buffer: Local<DelayBuffer<OpenVrPoseData>>,
) {
    let timestamp = query_performance_counter_ms();

    let system: &openvr::System = &open_vr_context_system.1;

    let left_controller_index = system
        .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::LeftHand)
        .unwrap_or((openvr::MAX_TRACKED_DEVICE_COUNT + 1) as u32) as usize;
    let right_controller_index = system
        .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::RightHand)
        .unwrap_or((openvr::MAX_TRACKED_DEVICE_COUNT + 1) as u32) as usize;

    let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.0);
    let mut pose_data: OpenVrPoseData = pose_data_buffer.back().cloned().unwrap_or_default();
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
    pose_data_buffer.push_for_timestamp(timestamp, pose_data);
    if let Some(pd) = pose_data_buffer.pop_for_delay(FIXED_DELAY_MS) {
        *delayed_pose_data = pd;
    }
    // *delayed_pose_data = pose_data_buffer.pop_for_delay(FIXED_DELAY_MS).unwrap_or_default();
}

fn debug_pose_data(pose_data: Res<OpenVrPoseData>, mut lines: ResMut<DebugLines>) {
    draw_debug_axes(&mut lines, &pose_data.hmd.transform, 0.2);
    draw_debug_axes(&mut lines, &pose_data.left_controller.transform, 0.2);
    draw_debug_axes(&mut lines, &pose_data.right_controller.transform, 0.2);
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Component)]
pub enum VrPoseMarker {
    #[default]
    Hmd,
    LeftController,
    RightController,
}

fn spawn_vr_pose_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let green = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        unlit: true,
        ..Default::default()
    });
    let red = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        unlit: true,
        ..Default::default()
    });
    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 1.0),
        unlit: true,
        ..Default::default()
    });

    commands.spawn((
        Name::new("hmd"),
        VrPoseMarker::Hmd,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.180, 0.12, 0.30))),
            material: green,
            ..default()
        },
    ));

    let capsule = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.030,
        depth: 0.150,
        ..Default::default()
    }));
    commands
        .spawn((
            Name::new("left controller"),
            TransformBundle::default(),
            VisibilityBundle::default(),
            VrPoseMarker::LeftController,
        ))
        .with_children(|parent| {
            parent.spawn((PbrBundle {
                mesh: capsule.clone(),
                material: red,
                transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0))
                    .with_translation(Vec3::new(0.0, 0.0, 0.15 / 2.0)),
                ..default()
            },));
        });
    commands
        .spawn((
            Name::new("right controller"),
            TransformBundle::default(),
            VisibilityBundle::default(),
            VrPoseMarker::RightController,
        ))
        .with_children(|parent| {
            parent.spawn((PbrBundle {
                mesh: capsule,
                material: blue,
                transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0))
                    .with_translation(Vec3::new(0.0, 0.0, 0.15 / 2.0)),
                ..default()
            },));
        });
}

fn update_vr_pose_markers(mut query: Query<(&VrPoseMarker, &mut Transform)>, vr_pose_data: Res<OpenVrPoseData>) {
    for (marker, mut transform) in query.iter_mut() {
        *transform = match marker {
            VrPoseMarker::Hmd => Transform::from_matrix(vr_pose_data.hmd.transform.into()),
            VrPoseMarker::LeftController => Transform::from_matrix(vr_pose_data.left_controller.transform.into()),
            VrPoseMarker::RightController => Transform::from_matrix(vr_pose_data.right_controller.transform.into()),
        };
    }
}

pub struct VrConnectorPlugin;
impl Plugin for VrConnectorPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(OpenVrPoseData::default())
            .add_startup_system(setup_openvr_connector.run_if(vr_input_enabled))
            .add_system(update_pose_data.run_if(vr_input_enabled))
            .add_system(debug_pose_data.run_if(vr_input_enabled).run_if(debug_axes_enabled))
            // TODO: set visibility instead?
            .add_startup_system(
                spawn_vr_pose_markers
                    .run_if(vr_input_enabled)
                    .run_if(debug_axes_enabled),
            )
            .add_system(
                update_vr_pose_markers
                    .run_if(vr_input_enabled)
                    .run_if(debug_axes_enabled),
            )
            .register_type::<OpenVrPoseData>()
            .register_type::<VrPoseMarker>();
    }
}
