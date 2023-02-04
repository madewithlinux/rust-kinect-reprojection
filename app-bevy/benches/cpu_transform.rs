use std::f32::consts::PI;

use bevy::{math::Affine3A, prelude::*};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rust_kinect_reprojection_app_bevy::{DEPTH_HEIGHT, DEPTH_WIDTH};

// TODO: put this stuff in AppSettings as well?
#[derive(Resource, Debug, Default, Clone, Reflect)]
#[reflect(Debug, Resource)]
pub struct KinectDepthTransformer {
    pub pixel_width: usize,
    pub width: f32,
    pub height: f32,
    pub kinect_position: Vec3,
    pub kinect_rot_deg: Vec3,
    pub kinect_scale: Vec3,
    pub point_transform_matrix: Affine3A,
    point_transform_matrix_inverse: Affine3A,
    pub point_cloud_skel: bool,
}
fn update_depth_transformer(kdt: &mut KinectDepthTransformer) {
    kdt.point_transform_matrix = Affine3A::from_scale_rotation_translation(
        kdt.kinect_scale,
        Quat::from_euler(
            EulerRot::XYZ,
            kdt.kinect_rot_deg.x * PI / 180.0,
            kdt.kinect_rot_deg.y * PI / 180.0,
            kdt.kinect_rot_deg.z * PI / 180.0,
        ),
        kdt.kinect_position,
    );
    kdt.point_transform_matrix_inverse = kdt.point_transform_matrix.inverse();
}
impl KinectDepthTransformer {
    pub fn new() -> Self {
        Self {
            pixel_width: DEPTH_WIDTH,
            width: DEPTH_WIDTH as f32,
            height: DEPTH_HEIGHT as f32,
            kinect_position: Vec3::new(0.18, 2.4273, 1.9451),
            kinect_rot_deg: Vec3::new(-33.0, 180.0, 0.0),
            kinect_scale: Vec3::new(1.0, 1.0, 1.0),
            point_transform_matrix: Affine3A::IDENTITY,
            point_transform_matrix_inverse: Affine3A::IDENTITY,
            point_cloud_skel: true,
        }
    }

    pub fn flat_index_to_ij(&self, flat_index: usize) -> (usize, usize) {
        (flat_index % self.pixel_width, flat_index / self.pixel_width)
    }
    pub fn ij_to_flat_index(&self, i: usize, j: usize) -> usize {
        i + j * self.pixel_width
    }

    pub fn index_depth_to_world(&self, flat_index: usize, depth_in_mm: u16) -> Vec3 {
        let (i, j) = self.flat_index_to_ij(flat_index);
        self.coordinate_depth_to_world(i, j, depth_in_mm)
    }
    pub fn coordinate_depth_to_world(&self, i: usize, j: usize, depth_in_mm: u16) -> Vec3 {
        let i = i as f32;
        let j = self.height - 1.0 - (j as f32);
        // ref https://openkinect.org/wiki/Imaging_Information
        let z = depth_in_mm as f32;
        let min_distance = -10.0;
        let scale_factor = 0.0021;
        let x = (i - self.width / 2.0) * (z + min_distance) * scale_factor;
        let y = (j - self.height / 2.0) * (z + min_distance) * scale_factor;
        let world_point = Vec3::new(x, y, z) / 1_000.0;
        let world_point = self.point_transform_matrix.transform_point3(world_point);
        world_point
    }
}

// pub fn transform_points((depths, points): (&[u16], &mut [Vec3])) {
pub fn transform_points(depths: &[u16], points: &mut [Vec3]) {
    let mut kdt = KinectDepthTransformer::new();
    update_depth_transformer(&mut kdt);

    // for (flat_index, (depth, point)) in depths.iter().zip(points.iter_mut()).enumerate() {
    //     let x = i % DEPTH_WIDTH;
    //     let y = i / DEPTH_WIDTH;
    //     *point = kdt.coordinate_depth_to_world(x, y, *depth);
    // }
    for (flat_index, depth) in depths.iter().enumerate() {
        // if *depth > 10000 {
        //     continue;
        // }
        let pt = kdt.index_depth_to_world(flat_index, *depth);
        let x = ((pt.x + 0.5) * (DEPTH_WIDTH as f32)).round() as usize;
        let y = ((pt.y + 0.5) * (DEPTH_HEIGHT as f32)).round() as usize;
        let out_index = y * DEPTH_WIDTH + x;
        if out_index < points.len() && points[out_index] == Vec3::ZERO {
            points[out_index] = pt;
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // let kdt = KinectDepthTransformer::new();
    // update_depth_transformer(&mut kdt);

    c.bench_function("transform points", |b| {
        b.iter_batched_ref(
            || {
                let mut rng = rand::thread_rng();
                let mut points = vec![12u16; DEPTH_HEIGHT * DEPTH_WIDTH];
                points.fill_with(|| rng.gen());
                let depths = vec![Vec3::ZERO; DEPTH_HEIGHT * DEPTH_WIDTH];
                (points, depths)
            },
            |(points, depths)| {
                transform_points(black_box(points), black_box(depths));
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
