use std::ffi::c_char;

#[repr(C)]
pub enum ShaderType {
    Vertex = 0,
    Fragment = 1
}

extern "C" {
    pub fn division_engine_shader_create(path: *const c_char, shader_type: ShaderType) -> i32;
}