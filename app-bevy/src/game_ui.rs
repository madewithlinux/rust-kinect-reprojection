use bevy::prelude::*;
use bevy_inspector_egui::{bevy_inspector, reflect_inspector, DefaultInspectorConfigPlugin};
use iyes_loopless::prelude::*;

use bevy::{
    prelude::*,
    render::{camera::RenderTarget, render_graph::RenderGraph, RenderApp},
    window::{CreateWindow, PresentMode, WindowId},
};
use bevy_egui::EguiContext;
use once_cell::sync::Lazy;

static SECOND_WINDOW_ID: Lazy<WindowId> = Lazy::new(WindowId::new);

use crate::{
    app_settings::AppSettings, camera2_vmc_osc_receiver::VmcCameraMarker,
    frame_visualization_util::FrameBufferImageHandle, receiver::KinectDepthTransformer, MainCamera,
};

pub struct AppUiGamePlugin;
impl Plugin for AppUiGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .add_startup_system(spawn_3d_camera)
            .register_type::<FrameBufferImageHandle>()
            .add_startup_system(setup_create_new_window)
            // .add_system(keyboard_shortcut_create_new_window_system)
            // .add_system_to_stage(CoreStage::PreUpdate, ui_second_window_system.at_end());
            .add_system_to_stage(CoreStage::PreUpdate, ui_second_window_system_exclusive.at_end());
        // .add_system(ui_second_window_system);

        let render_app = app.sub_app_mut(RenderApp);
        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

        bevy_egui::setup_pipeline(
            &mut graph,
            bevy_egui::RenderGraphConfig {
                window_id: *SECOND_WINDOW_ID,
                egui_pass: SECONDARY_EGUI_PASS,
            },
        );
    }
}

fn spawn_3d_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        VmcCameraMarker,
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.5, 3.6, 2.6))
                .looking_at(Vec3::new(0.0, 0.0, -0.8), Vec3::Y),
            ..default()
        },
    ));
}

const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";

// TODO: this doesn't work
fn keyboard_shortcut_create_new_window_system(
    keys: Res<Input<KeyCode>>,
    mut create_window_events: EventWriter<CreateWindow>,
) {
    if !keys.just_pressed(KeyCode::W) {
        return;
    }
    create_window_events.send(CreateWindow {
        id: *SECOND_WINDOW_ID,
        descriptor: WindowDescriptor {
            width: 800.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            title: "Kinect Reprojection Settings".to_string(),
            ..Default::default()
        },
    });
}

fn setup_create_new_window(mut create_window_events: EventWriter<CreateWindow>, mut commands: Commands) {
    // sends out a "CreateWindow" event, which will be received by the windowing backend
    create_window_events.send(CreateWindow {
        id: *SECOND_WINDOW_ID,
        descriptor: WindowDescriptor {
            width: 800.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            title: "Kinect Reprojection Settings".to_string(),
            ..Default::default()
        },
    });
}

#[derive(Default)]
struct UiState {
    // TODO: do I even need any UI state?
}

fn ui_second_window_system_exclusive(world: &mut World) {
    let ctx = {
        let mut egui_context = world.resource_mut::<bevy_egui::EguiContext>();
        match egui_context.try_ctx_for_window_mut(*SECOND_WINDOW_ID) {
            Some(ctx) => ctx.clone(),
            None => return,
        }
    };
    egui::CentralPanel::default().show(&ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // TODO: fix duplicate UI element ID issue
            ui.label("AppSettings");
            bevy_inspector::ui_for_resource::<AppSettings>(world, ui);
            ui.separator();

            ui.label("KinectDepthTransformer");
            bevy_inspector::ui_for_resource::<KinectDepthTransformer>(world, ui);
            ui.separator();
        });
    });
}

// fn ui_second_window_system(
//     world: &mut World,
//     mut egui_context: ResMut<EguiContext>,
//     mut ui_state: Local<UiState>,
//     type_registry: Res<AppTypeRegistry>,
//     mut app_settings: ResMut<AppSettings>,
//     mut kinect_depth_transformer: ResMut<KinectDepthTransformer>,
// ) {
//     let ctx = match egui_context.try_ctx_for_window_mut(*SECOND_WINDOW_ID) {
//         Some(ctx) => ctx,
//         None => return,
//     };
//     let type_registry = &type_registry.0.read();
//     let mut inspector_context = reflect_inspector::Context::default();
//     let mut inspector_ui = reflect_inspector::InspectorUi::new_no_short_circuit(type_registry, &mut inspector_context);
//     // egui::Window::new("Settings Window").vscroll(true).show(ctx, |ui| {
//     egui::CentralPanel::default().show(ctx, |ui| {
//         egui::ScrollArea::vertical().show(ui, |ui| {
//             ui.label("AppSettings");
//             // reflect_inspector::ui_for_value(app_settings.as_mut(), ui, type_registry);
//             // inspector_ui.ui_for_reflect(app_settings.as_mut(), ui);
//             // inspector_ui.ui_for_reflect(app_settings.as_mut(), ui);
//             bevy_inspector::ui_for_value(app_settings.as_mut(), ui, world);
//             ui.separator();

//             // ui.label("KinectDepthTransformer");
//             // // reflect_inspector::ui_for_value(kinect_depth_transformer.as_mut(), ui, type_registry);
//             // inspector_ui.ui_for_reflect(kinect_depth_transformer.as_mut(), ui);
//             // ui.separator();
//         });
//     });
// }
