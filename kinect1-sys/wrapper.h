
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

