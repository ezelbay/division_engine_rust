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
    pub fn division_engine_renderer_create(
        settings: *const DivisionEngineSettings, error_callback: DivisionEngineErrorFunc) -> bool;
    pub fn division_engine_renderer_run_loop(update_callback: DivisionEngineUpdateFunc);
    pub fn division_engine_renderer_destroy();
}
