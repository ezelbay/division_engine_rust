use std::ffi::{c_char};

pub type DivisionEngineErrorFunc = unsafe extern "C" fn(i32, *const c_char);
pub type DivisionEngineUpdateFunc = unsafe extern "C" fn(DivisionEngineState);

#[repr(C)]
pub struct DivisionEngineSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub title: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DivisionEngineState {
    pub delta_time: f64
}

extern "C" {
    pub fn division_engine_start_session(error_callback: DivisionEngineErrorFunc) -> bool;
    pub fn division_engine_window_create(settings: *const DivisionEngineSettings) -> i32;
    pub fn division_engine_window_run_event_loop(
        window_id: i32, update_callback: DivisionEngineUpdateFunc);
    pub fn division_engine_window_destroy(window_id: i32);
    pub fn division_engine_finish_session();
}
