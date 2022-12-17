// nui_sensor_interface.h
#pragma once
#ifndef kinect_helper_no_forward_decl
#include "forward_declarations.h"
#include "constants.h"
#include "system_types.h"
#endif

#define DECLARE_VTABLE_METHOD(return_type, class_name, method_name) \
    return_type c_ ## class_name ## method_name



ULONG c_INuiSensor_AddRef(
    INuiSensor * This);

ULONG c_INuiSensor_Release(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiInitialize(
    INuiSensor * This,
    /* [in] */ DWORD dwFlags);

void c_INuiSensor_NuiShutdown(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiSetFrameEndEvent(
    INuiSensor * This,
    /* [in] */ HANDLE hEvent,
    /* [in] */ DWORD dwFrameEventFlag);

HRESULT c_INuiSensor_NuiImageStreamOpen(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_TYPE eImageType,
    /* [in] */ NUI_IMAGE_RESOLUTION eResolution,
    /* [in] */ DWORD dwImageFrameFlags,
    /* [in] */ DWORD dwFrameLimit,
    /* [in] */ HANDLE hNextFrameEvent,
    /* [out] */ HANDLE *phStreamHandle);

HRESULT c_INuiSensor_NuiImageStreamSetImageFrameFlags(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ DWORD dwImageFrameFlags);

HRESULT c_INuiSensor_NuiImageStreamGetImageFrameFlags(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [retval][out] */ DWORD *pdwImageFrameFlags);

HRESULT c_INuiSensor_NuiImageStreamGetNextFrame(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ DWORD dwMillisecondsToWait,
    /* [retval][out] */ NUI_IMAGE_FRAME *pImageFrame);

HRESULT c_INuiSensor_NuiImageStreamReleaseFrame(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ NUI_IMAGE_FRAME *pImageFrame);

HRESULT c_INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixel(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ const NUI_IMAGE_VIEW_AREA *pcViewArea,
    /* [in] */ LONG lDepthX,
    /* [in] */ LONG lDepthY,
    /* [in] */ USHORT usDepthValue,
    /* [out] */ LONG *plColorX,
    /* [out] */ LONG *plColorY);

HRESULT c_INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ NUI_IMAGE_RESOLUTION eDepthResolution,
    /* [in] */ const NUI_IMAGE_VIEW_AREA *pcViewArea,
    /* [in] */ LONG lDepthX,
    /* [in] */ LONG lDepthY,
    /* [in] */ USHORT usDepthValue,
    /* [out] */ LONG *plColorX,
    /* [out] */ LONG *plColorY);

HRESULT c_INuiSensor_NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ NUI_IMAGE_RESOLUTION eDepthResolution,
    /* [in] */ DWORD cDepthValues,
    /* [size_is][in] */ USHORT *pDepthValues,
    /* [in] */ DWORD cColorCoordinates,
    /* [size_is][out][in] */ LONG *pColorCoordinates);

HRESULT c_INuiSensor_NuiCameraElevationSetAngle(
    INuiSensor * This,
    /* [in] */ LONG lAngleDegrees);

HRESULT c_INuiSensor_NuiCameraElevationGetAngle(
    INuiSensor * This,
    /* [retval][out] */ LONG *plAngleDegrees);

HRESULT c_INuiSensor_NuiSkeletonTrackingEnable(
    INuiSensor * This,
    /* [in] */ HANDLE hNextFrameEvent,
    /* [in] */ DWORD dwFlags);

HRESULT c_INuiSensor_NuiSkeletonTrackingDisable(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiSkeletonSetTrackedSkeletons(
    INuiSensor * This,
    /* [size_is][in] */ DWORD *TrackingIDs);

HRESULT c_INuiSensor_NuiSkeletonGetNextFrame(
    INuiSensor * This,
    /* [in] */ DWORD dwMillisecondsToWait,
    /* [out][in] */ NUI_SKELETON_FRAME *pSkeletonFrame);

HRESULT c_INuiSensor_NuiTransformSmooth(
    INuiSensor * This,
    NUI_SKELETON_FRAME *pSkeletonFrame,
    const NUI_TRANSFORM_SMOOTH_PARAMETERS *pSmoothingParams);

HRESULT c_INuiSensor_NuiGetAudioSource(
    INuiSensor * This,
    /* [out] */ INuiAudioBeam **ppDmo);

int c_INuiSensor_NuiInstanceIndex(
    INuiSensor * This);

BSTR c_INuiSensor_NuiDeviceConnectionId(
    INuiSensor * This);

BSTR c_INuiSensor_NuiUniqueId(
    INuiSensor * This);

BSTR c_INuiSensor_NuiAudioArrayId(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiStatus(
    INuiSensor * This);

DWORD c_INuiSensor_NuiInitializationFlags(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiGetCoordinateMapper(
    INuiSensor * This,
    /* [retval][out] */ INuiCoordinateMapper **pMapping);

HRESULT c_INuiSensor_NuiImageFrameGetDepthImagePixelFrameTexture(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ NUI_IMAGE_FRAME *pImageFrame,
    /* [out] */ BOOL *pNearMode,
    /* [out] */ INuiFrameTexture **ppFrameTexture);

HRESULT c_INuiSensor_NuiGetColorCameraSettings(
    INuiSensor * This,
    /* [retval][out] */ INuiColorCameraSettings **pCameraSettings);

BOOL c_INuiSensor_NuiGetForceInfraredEmitterOff(
    INuiSensor * This);

HRESULT c_INuiSensor_NuiSetForceInfraredEmitterOff(
    INuiSensor * This,
    /* [in] */ BOOL fForceInfraredEmitterOff);

HRESULT c_INuiSensor_NuiAccelerometerGetCurrentReading(
    INuiSensor * This,
    /* [retval][out] */ Vector4 *pReading);

HRESULT c_INuiSensor_NuiSetDepthFilter(
    INuiSensor * This,
    /* [in] */ INuiDepthFilter *pDepthFilter);

HRESULT c_INuiSensor_NuiGetDepthFilter(
    INuiSensor * This,
    /* [retval][out] */ INuiDepthFilter **ppDepthFilter);

HRESULT c_INuiSensor_NuiGetDepthFilterForTimeStamp(
    INuiSensor * This,
    /* [in] */ LARGE_INTEGER liTimeStamp,
    /* [retval][out] */ INuiDepthFilter **ppDepthFilter);
