use std::ffi::{c_char, CStr, CString};
use crate::division_engine::bridge::renderer::*;

pub struct Renderer;

impl Renderer {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let c_title = CString::new(title);
        let settings = DivisionEngineSettings {
            window_width: width,
            window_height: height,
            title: c_title.unwrap().as_ptr(),
        };
        unsafe {
            division_engine_renderer_create(&settings, error_callback);
            return Renderer {};
        }
    }

    pub fn run_loop(&self) {
        unsafe {
            division_engine_renderer_run_loop(update_callback);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            division_engine_renderer_destroy();
        }
    }
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!("Error code:{}, error message: {}", error_code, c_message);
}

unsafe extern "C" fn update_callback(_: DivisionEngineState) {
}