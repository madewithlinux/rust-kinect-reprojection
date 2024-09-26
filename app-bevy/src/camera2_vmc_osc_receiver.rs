use std::f32::consts::PI;

use anyhow::bail;
use anyhow::Context;
use bevy::math::vec2;
use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy_osc::{Osc, OscEvent, OscSettings};
#[cfg(feature = "debug_helpers")]
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;
use iyes_loopless::prelude::*;
use nannou_osc::Message;
use nannou_osc::Packet;
use nannou_osc::Type;
use serde::Deserialize;
use serde::Serialize;

use crate::app_settings::camera2_vmc_enabled;
use crate::app_settings::AppSettings;
use crate::delay_buffer::query_performance_counter_ms;
use crate::delay_buffer::DelayBuffer;
#[cfg(feature = "debug_helpers")]
use crate::util::draw_debug_axes;
use crate::util::try_read_from_json_file;
use crate::MainCamera;

#[derive(Debug, Copy, Clone, Reflect, Serialize, Deserialize)]
#[serde(default)]
pub struct Camera2ViewRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
impl Default for Camera2ViewRect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        }
    }
}
impl Camera2ViewRect {
    fn set_camera_settings(&self, camera: &mut Camera) {
        let Some(target_size) = camera.physical_target_size() else {
            return
        };
        let target_size = target_size.as_vec2();
        // in camera2, x/y 0/0 is the bottom left corner of the window
        let physical_position = vec2(self.x, 1.0 - self.y - self.height) * target_size;
        let physical_size = vec2(self.width, self.height) * target_size;

        camera.viewport = Some(Viewport {
            physical_position: physical_position.as_uvec2(),
            physical_size: physical_size.as_uvec2(),
            depth: 0.0..1.0,
        });
    }
}

#[derive(Resource, Debug, Default, Copy, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Resource)]
pub struct VmcCameraInfo {
    pub translation: Vec3,
    pub rotation: Quat,
    pub fov: f32,
}

impl VmcCameraInfo {
    fn set_camera_settings(&self, transform: &mut Transform, projection: &mut Projection) {
        *transform = Transform::from_translation(self.translation).with_rotation(self.rotation);
        *projection = PerspectiveProjection {
            fov: self.fov * PI / 180.0,
            ..default()
        }
        .into();
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Component)]
pub struct VmcCameraMarker(Camera2ViewRect);

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
            // run it at the end to make sure that the camera has spawned
            .add_system(setup_reload_initial_camera_settings.run_if(camera2_vmc_enabled).at_end())
            // .register_type()
            ;
    }
}

pub fn spawn_3d_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        VmcCameraMarker::default(),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.5, 3.6, 2.6))
                .looking_at(Vec3::new(0.0, 0.0, -0.8), Vec3::Y),
            ..default()
        },
    ));
}

const CAMERA2_VMC_ADDR: &str = "/VMC/Ext/Cam";

// region: VMC/OSC receiver

fn osc_transform_watcher(
    mut receive_buffer: ResMut<CameraReceiverBuffer>,
    settings: Res<AppSettings>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<VmcCameraMarker>>,
) {
    let Some(vmc_camera_info) = receive_buffer.0.pop_for_delay(settings.fixed_delay_ms) else { return; };
    let (mut transform, mut projection) = camera_query.single_mut();
    vmc_camera_info.set_camera_settings(&mut transform, &mut projection);
}

fn osc_event_listener_system(
    mut events: EventReader<OscEvent>,
    mut receive_buffer: ResMut<CameraReceiverBuffer>,
    #[cfg(feature = "debug_helpers")] mut lines: ResMut<DebugLines>,
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
                        let transform =
                            unity_to_bevy_coordinate_system([*xpos, *ypos, *zpos], [*xrot, *yrot, *zrot, *wrot]);
                        #[cfg(feature = "debug_helpers")]
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
                    _ => (),
                };
            }
            Packet::Bundle(_) => (),
        };
    }
}

// endregion

// region: read from JSON config files

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonXYZ {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum JsonCameraType {
    FirstPerson,
    #[default]
    Positionable,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonVMCProtocol {
    mode: String,
    destination: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonViewRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    locked: bool,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonCameraConfig {
    #[serde(rename = "type")]
    camera_type: JsonCameraType,
    #[serde(rename = "FOV")]
    fov: f32,
    #[serde(rename = "orthographic")]
    orthographic: bool,
    #[serde(rename = "viewRect")]
    view_rect: JsonViewRect,
    #[serde(rename = "VMCProtocol")]
    vmc_protocol: JsonVMCProtocol,
    #[serde(rename = "targetPos")]
    target_pos: JsonXYZ,
    #[serde(rename = "targetRot")]
    target_rot: JsonXYZ,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonScenes {
    #[serde(rename = "Menu")]
    menu: Vec<String>,
    #[serde(rename = "MultiplayerMenu")]
    multiplayer_menu: Vec<String>,
    #[serde(rename = "Playing")]
    playing: Vec<String>,
    #[serde(rename = "Playing360")]
    playing_360: Vec<String>,
    #[serde(rename = "PlayingModmap")]
    playing_modmap: Vec<String>,
    #[serde(rename = "PlayingMulti")]
    playing_multi: Vec<String>,
    #[serde(rename = "SpectatingMulti")]
    spectating_multi: Vec<String>,
    #[serde(rename = "Replay")]
    replay: Vec<String>,
    #[serde(rename = "FPFC")]
    fpfc: Vec<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct JsonSceneConfig {
    scenes: JsonScenes,
}

/// finds the first camera in camera2 that is active (in at least one scene)
/// and has type = Positionable and VMCProtocol.mode = Sender
pub fn read_camera2_info_from_config_files(
    camera2_settings_folder: impl AsRef<std::path::Path>,
) -> anyhow::Result<(VmcCameraInfo, Camera2ViewRect)> {
    let scenes_path = camera2_settings_folder.as_ref().join("Scenes.json");
    let scene_config: JsonSceneConfig = try_read_from_json_file(&scenes_path)?;
    let all_scenes = &[
        scene_config.scenes.menu.as_slice(),
        scene_config.scenes.multiplayer_menu.as_slice(),
        scene_config.scenes.playing.as_slice(),
        scene_config.scenes.playing_360.as_slice(),
        scene_config.scenes.playing_modmap.as_slice(),
        scene_config.scenes.playing_multi.as_slice(),
        scene_config.scenes.spectating_multi.as_slice(),
        scene_config.scenes.replay.as_slice(),
        scene_config.scenes.fpfc.as_slice(),
    ];
    let unique_camera_names = all_scenes
        .iter()
        .flat_map(|scene_cameras| scene_cameras.iter())
        .dedup()
        .collect_vec();
    for camera_name in unique_camera_names {
        let camera_config_path = camera2_settings_folder
            .as_ref()
            .join("Cameras")
            .join(camera_name)
            .with_extension("json");
        let camera_config: JsonCameraConfig = try_read_from_json_file(&camera_config_path).with_context(|| {
            format!(
                "cannot find camera {} (checked path {})",
                camera_name,
                &camera_config_path.to_string_lossy()
            )
        })?;
        if camera_config.camera_type == JsonCameraType::Positionable && camera_config.vmc_protocol.mode == "Sender" {
            let transform = unity_to_bevy_coordinate_system(
                [
                    camera_config.target_pos.x,
                    camera_config.target_pos.y,
                    camera_config.target_pos.z,
                ],
                Quat::from_euler(
                    EulerRot::YXZ,
                    camera_config.target_rot.y * PI / 180.0,
                    camera_config.target_rot.x * PI / 180.0,
                    camera_config.target_rot.z * PI / 180.0,
                )
                .to_array(),
            );
            let (_scale, rotation, translation) = transform.to_scale_rotation_translation();
            let camera_info = VmcCameraInfo {
                translation,
                rotation,
                fov: camera_config.fov,
            };
            let view_rect = Camera2ViewRect {
                x: camera_config.view_rect.x,
                y: camera_config.view_rect.y,
                width: camera_config.view_rect.width,
                height: camera_config.view_rect.height,
            };
            return Ok((camera_info, view_rect));
        }
    }

    bail!("no active camera found with type = Positionable and VMCProtocol.mode = Sender");
}

fn setup_reload_initial_camera_settings(
    settings: Res<AppSettings>,
    mut camera_query: Query<(&mut Camera, &mut Transform, &mut Projection), With<VmcCameraMarker>>,
    mut receive_buffer: ResMut<CameraReceiverBuffer>,
    mut initialized: Local<bool>,
    keys: Res<Input<KeyCode>>,
) {
    if !(*initialized) || keys.just_released(KeyCode::R) {
        match read_camera2_info_from_config_files(&settings.camera2_settings_folder) {
            Ok((vmc_camera_info, view_rect)) => {
                info!("read initial camera config: {:?}", &vmc_camera_info);
                receive_buffer.0.push(vmc_camera_info);
                let (mut camera, mut transform, mut projection) = camera_query.single_mut();
                vmc_camera_info.set_camera_settings(&mut transform, &mut projection);
                view_rect.set_camera_settings(&mut camera);
            }
            Err(e) => {
                info!("failed to load initial camera settings: {:?}", e);
                return;
            }
        }
        *initialized = true;
    }
}

// endregion

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
