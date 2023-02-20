use std::ptr::null_mut;

use bytemuck::cast_slice;

use kinect1_sys::{
    INuiCoordinateMapper, INuiSensor, NuiCreateSensorByIndex, NUI_COLOR_IMAGE_POINT, NUI_IMAGE_FRAME,
    NUI_IMAGE_RESOLUTION, NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA, NUI_INITIALIZE_FLAG_USES_COLOR,
    NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_SKELETON, NUI_LOCKED_RECT,
};
pub use kinect1_sys::{
    Vector4, NUI_SKELETON_DATA, NUI_SKELETON_FRAME, NUI_SKELETON_POSITION_TRACKING_STATE, NUI_SKELETON_TRACKING_STATE,
};

use tracing::{info, span};
use tracing::{instrument, Level};
use windows::Win32::{
    Foundation::WAIT_OBJECT_0,
    System::Threading::{WaitForMultipleObjects, WaitForSingleObject},
};

use crate::custom_coordinate_mapper::{CustomCoordinateMapperBuilder, DepthToColorMapping, DepthToSkeletonPointMapping};
use crate::packed_depth::PackedDepth;
use crate::{
    call_method, check_fail, convert_resolution_to_size,
    skeleton::{
        sk_vector4_to_vector4, SkVector4, SkeletonFrame, SkeletonPositionTrackingState, SKELETON_POSITION_COUNT,
    },
    try_call_method, vtable_method, ImageFrameInfo, MAX_ALLOWED_ELAPSED_TIME, NUI_IMAGE_RESOLUTION_640X480,
    NUI_IMAGE_TYPE_COLOR, NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KinectSensorWrapperArgs {
    pub sensor_index: i32,
    /// NUI_IMAGE_RESOLUTION
    pub color_resolution: NUI_IMAGE_RESOLUTION,
    pub color_stream_flags: u32,
    pub color_buffered_frame_limit: u32,

    /// NUI_IMAGE_RESOLUTION
    pub depth_resolution: NUI_IMAGE_RESOLUTION,
    pub depth_stream_flags: u32,
    pub depth_buffered_frame_limit: u32,

    pub skeleton_stream_enabled: bool,
    pub skeleton_stream_flags: u32,
}

impl KinectSensorWrapperArgs {
    pub fn get_color_size(&self) -> (usize, usize) {
        convert_resolution_to_size(self.color_resolution)
    }
    pub fn get_depth_size(&self) -> (usize, usize) {
        convert_resolution_to_size(self.depth_resolution)
    }
}

impl Default for KinectSensorWrapperArgs {
    fn default() -> Self {
        Self {
            sensor_index: 0,
            color_resolution: NUI_IMAGE_RESOLUTION_640X480,
            color_stream_flags: 0,
            color_buffered_frame_limit: 2,
            depth_resolution: NUI_IMAGE_RESOLUTION_640X480,
            depth_stream_flags: 0,
            depth_buffered_frame_limit: 2,

            skeleton_stream_enabled: true,
            skeleton_stream_flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct KinectWrapper {
    args: KinectSensorWrapperArgs,
    color_width: usize,
    color_height: usize,
    // depth_width: usize,
    // depth_height: usize,
    sensor: *mut INuiSensor,

    color_stream_handle: kinect1_sys::HANDLE,
    color_next_frame_event: windows::Win32::Foundation::HANDLE,
    color_frame: NUI_IMAGE_FRAME,
    color_frame_info: ImageFrameInfo,
    color_locked_rect: NUI_LOCKED_RECT,

    depth_stream_handle: kinect1_sys::HANDLE,
    depth_next_frame_event: windows::Win32::Foundation::HANDLE,
    depth_frame: NUI_IMAGE_FRAME,
    depth_frame_info: ImageFrameInfo,
    depth_locked_rect: NUI_LOCKED_RECT,

    skeleton_next_frame_event: windows::Win32::Foundation::HANDLE,
    skeleton_frame: NUI_SKELETON_FRAME,

    coordinate_mapper: *mut INuiCoordinateMapper,

    // data to send
    have_rgba_data: bool,
    have_depth_data: bool,
    have_skeleton_data: bool,
    processed_rgba: Vec<[u8; 4]>,
    processed_packed_depth: Vec<PackedDepth>,
    processed_skeleton_frame: SkeletonFrame,
}

const FRAME_MS_TO_WAIT: u32 = 0;

impl KinectWrapper {
    pub fn init_new(args: KinectSensorWrapperArgs) -> Self {
        let (color_width, color_height) = args.get_color_size();
        let (depth_width, depth_height) = args.get_depth_size();
        // TODO relax this requirement
        assert_eq!((color_width, color_height), (depth_width, depth_height));
        assert_eq!((color_width, color_height), (640, 480));

        let mut out = Self {
            args,
            color_width,
            color_height,
            // depth_width,
            // depth_height,
            sensor: null_mut(),

            color_stream_handle: null_mut(),
            color_next_frame_event: Default::default(),
            color_frame: Default::default(),
            color_frame_info: Default::default(),
            color_locked_rect: Default::default(),

            depth_stream_handle: null_mut(),
            depth_next_frame_event: Default::default(),
            depth_frame: Default::default(),
            depth_frame_info: Default::default(),
            depth_locked_rect: Default::default(),

            skeleton_next_frame_event: Default::default(),
            skeleton_frame: Default::default(),

            coordinate_mapper: null_mut(),

            have_rgba_data: false,
            have_depth_data: false,
            have_skeleton_data: false,
            processed_rgba: vec![Default::default(); color_width * color_height],
            processed_packed_depth: vec![Default::default(); depth_width * depth_height],
            processed_skeleton_frame: Default::default(),
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
                | NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA
                | if self.args.skeleton_stream_enabled {
                    NUI_INITIALIZE_FLAG_USES_SKELETON
                } else {
                    0
                }
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

        if self.args.skeleton_stream_enabled {
            call_method!(
                self.sensor,
                NuiSkeletonTrackingEnable,
                std::mem::transmute(self.skeleton_next_frame_event),
                self.args.skeleton_stream_flags
            );
        }

        call_method!(self.sensor, NuiGetCoordinateMapper, &mut self.coordinate_mapper);
    }

    #[instrument(skip_all)]
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
        let input_slide_bgra = cast_slice::<_, [u8; 4]>(input_slice);
        assert_eq!(input_slide_bgra.len(), self.processed_rgba.len());
        for (rgba, &[b, g, r, _a]) in self.processed_rgba.iter_mut().zip(input_slide_bgra.iter()) {
            *rgba = [r, g, b, 255];
        }

        self.color_frame_info = ImageFrameInfo::from_image_frame(&self.color_frame);

        call_method!(self.color_frame.pFrameTexture, UnlockRect, 0);
        call_method!(
            self.sensor,
            NuiImageStreamReleaseFrame,
            self.color_stream_handle,
            &mut self.color_frame
        );

        self.have_rgba_data = true;
    }

    #[instrument(skip_all)]
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

        call_method!(
            self.depth_frame.pFrameTexture,
            LockRect,
            0,
            &mut self.depth_locked_rect,
            null_mut(),
            0
        );
        let input_slice_u8 =
            unsafe { std::slice::from_raw_parts(self.depth_locked_rect.pBits, self.depth_locked_rect.size as usize) };
        let input_slice: &[PackedDepth] = cast_slice(input_slice_u8);
        assert_eq!(input_slice.len(), self.processed_packed_depth.len());
        self.processed_packed_depth.copy_from_slice(input_slice);
        self.depth_frame_info = ImageFrameInfo::from_image_frame(&self.depth_frame);

        call_method!(self.depth_frame.pFrameTexture, UnlockRect, 0);
        call_method!(
            self.sensor,
            NuiImageStreamReleaseFrame,
            self.depth_stream_handle,
            &mut self.depth_frame
        );

        self.have_depth_data = true;
    }

    #[instrument(skip_all)]
    fn receive_skeleton_frame(&mut self) {
        call_method!(
            self.sensor,
            NuiSkeletonGetNextFrame,
            FRAME_MS_TO_WAIT,
            &mut self.skeleton_frame
        );

        let transform_skeleton_point_to_color_index =
            |skeleton_point: &SkVector4, point_tracking_state: SkeletonPositionTrackingState| {
                let mut skeleton_point = sk_vector4_to_vector4(skeleton_point);
                let mut color_point: NUI_COLOR_IMAGE_POINT = Default::default();
                if point_tracking_state != SkeletonPositionTrackingState::NotTracked {
                    call_method!(
                        self.coordinate_mapper,
                        MapSkeletonPointToColorPoint,
                        &mut skeleton_point,
                        NUI_IMAGE_TYPE_COLOR,
                        self.args.color_resolution,
                        &mut color_point
                    );
                    if color_point.x < 0 || color_point.y < 0 {
                        0
                    } else {
                        (color_point.x as usize) + (color_point.y as usize) * self.color_width
                    }
                } else {
                    0
                }
            };

        self.processed_skeleton_frame = SkeletonFrame::from(&self.skeleton_frame);
        for skeleton in self.processed_skeleton_frame.skeleton_data.iter_mut() {
            for i in 0..SKELETON_POSITION_COUNT {
                skeleton.skeleton_pixel_indexes[i] = transform_skeleton_point_to_color_index(
                    &skeleton.skeleton_positions[i],
                    skeleton.skeleton_position_tracking_state[i],
                );
            }
        }

        self.have_skeleton_data = true;
    }

    #[instrument(skip_all)]
    pub fn wait_and_receive_next_frame(&mut self) -> RgbaDepthFrame {
        let width = self.color_width;
        let height = self.color_height;
        {
            let span = span!(Level::INFO, "allocate");
            let _enter = span.enter();
            self.processed_rgba.resize(width * height, Default::default());
            self.processed_packed_depth.resize(width * height, Default::default());
            self.processed_skeleton_frame = Default::default();
        }

        let mut have_new_rgba_data = false;
        let mut have_new_depth_data = false;
        let mut have_new_skeleton_data = false;

        loop {
            unsafe {
                if self.args.skeleton_stream_enabled {
                    WaitForMultipleObjects(
                        &[
                            self.color_next_frame_event,
                            self.depth_next_frame_event,
                            self.skeleton_next_frame_event,
                        ],
                        false,
                        100,
                    )
                } else {
                    WaitForMultipleObjects(&[self.color_next_frame_event, self.depth_next_frame_event], false, 100)
                }
            };

            if unsafe { WaitForSingleObject(self.color_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_color_frame();
                have_new_rgba_data = true;
            }
            if unsafe { WaitForSingleObject(self.depth_next_frame_event, 0) } == WAIT_OBJECT_0 {
                self.receive_depth_frame();
                have_new_depth_data = true;
            }
            if self.args.skeleton_stream_enabled
                && unsafe { WaitForSingleObject(self.skeleton_next_frame_event, 0) } == WAIT_OBJECT_0
            {
                self.receive_skeleton_frame();
                have_new_skeleton_data = true;
            }

            // only send a new frame message if we've got two matching frames
            if self.have_rgba_data
                && self.have_depth_data
                && (self.have_skeleton_data || !self.args.skeleton_stream_enabled)
                && have_new_rgba_data
                && have_new_depth_data
                && (have_new_skeleton_data || !self.args.skeleton_stream_enabled)
                && (self.color_frame_info.timestamp - self.depth_frame_info.timestamp).abs() <= MAX_ALLOWED_ELAPSED_TIME
                && self.color_frame_info.timestamp != Default::default()
                && self.depth_frame_info.timestamp != Default::default()
            {
                break;
            }
        }

        RgbaDepthFrame {
            // these will be re-initialized for the next frame
            rgba: std::mem::take(&mut self.processed_rgba),
            packed_depth: std::mem::take(&mut self.processed_packed_depth),
            skeleton_frame: std::mem::take(&mut self.processed_skeleton_frame),
            color_frame_info: self.color_frame_info,
            depth_frame_info: self.depth_frame_info,
        }
    }

    pub fn try_receive_next_frame(&mut self) -> Option<RgbaDepthFrame> {
        todo!()
    }

    pub fn build_depth_to_color_mapping(&mut self) -> DepthToColorMapping {
        CustomCoordinateMapperBuilder::new(
            self.coordinate_mapper,
            self.args.color_resolution,
            self.args.depth_resolution,
        )
        .build_depth_to_color_mapping()
    }

    pub fn build_depth_to_skeleton_point_mapping(&mut self) -> DepthToSkeletonPointMapping {
        CustomCoordinateMapperBuilder::new(
            self.coordinate_mapper,
            self.args.color_resolution,
            self.args.depth_resolution,
        )
        .build_depth_to_skeleton_point_mapping()
    }
}

#[derive(Clone, Default)]
pub struct RgbaDepthFrame {
    pub rgba: Vec<[u8; 4]>,
    pub packed_depth: Vec<PackedDepth>,
    pub skeleton_frame: SkeletonFrame,

    // raw fields from the kinect itself
    pub color_frame_info: ImageFrameInfo,
    pub depth_frame_info: ImageFrameInfo,
}

pub type FrameMessageReceiver = crossbeam::channel::Receiver<RgbaDepthFrame>;

pub fn start_frame_thread3(args: KinectSensorWrapperArgs) -> FrameMessageReceiver {
    // let args = ReceiverThreadArgs::default();
    let (sender, receiver) = crossbeam::channel::bounded(2);
    std::thread::spawn(move || {
        let mut thread_data = KinectWrapper::init_new(args);
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
