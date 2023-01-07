use std::any::TypeId;

use bevy::prelude::*;
use bevy_asset::{HandleId, ReflectAsset};
use bevy_inspector_egui::bevy_inspector::hierarchy::{hierarchy_ui, SelectedEntities};
use bevy_inspector_egui::bevy_inspector::{self, ui_for_entities_shared_components, ui_for_entity};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_reflect::TypeRegistry;
use bevy_render::camera::{CameraProjection, Viewport};
use egui::{Pos2, Rect};
use egui_dock::{NodeIndex, Tree};
use egui_gizmo::GizmoMode;

use crate::frame_visualization_util::{update_framebuffer_images, FrameBufferDescriptor, FrameBufferImageHandle};
use crate::receiver::{load_baseline_frame, KinectFrameBuffers};
use crate::{COLOR_HEIGHT, COLOR_WIDTH};

pub struct AppUiDockPlugin;
impl Plugin for AppUiDockPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(bevy_framepace::FramepacePlugin) // reduces input lag
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState::new())
            .add_system_to_stage(CoreStage::PreUpdate, show_ui_system.at_end())
            // .add_startup_system(spawn_2d_camera)
            .add_system(set_camera_viewport)
            .add_system(set_gizmo_mode)
            .add_system(update_framebuffer_images)
            .register_type::<FrameBufferImageHandle>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();
    }
}

#[derive(Component, Reflect)]
pub struct MainCamera;

fn show_ui_system(world: &mut World) {
    let mut egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| ui_state.ui(world, &mut egui_context));
}

// make camera only render to view not obstructed by UI
fn set_camera_viewport(
    ui_state: Res<UiState>,
    windows: Res<Windows>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let mut cam = cameras.single_mut();

    let window = windows.primary();
    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;
    if ui_state.viewport_rect == egui::Rect::NOTHING {
        // the game view tab hasn't been displayed yet
        return;
    }

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}

fn set_gizmo_mode(input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    for (key, mode) in [
        (KeyCode::R, GizmoMode::Rotate),
        (KeyCode::T, GizmoMode::Translate),
        (KeyCode::S, GizmoMode::Scale),
    ] {
        if input.just_pressed(key) {
            ui_state.gizmo_mode = mode;
        }
    }
}

#[derive(Eq, PartialEq)]
enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, HandleId),
}

#[derive(Resource)]
struct UiState {
    tree: Tree<Window>,
    viewport_rect: egui::Rect,
    selected_entities: SelectedEntities,
    selection: InspectorSelection,
    gizmo_mode: GizmoMode,
}

impl UiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![
            Window::GameView,
            Window::FrameBuffer(FrameBufferDescriptor::CurrentColor),
            Window::FrameBuffer(FrameBufferDescriptor::PointCloud),
            // Window::FrameBuffer(FrameBufferDescriptor::ActiveColor),
            // Window::FrameBuffer(FrameBufferDescriptor::ActiveDepth),
            // Window::FrameBuffer(FrameBufferDescriptor::CurrentDepth),
            // Window::FrameBuffer(FrameBufferDescriptor::CurrentPlayerIndex),
            Window::FrameBuffer(FrameBufferDescriptor::DerivedDepth),
            Window::FrameBuffer(FrameBufferDescriptor::DerivedPlayerIndex),
        ]);
        // let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![Window::Inspector]);
        let game = NodeIndex::root();
        let [_game, hierarchy] = tree.split_left(
            game,
            0.2,
            vec![
                Window::World,
                Window::Hierarchy,
                Window::WorldEntities,
                Window::Resources,
                Window::Assets,
                Window::Inspector,
            ],
        );
        let [_bottom, _controls] = tree.split_below(hierarchy, 0.8, vec![Window::Controls]);

        Self {
            tree,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
            gizmo_mode: GizmoMode::Translate,
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
            gizmo_mode: self.gizmo_mode,
        };
        egui_dock::DockArea::new(&mut self.tree).show(ctx, &mut tab_viewer);
    }
}

#[derive(Debug)]
enum Window {
    World,
    WorldEntities,
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
    Controls,
    FrameBuffer(FrameBufferDescriptor),
}

struct TabViewer<'a> {
    world: &'a mut World,
    selected_entities: &'a mut SelectedEntities,
    selection: &'a mut InspectorSelection,
    viewport_rect: &'a mut egui::Rect,
    gizmo_mode: GizmoMode,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Window;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            Window::World => bevy_inspector::ui_for_world(self.world, ui),
            Window::WorldEntities => bevy_inspector::ui_for_world_entities(self.world, ui),
            Window::GameView => {
                (*self.viewport_rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());

                // draw_gizmo(ui, self.world, self.selected_entities, self.gizmo_mode);
            }
            Window::Hierarchy => hierarchy_ui(self.world, ui, self.selected_entities),
            Window::Resources => select_resource(ui, &type_registry, self.selection),
            Window::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            Window::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity(self.world, entity, ui, false),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(self.world, type_id, ui, name, &type_registry)
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(self.world, type_id, handle, ui, &type_registry);
                }
            },
            Window::Controls => ui_controls(ui, self.world),
            Window::FrameBuffer(frame_buffer_name) => {
                ui.label(format!("{:?}", frame_buffer_name));
                let (image_handle, texture_id) = self.world.resource_scope::<bevy_egui::EguiContext, _>(
                    |world, mut egui_context: Mut<bevy_egui::EguiContext>| {
                        get_or_create_frame_buffer_image_handle(world, frame_buffer_name, &mut egui_context)
                    },
                );
                let (width, height) = self.world.resource_scope::<Assets<Image>, _>(|world, images| {
                    let image = images.get(&image_handle).unwrap();
                    let size = &image.texture_descriptor.size;
                    (size.width as f32, size.height as f32)
                });
                let texture_size = Rect::from_points(&[Pos2::ZERO, Pos2::new(width, height)]);
                let available_rect = ui.available_rect_before_wrap();
                let image_size = if texture_size.aspect_ratio() > available_rect.aspect_ratio() {
                    (
                        available_rect.width(),
                        available_rect.width() / texture_size.aspect_ratio(),
                    )
                } else {
                    (
                        available_rect.height() * texture_size.aspect_ratio(),
                        available_rect.height(),
                    )
                };
                ui.image(texture_id, image_size);
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, Window::GameView)
    }
}

fn ui_controls(ui: &mut egui::Ui, world: &mut World) {
    let pool = bevy::tasks::AsyncComputeTaskPool::get();

    ui.vertical(|ui| {
        if ui.button("save depth frame").clicked() {
            let depth_frame = world
                .query::<&crate::receiver::KinectFrameBuffers>()
                .single(world)
                .current_frame
                .depth_frame
                .clone();
            pool.spawn(async move {
                info!("saving depth frame");
                let Some(depth_filename) = rfd::FileDialog::new()
                    .add_filter("png", &["png", "PNG"])
                    .set_title("save depth frame")
                    .set_file_name("kinect_depth_data.png")
                    .save_file() else {
                        info!("file chooser cancelled");
                        return
                    };
                depth_frame.save(&depth_filename).unwrap();
                info!("saved {:?}", &depth_filename);
            })
            .detach();
        }

        if ui.button("open depth baseline image").clicked() {
            if let Some(depth_filename) = rfd::FileDialog::new()
                .add_filter("png", &["png", "PNG"])
                .set_title("open depth baseline image")
                .set_file_name("kinect_depth_data_empty.png")
                .pick_file()
            {
                world
                    .query::<&mut KinectFrameBuffers>()
                    .single_mut(world)
                    .depth_baseline_frame = load_baseline_frame(depth_filename).unwrap();
            }
        }
    });
}

fn get_or_create_frame_buffer_image_handle(
    world: &mut World,
    buffer_name: &FrameBufferDescriptor,
    egui_context: &mut bevy_egui::EguiContext,
) -> (Handle<Image>, egui::TextureId) {
    if let Some(found) = world
        .query::<&FrameBufferImageHandle>()
        .iter(world)
        .find(|&FrameBufferImageHandle(b, _)| b == buffer_name)
    {
        (found.1.clone(), egui_context.add_image(found.1.clone()))
    } else {
        info!("creating image resource for frame buffer {:?}", buffer_name);
        let image_handle = world.resource_mut::<Assets<Image>>().add(Image::new_fill(
            bevy_render::render_resource::Extent3d {
                width: COLOR_WIDTH as u32,
                height: COLOR_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            &[0, 0, 0, 255],
            bevy_render::render_resource::TextureFormat::Rgba8UnormSrgb,
        ));
        world.spawn((
            Name::new(format!("auto:{:?}", buffer_name)),
            FrameBufferImageHandle(*buffer_name, image_handle.clone()),
        ));
        (image_handle.clone(), egui_context.add_image(image_handle))
    }
}

// TODO: remove all the gizmo stuff from here?
fn draw_gizmo(ui: &mut egui::Ui, world: &mut World, selected_entities: &SelectedEntities, gizmo_mode: GizmoMode) {
    let (cam_transform, projection) = world
        .query_filtered::<(&GlobalTransform, &Projection), With<MainCamera>>()
        .single(world);
    let view_matrix = Mat4::from(cam_transform.affine().inverse());
    let projection_matrix = projection.get_projection_matrix();

    if selected_entities.len() != 1 {
        return;
    }

    for selected in selected_entities.iter() {
        let Some(transform) = world.get::<Transform>(selected)
            else { continue };
        let model_matrix = transform.compute_matrix();

        let Some(result) = egui_gizmo::Gizmo::new(selected)
                    .model_matrix(model_matrix.to_cols_array_2d())
                    .view_matrix(view_matrix.to_cols_array_2d())
                    .projection_matrix(projection_matrix.to_cols_array_2d())
                    .orientation(egui_gizmo::GizmoOrientation::Local)
                    .mode(gizmo_mode)
                    .interact(ui)
                else { continue };

        let mut transform = world.get_mut::<Transform>(selected).unwrap();
        *transform = Transform::from_matrix(Mat4::from_cols_array_2d(&result.transform));
    }
}

fn select_resource(ui: &mut egui::Ui, type_registry: &TypeRegistry, selection: &mut InspectorSelection) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| (registration.short_name().to_owned(), registration.type_id()))
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, &resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name);
        }
    }
}

fn select_asset(ui: &mut egui::Ui, type_registry: &TypeRegistry, world: &World, selection: &mut InspectorSelection) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.short_name().to_owned(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let mut handles: Vec<_> = reflect_asset.ids(world).collect();
        handles.sort();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui.selectable_label(selected, format!("{:?}", handle)).clicked() {
                    *selection = InspectorSelection::Asset(asset_type_id, asset_name.clone(), handle);
                }
            }
        });
    }
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}
