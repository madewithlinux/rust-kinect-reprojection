use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kinect1_sys::{NUI_COLOR_IMAGE_POINT, NUI_DEPTH_IMAGE_PIXEL};
use rand::Rng;

use glam::{DVec2, DVec3, DVec4, IVec2};
use itertools::Itertools;

use kinect1::coordinate_mapper_wrapper::CoordinateMapperWrapper;
use kinect1::{coordinate_mapper_wrapper, NuiDepthPixelToDepth, NuiDepthPixelToPlayerIndex, NUI_IMAGE_RESOLUTION_320X240};
use rand::prelude::Distribution;

const DEPTH_BUFFER: u16 = 100;
const MIN_DEPTH_MM: u16 = 800 + DEPTH_BUFFER;
const MAX_DEPTH_MM: u16 = 4000 - DEPTH_BUFFER;
// const MAX_DEPTH_MM: u16 = 2000;
const DEPTH_RANGE: u16 = MAX_DEPTH_MM - MIN_DEPTH_MM;

fn criterion_benchmark(c: &mut Criterion) {
    let mut args = coordinate_mapper_wrapper::ReceiverThreadArgs::default();
    // args.color_resolution = NUI_IMAGE_RESOLUTION_320X240;
    // args.depth_resolution = NUI_IMAGE_RESOLUTION_320X240;
    let (depth_width, depth_height) = args.get_depth_size();
    let (color_width, color_height) = args.get_color_size();

    println!("initializing kinect");
    let mut mapper = CoordinateMapperWrapper::init_new(args);
    println!("kinect initialized");
    println!("build_depth_to_color_mapping...");
    let depth_color_mapping = mapper.build_depth_to_color_mapping();
    println!("build_depth_to_color_mapping done");

    // let num_depth_samples = 100;
    // let depths = (0..num_depth_samples)
    //     .map(|i| MIN_DEPTH_MM + (DEPTH_RANGE as f64 * i as f64 / num_depth_samples as f64).round() as u16)
    //     .collect_vec();
    // dbg!(&depths);

    // let depth_frame = (0..(depth_width*depth_height)).map(|_| {
    //     let mut rng = rand::thread_rng();
    //     rng.
    // }).collect_vec();

    let mut packed_depth_frame = vec![0u16; depth_width * depth_height];
    let mut rng = rand::thread_rng();
    let depth_range = rand::distributions::Uniform::try_from((MIN_DEPTH_MM << 3)..(MAX_DEPTH_MM << 3)).unwrap();
    packed_depth_frame.fill_with(|| depth_range.sample(&mut rng));

    c.bench_function("MapDepthFrameToColorFrame_for_frame", |b| {
        b.iter(|| mapper.MapDepthFrameToColorFrame_for_frame(black_box(&packed_depth_frame)))
    });
    c.bench_function("map_depth_to_color_hypothetical_baseline", |b| {
        b.iter(|| map_depth_to_color_hypothetical_baseline(black_box(&packed_depth_frame)))
    });
    c.bench_function("compute_depth_color_offset_frame", |b| {
        b.iter(|| depth_color_mapping.compute_depth_color_offset_frame(black_box(&packed_depth_frame)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn map_depth_to_color_hypothetical_baseline(packed_depth_frame: &[u16]) -> Vec<DVec2> {
    // fn map_depth_to_color_hypothetical_baseline(packed_depth_frame: &[u16]) -> Vec<NUI_COLOR_IMAGE_POINT> {
    let depth_frame_pixels = packed_depth_frame
        .iter()
        .map(|&pd| NUI_DEPTH_IMAGE_PIXEL {
            depth: NuiDepthPixelToDepth(pd),
            playerIndex: NuiDepthPixelToPlayerIndex(pd),
        })
        .collect_vec();
    let mut color_frame_points: Vec<NUI_COLOR_IMAGE_POINT> = vec![Default::default(); packed_depth_frame.len()];

    for (i, (dp, cp)) in depth_frame_pixels.iter().zip(color_frame_points.iter_mut()).enumerate() {
        let x = (i % 640) as f32;
        let y = (i / 640) as f32;
        cp.x = (x + (dp.depth as f32) * 2.0 + 1.0) as i32;
        cp.y = (y + (dp.depth as f32) * 3.0 - 7.0) as i32;
    }

    // color_frame_points
    color_frame_points
        .iter()
        .map(|p| IVec2::new(p.x, p.y).as_dvec2())
        .collect_vec()
}
