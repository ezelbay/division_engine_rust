use std::ffi::c_char;
use super::context::DivisionContext;

#[repr(i32)]
pub enum ShaderType {
    Vertex = 1,
    Fragment = 2,
}

#[repr(C)]
pub struct DivisionShaderFileDescriptor {
    pub shader_type: ShaderType,
    pub file_path: *const c_char,
    pub entry_point_name: *const c_char,
}

extern "C" {
    pub fn division_engine_shader_program_alloc(
        ctx: *mut DivisionContext,
        settings: *const DivisionShaderFileDescriptor,
        source_count: i32,
        out_shader_program_id: *mut u32,
    ) -> bool;

    pub fn division_engine_shader_program_free(ctx: *mut DivisionContext, shader_program_id: u32);
}