use std::ffi::c_void;

use super::{context::DivisionContext, shader::ShaderVariableType};

#[repr(i32)]
pub enum RenderTopology {
    Triangles = 1,
    Points = 2,
    Lines = 3,
}

#[repr(C)]
pub struct VertexAttributeInputDescriptor {
    pub field_type: ShaderVariableType,
    pub location: i32,
}

#[repr(C)]
pub struct VertexBufferInputDescriptor {
    pub per_vertex_attributes: *const VertexAttributeInputDescriptor,
    pub per_instance_attributes: *const VertexAttributeInputDescriptor,
    pub per_vertex_attribute_count: i32,
    pub per_instance_attribute_count: i32,
    pub vertex_count: i32,
    pub instance_count: i32,
    pub topology: RenderTopology,
}

extern "C" {
    pub fn division_engine_vertex_buffer_alloc(
        ctx: *mut DivisionContext,
        desriptor: *const VertexBufferInputDescriptor,
        out_vertex_buffer_id: *mut u32,
    ) -> bool;

    pub fn division_engine_vertex_buffer_free(ctx: *mut DivisionContext, vertex_buffer_id: u32);

    pub fn division_engine_vertex_buffer_borrow_data_pointer(
        ctx: *mut DivisionContext,
        vertex_buffer: u32,
    ) -> *mut c_void;

    pub fn division_engine_vertex_buffer_return_data_pointer(
        ctx: *mut DivisionContext,
        vertex_buffer: u32,
        data_pointer: *mut c_void,
    );
}
