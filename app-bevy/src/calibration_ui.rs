use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use egui::{Button, RichText, ScrollArea, Ui};
use itertools::Itertools;

use crate::app_settings::AppSettings;
use crate::frame_visualization_util::{update_framebuffer_images, FrameBufferDescriptor, FrameBufferImageHandle};
use crate::{MainCamera, COLOR_HEIGHT, COLOR_WIDTH};

pub struct AppCalibrationUiPlugin;
impl Plugin for AppCalibrationUiPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState::new())
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
            .register_type::<FrameBufferImageHandle>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<RgbaSpriteMarker>()
            .register_type::<CursorImage>()
            .register_type::<CursorPixelPosition>()
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
                    self.edit_window_scale_factor(world, ui);
                    ui.separator();

                    for pos in &[
                        vec2(40.0, 40.0),
                        vec2(0.0, 0.0),
                        vec2(300.0, 40.0),
                        vec2(300.0, 0.0),
                        vec2((COLOR_WIDTH / 2) as f32, (COLOR_HEIGHT / 2) as f32),
                        vec2(COLOR_WIDTH as f32, COLOR_HEIGHT as f32),
                    ] {
                        if ui.button(format!("cursor to {:?}", pos)).clicked() {
                            world.query::<&mut CursorPixelPosition>().single_mut(world).0 = *pos;
                        }
                    }

                    ui.separator();
                    bevy_inspector::ui_for_world(world, ui);
                    // bevy_inspector::ui_for_world_entities(world, ui);
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
                flip_x: true,
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

// region: UI utility functions

// endregion
