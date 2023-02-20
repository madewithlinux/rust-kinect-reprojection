use bytemuck::{Pod, Zeroable};
use kinect1_sys::{
    NuiDepthPixelToDepth, NuiDepthPixelToPlayerIndex, NUI_DEPTH_IMAGE_PIXEL, NUI_DEPTH_IMAGE_POINT,
    NUI_IMAGE_PLAYER_INDEX_SHIFT,
};

use crate::{NUI_IMAGE_DEPTH_MAXIMUM, NUI_IMAGE_DEPTH_MINIMUM, NUI_IMAGE_DEPTH_TOO_FAR_VALUE};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PackedDepth(u16);

unsafe impl Zeroable for PackedDepth {}
unsafe impl Pod for PackedDepth {}

pub const MAX_DEPTH_MM: u16 = NuiDepthPixelToDepth(NUI_IMAGE_DEPTH_MAXIMUM);
pub const MIN_DEPTH_MM: u16 = NuiDepthPixelToDepth(NUI_IMAGE_DEPTH_MINIMUM);

impl PackedDepth {
    #[inline]
    pub fn from_depth_mm_player_index(depth_mm: u16, player_index: u8) -> Self {
        Self(depth_mm << NUI_IMAGE_PLAYER_INDEX_SHIFT | (player_index as u16))
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn is_too_far(&self) -> bool {
        self.0 == NUI_IMAGE_DEPTH_TOO_FAR_VALUE
    }

    #[inline]
    pub fn depth_mm(&self) -> u16 {
        NuiDepthPixelToDepth(self.0)
    }

    #[inline]
    pub fn player_index(&self) -> u8 {
        NuiDepthPixelToPlayerIndex(self.0) as u8
    }

    #[inline]
    pub fn depth(&self) -> f32 {
        self.depth_mm() as f32 / 1_000.0
    }

    #[inline]
    pub fn to_depth_image_point(&self, x: i32, y: i32) -> NUI_DEPTH_IMAGE_POINT {
        NUI_DEPTH_IMAGE_POINT {
            x,
            y,
            depth: self.depth_mm() as i32,
            reserved: 0,
        }
    }
}

impl From<NUI_DEPTH_IMAGE_PIXEL> for PackedDepth {
    fn from(value: NUI_DEPTH_IMAGE_PIXEL) -> Self {
        Self::from_depth_mm_player_index(value.depth, value.playerIndex as u8)
    }
}

impl Into<NUI_DEPTH_IMAGE_PIXEL> for PackedDepth {
    fn into(self) -> NUI_DEPTH_IMAGE_PIXEL {
        NUI_DEPTH_IMAGE_PIXEL {
            depth: self.depth_mm(),
            playerIndex: self.player_index() as u16,
        }
    }
}

// pub fn unpack_depth_frame(packed_depth_frame: &[PackedDepth]) -> Vec<NUI_DEPTH_IMAGE_PIXEL>
