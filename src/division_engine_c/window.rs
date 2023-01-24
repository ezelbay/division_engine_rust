use std::ffi::{c_char};
pub use super::division_engine_handler::*;

pub type DivisionEngineErrorFunc = unsafe extern "C" fn(i32, *const c_char);
pub type DivisionEngineUpdateFunc = unsafe extern "C" fn(DivisionEngineState);

#[repr(C)]
pub struct DivisionEngineSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub title: *const c_char,
    pub error_callback: DivisionEngineErrorFunc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DivisionEngineState {
    pub delta_time: f64
}

extern "C" {
    pub fn division_engine_create_window(
        settings: *const DivisionEngineSettings,
        output_handler: *mut DivisionEngineHandler
    ) -> bool;

    pub fn division_engine_run_event_loop(
        handler: DivisionEngineHandler, update_callback: DivisionEngineUpdateFunc);

    pub fn division_engine_destroy_window(handler: DivisionEngineHandler);
}
