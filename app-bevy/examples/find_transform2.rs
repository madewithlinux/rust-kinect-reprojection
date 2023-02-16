use std::f32::consts::PI;

use bevy::{
    math::{vec3, Affine3A, Mat3A},
    prelude::*,
};
use itertools::{iproduct, Itertools};
use ordered_float::OrderedFloat;
use rust_kinect_reprojection_app_bevy::app_settings::KinectTransform;

fn main() {
    let openvr_points = [
        vec3(0.123588726, 1.869599, 1.0079391),
        vec3(0.12097657, 1.3045535, 0.08676771),
        vec3(0.13649923, 2.059966, 0.8895519),
    ];
    let depths_in_meters = [1033.7073 / 1_000.0, 2210.162 / 1_000.0, 1044.0 / 1_000.0];

    let (close_point, close_depth, far_point, far_depth) = if depths_in_meters[0] > depths_in_meters[1] {
        (
            openvr_points[1],
            depths_in_meters[1],
            openvr_points[0],
            depths_in_meters[0],
        )
    } else {
        (
            openvr_points[0],
            depths_in_meters[0],
            openvr_points[1],
            depths_in_meters[1],
        )
    };

    // let kinect_position = openvr_points[0] - (openvr_points[1] - openvr_points[0]).normalize() * depths_in_meters[0];
    let kinect_position0 = far_point + (close_point - far_point).normalize() * far_depth;
    let kinect_position1 = close_point + (close_point - far_point).normalize() * close_depth;
    let kinect_position = (kinect_position0 + kinect_position1) / 2.0;
    dbg!(
        kinect_position0,
        kinect_position1,
        kinect_position0.distance(kinect_position1),
        kinect_position,
    );
    let dir = (kinect_position - close_point).normalize();
    let up = (openvr_points[2] - close_point).normalize();
    let transform = Affine3A::look_to_rh(kinect_position, dir, up);
    let transform = transform.inverse();
    let (scale, rotation, translation) = transform.to_scale_rotation_translation();
    let euler_angles = rotation.to_euler(EulerRot::XYZ);
    dbg!(scale, rotation, translation);
    dbg!(
        euler_angles.0 * 180.0 / PI,
        euler_angles.1 * 180.0 / PI,
        euler_angles.2 * 180.0 / PI,
    );

    let kinect_transform = KinectTransform {
        position: translation,
        euler_rotation: vec3(
            euler_angles.0 * 180.0 / PI,
            euler_angles.1 * 180.0 / PI,
            euler_angles.2 * 180.0 / PI,
        ),
        scale,
        ..Default::default()
    };
    println!("{}", serde_json::to_string_pretty(&kinect_transform).unwrap());
    println!("{}", serde_json::to_string_pretty(&transform).unwrap());
}
