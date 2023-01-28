use std::ffi::{CString, CStr, c_char};
use division_engine_rust::division_engine_c::window::*;
use division_engine_rust::division_engine_c::shader::*;

fn main() {
    unsafe {
        init_engine();
    }
}

unsafe fn init_engine() {
    let title = CString::new("Hey").unwrap();
    let settings = DivisionEngineSettings {
        window_width: 512,
        window_height: 512,
        title: title.as_ptr(),
        error_callback
    };

    let mut handler=  DivisionEngineHandler::new();
    division_engine_create_window(&settings, &mut handler);
    let shader_path = CString::new("resources/shaders/default_ui.frag").unwrap();
    division_engine_shader_create(shader_path.as_ptr(), ShaderType::Fragment);

    division_engine_run_event_loop(handler.clone(), update_callback);

    division_engine_destroy_window(handler);
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!("Error code:{}, error message: {}", error_code, c_message);
}

unsafe extern "C" fn update_callback(state: DivisionEngineState) {
}
