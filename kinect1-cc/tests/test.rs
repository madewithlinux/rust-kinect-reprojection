extern crate kinect1_sys;
extern crate kinect1_cc;
use kinect1_cc::hello;

#[test]
fn test_hello() {
    unsafe { assert_eq!(hello(), 42) }
}

#[test]
fn test_NuiGetSensorCount() {
    kinect1_sys::NuiGetSensorCount;
}
