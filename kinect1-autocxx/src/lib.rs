use autocxx::prelude::*; // use all the main autocxx functions
use miette::Diagnostic;
use miette::Result;
use thiserror::Error;

include_cpp! {
    #include "wrapper.hpp" // your header file name
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    block!("OLECHAR")
    // add this line for each function or type you wish to generate
    generate!("INuiSensor")
    // generate!("INuiSensorVtbl")
    generate!("NuiCreateSensorByIndex")
    generate!("NuiGetSensorCount")
    // generate!("S_OK")
}

#[derive(Error, Diagnostic, Debug)]
pub enum KinectError {
    #[error("HResult({0})")]
    HResult(i32),
}

pub type KinectResult<T> = Result<T, KinectError>;

fn check_fail(raw_hresult: c_long) -> Result<(), KinectError> {
    let raw_hresult = raw_hresult.0;
    if raw_hresult < 0 {
        Err(KinectError::HResult(raw_hresult))
    } else {
        Ok(())
    }
}

pub fn get_sensor_count() -> Result<i32> {
    unsafe {
        let mut i_sensor_count = c_int(0);
        check_fail(ffi::NuiGetSensorCount(&mut i_sensor_count))?;
        Ok(i_sensor_count.0)
    }
}


pub fn foo(s: &ffi::INuiSensor) {
    // (*s)
}

pub fn create_sensor_by_index(index: i32) -> KinectResult<()> {
    unsafe {
        // ffi::INuiSensor::get
        let mut p_nui_sensor: *mut ffi::INuiSensor = std::ptr::null_mut();
        // check_fail(ffi::NuiCreateSensorByIndex(index, &mut p_nui_sensor))?;
        Ok(())
        // Ok(Sensor {
        //     delegate: p_nui_sensor,
        // })
    }
}
