use std::ffi::{c_char};

pub type DivisionEngineErrorFunc = unsafe extern "C" fn(i32, *const c_char);

#[repr(C)]
pub struct DivisionEngineSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub title: *const c_char,
    pub error_callback: DivisionEngineErrorFunc,
}

extern "C" {
    pub fn division_engine_init(settings: *const DivisionEngineSettings) -> bool;
}
