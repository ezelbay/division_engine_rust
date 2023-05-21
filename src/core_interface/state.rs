use std::ffi::c_double;

#[repr(C)]
pub struct DivisionState {
    delta_time: c_double
}