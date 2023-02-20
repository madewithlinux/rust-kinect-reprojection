use glam::{IVec2, Vec3, Vec4};
use itertools::Itertools;
use kinect1_sys::{
    INuiCoordinateMapper, Vector4, NUI_COLOR_IMAGE_POINT, NUI_DEPTH_IMAGE_PIXEL, NUI_DEPTH_IMAGE_POINT,
    NUI_IMAGE_RESOLUTION,
};

use crate::packed_depth::{PackedDepth, MAX_DEPTH_MM, MIN_DEPTH_MM};
use crate::{
    call_method, check_fail, convert_resolution_to_size, try_call_method, vtable_method, NUI_IMAGE_TYPE_COLOR,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DepthToColorMapping {
    pub initial_offsets: Vec<[i8; 2]>,
    pub depth_steps: Vec<u16>,
}
impl DepthToColorMapping {
    pub fn compute_depth_color_offset_frame(&self, packed_depths: &[PackedDepth]) -> Vec<[i8; 2]> {
        assert_eq!(packed_depths.len(), self.initial_offsets.len());

        self.initial_offsets
            .iter()
            .zip(packed_depths.iter())
            .map(|(initial_offset, packed_depth)| {
                self.compute_depth_color_offset_helper(*initial_offset, *packed_depth)
            })
            .collect_vec()
    }

    pub fn compute_depth_color_mapping(&self, x: usize, y: usize, packed_depth: PackedDepth) -> IVec2 {
        let flat_index = x + y * 640;
        let offset = self.compute_depth_color_offset_helper(self.initial_offsets[flat_index], packed_depth);
        IVec2::new(x as i32, y as i32) + IVec2::new(offset[0] as i32, offset[1] as i32)
    }

    fn compute_depth_color_offset_helper(&self, initial_offset: [i8; 2], packed_depth: PackedDepth) -> [i8; 2] {
        let depth_mm = packed_depth.depth_mm();
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DepthToSkeletonPointMapping {
    pub origin: Vec<Vec3>,
    pub depth_mm_vector: Vec<Vec3>,
}
impl DepthToSkeletonPointMapping {
    // TODO
}

#[derive(Debug)]
pub(crate) struct CustomCoordinateMapperBuilder {
    coordinate_mapper: *mut INuiCoordinateMapper,
    color_resolution: NUI_IMAGE_RESOLUTION,
    color_width: usize,
    color_height: usize,
    depth_resolution: NUI_IMAGE_RESOLUTION,
    depth_width: usize,
    depth_height: usize,
}

impl CustomCoordinateMapperBuilder {
    pub(crate) fn new(
        coordinate_mapper: *mut INuiCoordinateMapper,
        color_resolution: NUI_IMAGE_RESOLUTION,
        depth_resolution: NUI_IMAGE_RESOLUTION,
    ) -> Self {
        let (color_width, color_height) = convert_resolution_to_size(color_resolution);
        let (depth_width, depth_height) = convert_resolution_to_size(depth_resolution);
        Self {
            coordinate_mapper,
            color_resolution,
            color_width,
            color_height,
            depth_resolution,
            depth_width,
            depth_height,
        }
    }

    #[allow(non_snake_case)]
    fn MapDepthPointToSkeletonPoint(&mut self, x: usize, y: usize, depth_mm: u16) -> Vec4 {
        let mut depth_point = NUI_DEPTH_IMAGE_POINT {
            x: x as i32,
            y: y as i32,
            depth: depth_mm as i32,
            reserved: 0,
        };

        let mut skeleton_point = Vector4::default();
        call_method!(
            self.coordinate_mapper,
            MapDepthPointToSkeletonPoint,
            self.depth_resolution,
            &mut depth_point,
            &mut skeleton_point
        );

        Vec4::new(skeleton_point.x, skeleton_point.y, skeleton_point.z, skeleton_point.w)
    }

    #[allow(non_snake_case)]
    fn MapDepthFrameToSkeletonFrame(&mut self, depth_mm: u16) -> Vec<Vec4> {
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
            self.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            skeleton_points.len() as u32,
            skeleton_points.as_mut_ptr()
        );

        skeleton_points
            .iter()
            .map(|v| Vec4::new(v.x, v.y, v.z, v.w))
            .collect_vec()
    }

    pub fn build_depth_to_skeleton_point_mapping(&mut self) -> DepthToSkeletonPointMapping {
        todo!()
    }

    #[allow(non_snake_case)]
    fn MapDepthFrameToColorFrame(&mut self, depth_mm: u16) -> Vec<IVec2> {
        let mut depth_frame_pixels = vec![
            NUI_DEPTH_IMAGE_PIXEL {
                depth: depth_mm,
                playerIndex: 0,
            };
            self.depth_width * self.depth_height
        ];
        let mut color_points = vec![Default::default(); self.color_width * self.color_height];

        call_method!(
            self.coordinate_mapper,
            MapDepthFrameToColorFrame,
            self.depth_resolution,
            depth_frame_pixels.len() as u32,
            depth_frame_pixels.as_mut_ptr(),
            NUI_IMAGE_TYPE_COLOR,
            self.color_resolution,
            color_points.len() as u32,
            color_points.as_mut_ptr()
        );

        color_points
            .iter()
            .map(|color_point| IVec2::new(color_point.x, color_point.y))
            .collect_vec()
    }

    #[allow(non_snake_case)]
    fn MapDepthPointToColorPoint(&mut self, x: usize, y: usize, depth_mm: u16) -> IVec2 {
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
            self.depth_resolution,
            &mut depth_point,
            NUI_IMAGE_TYPE_COLOR,
            self.color_resolution,
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

        // TODO: test the mapping before returning it
        DepthToColorMapping {
            initial_offsets,
            depth_steps,
        }
    }
}
