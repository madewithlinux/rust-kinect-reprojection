use std::ptr::null_mut;

use bytemuck::cast_slice;

use kinect1_sys::{
    INuiCoordinateMapper, INuiFrameTexture, INuiSensor, NuiCreateSensorByIndex, NuiDepthPixelToDepth,
    NuiDepthPixelToPlayerIndex, NUI_COLOR_IMAGE_POINT, NUI_DEPTH_IMAGE_PIXEL, NUI_DEPTH_IMAGE_POINT, NUI_IMAGE_FRAME,
    NUI_IMAGE_RESOLUTION, NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA, NUI_INITIALIZE_FLAG_USES_COLOR,
    NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_SKELETON, NUI_LOCKED_RECT,
};
pub use kinect1_sys::{
    Vector4, NUI_SKELETON_DATA, NUI_SKELETON_FRAME, NUI_SKELETON_POSITION_TRACKING_STATE, NUI_SKELETON_TRACKING_STATE,
};

use tracing::info;
use windows::Win32::{
    Foundation::WAIT_OBJECT_0,
    System::Threading::{WaitForMultipleObjects, WaitForSingleObject},
};

use crate::{
    call_method, check_fail, convert_resolution_to_size, try_call_method, vtable_method, ImageFrameInfo,
    MAX_ALLOWED_ELAPSED_TIME, NUI_IMAGE_RESOLUTION_640X480, NUI_IMAGE_TYPE_COLOR,
    NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRegistrationType {
    #[default]
    None,
    RemapDepth,
    RemapColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiverThreadArgs {
    pub sensor_index: i32,
    /// NUI_IMAGE_RESOLUTION
    pub color_resolution: NUI_IMAGE_RESOLUTION,
    pub color_stream_flags: u32,
    pub color_buffered_frame_limit: u32,

    /// NUI_IMAGE_RESOLUTION
    pub depth_resolution: NUI_IMAGE_RESOLUTION,
    pub depth_stream_flags: u32,
    pub depth_buffered_frame_limit: u32,
    pub use_extended_depth_range: bool,

    pub skeleton_stream_flags: u32,

    pub frame_registration: FrameRegistrationType,
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
            skeleton_stream_flags: 0,

            // mapping: FrameMappingType::RemapColor,
            frame_registration: FrameRegistrationType::RemapDepth,
            // mapping: FrameMappingType::None,
        }
    }
}

#[derive(Debug)]
pub struct ReceiverThreadData {
    args: ReceiverThreadArgs,
    color_width: usize,
    color_height: usize,
    depth_width: usize,
    depth_height: usize,

    sensor: *mut INuiSensor,

    color_stream_handle: kinect1_sys::HANDLE,
    color_next_frame_event: windows::Win32::Foundation::HANDLE,
    color_frame: NUI_IMAGE_FRAME,
    color_frame_info: ImageFrameInfo,
    color_locked_rect: NUI_LOCKED_RECT,
    /// stored as BGRA8
    color_frame_data: Vec<u8>,
    color_frame_points: Vec<NUI_COLOR_IMAGE_POINT>,

    depth_stream_handle: kinect1_sys::HANDLE,
    depth_next_frame_event: windows::Win32::Foundation::HANDLE,
    depth_frame: NUI_IMAGE_FRAME,
    depth_frame_info: ImageFrameInfo,
    depth_locked_rect: NUI_LOCKED_RECT,
    /// stored as u16 packed depth and player index
    // depth_frame_data: Vec<u16>,
    depth_frame_pixels: Vec<NUI_DEPTH_IMAGE_PIXEL>,
    depth_frame_points: Vec<NUI_DEPTH_IMAGE_POINT>,

    skeleton_next_frame_event: windows::Win32::Foundation::HANDLE,
    skeleton_frame: NUI_SKELETON_FRAME,

    coordinate_mapper: *mut INuiCoordinateMapper,

    // data to send
    have_rgba_data: bool,
    have_depth_data: bool,
    have_skeleton_data: bool,
    processed_rgba: Vec<[u8; 4]>,
    processed_depth: Vec<u16>,
    processed_player_index: Vec<u8>,
    /// indexes into depth data
    processed_skeleton_points: [[usize; 20]; 6],
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
            color_width,
            color_height,
            depth_width,
            depth_height,

            sensor: null_mut(),

            color_stream_handle: null_mut(),
            color_next_frame_event: Default::default(),
            color_frame: Default::default(),
            color_frame_info: Default::default(),
            color_locked_rect: Default::default(),
            color_frame_data: vec![Default::default(); color_width * color_height * 4],
            color_frame_points: vec![Default::default(); color_width * color_height],

            depth_stream_handle: null_mut(),
            depth_next_frame_event: Default::default(),
            depth_frame: Default::default(),
            depth_frame_info: Default::default(),
            depth_locked_rect: Default::default(),
            // depth_frame_data: vec![Default::default(); depth_width * depth_height],
            depth_frame_pixels: vec![Default::default(); depth_width * depth_height],
            depth_frame_points: vec![Default::default(); depth_width * depth_height],

            skeleton_next_frame_event: Default::default(),
            skeleton_frame: Default::default(),

            coordinate_mapper: null_mut(),

            have_rgba_data: false,
            have_depth_data: false,
            have_skeleton_data: false,
            processed_rgba: vec![Default::default(); color_width * color_height],
            processed_depth: vec![Default::default(); depth_width * depth_height],
            processed_player_index: vec![Default::default(); depth_width * depth_height],
            processed_skeleton_points: Default::default(),
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
        self.skeleton_next_frame_event =
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

        call_method!(
            self.sensor,
            NuiSkeletonTrackingEnable,
            std::mem::transmute(self.skeleton_next_frame_event),
            self.args.skeleton_stream_flags
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

        if self.args.frame_registration == FrameRegistrationType::RemapColor && self.have_depth_data {
            call_method!(
                self.coordinate_mapper,
                MapDepthFrameToColorFrame,
                self.args.depth_resolution,
                self.depth_frame_pixels.len() as u32,
                self.depth_frame_pixels.as_mut_ptr(),
                NUI_IMAGE_TYPE_COLOR,
                self.args.color_resolution,
                self.color_frame_points.len() as u32,
                self.color_frame_points.as_mut_ptr()
            );
            let bgra_color_frame = cast_slice::<_, [u8; 4]>(&self.color_frame_data);
            for (i, &point) in self.color_frame_points.iter().enumerate() {
                let [b, g, r, _a] = if point.x > 0
                    && point.y > 0
                    && point.x < (self.color_frame_info.width as i32)
                    && point.y < (self.color_frame_info.height as i32)
                {
                    let index = (point.y as usize) * self.color_frame_info.width + (point.x as usize);
                    bgra_color_frame[index]
                } else {
                    [0, 0, 0, 255]
                };
                self.processed_rgba[i] = [r, g, b, 255];
            }
        } else {
            for (i, &[b, g, r, _a]) in cast_slice::<_, [u8; 4]>(&self.color_frame_data).iter().enumerate() {
                self.processed_rgba[i] = [r, g, b, 255];
            }
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
            call_method!(frame_texture, LockRect, 0, &mut self.depth_locked_rect, null_mut(), 0);
            let input_slice_u8 = unsafe {
                std::slice::from_raw_parts(self.depth_locked_rect.pBits, self.depth_locked_rect.size as usize)
            };
            let input_slice: &[NUI_DEPTH_IMAGE_PIXEL] = cast_slice(input_slice_u8);
            self.depth_frame_pixels.copy_from_slice(input_slice);
            self.depth_frame_info = ImageFrameInfo::from_image_frame(&self.depth_frame);

            call_method!(frame_texture, UnlockRect, 0);
        } else {
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
            for (i, &packed_depth) in input_slice.iter().enumerate() {
                self.depth_frame_pixels[i].depth = NuiDepthPixelToDepth(packed_depth);
                self.depth_frame_pixels[i].playerIndex = NuiDepthPixelToPlayerIndex(packed_depth);
            }
            self.depth_frame_info = ImageFrameInfo::from_image_frame(&self.depth_frame);

            call_method!(self.depth_frame.pFrameTexture, UnlockRect, 0);
        }

        call_method!(
            self.sensor,
            NuiImageStreamReleaseFrame,
            self.depth_stream_handle,
            &mut self.depth_frame
        );

        if self.args.frame_registration == FrameRegistrationType::RemapDepth {
            call_method!(
                self.coordinate_mapper,
                MapColorFrameToDepthFrame,
                NUI_IMAGE_TYPE_COLOR,
                self.args.color_resolution,
                self.args.depth_resolution,
                self.depth_frame_pixels.len() as u32,
                self.depth_frame_pixels.as_mut_ptr(),
                self.depth_frame_points.len() as u32,
                self.depth_frame_points.as_mut_ptr()
            );
            for (i, &point) in self.depth_frame_points.iter().enumerate() {
                // find the original depth pixel because the depth point doesn't have the player index
                let pixel = if point.x > 0
                    && point.y > 0
                    && point.x < (self.depth_frame_info.width as i32)
                    && point.y < (self.depth_frame_info.height as i32)
                {
                    let index = (point.y as usize) * self.depth_frame_info.width + (point.x as usize);
                    self.depth_frame_pixels[index]
                } else {
                    NUI_DEPTH_IMAGE_PIXEL::default()
                };
                self.processed_depth[i] = pixel.depth;
                self.processed_player_index[i] = pixel.playerIndex as u8;
            }
        } else {
            for (i, &depth_pixel) in self.depth_frame_pixels.iter().enumerate() {
                self.processed_depth[i] = depth_pixel.depth;
                self.processed_player_index[i] = depth_pixel.playerIndex as u8;
            }
        }
        self.have_depth_data = true;
    }

    fn receive_skeleton_frame(&mut self) {
        call_method!(
            self.sensor,
            NuiSkeletonGetNextFrame,
            FRAME_MS_TO_WAIT,
            &mut self.skeleton_frame
        );

        for (i, skeleton) in self.skeleton_frame.SkeletonData.iter().enumerate() {
            for (j, skeleton_point) in skeleton.SkeletonPositions.iter().enumerate() {
                let mut skeleton_point = skeleton_point.clone();
                let mut color_point: NUI_COLOR_IMAGE_POINT = Default::default();
                if skeleton_point.w != 0.0
                    || skeleton_point.x != 0.0
                    || skeleton_point.y != 0.0
                    || skeleton_point.z != 0.0
                {
                    call_method!(
                        self.coordinate_mapper,
                        MapSkeletonPointToColorPoint,
                        &mut skeleton_point,
                        NUI_IMAGE_TYPE_COLOR,
                        self.args.color_resolution,
                        &mut color_point
                    );
                }
                let color_index = (color_point.x as usize) + (color_point.y as usize) * self.color_width;
                self.processed_skeleton_points[i][j] = color_index;
            }
        }

        self.have_skeleton_data = true;
    }

    pub fn wait_and_receive_next_frame(&mut self) -> FrameMessage {
        let width = self.color_width;
        let height = self.color_height;
        self.processed_rgba.resize(width * height, Default::default());
        self.processed_depth.resize(width * height, Default::default());
        self.processed_player_index.resize(width * height, Default::default());

        // TODO: should we allow sending frames that are partial duplicates?
        let mut have_new_rgba_data = false;
        let mut have_new_depth_data = false;
        let mut have_new_skeleton_data = false;

        loop {
            unsafe {
                WaitForMultipleObjects(
                    &[
                        self.color_next_frame_event,
                        self.depth_next_frame_event,
                        self.skeleton_next_frame_event,
                    ],
                    false,
                    100,
                )
            };

            if unsafe { WaitForSingleObject(self.color_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_color_frame();
                have_new_rgba_data = true;
            }
            if unsafe { WaitForSingleObject(self.depth_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_depth_frame();
                have_new_depth_data = true;
            }
            if unsafe { WaitForSingleObject(self.skeleton_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_skeleton_frame();
                have_new_skeleton_data = true;
            }

            // only send a new frame message if we've got two matching frames
            if self.have_rgba_data
                && self.have_depth_data
                && self.have_skeleton_data
                && have_new_rgba_data
                && have_new_depth_data
                && have_new_skeleton_data
                && (self.color_frame_info.timestamp - self.depth_frame_info.timestamp).abs() <= MAX_ALLOWED_ELAPSED_TIME
                && self.color_frame_info.timestamp != Default::default()
                && self.depth_frame_info.timestamp != Default::default()
            {
                break;
            }
        }

        // if self
        //     .skeleton_frame
        //     .SkeletonData
        //     .iter()
        //     .any(|&sk| format!("{:?}", sk) != format!("{:?}", NUI_SKELETON_DATA::default()))
        // {
        //     info!("skeleton_frame: {:?}", self.skeleton_frame);
        //     info!("self.processed_skeleton_points: {:?}", self.processed_skeleton_points);
        // }

        FrameMessage {
            width,
            height,
            // these will be re-initialized for the next frame
            rgba: std::mem::take(&mut self.processed_rgba),
            depth: std::mem::take(&mut self.processed_depth),
            player_index: std::mem::take(&mut self.processed_player_index),
            skeleton_frame: self.skeleton_frame,
            skeleton_points: self.processed_skeleton_points,
            color_frame_info: self.color_frame_info,
            depth_frame_info: self.depth_frame_info,
        }
    }
}

#[derive(Clone, Default)]
pub struct FrameMessage {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<[u8; 4]>,
    pub depth: Vec<u16>,
    pub player_index: Vec<u8>,
    pub skeleton_frame: NUI_SKELETON_FRAME,
    pub skeleton_points: [[usize; 20]; 6],
    // raw fields from the kinect itself
    pub color_frame_info: ImageFrameInfo,
    pub depth_frame_info: ImageFrameInfo,
}

pub type FrameMessageReceiver = crossbeam::channel::Receiver<FrameMessage>;

pub fn start_frame_thread2() -> FrameMessageReceiver {
    let args = ReceiverThreadArgs::default();
    let (sender, receiver) = crossbeam::channel::bounded(2);
    std::thread::spawn(move || {
        let mut thread_data = ReceiverThreadData::init_new(args);
        loop {
            let frame_message = thread_data.wait_and_receive_next_frame();
            match sender.send(frame_message) {
                Ok(_) => (),
                Err(e) => info!("frame receiver hung up, {}", e),
            }
        }
    });
    receiver
}
