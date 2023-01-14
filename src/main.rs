use std::ffi::{c_char, CString};

extern "C" {
    pub fn division_engine_init(width: i32, height: i32, title: *const c_char) -> bool;
}

fn main() {
    unsafe {
        let title = CString::new("Hey").unwrap();
        division_engine_init(512, 512, title.as_ptr());
    }
}
