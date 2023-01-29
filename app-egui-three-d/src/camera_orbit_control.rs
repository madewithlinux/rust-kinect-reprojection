use three_d::*;

use crate::app_settings::AppState;

/// similar to theee-d OrbitControl, but with customizable speed. and encapsulates the camera itself
#[derive(Debug)]
pub struct OrbitCamera {
    pub camera: Camera,
    control: CameraControl,
    drag_speed: f32,
}

impl OrbitCamera {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
        // target: Vec3,
        min_distance: f32,
        max_distance: f32,
    ) -> Self {
        Self {
            camera: Camera::new_perspective(viewport, position, target, up, field_of_view_y, z_near, z_far),
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.05 },
                left_drag_vertical: CameraAction::OrbitUp { target, speed: 0.05 },
                scroll_vertical: CameraAction::Zoom {
                    min: min_distance,
                    max: max_distance,
                    speed: 0.1,
                    target,
                },
                ..Default::default()
            },
            drag_speed: 0.5,
        }
    }

    fn set_orbit_speed(action: &mut CameraAction, new_speed: f32) {
        match action {
            CameraAction::OrbitUp { speed, .. } | CameraAction::OrbitLeft { speed, .. } => {
                *speed = new_speed;
            }
            _ => (),
        }
    }

    fn set_orbit_drag_speed(&mut self, drag_speed: f32) {
        self.drag_speed = drag_speed;
        Self::set_orbit_speed(&mut self.control.left_drag_horizontal, drag_speed);
        Self::set_orbit_speed(&mut self.control.left_drag_vertical, drag_speed);
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, events: &mut [Event]) -> bool {
        if let CameraAction::Zoom {
            speed,
            target,
            min,
            max,
        } = &mut self.control.scroll_vertical
        {
            let x = target.distance(*self.camera.position());
            *speed = 0.5 * smoothstep(*min, *max, x) + 0.001;
        }
        self.control.handle_events(&mut self.camera, events)
    }

    pub fn write_to_state(&self, state: &mut AppState) {
        state.camera_position = *self.camera.position();
        state.camera_target = *self.camera.target();
        state.camera_up = *self.camera.up();
    }

    pub fn read_from_state(&mut self, state: &AppState) {
        self.camera.set_view(
            state.camera_position,
            state.camera_target,
            state.camera_up,
        );
        self.camera.set_perspective_projection(
            degrees(state.camera_fov_deg),
            state.camera_z_near,
            state.camera_z_far,
        );
        set_camera_control_target(&mut self.control, *self.camera.target());
        self.set_orbit_drag_speed(state.orbit_drag_speed);
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}

fn set_camera_control_target(control: &mut CameraControl, new_target: Vec3) {
    set_camera_action_target(&mut control.left_drag_horizontal, new_target);
    set_camera_action_target(&mut control.left_drag_vertical, new_target);
    set_camera_action_target(&mut control.middle_drag_horizontal, new_target);
    set_camera_action_target(&mut control.middle_drag_vertical, new_target);
    set_camera_action_target(&mut control.right_drag_horizontal, new_target);
    set_camera_action_target(&mut control.right_drag_vertical, new_target);
    set_camera_action_target(&mut control.scroll_horizontal, new_target);
    set_camera_action_target(&mut control.scroll_vertical, new_target);
}

fn set_camera_action_target(action: &mut CameraAction, new_target: Vec3) {
    match action {
        CameraAction::OrbitUp { target, .. }
        | CameraAction::OrbitLeft { target, .. }
        | CameraAction::Zoom { target, .. } => *target = new_target,
        _ => (),
    }
}
