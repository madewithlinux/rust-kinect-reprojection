use std::{ffi::c_void, fs::create_dir_all, os::windows::raw::HANDLE, path::PathBuf, ptr::null_mut};

// use kinect1::NuiGetSensorCount;
use anyhow::Result;

use kinect1::{
    get_sensor_count, Sensor, NUI_IMAGE_RESOLUTION_640X480, NUI_IMAGE_TYPE_COLOR,
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

    let rgb_output_dir = PathBuf::from("data/rgb_frames/");
    create_dir_all(&rgb_output_dir)?;
    // save some frames
    for i in 0..10 {
        dbg!(i);
        let frame = sensor.get_next_frame(stream)?;
        frame.save(rgb_output_dir.join(format!("kinect_rgb_data_{}.png", i)))?;
    }

    Ok(())
}
