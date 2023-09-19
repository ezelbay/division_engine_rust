use std::ffi::{c_char, c_int, c_void};

extern "C" {
    pub fn stbi_load(
        path: *const c_char,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        required_channels: c_int,
    ) -> *mut u8;

    pub fn stbi_load_from_memory(
        buffer: *const u8,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut u8;

    pub fn stbi_set_flip_vertically_on_load(should_flip: c_int);

    pub fn stbi_image_free(data_ptr: *mut u8);
}
