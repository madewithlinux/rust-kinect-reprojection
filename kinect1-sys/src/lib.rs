#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bytemuck::{AnyBitPattern, Zeroable};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn NuiDepthPixelToDepth(packedPixel: u16) -> u16 {
    packedPixel >> NUI_IMAGE_PLAYER_INDEX_SHIFT
}

pub fn NuiDepthPixelToPlayerIndex(packedPixel: u16) -> u16 {
    packedPixel & (NUI_IMAGE_PLAYER_INDEX_MASK as u16)
}

impl Default for NUI_IMAGE_VIEW_AREA {
    fn default() -> Self {
        Self {
            eDigitalZoom: Default::default(),
            lCenterX: Default::default(),
            lCenterY: Default::default(),
        }
    }
}

impl Default for NUI_IMAGE_FRAME {
    fn default() -> Self {
        Self {
            liTimeStamp: Default::default(),
            dwFrameNumber: Default::default(),
            eImageType: Default::default(),
            eResolution: Default::default(),
            pFrameTexture: std::ptr::null_mut(),
            dwFrameFlags: Default::default(),
            ViewArea: Default::default(),
        }
    }
}

impl Default for NUI_LOCKED_RECT {
    fn default() -> Self {
        Self {
            Pitch: Default::default(),
            size: Default::default(),
            pBits: std::ptr::null_mut(),
        }
    }
}

impl Default for NUI_COLOR_IMAGE_POINT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Default for NUI_DEPTH_IMAGE_POINT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            depth: Default::default(),
            reserved: Default::default(),
        }
    }
}

unsafe impl Zeroable for NUI_DEPTH_IMAGE_PIXEL {}
unsafe impl AnyBitPattern for NUI_DEPTH_IMAGE_PIXEL {}

impl Default for NUI_DEPTH_IMAGE_PIXEL {
    fn default() -> Self {
        Self {
            playerIndex: Default::default(),
            depth: Default::default(),
        }
    }
}


impl Default for Vector4 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}

impl Default for NUI_SKELETON_DATA {
    fn default() -> Self {
        Self {
            eTrackingState: Default::default(),
            dwTrackingID: Default::default(),
            dwEnrollmentIndex: Default::default(),
            dwUserIndex: Default::default(),
            Position: Default::default(),
            SkeletonPositions: Default::default(),
            eSkeletonPositionTrackingState: Default::default(),
            dwQualityFlags: Default::default(),
        }
    }
}

impl Default for NUI_SKELETON_FRAME {
    fn default() -> Self {
        Self {
            liTimeStamp: Default::default(),
            dwFrameNumber: Default::default(),
            dwFlags: Default::default(),
            vFloorClipPlane: Default::default(),
            vNormalToGravity: Default::default(),
            SkeletonData: Default::default(),
        }
    }
}

// impl Drop for NUI_IMAGE_FRAME {
//     fn drop(&mut self) {
//         todo!()
//     }
// }
