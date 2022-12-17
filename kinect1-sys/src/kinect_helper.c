
#include <stddef.h>
#include <stdbool.h>
#include <assert.h>
#include <windows.h>

// TODO: maybe find a real fix, instead of changing this thing from "NuiSensor.h"
#define DEFINE_UUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8)
// #define DEFINE_UUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) extern "C"  const GUID name = { l, w1, w2, { b1, b2,  b3,  b4,  b5,  b6,  b7,  b8 } }
#define __No__NuiSkeleton_h__
#define CINTERFACE
#define COBJMACROS
#include "NuiApi.h"

#define kinect_helper_no_forward_decl
#include "kinect_helper.h"


int hello() {
    return 42;
}

struct c_INuiSensor {
    // TODO: is this ok to do
    void* _placeholder1;
    void* _placeholder2;
    void* _placeholder3;
};

HRESULT c_NuiGetSensorCount(int32_t* pCount) {
    return NuiGetSensorCount(pCount);
}

HRESULT c_NuiCreateSensorByIndex(int32_t index, INuiSensor ** ppNuiSensor ) {
    return NuiCreateSensorByIndex(index, ppNuiSensor);
}


// cobj macros
#include "nui_sensor_interface.h"
#include "nui_sensor_interface.inc"



