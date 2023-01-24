use std::ffi::{c_void};
use std::ptr::null;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DivisionEngineHandler {
    _internal_data: *const c_void,
}

impl DivisionEngineHandler {
    pub fn new() -> DivisionEngineHandler {
        return DivisionEngineHandler { _internal_data: null() };
    }
}

