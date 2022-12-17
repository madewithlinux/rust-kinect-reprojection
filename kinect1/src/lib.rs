use std::{
    ffi::c_void,
    ptr::{self, null_mut},
};

// use kinect1_sys::{INuiSensor, HRESULT, c_NuiCreateSensorByIndex, c_NuiGetSensorCount};
use kinect1_sys::{INuiSensor, HRESULT, NuiCreateSensorByIndex, NuiGetSensorCount};
pub use kinect1_sys::{
    _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_640x480 as NUI_IMAGE_RESOLUTION_640x480,
    NUI_IMAGE_RESOLUTION, NUI_IMAGE_TYPE, NUI_INITIALIZE_DEFAULT_HARDWARE_THREAD,
    NUI_INITIALIZE_FLAG_USES_AUDIO, NUI_INITIALIZE_FLAG_USES_COLOR, NUI_INITIALIZE_FLAG_USES_DEPTH,
    NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_HIGH_QUALITY_COLOR,
    NUI_INITIALIZE_FLAG_USES_SKELETON,
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

macro_rules! method {
    ($self:ident, $method_name:ident) => {
        (*$self.delegate)
            .lpVtbl
            .as_mut()
            .unwrap()
            .$method_name
            .unwrap()
    };
}
macro_rules! try_call_method {
    ($self:ident, $method_name:ident) => (
        unsafe {
            check_fail(method!($self, $method_name)($self.delegate))?;
            Ok(())
        }
    );
    ($self:ident, $method_name:ident, $($args:expr),+) => (
        unsafe {
            check_fail(method!($self, $method_name)(
                $self.delegate,
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
            Ok(Sensor {
                delegate: p_nui_sensor,
            })
        }
    }
    // pub fn status(&mut self) -> KinectResult<()> {
    //     check_fail(unsafe { kinect1_sys::c_INuiSensor_NuiStatus(self.delegate) })
    // }
    pub fn status(&mut self) -> KinectResult<()> {
        try_call_method!(self, NuiStatus)
    }

    pub fn initialize(&mut self, flags: u32) -> KinectResult<()> {
        try_call_method!(self, NuiInitialize, flags)
    }

    pub fn image_stream_open(
        &mut self,
        e_image_type: NUI_IMAGE_TYPE,
        e_resolution: NUI_IMAGE_RESOLUTION,
        dw_image_frame_flags: u32,
        dw_frame_limit: u32,
        h_next_frame_event: *mut c_void,
        // ph_stream_handle: *mut *mut c_void,
    ) -> KinectResult<*mut c_void> {
        let mut ph_stream_handle: *mut c_void = null_mut();
        try_call_method!(
            self,
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
            method!(self, Release)(self.delegate);
        }
    }
}
