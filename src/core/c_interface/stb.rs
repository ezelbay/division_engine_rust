use std::ffi::{c_char, c_int};

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

    pub fn stbi_write_png(
        file_path: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const u8,
        stride_in_bytes: c_int,
    ) -> bool;
    pub fn stbi_write_bmp(
        file_path: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const u8,
    ) -> bool;

    pub fn stbi_write_tga(
        file_path: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const u8,
    ) -> bool;

    pub fn stbi_write_jpg(
        file_path: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const u8,
        quality: c_int,
    ) -> bool;

    pub fn stbi_write_hdr(
        file_path: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const u8,
    ) -> bool;
}
