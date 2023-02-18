use glam::{DVec3, DVec4};
use indicatif::ProgressIterator;
use itertools::Itertools;
use kinect1::{coordinate_mapper, NUI_IMAGE_RESOLUTION_320X240};
use kinect1::coordinate_mapper::CoordinateMapperWrapper;
use ordered_float::OrderedFloat;

const DEPTH_BUFFER: u16 = 100;
const MIN_DEPTH_MM: u16 = 800 + DEPTH_BUFFER;
const MAX_DEPTH_MM: u16 = 4000 - DEPTH_BUFFER;
// const MAX_DEPTH_MM: u16 = 2000;
const DEPTH_RANGE: u16 = MAX_DEPTH_MM - MIN_DEPTH_MM;

fn main() {
    let mut args = coordinate_mapper::ReceiverThreadArgs::default();
    // args.color_resolution = NUI_IMAGE_RESOLUTION_320X240;
    // args.depth_resolution = NUI_IMAGE_RESOLUTION_320X240;
    let (depth_width, depth_height) = args.get_depth_size();
    let (color_width, color_height) = args.get_color_size();

    let num_depth_samples = 100;
    let depths = (0..num_depth_samples)
        .map(|i| MIN_DEPTH_MM + (DEPTH_RANGE as f64 * i as f64 / num_depth_samples as f64).round() as u16)
        .collect_vec();
    // dbg!(&depths);

    println!("initializing kinect");
    let mut mapper = CoordinateMapperWrapper::init_new(args);
    println!("kinect initialized");

    println!("getting skeleton frames");
    println!("depth");
    let depth_skeleton_frames: Vec<Vec<DVec4>> = depths
    .iter()
    .progress()
    .map(|depth_mm| mapper.MapDepthFrameToSkeletonFrame(*depth_mm))
    .collect_vec();
    println!("color");
    let color_skeleton_frames: Vec<Vec<DVec4>> = depths
        .iter()
        .progress()
        .map(|depth_mm| mapper.MapColorFrameToSkeletonFrame(*depth_mm))
        .collect_vec();
    println!();

    println!("color skeleton frame statistics");
    skeleton_frame_statistics(
        depth_width,
        depth_height,
        num_depth_samples,
        &depths,
        &color_skeleton_frames,
    );
    println!();

    println!("depth skeleton frame statistics");
    skeleton_frame_statistics(
        depth_width,
        depth_height,
        num_depth_samples,
        &depths,
        &depth_skeleton_frames,
    );
    println!();
}

fn skeleton_frame_statistics(
    depth_width: usize,
    depth_height: usize,
    num_depth_samples: usize,
    depths: &[u16],
    skeleton_frames: &[Vec<DVec4>],
) {
    let mut min_point = DVec4::splat(1.0);
    let mut max_point = DVec4::splat(0.0);
    let mut all_points = vec![];
    let mut depth_length_diff = vec![];
    let mut point3_normal_diff = vec![];

    for x in 0..depth_width {
        for y in 0..depth_height {
            // if x != depth_width/2 || y != depth_height/2 { continue; }
            let flat_index = x + y * depth_width;

            let mut point3_normal_min = DVec3::splat(1.0);
            let mut point3_normal_max = DVec3::splat(0.0);
            for sample in 0..num_depth_samples {
                let depth_mm = depths[sample];
                let point = skeleton_frames[sample][flat_index];
                if point == DVec4::new(0.0, 0.0, 0.0, 1.0) {
                    continue;
                }
                let point3 = point.truncate();

                min_point = min_point.min(point);
                max_point = max_point.max(point);
                all_points.push(point);

                depth_length_diff.push((point3.length() - (depth_mm as f64) / 1_000.0).abs());

                let point3_normal = point3.normalize();
                point3_normal_min = point3_normal_min.min(point3_normal);
                point3_normal_max = point3_normal_max.max(point3_normal);
            }
            if point3_normal_min != DVec3::splat(0.0) {
                // point3_normal_diff.push((point3_normal_max - point3_normal_min).length());
                point3_normal_diff.push((point3_normal_max.normalize() - point3_normal_min.normalize()).length());
            }
        }
    }

    // let mean_point: DVec4 = all_points.iter().sum::<DVec4>() / all_points.len() as f64;
    // // dbg!(min_point, max_point, mean_point);
    // // println!("points: {min_point}/{max_point}/{mean_point}");
    // println!("points:");
    // println!("\tmin:  {min_point}");
    // println!("\tmax:  {max_point}");
    // println!("\tmean: {mean_point}");
    let all_points_x = all_points.iter().map(|p| p.x).collect_vec();
    let all_points_y = all_points.iter().map(|p| p.y).collect_vec();
    let all_points_z = all_points.iter().map(|p| p.z).collect_vec();
    let all_points_w = all_points.iter().map(|p| p.w).collect_vec();

    statistics_header();
    statistics("all_points_x", &all_points_x);
    statistics("all_points_y", &all_points_y);
    statistics("all_points_z", &all_points_z);
    statistics("all_points_w", &all_points_w);

    statistics("depth_length_diff", &depth_length_diff);
    statistics("point3_normal_diff", &point3_normal_diff);
}

fn statistics(label: &str, v: &[f64]) {
    let count = v.len();
    let min = v.iter().map(|vi| OrderedFloat(*vi)).min().unwrap().0;
    let max = v.iter().map(|vi| OrderedFloat(*vi)).max().unwrap().0;
    let mean = mean(v);
    let std_deviation = std_deviation(v, mean);
    println!("{label:20}: {count:15} {min:15.5} / {max:15.5} / {mean:15.5} {std_deviation:15.5}");
}
fn statistics_header() {
    let label = "label";
    let count = "count";
    let min = "min";
    let max = "max";
    let mean = "mean";
    let std_deviation = "std_deviation";
    println!("{label:20}: {count:>15} {min:>15} / {max:>15} / {mean:>15} {std_deviation:>15}");
}

// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn mean(v: &[f64]) -> f64 {
    let sum = v.iter().sum::<f64>() as f64;
    let count = v.len() as f64;
    assert!(count > 0.0);
    sum / count
}

fn std_deviation(v: &[f64], data_mean: f64) -> f64 {
    let count = v.len();
    let variance = v
        .iter()
        .map(|value| {
            let diff = data_mean - (*value as f64);
            diff * diff
        })
        .sum::<f64>()
        / count as f64;

    variance.sqrt()
}
