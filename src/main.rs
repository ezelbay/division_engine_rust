use std::ffi::{c_ulong, c_long, c_float, CString, CStr, c_char};
use std::ptr::null_mut;
use division_engine_rust::division_engine::core_interface::context::*;
use division_engine_rust::division_engine::core_interface::renderer::division_engine_renderer_run_loop;
use division_engine_rust::division_engine::core_interface::settings::DivisionSettings;

static VERTICES: [f32; 9] = [
    -0.9, -0.9, 0.,
    0.85, -0.9, 0.,
    -0.9, 0.85, 0.
];

static mut BUFFER_ID: c_long = -1;

fn main() {
    unsafe {
        init_engine();
    }
}

unsafe fn init_engine() {
    let window_title = CString::new("Hello window").unwrap();
    let settings: DivisionSettings = DivisionSettings {
        window_width: 512,
        window_height: 512,
        window_title: window_title.as_ptr(),
        error_callback: error_func,
        init_callback: init_func,
        update_callback: update_func,
    };
    let mut context: *mut DivisionContext = null_mut();
    division_engine_context_alloc(&settings, (&mut context) as *mut *mut DivisionContext);

    division_engine_renderer_run_loop(context, &settings);

    division_engine_context_free(context);
}

unsafe extern "C" fn error_func(error_code: i32, error_msg: *const c_char) {
    let msg = CStr::from_ptr(error_msg);
    println!("Error code: {}, error message:\n {}\n", error_code, msg.to_str().unwrap());
}

extern "C" fn init_func(ctx: *const DivisionContext) {}

extern "C" fn update_func(ctx: *const DivisionContext) {}