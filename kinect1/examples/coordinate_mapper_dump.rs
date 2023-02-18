use std::io::Write;

use glam::{DVec3, DVec4};
use indicatif::ProgressIterator;
use itertools::Itertools;
use kinect1::coordinate_mapper::CoordinateMapperWrapper;
use kinect1::{coordinate_mapper, NUI_IMAGE_RESOLUTION_320X240};
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
    // println!("color");
    // let color_skeleton_frames: Vec<Vec<DVec4>> = depths
    //     .iter()
    //     .progress()
    //     .map(|depth_mm| mapper.MapColorFrameToSkeletonFrame(*depth_mm))
    //     .collect_vec();
    println!();

    // println!("color skeleton frame statistics");
    // dump_skeleton_frames(
    //     depth_width,
    //     depth_height,
    //     num_depth_samples,
    //     &depths,
    //     &color_skeleton_frames,
    // );
    // println!();

    println!("depth skeleton frame statistics");
    let depth_buf = dump_skeleton_frames(
        depth_width,
        depth_height,
        num_depth_samples,
        &depths,
        &depth_skeleton_frames,
    );
    std::fs::File::create("coordinate_mapper_depth_dump.csv")
        .unwrap()
        .write_all(&depth_buf)
        .unwrap();
    println!();
}

fn dump_skeleton_frames(
    depth_width: usize,
    depth_height: usize,
    num_depth_samples: usize,
    depths: &[u16],
    skeleton_frames: &[Vec<DVec4>],
) -> Vec<u8> {
    let mut buf = Vec::new();
    // write header
    writeln!(&mut buf, "xi,yi,sample,depth_mm,x,y,z,w").unwrap();

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
                // if point == DVec4::new(0.0, 0.0, 0.0, 1.0) {
                //     continue;
                // }
                let DVec4 { x, y, z, w } = point;
                writeln!(&mut buf, "{xi},{yi},{sample},{depth_mm},{x},{y},{z},{w}").unwrap();
            }
        }
    }

    buf
}
