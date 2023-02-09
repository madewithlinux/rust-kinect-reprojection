use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use egui::ScrollArea;

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
            .add_startup_system(spawn_2d_camera)
            .add_system(set_camera_viewport)
            .add_system(update_framebuffer_images)
            .add_startup_system(spawn_sprite)
            // .add_system(update_sprite_transform)
            .register_type::<FrameBufferImageHandle>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();
    }
}

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

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(128.0)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Hello World!");
                    ui.separator();
                    bevy_inspector::ui_for_world(world, ui);
                    // bevy_inspector::ui_for_world_entities(world, ui);
                });
            });

        self.viewport_rect = ctx.available_rect();
    }
}

#[derive(Component, Reflect, Debug)]
struct RgbaSpriteMarker;

fn spawn_sprite(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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
        Name::new("color image handle"),
        RgbaSpriteMarker,
        FrameBufferImageHandle(FrameBufferDescriptor::Rgba, color_image_handle.clone()),
        SpriteBundle {
            texture: color_image_handle,
            ..default()
        },
    ));
}

// fn update_sprite_transform(
//     cameras: Query<&mut Camera, With<MainCamera>>,
//     mut sprite_query: Query<&mut Transform, With<RgbaSpriteMarker>>,
//     windows: Res<Windows>,
// ) {
//     let window = windows.primary();
//     let scale_factor = window.scale_factor() as f32;

//     let cam = cameras.single();
//     let physical_size = match &cam.viewport {
//         Some(vp) => vp.physical_size,
//         None => return,
//     };
//     let viewport_width = (physical_size.x as f32) / scale_factor;
//     // let viewport_height = (physical_size.y as f32) / scale_factor;

//     for mut transform in sprite_query.iter_mut() {
//         *transform = Transform::from_scale(Vec3::splat(viewport_width)).with_translation(Vec3::ZERO);
//     }
// }
