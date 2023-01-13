use bevy::{
    math::{Affine3A, Mat3A},
    prelude::*,
};
use itertools::{iproduct, Itertools};
use ordered_float::OrderedFloat;

fn main() {
    let point_matches = [
        (
            Vec3::new(1.2902215, -0.021568049, -0.59659046),
            (136, 346),
            Vec3::new(-1.1512339, -0.67056423, 3.39),
        ),
        (
            Vec3::new(-1.4719752, 0.45078325, -0.96842957),
            (570, 278),
            Vec3::new(1.704511, -0.23467918, 3.528),
        ),
        (
            Vec3::new(1.1872808, 1.5832841, -0.95948),
            (134, 89),
            Vec3::new(-1.001785, 0.83905846, 2.9050002),
        ),
    ];

    // normal formula: y = A*x + b
    // or just y = A*x when A is an augmented matrix
    // so then, A = y*inv(x)

    // let y = Mat4::from_cols(
    //     point_matches[0].0.extend(1.0),
    //     point_matches[1].0.extend(1.0),
    //     point_matches[2].0.extend(1.0),
    //     Vec4::new(0.0, 0.0, 0.0, 0.0),
    // );
    // let x = Mat4::from_cols(
    //     point_matches[0].2.extend(1.0),
    //     point_matches[1].2.extend(1.0),
    //     point_matches[2].2.extend(1.0),
    //     Vec4::new(0.0, 0.0, 0.0, 0.0),
    // );
    // let y = y.transpose();
    // let x = x.transpose();

    // let x_inv = x.inverse();
    // println!("x_inv: {:?}", x_inv);
    // println!("y*x_inv:{:?}", y * x_inv);

    let vr_points = point_matches.map(|t| t.0);
    let kinect_points = point_matches.map(|t| t.2);
    let translation = center_of_mass(&vr_points) - center_of_mass(&kinect_points);
    let translated = kinect_points.map(|p| p + translation);
    println!("before: {:?}", sum_suqares_dist(&vr_points, &kinect_points));
    println!("after translation:  {:?}", sum_suqares_dist(&vr_points, &translated));

    brute_force_transform(&vr_points, &kinect_points);

    // let x = Mat3::from_cols(translated[0], translated[1], translated[2]);
    // let x_inv = x;
    // // let x_inv = x_inv.transpose();
    // let x_inv = x_inv.inverse();
    // // let x_inv = x_inv.transpose();

    // // println!("inverse:  {:?}", x_inv);
    // let full_transform = Affine3A::from_mat3_translation(x_inv, translation * -1.0);
    // // println!("full_transform: {:?}", full_transform);
    // let transformed = kinect_points.map(|p| full_transform.transform_point3(p));
    // println!("after full transform: {:?}", sum_suqares_dist(&vr_points, &transformed));

    // // for (vr_point, _, kinect_point) in point_matches.iter() {
    // //     let p2 = full_transform.transform_point3(*kinect_point);
    // //     println!("{:?}, {:?} => {:?}", vr_point, kinect_point, p2);
    // // }
    // // println!(
    // //     "full_transform.to_scale_rotation_translation() {:?}",
    // //     full_transform.to_scale_rotation_translation()
    // // );
}

fn center_of_mass(points: &[Vec3]) -> Vec3 {
    points.iter().sum::<Vec3>() / (points.len() as f32)
}

fn sum_suqares_dist(a_points: &[Vec3], b_points: &[Vec3]) -> f32 {
    assert_eq!(a_points.len(), b_points.len());
    a_points
        .iter()
        .zip(b_points.iter())
        .map(|(&a, &b)| (a - b).length_squared())
        .sum::<f32>()
}

fn sum_suqares_dist_transformed(a_points: &[Vec3], b_points: &[Vec3], b_to_a_transform: &Affine3A) -> f32 {
    assert_eq!(a_points.len(), b_points.len());
    a_points
        .iter()
        .zip(b_points.iter())
        .map(|(&a, &b)| (a - b_to_a_transform.transform_point3(b)).length_squared())
        .sum::<f32>()
}

fn brute_force_transform(a_points: &[Vec3], b_points: &[Vec3]) -> Affine3A {
    assert_eq!(a_points.len(), b_points.len());
    // assume no scale change, for now
    // let scale = Vec3::splat(1.0);

    let b_points = &b_points.iter().map(|p| *p * Vec3::new(-1.0, -1.0, 1.0)).collect_vec();

    let scale = Vec3::new(-1.0, -1.0, -1.0);

    // let translation = center_of_mass(&b_points.iter().map(|p| *p * scale).collect_vec()) - center_of_mass(&a_points);
    // let translation = center_of_mass(&a_points) - center_of_mass(&b_points.iter().map(|p| *p * scale).collect_vec());
    let translation = center_of_mass(&a_points) - center_of_mass(&b_points);
    // let translated = a_points.iter().map(|p| *p + translation).collect_vec();

    let rot_steps = 128;
    let rot_steps_factor = 1.0 / (rot_steps as f32) * std::f32::consts::TAU;

    let (best_transform, sq_dist) = iproduct!(
        // &[-1.0, 1.0f32],
        // &[-1.0, 1.0f32],
        // &[-1.0, 1.0f32],
        (0..rot_steps),
        (0..rot_steps),
        (0..rot_steps)
    )
    // .map(|(sx, sy, sz, a, b, c)| {
    .map(|(a, b, c)| {
        // let scale = Vec3::new(*sx, *sy, *sz);
        let a = (a as f32) * rot_steps_factor;
        let b = (b as f32) * rot_steps_factor;
        let c = (c as f32) * rot_steps_factor;
        let rotation = Quat::from_euler(EulerRot::XYZ, a, b, c);
        // let translation =
        //     center_of_mass(&a_points) - center_of_mass(&b_points.iter().map(|p| *p * scale).collect_vec());
        let current_transform = Affine3A::from_scale_rotation_translation(scale, rotation, translation);
        let sq_dist = sum_suqares_dist_transformed(&a_points, &b_points, &current_transform);
        (current_transform, OrderedFloat(sq_dist))
    })
    .min_by_key(|(_, sq_dist)| *sq_dist)
    .unwrap();
    println!("sq_dist {:?}", sq_dist);
    println!(
        "best_transform.to_scale_rotation_translation() {:?}",
        best_transform.to_scale_rotation_translation()
    );

    // for a in 0..rot_steps {
    //     let a = (a as f32) * rot_steps_factor;
    //     for b in 0..rot_steps {
    //         let b = (b as f32) * rot_steps_factor;
    //         for c in 0..rot_steps {
    //             let c = (c as f32) * rot_steps_factor;

    //             let rotation = Quat::from_euler(EulerRot::XYZ, a, b, c);
    //             let current_transform = Affine3A::from_scale_rotation_translation(scale, rotation, translation);
    //         }
    //     }
    // }

    best_transform
}
