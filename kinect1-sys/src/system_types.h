#pragma once
#include <stdint.h>
#ifndef kinect_helper_no_forward_decl

// manual types
#define _HRESULT_DEFINED
/** <div rustbindgen mustusetype></div> */
typedef int32_t HRESULT;

typedef void *HANDLE;
typedef wchar_t OLECHAR;
typedef wchar_t *BSTR;

// from random windows headers
#ifndef VOID
#define VOID void
typedef char CHAR;
typedef short SHORT;
typedef long LONG;
#if !defined(MIDL_PASS)
typedef int INT;
#endif
#endif

typedef unsigned long ULONG;
typedef ULONG *PULONG;
typedef unsigned short USHORT;
typedef USHORT *PUSHORT;
typedef unsigned char UCHAR;
typedef UCHAR *PUCHAR;
// typedef _Null_terminated_ char *PSZ;

typedef unsigned long DWORD;
typedef int BOOL;
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef float FLOAT;
typedef FLOAT *PFLOAT;

typedef float FLOAT;
typedef unsigned int UINT;
typedef int INT;
typedef unsigned int UINT;
typedef unsigned int *PUINT;

typedef unsigned char byte;
typedef byte cs_byte;
typedef unsigned char boolean;

// wtf?!?!
typedef __int64 LONGLONG;
typedef unsigned __int64 ULONGLONG;
typedef LONGLONG *PLONGLONG;
typedef ULONGLONG *PULONGLONG;

typedef struct _LARGE_INTEGER
{
    LONGLONG QuadPart;
} LARGE_INTEGER;

#endif