use std::ptr::null_mut;

use bytemuck::{cast_slice, try_cast_vec};

use glam::{DVec2, DVec4, IVec2, UVec2, Vec3, Vec4};
use itertools::Itertools;
use kinect1_sys::{
    INuiCoordinateMapper, INuiFrameTexture, INuiSensor, NuiCreateSensorByIndex, NuiDepthPixelToDepth,
    NuiDepthPixelToPlayerIndex, NUI_COLOR_IMAGE_POINT, NUI_DEPTH_IMAGE_PIXEL, NUI_DEPTH_IMAGE_POINT, NUI_IMAGE_FRAME,
    NUI_IMAGE_PLAYER_INDEX_SHIFT, NUI_IMAGE_RESOLUTION, NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA,
    NUI_INITIALIZE_FLAG_USES_COLOR, NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX, NUI_INITIALIZE_FLAG_USES_SKELETON,
    NUI_LOCKED_RECT,
};
pub use kinect1_sys::{
    Vector4, NUI_SKELETON_DATA, NUI_SKELETON_FRAME, NUI_SKELETON_POSITION_TRACKING_STATE, NUI_SKELETON_TRACKING_STATE,
};

use num::complex::ComplexFloat;
use tracing::{info, span};
use tracing::{instrument, Level};
use windows::Win32::{
    Foundation::WAIT_OBJECT_0,
    System::Threading::{WaitForMultipleObjects, WaitForSingleObject},
};

use crate::{
    call_method, check_fail, convert_resolution_to_size,
    skeleton::{
        sk_vector4_to_vector4, SkVector4, SkeletonFrame, SkeletonPositionTrackingState, SKELETON_POSITION_COUNT,
    },
    try_call_method, vtable_method, ImageFrameInfo, MAX_ALLOWED_ELAPSED_TIME, NUI_IMAGE_RESOLUTION_640X480,
    NUI_IMAGE_TYPE_COLOR, NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX,
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

    pub skeleton_stream_enabled: bool,
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
            use_extended_depth_range: false,

            skeleton_stream_enabled: true,
            skeleton_stream_flags: 0,

            // mapping: FrameMappingType::RemapColor,
            frame_registration: FrameRegistrationType::RemapDepth,
            // mapping: FrameMappingType::None,
        }
    }
}

const MIN_DEPTH_MM: u16 = 800;
const MAX_DEPTH_MM: u16 = 4000;

#[derive(Debug)]
pub struct CoordinateMapperWrapper {
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
    skeleton_points: Vec<Vector4>,

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
    processed_skeleton_frame: SkeletonFrame,
    processed_skeleton_points: Vec<Vec3>,
}

const FRAME_MS_TO_WAIT: u32 = 0;

impl CoordinateMapperWrapper {
    pub fn init_new(args: ReceiverThreadArgs) -> Self {
        let (color_width, color_height) = args.get_color_size();
        let (depth_width, depth_height) = args.get_depth_size();
        // TODO relax this requirement
        // assert_eq!((color_width, color_height), (depth_width, depth_height));
        // assert_eq!((color_width, color_height), (640, 480));

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
            skeleton_points: vec![Default::default(); depth_width * depth_height],

            skeleton_next_frame_event: Default::default(),
            skeleton_frame: Default::default(),

            coordinate_mapper: null_mut(),

            have_rgba_data: false,
            have_depth_data: false,
            have_skeleton_data: false,
            processed_rgba: vec![Default::default(); color_width * color_height],
            processed_depth: vec![Default::default(); depth_width * depth_height],
            processed_player_index: vec![Default::default(); depth_width * depth_height],
            processed_skeleton_frame: Default::default(),
            processed_skeleton_points: vec![Default::default(); depth_width * depth_height],
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

    #[allow(non_snake_case)]
    pub fn MapDepthFrameToColorFrame_for_frame(&mut self, packed_depth_frame: &[u16]) -> Vec<DVec2> {
        assert_eq!(packed_depth_frame.len(), self.depth_width * self.depth_height);
        let mut depth_frame_pixels = packed_depth_frame
            .iter()
            .map(|&pd| NUI_DEPTH_IMAGE_PIXEL {
                depth: NuiDepthPixelToDepth(pd),
                playerIndex: NuiDepthPixelToPlayerIndex(pd),
            })
            .collect_vec();
        let mut color_frame_points: Vec<NUI_COLOR_IMAGE_POINT> =
            vec![Default::default(); self.color_width * self.color_height];

        call_method!(
            self.coordinate_mapper,
            MapDepthFrameToColorFrame,
            self.args.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            color_frame_points.len() as u32,
            color_frame_points.as_mut_ptr()
        );

        color_frame_points
            .iter()
            .map(|p| IVec2::new(p.x, p.y).as_dvec2())
            .collect_vec()
    }

    pub fn map_skeleton_frame_to_color_points(&mut self, skeleton_frame: &[DVec4]) -> Vec<DVec2> {
        skeleton_frame
            .iter()
            .map(|&skeleton_point| self.MapSkeletonPointToColorPoint(skeleton_point))
            .collect_vec()
    }
    #[allow(non_snake_case)]
    pub fn MapSkeletonPointToColorPoint(&mut self, skeleton_point: DVec4) -> DVec2 {
        let mut skeleton_point = Vector4 {
            x: skeleton_point.x as f32,
            y: skeleton_point.y as f32,
            z: skeleton_point.z as f32,
            w: skeleton_point.w as f32,
        };

        let mut color_point = NUI_COLOR_IMAGE_POINT::default();
        call_method!(
            self.coordinate_mapper,
            MapSkeletonPointToColorPoint,
            &mut skeleton_point,
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            &mut color_point
        );

        IVec2::new(color_point.x, color_point.y).as_dvec2()
    }

    #[allow(non_snake_case)]
    pub fn MapDepthFrameToColorFrame(&mut self, depth_mm: u16) -> Vec<DVec2> {
        let mut depth_frame_pixels = vec![
            NUI_DEPTH_IMAGE_PIXEL {
                depth: depth_mm,
                playerIndex: 0,
            };
            self.depth_width * self.depth_height
        ];
        let mut color_frame_points = vec![Default::default(); self.color_width * self.color_height];

        call_method!(
            self.coordinate_mapper,
            MapDepthFrameToColorFrame,
            self.args.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            color_frame_points.len() as u32,
            color_frame_points.as_mut_ptr()
        );

        color_frame_points
            .iter()
            .map(|p| IVec2::new(p.x, p.y).as_dvec2())
            .collect_vec()
    }

    #[allow(non_snake_case)]
    pub fn MapColorFrameToSkeletonFrame(&mut self, depth_mm: u16) -> Vec<DVec4> {
        let mut depth_frame_pixels = vec![
            NUI_DEPTH_IMAGE_PIXEL {
                depth: depth_mm,
                playerIndex: 0,
            };
            self.depth_width * self.depth_height
        ];
        let mut skeleton_points = vec![Default::default(); self.color_width * self.color_height];

        call_method!(
            self.coordinate_mapper,
            MapColorFrameToSkeletonFrame,
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            self.args.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            skeleton_points.len() as u32,
            skeleton_points.as_mut_ptr()
        );

        convert_skeleton_points(skeleton_points)
    }

    #[allow(non_snake_case)]
    pub fn MapDepthFrameToSkeletonFrame(&mut self, depth_mm: u16) -> Vec<DVec4> {
        let mut depth_frame_pixels = vec![
            NUI_DEPTH_IMAGE_PIXEL {
                depth: depth_mm,
                playerIndex: 0,
            };
            self.depth_width * self.depth_height
        ];
        let mut skeleton_points = vec![Default::default(); self.depth_width * self.depth_height];

        call_method!(
            self.coordinate_mapper,
            MapDepthFrameToSkeletonFrame,
            self.args.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            skeleton_points.len() as u32,
            skeleton_points.as_mut_ptr()
        );

        convert_skeleton_points(skeleton_points)
    }

    #[allow(non_snake_case)]
    pub fn MapDepthPointToColorPoint(&mut self, x: usize, y: usize, depth_mm: u16) -> IVec2 {
        let mut depth_point = NUI_DEPTH_IMAGE_POINT {
            x: x as i32,
            y: y as i32,
            depth: depth_mm as i32,
            reserved: 0,
        };

        let mut color_point = NUI_COLOR_IMAGE_POINT::default();
        call_method!(
            self.coordinate_mapper,
            MapDepthPointToColorPoint,
            self.args.depth_resolution,
            &mut depth_point,
            NUI_IMAGE_TYPE_COLOR,
            self.args.color_resolution,
            &mut color_point
        );

        IVec2::new(color_point.x, color_point.y)
    }

    pub fn find_depth_color_steps(&mut self, x: usize, y: usize) -> Vec<u16> {
        let mut steps = vec![];
        let mut prev_color_point = self.MapDepthPointToColorPoint(x, y, MIN_DEPTH_MM);
        for depth_mm in MIN_DEPTH_MM..=MAX_DEPTH_MM {
            let color_point = self.MapDepthPointToColorPoint(x, y, depth_mm);
            if color_point != prev_color_point {
                assert_eq!(color_point.y, prev_color_point.y);
                assert!((color_point.x - prev_color_point.x).abs() == 1);
                assert_eq!(color_point.x, prev_color_point.x - 1);
                let x_offset = (x as i32) - color_point.x;
                let y_offset = (y as i32) - color_point.y;
                assert!(x_offset > i8::MIN as i32);
                assert!(x_offset < i8::MAX as i32);
                assert!(y_offset > i8::MIN as i32);
                assert!(y_offset < i8::MAX as i32);

                steps.push(depth_mm);
                prev_color_point = color_point;
            }
        }
        steps
    }

    pub fn build_depth_to_color_mapping(&mut self) -> DepthToColorMapping {
        let depth_steps = self.find_depth_color_steps(0, 0);

        let initial_offsets: Vec<[i8; 2]> = (0..(self.depth_width * self.depth_height))
            .map(|flat_index| {
                let x = flat_index % self.depth_width;
                let y = flat_index / self.depth_width;
                let color_point = self.MapDepthPointToColorPoint(x, y, MIN_DEPTH_MM);
                [(color_point.x - (x as i32)) as i8, (color_point.y - (y as i32)) as i8]
            })
            .collect_vec();

        DepthToColorMapping {
            initial_offsets,
            depth_steps,
        }
    }
}

fn convert_skeleton_points(skeleton_points: Vec<kinect1_sys::_Vector4>) -> Vec<DVec4> {
    skeleton_points
        .iter()
        .map(|v| Vec4::new(v.x, v.y, v.z, v.w).as_dvec4())
        .collect_vec()
}

pub struct DepthToColorMapping {
    pub initial_offsets: Vec<[i8; 2]>,
    pub depth_steps: Vec<u16>,
}
impl DepthToColorMapping {
    pub fn compute_depth_color_offset_frame(&self, packed_depths: &[u16]) -> Vec<[i8; 2]> {
        assert_eq!(packed_depths.len(), self.initial_offsets.len());

        self.initial_offsets
            .iter()
            .zip(packed_depths.iter())
            .map(|(initial_offset, packed_depth)| {
                self.compute_depth_color_offset_helper(*initial_offset, *packed_depth)
            })
            .collect_vec()
    }

    pub fn compute_depth_color_mapping(&self, x: usize, y: usize, packed_depth: u16) -> IVec2 {
        let flat_index = x + y * 640;
        let offset = self.compute_depth_color_offset_helper(self.initial_offsets[flat_index], packed_depth);
        IVec2::new(x as i32, y as i32) + IVec2::new(offset[0] as i32, offset[1] as i32)
    }

    fn compute_depth_color_offset_helper(&self, initial_offset: [i8; 2], packed_depth: u16) -> [i8; 2] {
        let depth_mm = NuiDepthPixelToDepth(packed_depth);
        let mut x = initial_offset[0];
        let y = initial_offset[1];
        for depth_step in self.depth_steps.iter() {
            if depth_mm >= *depth_step {
                x -= 1;
            }
        }
        [x, y]
        // // this is fancier but actually slower apparently
        // if depth_mm < self.depth_steps[0] {
        //     return initial_offset;
        // }
        // for i in 1..self.depth_steps.len() {
        //     if depth_mm >= self.depth_steps[i - 1] && depth_mm < self.depth_steps[i] {
        //         return [initial_offset[0] - i as i8, initial_offset[1]];
        //     }
        // }
        // [initial_offset[0] - self.depth_steps.len() as i8, initial_offset[1]]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_coordinate_mapper_depth_to_color_mapping() {}
// }
