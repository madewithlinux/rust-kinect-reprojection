use std::{
    ffi::c_void,
    ptr::{self, null_mut},
    sync::mpsc::SendError,
};

use image::RgbImage;
// use kinect1_sys::{INuiSensor, HRESULT, c_NuiCreateSensorByIndex, c_NuiGetSensorCount};
use kinect1_sys::{INuiSensor, NuiCreateSensorByIndex, NuiGetSensorCount, HRESULT, NUI_IMAGE_FRAME, NUI_LOCKED_RECT};

pub const NUI_IMAGE_RESOLUTION_1280X960: NUI_IMAGE_RESOLUTION =
    kinect1_sys::_NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_1280x960;
pub const NUI_IMAGE_RESOLUTION_320X240: NUI_IMAGE_RESOLUTION =
    kinect1_sys::_NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_320x240;
pub const NUI_IMAGE_RESOLUTION_640X480: NUI_IMAGE_RESOLUTION =
    kinect1_sys::_NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_640x480;
pub const NUI_IMAGE_RESOLUTION_80X60: NUI_IMAGE_RESOLUTION =
    kinect1_sys::_NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_80x60;
pub const NUI_IMAGE_RESOLUTION_INVALID: NUI_IMAGE_RESOLUTION =
    kinect1_sys::_NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_INVALID;

use kinect1_sys::HANDLE;
pub use kinect1_sys::{
    NUI_IMAGE_RESOLUTION, NUI_IMAGE_TYPE, NUI_INITIALIZE_DEFAULT_HARDWARE_THREAD, NUI_INITIALIZE_FLAG_USES_AUDIO,
    NUI_INITIALIZE_FLAG_USES_COLOR, NUI_INITIALIZE_FLAG_USES_DEPTH, NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX,
    NUI_INITIALIZE_FLAG_USES_HIGH_QUALITY_COLOR, NUI_INITIALIZE_FLAG_USES_SKELETON,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR as NUI_IMAGE_TYPE_COLOR,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_INFRARED as NUI_IMAGE_TYPE_COLOR_INFRARED,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_BAYER as NUI_IMAGE_TYPE_COLOR_RAW_BAYER,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_YUV as NUI_IMAGE_TYPE_COLOR_RAW_YUV,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_YUV as NUI_IMAGE_TYPE_COLOR_YUV,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH as NUI_IMAGE_TYPE_DEPTH,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
    _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX as NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
};
use thiserror::Error;
use winresult::{HResult, HResultError};

mod hresult_helper;

#[derive(Error, Debug)]
pub enum KinectError {
    #[error("HResultError({0:?})")]
    Hre(HResultError),
    #[error("NuiError({0})")]
    NuiError(String),
    #[error("SendError({0})")]
    SendError(#[from] SendError<RgbImage>),
}

pub type KinectResult<T> = Result<T, KinectError>;

fn check_fail(raw_hresult: HRESULT) -> Result<(), KinectError> {
    if raw_hresult == hresult_helper::S_OK {
        return Ok(());
    }
    if let Some(nui_hresult_name) = hresult_helper::try_get_nui_hresult_name(raw_hresult) {
        return Err(KinectError::NuiError(nui_hresult_name));
    }
    HResult::from(raw_hresult as i32)
        .succeeded()
        .map_err(|e| KinectError::Hre(e))?;
    Ok(())
}

pub fn get_sensor_count() -> KinectResult<i32> {
    unsafe {
        let mut i_sensor_count = 0;
        check_fail(NuiGetSensorCount(&mut i_sensor_count))?;
        Ok(i_sensor_count)
    }
}

#[derive(Debug)]
pub struct Sensor {
    delegate: *mut INuiSensor,
}

macro_rules! vtable_method {
    ($mut_ref_self:expr, $method_name:ident) => {
        (*$mut_ref_self).lpVtbl.as_mut().unwrap().$method_name.unwrap()
    };
}
macro_rules! try_call_method {
    ($self:expr, $method_name:ident) => (
        unsafe {
            check_fail(vtable_method!($self, $method_name)($self))?;
            Ok(())
        }
    );
    ($self:expr, $method_name:ident, $($args:expr),+) => (
        unsafe {
            check_fail(vtable_method!($self, $method_name)(
                $self,
                $($args),+
        ))?;
        KinectResult::Ok(())
        }
    );
}

impl Sensor {
    pub fn create_sensor_by_index(index: i32) -> KinectResult<Sensor> {
        unsafe {
            let mut p_nui_sensor: *mut INuiSensor = ptr::null_mut();
            check_fail(NuiCreateSensorByIndex(index, &mut p_nui_sensor))?;
            Ok(Sensor { delegate: p_nui_sensor })
        }
    }

    pub fn status(&mut self) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiStatus)
    }

    pub fn initialize(&mut self, flags: u32) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiInitialize, flags)
    }

    pub fn image_stream_open(
        &mut self,
        e_image_type: NUI_IMAGE_TYPE,
        e_resolution: NUI_IMAGE_RESOLUTION,
        dw_image_frame_flags: u32,
        dw_frame_limit: u32,
        h_next_frame_event: *mut c_void,
    ) -> KinectResult<HANDLE> {
        let mut ph_stream_handle: HANDLE = null_mut();
        try_call_method!(
            self.delegate,
            NuiImageStreamOpen,
            e_image_type,
            e_resolution,
            dw_image_frame_flags,
            dw_frame_limit,
            h_next_frame_event,
            &mut ph_stream_handle
        )?;
        Ok(ph_stream_handle)
    }

    pub fn shutdown(&mut self) {
        unsafe {
            vtable_method!(self.delegate, NuiShutdown)(self.delegate);
        }
    }

    pub fn release(&mut self) {
        unsafe {
            vtable_method!(self.delegate, Release)(self.delegate);
        }
    }

    fn image_stream_get_next_frame(
        &mut self,
        stream: HANDLE,
        ms_to_wait: u32,
        out_frame: *mut NUI_IMAGE_FRAME,
    ) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiImageStreamGetNextFrame, stream, ms_to_wait, out_frame)?;
        Ok(())
    }

    fn image_stream_release_frame(&mut self, stream: HANDLE, mut frame: *mut NUI_IMAGE_FRAME) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiImageStreamReleaseFrame, stream, frame)?;
        Ok(())
    }

    pub fn get_next_frame(self: &mut Sensor, stream: HANDLE) -> KinectResult<RgbImage> {
        let (width, height, bgra_data) = self.get_next_frame_data(stream)?;
        let rgb_data = build_color_rgb_image_buffer(width, height, bgra_data);
        Ok(RgbImage::from_vec(width as u32, height as u32, rgb_data).unwrap())
    }

    fn get_next_frame_data(self: &mut Sensor, stream: HANDLE) -> KinectResult<(usize, usize, Vec<u8>)> {
        let mut frame = NUI_IMAGE_FRAME::default();
        self.image_stream_get_next_frame(stream, 1000, &mut frame)?;
        // dbg!(&frame);

        let mut locked_rect: NUI_LOCKED_RECT = Default::default();
        try_call_method!(frame.pFrameTexture, LockRect, 0, &mut locked_rect, null_mut(), 0)?;
        // dbg!(locked_rect);

        // let texture_pitch = locked_rect.Pitch as usize;

        let (width, height) = convert_resolution_to_size(frame.eResolution);
        let bpp = 4;
        let mem_size = width * height * bpp;
        let mut frame_data = vec![0u8; mem_size];
        // let frame_data_pitch = width * bpp;
        // let best_pitch = std::cmp::min(frame_data_pitch, texture_pitch);
        // dbg!(
        //     width,
        //     height,
        //     bpp,
        //     mem_size,
        //     locked_rect.size,
        //     frame_data_pitch,
        //     texture_pitch,
        //     best_pitch
        // );

        let input_slice = unsafe { std::slice::from_raw_parts(locked_rect.pBits, locked_rect.size as usize) };
        frame_data.copy_from_slice(input_slice);

        try_call_method!(frame.pFrameTexture, UnlockRect, 0)?;

        self.image_stream_release_frame(stream, &mut frame)?;
        Ok((width, height, frame_data))
    }
}

// don't bother, since this takes too long
// impl Drop for Sensor {
//     fn drop(&mut self) {
//         // this seems to take a while, for some reason
//         // dbg!("sensor shutdown");
//         self.shutdown();
//         // dbg!("sensor release");
//         self.release();
//         // dbg!("sensor dropped");
//     }
// }

pub fn convert_resolution_to_size(resolution: NUI_IMAGE_RESOLUTION) -> (usize, usize) {
    match resolution {
        NUI_IMAGE_RESOLUTION_80X60 => (80, 60),
        NUI_IMAGE_RESOLUTION_320X240 => (320, 240),
        NUI_IMAGE_RESOLUTION_640X480 => (640, 480),
        NUI_IMAGE_RESOLUTION_1280X960 => (1280, 960),
        NUI_IMAGE_RESOLUTION_INVALID => panic!("invalid resolution"),
        r => panic!("unknown resolution {}", r),
    }
}

fn build_color_rgb_image_buffer(width: usize, height: usize, bgra_data: Vec<u8>) -> Vec<u8> {
    let mut rgb_data = vec![0; width * height * 3];
    for i in 0..(width * height) {
        rgb_data[i * 3] = bgra_data[i * 4 + 2];
        rgb_data[i * 3 + 1] = bgra_data[i * 4 + 1];
        rgb_data[i * 3 + 2] = bgra_data[i * 4];
    }
    rgb_data
}

// TODO: also do depth stream
fn frame_thread(sender: std::sync::mpsc::Sender<RgbImage>) -> KinectResult<()> {
    let mut sensor = Sensor::create_sensor_by_index(0)?;
    dbg!(&sensor);
    dbg!(sensor.status()?);

    sensor.initialize(NUI_INITIALIZE_FLAG_USES_COLOR | NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX)?;

    let color_stream = sensor.image_stream_open(
        // NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
        NUI_IMAGE_TYPE_COLOR,
        NUI_IMAGE_RESOLUTION_640X480,
        0,
        2,
        null_mut(),
    )?;

    loop {
        let frame = sensor.get_next_frame(color_stream)?;
        sender.send(frame)?;
    }
}

pub fn start_frame_thread() -> std::sync::mpsc::Receiver<RgbImage> {
    let (sender, receiver) = std::sync::mpsc::channel();

    // let sensor = Arc::clone(&sensor);
    std::thread::spawn(move || frame_thread(sender).unwrap());

    receiver
}

// pub struct ColorFrameData {
//     width: usize,
//     height: usize,
//     data: Vec<u8>,
// }
