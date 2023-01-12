use bevy::{math::Affine3A, prelude::*};
use bevy_prototype_debug_lines::DebugLines;

pub(crate) fn draw_debug_axes(lines: &mut DebugLines, transform: &Affine3A, scale: f32) {
    let origin = transform.transform_point3(Vec3::ZERO);
    let dx = (transform.transform_point3(Vec3::X) - origin).normalize();
    let dy = (transform.transform_point3(Vec3::Y) - origin).normalize();
    let dz = (transform.transform_point3(Vec3::Z) - origin).normalize();
    lines.line_colored(origin, origin + dx * scale, 0.0, Color::RED);
    lines.line_colored(origin, origin + dy * scale, 0.0, Color::GREEN);
    lines.line_colored(origin, origin + dz * scale, 0.0, Color::BLUE);
}
