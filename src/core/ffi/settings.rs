use std::ffi::c_char;

#[repr(C)]
pub struct DivisionSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: *const c_char,
}
