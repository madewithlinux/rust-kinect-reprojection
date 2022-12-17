#![feature(prelude_import)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub const NUI_INITIALIZE_FLAG_USES_AUDIO: u32 = 268435456;
pub const NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX: u32 = 1;
pub const NUI_INITIALIZE_FLAG_USES_COLOR: u32 = 2;
pub const NUI_INITIALIZE_FLAG_USES_SKELETON: u32 = 8;
pub const NUI_INITIALIZE_FLAG_USES_DEPTH: u32 = 32;
pub const NUI_INITIALIZE_FLAG_USES_HIGH_QUALITY_COLOR: u32 = 64;
pub const NUI_INITIALIZE_DEFAULT_HARDWARE_THREAD: u32 = 4294967295;
pub const NUI_SKELETON_COUNT: u32 = 6;
pub const MICARRAY_ADAPTIVE_BEAM: u32 = 4352;
pub type wchar_t = ::std::os::raw::c_ushort;
#[repr(C)]
pub struct INuiAudioBeam {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiAudioBeam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiAudioBeam",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiAudioBeam {}
#[automatically_derived]
impl ::core::clone::Clone for INuiAudioBeam {
    #[inline]
    fn clone(&self) -> INuiAudioBeam {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiFrameTexture {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiFrameTexture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiFrameTexture",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiFrameTexture {}
#[automatically_derived]
impl ::core::clone::Clone for INuiFrameTexture {
    #[inline]
    fn clone(&self) -> INuiFrameTexture {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiCoordinateMapperParametersChangedEvent {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapperParametersChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiCoordinateMapperParametersChangedEvent",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapperParametersChangedEvent {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapperParametersChangedEvent {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapperParametersChangedEvent {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiCoordinateMapper {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiCoordinateMapper",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapper {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapper {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapper {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiColorCameraSettings {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiColorCameraSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiColorCameraSettings",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiColorCameraSettings {}
#[automatically_derived]
impl ::core::clone::Clone for INuiColorCameraSettings {
    #[inline]
    fn clone(&self) -> INuiColorCameraSettings {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiDepthFilter {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiDepthFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiDepthFilter",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiDepthFilter {}
#[automatically_derived]
impl ::core::clone::Clone for INuiDepthFilter {
    #[inline]
    fn clone(&self) -> INuiDepthFilter {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct INuiSensor {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiSensor",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiSensor {}
#[automatically_derived]
impl ::core::clone::Clone for INuiSensor {
    #[inline]
    fn clone(&self) -> INuiSensor {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
/// <div rustbindgen mustusetype></div>
pub type HRESULT = i32;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type BSTR = *mut wchar_t;
pub type LONG = ::std::os::raw::c_long;
pub type INT = ::std::os::raw::c_int;
pub type ULONG = ::std::os::raw::c_ulong;
pub type USHORT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type FLOAT = f32;
pub type UINT = ::std::os::raw::c_uint;
pub type LONGLONG = ::std::os::raw::c_longlong;
#[repr(C)]
pub struct _LARGE_INTEGER {
    pub QuadPart: LONGLONG,
}
#[automatically_derived]
impl ::core::fmt::Debug for _LARGE_INTEGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "_LARGE_INTEGER",
            "QuadPart",
            &&self.QuadPart,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _LARGE_INTEGER {}
#[automatically_derived]
impl ::core::clone::Clone for _LARGE_INTEGER {
    #[inline]
    fn clone(&self) -> _LARGE_INTEGER {
        let _: ::core::clone::AssertParamIsClone<LONGLONG>;
        *self
    }
}
pub type LARGE_INTEGER = _LARGE_INTEGER;
#[repr(C)]
pub struct _Vector4 {
    pub x: FLOAT,
    pub y: FLOAT,
    pub z: FLOAT,
    pub w: FLOAT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _Vector4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "_Vector4",
            "x",
            &&self.x,
            "y",
            &&self.y,
            "z",
            &&self.z,
            "w",
            &&self.w,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _Vector4 {}
#[automatically_derived]
impl ::core::clone::Clone for _Vector4 {
    #[inline]
    fn clone(&self) -> _Vector4 {
        let _: ::core::clone::AssertParamIsClone<FLOAT>;
        *self
    }
}
pub type Vector4 = _Vector4;
#[repr(C)]
pub struct _Matrix4 {
    pub M11: FLOAT,
    pub M12: FLOAT,
    pub M13: FLOAT,
    pub M14: FLOAT,
    pub M21: FLOAT,
    pub M22: FLOAT,
    pub M23: FLOAT,
    pub M24: FLOAT,
    pub M31: FLOAT,
    pub M32: FLOAT,
    pub M33: FLOAT,
    pub M34: FLOAT,
    pub M41: FLOAT,
    pub M42: FLOAT,
    pub M43: FLOAT,
    pub M44: FLOAT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _Matrix4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "M11",
            "M12",
            "M13",
            "M14",
            "M21",
            "M22",
            "M23",
            "M24",
            "M31",
            "M32",
            "M33",
            "M34",
            "M41",
            "M42",
            "M43",
            "M44",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.M11,
            &&self.M12,
            &&self.M13,
            &&self.M14,
            &&self.M21,
            &&self.M22,
            &&self.M23,
            &&self.M24,
            &&self.M31,
            &&self.M32,
            &&self.M33,
            &&self.M34,
            &&self.M41,
            &&self.M42,
            &&self.M43,
            &&self.M44,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "_Matrix4", names, values)
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _Matrix4 {}
#[automatically_derived]
impl ::core::clone::Clone for _Matrix4 {
    #[inline]
    fn clone(&self) -> _Matrix4 {
        let _: ::core::clone::AssertParamIsClone<FLOAT>;
        *self
    }
}
pub type Matrix4 = _Matrix4;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_CENTER: _NUI_SKELETON_POSITION_INDEX = 0;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SPINE: _NUI_SKELETON_POSITION_INDEX = 1;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_CENTER: _NUI_SKELETON_POSITION_INDEX = 2;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HEAD: _NUI_SKELETON_POSITION_INDEX = 3;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_LEFT: _NUI_SKELETON_POSITION_INDEX = 4;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ELBOW_LEFT: _NUI_SKELETON_POSITION_INDEX = 5;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_WRIST_LEFT: _NUI_SKELETON_POSITION_INDEX = 6;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HAND_LEFT: _NUI_SKELETON_POSITION_INDEX = 7;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_SHOULDER_RIGHT: _NUI_SKELETON_POSITION_INDEX = 8;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ELBOW_RIGHT: _NUI_SKELETON_POSITION_INDEX = 9;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_WRIST_RIGHT: _NUI_SKELETON_POSITION_INDEX = 10;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HAND_RIGHT: _NUI_SKELETON_POSITION_INDEX = 11;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_LEFT: _NUI_SKELETON_POSITION_INDEX = 12;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_KNEE_LEFT: _NUI_SKELETON_POSITION_INDEX = 13;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ANKLE_LEFT: _NUI_SKELETON_POSITION_INDEX = 14;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_FOOT_LEFT: _NUI_SKELETON_POSITION_INDEX = 15;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_HIP_RIGHT: _NUI_SKELETON_POSITION_INDEX = 16;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_KNEE_RIGHT: _NUI_SKELETON_POSITION_INDEX = 17;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_ANKLE_RIGHT: _NUI_SKELETON_POSITION_INDEX = 18;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_FOOT_RIGHT: _NUI_SKELETON_POSITION_INDEX = 19;
pub const _NUI_SKELETON_POSITION_INDEX_NUI_SKELETON_POSITION_COUNT: _NUI_SKELETON_POSITION_INDEX = 20;
pub type _NUI_SKELETON_POSITION_INDEX = ::std::os::raw::c_int;
pub use self::_NUI_SKELETON_POSITION_INDEX as NUI_SKELETON_POSITION_INDEX;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX: _NUI_IMAGE_TYPE = 0;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR: _NUI_IMAGE_TYPE = 1;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_YUV: _NUI_IMAGE_TYPE = 2;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_YUV: _NUI_IMAGE_TYPE = 3;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_DEPTH: _NUI_IMAGE_TYPE = 4;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_INFRARED: _NUI_IMAGE_TYPE = 5;
pub const _NUI_IMAGE_TYPE_NUI_IMAGE_TYPE_COLOR_RAW_BAYER: _NUI_IMAGE_TYPE = 6;
pub type _NUI_IMAGE_TYPE = ::std::os::raw::c_int;
pub use self::_NUI_IMAGE_TYPE as NUI_IMAGE_TYPE;
pub const _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_INVALID: _NUI_IMAGE_RESOLUTION = -1;
pub const _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_80x60: _NUI_IMAGE_RESOLUTION = 0;
pub const _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_320x240: _NUI_IMAGE_RESOLUTION = 1;
pub const _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_640x480: _NUI_IMAGE_RESOLUTION = 2;
pub const _NUI_IMAGE_RESOLUTION_NUI_IMAGE_RESOLUTION_1280x960: _NUI_IMAGE_RESOLUTION = 3;
pub type _NUI_IMAGE_RESOLUTION = ::std::os::raw::c_int;
pub use self::_NUI_IMAGE_RESOLUTION as NUI_IMAGE_RESOLUTION;
pub const _NUI_BACKLIGHT_COMPENSATION_MODE_NUI_BACKLIGHT_COMPENSATION_MODE_AVERAGE_BRIGHTNESS: _NUI_BACKLIGHT_COMPENSATION_MODE = 0;
pub const _NUI_BACKLIGHT_COMPENSATION_MODE_NUI_BACKLIGHT_COMPENSATION_MODE_CENTER_PRIORITY: _NUI_BACKLIGHT_COMPENSATION_MODE = 1;
pub const _NUI_BACKLIGHT_COMPENSATION_MODE_NUI_BACKLIGHT_COMPENSATION_MODE_LOWLIGHTS_PRIORITY: _NUI_BACKLIGHT_COMPENSATION_MODE = 2;
pub const _NUI_BACKLIGHT_COMPENSATION_MODE_NUI_BACKLIGHT_COMPENSATION_MODE_CENTER_ONLY: _NUI_BACKLIGHT_COMPENSATION_MODE = 4;
pub type _NUI_BACKLIGHT_COMPENSATION_MODE = ::std::os::raw::c_int;
pub use self::_NUI_BACKLIGHT_COMPENSATION_MODE as NUI_BACKLIGHT_COMPENSATION_MODE;
pub const _NUI_POWER_LINE_FREQUENCY_NUI_POWER_LINE_FREQUENCY_DISABLED: _NUI_POWER_LINE_FREQUENCY = 0;
pub const _NUI_POWER_LINE_FREQUENCY_NUI_POWER_LINE_FREQUENCY_50HZ: _NUI_POWER_LINE_FREQUENCY = 1;
pub const _NUI_POWER_LINE_FREQUENCY_NUI_POWER_LINE_FREQUENCY_60HZ: _NUI_POWER_LINE_FREQUENCY = 2;
pub type _NUI_POWER_LINE_FREQUENCY = ::std::os::raw::c_int;
pub use self::_NUI_POWER_LINE_FREQUENCY as NUI_POWER_LINE_FREQUENCY;
#[repr(C)]
pub struct _NUI_IMAGE_VIEW_AREA {
    pub eDigitalZoom: ::std::os::raw::c_int,
    pub lCenterX: LONG,
    pub lCenterY: LONG,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_IMAGE_VIEW_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "_NUI_IMAGE_VIEW_AREA",
            "eDigitalZoom",
            &&self.eDigitalZoom,
            "lCenterX",
            &&self.lCenterX,
            "lCenterY",
            &&self.lCenterY,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_IMAGE_VIEW_AREA {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_IMAGE_VIEW_AREA {
    #[inline]
    fn clone(&self) -> _NUI_IMAGE_VIEW_AREA {
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
        let _: ::core::clone::AssertParamIsClone<LONG>;
        *self
    }
}
pub type NUI_IMAGE_VIEW_AREA = _NUI_IMAGE_VIEW_AREA;
#[repr(C)]
pub struct _NUI_TRANSFORM_SMOOTH_PARAMETERS {
    pub fSmoothing: FLOAT,
    pub fCorrection: FLOAT,
    pub fPrediction: FLOAT,
    pub fJitterRadius: FLOAT,
    pub fMaxDeviationRadius: FLOAT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "_NUI_TRANSFORM_SMOOTH_PARAMETERS",
            "fSmoothing",
            &&self.fSmoothing,
            "fCorrection",
            &&self.fCorrection,
            "fPrediction",
            &&self.fPrediction,
            "fJitterRadius",
            &&self.fJitterRadius,
            "fMaxDeviationRadius",
            &&self.fMaxDeviationRadius,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_TRANSFORM_SMOOTH_PARAMETERS {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
    #[inline]
    fn clone(&self) -> _NUI_TRANSFORM_SMOOTH_PARAMETERS {
        let _: ::core::clone::AssertParamIsClone<FLOAT>;
        *self
    }
}
pub type NUI_TRANSFORM_SMOOTH_PARAMETERS = _NUI_TRANSFORM_SMOOTH_PARAMETERS;
#[repr(C)]
pub struct _NUI_SURFACE_DESC {
    pub Width: UINT,
    pub Height: UINT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_SURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "_NUI_SURFACE_DESC",
            "Width",
            &&self.Width,
            "Height",
            &&self.Height,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_SURFACE_DESC {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_SURFACE_DESC {
    #[inline]
    fn clone(&self) -> _NUI_SURFACE_DESC {
        let _: ::core::clone::AssertParamIsClone<UINT>;
        *self
    }
}
pub type NUI_SURFACE_DESC = _NUI_SURFACE_DESC;
pub const _NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_NOT_TRACKED: _NUI_SKELETON_POSITION_TRACKING_STATE = 0;
pub const _NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_INFERRED: _NUI_SKELETON_POSITION_TRACKING_STATE = 1;
pub const _NUI_SKELETON_POSITION_TRACKING_STATE_NUI_SKELETON_POSITION_TRACKED: _NUI_SKELETON_POSITION_TRACKING_STATE = 2;
pub type _NUI_SKELETON_POSITION_TRACKING_STATE = ::std::os::raw::c_int;
pub use self::_NUI_SKELETON_POSITION_TRACKING_STATE as NUI_SKELETON_POSITION_TRACKING_STATE;
pub const _NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_NOT_TRACKED: _NUI_SKELETON_TRACKING_STATE = 0;
pub const _NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_POSITION_ONLY: _NUI_SKELETON_TRACKING_STATE = 1;
pub const _NUI_SKELETON_TRACKING_STATE_NUI_SKELETON_TRACKED: _NUI_SKELETON_TRACKING_STATE = 2;
pub type _NUI_SKELETON_TRACKING_STATE = ::std::os::raw::c_int;
pub use self::_NUI_SKELETON_TRACKING_STATE as NUI_SKELETON_TRACKING_STATE;
#[repr(C)]
pub struct _NUI_SKELETON_DATA {
    pub eTrackingState: NUI_SKELETON_TRACKING_STATE,
    pub dwTrackingID: DWORD,
    pub dwEnrollmentIndex: DWORD,
    pub dwUserIndex: DWORD,
    pub Position: Vector4,
    pub SkeletonPositions: [Vector4; 20usize],
    pub eSkeletonPositionTrackingState: [NUI_SKELETON_POSITION_TRACKING_STATE; 20usize],
    pub dwQualityFlags: DWORD,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_SKELETON_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "eTrackingState",
            "dwTrackingID",
            "dwEnrollmentIndex",
            "dwUserIndex",
            "Position",
            "SkeletonPositions",
            "eSkeletonPositionTrackingState",
            "dwQualityFlags",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.eTrackingState,
            &&self.dwTrackingID,
            &&self.dwEnrollmentIndex,
            &&self.dwUserIndex,
            &&self.Position,
            &&self.SkeletonPositions,
            &&self.eSkeletonPositionTrackingState,
            &&self.dwQualityFlags,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "_NUI_SKELETON_DATA",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_SKELETON_DATA {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_SKELETON_DATA {
    #[inline]
    fn clone(&self) -> _NUI_SKELETON_DATA {
        let _: ::core::clone::AssertParamIsClone<NUI_SKELETON_TRACKING_STATE>;
        let _: ::core::clone::AssertParamIsClone<DWORD>;
        let _: ::core::clone::AssertParamIsClone<Vector4>;
        let _: ::core::clone::AssertParamIsClone<[Vector4; 20usize]>;
        let _: ::core::clone::AssertParamIsClone<
            [NUI_SKELETON_POSITION_TRACKING_STATE; 20usize],
        >;
        *self
    }
}
pub type NUI_SKELETON_DATA = _NUI_SKELETON_DATA;
#[repr(C)]
pub struct _NUI_SKELETON_FRAME {
    pub liTimeStamp: LARGE_INTEGER,
    pub dwFrameNumber: DWORD,
    pub dwFlags: DWORD,
    pub vFloorClipPlane: Vector4,
    pub vNormalToGravity: Vector4,
    pub SkeletonData: [NUI_SKELETON_DATA; 6usize],
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_SKELETON_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "liTimeStamp",
            "dwFrameNumber",
            "dwFlags",
            "vFloorClipPlane",
            "vNormalToGravity",
            "SkeletonData",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.liTimeStamp,
            &&self.dwFrameNumber,
            &&self.dwFlags,
            &&self.vFloorClipPlane,
            &&self.vNormalToGravity,
            &&self.SkeletonData,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "_NUI_SKELETON_FRAME",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_SKELETON_FRAME {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_SKELETON_FRAME {
    #[inline]
    fn clone(&self) -> _NUI_SKELETON_FRAME {
        let _: ::core::clone::AssertParamIsClone<LARGE_INTEGER>;
        let _: ::core::clone::AssertParamIsClone<DWORD>;
        let _: ::core::clone::AssertParamIsClone<Vector4>;
        let _: ::core::clone::AssertParamIsClone<[NUI_SKELETON_DATA; 6usize]>;
        *self
    }
}
pub type NUI_SKELETON_FRAME = _NUI_SKELETON_FRAME;
#[repr(C)]
pub struct _NUI_SKELETON_BONE_ROTATION {
    pub rotationMatrix: Matrix4,
    pub rotationQuaternion: Vector4,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_SKELETON_BONE_ROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "_NUI_SKELETON_BONE_ROTATION",
            "rotationMatrix",
            &&self.rotationMatrix,
            "rotationQuaternion",
            &&self.rotationQuaternion,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_SKELETON_BONE_ROTATION {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_SKELETON_BONE_ROTATION {
    #[inline]
    fn clone(&self) -> _NUI_SKELETON_BONE_ROTATION {
        let _: ::core::clone::AssertParamIsClone<Matrix4>;
        let _: ::core::clone::AssertParamIsClone<Vector4>;
        *self
    }
}
pub type NUI_SKELETON_BONE_ROTATION = _NUI_SKELETON_BONE_ROTATION;
#[repr(C)]
pub struct _NUI_SKELETON_BONE_ORIENTATION {
    pub endJoint: NUI_SKELETON_POSITION_INDEX,
    pub startJoint: NUI_SKELETON_POSITION_INDEX,
    pub hierarchicalRotation: NUI_SKELETON_BONE_ROTATION,
    pub absoluteRotation: NUI_SKELETON_BONE_ROTATION,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_SKELETON_BONE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "_NUI_SKELETON_BONE_ORIENTATION",
            "endJoint",
            &&self.endJoint,
            "startJoint",
            &&self.startJoint,
            "hierarchicalRotation",
            &&self.hierarchicalRotation,
            "absoluteRotation",
            &&self.absoluteRotation,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_SKELETON_BONE_ORIENTATION {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_SKELETON_BONE_ORIENTATION {
    #[inline]
    fn clone(&self) -> _NUI_SKELETON_BONE_ORIENTATION {
        let _: ::core::clone::AssertParamIsClone<NUI_SKELETON_POSITION_INDEX>;
        let _: ::core::clone::AssertParamIsClone<NUI_SKELETON_BONE_ROTATION>;
        *self
    }
}
pub type NUI_SKELETON_BONE_ORIENTATION = _NUI_SKELETON_BONE_ORIENTATION;
#[repr(C)]
pub struct _NUI_LOCKED_RECT {
    pub Pitch: INT,
    pub size: INT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_LOCKED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "_NUI_LOCKED_RECT",
            "Pitch",
            &&self.Pitch,
            "size",
            &&self.size,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_LOCKED_RECT {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_LOCKED_RECT {
    #[inline]
    fn clone(&self) -> _NUI_LOCKED_RECT {
        let _: ::core::clone::AssertParamIsClone<INT>;
        *self
    }
}
pub type NUI_LOCKED_RECT = _NUI_LOCKED_RECT;
#[repr(C)]
pub struct _NUI_DEPTH_IMAGE_POINT {
    pub x: LONG,
    pub y: LONG,
    pub depth: LONG,
    pub reserved: LONG,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_DEPTH_IMAGE_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "_NUI_DEPTH_IMAGE_POINT",
            "x",
            &&self.x,
            "y",
            &&self.y,
            "depth",
            &&self.depth,
            "reserved",
            &&self.reserved,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_DEPTH_IMAGE_POINT {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_DEPTH_IMAGE_POINT {
    #[inline]
    fn clone(&self) -> _NUI_DEPTH_IMAGE_POINT {
        let _: ::core::clone::AssertParamIsClone<LONG>;
        *self
    }
}
pub type NUI_DEPTH_IMAGE_POINT = _NUI_DEPTH_IMAGE_POINT;
#[repr(C)]
pub struct _NUI_COLOR_IMAGE_POINT {
    pub x: LONG,
    pub y: LONG,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_COLOR_IMAGE_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "_NUI_COLOR_IMAGE_POINT",
            "x",
            &&self.x,
            "y",
            &&self.y,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_COLOR_IMAGE_POINT {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_COLOR_IMAGE_POINT {
    #[inline]
    fn clone(&self) -> _NUI_COLOR_IMAGE_POINT {
        let _: ::core::clone::AssertParamIsClone<LONG>;
        *self
    }
}
pub type NUI_COLOR_IMAGE_POINT = _NUI_COLOR_IMAGE_POINT;
#[repr(C)]
pub struct _NUI_IMAGE_FRAME {
    pub liTimeStamp: LARGE_INTEGER,
    pub dwFrameNumber: DWORD,
    pub eImageType: NUI_IMAGE_TYPE,
    pub eResolution: NUI_IMAGE_RESOLUTION,
    pub pFrameTexture: *mut INuiFrameTexture,
    pub dwFrameFlags: DWORD,
    pub ViewArea: NUI_IMAGE_VIEW_AREA,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_IMAGE_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "liTimeStamp",
            "dwFrameNumber",
            "eImageType",
            "eResolution",
            "pFrameTexture",
            "dwFrameFlags",
            "ViewArea",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.liTimeStamp,
            &&self.dwFrameNumber,
            &&self.eImageType,
            &&self.eResolution,
            &&self.pFrameTexture,
            &&self.dwFrameFlags,
            &&self.ViewArea,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "_NUI_IMAGE_FRAME",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_IMAGE_FRAME {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_IMAGE_FRAME {
    #[inline]
    fn clone(&self) -> _NUI_IMAGE_FRAME {
        let _: ::core::clone::AssertParamIsClone<LARGE_INTEGER>;
        let _: ::core::clone::AssertParamIsClone<DWORD>;
        let _: ::core::clone::AssertParamIsClone<NUI_IMAGE_TYPE>;
        let _: ::core::clone::AssertParamIsClone<NUI_IMAGE_RESOLUTION>;
        let _: ::core::clone::AssertParamIsClone<*mut INuiFrameTexture>;
        let _: ::core::clone::AssertParamIsClone<NUI_IMAGE_VIEW_AREA>;
        *self
    }
}
pub type NUI_IMAGE_FRAME = _NUI_IMAGE_FRAME;
#[repr(C)]
pub struct _NUI_DEPTH_IMAGE_PIXEL {
    pub playerIndex: USHORT,
    pub depth: USHORT,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_DEPTH_IMAGE_PIXEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "_NUI_DEPTH_IMAGE_PIXEL",
            "playerIndex",
            &&self.playerIndex,
            "depth",
            &&self.depth,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _NUI_DEPTH_IMAGE_PIXEL {}
#[automatically_derived]
impl ::core::clone::Clone for _NUI_DEPTH_IMAGE_PIXEL {
    #[inline]
    fn clone(&self) -> _NUI_DEPTH_IMAGE_PIXEL {
        let _: ::core::clone::AssertParamIsClone<USHORT>;
        *self
    }
}
pub type NUI_DEPTH_IMAGE_PIXEL = _NUI_DEPTH_IMAGE_PIXEL;
extern "C" {
    pub fn hello() -> i32;
}
#[repr(C)]
pub struct c_INuiSensor {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for c_INuiSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "c_INuiSensor",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for c_INuiSensor {}
#[automatically_derived]
impl ::core::clone::Clone for c_INuiSensor {
    #[inline]
    fn clone(&self) -> c_INuiSensor {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
extern "C" {
    #[must_use]
    pub fn c_NuiGetSensorCount(pCount: *mut i32) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_NuiCreateSensorByIndex(
        index: i32,
        ppNuiSensor: *mut *mut INuiSensor,
    ) -> HRESULT;
}
extern "C" {
    pub fn c_INuiSensor_AddRef(This: *mut INuiSensor) -> ULONG;
}
extern "C" {
    pub fn c_INuiSensor_Release(This: *mut INuiSensor) -> ULONG;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiInitialize(This: *mut INuiSensor, dwFlags: DWORD) -> HRESULT;
}
extern "C" {
    pub fn c_INuiSensor_NuiShutdown(This: *mut INuiSensor);
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSetFrameEndEvent(
        This: *mut INuiSensor,
        hEvent: HANDLE,
        dwFrameEventFlag: DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageStreamOpen(
        This: *mut INuiSensor,
        eImageType: NUI_IMAGE_TYPE,
        eResolution: NUI_IMAGE_RESOLUTION,
        dwImageFrameFlags: DWORD,
        dwFrameLimit: DWORD,
        hNextFrameEvent: HANDLE,
        phStreamHandle: *mut HANDLE,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageStreamSetImageFrameFlags(
        This: *mut INuiSensor,
        hStream: HANDLE,
        dwImageFrameFlags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageStreamGetImageFrameFlags(
        This: *mut INuiSensor,
        hStream: HANDLE,
        pdwImageFrameFlags: *mut DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageStreamGetNextFrame(
        This: *mut INuiSensor,
        hStream: HANDLE,
        dwMillisecondsToWait: DWORD,
        pImageFrame: *mut NUI_IMAGE_FRAME,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageStreamReleaseFrame(
        This: *mut INuiSensor,
        hStream: HANDLE,
        pImageFrame: *mut NUI_IMAGE_FRAME,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixel(
        This: *mut INuiSensor,
        eColorResolution: NUI_IMAGE_RESOLUTION,
        pcViewArea: *const NUI_IMAGE_VIEW_AREA,
        lDepthX: LONG,
        lDepthY: LONG,
        usDepthValue: USHORT,
        plColorX: *mut LONG,
        plColorY: *mut LONG,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(
        This: *mut INuiSensor,
        eColorResolution: NUI_IMAGE_RESOLUTION,
        eDepthResolution: NUI_IMAGE_RESOLUTION,
        pcViewArea: *const NUI_IMAGE_VIEW_AREA,
        lDepthX: LONG,
        lDepthY: LONG,
        usDepthValue: USHORT,
        plColorX: *mut LONG,
        plColorY: *mut LONG,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(
        This: *mut INuiSensor,
        eColorResolution: NUI_IMAGE_RESOLUTION,
        eDepthResolution: NUI_IMAGE_RESOLUTION,
        cDepthValues: DWORD,
        pDepthValues: *mut USHORT,
        cColorCoordinates: DWORD,
        pColorCoordinates: *mut LONG,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiCameraElevationSetAngle(
        This: *mut INuiSensor,
        lAngleDegrees: LONG,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiCameraElevationGetAngle(
        This: *mut INuiSensor,
        plAngleDegrees: *mut LONG,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSkeletonTrackingEnable(
        This: *mut INuiSensor,
        hNextFrameEvent: HANDLE,
        dwFlags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSkeletonTrackingDisable(This: *mut INuiSensor) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSkeletonSetTrackedSkeletons(
        This: *mut INuiSensor,
        TrackingIDs: *mut DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSkeletonGetNextFrame(
        This: *mut INuiSensor,
        dwMillisecondsToWait: DWORD,
        pSkeletonFrame: *mut NUI_SKELETON_FRAME,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiTransformSmooth(
        This: *mut INuiSensor,
        pSkeletonFrame: *mut NUI_SKELETON_FRAME,
        pSmoothingParams: *const NUI_TRANSFORM_SMOOTH_PARAMETERS,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiGetAudioSource(
        This: *mut INuiSensor,
        ppDmo: *mut *mut INuiAudioBeam,
    ) -> HRESULT;
}
extern "C" {
    pub fn c_INuiSensor_NuiInstanceIndex(This: *mut INuiSensor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn c_INuiSensor_NuiDeviceConnectionId(This: *mut INuiSensor) -> BSTR;
}
extern "C" {
    pub fn c_INuiSensor_NuiUniqueId(This: *mut INuiSensor) -> BSTR;
}
extern "C" {
    pub fn c_INuiSensor_NuiAudioArrayId(This: *mut INuiSensor) -> BSTR;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiStatus(This: *mut INuiSensor) -> HRESULT;
}
extern "C" {
    pub fn c_INuiSensor_NuiInitializationFlags(This: *mut INuiSensor) -> DWORD;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiGetCoordinateMapper(
        This: *mut INuiSensor,
        pMapping: *mut *mut INuiCoordinateMapper,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiImageFrameGetDepthImagePixelFrameTexture(
        This: *mut INuiSensor,
        hStream: HANDLE,
        pImageFrame: *mut NUI_IMAGE_FRAME,
        pNearMode: *mut BOOL,
        ppFrameTexture: *mut *mut INuiFrameTexture,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiGetColorCameraSettings(
        This: *mut INuiSensor,
        pCameraSettings: *mut *mut INuiColorCameraSettings,
    ) -> HRESULT;
}
extern "C" {
    pub fn c_INuiSensor_NuiGetForceInfraredEmitterOff(This: *mut INuiSensor) -> BOOL;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSetForceInfraredEmitterOff(
        This: *mut INuiSensor,
        fForceInfraredEmitterOff: BOOL,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiAccelerometerGetCurrentReading(
        This: *mut INuiSensor,
        pReading: *mut Vector4,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiSetDepthFilter(
        This: *mut INuiSensor,
        pDepthFilter: *mut INuiDepthFilter,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiGetDepthFilter(
        This: *mut INuiSensor,
        ppDepthFilter: *mut *mut INuiDepthFilter,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn c_INuiSensor_NuiGetDepthFilterForTimeStamp(
        This: *mut INuiSensor,
        liTimeStamp: LARGE_INTEGER,
        ppDepthFilter: *mut *mut INuiDepthFilter,
    ) -> HRESULT;
}
