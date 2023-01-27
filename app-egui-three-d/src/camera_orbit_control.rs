use three_d::*;

/// similar to theee-d OrbitControl, but with customizable speed
#[derive(Debug)]
pub struct CameraOrbitControl {
    control: CameraControl,
    drag_speed: f32,
}

impl CameraOrbitControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.5 },
                left_drag_vertical: CameraAction::OrbitUp { target, speed: 0.5 },
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
    pub fn get_orbit_drag_speed(&self) -> f32 {
        self.drag_speed
    }
    pub fn set_orbit_drag_speed(&mut self, drag_speed: f32) {
        self.drag_speed = drag_speed;
        Self::set_orbit_speed(&mut self.control.left_drag_horizontal, drag_speed);
        Self::set_orbit_speed(&mut self.control.left_drag_vertical, drag_speed);
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        if let CameraAction::Zoom {
            speed,
            target,
            min,
            max,
        } = &mut self.control.scroll_vertical
        {
            let x = target.distance(*camera.position());
            *speed = 0.5 * smoothstep(*min, *max, x) + 0.001;
        }
        self.control.handle_events(camera, events)
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}
