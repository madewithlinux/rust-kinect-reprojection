#include <stdint.h>
// #include "constants.h"
#ifndef kinect_helper_no_forward_decl
#include "forward_declarations.h"
#include "constants.h"
#include "system_types.h"
#endif
// #include "nui_sensor_interface.h"


// #include <stddef.h>
// #include <stdbool.h>
// #include <assert.h>
// #include <windows.h>
// #include "NuiApi.h"

int32_t hello();



typedef struct c_INuiSensor c_INuiSensor;


HRESULT c_NuiGetSensorCount(int32_t* pCount);


HRESULT c_NuiCreateSensorByIndex(int32_t index, INuiSensor ** ppNuiSensor );


// HRESULT c_INuiSensor_NuiStatus(
//     INuiSensor * This);
