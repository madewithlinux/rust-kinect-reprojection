#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

// impl Drop for NUI_IMAGE_FRAME {
//     fn drop(&mut self) {
//         todo!()
//     }
// }
