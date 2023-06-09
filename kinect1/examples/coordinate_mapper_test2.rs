use bytemuck::cast_slice;
use glam::{DVec3, DVec4};
use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;
use kinect1::coordinate_mapper_wrapper::CoordinateMapperWrapper;
use kinect1::{coordinate_mapper_wrapper, NUI_IMAGE_RESOLUTION_320X240};
use ordered_float::OrderedFloat;

// const DEPTH_BUFFER: u16 = 100;
const DEPTH_BUFFER: u16 = 0;
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

    // let num_depth_samples = 100;
    // let depths = (0..num_depth_samples)
    //     .map(|i| MIN_DEPTH_MM + (DEPTH_RANGE as f64 * i as f64 / num_depth_samples as f64).round() as u16)
    //     .collect_vec();
    // dbg!(&depths);
    let depths = (MIN_DEPTH_MM..=MAX_DEPTH_MM).collect_vec();

    println!("initializing kinect");
    let mut mapper = CoordinateMapperWrapper::init_new(args);
    println!("kinect initialized");

    let depth_color_steps = mapper.find_depth_color_steps(0, 0);
    println!("depth_color_steps: {:?}", &depth_color_steps);

    // println!("check all other depth_color_steps");
    // let bar = ProgressBar::new((depth_width * depth_height) as u64);
    // for xi in 0..depth_width {
    //     for yi in 0..depth_height {
    //         bar.inc(1);
    //         // only output a subset of pixels to reduce data size
    //         if xi % 10 != 0 || yi % 10 != 0 {
    //             continue;
    //         }
    //         let depth_color_steps_for_point = mapper.find_depth_color_steps(xi, yi);
    //         if depth_color_steps_for_point != depth_color_steps {
    //             println!("depth_color_steps({xi},{yi}): {:?}", &depth_color_steps_for_point);
    //         }
    //     }
    // }
    // bar.finish();

    for depth in [MIN_DEPTH_MM, 814, 815, 816, 817] {
        println!(
            "mapper.MapDepthPointToColorPoint(0, 0, {depth}): {:?}",
            mapper.MapDepthPointToColorPoint(0, 0, depth)
        );
    }

    println!("make a DepthToColorMapping");
    let depth_color_mapping = mapper.build_depth_to_color_mapping();
    // println!("visualize depth to color mapping");
    // let rgba = depth_color_mapping
    //     .initial_offsets
    //     .iter()
    //     .map(|[x, y]| [(*x as i32 + 128) as u8, (*y as i32 + 128) as u8, 0, 255])
    //     .collect_vec();
    // let image_buffer: ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::from_raw(640, 480, cast_slice(&rgba)).unwrap();
    // image_buffer.save("DepthToColorMapping_initial_offsets.png").unwrap();

    println!("check all values of DepthToColorMapping vs original");
    let bar = ProgressBar::new((depth_width * depth_height) as u64);
    for xi in 0..depth_width {
        for yi in 0..depth_height {
            bar.inc(1);
            // only output a subset of pixels to reduce data size
            if xi % 10 != 0 || yi % 10 != 0 {
                continue;
            }
            for depth_mm in depths.iter() {
                let expected = mapper.MapDepthPointToColorPoint(xi, yi, *depth_mm);
                let packed_depth = *depth_mm << 3;
                let actual = depth_color_mapping.compute_depth_color_mapping(xi, yi, packed_depth);
                assert_eq!(expected, actual, "({xi}, {yi}), {depth_mm}");
            }
        }
    }
    bar.finish();
}
