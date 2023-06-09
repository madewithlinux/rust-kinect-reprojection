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
pub const FACILITY_NUI: u32 = 769;
pub const NUI_SKELETON_COUNT: u32 = 6;
pub const MICARRAY_ADAPTIVE_BEAM: u32 = 4352;
pub const MAX_DEV_STR_LEN: u32 = 512;
pub const NUI_IMAGE_PLAYER_INDEX_SHIFT: u32 = 3;
pub const NUI_IMAGE_PLAYER_INDEX_MASK: u32 = 7;
pub const NUI_IMAGE_DEPTH_MAXIMUM: u32 = 32007;
pub const NUI_IMAGE_DEPTH_MINIMUM: u32 = 6400;
pub const NUI_IMAGE_DEPTH_MAXIMUM_NEAR_MODE: u32 = 24007;
pub const NUI_IMAGE_DEPTH_MINIMUM_NEAR_MODE: u32 = 3200;
pub const NUI_IMAGE_DEPTH_NO_VALUE: u32 = 0;
pub const NUI_IMAGE_DEPTH_TOO_FAR_VALUE: u32 = 32760;
pub const NUI_IMAGE_DEPTH_UNKNOWN_VALUE: u32 = 65528;
pub const NUI_DEPTH_DEPTH_UNKNOWN_VALUE: u32 = 65528;
pub const NUI_DEPTH_UNKNOWN: u32 = 0;
pub const NUI_PIXEL_COORDINATE_UNKNOWN: i32 = -2147483648;
pub const NUI_CAMERA_DEPTH_NOMINAL_FOCAL_LENGTH_IN_PIXELS: f64 = 285.63;
pub const NUI_CAMERA_DEPTH_NOMINAL_DIAGONAL_FOV: f64 = 70.0;
pub const NUI_CAMERA_DEPTH_NOMINAL_HORIZONTAL_FOV: f64 = 58.5;
pub const NUI_CAMERA_DEPTH_NOMINAL_VERTICAL_FOV: f64 = 45.6;
pub const NUI_CAMERA_COLOR_NOMINAL_FOCAL_LENGTH_IN_PIXELS: f64 = 531.15;
pub const NUI_CAMERA_COLOR_NOMINAL_DIAGONAL_FOV: f64 = 73.9;
pub const NUI_CAMERA_COLOR_NOMINAL_HORIZONTAL_FOV: f64 = 62.0;
pub const NUI_CAMERA_COLOR_NOMINAL_VERTICAL_FOV: f64 = 48.6;
pub const NUI_IMAGE_FRAME_FLAG_NONE: u32 = 0;
pub const NUI_IMAGE_FRAME_FLAG_VIEW_AREA_UNKNOWN: u32 = 1;
pub const NUI_IMAGE_FRAME_FLAG_NEAR_MODE_ENABLED: u32 = 131072;
pub const NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA: u32 = 65536;
pub const NUI_IMAGE_STREAM_FLAG_ENABLE_NEAR_MODE: u32 = 131072;
pub const NUI_IMAGE_STREAM_FLAG_DISTINCT_OVERFLOW_DEPTH_VALUES: u32 = 262144;
pub const NUI_IMAGE_STREAM_FRAME_LIMIT_MAXIMUM: u32 = 4;
pub const NUI_CAMERA_ELEVATION_MAXIMUM: u32 = 27;
pub const NUI_CAMERA_ELEVATION_MINIMUM: i32 = -27;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type ULONG = ::std::os::raw::c_ulong;
pub type USHORT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type FLOAT = f32;
pub type INT = ::std::os::raw::c_int;
pub type UINT = ::std::os::raw::c_uint;
pub type CHAR = ::std::os::raw::c_char;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type LONGLONG = ::std::os::raw::c_longlong;
#[repr(C)]
pub struct _GUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
#[automatically_derived]
impl ::core::fmt::Debug for _GUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "_GUID",
            "Data1",
            &&self.Data1,
            "Data2",
            &&self.Data2,
            "Data3",
            &&self.Data3,
            "Data4",
            &&self.Data4,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for _GUID {}
#[automatically_derived]
impl ::core::clone::Clone for _GUID {
    #[inline]
    fn clone(&self) -> _GUID {
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ulong>;
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
        let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_uchar; 8usize]>;
        *self
    }
}
pub type GUID = _GUID;
pub type IID = GUID;
#[repr(C)]
pub struct tagRECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
#[automatically_derived]
impl ::core::fmt::Debug for tagRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "tagRECT",
            "left",
            &&self.left,
            "top",
            &&self.top,
            "right",
            &&self.right,
            "bottom",
            &&self.bottom,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for tagRECT {}
#[automatically_derived]
impl ::core::clone::Clone for tagRECT {
    #[inline]
    fn clone(&self) -> tagRECT {
        let _: ::core::clone::AssertParamIsClone<LONG>;
        *self
    }
}
pub type RECT = tagRECT;
pub type RPC_IF_HANDLE = *mut ::std::os::raw::c_void;
pub type byte = ::std::os::raw::c_uchar;
pub type OLECHAR = WCHAR;
pub type BSTR = *mut OLECHAR;
/// <div rustbindgen replaces="LARGE_INTEGER"></div>
pub type LARGE_INTEGER = LONGLONG;
/** <div rustbindgen mustusetype></div>
 <div rustbindgen replaces="HRESULT"></div>*/
pub type HRESULT = i32;
extern "C" {
    #[must_use]
    /** <summary>
 Initializes the sensor. If the sensor is already initialized, this will shut down the sensor
 and reinitialize it.
 </summary>
 <param name="dwFlags">
 The NUI subsystems to initialize, as a bitwise-OR combination of the NUI_INITIALIZE constants.
 </param>
 <returns>
 <para>Returns S_OK if successful; otherwise, returns a failure code.</para>
 </returns>*/
    pub fn NuiInitialize(dwFlags: DWORD) -> HRESULT;
}
extern "C" {
    /** <summary>
 Shuts down the sensor. If the sensor is already shut down, nothing happens.
 </summary>*/
    pub fn NuiShutdown();
}
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
    pub pBits: *mut byte,
}
#[automatically_derived]
impl ::core::fmt::Debug for _NUI_LOCKED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "_NUI_LOCKED_RECT",
            "Pitch",
            &&self.Pitch,
            "size",
            &&self.size,
            "pBits",
            &&self.pBits,
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
        let _: ::core::clone::AssertParamIsClone<*mut byte>;
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
extern "C" {
    pub static mut __MIDL_itf_Kinect_0000_0000_v0_0_c_ifspec: RPC_IF_HANDLE;
}
extern "C" {
    pub static mut __MIDL_itf_Kinect_0000_0000_v0_0_s_ifspec: RPC_IF_HANDLE;
}
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
    pub static IID_INuiAudioBeam: IID;
}
#[repr(C)]
pub struct INuiAudioBeamVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiAudioBeam,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiAudioBeam) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiAudioBeam) -> ULONG,
    >,
    pub GetBeam: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiAudioBeam, angle: *mut f64) -> HRESULT,
    >,
    pub SetBeam: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiAudioBeam, angle: f64) -> HRESULT,
    >,
    pub GetPosition: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiAudioBeam,
            angle: *mut f64,
            confidence: *mut f64,
        ) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiAudioBeamVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "QueryInterface",
            "AddRef",
            "Release",
            "GetBeam",
            "SetBeam",
            "GetPosition",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.QueryInterface,
            &&self.AddRef,
            &&self.Release,
            &&self.GetBeam,
            &&self.SetBeam,
            &&self.GetPosition,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "INuiAudioBeamVtbl",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiAudioBeamVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiAudioBeamVtbl {
    #[inline]
    fn clone(&self) -> INuiAudioBeamVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiAudioBeam,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiAudioBeam) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiAudioBeam) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiAudioBeam,
                    angle: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiAudioBeam, angle: f64) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiAudioBeam,
                    angle: *mut f64,
                    confidence: *mut f64,
                ) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiAudioBeam {
    pub lpVtbl: *mut INuiAudioBeamVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiAudioBeam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiAudioBeam",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiAudioBeam {}
#[automatically_derived]
impl ::core::clone::Clone for INuiAudioBeam {
    #[inline]
    fn clone(&self) -> INuiAudioBeam {
        let _: ::core::clone::AssertParamIsClone<*mut INuiAudioBeamVtbl>;
        *self
    }
}
extern "C" {
    pub static IID_INuiFrameTexture: IID;
}
#[repr(C)]
pub struct INuiFrameTextureVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiFrameTexture,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ULONG,
    >,
    pub BufferLen: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ::std::os::raw::c_int,
    >,
    pub Pitch: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ::std::os::raw::c_int,
    >,
    pub LockRect: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiFrameTexture,
            Level: UINT,
            pLockedRect: *mut NUI_LOCKED_RECT,
            pRect: *mut RECT,
            Flags: DWORD,
        ) -> HRESULT,
    >,
    pub GetLevelDesc: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiFrameTexture,
            Level: UINT,
            pDesc: *mut NUI_SURFACE_DESC,
        ) -> HRESULT,
    >,
    pub UnlockRect: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiFrameTexture, Level: UINT) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiFrameTextureVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "QueryInterface",
            "AddRef",
            "Release",
            "BufferLen",
            "Pitch",
            "LockRect",
            "GetLevelDesc",
            "UnlockRect",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.QueryInterface,
            &&self.AddRef,
            &&self.Release,
            &&self.BufferLen,
            &&self.Pitch,
            &&self.LockRect,
            &&self.GetLevelDesc,
            &&self.UnlockRect,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "INuiFrameTextureVtbl",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiFrameTextureVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiFrameTextureVtbl {
    #[inline]
    fn clone(&self) -> INuiFrameTextureVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiFrameTexture,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiFrameTexture) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiFrameTexture,
                ) -> ::std::os::raw::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiFrameTexture,
                ) -> ::std::os::raw::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiFrameTexture,
                    Level: UINT,
                    pLockedRect: *mut NUI_LOCKED_RECT,
                    pRect: *mut RECT,
                    Flags: DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiFrameTexture,
                    Level: UINT,
                    pDesc: *mut NUI_SURFACE_DESC,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiFrameTexture, Level: UINT) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiFrameTexture {
    pub lpVtbl: *mut INuiFrameTextureVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiFrameTexture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiFrameTexture",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiFrameTexture {}
#[automatically_derived]
impl ::core::clone::Clone for INuiFrameTexture {
    #[inline]
    fn clone(&self) -> INuiFrameTexture {
        let _: ::core::clone::AssertParamIsClone<*mut INuiFrameTextureVtbl>;
        *self
    }
}
extern "C" {
    pub static IID_INuiCoordinateMapperParametersChangedEvent: IID;
}
#[repr(C)]
pub struct INuiCoordinateMapperParametersChangedEventVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapperParametersChangedEvent,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapperParametersChangedEvent,
        ) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapperParametersChangedEvent,
        ) -> ULONG,
    >,
    pub Invoke: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapperParametersChangedEvent,
        ) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapperParametersChangedEventVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "INuiCoordinateMapperParametersChangedEventVtbl",
            "QueryInterface",
            &&self.QueryInterface,
            "AddRef",
            &&self.AddRef,
            "Release",
            &&self.Release,
            "Invoke",
            &&self.Invoke,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapperParametersChangedEventVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapperParametersChangedEventVtbl {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapperParametersChangedEventVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapperParametersChangedEvent,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapperParametersChangedEvent,
                ) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapperParametersChangedEvent,
                ) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapperParametersChangedEvent,
                ) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiCoordinateMapperParametersChangedEvent {
    pub lpVtbl: *mut INuiCoordinateMapperParametersChangedEventVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapperParametersChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiCoordinateMapperParametersChangedEvent",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapperParametersChangedEvent {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapperParametersChangedEvent {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapperParametersChangedEvent {
        let _: ::core::clone::AssertParamIsClone<
            *mut INuiCoordinateMapperParametersChangedEventVtbl,
        >;
        *self
    }
}
extern "C" {
    pub static IID_INuiCoordinateMapper: IID;
}
#[repr(C)]
pub struct INuiCoordinateMapperVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiCoordinateMapper) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiCoordinateMapper) -> ULONG,
    >,
    pub GetColorToDepthRelationalParameters: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            pDataByteCount: *mut ULONG,
            ppData: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub NotifyParametersChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            pCallback: *mut INuiCoordinateMapperParametersChangedEvent,
        ) -> HRESULT,
    >,
    pub MapColorFrameToDepthFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eColorType: NUI_IMAGE_TYPE,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            cDepthPixels: DWORD,
            pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
            cDepthPoints: DWORD,
            pDepthPoints: *mut NUI_DEPTH_IMAGE_POINT,
        ) -> HRESULT,
    >,
    pub MapColorFrameToSkeletonFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eColorType: NUI_IMAGE_TYPE,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            cDepthPixels: DWORD,
            pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
            cSkeletonPoints: DWORD,
            pSkeletonPoints: *mut Vector4,
        ) -> HRESULT,
    >,
    pub MapDepthFrameToColorFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            cDepthPixels: DWORD,
            pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
            eColorType: NUI_IMAGE_TYPE,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            cColorPoints: DWORD,
            pColorPoints: *mut NUI_COLOR_IMAGE_POINT,
        ) -> HRESULT,
    >,
    pub MapDepthFrameToSkeletonFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            cDepthPixels: DWORD,
            pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
            cSkeletonPoints: DWORD,
            pSkeletonPoints: *mut Vector4,
        ) -> HRESULT,
    >,
    pub MapDepthPointToColorPoint: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
            eColorType: NUI_IMAGE_TYPE,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            pColorPoint: *mut NUI_COLOR_IMAGE_POINT,
        ) -> HRESULT,
    >,
    pub MapDepthPointToSkeletonPoint: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
            pSkeletonPoint: *mut Vector4,
        ) -> HRESULT,
    >,
    pub MapSkeletonPointToColorPoint: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            pSkeletonPoint: *mut Vector4,
            eColorType: NUI_IMAGE_TYPE,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            pColorPoint: *mut NUI_COLOR_IMAGE_POINT,
        ) -> HRESULT,
    >,
    pub MapSkeletonPointToDepthPoint: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiCoordinateMapper,
            pSkeletonPoint: *mut Vector4,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
        ) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapperVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "QueryInterface",
            "AddRef",
            "Release",
            "GetColorToDepthRelationalParameters",
            "NotifyParametersChanged",
            "MapColorFrameToDepthFrame",
            "MapColorFrameToSkeletonFrame",
            "MapDepthFrameToColorFrame",
            "MapDepthFrameToSkeletonFrame",
            "MapDepthPointToColorPoint",
            "MapDepthPointToSkeletonPoint",
            "MapSkeletonPointToColorPoint",
            "MapSkeletonPointToDepthPoint",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.QueryInterface,
            &&self.AddRef,
            &&self.Release,
            &&self.GetColorToDepthRelationalParameters,
            &&self.NotifyParametersChanged,
            &&self.MapColorFrameToDepthFrame,
            &&self.MapColorFrameToSkeletonFrame,
            &&self.MapDepthFrameToColorFrame,
            &&self.MapDepthFrameToSkeletonFrame,
            &&self.MapDepthPointToColorPoint,
            &&self.MapDepthPointToSkeletonPoint,
            &&self.MapSkeletonPointToColorPoint,
            &&self.MapSkeletonPointToDepthPoint,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "INuiCoordinateMapperVtbl",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapperVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapperVtbl {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapperVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiCoordinateMapper) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiCoordinateMapper) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    pDataByteCount: *mut ULONG,
                    ppData: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    pCallback: *mut INuiCoordinateMapperParametersChangedEvent,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eColorType: NUI_IMAGE_TYPE,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    cDepthPixels: DWORD,
                    pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
                    cDepthPoints: DWORD,
                    pDepthPoints: *mut NUI_DEPTH_IMAGE_POINT,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eColorType: NUI_IMAGE_TYPE,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    cDepthPixels: DWORD,
                    pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
                    cSkeletonPoints: DWORD,
                    pSkeletonPoints: *mut Vector4,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    cDepthPixels: DWORD,
                    pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
                    eColorType: NUI_IMAGE_TYPE,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    cColorPoints: DWORD,
                    pColorPoints: *mut NUI_COLOR_IMAGE_POINT,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    cDepthPixels: DWORD,
                    pDepthPixels: *mut NUI_DEPTH_IMAGE_PIXEL,
                    cSkeletonPoints: DWORD,
                    pSkeletonPoints: *mut Vector4,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
                    eColorType: NUI_IMAGE_TYPE,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    pColorPoint: *mut NUI_COLOR_IMAGE_POINT,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
                    pSkeletonPoint: *mut Vector4,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    pSkeletonPoint: *mut Vector4,
                    eColorType: NUI_IMAGE_TYPE,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    pColorPoint: *mut NUI_COLOR_IMAGE_POINT,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiCoordinateMapper,
                    pSkeletonPoint: *mut Vector4,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    pDepthPoint: *mut NUI_DEPTH_IMAGE_POINT,
                ) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiCoordinateMapper {
    pub lpVtbl: *mut INuiCoordinateMapperVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiCoordinateMapper",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiCoordinateMapper {}
#[automatically_derived]
impl ::core::clone::Clone for INuiCoordinateMapper {
    #[inline]
    fn clone(&self) -> INuiCoordinateMapper {
        let _: ::core::clone::AssertParamIsClone<*mut INuiCoordinateMapperVtbl>;
        *self
    }
}
extern "C" {
    pub static IID_INuiColorCameraSettings: IID;
}
#[repr(C)]
pub struct INuiColorCameraSettingsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> ULONG,
    >,
    pub SetAutoWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            AutoWhiteBalanceEnabled: BOOL,
        ) -> HRESULT,
    >,
    pub GetAutoWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pAutoWhiteBalanceEnabled: *mut BOOL,
        ) -> HRESULT,
    >,
    pub SetWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            WhiteBalance: LONG,
        ) -> HRESULT,
    >,
    pub GetWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pWhiteBalance: *mut LONG,
        ) -> HRESULT,
    >,
    pub GetMinWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pWhiteBalance: *mut LONG,
        ) -> HRESULT,
    >,
    pub GetMaxWhiteBalance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pWhiteBalance: *mut LONG,
        ) -> HRESULT,
    >,
    pub SetContrast: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            Contrast: f64,
        ) -> HRESULT,
    >,
    pub GetContrast: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pContrast: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinContrast: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pContrast: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxContrast: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pContrast: *mut f64,
        ) -> HRESULT,
    >,
    pub SetHue: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings, Hue: f64) -> HRESULT,
    >,
    pub GetHue: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pHue: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinHue: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pHue: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxHue: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pHue: *mut f64,
        ) -> HRESULT,
    >,
    pub SetSaturation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            Saturation: f64,
        ) -> HRESULT,
    >,
    pub GetSaturation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSaturation: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinSaturation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSaturation: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxSaturation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSaturation: *mut f64,
        ) -> HRESULT,
    >,
    pub SetGamma: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings, Gamma: f64) -> HRESULT,
    >,
    pub GetGamma: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGamma: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinGamma: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGamma: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxGamma: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGamma: *mut f64,
        ) -> HRESULT,
    >,
    pub SetSharpness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            Sharpness: f64,
        ) -> HRESULT,
    >,
    pub GetSharpness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSharpness: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinSharpness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSharpness: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxSharpness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pSharpness: *mut f64,
        ) -> HRESULT,
    >,
    pub SetAutoExposure: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            AutoExposureEnabled: BOOL,
        ) -> HRESULT,
    >,
    pub GetAutoExposure: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pAutoExposureEnabled: *mut BOOL,
        ) -> HRESULT,
    >,
    pub SetExposureTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            ExposureTime: f64,
        ) -> HRESULT,
    >,
    pub GetExposureTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pExposureTime: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinExposureTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pExposureTime: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxExposureTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pExposureTime: *mut f64,
        ) -> HRESULT,
    >,
    pub SetFrameInterval: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            FrameInterval: f64,
        ) -> HRESULT,
    >,
    pub GetFrameInterval: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pFrameInterval: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinFrameInterval: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pFrameInterval: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxFrameInterval: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pFrameInterval: *mut f64,
        ) -> HRESULT,
    >,
    pub SetBrightness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            Brightness: f64,
        ) -> HRESULT,
    >,
    pub GetBrightness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pBrightness: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinBrightness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pBrightness: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxBrightness: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pBrightness: *mut f64,
        ) -> HRESULT,
    >,
    pub SetPowerLineFrequency: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            PowerLineFrequency: NUI_POWER_LINE_FREQUENCY,
        ) -> HRESULT,
    >,
    pub GetPowerLineFrequency: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pPowerLineFrequency: *mut NUI_POWER_LINE_FREQUENCY,
        ) -> HRESULT,
    >,
    pub SetBacklightCompensationMode: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            BacklightCompensationMode: NUI_BACKLIGHT_COMPENSATION_MODE,
        ) -> HRESULT,
    >,
    pub GetBacklightCompensationMode: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pBacklightCompensationMode: *mut NUI_BACKLIGHT_COMPENSATION_MODE,
        ) -> HRESULT,
    >,
    pub SetGain: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings, Gain: f64) -> HRESULT,
    >,
    pub GetGain: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGain: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMinGain: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGain: *mut f64,
        ) -> HRESULT,
    >,
    pub GetMaxGain: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiColorCameraSettings,
            pGain: *mut f64,
        ) -> HRESULT,
    >,
    pub ResetCameraSettingsToDefault: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiColorCameraSettingsVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "QueryInterface",
            "AddRef",
            "Release",
            "SetAutoWhiteBalance",
            "GetAutoWhiteBalance",
            "SetWhiteBalance",
            "GetWhiteBalance",
            "GetMinWhiteBalance",
            "GetMaxWhiteBalance",
            "SetContrast",
            "GetContrast",
            "GetMinContrast",
            "GetMaxContrast",
            "SetHue",
            "GetHue",
            "GetMinHue",
            "GetMaxHue",
            "SetSaturation",
            "GetSaturation",
            "GetMinSaturation",
            "GetMaxSaturation",
            "SetGamma",
            "GetGamma",
            "GetMinGamma",
            "GetMaxGamma",
            "SetSharpness",
            "GetSharpness",
            "GetMinSharpness",
            "GetMaxSharpness",
            "SetAutoExposure",
            "GetAutoExposure",
            "SetExposureTime",
            "GetExposureTime",
            "GetMinExposureTime",
            "GetMaxExposureTime",
            "SetFrameInterval",
            "GetFrameInterval",
            "GetMinFrameInterval",
            "GetMaxFrameInterval",
            "SetBrightness",
            "GetBrightness",
            "GetMinBrightness",
            "GetMaxBrightness",
            "SetPowerLineFrequency",
            "GetPowerLineFrequency",
            "SetBacklightCompensationMode",
            "GetBacklightCompensationMode",
            "SetGain",
            "GetGain",
            "GetMinGain",
            "GetMaxGain",
            "ResetCameraSettingsToDefault",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.QueryInterface,
            &&self.AddRef,
            &&self.Release,
            &&self.SetAutoWhiteBalance,
            &&self.GetAutoWhiteBalance,
            &&self.SetWhiteBalance,
            &&self.GetWhiteBalance,
            &&self.GetMinWhiteBalance,
            &&self.GetMaxWhiteBalance,
            &&self.SetContrast,
            &&self.GetContrast,
            &&self.GetMinContrast,
            &&self.GetMaxContrast,
            &&self.SetHue,
            &&self.GetHue,
            &&self.GetMinHue,
            &&self.GetMaxHue,
            &&self.SetSaturation,
            &&self.GetSaturation,
            &&self.GetMinSaturation,
            &&self.GetMaxSaturation,
            &&self.SetGamma,
            &&self.GetGamma,
            &&self.GetMinGamma,
            &&self.GetMaxGamma,
            &&self.SetSharpness,
            &&self.GetSharpness,
            &&self.GetMinSharpness,
            &&self.GetMaxSharpness,
            &&self.SetAutoExposure,
            &&self.GetAutoExposure,
            &&self.SetExposureTime,
            &&self.GetExposureTime,
            &&self.GetMinExposureTime,
            &&self.GetMaxExposureTime,
            &&self.SetFrameInterval,
            &&self.GetFrameInterval,
            &&self.GetMinFrameInterval,
            &&self.GetMaxFrameInterval,
            &&self.SetBrightness,
            &&self.GetBrightness,
            &&self.GetMinBrightness,
            &&self.GetMaxBrightness,
            &&self.SetPowerLineFrequency,
            &&self.GetPowerLineFrequency,
            &&self.SetBacklightCompensationMode,
            &&self.GetBacklightCompensationMode,
            &&self.SetGain,
            &&self.GetGain,
            &&self.GetMinGain,
            &&self.GetMaxGain,
            &&self.ResetCameraSettingsToDefault,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "INuiColorCameraSettingsVtbl",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiColorCameraSettingsVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiColorCameraSettingsVtbl {
    #[inline]
    fn clone(&self) -> INuiColorCameraSettingsVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    AutoWhiteBalanceEnabled: BOOL,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pAutoWhiteBalanceEnabled: *mut BOOL,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    WhiteBalance: LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pWhiteBalance: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pWhiteBalance: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pWhiteBalance: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Contrast: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pContrast: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pContrast: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pContrast: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Hue: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pHue: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pHue: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pHue: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Saturation: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSaturation: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSaturation: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSaturation: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Gamma: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGamma: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGamma: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGamma: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Sharpness: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSharpness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSharpness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pSharpness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    AutoExposureEnabled: BOOL,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pAutoExposureEnabled: *mut BOOL,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    ExposureTime: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pExposureTime: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pExposureTime: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pExposureTime: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    FrameInterval: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pFrameInterval: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pFrameInterval: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pFrameInterval: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Brightness: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pBrightness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pBrightness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pBrightness: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    PowerLineFrequency: NUI_POWER_LINE_FREQUENCY,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pPowerLineFrequency: *mut NUI_POWER_LINE_FREQUENCY,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    BacklightCompensationMode: NUI_BACKLIGHT_COMPENSATION_MODE,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pBacklightCompensationMode: *mut NUI_BACKLIGHT_COMPENSATION_MODE,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    Gain: f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGain: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGain: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiColorCameraSettings,
                    pGain: *mut f64,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiColorCameraSettings) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiColorCameraSettings {
    pub lpVtbl: *mut INuiColorCameraSettingsVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiColorCameraSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiColorCameraSettings",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiColorCameraSettings {}
#[automatically_derived]
impl ::core::clone::Clone for INuiColorCameraSettings {
    #[inline]
    fn clone(&self) -> INuiColorCameraSettings {
        let _: ::core::clone::AssertParamIsClone<*mut INuiColorCameraSettingsVtbl>;
        *self
    }
}
extern "C" {
    pub static IID_INuiDepthFilter: IID;
}
#[repr(C)]
pub struct INuiDepthFilterVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiDepthFilter,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiDepthFilter) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiDepthFilter) -> ULONG,
    >,
    pub ProcessFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiDepthFilter,
            liTimeStamp: LARGE_INTEGER,
            width: UINT,
            height: UINT,
            pDepthImagePixels: *mut NUI_DEPTH_IMAGE_PIXEL,
            pFrameModified: *mut BOOL,
        ) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiDepthFilterVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "INuiDepthFilterVtbl",
            "QueryInterface",
            &&self.QueryInterface,
            "AddRef",
            &&self.AddRef,
            "Release",
            &&self.Release,
            "ProcessFrame",
            &&self.ProcessFrame,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiDepthFilterVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiDepthFilterVtbl {
    #[inline]
    fn clone(&self) -> INuiDepthFilterVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiDepthFilter,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiDepthFilter) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiDepthFilter) -> ULONG,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiDepthFilter,
                    liTimeStamp: LARGE_INTEGER,
                    width: UINT,
                    height: UINT,
                    pDepthImagePixels: *mut NUI_DEPTH_IMAGE_PIXEL,
                    pFrameModified: *mut BOOL,
                ) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiDepthFilter {
    pub lpVtbl: *mut INuiDepthFilterVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiDepthFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiDepthFilter",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiDepthFilter {}
#[automatically_derived]
impl ::core::clone::Clone for INuiDepthFilter {
    #[inline]
    fn clone(&self) -> INuiDepthFilter {
        let _: ::core::clone::AssertParamIsClone<*mut INuiDepthFilterVtbl>;
        *self
    }
}
extern "C" {
    pub static IID_INuiSensor: IID;
}
#[repr(C)]
pub struct INuiSensorVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> ULONG,
    >,
    pub NuiInitialize: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor, dwFlags: DWORD) -> HRESULT,
    >,
    pub NuiShutdown: ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor)>,
    pub NuiSetFrameEndEvent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hEvent: HANDLE,
            dwFrameEventFlag: DWORD,
        ) -> HRESULT,
    >,
    pub NuiImageStreamOpen: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            eImageType: NUI_IMAGE_TYPE,
            eResolution: NUI_IMAGE_RESOLUTION,
            dwImageFrameFlags: DWORD,
            dwFrameLimit: DWORD,
            hNextFrameEvent: HANDLE,
            phStreamHandle: *mut HANDLE,
        ) -> HRESULT,
    >,
    pub NuiImageStreamSetImageFrameFlags: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hStream: HANDLE,
            dwImageFrameFlags: DWORD,
        ) -> HRESULT,
    >,
    pub NuiImageStreamGetImageFrameFlags: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hStream: HANDLE,
            pdwImageFrameFlags: *mut DWORD,
        ) -> HRESULT,
    >,
    pub NuiImageStreamGetNextFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hStream: HANDLE,
            dwMillisecondsToWait: DWORD,
            pImageFrame: *mut NUI_IMAGE_FRAME,
        ) -> HRESULT,
    >,
    pub NuiImageStreamReleaseFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hStream: HANDLE,
            pImageFrame: *mut NUI_IMAGE_FRAME,
        ) -> HRESULT,
    >,
    pub NuiImageGetColorPixelCoordinatesFromDepthPixel: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            pcViewArea: *const NUI_IMAGE_VIEW_AREA,
            lDepthX: LONG,
            lDepthY: LONG,
            usDepthValue: USHORT,
            plColorX: *mut LONG,
            plColorY: *mut LONG,
        ) -> HRESULT,
    >,
    pub NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            pcViewArea: *const NUI_IMAGE_VIEW_AREA,
            lDepthX: LONG,
            lDepthY: LONG,
            usDepthValue: USHORT,
            plColorX: *mut LONG,
            plColorY: *mut LONG,
        ) -> HRESULT,
    >,
    pub NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            eColorResolution: NUI_IMAGE_RESOLUTION,
            eDepthResolution: NUI_IMAGE_RESOLUTION,
            cDepthValues: DWORD,
            pDepthValues: *mut USHORT,
            cColorCoordinates: DWORD,
            pColorCoordinates: *mut LONG,
        ) -> HRESULT,
    >,
    pub NuiCameraElevationSetAngle: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor, lAngleDegrees: LONG) -> HRESULT,
    >,
    pub NuiCameraElevationGetAngle: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor, plAngleDegrees: *mut LONG) -> HRESULT,
    >,
    pub NuiSkeletonTrackingEnable: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hNextFrameEvent: HANDLE,
            dwFlags: DWORD,
        ) -> HRESULT,
    >,
    pub NuiSkeletonTrackingDisable: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> HRESULT,
    >,
    pub NuiSkeletonSetTrackedSkeletons: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor, TrackingIDs: *mut DWORD) -> HRESULT,
    >,
    pub NuiSkeletonGetNextFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            dwMillisecondsToWait: DWORD,
            pSkeletonFrame: *mut NUI_SKELETON_FRAME,
        ) -> HRESULT,
    >,
    pub NuiTransformSmooth: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            pSkeletonFrame: *mut NUI_SKELETON_FRAME,
            pSmoothingParams: *const NUI_TRANSFORM_SMOOTH_PARAMETERS,
        ) -> HRESULT,
    >,
    pub NuiGetAudioSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            ppDmo: *mut *mut INuiAudioBeam,
        ) -> HRESULT,
    >,
    pub NuiInstanceIndex: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> ::std::os::raw::c_int,
    >,
    pub NuiDeviceConnectionId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR,
    >,
    pub NuiUniqueId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR,
    >,
    pub NuiAudioArrayId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR,
    >,
    pub NuiStatus: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> HRESULT,
    >,
    pub NuiInitializationFlags: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> DWORD,
    >,
    pub NuiGetCoordinateMapper: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            pMapping: *mut *mut INuiCoordinateMapper,
        ) -> HRESULT,
    >,
    pub NuiImageFrameGetDepthImagePixelFrameTexture: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            hStream: HANDLE,
            pImageFrame: *mut NUI_IMAGE_FRAME,
            pNearMode: *mut BOOL,
            ppFrameTexture: *mut *mut INuiFrameTexture,
        ) -> HRESULT,
    >,
    pub NuiGetColorCameraSettings: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            pCameraSettings: *mut *mut INuiColorCameraSettings,
        ) -> HRESULT,
    >,
    pub NuiGetForceInfraredEmitterOff: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor) -> BOOL,
    >,
    pub NuiSetForceInfraredEmitterOff: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            fForceInfraredEmitterOff: BOOL,
        ) -> HRESULT,
    >,
    pub NuiAccelerometerGetCurrentReading: ::std::option::Option<
        unsafe extern "C" fn(This: *mut INuiSensor, pReading: *mut Vector4) -> HRESULT,
    >,
    pub NuiSetDepthFilter: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            pDepthFilter: *mut INuiDepthFilter,
        ) -> HRESULT,
    >,
    pub NuiGetDepthFilter: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            ppDepthFilter: *mut *mut INuiDepthFilter,
        ) -> HRESULT,
    >,
    pub NuiGetDepthFilterForTimeStamp: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INuiSensor,
            liTimeStamp: LARGE_INTEGER,
            ppDepthFilter: *mut *mut INuiDepthFilter,
        ) -> HRESULT,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiSensorVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "QueryInterface",
            "AddRef",
            "Release",
            "NuiInitialize",
            "NuiShutdown",
            "NuiSetFrameEndEvent",
            "NuiImageStreamOpen",
            "NuiImageStreamSetImageFrameFlags",
            "NuiImageStreamGetImageFrameFlags",
            "NuiImageStreamGetNextFrame",
            "NuiImageStreamReleaseFrame",
            "NuiImageGetColorPixelCoordinatesFromDepthPixel",
            "NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution",
            "NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution",
            "NuiCameraElevationSetAngle",
            "NuiCameraElevationGetAngle",
            "NuiSkeletonTrackingEnable",
            "NuiSkeletonTrackingDisable",
            "NuiSkeletonSetTrackedSkeletons",
            "NuiSkeletonGetNextFrame",
            "NuiTransformSmooth",
            "NuiGetAudioSource",
            "NuiInstanceIndex",
            "NuiDeviceConnectionId",
            "NuiUniqueId",
            "NuiAudioArrayId",
            "NuiStatus",
            "NuiInitializationFlags",
            "NuiGetCoordinateMapper",
            "NuiImageFrameGetDepthImagePixelFrameTexture",
            "NuiGetColorCameraSettings",
            "NuiGetForceInfraredEmitterOff",
            "NuiSetForceInfraredEmitterOff",
            "NuiAccelerometerGetCurrentReading",
            "NuiSetDepthFilter",
            "NuiGetDepthFilter",
            "NuiGetDepthFilterForTimeStamp",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.QueryInterface,
            &&self.AddRef,
            &&self.Release,
            &&self.NuiInitialize,
            &&self.NuiShutdown,
            &&self.NuiSetFrameEndEvent,
            &&self.NuiImageStreamOpen,
            &&self.NuiImageStreamSetImageFrameFlags,
            &&self.NuiImageStreamGetImageFrameFlags,
            &&self.NuiImageStreamGetNextFrame,
            &&self.NuiImageStreamReleaseFrame,
            &&self.NuiImageGetColorPixelCoordinatesFromDepthPixel,
            &&self.NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution,
            &&self.NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution,
            &&self.NuiCameraElevationSetAngle,
            &&self.NuiCameraElevationGetAngle,
            &&self.NuiSkeletonTrackingEnable,
            &&self.NuiSkeletonTrackingDisable,
            &&self.NuiSkeletonSetTrackedSkeletons,
            &&self.NuiSkeletonGetNextFrame,
            &&self.NuiTransformSmooth,
            &&self.NuiGetAudioSource,
            &&self.NuiInstanceIndex,
            &&self.NuiDeviceConnectionId,
            &&self.NuiUniqueId,
            &&self.NuiAudioArrayId,
            &&self.NuiStatus,
            &&self.NuiInitializationFlags,
            &&self.NuiGetCoordinateMapper,
            &&self.NuiImageFrameGetDepthImagePixelFrameTexture,
            &&self.NuiGetColorCameraSettings,
            &&self.NuiGetForceInfraredEmitterOff,
            &&self.NuiSetForceInfraredEmitterOff,
            &&self.NuiAccelerometerGetCurrentReading,
            &&self.NuiSetDepthFilter,
            &&self.NuiGetDepthFilter,
            &&self.NuiGetDepthFilterForTimeStamp,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "INuiSensorVtbl",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiSensorVtbl {}
#[automatically_derived]
impl ::core::clone::Clone for INuiSensorVtbl {
    #[inline]
    fn clone(&self) -> INuiSensorVtbl {
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    riid: *const IID,
                    ppvObject: *mut *mut ::std::os::raw::c_void,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> ULONG>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> ULONG>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiSensor, dwFlags: DWORD) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor)>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hEvent: HANDLE,
                    dwFrameEventFlag: DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    eImageType: NUI_IMAGE_TYPE,
                    eResolution: NUI_IMAGE_RESOLUTION,
                    dwImageFrameFlags: DWORD,
                    dwFrameLimit: DWORD,
                    hNextFrameEvent: HANDLE,
                    phStreamHandle: *mut HANDLE,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hStream: HANDLE,
                    dwImageFrameFlags: DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hStream: HANDLE,
                    pdwImageFrameFlags: *mut DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hStream: HANDLE,
                    dwMillisecondsToWait: DWORD,
                    pImageFrame: *mut NUI_IMAGE_FRAME,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hStream: HANDLE,
                    pImageFrame: *mut NUI_IMAGE_FRAME,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const NUI_IMAGE_VIEW_AREA,
                    lDepthX: LONG,
                    lDepthY: LONG,
                    usDepthValue: USHORT,
                    plColorX: *mut LONG,
                    plColorY: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const NUI_IMAGE_VIEW_AREA,
                    lDepthX: LONG,
                    lDepthY: LONG,
                    usDepthValue: USHORT,
                    plColorX: *mut LONG,
                    plColorY: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    eColorResolution: NUI_IMAGE_RESOLUTION,
                    eDepthResolution: NUI_IMAGE_RESOLUTION,
                    cDepthValues: DWORD,
                    pDepthValues: *mut USHORT,
                    cColorCoordinates: DWORD,
                    pColorCoordinates: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    lAngleDegrees: LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    plAngleDegrees: *mut LONG,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hNextFrameEvent: HANDLE,
                    dwFlags: DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> HRESULT>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    TrackingIDs: *mut DWORD,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    dwMillisecondsToWait: DWORD,
                    pSkeletonFrame: *mut NUI_SKELETON_FRAME,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    pSkeletonFrame: *mut NUI_SKELETON_FRAME,
                    pSmoothingParams: *const NUI_TRANSFORM_SMOOTH_PARAMETERS,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    ppDmo: *mut *mut INuiAudioBeam,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(This: *mut INuiSensor) -> ::std::os::raw::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> BSTR>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> HRESULT>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> DWORD>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    pMapping: *mut *mut INuiCoordinateMapper,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    hStream: HANDLE,
                    pImageFrame: *mut NUI_IMAGE_FRAME,
                    pNearMode: *mut BOOL,
                    ppFrameTexture: *mut *mut INuiFrameTexture,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    pCameraSettings: *mut *mut INuiColorCameraSettings,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<unsafe extern "C" fn(This: *mut INuiSensor) -> BOOL>,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    fForceInfraredEmitterOff: BOOL,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    pReading: *mut Vector4,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    pDepthFilter: *mut INuiDepthFilter,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    ppDepthFilter: *mut *mut INuiDepthFilter,
                ) -> HRESULT,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::std::option::Option<
                unsafe extern "C" fn(
                    This: *mut INuiSensor,
                    liTimeStamp: LARGE_INTEGER,
                    ppDepthFilter: *mut *mut INuiDepthFilter,
                ) -> HRESULT,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct INuiSensor {
    pub lpVtbl: *mut INuiSensorVtbl,
}
#[automatically_derived]
impl ::core::fmt::Debug for INuiSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "INuiSensor",
            "lpVtbl",
            &&self.lpVtbl,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for INuiSensor {}
#[automatically_derived]
impl ::core::clone::Clone for INuiSensor {
    #[inline]
    fn clone(&self) -> INuiSensor {
        let _: ::core::clone::AssertParamIsClone<*mut INuiSensorVtbl>;
        *self
    }
}
extern "C" {
    #[must_use]
    /** <summary>
 Returns the number of Kinect sensors that are connected to the computer.
 </summary>
 <param name="pCount">Pointer to an integer which receives the number of Kinect sensors.</param>
 <returns>
 <para>Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="pCount"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>*/
    pub fn NuiGetSensorCount(pCount: *mut ::std::os::raw::c_int) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Creates an instance of the sensor with the specified index so that an application can open and use it.
 </summary>
 <param name="index">
 The zero-based index of the sensor to open. Valid values range from zero to one less than the
 value returned by the NuiGetSensorCount function.
 </param>
 <param name="ppNuiSensor">A pointer that receives a reference to the created INuiSensor interface. This must not be NULL.</param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following error codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="index"/> parameter is negative.</description>
    </item>
    <item>
       <term>E_NUI_BADINDEX</term>
       <description>The <paramref name="index"/> parameter is out of range.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="ppNuiSensor"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>Applications that support more than one Kinect sensor call this function to access the
 second and subsequent devices. This function returns a pointer to an INuiSensor interface,
 which provides functions that are identical to those in NuiApi.h</para>

 <para>Repeated calls to this method with the same ID may return the same interface pointer.</para>
 </remarks>*/
    pub fn NuiCreateSensorByIndex(
        index: ::std::os::raw::c_int,
        ppNuiSensor: *mut *mut INuiSensor,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Creates an instance of the sensor with the specified ID so that an application can open and use it.
 </summary>
 <param name="strInstanceId">A pointer to the ID of the Kinect sensor to open. This must not be NULL.</param>
 <param name="ppNuiSensor">A pointer that receives a reference to the created INuiSensor interface. This must not be NULL.</param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following error codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="strInstanceId"/> parameter is NULL.</description>
    </item>
    <item>
       <term>E_NUI_BADINDEX</term>
       <description>The <paramref name="strInstanceId"/> parameter does not match any attached device.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="ppNuiSensor"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>Applications that support more than one Kinect sensor call this function to access the
 second and subsequent devices. This function returns a pointer to an INuiSensor interface,
 which provides functions that are identical to those in NuiApi.h</para>

 <para>Repeated calls to this method with the same ID may return the same interface pointer.</para>
 </remarks>*/
    pub fn NuiCreateSensorById(
        strInstanceId: *const OLECHAR,
        ppNuiSensor: *mut *mut INuiSensor,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn NuiGetAudioSource(ppDmo: *mut *mut INuiAudioBeam) -> HRESULT;
}
pub type NuiStatusProc = ::std::option::Option<
    unsafe extern "C" fn(
        hrStatus: HRESULT,
        instanceName: *const OLECHAR,
        uniqueDeviceName: *const OLECHAR,
        pUserData: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    /** <summary>
 Sets a callback function that gets notified when the sensor connection status changes.
 </summary>
 <param name="callback">Function pointer to the callback.</param>
 <param name="pUserData">Pointer to optional context data that will be passed to the callback.</param>
 <remarks>
 Use this method to handle the case of a user connecting or disconnecting a sensor.
 </remarks>*/
    pub fn NuiSetDeviceStatusCallback(
        callback: NuiStatusProc,
        pUserData: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
pub struct NUI_MICROPHONE_ARRAY_DEVICE {
    pub szDeviceName: [wchar_t; 512usize],
    pub szDeviceID: [wchar_t; 512usize],
    pub iDeviceIndex: ::std::os::raw::c_int,
}
#[automatically_derived]
impl ::core::fmt::Debug for NUI_MICROPHONE_ARRAY_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "NUI_MICROPHONE_ARRAY_DEVICE",
            "szDeviceName",
            &&self.szDeviceName,
            "szDeviceID",
            &&self.szDeviceID,
            "iDeviceIndex",
            &&self.iDeviceIndex,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for NUI_MICROPHONE_ARRAY_DEVICE {}
#[automatically_derived]
impl ::core::clone::Clone for NUI_MICROPHONE_ARRAY_DEVICE {
    #[inline]
    fn clone(&self) -> NUI_MICROPHONE_ARRAY_DEVICE {
        let _: ::core::clone::AssertParamIsClone<[wchar_t; 512usize]>;
        let _: ::core::clone::AssertParamIsClone<[wchar_t; 512usize]>;
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
        *self
    }
}
pub type PNUI_MICROPHONE_ARRAY_DEVICE = *mut NUI_MICROPHONE_ARRAY_DEVICE;
extern "C" {
    #[must_use]
    /** <summary>
 Gets device information for the connected Kinect sensors.
 </summary>
 <param name="pDeviceInfo">
 Pointer to an array of NUI_MICROPHONE_ARRAY_DEVICE structures, allocated by the caller, each of
 which receives the device information for a single connected Kinect sensor. If you set this
 parameter to NULL, the <paramref name="piDeviceCount"/> parameter will still receive the number
 of connected Kinect sensors.
 </param>
 <param name="size">
 Size of the array pointed to by the <paramref name="pDeviceInfo"/> parameter.
 </param>
 <param name="piDeviceCount">
 Receives the number of connected Kinect sensors. When this function returns, this parameter is
 set to the number of structures in the array pointed to by the <paramref name="pDeviceInfo"/>
 parameter that contain valid information.
 </param>
 <returns>
 Returns S_OK if successful; otherwise returns a failure code.
 </returns>*/
    pub fn NuiGetMicrophoneArrayDevices(
        pDeviceInfo: PNUI_MICROPHONE_ARRAY_DEVICE,
        size: ::std::os::raw::c_int,
        piDeviceCount: *mut ::std::os::raw::c_int,
    ) -> HRESULT;
}
#[repr(C)]
pub struct NUI_SPEAKER_DEVICE {
    pub szDeviceName: [wchar_t; 512usize],
    pub iDeviceIndex: ::std::os::raw::c_int,
    pub fDefault: bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for NUI_SPEAKER_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "NUI_SPEAKER_DEVICE",
            "szDeviceName",
            &&self.szDeviceName,
            "iDeviceIndex",
            &&self.iDeviceIndex,
            "fDefault",
            &&self.fDefault,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for NUI_SPEAKER_DEVICE {}
#[automatically_derived]
impl ::core::clone::Clone for NUI_SPEAKER_DEVICE {
    #[inline]
    fn clone(&self) -> NUI_SPEAKER_DEVICE {
        let _: ::core::clone::AssertParamIsClone<[wchar_t; 512usize]>;
        let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
        let _: ::core::clone::AssertParamIsClone<bool>;
        *self
    }
}
pub type PNUI_SPEAKER_DEVICE = *mut NUI_SPEAKER_DEVICE;
extern "C" {
    #[must_use]
    /** <summary>
 Gets the active speaker devices found on the system.
 </summary>
 <param name="pDeviceInfo">
 Pointer to an array of NUI_SPEAKER_DEVICE structures, allocated by the caller, each of which
 receives the device information for a single connected speaker device. If you set this
 parameter to NULL, the <paramref name="piDeviceCount"/> parameter will still receive the number
 of connected speaker devices.
 </param>
 <param name="size">
 Size of the array pointed to by the <paramref name="pDeviceInfo"/> parameter.
 </param>
 <param name="piDeviceCount">
 Receives the number of connected speaker devices. When this function returns, this parameter is
 set to the number of structures in the array pointed to by the <paramref name="pDeviceInfo"/>
 parameter that contain valid information.
 </param>
 <returns>
 Returns S_OK if successful; otherwise returns a failure code.
 </returns>*/
    pub fn NuiGetSpeakerDevices(
        pDeviceInfo: PNUI_SPEAKER_DEVICE,
        size: ::std::os::raw::c_int,
        piDeviceCount: *mut ::std::os::raw::c_int,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    pub fn NuiCreateCoordinateMapperFromParameters(
        dataByteCount: ULONG,
        pData: *mut ::std::os::raw::c_void,
        ppCoordinateMapper: *mut *mut INuiCoordinateMapper,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Creates a filter that can be applied to depth frames.
 </summary>
 <param name="filename">
 The name of the DLL that implements the depth filter. Must be an absolute path or
 a simple filename. Other forms of relative paths are not permitted. If a simple
 filename, the file must be found in the same directory as the process executable.
 </param>
 <param name="factoryEntryPoint">
 The name of the DLL entry point to instantiate the depth filter.
 </param>
 <param name="ppDepthFilter">
 Receives a reference-counted pointer to the newly created filter.
 </param>
 <returns>
 Returns S_OK if successful; otherwise returns a failure code.
 </returns>*/
    pub fn NuiCreateDepthFilter(
        filename: LPCWSTR,
        factoryEntryPoint: LPCSTR,
        ppDepthFilter: *mut *mut INuiDepthFilter,
    ) -> HRESULT;
}
extern "C" {
    pub static mut __MIDL_itf_Kinect_0001_0070_v0_0_c_ifspec: RPC_IF_HANDLE;
}
extern "C" {
    pub static mut __MIDL_itf_Kinect_0001_0070_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub const _NUI_IMAGE_DIGITALZOOM_NUI_IMAGE_DIGITAL_ZOOM_1X: _NUI_IMAGE_DIGITALZOOM = 0;
pub type _NUI_IMAGE_DIGITALZOOM = ::std::os::raw::c_int;
pub use self::_NUI_IMAGE_DIGITALZOOM as NUI_IMAGE_DIGITALZOOM;
extern "C" {
    #[must_use]
    /** <summary>
 Sets the image frame flags for the specified stream.
 </summary>
 <param name="hStream">A handle to the stream.</param>
 <param name="dwImageFrameFlags">The image frame flags, as a bitwise-OR combination of the NUI_IMAGE_STREAM_FLAG constants.</param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="hStream"/> parameter is NULL.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
 </list>
 </returns>
 <remarks>
 The maximum number of output frames you can set is defined by NUI_IMAGE_STREAM_FRAME_LIMIT_MAXIMUM.
 </remarks>*/
    pub fn NuiImageStreamSetImageFrameFlags(
        hStream: HANDLE,
        dwImageFrameFlags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Gets the image frame flags for the specified stream.
 </summary>
 <param name="hStream">A handle to the stream.</param>
 <param name="pdwImageFrameFlags">A pointer to a DWORD that receives the image frame flags.</param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="hStream"/> parameter is NULL.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="pdwImageFrameFlags"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>*/
    pub fn NuiImageStreamGetImageFrameFlags(
        hStream: HANDLE,
        pdwImageFrameFlags: *mut DWORD,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Sets the event that signals the last frame.
 </summary>
 <param name="hEvent">A handle to the event.</param>
 <param name="dwFrameEventFlag">
 The frame event options, as a bitwise-OR combination of the NUI_IMAGE_STREAM_FLAG constants.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following error codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="hEvent"/> parameter is an invalid handle.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>If the frame does not process any data, the event is still signaled unless you specify
 NUI_FRAME_EVENT_FLAG_SUPPRESS_NO_FRAME_DATA.</para>

 <para>The event provided is signaled after the NUI runtime is finished processing all data
 associated with a frame. When you use NuiImageStreamGetNextFrame and NuiSkeletonGetNextFrame,
 all stream data generated in that frame is available before the event is signaled.</para>

 <para>This event is never reset by the NUI runtime because there is not a well-defined time to
 do so. This is unlike the events provided to NuiImageStreamOpen and NuiSkeletonTrackingEnable.
 Similarly, proper operation requires an auto-reset event, instead of a manual reset event.</para>

 <para>When pumping multiple streams with a single thread, you should use a NuiSetFrameEndEvent
 event over separate per-stream events.</para>

 <para>The flag NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA still applies on a per-stream basis
 when you call NuiImageStreamGetNextFrame provided you specify that flag for the stream in
 NuiImageStreamOpen.</para>

 <para>The flag NUI_SKELETON_TRACKING_FLAG_SUPPRESS_NO_FRAME_DATA still applies on a per-stream
 basis when you call NuiSkeletonGetNextFrame provided you specify that flag for the stream in
 NuiSkeletonTrackingEnable.</para>

 <para>When processing multiple streams on multiple threads, it's more efficient to create a
 separate event for each stream instead of using NuiSetFrameEndEvent. Using separate events
 ensures that the app doesn't have to wait until processing for skeleton tracking is done before
 it can get color and depth data.</para>

 <para>For signaling when skeleton data is available, use an event in NuiSkeletonTrackingEnable.
 For signaling when an image stream is available, use an event in NuiImageStreamOpen.</para>
 </remarks>*/
    pub fn NuiSetFrameEndEvent(hEvent: HANDLE, dwFrameEventFlag: DWORD) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Opens an image stream.
 </summary>
 <param name="eImageType">
 A NUI_IMAGE_TYPE value that specifies which image stream to open. The valid values for this
 parameter depend on the flags passed to the NuiInitialize method; for more information see
 remarks.
 </param>
 <param name="eResolution">
 A NUI_IMAGE_RESOLUTION value that specifies which resolution to use for the image stream. The
 valid values for this parameter depend on the flags passed to the NuiInitialize method; for
 more information, see remarks.
 </param>
 <param name="dwImageFrameFlags">
 The stream options, as a bitwise-OR combination of the NUI_IMAGE_STREAM_FLAG constants.
 </param>
 <param name="dwFrameLimit">
 The number of frames that the NUI runtime should buffer. The maximum value is
 NUI_IMAGE_STREAM_FRAME_LIMIT_MAXIMUM. Most applications should use a frame limit of two.
 </param>
 <param name="hNextFrameEvent">
 A handle to a manual reset event that will be fired when the next frame in the stream is
 available.
 </param>
 <param name="phStreamHandle">
 A pointer that receives a handle to the opened stream. This must not be NULL.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="dwFrameLimit"/> parameter is out of range.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device has not been initialized. <see cref="NuiInitialize"/>.</description>
    </item>
    <item>
       <term>E_OUTOFMEMORY</term>
       <description>The <paramref name="hEvent"/> parameter is an invalid handle.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="hEvent"/> parameter is an invalid handle.</description>
    </item>
    <item>
       <term>E_FAIL</term>
       <description>An unspecified error occurred.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>The NUI runtime buffers the number of frames specified by the dwFrameLimit parameter. If
 the application does not retrieve and release a frame before the buffer is full, the runtime
 replaces the oldest frame in the buffer with an incoming frame. As a result, frames can
 occasionally be dropped.</para>

 <para>The valid values for the eImageType and eResolution parameters depend on the NUI
 initialization flags passed to the NuiInitialize method in the dwFlags parameter. The following
 tables summarize the combinations that are currently valid.</para>

 <para>If <paramref name="dwFlags"/> includes NUI_INITIALIZE_FLAG_USES_DEPTH:
 <list>
    <listheader>
       <term>NUI_IMAGE_TYPE value</term>
       <description>NUI_IMAGE_RESOLUTION value</description>
    </listheader>
    <item>
       <term>NUI_IMAGE_TYPE_DEPTH</term>
       <description>NUI_IMAGE_RESOLUTION_640x480</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_DEPTH</term>
       <description>NUI_IMAGE_RESOLUTION_320x240</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_DEPTH</term>
       <description>NUI_IMAGE_RESOLUTION_80x60</description>
    </item>
 </list></para>

 <para>If <paramref name="dwFlags"/> includes NUI_INITIALIZE_FLAG_USES_DEPTH_AND_PLAYER_INDEX:
 <list>
    <listheader>
       <term>NUI_IMAGE_TYPE value</term>
       <description>NUI_IMAGE_RESOLUTION value</description>
    </listheader>
    <item>
       <term>NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX</term>
       <description>NUI_IMAGE_RESOLUTION_320x240</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX</term>
       <description>NUI_IMAGE_RESOLUTION_80x60</description>
    </item>
 </list></para>

 <para>If <paramref name="dwFlags"/> includes NUI_INITIALIZE_FLAG_USES_COLOR:
 <list>
    <listheader>
       <term>NUI_IMAGE_TYPE value</term>
       <description>NUI_IMAGE_RESOLUTION value</description>
    </listheader>
    <item>
       <term>NUI_IMAGE_TYPE_COLOR</term>
       <description>NUI_IMAGE_RESOLUTION_1280x960</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_COLOR</term>
       <description>NUI_IMAGE_RESOLUTION_640x480</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_COLOR_YUV</term>
       <description>NUI_IMAGE_RESOLUTION_640x480</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_COLOR_RAW_YUV</term>
       <description>NUI_IMAGE_RESOLUTION_640x480</description>
    </item>
    <item>
       <term>NUI_IMAGE_TYPE_COLOR_INFRARED</term>
       <description>NUI_IMAGE_RESOLUTION_640x480</description>
    </item>
 </list></para>
 </remarks>*/
    pub fn NuiImageStreamOpen(
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
    /** <summary>
 Gets the next frame of data from the specified image stream.
 </summary>
 <param name="hStream">
 A handle to the image stream. This stream must have been opened by a call to the
 NuiImageStreamOpen method.
 </param>
 <param name="dwMillisecondsToWait">
 The timeout (in milliseconds) before returning without a new frame.
 </param>
 <param name="pImageFrame">
 A pointer to a NUI_IMAGE_FRAME structure that receives the next image frame in the specified
 stream. The pFrameTexture member of the structure points to an INuiFrameTexture instance that
 contains the frame data.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes: , .</para>
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>S_FALSE</term>
       <description>The wait timeout expired before a new frame was available and <paramref name="hStream"/> was opened with the NUI_IMAGE_STREAM_FLAG_SUPPRESS_NO_FRAME_DATA flag.</description>
    </item>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="hStream"/> parameter is NULL.</description>
    </item>
    <item>
       <term>E_NUI_FRAME_NO_DATA</term>
       <description>The wait timeout expired before a new frame was available.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="pImageFrame"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>*/
    pub fn NuiImageStreamGetNextFrame(
        hStream: HANDLE,
        dwMillisecondsToWait: DWORD,
        ppcImageFrame: *mut *const NUI_IMAGE_FRAME,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Releases the specified frame of data from the specified stream.
 </summary>
 <param name="hStream">
 A handle to the image stream. This stream must have been opened by a call to the
 NuiImageStreamOpen method.
 </param>
 <param name="pImageFrame">A pointer to the frame to release.</param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="hStream"/> or <paramref name="pImageFrame"/> parameter is NULL.</description>
    </item>
    <item>
       <term>E_NOINTERFACE</term>
       <description>The <paramref name="pImageFrame"/> parameter's <c>pFrameTexture</c> member is NULL.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
 </list>
 </returns>
 <remarks>
 Before you call this function, reset the notification event for the stream.
 </remarks>*/
    pub fn NuiImageStreamReleaseFrame(
        hStream: HANDLE,
        pImageFrame: *const NUI_IMAGE_FRAME,
    ) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Gets the pixel coordinates in color space that correspond to the specified pixel coordinates in
 depth space.
 </summary>
 <param name="eColorResolution">
 The resolution of the color image, as a NUI_IMAGE_RESOLUTION enumeration constant.
 </param>
 <param name="pcViewArea">
 The optional zoom and pan settings of the color image, as a pointer to a NUI_IMAGE_VIEW_AREA
 structure. To ensure that the settings are valid, use the ViewArea member of the NUI_IMAGE_FRAME
 that you are registering pixels against. Do not instantiate and populate this structure manually.
 </param>
 <param name="lDepthX">The X coordinate in depth image space.</param>
 <param name="lDepthY">The Y coordinate in depth image space.</param>
 <param name="usDepthValue">
 The depth value in depth image space. This value is constrained between NUI_IMAGE_DEPTH_MINIMUM
 and NUI_IMAGE_DEPTH_MAXIMUM.
 </param>
 <param name="plColorX">
 Pointer to a LONG value that receives the X coordinate of the pixel in color image space. This
 pointer must be non-NULL when you call this function. If this method does not return S_OK, this
 data is invalid. This value can be outside of the bounds of the color image.
 </param>
 <param name="plColorY">
 Pointer to a LONG value that receives the Y coordinate of the pixel in color image space. This
 pointer must be non-NULL when you call this function. If this method does not return S_OK, this
 data is invalid. This value can be outside of the bounds of the color image.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="eColorResolution"/> parameter does not specify a valid resolution, or pcViewArea is provided but does not describe the full frame.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="plColorX"/> or <paramref name="plColorY"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>Because depth image data and color image data come from separate sensors, pixels in the
 two images may not always line up exactly. The two sensors may have different fields of view,
 or may not be aimed precisely in the same direction. This means that a point near the edge of
 the depth image may correspond to a pixel just beyond the edge of the color image, or vice
 versa.</para>

 <para>This function accepts coordinates outside the bounds of the depth image. It may return
 pixels outside the color image. This means that you can use data from the two images in
 combination, even when the two images do not line up completely. You must verify that the
 coordinates that are returned lie within the color image before using the coordinates to
 reference pixels in that color image.</para>

 <para>The depth image coordinates you specify are not required to be within the bounds of the
 depth frame image, but they should not be too far outside the depth frame image bounds. If the
 coordinates are far outside the depth frame image, they are unlikely to map to coordinates
 inside the bounds of the color image. This function will then return color image coordinates
 that are unlikely to be useful.</para>
 </remarks>
 <seealso cref="NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution"/>*/
    pub fn NuiImageGetColorPixelCoordinatesFromDepthPixel(
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
    /** <summary>
 Gets the pixel coordinates in color space that correspond to the specified pixel coordinates in
 depth space, using the specified depth resolution.
 </summary>
 <param name="eColorResolution">
 The resolution of the color image, as a NUI_IMAGE_RESOLUTION enumeration constant.
 </param>
 <param name="eDepthResolution">
 The resolution of the depth image, as a NUI_IMAGE_RESOLUTION enumeration constant.
 </param>
 <param name="pcViewArea">
 The optional zoom and pan settings of the color image, as a pointer to a NUI_IMAGE_VIEW_AREA
 structure. To ensure that the settings are valid, use the ViewArea member of the NUI_IMAGE_FRAME
 that you are registering pixels against. Do not instantiate and populate this structure manually.
 </param>
 <param name="lDepthX">The X coordinate in depth image space.</param>
 <param name="lDepthY">The Y coordinate in depth image space.</param>
 <param name="usDepthValue">
 The depth value in depth image space. This value is constrained between NUI_IMAGE_DEPTH_MINIMUM
 and NUI_IMAGE_DEPTH_MAXIMUM.
 </param>
 <param name="plColorX">
 Pointer to a LONG value that receives the X coordinate of the pixel in color image space. This
 pointer must be non-NULL when you call this function. If this method does not return S_OK, this
 data is invalid. This value can be outside of the bounds of the color image.
 </param>
 <param name="plColorY">
 Pointer to a LONG value that receives the Y coordinate of the pixel in color image space. This
 pointer must be non-NULL when you call this function. If this method does not return S_OK, this
 data is invalid. This value can be outside of the bounds of the color image.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_INVALIDARG</term>
       <description>The <paramref name="eColorResolution"/> or <paramref name="eDepthResolution"/> parameter does not specify a valid resolution, or pcViewArea is provided but does not describe the full frame.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="plColorX"/> or <paramref name="plColorY"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>Because depth image data and color image data come from separate sensors, pixels in the
 two images may not always line up exactly. The two sensors may have different fields of view,
 or may not be aimed precisely in the same direction. This means that a point near the edge of
 the depth image may correspond to a pixel just beyond the edge of the color image, or vice
 versa.</para>

 <para>This function accepts coordinates outside the bounds of the depth image. It may return
 pixels outside the color image. This means that you can use data from the two images in
 combination, even when the two images do not line up completely. You must verify that the
 coordinates that are returned lie within the color image before using the coordinates to
 reference pixels in that color image.</para>

 <para>The depth image coordinates you specify are not required to be within the bounds of the
 depth frame image, but they should not be too far outside the depth frame image bounds. If the
 coordinates are far outside the depth frame image, they are unlikely to map to coordinates
 inside the bounds of the color image. This function will then return color image coordinates
 that are unlikely to be useful.</para>
 </remarks>
 <seealso cref="NuiImageGetColorPixelCoordinatesFromDepthPixel"/>*/
    pub fn NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(
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
    /** <summary>
 Gets the elevation angle of the Kinect sensor.
 </summary>
 <param name="plAngleDegrees">Pointer to a LONG which receives the angle of the sensor in degrees.</param>
 <returns>
 Returs S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
    <item>
       <term>E_POINTER</term>
       <description>The <paramref name="plAngleDegrees"/> parameter is NULL.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>An elevation angle of zero indicates that the Kinect sensor is pointing perpendicular to
 gravity. The tilt is relative to gravity rather than relative to the sensor base. The angle is
 subject to the physical limitations of the sensor. If the sensor base is resting on a tilted
 surface, the middle of the sensor's tilt range will not correspond to a tilt angle of zero, and
 the sensor may not be physically capable of reaching the outer limits of the range allowed by
 the API. If the sensor is moved so that the base is at a different angle relative to gravity,
 or if the sensor is tilted manually, the angle reported by the API will change, even if the
 tilt angle has not been changed programmatically.</para>
 </remarks>
 <seealso cref="NuiCameraElevationSetAngle"/>*/
    pub fn NuiCameraElevationGetAngle(plAngleDegrees: *mut LONG) -> HRESULT;
}
extern "C" {
    #[must_use]
    /** <summary>
 Sets the elevation angle of the Kinect sensor.
 </summary>
 <param name="lAngleDegrees">
 The elevation angle relative to gravity, in degrees. A value of zero indicates that the sensor
 array should point exactly horizontally. Positive values indicate that the sensor array should
 point above the horizon, and negative values indicate that the sensor array should point below
 the horizon. This value is constrained between NUI_CAMERA_ELEVATION_MINIMUM and
 NUI_CAMERA_ELEVATION_MAXIMUM.
 </param>
 <returns>
 Returns S_OK if successful; otherwise, returns one of the following failure codes:
 <list type="table">
    <listheader>
       <term>Error code</term>
       <description>Description</description>
    </listheader>
    <item>
       <term>__HRESULT_FROM_WIN32(ERROR_TOO_MANY_CMDS)</term>
       <description>There were too many calls to <c>NuiCameraElevationSetAngle</c> within a given timespan.</description>
    </item>
    <item>
       <term>__HRESULT_FROM_WIN32(ERROR_RETRY)</term>
       <description>There was too little time between subsequent <c>NuiCameraElevationSetAngle</c> calls.</description>
    </item>
    <item>
       <term>E_NUI_DEVICE_NOT_READY</term>
       <description>The device is uninitialized. <see cref="NuiInitialize"/>.</description>
    </item>
 </list>
 </returns>
 <remarks>
 <para>You should tilt the Kinect sensor as few times as possible, to minimize wear on the
 sensor and to minimize tilting time. The tilt motor is not designed for constant or repetitive
 movement, and attempts to use it that way can cause degradation of motor function. To reduce
 wear on the Kinect sensor's tilt motor, your application can change the elevation angle no more
 than once per second. In addition, you must allow at least 20 seconds of rest after 15
 consecutive changes. If your application exceeds these limits, additional attempts to set the
 elevation angle during the lockout period will result in an error code.</para>

 <para>An elevation angle of zero indicates that the Kinect sensor is pointing perpendicular to
 gravity. The tilt is relative to gravity rather than relative to the sensor base. The angle is
 subject to the physical limitations of the sensor. If the sensor base is resting on a tilted
 surface, the middle of the sensor's tilt range will not correspond to a tilt angle of zero, and
 the sensor may not be physically capable of reaching the outer limits of the range allowed by
 the API. If the sensor is moved so that the base is at a different angle relative to gravity,
 or if the sensor is tilted manually, the angle reported by the API will change, even if the
 tilt angle has not been changed programmatically.</para>
 </remarks>
 <seealso cref="NuiCameraElevationGetAngle"/>*/
    pub fn NuiCameraElevationSetAngle(lAngleDegrees: LONG) -> HRESULT;
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
