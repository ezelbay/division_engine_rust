use std::ffi::{c_char, CStr, CString};

type DivisionEngineErrorFunc = unsafe extern "C" fn(i32, *const c_char);

#[repr(C)]
pub struct DivisionEngineSettings {
    window_width: i32,
    window_height: i32,
    title: *const c_char,
    error_callback: DivisionEngineErrorFunc
}

extern "C" {
    pub fn division_engine_init(settings: *const DivisionEngineSettings) -> bool;
}

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
    }
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    eprintln!(
        "Error code:{}, error message: {}",
        error_code,
        CStr::from_ptr(message).to_str().unwrap()
    );
}
