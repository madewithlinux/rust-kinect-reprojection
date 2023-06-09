use std::io::{Read, Write};

#[cfg(feature = "debug_helpers")]
use bevy::{math::Affine3A, prelude::*};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "debug_helpers")]
pub(crate) fn draw_debug_axes(lines: &mut bevy_prototype_debug_lines::DebugLines, transform: &Affine3A, scale: f32) {
    let origin = transform.transform_point3(Vec3::ZERO);
    let dx = (transform.transform_point3(Vec3::X) - origin).normalize();
    let dy = (transform.transform_point3(Vec3::Y) - origin).normalize();
    let dz = (transform.transform_point3(Vec3::Z) - origin).normalize();
    lines.line_colored(origin, origin + dx * scale, 0.0, Color::RED);
    lines.line_colored(origin, origin + dy * scale, 0.0, Color::GREEN);
    lines.line_colored(origin, origin + dz * scale, 0.0, Color::BLUE);
}

pub fn write_to_pretty_json_file<T: Serialize>(value: &T, file_path: impl AsRef<std::path::Path>) {
    let s = serde_json::to_string_pretty(value).unwrap();
    std::fs::File::create(file_path)
        .unwrap()
        .write_all(s.as_bytes())
        .unwrap();
}

pub fn write_to_json_file<T: Serialize>(value: &T, file_path: impl AsRef<std::path::Path>) {
    let s = serde_json::to_string(value).unwrap();
    std::fs::File::create(file_path)
        .unwrap()
        .write_all(s.as_bytes())
        .unwrap();
}

pub fn read_from_json_file<T: DeserializeOwned>(file_path: impl AsRef<std::path::Path>) -> T {
    let mut s = String::new();
    std::fs::File::open(&file_path).unwrap().read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

pub fn try_read_from_json_file<T: DeserializeOwned>(file_path: impl AsRef<std::path::Path>) -> anyhow::Result<T> {
    let mut s = String::new();
    std::fs::File::open(&file_path)?.read_to_string(&mut s)?;
    let t = serde_json::from_str(&s)?;
    Ok(t)
}
