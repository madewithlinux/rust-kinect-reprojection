use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;
use kinect1_sys::{NUI_SKELETON_COUNT, _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_COUNT};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;

// TODO: remove this? since it prob won't end up getting used
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
// pub struct SkeletonPoints {
//     pub hip_center: usize,
//     pub spine: usize,
//     pub shoulder_center: usize,
//     pub head: usize,
//     pub shoulder_left: usize,
//     pub elbow_left: usize,
//     pub wrist_left: usize,
//     pub hand_left: usize,
//     pub shoulder_right: usize,
//     pub elbow_right: usize,
//     pub wrist_right: usize,
//     pub hand_right: usize,
//     pub hip_left: usize,
//     pub knee_left: usize,
//     pub ankle_left: usize,
//     pub foot_left: usize,
//     pub hip_right: usize,
//     pub knee_right: usize,
//     pub ankle_right: usize,
//     pub foot_right: usize,
// }

pub const SKELETON_POSITION_COUNT: usize = _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_COUNT as usize;
pub const SKELETON_COUNT: usize = NUI_SKELETON_COUNT as usize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum SkeletonPositionIndex {
    #[default]
    HipCenter = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_CENTER as isize,
    Spine = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SPINE as isize,
    ShoulderCenter = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_CENTER as isize,
    Head = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HEAD as isize,
    ShoulderLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_LEFT as isize,
    ElbowLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ELBOW_LEFT as isize,
    WristLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_WRIST_LEFT as isize,
    HandLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HAND_LEFT as isize,
    ShoulderRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_RIGHT as isize,
    ElbowRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ELBOW_RIGHT as isize,
    WristRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_WRIST_RIGHT as isize,
    HandRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HAND_RIGHT as isize,
    HipLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_LEFT as isize,
    KneeLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_KNEE_LEFT as isize,
    AnkleLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ANKLE_LEFT as isize,
    FootLeft = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_FOOT_LEFT as isize,
    HipRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_RIGHT as isize,
    KneeRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_KNEE_RIGHT as isize,
    AnkleRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ANKLE_RIGHT as isize,
    FootRight = kinect1_sys::_NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_FOOT_RIGHT as isize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum SkeletonPositionTrackingState {
    #[default]
    NotTracked = kinect1_sys::_NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_NOT_TRACKED as isize,
    Inferred = kinect1_sys::_NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_INFERRED as isize,
    Tracked = kinect1_sys::_NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_TRACKED as isize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum SkeletonTrackingState {
    #[default]
    NotTracked = kinect1_sys::_NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_NOT_TRACKED as isize,
    PositionOnly = kinect1_sys::_NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_POSITION_ONLY as isize,
    Tracked = kinect1_sys::_NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_TRACKED as isize,
}

#[derive(Debug, EnumSetType)]
#[enumset(repr = "u32")]
pub enum SkeletonQualityFlags {
    // these are the bit offset, not the actual mask
    ClippedRight = 0,
    ClippedLeft = 1,
    ClippedTop = 2,
    ClippedBottom = 3,
}

pub type SkVector4 = [OrderedFloat<f32>; 4];

pub(crate) fn vector4_to_sk_vector4(v: &kinect1_sys::Vector4) -> SkVector4 {
    [
        OrderedFloat(v.x),
        OrderedFloat(v.y),
        OrderedFloat(v.z),
        OrderedFloat(v.w),
    ]
}

pub(crate) fn sk_vector4_to_vector4(v: &SkVector4) -> kinect1_sys::Vector4 {
    kinect1_sys::Vector4 {
        x: v[0].0,
        y: v[1].0,
        z: v[2].0,
        w: v[3].0,
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SkeletonData {
    pub tracking_state: SkeletonTrackingState,
    pub tracking_id: u32,
    pub enrollment_index: u32,
    pub user_index: u32,
    pub position: SkVector4,
    pub skeleton_positions: [SkVector4; SKELETON_POSITION_COUNT],
    pub skeleton_position_tracking_state: [SkeletonPositionTrackingState; SKELETON_POSITION_COUNT],
    pub quality_flags: EnumSet<SkeletonQualityFlags>,
    /// additional data, not directly from the API. indexes into depth data
    pub skeleton_pixel_indexes: [usize; SKELETON_POSITION_COUNT],
}
#[derive(Debug, Default, Clone, Copy)]
pub struct SkeletonPositionData {
    pub index: SkeletonPositionIndex,
    pub position: SkVector4,
    pub tracking_state: SkeletonPositionTrackingState,
    pub pixel_index: usize,
}

pub const SKELETON_BONES_COUNT: usize = 19;
pub const SKELETON_BONES_INDEXES: [[SkeletonPositionIndex; 2]; SKELETON_BONES_COUNT] = [
    [SkeletonPositionIndex::Head, SkeletonPositionIndex::ShoulderCenter],
    [
        SkeletonPositionIndex::ShoulderCenter,
        SkeletonPositionIndex::ShoulderLeft,
    ],
    [
        SkeletonPositionIndex::ShoulderCenter,
        SkeletonPositionIndex::ShoulderRight,
    ],
    [SkeletonPositionIndex::ShoulderCenter, SkeletonPositionIndex::Spine],
    [SkeletonPositionIndex::Spine, SkeletonPositionIndex::HipCenter],
    [SkeletonPositionIndex::HipCenter, SkeletonPositionIndex::HipLeft],
    [SkeletonPositionIndex::HipCenter, SkeletonPositionIndex::HipRight],
    [SkeletonPositionIndex::ShoulderLeft, SkeletonPositionIndex::ElbowLeft],
    [SkeletonPositionIndex::ElbowLeft, SkeletonPositionIndex::WristLeft],
    [SkeletonPositionIndex::WristLeft, SkeletonPositionIndex::HandLeft],
    [SkeletonPositionIndex::ShoulderRight, SkeletonPositionIndex::ElbowRight],
    [SkeletonPositionIndex::ElbowRight, SkeletonPositionIndex::WristRight],
    [SkeletonPositionIndex::WristRight, SkeletonPositionIndex::HandRight],
    [SkeletonPositionIndex::HipLeft, SkeletonPositionIndex::KneeLeft],
    [SkeletonPositionIndex::KneeLeft, SkeletonPositionIndex::AnkleLeft],
    [SkeletonPositionIndex::AnkleLeft, SkeletonPositionIndex::FootLeft],
    [SkeletonPositionIndex::HipRight, SkeletonPositionIndex::KneeRight],
    [SkeletonPositionIndex::KneeRight, SkeletonPositionIndex::AnkleRight],
    [SkeletonPositionIndex::AnkleRight, SkeletonPositionIndex::FootRight],
];

impl SkeletonData {
    pub fn get_skeleton_position_data(&self, index: SkeletonPositionIndex) -> SkeletonPositionData {
        let i: usize = index.to_usize().unwrap();
        SkeletonPositionData {
            index,
            position: self.skeleton_positions[i],
            tracking_state: self.skeleton_position_tracking_state[i],
            pixel_index: self.skeleton_pixel_indexes[i],
        }
    }
    pub fn get_skeleton_bones(&self) -> [[SkeletonPositionData; 2]; 19] {
        SKELETON_BONES_INDEXES.map(|[start, end]| {
            [
                self.get_skeleton_position_data(start),
                self.get_skeleton_position_data(end),
            ]
        })
    }
}

fn map_skeleton_values<T, U: Default, F: FnMut(&T) -> U>(
    array: [T; SKELETON_POSITION_COUNT],
    mut func: F,
) -> [U; SKELETON_POSITION_COUNT] {
    let mut out: [U; SKELETON_POSITION_COUNT] = Default::default();
    for i in 0..SKELETON_POSITION_COUNT {
        out[i] = func(&array[i]);
    }
    out
}

impl From<&kinect1_sys::NUI_SKELETON_DATA> for SkeletonData {
    fn from(value: &kinect1_sys::NUI_SKELETON_DATA) -> Self {
        Self {
            tracking_state: SkeletonTrackingState::from_i32(value.eTrackingState).unwrap(),
            tracking_id: value.dwTrackingID,
            enrollment_index: value.dwEnrollmentIndex,
            user_index: value.dwUserIndex,
            position: vector4_to_sk_vector4(&value.Position),
            skeleton_positions: map_skeleton_values(value.SkeletonPositions, vector4_to_sk_vector4),
            skeleton_position_tracking_state: map_skeleton_values(value.eSkeletonPositionTrackingState, |&s| {
                SkeletonPositionTrackingState::from_i32(s).unwrap()
            }),
            quality_flags: EnumSet::<SkeletonQualityFlags>::from_u32(value.dwQualityFlags),
            skeleton_pixel_indexes: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SkeletonFrame {
    pub timestamp: i64,
    pub frame_number: u32,
    pub frame_flags: u32,
    pub floor_clip_plane: SkVector4,
    pub normal_to_gravity: SkVector4,
    pub skeleton_data: Vec<SkeletonData>,
}

impl From<&kinect1_sys::NUI_SKELETON_FRAME> for SkeletonFrame {
    fn from(value: &kinect1_sys::NUI_SKELETON_FRAME) -> Self {
        Self {
            timestamp: value.liTimeStamp,
            frame_number: value.dwFrameNumber,
            frame_flags: value.dwFlags,
            floor_clip_plane: vector4_to_sk_vector4(&value.vFloorClipPlane),
            normal_to_gravity: vector4_to_sk_vector4(&value.vNormalToGravity),
            skeleton_data: value
                .SkeletonData
                .iter()
                .map(|sk| SkeletonData::from(sk))
                .filter(|sk| sk.tracking_state != SkeletonTrackingState::NotTracked)
                .collect_vec(),
        }
    }
}

impl SkeletonFrame {
    pub fn first_skeleton(&self) -> Option<SkeletonData> {
        self.skeleton_data.get(0).map(|sk| *sk)
    }
    pub fn left_hand(&self) -> Option<SkeletonPositionData> {
        self.skeleton_data
            .get(0)
            .map(|sk| sk.get_skeleton_position_data(SkeletonPositionIndex::HandLeft))
    }
    pub fn right_hand(&self) -> Option<SkeletonPositionData> {
        self.skeleton_data
            .get(0)
            .map(|sk| sk.get_skeleton_position_data(SkeletonPositionIndex::HandRight))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of_skeleton_frame() {
        assert_eq!(std::mem::size_of::<SkeletonFrame>(), 72);
    }
}
