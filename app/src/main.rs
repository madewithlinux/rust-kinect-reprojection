use std::{ffi::c_void, os::windows::raw::HANDLE, ptr::null_mut};

// use kinect1::NuiGetSensorCount;
use anyhow::Result;

use kinect1::{
    get_next_frame_data, get_sensor_count, Sensor, NUI_IMAGE_RESOLUTION_640X480, NUI_IMAGE_TYPE_COLOR,
    NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_COLOR,
    NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX,
};

fn main() -> Result<()> {
    println!("Hello, world!");
    dbg!(get_sensor_count().unwrap());
    let mut sensor = Sensor::create_sensor_by_index(0)?;
    dbg!(&sensor);
    dbg!(sensor.status()?);

    sensor.initialize(NUI_INITIALIZE_FLAG_USES_COLOR | NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX)?;

    dbg!(sensor.status()?);
    let stream = sensor.image_stream_open(
        // NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
        NUI_IMAGE_TYPE_COLOR,
        NUI_IMAGE_RESOLUTION_640X480,
        0,
        2,
        null_mut(),
    )?;
    dbg!(stream);

    dbg!(sensor.status()?);

    for i in 0..10 {
        let frame = sensor.image_stream_get_next_frame(stream)?;
        dbg!(i, &frame);
        sensor.image_stream_release_frame(stream, frame)?;
    }

    for i in 0..10 {
        let frame_data = get_next_frame_data(&mut sensor, stream)?;
        // dbg!(i, &frame_data);
        let avg_pix: f64 = frame_data.iter().map(|&v| v as f64).sum::<f64>() / (frame_data.len() as f64);
        dbg!(i, &avg_pix);
    }

    Ok(())
}
