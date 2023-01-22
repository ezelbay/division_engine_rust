use std::ffi::{CString, CStr, c_char};
use division_engine_rust::division_engine_c::window::*;
use division_engine_rust::division_engine_c::shader::*;

fn main() {
    unsafe {
        let title = CString::new("Hey").unwrap();
        let settings = DivisionEngineSettings {
            window_width: 512,
            window_height: 512,
            title: title.as_ptr(),
            error_callback
        };

        division_engine_init(&settings);
        let shader_path = CString::new("resources/shaders/default_ui.frag").unwrap();
        let res = division_engine_shader_create(
            shader_path.as_ptr(),
            ShaderType::Fragment
        );
    }
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!(
        "Error code:{}, error message: {}",
        error_code,
        c_message
    );
}
