use alloc::ffi::CString;
use std::ffi::{c_char, CStr};
use crate::division_engine::bridge::renderer::*;

static mut _UPDATE_INSTANCE: Option<fn(s: DivisionEngineState)> = None;

pub struct Renderer;

impl Renderer {
    pub fn new(
        title: &str,
        width: i32,
        height: i32,
        update_callback: fn(s: DivisionEngineState)
    ) -> Self {
        let c_title = CString::new(title).unwrap();
        let settings = DivisionEngineSettings {
            window_width: width,
            window_height: height,
            title: c_title.as_ptr(),
        };
        unsafe {
            division_engine_renderer_create(&settings, error_callback);
            _UPDATE_INSTANCE = Some(update_callback);

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
            _UPDATE_INSTANCE = None;
        }
    }
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!("Error code:{}, error message: {}", error_code, c_message);
}

unsafe extern "C" fn update_callback(s: DivisionEngineState) {
    (_UPDATE_INSTANCE.unwrap())(s);
}