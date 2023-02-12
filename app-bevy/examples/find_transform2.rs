use bevy::{
    math::{vec3, Affine3A, Mat3A},
    prelude::*,
};
use itertools::{iproduct, Itertools};
use ordered_float::OrderedFloat;

fn main() {
    let openvr_points = [
        vec3(0.123588726, 1.869599, 1.0079391),
        vec3(0.12097657, 1.3045535, 0.08676771),
        vec3(0.13649923, 2.059966, 0.8895519),
    ];
    let depths_in_meters = [1033.7073 / 1_000.0, 2210.162 / 1_000.0, 1044.0 / 1_000.0];

    let kinect_position = openvr_points[0] - (openvr_points[1] - openvr_points[0]).normalize() * depths_in_meters[0];
    dbg!(kinect_position);
}
