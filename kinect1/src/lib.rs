use std::{
    ffi::c_void,
    pin::Pin,
    ptr::{self, null_mut},
};

// use kinect1_sys::{INuiSensor, HRESULT, c_NuiCreateSensorByIndex, c_NuiGetSensorCount};
use kinect1_sys::{
    INuiSensor, NuiCreateSensorByIndex, NuiGetSensorCount, NuiImageStreamGetNextFrame, HRESULT, NUI_IMAGE_FRAME,
    NUI_LOCKED_RECT,
};

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

use kinect1_sys::{HANDLE, NUI_IMAGE_VIEW_AREA};
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

#[derive(Error, Debug)]
pub enum KinectError {
    #[error("HResultError({0:?})")]
    Hre(HResultError),
}

pub type KinectResult<T> = Result<T, KinectError>;

fn check_fail(raw_hresult: HRESULT) -> Result<(), KinectError> {
    HResult::from(raw_hresult)
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
macro_rules! delegate_method {
    ($self:expr, $method_name:ident) => {
        (*$self.delegate).lpVtbl.as_mut().unwrap().$method_name.unwrap()
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
        Ok(())
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
    // pub fn status(&mut self) -> KinectResult<()> {
    //     check_fail(unsafe { kinect1_sys::c_INuiSensor_NuiStatus(self.delegate) })
    // }
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
        // ph_stream_handle: *mut *mut c_void,
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
            // ph_stream_handle
            &mut ph_stream_handle
        )?;
        Ok(ph_stream_handle)
    }

    pub fn release(&mut self) {
        unsafe {
            delegate_method!(self, Release)(self.delegate);
        }
    }

    pub fn image_stream_get_next_frame(&mut self, stream: HANDLE) -> KinectResult<Pin<Box<NUI_IMAGE_FRAME>>> {
        let mut frame = Box::pin(NUI_IMAGE_FRAME::default());
        try_call_method!(self.delegate, NuiImageStreamGetNextFrame, stream, 1000, &mut *frame)?;
        Ok(frame)
    }

    pub fn image_stream_release_frame(
        &mut self,
        stream: HANDLE,
        mut frame: Pin<Box<NUI_IMAGE_FRAME>>,
    ) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiImageStreamReleaseFrame, stream, &mut *frame)?;
        Ok(())
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.release()
    }
}

pub fn get_next_frame_data(sensor: &mut Sensor, stream: HANDLE) -> KinectResult<Vec<u8>> {
    let frame = sensor.image_stream_get_next_frame(stream)?;
    dbg!(&frame);

    let mut locked_rect: NUI_LOCKED_RECT = Default::default();
    try_call_method!(frame.pFrameTexture, LockRect, 0, &mut locked_rect, null_mut(), 0)?;
    dbg!(locked_rect);

    let texture_pitch = locked_rect.Pitch as usize;

    let (width, height) = convert_resolution_to_size(frame.eResolution);
    let bpp = 4;
    let mem_size = width * height * bpp;
    let mut frame_data = vec![0u8; mem_size];
    let frame_data_pitch = width * bpp;

    let best_pitch = std::cmp::min(frame_data_pitch, texture_pitch);
    dbg!(
        width,
        height,
        bpp,
        mem_size,
        frame_data_pitch,
        texture_pitch,
        best_pitch
    );

    let input_slice = unsafe { std::slice::from_raw_parts(locked_rect.pBits, locked_rect.size as usize) };
    for y in 0..height {
        // dbg!(y, y * texture_pitch, y * frame_data_pitch);
        // let input_slice = unsafe { std::slice::from_raw_parts(locked_rect.pBits, y * texture_pitch) };
        frame_data[y * frame_data_pitch..y * frame_data_pitch + best_pitch]
            .copy_from_slice(&input_slice[y * texture_pitch..y * texture_pitch + best_pitch]);
    }

    try_call_method!(frame.pFrameTexture, UnlockRect, 0)?;

    sensor.image_stream_release_frame(stream, frame)?;
    Ok(frame_data)
}

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
