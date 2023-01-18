use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy::{
    prelude::*,
    render::{camera::RenderTarget, render_graph::RenderGraph, RenderApp},
    window::{CreateWindow, PresentMode, WindowId},
};
use bevy_egui::EguiContext;
use once_cell::sync::Lazy;

static SECOND_WINDOW_ID: Lazy<WindowId> = Lazy::new(WindowId::new);

use crate::{camera2_vmc_osc_receiver::VmcCameraMarker, frame_visualization_util::FrameBufferImageHandle, MainCamera};

pub struct AppUiGamePlugin;
impl Plugin for AppUiGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(bevy_egui::EguiPlugin)
            .add_startup_system(spawn_3d_camera)
            .register_type::<FrameBufferImageHandle>()
            .add_startup_system(create_new_window_system)
            .add_system(ui_second_window_system);

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

fn create_new_window_system(mut create_window_events: EventWriter<CreateWindow>, mut commands: Commands) {
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
    // second window camera
    // TODO: figure out how to make it not render the rest of the game world at all
    commands.spawn(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Window(*SECOND_WINDOW_ID),
            ..Default::default()
        },
        transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

#[derive(Default)]
struct UiState {
    input: String,
}

// TODO: show UI stuff that's actually useful
// TODO: make the egui UI take up the whole window
fn ui_second_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    // mut shared_ui_state: ResMut<SharedUiState>,
) {
    // let bevy_texture_id = egui_context.add_image(images.bevy_icon.clone_weak());
    let ctx = match egui_context.try_ctx_for_window_mut(*SECOND_WINDOW_ID) {
        Some(ctx) => ctx,
        None => return,
    };
    egui::Window::new("Second Window").vscroll(true).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Write something else: ");
            ui.text_edit_singleline(&mut ui_state.input);
        });
        // ui.horizontal(|ui| {
        //     ui.label("Shared input: ");
        //     ui.text_edit_singleline(&mut shared_ui_state.shared_input);
        // });

        // ui.add(egui::widgets::Image::new(bevy_texture_id, [256.0, 256.0]));
    });
}
