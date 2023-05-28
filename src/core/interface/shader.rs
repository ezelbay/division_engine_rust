use std::ffi::c_char;
use super::context::DivisionContext;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex = 1,
    Fragment = 2,
}

#[repr(i32)]
pub enum ShaderVariableType {
    Float = 1,
    Double = 2,
    Integer = 3,
    FVec2 = 4,
    FVec3 = 5,
    FVec4 = 6,
    FMat4x4 = 7
}

#[repr(C)]
pub struct DivisionShaderSourceDescriptor {
    pub shader_type: ShaderType,
    pub entry_point_name: *const c_char,
    pub source: *const c_char,
    pub source_size: i32
}

extern "C" {
    pub fn division_engine_shader_program_alloc(
        ctx: *mut DivisionContext,
        descriptors: *const DivisionShaderSourceDescriptor,
        descriptor_count: i32,
        out_shader_program_id: *mut u32,
    ) -> bool;

    pub fn division_engine_shader_program_free(ctx: *mut DivisionContext, shader_program_id: u32);
}