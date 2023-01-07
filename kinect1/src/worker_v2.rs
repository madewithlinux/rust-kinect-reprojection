use std::ptr::null_mut;

use bytemuck::cast_slice;
use image::RgbImage;
use kinect1_sys::{
    INuiCoordinateMapper, INuiFrameTexture, INuiSensor, NuiCreateSensorByIndex, NuiDepthPixelToDepth,
    NuiDepthPixelToPlayerIndex, NUI_DEPTH_IMAGE_PIXEL, NUI_DEPTH_IMAGE_POINT, NUI_IMAGE_FRAME,
    NUI_IMAGE_PLAYER_INDEX_SHIFT, NUI_IMAGE_RESOLUTION, NUI_IMAGE_STREAM_FLAG_DISTINCT_OVERFLOW_DEPTH_VALUES,
    NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA, NUI_INITIALIZE_FLAG_USES_COLOR,
    NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_SKELETON, NUI_LOCKED_RECT,
};
use tracing::{info, warn};
use windows::Win32::{
    Foundation::WAIT_OBJECT_0,
    System::Threading::{WaitForMultipleObjects, WaitForSingleObject},
};

use crate::{
    call_method, check_fail, convert_resolution_to_size, try_call_method, vtable_method, FrameMessageReceiver,
    Gray16Image, ImageFrameInfo, KinectFrameMessage, MAX_ALLOWED_ELAPSED_TIME, NUI_IMAGE_RESOLUTION_640X480,
    NUI_IMAGE_TYPE_COLOR, NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameMappingType {
    #[default]
    None,
    // TODO: name these in a better way that's less ambiguous which one is getting changed
    ColorToDepth,
    DepthToColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiverThreadArgs {
    pub sensor_index: i32,
    pub color_resolution: NUI_IMAGE_RESOLUTION,
    pub color_stream_flags: u32,
    pub color_buffered_frame_limit: u32,

    pub depth_resolution: NUI_IMAGE_RESOLUTION,
    pub depth_stream_flags: u32,
    pub depth_buffered_frame_limit: u32,
    pub use_extended_depth_range: bool,

    pub mapping: FrameMappingType,
}

impl ReceiverThreadArgs {
    pub fn get_color_size(&self) -> (usize, usize) {
        convert_resolution_to_size(self.color_resolution)
    }
    pub fn get_depth_size(&self) -> (usize, usize) {
        convert_resolution_to_size(self.depth_resolution)
    }
}

impl Default for ReceiverThreadArgs {
    fn default() -> Self {
        Self {
            sensor_index: 0,
            color_resolution: NUI_IMAGE_RESOLUTION_640X480,
            color_stream_flags: 0,
            color_buffered_frame_limit: 2,
            depth_resolution: NUI_IMAGE_RESOLUTION_640X480,
            depth_stream_flags: 0,
            depth_buffered_frame_limit: 2,
            use_extended_depth_range: true,
            // TODO
            // mapping: FrameMappingType::DepthToColor,
            mapping: FrameMappingType::None,
        }
    }
}

#[derive(Debug)]
pub struct ReceiverThreadData {
    args: ReceiverThreadArgs,

    sensor: *mut INuiSensor,

    color_stream_handle: kinect1_sys::HANDLE,
    color_next_frame_event: windows::Win32::Foundation::HANDLE,
    color_frame: NUI_IMAGE_FRAME,
    color_frame_info: ImageFrameInfo,
    color_locked_rect: NUI_LOCKED_RECT,
    /// stored as BGRA8
    color_frame_data: Vec<u8>,

    depth_stream_handle: kinect1_sys::HANDLE,
    depth_next_frame_event: windows::Win32::Foundation::HANDLE,
    depth_frame: NUI_IMAGE_FRAME,
    depth_frame_info: ImageFrameInfo,
    depth_locked_rect: NUI_LOCKED_RECT,
    /// stored as u16 packed depth and player index
    depth_frame_data: Vec<u16>,
    depth_frame_pixels: Vec<NUI_DEPTH_IMAGE_PIXEL>,
    depth_frame_points: Vec<NUI_DEPTH_IMAGE_POINT>,

    coordinate_mapper: *mut INuiCoordinateMapper,

    // data to send
    have_rgba_data: bool,
    have_depth_data: bool,
    processed_rgba: Vec<u8>,
    processed_depth: Vec<u16>,
    processed_player_index: Vec<u8>,
}

const FRAME_MS_TO_WAIT: u32 = 0;

impl ReceiverThreadData {
    pub fn init_new(args: ReceiverThreadArgs) -> Self {
        let (color_width, color_height) = args.get_color_size();
        let (depth_width, depth_height) = args.get_depth_size();
        // TODO relax this requirement
        assert_eq!((color_width, color_height), (depth_width, depth_height));
        assert_eq!((color_width, color_height), (640, 480));

        let mut out = Self {
            args,
            sensor: null_mut(),

            color_stream_handle: null_mut(),
            color_next_frame_event: Default::default(),
            color_frame: Default::default(),
            color_frame_info: Default::default(),
            color_locked_rect: Default::default(),
            color_frame_data: vec![Default::default(); color_width * color_height * 4],

            depth_stream_handle: null_mut(),
            depth_next_frame_event: Default::default(),
            depth_frame: Default::default(),
            depth_frame_info: Default::default(),
            depth_locked_rect: Default::default(),
            depth_frame_data: vec![Default::default(); depth_width * depth_height],
            depth_frame_pixels: vec![Default::default(); depth_width * depth_height],
            depth_frame_points: vec![Default::default(); depth_width * depth_height],

            coordinate_mapper: null_mut(),

            have_rgba_data: false,
            have_depth_data: false,
            processed_rgba: vec![Default::default(); color_width * color_height],
            processed_depth: vec![Default::default(); depth_width * depth_height],
            processed_player_index: vec![Default::default(); depth_width * depth_height],
        };
        out.init();
        out
    }

    fn init(&mut self) {
        check_fail(unsafe { NuiCreateSensorByIndex(self.args.sensor_index, &mut self.sensor) }).unwrap();

        call_method!(
            self.sensor,
            NuiInitialize,
            NUI_INITIALIZE_FLAG_USES_COLOR
                | NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX
                | NUI_INITIALIZE_FLAG_USES_SKELETON
                | NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA
        );

        self.color_next_frame_event =
            unsafe { windows::Win32::System::Threading::CreateEventW(None, true, false, None).unwrap() };
        self.depth_next_frame_event =
            unsafe { windows::Win32::System::Threading::CreateEventW(None, true, false, None).unwrap() };

        call_method!(
            self.sensor,
            NuiImageStreamOpen,
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            self.args.color_stream_flags,
            self.args.color_buffered_frame_limit,
            std::mem::transmute(self.color_next_frame_event),
            &mut self.color_stream_handle
        );

        call_method!(
            self.sensor,
            NuiImageStreamOpen,
            NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
            self.args.depth_resolution,
            self.args.depth_stream_flags,
            self.args.depth_buffered_frame_limit,
            std::mem::transmute(self.depth_next_frame_event),
            &mut self.depth_stream_handle
        );

        call_method!(self.sensor, NuiGetCoordinateMapper, &mut self.coordinate_mapper);
    }

    fn receive_color_frame(&mut self) {
        self.color_frame = Default::default();
        self.color_locked_rect = Default::default();

        call_method!(
            self.sensor,
            NuiImageStreamGetNextFrame,
            self.color_stream_handle,
            FRAME_MS_TO_WAIT,
            &mut self.color_frame
        );

        call_method!(
            self.color_frame.pFrameTexture,
            LockRect,
            0,
            &mut self.color_locked_rect,
            null_mut(),
            0
        );
        let input_slice =
            unsafe { std::slice::from_raw_parts(self.color_locked_rect.pBits, self.color_locked_rect.size as usize) };
        self.color_frame_data.fill(0);
        self.color_frame_data.copy_from_slice(input_slice);
        self.color_frame_info = ImageFrameInfo::from_image_frame(&self.color_frame);

        call_method!(self.color_frame.pFrameTexture, UnlockRect, 0);
        call_method!(
            self.sensor,
            NuiImageStreamReleaseFrame,
            self.color_stream_handle,
            &mut self.color_frame
        );

        if self.args.mapping == FrameMappingType::DepthToColor {
            // TODO
            warn!("TODO: implement frame mapping")
        }

        self.processed_rgba.clear();
        for &[b, g, r, _a] in cast_slice::<_, [u8; 4]>(&self.color_frame_data) {
            self.processed_rgba.push(r);
            self.processed_rgba.push(g);
            self.processed_rgba.push(b);
            self.processed_rgba.push(255);
        }
        self.have_rgba_data = true;
    }

    fn receive_depth_frame(&mut self) {
        self.depth_frame = Default::default();
        self.depth_locked_rect = Default::default();

        call_method!(
            self.sensor,
            NuiImageStreamGetNextFrame,
            self.depth_stream_handle,
            FRAME_MS_TO_WAIT,
            &mut self.depth_frame
        );

        if self.args.use_extended_depth_range {
            let mut near_mode = 0i32;
            let mut frame_texture: *mut INuiFrameTexture = null_mut();
            call_method!(
                self.sensor,
                NuiImageFrameGetDepthImagePixelFrameTexture,
                self.depth_stream_handle,
                &mut self.depth_frame,
                &mut near_mode,
                &mut frame_texture
            );
            // TODO: use extended frame texture to get the extended depth range data
            call_method!(frame_texture, LockRect, 0, &mut self.depth_locked_rect, null_mut(), 0);
            let input_slice_u8 = unsafe {
                std::slice::from_raw_parts(self.depth_locked_rect.pBits, self.depth_locked_rect.size as usize)
            };
            let input_slice: &[NUI_DEPTH_IMAGE_PIXEL] = cast_slice(input_slice_u8);
            self.depth_frame_pixels.copy_from_slice(input_slice);
            self.depth_frame_info = ImageFrameInfo::from_image_frame(&self.depth_frame);

            call_method!(frame_texture, UnlockRect, 0);
            self.processed_depth.clear();
            self.processed_player_index.clear();
            for &depth_pixel in self.depth_frame_pixels.iter() {
                self.processed_depth.push(depth_pixel.depth);
                self.processed_player_index.push(depth_pixel.playerIndex as u8);
            }
            self.have_depth_data = true;
        } else {
            // TODO: make sure this works
            call_method!(
                self.depth_frame.pFrameTexture,
                LockRect,
                0,
                &mut self.depth_locked_rect,
                null_mut(),
                0
            );
            let input_slice_u8 = unsafe {
                std::slice::from_raw_parts(self.depth_locked_rect.pBits, self.depth_locked_rect.size as usize)
            };
            let input_slice: &[u16] = cast_slice(input_slice_u8);
            self.depth_frame_data.copy_from_slice(input_slice);
            self.depth_frame_info = ImageFrameInfo::from_image_frame(&self.depth_frame);

            call_method!(self.depth_frame.pFrameTexture, UnlockRect, 0);

            self.processed_depth.clear();
            self.processed_player_index.clear();
            for &packed_depth in self.depth_frame_data.iter() {
                self.processed_depth.push(NuiDepthPixelToDepth(packed_depth));
                self.processed_player_index
                    .push(NuiDepthPixelToPlayerIndex(packed_depth) as u8);
            }
            self.have_depth_data = true;
        }

        call_method!(
            self.sensor,
            NuiImageStreamReleaseFrame,
            self.depth_stream_handle,
            &mut self.depth_frame
        );

        if self.args.mapping == FrameMappingType::ColorToDepth {
            // TODO
            warn!("TODO: implement frame mapping")
        }
    }

    pub fn wait_and_receive_next_frame(&mut self) -> KinectFrameMessage {
        // TODO: should we allow sending frames that are partial duplicates?
        let mut have_new_rgba_data = false;
        let mut have_new_depth_data = false;
        // TODO: return a better value than this, so that we don't have to map to the other data type
        let (color_width, color_height) = self.args.get_color_size();
        let (depth_width, depth_height) = self.args.get_depth_size();
        let mut processed_rgb = Vec::with_capacity(color_width * color_height * 3);
        let mut packed_depth_frame = vec![0u16; depth_width * depth_height];

        loop {
            unsafe { WaitForMultipleObjects(&[self.color_next_frame_event, self.depth_next_frame_event], false, 100) };

            if unsafe { WaitForSingleObject(self.color_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_color_frame();
                processed_rgb.clear();
                for &[r, g, b, _a] in cast_slice::<_, [u8; 4]>(&self.processed_rgba) {
                    processed_rgb.push(r);
                    processed_rgb.push(g);
                    processed_rgb.push(b);
                }
                have_new_rgba_data = true;
            }
            if unsafe { WaitForSingleObject(self.depth_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_depth_frame();
                for i in 0..packed_depth_frame.len() {
                    packed_depth_frame[i] = (self.processed_depth[i] << NUI_IMAGE_PLAYER_INDEX_SHIFT)
                        | (self.processed_player_index[i] as u16);
                }
                have_new_depth_data = true;
            }

            // only send a new frame message if we've got two matching frames
            if self.have_rgba_data
                && self.have_depth_data
                && have_new_rgba_data
                && have_new_depth_data
                && (self.color_frame_info.timestamp - self.depth_frame_info.timestamp).abs() <= MAX_ALLOWED_ELAPSED_TIME
                && self.color_frame_info.timestamp != Default::default()
                && self.depth_frame_info.timestamp != Default::default()
            {
                break;
            }
        }

        KinectFrameMessage {
            color_frame: RgbImage::from_vec(color_width as u32, color_height as u32, processed_rgb).unwrap(),
            depth_frame: Gray16Image::from_vec(depth_width as u32, depth_height as u32, packed_depth_frame).unwrap(),
            color_frame_info: self.color_frame_info,
            depth_frame_info: self.depth_frame_info,
        }
    }
}

pub fn start_frame_thread2() -> FrameMessageReceiver {
    let args = ReceiverThreadArgs::default();
    let (sender, receiver) = crossbeam::channel::bounded(2);
    std::thread::spawn(move || {
        let mut thread_data = ReceiverThreadData::init_new(args);
        loop {
            let frame_message = thread_data.wait_and_receive_next_frame();
            match sender.send(frame_message) {
                Ok(_) => (),
                Err(e) => info!("v2: frame receiver hung up, {}", e),
            }
        }
    });
    receiver
}
