use std::ffi::c_char;

use super::context::DivisionContext;

pub type DivisionEngineErrorFunc = unsafe extern "C" fn(i32, *const c_char);
pub type DivisionLifecycleFunc = unsafe extern "C" fn(*mut DivisionContext);

#[repr(C)]
pub struct DivisionSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: *const c_char,
    pub error_callback: DivisionEngineErrorFunc,
    pub init_callback: DivisionLifecycleFunc,
    pub update_callback: DivisionLifecycleFunc,
}