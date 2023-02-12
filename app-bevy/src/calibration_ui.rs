use std::time::Duration;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use egui::{Button, CollapsingHeader, Grid, RichText, ScrollArea, Ui};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::app_settings::AppSettings;
use crate::frame_visualization_util::{update_framebuffer_images, FrameBufferDescriptor, FrameBufferImageHandle};
use crate::gui_common::GuiViewable;
use crate::receiver::{KinectDepthTransformer, KinectFrameBuffers};
use crate::vr_connector::{ControllerButtonEvents, ControllerButtonState, LeftOrRightController, OpenVrPoseData};
use crate::{MainCamera, COLOR_HEIGHT, COLOR_WIDTH};

pub struct AppCalibrationUiPlugin;
impl Plugin for AppCalibrationUiPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState::new())
            .init_resource::<CalibrationUiState>()
            .init_resource::<CalibrationProcedureState>()
            .add_system_to_stage(CoreStage::PreUpdate, show_ui_system.at_end())
            //
            .add_startup_system(set_egui_scale_factor)
            .add_startup_system(spawn_2d_camera)
            .add_system(set_camera_viewport)
            .add_system(update_framebuffer_images)
            //
            .add_startup_system(spawn_rgba_sprite)
            .add_startup_system(spawn_cursor_sprite)
            .add_system(update_cursor_sprite_transform)
            //
            .add_system(update_calibration_ui)
            .add_system(calibration_procedure_system)
            //
            .register_type::<FrameBufferImageHandle>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<RgbaSpriteMarker>()
            .register_type::<CursorImage>()
            .register_type::<CursorPixelPosition>()
            .register_type::<LeftOrRightController>()
            .register_type::<CalibrationSampleParams>()
            .register_type::<CalibrationUiState>()
            .register_type::<CalibrationSample>()
            .register_type::<AlphaMode>();
    }
}

// region: UI boilerplate

fn show_ui_system(world: &mut World) {
    let mut egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| ui_state.ui(world, &mut egui_context));
}

fn spawn_2d_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::Auto {
                    min_width: COLOR_WIDTH as f32,
                    min_height: COLOR_HEIGHT as f32,
                },
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

fn set_egui_scale_factor(mut egui_settings: ResMut<bevy_egui::EguiSettings>, app_settings: Res<AppSettings>) {
    if let Some(scale_factor) = app_settings.gui_scale_factor_override {
        egui_settings.scale_factor = scale_factor;
    }
}

// make camera only render to view not obstructed by UI
// TODO: is this still exactly the same as the one in dock_ui.rs?
fn set_camera_viewport(
    ui_state: Res<UiState>,
    windows: Res<Windows>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut camera_query: Query<&mut Camera, With<MainCamera>>,
) {
    let mut camera = camera_query.single_mut();

    let window = windows.primary();
    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;
    if ui_state.viewport_rect == egui::Rect::NOTHING {
        // the game view tab hasn't been displayed yet
        return;
    }

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}

#[derive(Resource)]
struct UiState {
    viewport_rect: egui::Rect,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    fn edit_window_scale_factor(&mut self, world: &mut World, ui: &mut Ui) {
        let mut egui_settings = world.resource_mut::<bevy_egui::EguiSettings>();

        let selected = &mut egui_settings.scale_factor;
        egui::ComboBox::from_label("GUI scale")
            .selected_text(format!("{:?}", selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(selected, 0.5, "0.5");
                ui.selectable_value(selected, 0.75, "0.75");
                ui.selectable_value(selected, 1.0, "1.0");
                ui.selectable_value(selected, 1.25, "1.25");
                ui.selectable_value(selected, 1.5, "1.5");
                ui.selectable_value(selected, 1.75, "1.75");
                ui.selectable_value(selected, 2.0, "2.0");
                ui.selectable_value(selected, 2.5, "2.5");
                ui.selectable_value(selected, 3.0, "3.0");
                ui.selectable_value(selected, 3.5, "3.5");
                ui.selectable_value(selected, 4.0, "4.0");
            });
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(256.0)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Calibration UI");
                    self.edit_window_scale_factor(world, ui);
                    ui.separator();

                    world.resource_scope::<CalibrationUiState, _>(|world, mut cal_ui_state| {
                        cal_ui_state.ui(world, ui);
                    });
                    ui.separator();

                    // for pos in &[
                    //     vec2(40.0, 40.0),
                    //     vec2(0.0, 0.0),
                    //     vec2(300.0, 40.0),
                    //     vec2(300.0, 0.0),
                    //     vec2((COLOR_WIDTH / 2) as f32, (COLOR_HEIGHT / 2) as f32),
                    //     vec2(COLOR_WIDTH as f32, COLOR_HEIGHT as f32),
                    // ] {
                    //     if ui.button(format!("cursor to {:?}", pos)).clicked() {
                    //         world.query::<&mut CursorPixelPosition>().single_mut(world).0 = *pos;
                    //     }
                    // }
                    // ui.separator();

                    CollapsingHeader::new("inspector").default_open(false).show(ui, |ui| {
                        bevy_inspector::ui_for_world(world, ui);
                    });
                });
            });

        self.viewport_rect = ctx.available_rect();
    }
}

// endregion

// region: rgba sprite
#[derive(Component, Reflect, Debug)]
struct RgbaSpriteMarker;

fn spawn_rgba_sprite(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let color_image_handle = images.add(Image::new_fill(
        Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[255, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));

    commands.spawn((
        Name::new("color image"),
        RgbaSpriteMarker,
        FrameBufferImageHandle(FrameBufferDescriptor::Rgba, color_image_handle.clone()),
        SpriteBundle {
            sprite: Sprite {
                // TODO: is this the right place to flip the x axis? or should we flip it as part of the transform of the camera, or something?
                // flip_x: true,
                ..default()
            },
            texture: color_image_handle,
            ..default()
        },
    ));
}

// endregion

// region: cursor sprite

#[derive(Resource, Reflect, Debug)]
struct CursorImage(Handle<Image>);

#[derive(Component, Reflect, Debug)]
pub struct CursorPixelPosition(Vec2);

impl CursorPixelPosition {
    pub fn to_usize_pair(&self) -> (usize, usize) {
        (self.0.x.round() as usize, self.0.y.round() as usize)
    }
}

impl CursorPixelPosition {
    pub fn from_coordinate(i: usize, j: usize) -> Self {
        Self(vec2(i as f32, j as f32))
    }
}
impl From<(usize, usize)> for CursorPixelPosition {
    fn from((i, j): (usize, usize)) -> Self {
        Self(vec2(i as f32, j as f32))
    }
}
impl From<(u32, u32)> for CursorPixelPosition {
    fn from((i, j): (u32, u32)) -> Self {
        Self(vec2(i as f32, j as f32))
    }
}
impl From<Vec2> for CursorPixelPosition {
    fn from(pos: Vec2) -> Self {
        Self(pos)
    }
}

fn spawn_cursor_sprite(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    #[rustfmt::skip]
    let cursor_mask = vec![
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
       255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0, 255,   0,   0,   0,   0,   0,   0,   0,
    ];
    let cursor_size = (cursor_mask.len() as f32).sqrt() as u32;
    assert_eq!(cursor_size * cursor_size, cursor_mask.len() as u32);
    let cursor_pixels: Vec<u8> = cursor_mask
        .iter()
        .map(|m| match m {
            255 => [255, 255, 255, 255],
            0 => [0, 0, 0, 0],
            _ => panic!(),
        })
        .flatten()
        .collect_vec();

    let mut image = Image::new(
        Extent3d {
            width: cursor_size,
            height: cursor_size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        cursor_pixels,
        TextureFormat::Rgba8UnormSrgb,
    );
    image.sampler_descriptor = ImageSampler::nearest();
    let cursor_image = images.add(image);
    commands.insert_resource(CursorImage(cursor_image.clone()));

    commands.spawn((
        Name::new("cursor"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            texture: cursor_image,
            ..default()
        },
        CursorPixelPosition::from_coordinate(COLOR_WIDTH / 2, COLOR_HEIGHT / 2),
    ));
}

fn update_cursor_sprite_transform(mut cursor_query: Query<(&CursorPixelPosition, &mut Transform)>) {
    let middle = vec2((COLOR_WIDTH / 2) as f32, (COLOR_HEIGHT / 2) as f32);
    for (CursorPixelPosition(pixel_position), mut transform) in cursor_query.iter_mut() {
        // flip y-axis
        let sprite_space_pos = (*pixel_position - middle) * vec2(1.0, -1.0);
        *transform = Transform::from_translation(sprite_space_pos.extend(1.0));
    }
}

// endregion

// region: calibration UI

#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationSampleParams {
    pub wait_before_sample_ms: u64,
    pub subsample_time_ms: u64,
    pub depth_sample_radius: f32,
}
impl Default for CalibrationSampleParams {
    fn default() -> Self {
        Self {
            wait_before_sample_ms: 100,
            subsample_time_ms: 650,
            depth_sample_radius: 0.0,
        }
    }
}

#[derive(Reflect, Debug, Default, Clone, Serialize, Deserialize)]
#[reflect(Default)]
pub struct CalibrationSample {
    pub pixel_position: (usize, usize),
    pub depth_mm: f32,
    pub openvr_position: Vec3,
    pub depth_sample_count: usize,
    pub openvr_position_sample_count: usize,
}
impl CalibrationSample {
    pub fn from_subsamples(
        pixel_position: (usize, usize),
        depth_subsamples: &[f32],
        openvr_position_subsamples: &[Vec3],
    ) -> Self {
        let depth_sample_count = depth_subsamples.len();
        let depth_mm = depth_subsamples.iter().sum::<f32>() / (depth_sample_count as f32);
        let openvr_position_sample_count = openvr_position_subsamples.len();
        let openvr_position = openvr_position_subsamples.iter().sum::<Vec3>() / (openvr_position_sample_count as f32);
        Self {
            pixel_position,
            depth_mm,
            openvr_position,
            depth_sample_count,
            openvr_position_sample_count,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
pub struct CalibrationUiState {
    pub left_or_right_controller: LeftOrRightController,
    pub sample_params: CalibrationSampleParams,
    pub points_to_sample: Vec<(usize, usize)>,
    pub controller_state: ControllerButtonState,
    pub controller_button_events: ControllerButtonEvents,
    pub controller_position: Vec3,
}
impl FromWorld for CalibrationUiState {
    fn from_world(world: &mut World) -> Self {
        Self {
            left_or_right_controller: default(),
            sample_params: default(),
            // current_step: default(),
            points_to_sample: vec![
                (COLOR_WIDTH / 2, COLOR_HEIGHT / 2),
                (COLOR_WIDTH / 2, COLOR_HEIGHT / 2),
                (COLOR_WIDTH / 2, COLOR_HEIGHT / 4),
            ],
            controller_state: default(),
            controller_button_events: default(),
            controller_position: default(),
        }
    }
}
impl CalibrationUiState {
    fn ui(&mut self, world: &mut World, ui: &mut Ui) {
        Grid::new("calibration ui grid").striped(true).show(ui, |ui| {
            ui.label("controller");
            bevy_inspector::ui_for_value(&mut self.left_or_right_controller, ui, world);
            ui.end_row();
            ui.label("controller position");
            self.controller_position.gui_view(ui);
            ui.end_row();

            let is_initial_state = world.resource::<CalibrationProcedureState>().current_step == ProcedureStep::Idle;
            if ui.add_enabled(is_initial_state, Button::new("start")).clicked() {
                self.start_calibration_procedure(world);
            };
            if ui.button("reset and restart").clicked() {
                self.reset_calibration_procedure(world);
            };

            // ui.label("current step");
            // ui.label(format!("{:?}", self.current_step));
            // ui.end_row();

            // ui.label("sample params");
            // bevy_inspector::ui_for_value(&mut self.sample_params, ui, world);
            // ui.end_row();

            // ui.label("points to sample");
            // bevy_inspector::ui_for_value(&mut self.points_to_sample, ui, world);
            // ui.end_row();
        });

        CollapsingHeader::new("calibration procedure")
            .default_open(true)
            .show(ui, |ui| {
                let procedure_state = world.resource::<CalibrationProcedureState>();
                procedure_state.gui_view(ui);
            });

        CollapsingHeader::new("controller state")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("controller state grid").striped(true).show(ui, |ui| {
                    ui.label("position");
                    self.controller_position.gui_view(ui);
                    ui.end_row();

                    let button_constants = [
                        (self.controller_state.menu_pressed, "menu"),
                        (self.controller_state.back_pressed, "back"),
                        (self.controller_state.trigger_pressed, "trigger"),
                        (self.controller_state.touchpad_pressed, "touchpad"),
                    ];
                    for (pressed, name) in button_constants.iter() {
                        ui.label(*name);
                        ui.label(if *pressed { "pressed" } else { "" });
                        ui.end_row();
                    }
                });
                // ui.label("state");
                // bevy_inspector::ui_for_value(&mut self.controller_state, ui, world);
                ui.label("events");
                bevy_inspector::ui_for_value(&mut self.controller_button_events, ui, world);
                // ui.label("position");
                // bevy_inspector::ui_for_value(&mut self.controller_position, ui, world);
            });
    }

    fn start_calibration_procedure(&mut self, world: &mut World) {
        let mut procedure_state = world.resource_mut::<CalibrationProcedureState>();
        *procedure_state = default();
        procedure_state.left_or_right_controller = self.left_or_right_controller;
        procedure_state.sample_params = self.sample_params.clone();
        procedure_state.remaining_points_to_sample = self.points_to_sample.clone();
        procedure_state.current_step = ProcedureStep::WaitForTriggerRelease;
    }

    fn reset_calibration_procedure(&mut self, world: &mut World) {
        let mut procedure_state = world.resource_mut::<CalibrationProcedureState>();
        *procedure_state = default();
    }
}

fn update_calibration_ui(
    // buffers: Res<KinectFrameBuffers>,
    // depth_transformer: Res<KinectDepthTransformer>,
    // frame_delay_buffer: Res<KinectFrameDataDelayBufferV2>,
    // settings: Res<AppSettings>,
    vr_pose_data: Res<OpenVrPoseData>,
    mut cal_ui_state: ResMut<CalibrationUiState>,
) {
    let (controller_pose, controller_state, controller_events) =
        vr_pose_data.get_controller_data(cal_ui_state.left_or_right_controller);
    cal_ui_state.controller_state = *controller_state;
    cal_ui_state.controller_position = controller_pose.position;
    cal_ui_state.controller_button_events = controller_events.clone();
}

#[derive(Reflect, Debug, Default, Copy, Clone, PartialEq)]
pub enum ProcedureStep {
    #[default]
    Idle,
    WaitForTriggerRelease,
    WaitForTriggerPress,
    WaitBeforeSample(u64),
    Sampling {
        pixel_position: (usize, usize),
        ms: u64,
    },
    Done,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct CalibrationProcedureState {
    pub left_or_right_controller: LeftOrRightController,
    pub sample_params: CalibrationSampleParams,
    pub current_step: ProcedureStep,
    //
    pub remaining_points_to_sample: Vec<(usize, usize)>,
    pub samples: Vec<CalibrationSample>,
    //
    pub subsamples_openvr_position: Vec<Vec3>,
    pub subsamples_depth: Vec<f32>,
}

fn calibration_procedure_system(
    time: Res<Time>,
    buffers: Res<KinectFrameBuffers>,
    depth_transformer: Res<KinectDepthTransformer>,
    // frame_delay_buffer: Res<KinectFrameDataDelayBufferV2>,
    // settings: Res<AppSettings>,
    vr_pose_data: Res<OpenVrPoseData>,
    mut cursor_query: Query<&mut CursorPixelPosition>,
    // cal_ui_state: Res<CalibrationUiState>,
    mut state: ResMut<CalibrationProcedureState>,
) {
    // gather info for state logic
    let time_delta_ms = time.delta().as_millis() as u64;
    let (controller_pose, controller_state, controller_events) =
        vr_pose_data.get_controller_data(state.left_or_right_controller);
    let cursor_pos = cursor_query.single().to_usize_pair();
    // TODO: subsample in a radius around cursor_pos
    let cursor_depth = buffers.depth[depth_transformer.ij_to_flat_index(cursor_pos.0, cursor_pos.1)];

    // advance state
    let current_step = state.current_step;
    let next_step = match current_step {
        ProcedureStep::Idle => ProcedureStep::Idle,
        // _ if state.remaining_points_to_sample.is_empty() => ProcedureStep::Done,
        ProcedureStep::WaitForTriggerRelease if controller_state.trigger_pressed => {
            ProcedureStep::WaitForTriggerRelease
        }
        ProcedureStep::WaitForTriggerRelease if state.remaining_points_to_sample.is_empty() => ProcedureStep::Done,
        ProcedureStep::WaitForTriggerRelease if !controller_state.trigger_pressed => ProcedureStep::WaitForTriggerPress,
        ProcedureStep::WaitForTriggerPress if !controller_events.trigger_just_pressed => {
            let pixel_position = state.remaining_points_to_sample[0];
            *cursor_query.single_mut() = CursorPixelPosition::from(pixel_position);
            ProcedureStep::WaitForTriggerPress
        }
        ProcedureStep::WaitForTriggerPress if controller_events.trigger_just_pressed => {
            state.subsamples_depth.clear();
            state.subsamples_openvr_position.clear();
            ProcedureStep::WaitBeforeSample(0)
        }
        ProcedureStep::WaitBeforeSample(ms) if ms < state.sample_params.wait_before_sample_ms => {
            ProcedureStep::WaitBeforeSample(ms + time_delta_ms)
        }
        ProcedureStep::WaitBeforeSample(ms) if ms >= state.sample_params.wait_before_sample_ms => {
            ProcedureStep::Sampling {
                pixel_position: cursor_pos,
                ms: 0,
            }
        }
        ProcedureStep::Sampling { pixel_position, ms } if ms < state.sample_params.subsample_time_ms => {
            state.subsamples_openvr_position.push(controller_pose.position);
            // TODO: only sample depth if it has changed since last subsample?
            if cursor_depth > 0 {
                state.subsamples_depth.push(cursor_depth as f32);
            }
            ProcedureStep::Sampling {
                pixel_position,
                ms: ms + time_delta_ms,
            }
        }
        ProcedureStep::Sampling { pixel_position, ms } if ms >= state.sample_params.subsample_time_ms => {
            assert_eq!(state.remaining_points_to_sample[0], pixel_position);
            let sample = CalibrationSample::from_subsamples(
                pixel_position,
                &state.subsamples_depth,
                &state.subsamples_openvr_position,
            );
            state.samples.push(sample);
            state.remaining_points_to_sample.remove(0);
            ProcedureStep::WaitForTriggerRelease
        }
        o => o,
    };
    state.current_step = next_step;
}

// endregion

// region: GuiViewable impls
impl GuiViewable for LeftOrRightController {
    fn gui_view(&self, ui: &mut Ui) -> egui::Response {
        ui.label(format!("{:?}", self))
    }
}
impl GuiViewable for ProcedureStep {
    fn gui_view(&self, ui: &mut Ui) -> egui::Response {
        ui.label(format!("{:?}", self))
    }
}
impl GuiViewable for CalibrationSample {
    fn gui_view(&self, ui: &mut Ui) -> egui::Response {
        ui.label(format!("{:?}", self))
    }
}
impl GuiViewable for CalibrationProcedureState {
    fn gui_view(&self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            Grid::new("controller state grid")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("left_or_right_controller");
                    self.left_or_right_controller.gui_view(ui);
                    ui.end_row();

                    ui.label("current_step");
                    self.current_step.gui_view(ui);
                    ui.end_row();

                    ui.label("sample_params");
                    ui.end_row();
                    ui.label("wait_before_sample_ms");
                    self.sample_params.wait_before_sample_ms.gui_view(ui);
                    ui.end_row();
                    ui.label("subsample_time_ms");
                    self.sample_params.subsample_time_ms.gui_view(ui);
                    ui.end_row();
                    ui.label("depth_sample_radius");
                    self.sample_params.depth_sample_radius.gui_view(ui);
                    ui.end_row();
                    ui.separator();
                    ui.end_row();

                    ui.label("remaining_points_to_sample");
                    if self.remaining_points_to_sample.is_empty() {
                        ui.label("none");
                        ui.end_row();
                    } else {
                        for point in self.remaining_points_to_sample.iter() {
                            point.gui_view(ui);
                            ui.end_row();
                        }
                    }
                    ui.separator();
                    ui.end_row();

                    ui.label("subsamples_openvr_position.len()");
                    self.subsamples_openvr_position.len().gui_view(ui);
                    ui.end_row();
                    ui.label("subsamples_depth.len()");
                    self.subsamples_depth.len().gui_view(ui);
                    ui.end_row();
                });

            ui.label("samples");
            let mut samples_pretty_json = serde_json::to_string_pretty(&self.samples).unwrap();
            ui.text_edit_multiline(&mut samples_pretty_json);
            // Grid::new("samples grid").striped(true).num_columns(5).show(ui, |ui| {
            //     for sample in self.samples.iter() {
            //         sample.pixel_position.gui_view(ui);
            //         sample.depth_mm.gui_view(ui);
            //         sample.openvr_position.gui_view(ui);
            //         sample.depth_sample_count.gui_view(ui);
            //         sample.openvr_position_sample_count.gui_view(ui);
            //     }
            // });
        })
        .response
    }
}

// endregion
