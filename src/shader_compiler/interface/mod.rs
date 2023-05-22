use std::ffi::{c_char, c_int, c_void, c_ulong};

pub static SHADER_TYPE_VERTEX: c_int = 1;
pub static SHADER_TYPE_FRAGMENT: c_int = 2;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ShaderType {
    Vertex = 1, 
    Fragment = 2
}

extern "C" {
    pub fn division_shader_compiler_alloc() -> bool;

    pub fn division_shader_compiler_compile_glsl_to_spirv(
        source: *const c_char, 
        source_size: i32,
        shader_type: ShaderType,
        spirv_entry_point_name: *const c_char,
        out_spirv: *mut *mut c_void,
        out_spirv_byte_count: *mut c_ulong
    ) -> bool;

    pub fn division_shader_compiler_compile_spirv_to_metal(
        spirv_bytes: *const c_void,
        spirv_byte_count: c_ulong,
        shader_type: ShaderType,
        entry_point: *const c_char,
        out_metal_source: *mut *mut c_char,
        out_metal_size: *mut c_ulong
    ) -> bool;

    pub fn division_shader_compiler_spirv_source_free(spirv_source: *mut c_void);
    pub fn division_shader_compiler_metal_source_free(metal_source: *mut c_char);

    pub fn division_shader_compiler_free();
}