use std::{
    ffi::c_void,
    ptr::{self, null_mut},
    sync::mpsc::SendError,
};

pub type Gray16Image = ImageBuffer<Luma<u16>, Vec<u16>>;
pub use image::{Rgb, RgbImage};

use image::{ImageBuffer, Luma};
// use kinect1_sys::{INuiSensor, HRESULT, c_NuiCreateSensorByIndex, c_NuiGetSensorCount};
use kinect1_sys::{
    INuiSensor, NuiCreateSensorByIndex, NuiGetSensorCount, HANDLE, HRESULT, NUI_IMAGE_FRAME, NUI_LOCKED_RECT,
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

pub const NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX: NUI_IMAGE_TYPE =
    kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX;
pub const NUI_IMAGE_TYPE_COLOR: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR;
pub const NUI_IMAGE_TYPE_COLOR_YUV: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_YUV;
pub const NUI_IMAGE_TYPE_COLOR_RAW_YUV: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_YUV;
pub const NUI_IMAGE_TYPE_DEPTH: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH;
pub const NUI_IMAGE_TYPE_COLOR_INFRARED: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_INFRARED;
pub const NUI_IMAGE_TYPE_COLOR_RAW_BAYER: NUI_IMAGE_TYPE = kinect1_sys::_NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_BAYER;

// pub use kinect1_sys::{
//     NUI_DEPTH_DEPTH_UNKNOWN_VALUE, NUI_IMAGE_DEPTH_MAXIMUM, NUI_IMAGE_DEPTH_MAXIMUM_NEAR_MODE, NUI_IMAGE_DEPTH_MINIMUM,
//     NUI_IMAGE_DEPTH_MINIMUM_NEAR_MODE, NUI_IMAGE_DEPTH_NO_VALUE, NUI_IMAGE_DEPTH_TOO_FAR_VALUE,
//     NUI_IMAGE_DEPTH_UNKNOWN_VALUE,
// };
pub const NUI_IMAGE_DEPTH_MAXIMUM: u16 = kinect1_sys::NUI_IMAGE_DEPTH_MAXIMUM as u16;
pub const NUI_IMAGE_DEPTH_MINIMUM: u16 = kinect1_sys::NUI_IMAGE_DEPTH_MINIMUM as u16;
pub const NUI_IMAGE_DEPTH_MAXIMUM_NEAR_MODE: u16 = kinect1_sys::NUI_IMAGE_DEPTH_MAXIMUM_NEAR_MODE as u16;
pub const NUI_IMAGE_DEPTH_MINIMUM_NEAR_MODE: u16 = kinect1_sys::NUI_IMAGE_DEPTH_MINIMUM_NEAR_MODE as u16;
pub const NUI_IMAGE_DEPTH_NO_VALUE: u16 = kinect1_sys::NUI_IMAGE_DEPTH_NO_VALUE as u16;
pub const NUI_IMAGE_DEPTH_TOO_FAR_VALUE: u16 = kinect1_sys::NUI_IMAGE_DEPTH_TOO_FAR_VALUE as u16;
pub const NUI_IMAGE_DEPTH_UNKNOWN_VALUE: u16 = kinect1_sys::NUI_IMAGE_DEPTH_UNKNOWN_VALUE as u16;
pub const NUI_DEPTH_DEPTH_UNKNOWN_VALUE: u16 = kinect1_sys::NUI_DEPTH_DEPTH_UNKNOWN_VALUE as u16;
pub const NUI_DEPTH_UNKNOWN: u16 = kinect1_sys::NUI_DEPTH_UNKNOWN as u16;

pub const KINECT_MAX_FRAMERATE: i64 = 30;
pub const MAX_ALLOWED_ELAPSED_TIME: i64 = (1000 / KINECT_MAX_FRAMERATE) / 2;

pub use kinect1_sys::{
    NUI_IMAGE_RESOLUTION, NUI_IMAGE_TYPE, NUI_INITIALIZE_DEFAULT_HARDWARE_THREAD, NUI_INITIALIZE_FLAG_USES_AUDIO,
    NUI_INITIALIZE_FLAG_USES_COLOR, NUI_INITIALIZE_FLAG_USES_DEPTH, NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX,
    NUI_INITIALIZE_FLAG_USES_HIGH_QUALITY_COLOR, NUI_INITIALIZE_FLAG_USES_SKELETON,
};

use thiserror::Error;
use windows::Win32::{
    Foundation::WAIT_OBJECT_0,
    System::Threading::{WaitForMultipleObjects, WaitForSingleObject},
};
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

#[derive(Debug, Copy, Clone, Default)]
pub struct ImageFrameInfo {
    pub width: usize,
    pub height: usize,
    pub timestamp: i64,
    pub frame_number: u32,
    pub image_type: NUI_IMAGE_TYPE,
    pub resolution: NUI_IMAGE_RESOLUTION,
    pub frame_flags: u32,
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

    fn image_stream_release_frame(&mut self, stream: HANDLE, frame: *mut NUI_IMAGE_FRAME) -> KinectResult<()> {
        try_call_method!(self.delegate, NuiImageStreamReleaseFrame, stream, frame)?;
        Ok(())
    }

    pub fn get_next_color_frame(
        self: &mut Sensor,
        stream: HANDLE,
        ms_to_wait: u32,
    ) -> KinectResult<(RgbImage, ImageFrameInfo)> {
        // let (width, height, bgra_data, image_type) = self.get_next_frame_data(stream, ms_to_wait)?;
        let (bgra_data, image_frame_info) = self.get_next_frame_data(stream, ms_to_wait)?;
        let ImageFrameInfo {
            width,
            height,
            image_type,
            ..
        } = image_frame_info;
        let rgb_data = build_color_rgb_image_buffer(width, height, bgra_data, image_type);
        Ok((
            RgbImage::from_vec(width as u32, height as u32, rgb_data).unwrap(),
            image_frame_info,
        ))
    }

    pub fn get_next_depth_frame(
        self: &mut Sensor,
        stream: HANDLE,
        ms_to_wait: u32,
    ) -> KinectResult<(Gray16Image, ImageFrameInfo)> {
        let (frame_data, image_frame_info) = self.get_next_frame_data(stream, ms_to_wait)?;
        let ImageFrameInfo {
            width,
            height,
            image_type,
            ..
        } = image_frame_info;
        let depth_data = build_depth_image_buffer(width, height, frame_data, image_type);
        Ok((
            Gray16Image::from_vec(width as u32, height as u32, depth_data).unwrap(),
            image_frame_info,
        ))
    }

    fn get_next_frame_data(
        self: &mut Sensor,
        stream: HANDLE,
        ms_to_wait: u32,
        // ) -> KinectResult<(usize, usize, Vec<u8>, NUI_IMAGE_TYPE)> {
    ) -> KinectResult<(Vec<u8>, ImageFrameInfo)> {
        let mut frame = NUI_IMAGE_FRAME::default();
        self.image_stream_get_next_frame(stream, ms_to_wait, &mut frame)?;
        // dbg!(&frame);

        let mut locked_rect: NUI_LOCKED_RECT = Default::default();
        try_call_method!(frame.pFrameTexture, LockRect, 0, &mut locked_rect, null_mut(), 0)?;

        let (width, height) = convert_resolution_to_size(frame.eResolution);
        let bpp = (locked_rect.size as usize) / (width * height);
        let mem_size = width * height * bpp;
        let mut frame_data = vec![0u8; mem_size];

        let input_slice = unsafe { std::slice::from_raw_parts(locked_rect.pBits, locked_rect.size as usize) };
        frame_data.copy_from_slice(input_slice);

        try_call_method!(frame.pFrameTexture, UnlockRect, 0)?;

        self.image_stream_release_frame(stream, &mut frame)?;
        Ok((
            frame_data,
            ImageFrameInfo {
                width,
                height,
                timestamp: frame.liTimeStamp,
                frame_number: frame.dwFrameNumber,
                image_type: frame.eImageType,
                resolution: frame.eResolution,
                frame_flags: frame.dwFrameFlags,
            },
        ))
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

fn build_depth_image_buffer(width: usize, height: usize, depth_data: Vec<u8>, image_type: NUI_IMAGE_TYPE) -> Vec<u16> {
    let mut grayscale_data = vec![0; width * height];
    assert_eq!(image_type, NUI_IMAGE_TYPE_DEPTH);
    assert_eq!(depth_data.len() / 2, grayscale_data.len());
    for i in 0..(width * height) {
        let depth_and_player_index = u16::from_ne_bytes([depth_data[i * 2], depth_data[i * 2 + 1]]);
        // grayscale_data[i] = depth_and_player_index >> NUI_IMAGE_PLAYER_INDEX_SHIFT;
        grayscale_data[i] = depth_and_player_index;
    }
    grayscale_data
}

fn build_color_rgb_image_buffer(width: usize, height: usize, bgra: Vec<u8>, image_type: NUI_IMAGE_TYPE) -> Vec<u8> {
    let mut rgb_data = vec![0; width * height * 3];
    assert_eq!(image_type, NUI_IMAGE_TYPE_COLOR);
    assert_eq!(bgra.len() / 4, rgb_data.len() / 3);
    for i in 0..(width * height) {
        rgb_data[i * 3] = bgra[i * 4 + 2];
        rgb_data[i * 3 + 1] = bgra[i * 4 + 1];
        rgb_data[i * 3 + 2] = bgra[i * 4];
    }
    rgb_data
}

pub fn depth_to_rgb_color(depth: u16) -> Rgb<u8> {
    match depth {
        NUI_IMAGE_DEPTH_NO_VALUE => Rgb([255, 0, 0]),
        NUI_IMAGE_DEPTH_MAXIMUM => Rgb([0, 255, 0]),
        NUI_IMAGE_DEPTH_MINIMUM => Rgb([0, 0, 255]),
        NUI_IMAGE_DEPTH_TOO_FAR_VALUE => Rgb([0, 255, 255]),
        NUI_IMAGE_DEPTH_UNKNOWN_VALUE => Rgb([255, 255, 0]),
        depth => {
            let normalized =
                (depth - NUI_IMAGE_DEPTH_MINIMUM) as f64 / (NUI_IMAGE_DEPTH_MAXIMUM - NUI_IMAGE_DEPTH_MINIMUM) as f64;
            let luma = (normalized * 255.0) as u8;
            Rgb([luma, luma, luma])
        }
    }
}

fn frame_thread(sender: std::sync::mpsc::Sender<KinectFrameMessage>) -> KinectResult<()> {
    let mut sensor = Sensor::create_sensor_by_index(0)?;
    dbg!(&sensor);
    dbg!(sensor.status()?);

    let color_event = unsafe { windows::Win32::System::Threading::CreateEventW(None, true, false, None).unwrap() };
    let depth_event = unsafe { windows::Win32::System::Threading::CreateEventW(None, true, false, None).unwrap() };

    sensor.initialize(NUI_INITIALIZE_FLAG_USES_COLOR | NUI_INITIALIZE_FLAG_USES_DEPTH)?;

    unsafe {
        windows::Win32::System::Threading::ResetEvent(color_event);
        windows::Win32::System::Threading::ResetEvent(depth_event);
    };

    let color_stream = sensor
        .image_stream_open(NUI_IMAGE_TYPE_COLOR, NUI_IMAGE_RESOLUTION_640X480, 0, 2, unsafe {
            std::mem::transmute(color_event.0)
        })
        .unwrap();
    let depth_stream = sensor
        .image_stream_open(NUI_IMAGE_TYPE_DEPTH, NUI_IMAGE_RESOLUTION_640X480, 0, 2, unsafe {
            std::mem::transmute(depth_event.0)
        })
        .unwrap();

    let mut current_color_frame = Default::default();
    let mut current_color_frame_info = Default::default();
    let mut current_depth_frame = Default::default();
    let mut current_depth_frame_info = Default::default();
    loop {
        unsafe { WaitForMultipleObjects(&[color_event, depth_event], false, 100) };

        if unsafe { WaitForSingleObject(color_event, 0) } == WAIT_OBJECT_0 {
            (current_color_frame, current_color_frame_info) = sensor.get_next_color_frame(color_stream, 1).unwrap();
        }
        if unsafe { WaitForSingleObject(depth_event, 0) } == WAIT_OBJECT_0 {
            (current_depth_frame, current_depth_frame_info) = sensor.get_next_depth_frame(depth_stream, 1).unwrap();
        }

        // only send a new frame message if we've got both frames
        if (current_color_frame_info.timestamp - current_depth_frame_info.timestamp).abs() > MAX_ALLOWED_ELAPSED_TIME
            || current_color_frame_info.timestamp == Default::default()
            || current_depth_frame_info.timestamp == Default::default()
        {
            continue;
        }

        // TODO: map color to depth frame (or vise versa?)

        match sender.send(KinectFrameMessage {
            color_frame: current_color_frame.clone(),
            depth_frame: current_depth_frame.clone(),
            color_frame_info: current_color_frame_info,
            depth_frame_info: current_depth_frame_info,
        }) {
            Ok(_) => (),
            Err(_) => println!("frame receiver hung up"),
        }
    }
}

#[derive(Clone, Default)]
pub struct KinectFrameMessage {
    pub color_frame: RgbImage,
    pub depth_frame: Gray16Image,
    pub color_frame_info: ImageFrameInfo,
    pub depth_frame_info: ImageFrameInfo,
}

pub fn start_frame_thread() -> std::sync::mpsc::Receiver<KinectFrameMessage> {
    let (sender, receiver) = std::sync::mpsc::channel();

    // let sensor = Arc::clone(&sensor);
    std::thread::spawn(move || frame_thread(sender).unwrap());

    receiver
}
