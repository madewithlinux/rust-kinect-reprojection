use std::io::Write;

use glam::{DVec2, DVec3, DVec4};
use indicatif::ProgressIterator;
use itertools::Itertools;
use kinect1::coordinate_mapper_wrapper::CoordinateMapperWrapper;
use kinect1::{coordinate_mapper_wrapper, NUI_IMAGE_RESOLUTION_320X240};
use ordered_float::OrderedFloat;

const DEPTH_BUFFER: u16 = 100;
const MIN_DEPTH_MM: u16 = 800 + DEPTH_BUFFER;
const MAX_DEPTH_MM: u16 = 4000 - DEPTH_BUFFER;
// const MAX_DEPTH_MM: u16 = 2000;
const DEPTH_RANGE: u16 = MAX_DEPTH_MM - MIN_DEPTH_MM;

fn main() {
    let mut args = coordinate_mapper_wrapper::ReceiverThreadArgs::default();
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
    let depth_skeleton_frame_color_points: Vec<Vec<DVec2>> = depth_skeleton_frames
        .iter()
        .progress()
        .map(|skeleton_frame| mapper.map_skeleton_frame_to_color_points(skeleton_frame))
        .collect_vec();
    println!();

    println!("depth skeleton frame");
    let depth_buf = dump_skeleton_frames(
        depth_width,
        depth_height,
        num_depth_samples,
        &depths,
        &depth_skeleton_frames,
        &depth_skeleton_frame_color_points,
    );
    std::fs::File::create("coordinate_mapper_depth_dump.csv")
        .unwrap()
        .write_all(&depth_buf)
        .unwrap();
    println!();

    // println!("getting depth_to_color_mapping");
    // let color_to_depth_mappings: Vec<Vec<DVec2>> = depths
    //     .iter()
    //     .progress()
    //     .map(|depth_mm| mapper.MapDepthFrameToColorFrame(*depth_mm))
    //     .collect_vec();
    // std::fs::File::create("coordinate_mapper_color_to_depth_mapping.csv")
    //     .unwrap()
    //     .write_all(&dump_depth_to_color_mapping(
    //         depth_width,
    //         depth_height,
    //         num_depth_samples,
    //         &depths,
    //         &color_to_depth_mappings,
    //     ))
    //     .unwrap();
    // println!();
}

fn dump_skeleton_frames(
    depth_width: usize,
    depth_height: usize,
    num_depth_samples: usize,
    depths: &[u16],
    skeleton_frames: &[Vec<DVec4>],
    depth_skeleton_frame_color_points: &[Vec<DVec2>],
) -> Vec<u8> {
    let mut buf = Vec::new();
    // write header
    writeln!(&mut buf, "xi,yi,sample,depth_mm,x,y,z,w,cx,cy").unwrap();

    for xi in 0..depth_width {
        for yi in 0..depth_height {
            // only output a subset of pixels to reduce data size
            if xi % 10 != 0 || yi % 10 != 0 {
                continue;
            }

            let flat_index = xi + yi * depth_width;

            for sample in 0..num_depth_samples {
                let depth_mm = depths[sample];
                let point = skeleton_frames[sample][flat_index];
                let color_point = depth_skeleton_frame_color_points[sample][flat_index];
                // if point == DVec4::new(0.0, 0.0, 0.0, 1.0) {
                //     continue;
                // }
                let DVec4 { x, y, z, w } = point;
                let cx = color_point.x;
                let cy = color_point.y;
                writeln!(&mut buf, "{xi},{yi},{sample},{depth_mm},{x},{y},{z},{w},{cx},{cy}").unwrap();
            }
        }
    }

    buf
}

fn dump_depth_to_color_mapping(
    depth_width: usize,
    depth_height: usize,
    num_depth_samples: usize,
    depths: &[u16],
    color_to_depth_mappings: &[Vec<DVec2>],
) -> Vec<u8> {
    let mut buf = Vec::new();
    // write header
    writeln!(&mut buf, "xi,yi,sample,depth_mm,cx,cy").unwrap();

    for xi in 0..depth_width {
        for yi in 0..depth_height {
            // only output a subset of pixels to reduce data size
            if xi % 10 != 0 || yi % 10 != 0 {
                continue;
            }

            let flat_index = xi + yi * depth_width;

            for sample in 0..num_depth_samples {
                let depth_mm = depths[sample];
                let point = color_to_depth_mappings[sample][flat_index];
                let cx = point.x;
                let cy = point.y;
                writeln!(&mut buf, "{xi},{yi},{sample},{depth_mm},{cx},{cy}").unwrap();
            }
        }
    }

    buf
}
