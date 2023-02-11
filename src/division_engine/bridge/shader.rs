use std::ffi::{c_char};

#[repr(C)]
pub enum ShaderType {
    Vertex = 0,
    Fragment = 1,
}

extern "C" {
    pub fn division_engine_shader_create_program() -> i32;
    pub fn division_engine_shader_attach_to_program(
        path: *const c_char, shader_type: ShaderType, program_id: i32) -> bool;
    pub fn division_engine_shader_link_program(program_id: i32) -> bool;
    pub fn division_engine_shader_use_program(program_id: i32);
}