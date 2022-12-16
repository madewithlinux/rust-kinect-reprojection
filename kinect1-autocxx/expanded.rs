#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use autocxx::prelude::*;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod ffi {
    pub trait ToCppString {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString>;
    }
    impl ToCppString for &str {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(&self)
        }
    }
    impl ToCppString for &String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for cxx::UniquePtr<cxx::CxxString> {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            self
        }
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_IMAGE_TYPE {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::I,
            ::cxx::M,
            ::cxx::A,
            ::cxx::G,
            ::cxx::E,
            ::cxx::__,
            ::cxx::T,
            ::cxx::Y,
            ::cxx::P,
            ::cxx::E,
        );
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_IMAGE_RESOLUTION {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::I,
            ::cxx::M,
            ::cxx::A,
            ::cxx::G,
            ::cxx::E,
            ::cxx::__,
            ::cxx::R,
            ::cxx::E,
            ::cxx::S,
            ::cxx::O,
            ::cxx::L,
            ::cxx::U,
            ::cxx::T,
            ::cxx::I,
            ::cxx::O,
            ::cxx::N,
        );
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_IMAGE_FRAME {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::I,
            ::cxx::M,
            ::cxx::A,
            ::cxx::G,
            ::cxx::E,
            ::cxx::__,
            ::cxx::F,
            ::cxx::R,
            ::cxx::A,
            ::cxx::M,
            ::cxx::E,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_IMAGE_VIEW_AREA {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::I,
            ::cxx::M,
            ::cxx::A,
            ::cxx::G,
            ::cxx::E,
            ::cxx::__,
            ::cxx::V,
            ::cxx::I,
            ::cxx::E,
            ::cxx::W,
            ::cxx::__,
            ::cxx::A,
            ::cxx::R,
            ::cxx::E,
            ::cxx::A,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_SKELETON_FRAME {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::S,
            ::cxx::K,
            ::cxx::E,
            ::cxx::L,
            ::cxx::E,
            ::cxx::T,
            ::cxx::O,
            ::cxx::N,
            ::cxx::__,
            ::cxx::F,
            ::cxx::R,
            ::cxx::A,
            ::cxx::M,
            ::cxx::E,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::_NUI_TRANSFORM_SMOOTH_PARAMETERS {
        type Id = (
            ::cxx::__,
            ::cxx::N,
            ::cxx::U,
            ::cxx::I,
            ::cxx::__,
            ::cxx::T,
            ::cxx::R,
            ::cxx::A,
            ::cxx::N,
            ::cxx::S,
            ::cxx::F,
            ::cxx::O,
            ::cxx::R,
            ::cxx::M,
            ::cxx::__,
            ::cxx::S,
            ::cxx::M,
            ::cxx::O,
            ::cxx::O,
            ::cxx::T,
            ::cxx::H,
            ::cxx::__,
            ::cxx::P,
            ::cxx::A,
            ::cxx::R,
            ::cxx::A,
            ::cxx::M,
            ::cxx::E,
            ::cxx::T,
            ::cxx::E,
            ::cxx::R,
            ::cxx::S,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::_Vector4 {
        type Id = (
            ::cxx::__,
            ::cxx::V,
            ::cxx::e,
            ::cxx::c,
            ::cxx::t,
            ::cxx::o,
            ::cxx::r,
            ::cxx::_4,
        );
        type Kind = cxx::kind::Opaque;
    }
    mod bindgen {
        pub(super) mod root {
            pub use cxxbridge::INuiSensor;
            ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
            pub struct NuiCreateSensorByIndex;
            pub type DWORD = autocxx::c_ulong;
            pub type HRESULT = autocxx::c_long;
            pub type HANDLE = *mut autocxx::c_void;
            pub use root::_NUI_IMAGE_TYPE as NUI_IMAGE_TYPE;
            #[repr(i32)]
            pub enum _NUI_IMAGE_TYPE {
                NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX = 0,
                NUI_IMAGE_TYPE_COLOR = 1,
                NUI_IMAGE_TYPE_COLOR_YUV = 2,
                NUI_IMAGE_TYPE_COLOR_RAW_YUV = 3,
                NUI_IMAGE_TYPE_DEPTH = 4,
                NUI_IMAGE_TYPE_COLOR_INFRARED = 5,
                NUI_IMAGE_TYPE_COLOR_RAW_BAYER = 6,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for _NUI_IMAGE_TYPE {
                #[inline]
                fn clone(&self) -> _NUI_IMAGE_TYPE {
                    match self {
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_DEPTH_AND_PLAYER_INDEX
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_YUV => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_YUV
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_RAW_YUV => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_RAW_YUV
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_DEPTH => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_DEPTH
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_INFRARED => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_INFRARED
                        }
                        _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_RAW_BAYER => {
                            _NUI_IMAGE_TYPE::NUI_IMAGE_TYPE_COLOR_RAW_BAYER
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for _NUI_IMAGE_TYPE {
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_tag, state)
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for _NUI_IMAGE_TYPE {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for _NUI_IMAGE_TYPE {
                #[inline]
                fn eq(&self, other: &_NUI_IMAGE_TYPE) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for _NUI_IMAGE_TYPE {}
            #[automatically_derived]
            impl ::core::cmp::Eq for _NUI_IMAGE_TYPE {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            pub use root::_NUI_IMAGE_RESOLUTION as NUI_IMAGE_RESOLUTION;
            #[repr(i32)]
            pub enum _NUI_IMAGE_RESOLUTION {
                NUI_IMAGE_RESOLUTION_INVALID = -1,
                NUI_IMAGE_RESOLUTION_80x60 = 0,
                NUI_IMAGE_RESOLUTION_320x240 = 1,
                NUI_IMAGE_RESOLUTION_640x480 = 2,
                NUI_IMAGE_RESOLUTION_1280x960 = 3,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for _NUI_IMAGE_RESOLUTION {
                #[inline]
                fn clone(&self) -> _NUI_IMAGE_RESOLUTION {
                    match self {
                        _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_INVALID => {
                            _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_INVALID
                        }
                        _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_80x60 => {
                            _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_80x60
                        }
                        _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_320x240 => {
                            _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_320x240
                        }
                        _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_640x480 => {
                            _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_640x480
                        }
                        _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_1280x960 => {
                            _NUI_IMAGE_RESOLUTION::NUI_IMAGE_RESOLUTION_1280x960
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for _NUI_IMAGE_RESOLUTION {
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_tag, state)
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for _NUI_IMAGE_RESOLUTION {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for _NUI_IMAGE_RESOLUTION {
                #[inline]
                fn eq(&self, other: &_NUI_IMAGE_RESOLUTION) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for _NUI_IMAGE_RESOLUTION {}
            #[automatically_derived]
            impl ::core::cmp::Eq for _NUI_IMAGE_RESOLUTION {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            pub type NUI_IMAGE_FRAME = root::_NUI_IMAGE_FRAME;
            #[repr(C, align(8))]
            pub struct _NUI_IMAGE_FRAME {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 48],
            }
            pub type NUI_IMAGE_VIEW_AREA = root::_NUI_IMAGE_VIEW_AREA;
            #[repr(C, align(4))]
            pub struct _NUI_IMAGE_VIEW_AREA {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 12],
            }
            pub type LONG = autocxx::c_long;
            pub type USHORT = autocxx::c_ushort;
            pub type NUI_SKELETON_FRAME = root::_NUI_SKELETON_FRAME;
            #[repr(C, align(8))]
            pub struct _NUI_SKELETON_FRAME {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 2664],
            }
            pub type NUI_TRANSFORM_SMOOTH_PARAMETERS = root::_NUI_TRANSFORM_SMOOTH_PARAMETERS;
            #[repr(C, align(4))]
            pub struct _NUI_TRANSFORM_SMOOTH_PARAMETERS {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 20],
            }
            pub type BOOL = autocxx::c_int;
            pub type Vector4 = root::_Vector4;
            #[repr(C, align(4))]
            pub struct _Vector4 {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 16],
            }
            pub use cxxbridge::INuiDepthFilter;
            impl INuiSensor {
                pub fn NuiInitialize(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    dwFlags: autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiInitialize_autocxx_wrapper(self, dwFlags)
                }
                pub fn NuiShutdown(self: ::std::pin::Pin<&mut root::INuiSensor>) {
                    cxxbridge::NuiShutdown_autocxx_wrapper(self)
                }
                pub unsafe fn NuiSetFrameEndEvent(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hEvent: *mut autocxx::c_void,
                    dwFrameEventFlag: autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSetFrameEndEvent_autocxx_wrapper(
                        self,
                        hEvent,
                        dwFrameEventFlag,
                    )
                }
                pub unsafe fn NuiImageStreamOpen(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    eImageType: root::_NUI_IMAGE_TYPE,
                    eResolution: root::_NUI_IMAGE_RESOLUTION,
                    dwImageFrameFlags: autocxx::c_ulong,
                    dwFrameLimit: autocxx::c_ulong,
                    hNextFrameEvent: *mut autocxx::c_void,
                    phStreamHandle: *mut *mut autocxx::c_void,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageStreamOpen_autocxx_wrapper(
                        self,
                        eImageType,
                        eResolution,
                        dwImageFrameFlags,
                        dwFrameLimit,
                        hNextFrameEvent,
                        phStreamHandle,
                    )
                }
                pub unsafe fn NuiImageStreamSetImageFrameFlags(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hStream: *mut autocxx::c_void,
                    dwImageFrameFlags: autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageStreamSetImageFrameFlags_autocxx_wrapper(
                        self,
                        hStream,
                        dwImageFrameFlags,
                    )
                }
                pub unsafe fn NuiImageStreamGetImageFrameFlags(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hStream: *mut autocxx::c_void,
                    pdwImageFrameFlags: *mut autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageStreamGetImageFrameFlags_autocxx_wrapper(
                        self,
                        hStream,
                        pdwImageFrameFlags,
                    )
                }
                pub unsafe fn NuiImageStreamGetNextFrame(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hStream: *mut autocxx::c_void,
                    dwMillisecondsToWait: autocxx::c_ulong,
                    pImageFrame: *mut root::_NUI_IMAGE_FRAME,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageStreamGetNextFrame_autocxx_wrapper(
                        self,
                        hStream,
                        dwMillisecondsToWait,
                        pImageFrame,
                    )
                }
                pub unsafe fn NuiImageStreamReleaseFrame(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hStream: *mut autocxx::c_void,
                    pImageFrame: *mut root::_NUI_IMAGE_FRAME,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageStreamReleaseFrame_autocxx_wrapper(
                        self,
                        hStream,
                        pImageFrame,
                    )
                }
                pub unsafe fn NuiImageGetColorPixelCoordinatesFromDepthPixel(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    eColorResolution: root::_NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const root::_NUI_IMAGE_VIEW_AREA,
                    lDepthX: autocxx::c_long,
                    lDepthY: autocxx::c_long,
                    usDepthValue: autocxx::c_ushort,
                    plColorX: *mut autocxx::c_long,
                    plColorY: *mut autocxx::c_long,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageGetColorPixelCoordinatesFromDepthPixel_autocxx_wrapper(
                        self,
                        eColorResolution,
                        pcViewArea,
                        lDepthX,
                        lDepthY,
                        usDepthValue,
                        plColorX,
                        plColorY,
                    )
                }
                pub unsafe fn NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    eColorResolution: root::_NUI_IMAGE_RESOLUTION,
                    eDepthResolution: root::_NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const root::_NUI_IMAGE_VIEW_AREA,
                    lDepthX: autocxx::c_long,
                    lDepthY: autocxx::c_long,
                    usDepthValue: autocxx::c_ushort,
                    plColorX: *mut autocxx::c_long,
                    plColorY: *mut autocxx::c_long,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution_autocxx_wrapper(
                        self,
                        eColorResolution,
                        eDepthResolution,
                        pcViewArea,
                        lDepthX,
                        lDepthY,
                        usDepthValue,
                        plColorX,
                        plColorY,
                    )
                }
                pub unsafe fn NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    eColorResolution: root::_NUI_IMAGE_RESOLUTION,
                    eDepthResolution: root::_NUI_IMAGE_RESOLUTION,
                    cDepthValues: autocxx::c_ulong,
                    pDepthValues: *mut autocxx::c_ushort,
                    cColorCoordinates: autocxx::c_ulong,
                    pColorCoordinates: *mut autocxx::c_long,
                ) -> autocxx::c_long {
                    cxxbridge::NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution_autocxx_wrapper(
                        self,
                        eColorResolution,
                        eDepthResolution,
                        cDepthValues,
                        pDepthValues,
                        cColorCoordinates,
                        pColorCoordinates,
                    )
                }
                pub fn NuiCameraElevationSetAngle(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    lAngleDegrees: autocxx::c_long,
                ) -> autocxx::c_long {
                    cxxbridge::NuiCameraElevationSetAngle_autocxx_wrapper(
                        self,
                        lAngleDegrees,
                    )
                }
                pub unsafe fn NuiCameraElevationGetAngle(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    plAngleDegrees: *mut autocxx::c_long,
                ) -> autocxx::c_long {
                    cxxbridge::NuiCameraElevationGetAngle_autocxx_wrapper(
                        self,
                        plAngleDegrees,
                    )
                }
                pub unsafe fn NuiSkeletonTrackingEnable(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    hNextFrameEvent: *mut autocxx::c_void,
                    dwFlags: autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSkeletonTrackingEnable_autocxx_wrapper(
                        self,
                        hNextFrameEvent,
                        dwFlags,
                    )
                }
                pub fn NuiSkeletonTrackingDisable(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSkeletonTrackingDisable_autocxx_wrapper(self)
                }
                pub unsafe fn NuiSkeletonSetTrackedSkeletons(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    TrackingIDs: *mut autocxx::c_ulong,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSkeletonSetTrackedSkeletons_autocxx_wrapper(
                        self,
                        TrackingIDs,
                    )
                }
                pub unsafe fn NuiSkeletonGetNextFrame(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    dwMillisecondsToWait: autocxx::c_ulong,
                    pSkeletonFrame: *mut root::_NUI_SKELETON_FRAME,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSkeletonGetNextFrame_autocxx_wrapper(
                        self,
                        dwMillisecondsToWait,
                        pSkeletonFrame,
                    )
                }
                pub unsafe fn NuiTransformSmooth(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    pSkeletonFrame: *mut root::_NUI_SKELETON_FRAME,
                    pSmoothingParams: *const root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                ) -> autocxx::c_long {
                    cxxbridge::NuiTransformSmooth_autocxx_wrapper(
                        self,
                        pSkeletonFrame,
                        pSmoothingParams,
                    )
                }
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiGetAudioSource(_uhoh: autocxx::BindingGenerationFailure) {}
                pub fn NuiInstanceIndex(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                ) -> autocxx::c_int {
                    cxxbridge::NuiInstanceIndex_autocxx_wrapper(self)
                }
                ///autocxx bindings couldn't be generated: This item depends on some other type(s) which autocxx could not generate, some of them are: BSTR
                fn NuiDeviceConnectionId_autocxx_wrapper(
                    _uhoh: autocxx::BindingGenerationFailure,
                ) {}
                ///autocxx bindings couldn't be generated: This item depends on some other type(s) which autocxx could not generate, some of them are: BSTR
                fn NuiUniqueId_autocxx_wrapper(
                    _uhoh: autocxx::BindingGenerationFailure,
                ) {}
                ///autocxx bindings couldn't be generated: This item depends on some other type(s) which autocxx could not generate, some of them are: BSTR
                fn NuiAudioArrayId_autocxx_wrapper(
                    _uhoh: autocxx::BindingGenerationFailure,
                ) {}
                pub fn NuiStatus(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                ) -> autocxx::c_long {
                    cxxbridge::NuiStatus_autocxx_wrapper(self)
                }
                pub fn NuiInitializationFlags(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                ) -> autocxx::c_ulong {
                    cxxbridge::NuiInitializationFlags_autocxx_wrapper(self)
                }
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiGetCoordinateMapper(_uhoh: autocxx::BindingGenerationFailure) {}
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiImageFrameGetDepthImagePixelFrameTexture(
                    _uhoh: autocxx::BindingGenerationFailure,
                ) {}
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiGetColorCameraSettings(_uhoh: autocxx::BindingGenerationFailure) {}
                pub fn NuiGetForceInfraredEmitterOff(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                ) -> autocxx::c_int {
                    cxxbridge::NuiGetForceInfraredEmitterOff_autocxx_wrapper(self)
                }
                pub fn NuiSetForceInfraredEmitterOff(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    fForceInfraredEmitterOff: autocxx::c_int,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSetForceInfraredEmitterOff_autocxx_wrapper(
                        self,
                        fForceInfraredEmitterOff,
                    )
                }
                pub unsafe fn NuiAccelerometerGetCurrentReading(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    pReading: *mut root::_Vector4,
                ) -> autocxx::c_long {
                    cxxbridge::NuiAccelerometerGetCurrentReading_autocxx_wrapper(
                        self,
                        pReading,
                    )
                }
                pub unsafe fn NuiSetDepthFilter(
                    self: ::std::pin::Pin<&mut root::INuiSensor>,
                    pDepthFilter: *mut root::INuiDepthFilter,
                ) -> autocxx::c_long {
                    cxxbridge::NuiSetDepthFilter_autocxx_wrapper(self, pDepthFilter)
                }
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiGetDepthFilter(_uhoh: autocxx::BindingGenerationFailure) {}
                ///autocxx bindings couldn't be generated: Pointer pointed to something unsupported
                fn NuiGetDepthFilterForTimeStamp(
                    _uhoh: autocxx::BindingGenerationFailure,
                ) {}
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::INuiSensor {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::INuiSensor {
                    cxxbridge::INuiSensor_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::INuiSensor) {
                    cxxbridge::INuiSensor_free_autocxx_wrapper(arg0)
                }
            }
            impl Drop for root::INuiSensor {
                ///Synthesized destructor.
                fn drop(self: &mut root::INuiSensor) {
                    unsafe {
                        cxxbridge::INuiSensor_synthetic_destructor_0xb6c838ca31d09891_autocxx_wrapper(
                            self,
                        )
                    }
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::_NUI_IMAGE_FRAME {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::_NUI_IMAGE_FRAME {
                    cxxbridge::_NUI_IMAGE_FRAME_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(
                    arg0: *mut root::_NUI_IMAGE_FRAME,
                ) {
                    cxxbridge::_NUI_IMAGE_FRAME_free_autocxx_wrapper(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::_NUI_IMAGE_FRAME {
                ///Synthesized move constructor.
                unsafe fn move_new(
                    mut other: ::std::pin::Pin<
                        autocxx::moveit::MoveRef<'_, root::_NUI_IMAGE_FRAME>,
                    >,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_IMAGE_FRAME>,
                    >,
                ) {
                    cxxbridge::_NUI_IMAGE_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::std::pin::Pin::into_inner_unchecked(
                                other.as_mut(),
                            );
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::_NUI_IMAGE_FRAME {
                ///Synthesized copy constructor.
                unsafe fn copy_new(
                    other: &root::_NUI_IMAGE_FRAME,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_IMAGE_FRAME>,
                    >,
                ) {
                    cxxbridge::_NUI_IMAGE_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::_NUI_IMAGE_VIEW_AREA {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::_NUI_IMAGE_VIEW_AREA {
                    cxxbridge::_NUI_IMAGE_VIEW_AREA_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(
                    arg0: *mut root::_NUI_IMAGE_VIEW_AREA,
                ) {
                    cxxbridge::_NUI_IMAGE_VIEW_AREA_free_autocxx_wrapper(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::_NUI_IMAGE_VIEW_AREA {
                ///Synthesized move constructor.
                unsafe fn move_new(
                    mut other: ::std::pin::Pin<
                        autocxx::moveit::MoveRef<'_, root::_NUI_IMAGE_VIEW_AREA>,
                    >,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_IMAGE_VIEW_AREA>,
                    >,
                ) {
                    cxxbridge::_NUI_IMAGE_VIEW_AREA_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::std::pin::Pin::into_inner_unchecked(
                                other.as_mut(),
                            );
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::_NUI_IMAGE_VIEW_AREA {
                ///Synthesized copy constructor.
                unsafe fn copy_new(
                    other: &root::_NUI_IMAGE_VIEW_AREA,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_IMAGE_VIEW_AREA>,
                    >,
                ) {
                    cxxbridge::_NUI_IMAGE_VIEW_AREA_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::_NUI_SKELETON_FRAME {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::_NUI_SKELETON_FRAME {
                    cxxbridge::_NUI_SKELETON_FRAME_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(
                    arg0: *mut root::_NUI_SKELETON_FRAME,
                ) {
                    cxxbridge::_NUI_SKELETON_FRAME_free_autocxx_wrapper(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::_NUI_SKELETON_FRAME {
                ///Synthesized move constructor.
                unsafe fn move_new(
                    mut other: ::std::pin::Pin<
                        autocxx::moveit::MoveRef<'_, root::_NUI_SKELETON_FRAME>,
                    >,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_SKELETON_FRAME>,
                    >,
                ) {
                    cxxbridge::_NUI_SKELETON_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::std::pin::Pin::into_inner_unchecked(
                                other.as_mut(),
                            );
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::_NUI_SKELETON_FRAME {
                ///Synthesized copy constructor.
                unsafe fn copy_new(
                    other: &root::_NUI_SKELETON_FRAME,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<root::_NUI_SKELETON_FRAME>,
                    >,
                ) {
                    cxxbridge::_NUI_SKELETON_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage
            for root::_NUI_TRANSFORM_SMOOTH_PARAMETERS {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::_NUI_TRANSFORM_SMOOTH_PARAMETERS {
                    cxxbridge::_NUI_TRANSFORM_SMOOTH_PARAMETERS_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(
                    arg0: *mut root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                ) {
                    cxxbridge::_NUI_TRANSFORM_SMOOTH_PARAMETERS_free_autocxx_wrapper(
                        arg0,
                    )
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew
            for root::_NUI_TRANSFORM_SMOOTH_PARAMETERS {
                ///Synthesized move constructor.
                unsafe fn move_new(
                    mut other: ::std::pin::Pin<
                        autocxx::moveit::MoveRef<
                            '_,
                            root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                        >,
                    >,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<
                            root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                        >,
                    >,
                ) {
                    cxxbridge::_NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::std::pin::Pin::into_inner_unchecked(
                                other.as_mut(),
                            );
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew
            for root::_NUI_TRANSFORM_SMOOTH_PARAMETERS {
                ///Synthesized copy constructor.
                unsafe fn copy_new(
                    other: &root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                    this: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<
                            root::_NUI_TRANSFORM_SMOOTH_PARAMETERS,
                        >,
                    >,
                ) {
                    cxxbridge::_NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::_Vector4 {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::_Vector4 {
                    cxxbridge::_Vector4_alloc_autocxx_wrapper()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::_Vector4) {
                    cxxbridge::_Vector4_free_autocxx_wrapper(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::_Vector4 {
                ///Synthesized move constructor.
                unsafe fn move_new(
                    mut other: ::std::pin::Pin<
                        autocxx::moveit::MoveRef<'_, root::_Vector4>,
                    >,
                    this: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<root::_Vector4>>,
                ) {
                    cxxbridge::_Vector4_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::std::pin::Pin::into_inner_unchecked(
                                other.as_mut(),
                            );
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::_Vector4 {
                ///Synthesized copy constructor.
                unsafe fn copy_new(
                    other: &root::_Vector4,
                    this: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<root::_Vector4>>,
                ) {
                    cxxbridge::_Vector4_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            #[allow(unused_imports)]
            use self::super::super::{cxxbridge, ToCppString};
            #[allow(unused_imports)]
            use self::super::root;
        }
    }
    #[deny(improper_ctypes, improper_ctypes_definitions)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
    mod cxxbridge {
        pub fn autocxx_make_string_0xb6c838ca31d09891(
            str_: &str,
        ) -> ::cxx::UniquePtr<::cxx::CxxString> {
            extern "C" {
                #[link_name = "cxxbridge1$autocxx_make_string_0xb6c838ca31d09891"]
                fn __autocxx_make_string_0xb6c838ca31d09891(
                    str_: ::cxx::private::RustStr,
                ) -> *mut ::cxx::CxxString;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(
                    __autocxx_make_string_0xb6c838ca31d09891(
                        ::cxx::private::RustStr::from(str_),
                    ),
                )
            }
        }
        pub unsafe fn INuiSensor_alloc_autocxx_wrapper() -> *mut INuiSensor {
            extern "C" {
                #[link_name = "cxxbridge1$INuiSensor_alloc_autocxx_wrapper"]
                fn __INuiSensor_alloc_autocxx_wrapper() -> *mut INuiSensor;
            }
            __INuiSensor_alloc_autocxx_wrapper()
        }
        pub unsafe fn INuiSensor_free_autocxx_wrapper(arg0: *mut INuiSensor) {
            extern "C" {
                #[link_name = "cxxbridge1$INuiSensor_free_autocxx_wrapper"]
                fn __INuiSensor_free_autocxx_wrapper(arg0: *mut INuiSensor);
            }
            __INuiSensor_free_autocxx_wrapper(arg0)
        }
        #[repr(C)]
        pub struct INuiSensor {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for INuiSensor {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::I,
                ::cxx::N,
                ::cxx::u,
                ::cxx::i,
                ::cxx::S,
                ::cxx::e,
                ::cxx::n,
                ::cxx::s,
                ::cxx::o,
                ::cxx::r,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        pub fn NuiInitialize_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            dwFlags: c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiInitialize_autocxx_wrapper"]
                fn __NuiInitialize_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    dwFlags: *mut c_ulong,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut dwFlags = ::cxx::core::mem::MaybeUninit::new(dwFlags);
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __NuiInitialize_autocxx_wrapper(
                    autocxx_gen_this,
                    dwFlags.as_mut_ptr(),
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub fn NuiShutdown_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$NuiShutdown_autocxx_wrapper"]
                fn __NuiShutdown_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                );
            }
            unsafe { __NuiShutdown_autocxx_wrapper(autocxx_gen_this) }
        }
        pub unsafe fn NuiSetFrameEndEvent_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hEvent: *mut c_void,
            dwFrameEventFlag: c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSetFrameEndEvent_autocxx_wrapper"]
                fn __NuiSetFrameEndEvent_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hEvent: *mut ::cxx::core::ffi::c_void,
                    dwFrameEventFlag: *mut c_ulong,
                    __return: *mut c_long,
                );
            }
            let mut dwFrameEventFlag = ::cxx::core::mem::MaybeUninit::new(
                dwFrameEventFlag,
            );
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiSetFrameEndEvent_autocxx_wrapper(
                autocxx_gen_this,
                hEvent.cast(),
                dwFrameEventFlag.as_mut_ptr(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageStreamOpen_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            eImageType: _NUI_IMAGE_TYPE,
            eResolution: _NUI_IMAGE_RESOLUTION,
            dwImageFrameFlags: c_ulong,
            dwFrameLimit: c_ulong,
            hNextFrameEvent: *mut c_void,
            phStreamHandle: *mut *mut c_void,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageStreamOpen_autocxx_wrapper"]
                fn __NuiImageStreamOpen_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    eImageType: *mut _NUI_IMAGE_TYPE,
                    eResolution: *mut _NUI_IMAGE_RESOLUTION,
                    dwImageFrameFlags: *mut c_ulong,
                    dwFrameLimit: *mut c_ulong,
                    hNextFrameEvent: *mut ::cxx::core::ffi::c_void,
                    phStreamHandle: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut eImageType = ::cxx::core::mem::MaybeUninit::new(eImageType);
            let mut eResolution = ::cxx::core::mem::MaybeUninit::new(eResolution);
            let mut dwImageFrameFlags = ::cxx::core::mem::MaybeUninit::new(
                dwImageFrameFlags,
            );
            let mut dwFrameLimit = ::cxx::core::mem::MaybeUninit::new(dwFrameLimit);
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageStreamOpen_autocxx_wrapper(
                autocxx_gen_this,
                eImageType.as_mut_ptr(),
                eResolution.as_mut_ptr(),
                dwImageFrameFlags.as_mut_ptr(),
                dwFrameLimit.as_mut_ptr(),
                hNextFrameEvent.cast(),
                phStreamHandle.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageStreamSetImageFrameFlags_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hStream: *mut c_void,
            dwImageFrameFlags: c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageStreamSetImageFrameFlags_autocxx_wrapper"]
                fn __NuiImageStreamSetImageFrameFlags_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hStream: *mut ::cxx::core::ffi::c_void,
                    dwImageFrameFlags: *mut c_ulong,
                    __return: *mut c_long,
                );
            }
            let mut dwImageFrameFlags = ::cxx::core::mem::MaybeUninit::new(
                dwImageFrameFlags,
            );
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageStreamSetImageFrameFlags_autocxx_wrapper(
                autocxx_gen_this,
                hStream.cast(),
                dwImageFrameFlags.as_mut_ptr(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageStreamGetImageFrameFlags_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hStream: *mut c_void,
            pdwImageFrameFlags: *mut c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageStreamGetImageFrameFlags_autocxx_wrapper"]
                fn __NuiImageStreamGetImageFrameFlags_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hStream: *mut ::cxx::core::ffi::c_void,
                    pdwImageFrameFlags: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageStreamGetImageFrameFlags_autocxx_wrapper(
                autocxx_gen_this,
                hStream.cast(),
                pdwImageFrameFlags.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageStreamGetNextFrame_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hStream: *mut c_void,
            dwMillisecondsToWait: c_ulong,
            pImageFrame: *mut _NUI_IMAGE_FRAME,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageStreamGetNextFrame_autocxx_wrapper"]
                fn __NuiImageStreamGetNextFrame_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hStream: *mut ::cxx::core::ffi::c_void,
                    dwMillisecondsToWait: *mut c_ulong,
                    pImageFrame: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut dwMillisecondsToWait = ::cxx::core::mem::MaybeUninit::new(
                dwMillisecondsToWait,
            );
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageStreamGetNextFrame_autocxx_wrapper(
                autocxx_gen_this,
                hStream.cast(),
                dwMillisecondsToWait.as_mut_ptr(),
                pImageFrame.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageStreamReleaseFrame_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hStream: *mut c_void,
            pImageFrame: *mut _NUI_IMAGE_FRAME,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageStreamReleaseFrame_autocxx_wrapper"]
                fn __NuiImageStreamReleaseFrame_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hStream: *mut ::cxx::core::ffi::c_void,
                    pImageFrame: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageStreamReleaseFrame_autocxx_wrapper(
                autocxx_gen_this,
                hStream.cast(),
                pImageFrame.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageGetColorPixelCoordinatesFromDepthPixel_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            eColorResolution: _NUI_IMAGE_RESOLUTION,
            pcViewArea: *const _NUI_IMAGE_VIEW_AREA,
            lDepthX: c_long,
            lDepthY: c_long,
            usDepthValue: c_ushort,
            plColorX: *mut c_long,
            plColorY: *mut c_long,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageGetColorPixelCoordinatesFromDepthPixel_autocxx_wrapper"]
                fn __NuiImageGetColorPixelCoordinatesFromDepthPixel_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    eColorResolution: *mut _NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const ::cxx::core::ffi::c_void,
                    lDepthX: *mut c_long,
                    lDepthY: *mut c_long,
                    usDepthValue: *mut c_ushort,
                    plColorX: *mut ::cxx::core::ffi::c_void,
                    plColorY: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut eColorResolution = ::cxx::core::mem::MaybeUninit::new(
                eColorResolution,
            );
            let mut lDepthX = ::cxx::core::mem::MaybeUninit::new(lDepthX);
            let mut lDepthY = ::cxx::core::mem::MaybeUninit::new(lDepthY);
            let mut usDepthValue = ::cxx::core::mem::MaybeUninit::new(usDepthValue);
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageGetColorPixelCoordinatesFromDepthPixel_autocxx_wrapper(
                autocxx_gen_this,
                eColorResolution.as_mut_ptr(),
                pcViewArea.cast(),
                lDepthX.as_mut_ptr(),
                lDepthY.as_mut_ptr(),
                usDepthValue.as_mut_ptr(),
                plColorX.cast(),
                plColorY.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            eColorResolution: _NUI_IMAGE_RESOLUTION,
            eDepthResolution: _NUI_IMAGE_RESOLUTION,
            pcViewArea: *const _NUI_IMAGE_VIEW_AREA,
            lDepthX: c_long,
            lDepthY: c_long,
            usDepthValue: c_ushort,
            plColorX: *mut c_long,
            plColorY: *mut c_long,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution_autocxx_wrapper"]
                fn __NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    eColorResolution: *mut _NUI_IMAGE_RESOLUTION,
                    eDepthResolution: *mut _NUI_IMAGE_RESOLUTION,
                    pcViewArea: *const ::cxx::core::ffi::c_void,
                    lDepthX: *mut c_long,
                    lDepthY: *mut c_long,
                    usDepthValue: *mut c_ushort,
                    plColorX: *mut ::cxx::core::ffi::c_void,
                    plColorY: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut eColorResolution = ::cxx::core::mem::MaybeUninit::new(
                eColorResolution,
            );
            let mut eDepthResolution = ::cxx::core::mem::MaybeUninit::new(
                eDepthResolution,
            );
            let mut lDepthX = ::cxx::core::mem::MaybeUninit::new(lDepthX);
            let mut lDepthY = ::cxx::core::mem::MaybeUninit::new(lDepthY);
            let mut usDepthValue = ::cxx::core::mem::MaybeUninit::new(usDepthValue);
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution_autocxx_wrapper(
                autocxx_gen_this,
                eColorResolution.as_mut_ptr(),
                eDepthResolution.as_mut_ptr(),
                pcViewArea.cast(),
                lDepthX.as_mut_ptr(),
                lDepthY.as_mut_ptr(),
                usDepthValue.as_mut_ptr(),
                plColorX.cast(),
                plColorY.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            eColorResolution: _NUI_IMAGE_RESOLUTION,
            eDepthResolution: _NUI_IMAGE_RESOLUTION,
            cDepthValues: c_ulong,
            pDepthValues: *mut c_ushort,
            cColorCoordinates: c_ulong,
            pColorCoordinates: *mut c_long,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution_autocxx_wrapper"]
                fn __NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    eColorResolution: *mut _NUI_IMAGE_RESOLUTION,
                    eDepthResolution: *mut _NUI_IMAGE_RESOLUTION,
                    cDepthValues: *mut c_ulong,
                    pDepthValues: *mut ::cxx::core::ffi::c_void,
                    cColorCoordinates: *mut c_ulong,
                    pColorCoordinates: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut eColorResolution = ::cxx::core::mem::MaybeUninit::new(
                eColorResolution,
            );
            let mut eDepthResolution = ::cxx::core::mem::MaybeUninit::new(
                eDepthResolution,
            );
            let mut cDepthValues = ::cxx::core::mem::MaybeUninit::new(cDepthValues);
            let mut cColorCoordinates = ::cxx::core::mem::MaybeUninit::new(
                cColorCoordinates,
            );
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution_autocxx_wrapper(
                autocxx_gen_this,
                eColorResolution.as_mut_ptr(),
                eDepthResolution.as_mut_ptr(),
                cDepthValues.as_mut_ptr(),
                pDepthValues.cast(),
                cColorCoordinates.as_mut_ptr(),
                pColorCoordinates.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub fn NuiCameraElevationSetAngle_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            lAngleDegrees: c_long,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiCameraElevationSetAngle_autocxx_wrapper"]
                fn __NuiCameraElevationSetAngle_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    lAngleDegrees: *mut c_long,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut lAngleDegrees = ::cxx::core::mem::MaybeUninit::new(
                    lAngleDegrees,
                );
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __NuiCameraElevationSetAngle_autocxx_wrapper(
                    autocxx_gen_this,
                    lAngleDegrees.as_mut_ptr(),
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub unsafe fn NuiCameraElevationGetAngle_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            plAngleDegrees: *mut c_long,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiCameraElevationGetAngle_autocxx_wrapper"]
                fn __NuiCameraElevationGetAngle_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    plAngleDegrees: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiCameraElevationGetAngle_autocxx_wrapper(
                autocxx_gen_this,
                plAngleDegrees.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiSkeletonTrackingEnable_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            hNextFrameEvent: *mut c_void,
            dwFlags: c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSkeletonTrackingEnable_autocxx_wrapper"]
                fn __NuiSkeletonTrackingEnable_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    hNextFrameEvent: *mut ::cxx::core::ffi::c_void,
                    dwFlags: *mut c_ulong,
                    __return: *mut c_long,
                );
            }
            let mut dwFlags = ::cxx::core::mem::MaybeUninit::new(dwFlags);
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiSkeletonTrackingEnable_autocxx_wrapper(
                autocxx_gen_this,
                hNextFrameEvent.cast(),
                dwFlags.as_mut_ptr(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub fn NuiSkeletonTrackingDisable_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSkeletonTrackingDisable_autocxx_wrapper"]
                fn __NuiSkeletonTrackingDisable_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __NuiSkeletonTrackingDisable_autocxx_wrapper(
                    autocxx_gen_this,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub unsafe fn NuiSkeletonSetTrackedSkeletons_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            TrackingIDs: *mut c_ulong,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSkeletonSetTrackedSkeletons_autocxx_wrapper"]
                fn __NuiSkeletonSetTrackedSkeletons_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    TrackingIDs: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiSkeletonSetTrackedSkeletons_autocxx_wrapper(
                autocxx_gen_this,
                TrackingIDs.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiSkeletonGetNextFrame_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            dwMillisecondsToWait: c_ulong,
            pSkeletonFrame: *mut _NUI_SKELETON_FRAME,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSkeletonGetNextFrame_autocxx_wrapper"]
                fn __NuiSkeletonGetNextFrame_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    dwMillisecondsToWait: *mut c_ulong,
                    pSkeletonFrame: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut dwMillisecondsToWait = ::cxx::core::mem::MaybeUninit::new(
                dwMillisecondsToWait,
            );
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiSkeletonGetNextFrame_autocxx_wrapper(
                autocxx_gen_this,
                dwMillisecondsToWait.as_mut_ptr(),
                pSkeletonFrame.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiTransformSmooth_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            pSkeletonFrame: *mut _NUI_SKELETON_FRAME,
            pSmoothingParams: *const _NUI_TRANSFORM_SMOOTH_PARAMETERS,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiTransformSmooth_autocxx_wrapper"]
                fn __NuiTransformSmooth_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    pSkeletonFrame: *mut ::cxx::core::ffi::c_void,
                    pSmoothingParams: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiTransformSmooth_autocxx_wrapper(
                autocxx_gen_this,
                pSkeletonFrame.cast(),
                pSmoothingParams.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub fn NuiInstanceIndex_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) -> c_int {
            extern "C" {
                #[link_name = "cxxbridge1$NuiInstanceIndex_autocxx_wrapper"]
                fn __NuiInstanceIndex_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    __return: *mut c_int,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_int>::uninit();
                __NuiInstanceIndex_autocxx_wrapper(
                    autocxx_gen_this,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub fn NuiStatus_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiStatus_autocxx_wrapper"]
                fn __NuiStatus_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __NuiStatus_autocxx_wrapper(autocxx_gen_this, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub fn NuiInitializationFlags_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) -> c_ulong {
            extern "C" {
                #[link_name = "cxxbridge1$NuiInitializationFlags_autocxx_wrapper"]
                fn __NuiInitializationFlags_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    __return: *mut c_ulong,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_ulong>::uninit();
                __NuiInitializationFlags_autocxx_wrapper(
                    autocxx_gen_this,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub fn NuiGetForceInfraredEmitterOff_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
        ) -> c_int {
            extern "C" {
                #[link_name = "cxxbridge1$NuiGetForceInfraredEmitterOff_autocxx_wrapper"]
                fn __NuiGetForceInfraredEmitterOff_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    __return: *mut c_int,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_int>::uninit();
                __NuiGetForceInfraredEmitterOff_autocxx_wrapper(
                    autocxx_gen_this,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub fn NuiSetForceInfraredEmitterOff_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            fForceInfraredEmitterOff: c_int,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSetForceInfraredEmitterOff_autocxx_wrapper"]
                fn __NuiSetForceInfraredEmitterOff_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    fForceInfraredEmitterOff: *mut c_int,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut fForceInfraredEmitterOff = ::cxx::core::mem::MaybeUninit::new(
                    fForceInfraredEmitterOff,
                );
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __NuiSetForceInfraredEmitterOff_autocxx_wrapper(
                    autocxx_gen_this,
                    fForceInfraredEmitterOff.as_mut_ptr(),
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        pub unsafe fn NuiAccelerometerGetCurrentReading_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            pReading: *mut _Vector4,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiAccelerometerGetCurrentReading_autocxx_wrapper"]
                fn __NuiAccelerometerGetCurrentReading_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    pReading: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiAccelerometerGetCurrentReading_autocxx_wrapper(
                autocxx_gen_this,
                pReading.cast(),
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        pub unsafe fn NuiSetDepthFilter_autocxx_wrapper(
            autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
            pDepthFilter: *mut INuiDepthFilter,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiSetDepthFilter_autocxx_wrapper"]
                fn __NuiSetDepthFilter_autocxx_wrapper(
                    autocxx_gen_this: ::cxx::core::pin::Pin<&mut INuiSensor>,
                    pDepthFilter: *mut INuiDepthFilter,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiSetDepthFilter_autocxx_wrapper(
                autocxx_gen_this,
                pDepthFilter,
                __return.as_mut_ptr(),
            );
            __return.assume_init()
        }
        /// <summary>
        /// Returns the number of Kinect sensors that are connected to the computer.
        /// </summary>
        /// <param name="pCount">Pointer to an integer which receives the number of Kinect sensors.</param>
        /// <returns>
        /// <para>Returns S_OK if successful; otherwise, returns one of the following failure codes:
        /// <list type="table">
        ///    <listheader>
        ///       <term>Error code</term>
        ///       <description>Description</description>
        ///    </listheader>
        ///    <item>
        ///       <term>E_POINTER</term>
        ///       <description>The <paramref name="pCount"/> parameter is NULL.</description>
        ///    </item>
        /// </list>
        /// </returns>
        pub unsafe fn NuiGetSensorCount(pCount: *mut c_int) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$NuiGetSensorCount"]
                fn __NuiGetSensorCount(
                    pCount: *mut ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
            __NuiGetSensorCount(pCount.cast(), __return.as_mut_ptr());
            __return.assume_init()
        }
        ///Synthesized destructor.
        pub unsafe fn INuiSensor_synthetic_destructor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut INuiSensor,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$INuiSensor_synthetic_destructor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn __INuiSensor_synthetic_destructor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut INuiSensor,
                );
            }
            __INuiSensor_synthetic_destructor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this,
            )
        }
        pub type _NUI_IMAGE_TYPE = super::bindgen::root::_NUI_IMAGE_TYPE;
        pub type _NUI_IMAGE_RESOLUTION = super::bindgen::root::_NUI_IMAGE_RESOLUTION;
        pub type _NUI_IMAGE_FRAME = super::bindgen::root::_NUI_IMAGE_FRAME;
        pub type _NUI_IMAGE_VIEW_AREA = super::bindgen::root::_NUI_IMAGE_VIEW_AREA;
        pub type _NUI_SKELETON_FRAME = super::bindgen::root::_NUI_SKELETON_FRAME;
        pub type _NUI_TRANSFORM_SMOOTH_PARAMETERS = super::bindgen::root::_NUI_TRANSFORM_SMOOTH_PARAMETERS;
        pub type _Vector4 = super::bindgen::root::_Vector4;
        #[repr(C)]
        pub struct INuiDepthFilter {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for INuiDepthFilter {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::I,
                ::cxx::N,
                ::cxx::u,
                ::cxx::i,
                ::cxx::D,
                ::cxx::e,
                ::cxx::p,
                ::cxx::t,
                ::cxx::h,
                ::cxx::F,
                ::cxx::i,
                ::cxx::l,
                ::cxx::t,
                ::cxx::e,
                ::cxx::r,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        pub unsafe fn _NUI_IMAGE_FRAME_alloc_autocxx_wrapper() -> *mut _NUI_IMAGE_FRAME {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_FRAME_alloc_autocxx_wrapper"]
                fn ___NUI_IMAGE_FRAME_alloc_autocxx_wrapper() -> *mut ::cxx::core::ffi::c_void;
            }
            ___NUI_IMAGE_FRAME_alloc_autocxx_wrapper().cast()
        }
        pub unsafe fn _NUI_IMAGE_FRAME_free_autocxx_wrapper(
            arg0: *mut _NUI_IMAGE_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_FRAME_free_autocxx_wrapper"]
                fn ___NUI_IMAGE_FRAME_free_autocxx_wrapper(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_FRAME_free_autocxx_wrapper(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn _NUI_IMAGE_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_IMAGE_FRAME,
            other: *mut _NUI_IMAGE_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_IMAGE_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        ///Synthesized copy constructor.
        pub unsafe fn _NUI_IMAGE_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_IMAGE_FRAME,
            other: &_NUI_IMAGE_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_IMAGE_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other as *const _NUI_IMAGE_FRAME as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn _NUI_IMAGE_VIEW_AREA_alloc_autocxx_wrapper() -> *mut _NUI_IMAGE_VIEW_AREA {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_VIEW_AREA_alloc_autocxx_wrapper"]
                fn ___NUI_IMAGE_VIEW_AREA_alloc_autocxx_wrapper() -> *mut ::cxx::core::ffi::c_void;
            }
            ___NUI_IMAGE_VIEW_AREA_alloc_autocxx_wrapper().cast()
        }
        pub unsafe fn _NUI_IMAGE_VIEW_AREA_free_autocxx_wrapper(
            arg0: *mut _NUI_IMAGE_VIEW_AREA,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_VIEW_AREA_free_autocxx_wrapper"]
                fn ___NUI_IMAGE_VIEW_AREA_free_autocxx_wrapper(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_VIEW_AREA_free_autocxx_wrapper(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn _NUI_IMAGE_VIEW_AREA_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_IMAGE_VIEW_AREA,
            other: *mut _NUI_IMAGE_VIEW_AREA,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_VIEW_AREA_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_IMAGE_VIEW_AREA_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_VIEW_AREA_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        ///Synthesized copy constructor.
        pub unsafe fn _NUI_IMAGE_VIEW_AREA_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_IMAGE_VIEW_AREA,
            other: &_NUI_IMAGE_VIEW_AREA,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_IMAGE_VIEW_AREA_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_IMAGE_VIEW_AREA_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_IMAGE_VIEW_AREA_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other as *const _NUI_IMAGE_VIEW_AREA as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn _NUI_SKELETON_FRAME_alloc_autocxx_wrapper() -> *mut _NUI_SKELETON_FRAME {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_SKELETON_FRAME_alloc_autocxx_wrapper"]
                fn ___NUI_SKELETON_FRAME_alloc_autocxx_wrapper() -> *mut ::cxx::core::ffi::c_void;
            }
            ___NUI_SKELETON_FRAME_alloc_autocxx_wrapper().cast()
        }
        pub unsafe fn _NUI_SKELETON_FRAME_free_autocxx_wrapper(
            arg0: *mut _NUI_SKELETON_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_SKELETON_FRAME_free_autocxx_wrapper"]
                fn ___NUI_SKELETON_FRAME_free_autocxx_wrapper(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_SKELETON_FRAME_free_autocxx_wrapper(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn _NUI_SKELETON_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_SKELETON_FRAME,
            other: *mut _NUI_SKELETON_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_SKELETON_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_SKELETON_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_SKELETON_FRAME_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        ///Synthesized copy constructor.
        pub unsafe fn _NUI_SKELETON_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_SKELETON_FRAME,
            other: &_NUI_SKELETON_FRAME,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_SKELETON_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_SKELETON_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_SKELETON_FRAME_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other as *const _NUI_SKELETON_FRAME as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn _NUI_TRANSFORM_SMOOTH_PARAMETERS_alloc_autocxx_wrapper() -> *mut _NUI_TRANSFORM_SMOOTH_PARAMETERS {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_TRANSFORM_SMOOTH_PARAMETERS_alloc_autocxx_wrapper"]
                fn ___NUI_TRANSFORM_SMOOTH_PARAMETERS_alloc_autocxx_wrapper() -> *mut ::cxx::core::ffi::c_void;
            }
            ___NUI_TRANSFORM_SMOOTH_PARAMETERS_alloc_autocxx_wrapper().cast()
        }
        pub unsafe fn _NUI_TRANSFORM_SMOOTH_PARAMETERS_free_autocxx_wrapper(
            arg0: *mut _NUI_TRANSFORM_SMOOTH_PARAMETERS,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_TRANSFORM_SMOOTH_PARAMETERS_free_autocxx_wrapper"]
                fn ___NUI_TRANSFORM_SMOOTH_PARAMETERS_free_autocxx_wrapper(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_TRANSFORM_SMOOTH_PARAMETERS_free_autocxx_wrapper(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn _NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_TRANSFORM_SMOOTH_PARAMETERS,
            other: *mut _NUI_TRANSFORM_SMOOTH_PARAMETERS,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        ///Synthesized copy constructor.
        pub unsafe fn _NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _NUI_TRANSFORM_SMOOTH_PARAMETERS,
            other: &_NUI_TRANSFORM_SMOOTH_PARAMETERS,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            ___NUI_TRANSFORM_SMOOTH_PARAMETERS_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other as *const _NUI_TRANSFORM_SMOOTH_PARAMETERS
                    as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn _Vector4_alloc_autocxx_wrapper() -> *mut _Vector4 {
            extern "C" {
                #[link_name = "cxxbridge1$_Vector4_alloc_autocxx_wrapper"]
                fn ___Vector4_alloc_autocxx_wrapper() -> *mut ::cxx::core::ffi::c_void;
            }
            ___Vector4_alloc_autocxx_wrapper().cast()
        }
        pub unsafe fn _Vector4_free_autocxx_wrapper(arg0: *mut _Vector4) {
            extern "C" {
                #[link_name = "cxxbridge1$_Vector4_free_autocxx_wrapper"]
                fn ___Vector4_free_autocxx_wrapper(arg0: *mut ::cxx::core::ffi::c_void);
            }
            ___Vector4_free_autocxx_wrapper(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn _Vector4_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _Vector4,
            other: *mut _Vector4,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_Vector4_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___Vector4_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            ___Vector4_new_synthetic_move_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        ///Synthesized copy constructor.
        pub unsafe fn _Vector4_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
            autocxx_gen_this: *mut _Vector4,
            other: &_Vector4,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$_Vector4_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper"]
                fn ___Vector4_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            ___Vector4_new_synthetic_const_copy_ctor_0xb6c838ca31d09891_autocxx_wrapper(
                autocxx_gen_this.cast(),
                other as *const _Vector4 as *const ::cxx::core::ffi::c_void,
            )
        }
        pub type c_ulong = autocxx::c_ulong;
        pub type c_long = autocxx::c_long;
        pub type c_ushort = autocxx::c_ushort;
        pub type c_int = autocxx::c_int;
        pub type c_void = autocxx::c_void;
        unsafe impl ::cxx::private::UniquePtrTarget for INuiSensor {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiSensor")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiSensor$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiSensor$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiSensor$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiSensor$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiSensor$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for INuiSensor {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiSensor")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiSensor$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiSensor$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiSensor$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiSensor$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for INuiSensor {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiSensor")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiSensor$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiSensor$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiSensor$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiSensor$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiSensor$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for INuiSensor {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiSensor")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$INuiSensor$size"]
                    fn __vector_size(_: &::cxx::CxxVector<INuiSensor>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$INuiSensor$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<INuiSensor>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiSensor$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiSensor$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<INuiSensor>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiSensor$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<INuiSensor>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiSensor$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<INuiSensor>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiSensor$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _NUI_IMAGE_TYPE {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_TYPE")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<_NUI_IMAGE_TYPE>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_TYPE$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _NUI_IMAGE_TYPE {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_TYPE")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_TYPE$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_TYPE$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_IMAGE_TYPE>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_TYPE$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_TYPE$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_TYPE$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_IMAGE_TYPE {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_TYPE")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_TYPE$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_TYPE$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_TYPE$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_TYPE$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_TYPE$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_IMAGE_TYPE {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_TYPE")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_TYPE$size"]
                    fn __vector_size(_: &::cxx::CxxVector<_NUI_IMAGE_TYPE>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_TYPE$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_IMAGE_TYPE>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_TYPE$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_TYPE>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_TYPE$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_TYPE>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_TYPE$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_TYPE$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_IMAGE_TYPE>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_TYPE$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_IMAGE_TYPE>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_TYPE$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_IMAGE_TYPE>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_TYPE$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _NUI_IMAGE_RESOLUTION {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_RESOLUTION")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __uninit(&mut repr).cast::<_NUI_IMAGE_RESOLUTION>().write(value)
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_RESOLUTION$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _NUI_IMAGE_RESOLUTION {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_RESOLUTION")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_RESOLUTION$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_RESOLUTION$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_IMAGE_RESOLUTION>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_RESOLUTION$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_RESOLUTION$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_RESOLUTION$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_IMAGE_RESOLUTION {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_RESOLUTION")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_RESOLUTION$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_RESOLUTION$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_RESOLUTION$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_RESOLUTION$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_RESOLUTION$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_IMAGE_RESOLUTION {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_RESOLUTION")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_RESOLUTION$size"]
                    fn __vector_size(
                        _: &::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>,
                    ) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_RESOLUTION$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_RESOLUTION$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_RESOLUTION$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_RESOLUTION$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_RESOLUTION$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_RESOLUTION$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_RESOLUTION$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_IMAGE_RESOLUTION>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_RESOLUTION$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _NUI_IMAGE_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_FRAME")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<_NUI_IMAGE_FRAME>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_FRAME$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _NUI_IMAGE_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_FRAME")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_FRAME$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_FRAME$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_IMAGE_FRAME>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_FRAME$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_FRAME$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_FRAME$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_IMAGE_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_FRAME")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_FRAME$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_FRAME$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_FRAME$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_FRAME$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_FRAME$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_IMAGE_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_FRAME")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_FRAME$size"]
                    fn __vector_size(_: &::cxx::CxxVector<_NUI_IMAGE_FRAME>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_FRAME$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_IMAGE_FRAME>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_FRAME$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_FRAME>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_FRAME$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_FRAME>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_FRAME$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_FRAME$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_IMAGE_FRAME>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_FRAME$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_IMAGE_FRAME>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_FRAME$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_IMAGE_FRAME>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_FRAME$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _NUI_IMAGE_VIEW_AREA {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_VIEW_AREA")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __uninit(&mut repr).cast::<_NUI_IMAGE_VIEW_AREA>().write(value)
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_IMAGE_VIEW_AREA$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _NUI_IMAGE_VIEW_AREA {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_VIEW_AREA")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_VIEW_AREA$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_VIEW_AREA$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_IMAGE_VIEW_AREA>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_VIEW_AREA$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_VIEW_AREA$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_IMAGE_VIEW_AREA$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_IMAGE_VIEW_AREA {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_VIEW_AREA")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_VIEW_AREA$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_VIEW_AREA$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_VIEW_AREA$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_VIEW_AREA$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_IMAGE_VIEW_AREA$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_IMAGE_VIEW_AREA {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_IMAGE_VIEW_AREA")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_VIEW_AREA$size"]
                    fn __vector_size(
                        _: &::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>,
                    ) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_VIEW_AREA$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_VIEW_AREA$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_IMAGE_VIEW_AREA$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_VIEW_AREA$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_VIEW_AREA$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_VIEW_AREA$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_VIEW_AREA$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_IMAGE_VIEW_AREA>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_IMAGE_VIEW_AREA$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _NUI_SKELETON_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_SKELETON_FRAME")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<_NUI_SKELETON_FRAME>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_SKELETON_FRAME$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _NUI_SKELETON_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_SKELETON_FRAME")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_SKELETON_FRAME$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_SKELETON_FRAME$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_SKELETON_FRAME>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_SKELETON_FRAME$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_SKELETON_FRAME$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_SKELETON_FRAME$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_SKELETON_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_SKELETON_FRAME")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_SKELETON_FRAME$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_SKELETON_FRAME$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_SKELETON_FRAME$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_SKELETON_FRAME$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_SKELETON_FRAME$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_SKELETON_FRAME {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_SKELETON_FRAME")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_SKELETON_FRAME$size"]
                    fn __vector_size(_: &::cxx::CxxVector<_NUI_SKELETON_FRAME>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_SKELETON_FRAME$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_SKELETON_FRAME>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_SKELETON_FRAME$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_SKELETON_FRAME>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_SKELETON_FRAME$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_SKELETON_FRAME>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_SKELETON_FRAME$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_SKELETON_FRAME$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_SKELETON_FRAME>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_SKELETON_FRAME$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_SKELETON_FRAME>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_SKELETON_FRAME$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_SKELETON_FRAME>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_SKELETON_FRAME$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget
        for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_TRANSFORM_SMOOTH_PARAMETERS")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __uninit(&mut repr)
                        .cast::<_NUI_TRANSFORM_SMOOTH_PARAMETERS>()
                        .write(value)
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget
        for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_TRANSFORM_SMOOTH_PARAMETERS")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_NUI_TRANSFORM_SMOOTH_PARAMETERS>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_TRANSFORM_SMOOTH_PARAMETERS")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_NUI_TRANSFORM_SMOOTH_PARAMETERS$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _NUI_TRANSFORM_SMOOTH_PARAMETERS {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_NUI_TRANSFORM_SMOOTH_PARAMETERS")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$size"]
                    fn __vector_size(
                        _: &::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>,
                    ) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>,
                        >,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<
                            &mut ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>,
                        >,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_NUI_TRANSFORM_SMOOTH_PARAMETERS>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_NUI_TRANSFORM_SMOOTH_PARAMETERS$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for _Vector4 {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_Vector4")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(
                value: Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<_Vector4>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$_Vector4$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for _Vector4 {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_Vector4")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_Vector4$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_Vector4$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<_Vector4>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_Vector4$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_Vector4$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$_Vector4$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for _Vector4 {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_Vector4")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_Vector4$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_Vector4$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_Vector4$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_Vector4$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$_Vector4$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for _Vector4 {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("_Vector4")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_Vector4$size"]
                    fn __vector_size(_: &::cxx::CxxVector<_Vector4>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_Vector4$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<_Vector4>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_Vector4$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<_Vector4>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$_Vector4$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<_Vector4>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_Vector4$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_Vector4$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<_Vector4>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_Vector4$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<_Vector4>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_Vector4$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<_Vector4>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$_Vector4$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for INuiDepthFilter {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiDepthFilter")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiDepthFilter$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiDepthFilter$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiDepthFilter$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiDepthFilter$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$INuiDepthFilter$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for INuiDepthFilter {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiDepthFilter")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiDepthFilter$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiDepthFilter$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiDepthFilter$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$INuiDepthFilter$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for INuiDepthFilter {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiDepthFilter")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiDepthFilter$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiDepthFilter$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiDepthFilter$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiDepthFilter$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$INuiDepthFilter$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for INuiDepthFilter {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("INuiDepthFilter")
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$INuiDepthFilter$size"]
                    fn __vector_size(_: &::cxx::CxxVector<INuiDepthFilter>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(
                v: *mut ::cxx::CxxVector<Self>,
                pos: usize,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$INuiDepthFilter$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<INuiDepthFilter>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<
                *mut ::cxx::core::ffi::c_void,
            > {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiDepthFilter$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiDepthFilter$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::CxxVector<INuiDepthFilter>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiDepthFilter$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::CxxVector<INuiDepthFilter>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiDepthFilter$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::CxxVector<INuiDepthFilter>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$INuiDepthFilter$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        #[doc(hidden)]
        const _: () = {
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <INuiSensor as __AmbiguousIfImpl<_>>::infer
            };
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_IMAGE_TYPE,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::I,
                    ::cxx::M,
                    ::cxx::A,
                    ::cxx::G,
                    ::cxx::E,
                    ::cxx::__,
                    ::cxx::T,
                    ::cxx::Y,
                    ::cxx::P,
                    ::cxx::E,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                _NUI_IMAGE_TYPE,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_IMAGE_RESOLUTION,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::I,
                    ::cxx::M,
                    ::cxx::A,
                    ::cxx::G,
                    ::cxx::E,
                    ::cxx::__,
                    ::cxx::R,
                    ::cxx::E,
                    ::cxx::S,
                    ::cxx::O,
                    ::cxx::L,
                    ::cxx::U,
                    ::cxx::T,
                    ::cxx::I,
                    ::cxx::O,
                    ::cxx::N,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                _NUI_IMAGE_RESOLUTION,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_IMAGE_FRAME,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::I,
                    ::cxx::M,
                    ::cxx::A,
                    ::cxx::G,
                    ::cxx::E,
                    ::cxx::__,
                    ::cxx::F,
                    ::cxx::R,
                    ::cxx::A,
                    ::cxx::M,
                    ::cxx::E,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_IMAGE_VIEW_AREA,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::I,
                    ::cxx::M,
                    ::cxx::A,
                    ::cxx::G,
                    ::cxx::E,
                    ::cxx::__,
                    ::cxx::V,
                    ::cxx::I,
                    ::cxx::E,
                    ::cxx::W,
                    ::cxx::__,
                    ::cxx::A,
                    ::cxx::R,
                    ::cxx::E,
                    ::cxx::A,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_SKELETON_FRAME,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::S,
                    ::cxx::K,
                    ::cxx::E,
                    ::cxx::L,
                    ::cxx::E,
                    ::cxx::T,
                    ::cxx::O,
                    ::cxx::N,
                    ::cxx::__,
                    ::cxx::F,
                    ::cxx::R,
                    ::cxx::A,
                    ::cxx::M,
                    ::cxx::E,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _NUI_TRANSFORM_SMOOTH_PARAMETERS,
                (
                    ::cxx::__,
                    ::cxx::N,
                    ::cxx::U,
                    ::cxx::I,
                    ::cxx::__,
                    ::cxx::T,
                    ::cxx::R,
                    ::cxx::A,
                    ::cxx::N,
                    ::cxx::S,
                    ::cxx::F,
                    ::cxx::O,
                    ::cxx::R,
                    ::cxx::M,
                    ::cxx::__,
                    ::cxx::S,
                    ::cxx::M,
                    ::cxx::O,
                    ::cxx::O,
                    ::cxx::T,
                    ::cxx::H,
                    ::cxx::__,
                    ::cxx::P,
                    ::cxx::A,
                    ::cxx::R,
                    ::cxx::A,
                    ::cxx::M,
                    ::cxx::E,
                    ::cxx::T,
                    ::cxx::E,
                    ::cxx::R,
                    ::cxx::S,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                _Vector4,
                (
                    ::cxx::__,
                    ::cxx::V,
                    ::cxx::e,
                    ::cxx::c,
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::_4,
                ),
            >;
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <INuiDepthFilter as __AmbiguousIfImpl<_>>::infer
            };
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_ulong,
                (::cxx::c, ::cxx::__, ::cxx::u, ::cxx::l, ::cxx::o, ::cxx::n, ::cxx::g),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                c_ulong,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_long,
                (::cxx::c, ::cxx::__, ::cxx::l, ::cxx::o, ::cxx::n, ::cxx::g),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                c_long,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_ushort,
                (
                    ::cxx::c,
                    ::cxx::__,
                    ::cxx::u,
                    ::cxx::s,
                    ::cxx::h,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::t,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                c_ushort,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_int,
                (::cxx::c, ::cxx::__, ::cxx::i, ::cxx::n, ::cxx::t),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<
                c_int,
                ::cxx::kind::Trivial,
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_void,
                (::cxx::c, ::cxx::__, ::cxx::v, ::cxx::o, ::cxx::i, ::cxx::d),
            >;
        };
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use cxxbridge::autocxx_make_string_0xb6c838ca31d09891 as make_string;
    pub use bindgen::root::INuiSensor;
    pub use cxxbridge::NuiGetSensorCount;
    pub use bindgen::root::NuiCreateSensorByIndex;
    pub use bindgen::root::DWORD;
    pub use bindgen::root::HRESULT;
    pub use bindgen::root::HANDLE;
    pub use bindgen::root::NUI_IMAGE_TYPE;
    pub use bindgen::root::_NUI_IMAGE_TYPE;
    pub use bindgen::root::NUI_IMAGE_RESOLUTION;
    pub use bindgen::root::_NUI_IMAGE_RESOLUTION;
    pub use bindgen::root::NUI_IMAGE_FRAME;
    pub use bindgen::root::_NUI_IMAGE_FRAME;
    pub use bindgen::root::NUI_IMAGE_VIEW_AREA;
    pub use bindgen::root::_NUI_IMAGE_VIEW_AREA;
    pub use bindgen::root::LONG;
    pub use bindgen::root::USHORT;
    pub use bindgen::root::NUI_SKELETON_FRAME;
    pub use bindgen::root::_NUI_SKELETON_FRAME;
    pub use bindgen::root::NUI_TRANSFORM_SMOOTH_PARAMETERS;
    pub use bindgen::root::_NUI_TRANSFORM_SMOOTH_PARAMETERS;
    pub use bindgen::root::BOOL;
    pub use bindgen::root::Vector4;
    pub use bindgen::root::_Vector4;
    pub use bindgen::root::INuiDepthFilter;
}
