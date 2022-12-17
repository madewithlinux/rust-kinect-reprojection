#include "nui_sensor_interface.h"

#define DEFINE_VTABLE_METHOD(return_type, class_name, method_name) \
    return_type c_ ## class_name ## method_name



#define INuiSensor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define INuiSensor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define INuiSensor_NuiInitialize(This,dwFlags)	\
    ( (This)->lpVtbl -> NuiInitialize(This,dwFlags) )

#define INuiSensor_NuiShutdown(This)	\
    ( (This)->lpVtbl -> NuiShutdown(This) )

#define INuiSensor_NuiSetFrameEndEvent(This,hEvent,dwFrameEventFlag)	\
    ( (This)->lpVtbl -> NuiSetFrameEndEvent(This,hEvent,dwFrameEventFlag) )

#define INuiSensor_NuiImageStreamOpen(This,eImageType,eResolution,dwImageFrameFlags,dwFrameLimit,hNextFrameEvent,phStreamHandle)	\
    ( (This)->lpVtbl -> NuiImageStreamOpen(This,eImageType,eResolution,dwImageFrameFlags,dwFrameLimit,hNextFrameEvent,phStreamHandle) )

#define INuiSensor_NuiImageStreamSetImageFrameFlags(This,hStream,dwImageFrameFlags)	\
    ( (This)->lpVtbl -> NuiImageStreamSetImageFrameFlags(This,hStream,dwImageFrameFlags) )

#define INuiSensor_NuiImageStreamGetImageFrameFlags(This,hStream,pdwImageFrameFlags)	\
    ( (This)->lpVtbl -> NuiImageStreamGetImageFrameFlags(This,hStream,pdwImageFrameFlags) )

#define INuiSensor_NuiImageStreamGetNextFrame(This,hStream,dwMillisecondsToWait,pImageFrame)	\
    ( (This)->lpVtbl -> NuiImageStreamGetNextFrame(This,hStream,dwMillisecondsToWait,pImageFrame) )

#define INuiSensor_NuiImageStreamReleaseFrame(This,hStream,pImageFrame)	\
    ( (This)->lpVtbl -> NuiImageStreamReleaseFrame(This,hStream,pImageFrame) )

#define INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixel(This,eColorResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY)	\
    ( (This)->lpVtbl -> NuiImageGetColorPixelCoordinatesFromDepthPixel(This,eColorResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY) )

#define INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(This,eColorResolution,eDepthResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY)	\
    ( (This)->lpVtbl -> NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(This,eColorResolution,eDepthResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY) )

#define INuiSensor_NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(This,eColorResolution,eDepthResolution,cDepthValues,pDepthValues,cColorCoordinates,pColorCoordinates)	\
    ( (This)->lpVtbl -> NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(This,eColorResolution,eDepthResolution,cDepthValues,pDepthValues,cColorCoordinates,pColorCoordinates) )

#define INuiSensor_NuiCameraElevationSetAngle(This,lAngleDegrees)	\
    ( (This)->lpVtbl -> NuiCameraElevationSetAngle(This,lAngleDegrees) )

#define INuiSensor_NuiCameraElevationGetAngle(This,plAngleDegrees)	\
    ( (This)->lpVtbl -> NuiCameraElevationGetAngle(This,plAngleDegrees) )

#define INuiSensor_NuiSkeletonTrackingEnable(This,hNextFrameEvent,dwFlags)	\
    ( (This)->lpVtbl -> NuiSkeletonTrackingEnable(This,hNextFrameEvent,dwFlags) )

#define INuiSensor_NuiSkeletonTrackingDisable(This)	\
    ( (This)->lpVtbl -> NuiSkeletonTrackingDisable(This) )

#define INuiSensor_NuiSkeletonSetTrackedSkeletons(This,TrackingIDs)	\
    ( (This)->lpVtbl -> NuiSkeletonSetTrackedSkeletons(This,TrackingIDs) )

#define INuiSensor_NuiSkeletonGetNextFrame(This,dwMillisecondsToWait,pSkeletonFrame)	\
    ( (This)->lpVtbl -> NuiSkeletonGetNextFrame(This,dwMillisecondsToWait,pSkeletonFrame) )

#define INuiSensor_NuiTransformSmooth(This,pSkeletonFrame,pSmoothingParams)	\
    ( (This)->lpVtbl -> NuiTransformSmooth(This,pSkeletonFrame,pSmoothingParams) )

#define INuiSensor_NuiGetAudioSource(This,ppDmo)	\
    ( (This)->lpVtbl -> NuiGetAudioSource(This,ppDmo) )

#define INuiSensor_NuiInstanceIndex(This)	\
    ( (This)->lpVtbl -> NuiInstanceIndex(This) )

#define INuiSensor_NuiDeviceConnectionId(This)	\
    ( (This)->lpVtbl -> NuiDeviceConnectionId(This) )

#define INuiSensor_NuiUniqueId(This)	\
    ( (This)->lpVtbl -> NuiUniqueId(This) )

#define INuiSensor_NuiAudioArrayId(This)	\
    ( (This)->lpVtbl -> NuiAudioArrayId(This) )

#define INuiSensor_NuiStatus(This)	\
    ( (This)->lpVtbl -> NuiStatus(This) )

#define INuiSensor_NuiInitializationFlags(This)	\
    ( (This)->lpVtbl -> NuiInitializationFlags(This) )

#define INuiSensor_NuiGetCoordinateMapper(This,pMapping)	\
    ( (This)->lpVtbl -> NuiGetCoordinateMapper(This,pMapping) )

#define INuiSensor_NuiImageFrameGetDepthImagePixelFrameTexture(This,hStream,pImageFrame,pNearMode,ppFrameTexture)	\
    ( (This)->lpVtbl -> NuiImageFrameGetDepthImagePixelFrameTexture(This,hStream,pImageFrame,pNearMode,ppFrameTexture) )

#define INuiSensor_NuiGetColorCameraSettings(This,pCameraSettings)	\
    ( (This)->lpVtbl -> NuiGetColorCameraSettings(This,pCameraSettings) )

#define INuiSensor_NuiGetForceInfraredEmitterOff(This)	\
    ( (This)->lpVtbl -> NuiGetForceInfraredEmitterOff(This) )

#define INuiSensor_NuiSetForceInfraredEmitterOff(This,fForceInfraredEmitterOff)	\
    ( (This)->lpVtbl -> NuiSetForceInfraredEmitterOff(This,fForceInfraredEmitterOff) )

#define INuiSensor_NuiAccelerometerGetCurrentReading(This,pReading)	\
    ( (This)->lpVtbl -> NuiAccelerometerGetCurrentReading(This,pReading) )

#define INuiSensor_NuiSetDepthFilter(This,pDepthFilter)	\
    ( (This)->lpVtbl -> NuiSetDepthFilter(This,pDepthFilter) )

#define INuiSensor_NuiGetDepthFilter(This,ppDepthFilter)	\
    ( (This)->lpVtbl -> NuiGetDepthFilter(This,ppDepthFilter) )

#define INuiSensor_NuiGetDepthFilterForTimeStamp(This,liTimeStamp,ppDepthFilter)	\
    ( (This)->lpVtbl -> NuiGetDepthFilterForTimeStamp(This,liTimeStamp,ppDepthFilter) )




DECLARE_VTABLE_METHOD(ULONG, INuiSensor, AddRef )(
    INuiSensor * This)
    {return INuiSensor_AddRef(This); }

DECLARE_VTABLE_METHOD(ULONG, INuiSensor, Release )(
    INuiSensor * This)
    {return INuiSensor_Release(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiInitialize )(
    INuiSensor * This,
    /* [in] */ DWORD dwFlags)
    {return INuiSensor_NuiInitialize(This,dwFlags); }

DECLARE_VTABLE_METHOD(void, INuiSensor, NuiShutdown )(
    INuiSensor * This)
    {return INuiSensor_NuiShutdown(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSetFrameEndEvent )(
    INuiSensor * This,
    /* [in] */ HANDLE hEvent,
    /* [in] */ DWORD dwFrameEventFlag)
    {return INuiSensor_NuiSetFrameEndEvent(This,hEvent,dwFrameEventFlag); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageStreamOpen )(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_TYPE eImageType,
    /* [in] */ NUI_IMAGE_RESOLUTION eResolution,
    /* [in] */ DWORD dwImageFrameFlags,
    /* [in] */ DWORD dwFrameLimit,
    /* [in] */ HANDLE hNextFrameEvent,
    /* [out] */ HANDLE *phStreamHandle)
    {return INuiSensor_NuiImageStreamOpen(This,eImageType,eResolution,dwImageFrameFlags,dwFrameLimit,hNextFrameEvent,phStreamHandle); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageStreamSetImageFrameFlags )(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ DWORD dwImageFrameFlags)
    {return INuiSensor_NuiImageStreamSetImageFrameFlags(This,hStream,dwImageFrameFlags); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageStreamGetImageFrameFlags )(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [retval][out] */ DWORD *pdwImageFrameFlags)
    {return INuiSensor_NuiImageStreamGetImageFrameFlags(This,hStream,pdwImageFrameFlags); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageStreamGetNextFrame )(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ DWORD dwMillisecondsToWait,
    /* [retval][out] */ NUI_IMAGE_FRAME *pImageFrame)
    {return INuiSensor_NuiImageStreamGetNextFrame(This,hStream,dwMillisecondsToWait,pImageFrame); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageStreamReleaseFrame )(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ NUI_IMAGE_FRAME *pImageFrame)
    {return INuiSensor_NuiImageStreamReleaseFrame(This,hStream,pImageFrame); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageGetColorPixelCoordinatesFromDepthPixel )(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ const NUI_IMAGE_VIEW_AREA *pcViewArea,
    /* [in] */ LONG lDepthX,
    /* [in] */ LONG lDepthY,
    /* [in] */ USHORT usDepthValue,
    /* [out] */ LONG *plColorX,
    /* [out] */ LONG *plColorY)
    {return INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixel(This,eColorResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution )(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ NUI_IMAGE_RESOLUTION eDepthResolution,
    /* [in] */ const NUI_IMAGE_VIEW_AREA *pcViewArea,
    /* [in] */ LONG lDepthX,
    /* [in] */ LONG lDepthY,
    /* [in] */ USHORT usDepthValue,
    /* [out] */ LONG *plColorX,
    /* [out] */ LONG *plColorY)
    {return INuiSensor_NuiImageGetColorPixelCoordinatesFromDepthPixelAtResolution(This,eColorResolution,eDepthResolution,pcViewArea,lDepthX,lDepthY,usDepthValue,plColorX,plColorY); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution )(
    INuiSensor * This,
    /* [in] */ NUI_IMAGE_RESOLUTION eColorResolution,
    /* [in] */ NUI_IMAGE_RESOLUTION eDepthResolution,
    /* [in] */ DWORD cDepthValues,
    /* [size_is][in] */ USHORT *pDepthValues,
    /* [in] */ DWORD cColorCoordinates,
    /* [size_is][out][in] */ LONG *pColorCoordinates)
    {return INuiSensor_NuiImageGetColorPixelCoordinateFrameFromDepthPixelFrameAtResolution(This,eColorResolution,eDepthResolution,cDepthValues,pDepthValues,cColorCoordinates,pColorCoordinates); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiCameraElevationSetAngle )(
    INuiSensor * This,
    /* [in] */ LONG lAngleDegrees)
    {return INuiSensor_NuiCameraElevationSetAngle(This,lAngleDegrees); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiCameraElevationGetAngle )(
    INuiSensor * This,
    /* [retval][out] */ LONG *plAngleDegrees)
    {return INuiSensor_NuiCameraElevationGetAngle(This,plAngleDegrees); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSkeletonTrackingEnable )(
    INuiSensor * This,
    /* [in] */ HANDLE hNextFrameEvent,
    /* [in] */ DWORD dwFlags)
    {return INuiSensor_NuiSkeletonTrackingEnable(This,hNextFrameEvent,dwFlags); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSkeletonTrackingDisable )(
    INuiSensor * This)
    {return INuiSensor_NuiSkeletonTrackingDisable(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSkeletonSetTrackedSkeletons )(
    INuiSensor * This,
    /* [size_is][in] */ DWORD *TrackingIDs)
    {return INuiSensor_NuiSkeletonSetTrackedSkeletons(This,TrackingIDs); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSkeletonGetNextFrame )(
    INuiSensor * This,
    /* [in] */ DWORD dwMillisecondsToWait,
    /* [out][in] */ NUI_SKELETON_FRAME *pSkeletonFrame)
    {return INuiSensor_NuiSkeletonGetNextFrame(This,dwMillisecondsToWait,pSkeletonFrame); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiTransformSmooth )(
    INuiSensor * This,
    NUI_SKELETON_FRAME *pSkeletonFrame,
    const NUI_TRANSFORM_SMOOTH_PARAMETERS *pSmoothingParams)
    {return INuiSensor_NuiTransformSmooth(This,pSkeletonFrame,pSmoothingParams); }

DECLARE_VTABLE_METHOD(/* [helpstring] */ HRESULT, INuiSensor, NuiGetAudioSource )(
    INuiSensor * This,
    /* [out] */ INuiAudioBeam **ppDmo)
    {return INuiSensor_NuiGetAudioSource(This,ppDmo); }

DECLARE_VTABLE_METHOD(int, INuiSensor, NuiInstanceIndex )(
    INuiSensor * This)
    {return INuiSensor_NuiInstanceIndex(This); }

DECLARE_VTABLE_METHOD(BSTR, INuiSensor, NuiDeviceConnectionId )(
    INuiSensor * This)
    {return INuiSensor_NuiDeviceConnectionId(This); }

DECLARE_VTABLE_METHOD(BSTR, INuiSensor, NuiUniqueId )(
    INuiSensor * This)
    {return INuiSensor_NuiUniqueId(This); }

DECLARE_VTABLE_METHOD(BSTR, INuiSensor, NuiAudioArrayId )(
    INuiSensor * This)
    {return INuiSensor_NuiAudioArrayId(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiStatus )(
    INuiSensor * This)
    {return INuiSensor_NuiStatus(This); }

DECLARE_VTABLE_METHOD(DWORD, INuiSensor, NuiInitializationFlags )(
    INuiSensor * This)
    {return INuiSensor_NuiInitializationFlags(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiGetCoordinateMapper )(
    INuiSensor * This,
    /* [retval][out] */ INuiCoordinateMapper **pMapping)
    {return INuiSensor_NuiGetCoordinateMapper(This,pMapping); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiImageFrameGetDepthImagePixelFrameTexture )(
    INuiSensor * This,
    /* [in] */ HANDLE hStream,
    /* [in] */ NUI_IMAGE_FRAME *pImageFrame,
    /* [out] */ BOOL *pNearMode,
    /* [out] */ INuiFrameTexture **ppFrameTexture)
    {return INuiSensor_NuiImageFrameGetDepthImagePixelFrameTexture(This,hStream,pImageFrame,pNearMode,ppFrameTexture); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiGetColorCameraSettings )(
    INuiSensor * This,
    /* [retval][out] */ INuiColorCameraSettings **pCameraSettings)
    {return INuiSensor_NuiGetColorCameraSettings(This,pCameraSettings); }

DECLARE_VTABLE_METHOD(BOOL, INuiSensor, NuiGetForceInfraredEmitterOff )(
    INuiSensor * This)
    {return INuiSensor_NuiGetForceInfraredEmitterOff(This); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSetForceInfraredEmitterOff )(
    INuiSensor * This,
    /* [in] */ BOOL fForceInfraredEmitterOff)
    {return INuiSensor_NuiSetForceInfraredEmitterOff(This,fForceInfraredEmitterOff); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiAccelerometerGetCurrentReading )(
    INuiSensor * This,
    /* [retval][out] */ Vector4 *pReading)
    {return INuiSensor_NuiAccelerometerGetCurrentReading(This,pReading); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiSetDepthFilter )(
    INuiSensor * This,
    /* [in] */ INuiDepthFilter *pDepthFilter)
    {return INuiSensor_NuiSetDepthFilter(This,pDepthFilter); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiGetDepthFilter )(
    INuiSensor * This,
    /* [retval][out] */ INuiDepthFilter **ppDepthFilter)
    {return INuiSensor_NuiGetDepthFilter(This,ppDepthFilter); }

DECLARE_VTABLE_METHOD(HRESULT, INuiSensor, NuiGetDepthFilterForTimeStamp )(
    INuiSensor * This,
    /* [in] */ LARGE_INTEGER liTimeStamp,
    /* [retval][out] */ INuiDepthFilter **ppDepthFilter)
    {return INuiSensor_NuiGetDepthFilterForTimeStamp(This,liTimeStamp,ppDepthFilter); }
