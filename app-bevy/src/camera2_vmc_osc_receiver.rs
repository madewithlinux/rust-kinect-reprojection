use std::f32::consts::PI;

use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy_osc::{Osc, OscEvent, OscSettings};
use bevy_prototype_debug_lines::DebugLines;
use iyes_loopless::prelude::*;
use nannou_osc::Message;
use nannou_osc::Packet;
use nannou_osc::Type;

use crate::app_settings::camera2_vmc_enabled;
use crate::app_settings::AppSettings;
use crate::delay_buffer::query_performance_counter_ms;
use crate::delay_buffer::DelayBuffer;
use crate::util::draw_debug_axes;

#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct VmcCameraInfo {
    pub translation: Vec3,
    pub rotation: Quat,
    pub fov: f32,
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Component)]
pub struct VmcCameraMarker;

#[derive(Resource, Debug, Default, Clone)]
struct CameraReceiverBuffer(DelayBuffer<VmcCameraInfo>);

pub struct OscReceiverPlugin;
impl Plugin for OscReceiverPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(Osc)
            .insert_resource(OscSettings {
                recv_addr: Some("127.0.0.1:34254"),
                log: false,
                ..Default::default()
            })
            .insert_resource(CameraReceiverBuffer::default())
            .add_system(osc_event_listener_system.run_if(camera2_vmc_enabled))
            .add_system(osc_transform_watcher.run_if(camera2_vmc_enabled))
            // .register_type()
            ;
    }
}

const CAMERA2_VMC_ADDR: &str = "/VMC/Ext/Cam";

fn osc_transform_watcher(
    mut receive_buffer: ResMut<CameraReceiverBuffer>,
    settings: Res<AppSettings>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<VmcCameraMarker>>,
) {
    let Some(vmc_camera_info) = receive_buffer.0.pop_for_delay(settings.fixed_delay_ms) else { return; };
    let (mut transform, mut projection) = camera_query.single_mut();
    *transform = Transform::from_translation(vmc_camera_info.translation).with_rotation(vmc_camera_info.rotation);
    *projection = PerspectiveProjection {
        fov: vmc_camera_info.fov * PI / 180.0,
        ..default()
    }
    .into();
}

fn osc_event_listener_system(
    mut events: EventReader<OscEvent>,
    mut receive_buffer: ResMut<CameraReceiverBuffer>,
    mut lines: ResMut<DebugLines>,
    settings: Res<AppSettings>,
) {
    let timestamp = query_performance_counter_ms();
    for event in events.iter() {
        info!("osc event: {:?}", &event.packet);
        match &event.packet {
            Packet::Message(Message { addr, args }) => {
                if addr != CAMERA2_VMC_ADDR {
                    continue;
                }
                let Some(args) = args else { continue };
                match &args[..] {
                    [Type::String(camera), Type::Float(xpos), Type::Float(ypos), Type::Float(zpos), Type::Float(xrot), Type::Float(yrot), Type::Float(zrot), Type::Float(wrot), Type::Float(fov)]
                        if camera == "Camera" =>
                    {
                        // TODO: check if this works
                        let transform =
                            unity_to_bevy_coordinate_system([*xpos, *ypos, *zpos], [*xrot, *yrot, *zrot, *wrot]);
                        if settings.show_debug_axes {
                            draw_debug_axes(&mut lines, &transform, 0.2);
                        }
                        let (scale, rotation, translation) = transform.to_scale_rotation_translation();
                        info!("timestamp   = {:?}", timestamp);
                        info!("scale       = {:?}", scale);
                        info!("rotation    = {:?}", rotation);
                        info!("translation = {:?}", translation);
                        receive_buffer.0.push_for_timestamp(
                            timestamp,
                            VmcCameraInfo {
                                translation,
                                rotation,
                                fov: *fov,
                            },
                        );
                    }
                    _ => todo!(),
                };
            }
            Packet::Bundle(_) => todo!(),
        };
    }
}

fn unity_to_bevy_coordinate_system(pos: [f32; 3], rot: [f32; 4]) -> Affine3A {
    let unity_translation = Vec3::from_array(pos);
    let unity_rotation = Quat::from_array(rot);

    let translation = unity_translation * Vec3::new(1.0, 1.0, -1.0);
    // ref https://math.stackexchange.com/questions/2255822/conversion-of-rotation-between-lh-and-rh-coordinate-systems-with-quaternions
    let rotation = Quat::from_xyzw(0.0, 0.0, -1.0, 0.0) * unity_rotation * Quat::from_xyzw(0.0, 0.0, 1.0, 0.0);
    let transform = Affine3A::from_rotation_translation(rotation, translation);
    transform
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unity_to_bevy_coordinate_system() {
        let xpos = 0.077;
        let ypos = 2.4273;
        let zpos = -1.9451;
        let xrot = 0.2192877;
        let yrot = 0.0;
        let zrot = 0.0;
        let wrot = 0.97566026;

        let transform = unity_to_bevy_coordinate_system([xpos, ypos, zpos], [xrot, yrot, zrot, wrot]);

        let (scale, rotation, translation) = transform.to_scale_rotation_translation();

        println!("scale       = {:?}", scale);
        println!("rotation    = {:?}", rotation);
        println!("translation = {:?}", translation);

        assert_eq!(scale, Vec3::splat(1.0));
        assert_eq!(translation, Vec3::new(0.077, 2.4273, 1.9451,));
        // TODO: check rotation
    }
}
