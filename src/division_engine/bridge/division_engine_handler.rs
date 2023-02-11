use std::ffi::{c_void};
use std::ptr::null;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DivisionEngineBridgeHandler {
    _internal_data: *const c_void,
}

impl DivisionEngineBridgeHandler {
    pub fn new() -> DivisionEngineBridgeHandler {
        return DivisionEngineBridgeHandler { _internal_data: null() };
    }
}

