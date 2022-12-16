use std::{ffi::c_void, ptr::null_mut};

// use kinect1::NuiGetSensorCount;
use anyhow::Result;

use kinect1::{
    get_sensor_count, NUI_IMAGE_RESOLUTION_640x480, Sensor, NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
    NUI_INITIALIZE_FLAG_USES_COLOR, NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX,
};

fn main() -> Result<()> {
    println!("Hello, world!");
    dbg!(get_sensor_count().unwrap());
    let mut sensor = Sensor::create_sensor_by_index(0)?;
    dbg!(&sensor);
    dbg!(sensor.status()?);

    sensor.initialize(
        NUI_INITIALIZE_FLAG_USES_COLOR | NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX,
    )?;

    dbg!(sensor.status()?);
    let stream_handle = sensor.image_stream_open(
        NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
        NUI_IMAGE_RESOLUTION_640x480,
        0,
        2,
        null_mut(),
    )?;
    dbg!(stream_handle);

    dbg!(sensor.status()?);

    Ok(())
}
