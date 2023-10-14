use std::ffi::c_void;

use super::{context::DivisionContext, shader::DivisionShaderVariableType};

#[repr(i32)]
pub enum DivisionRenderTopology {
    Triangles = 1,
    Points = 2,
    Lines = 3,
}

#[repr(C)]
pub struct DivisionVertexAttributeDescriptor {
    pub field_type: DivisionShaderVariableType,
    pub location: i32,
}

#[repr(C)]
pub struct DivisionVertexBufferDescriptor {
    pub per_vertex_attributes: *const DivisionVertexAttributeDescriptor,
    pub per_instance_attributes: *const DivisionVertexAttributeDescriptor,
    pub per_vertex_attribute_count: i32,
    pub per_instance_attribute_count: i32,
    pub vertex_count: i32,
    pub index_count: i32,
    pub instance_count: i32,
    pub topology: DivisionRenderTopology,
}

#[repr(C)]
pub struct DivisionVertexBufferBorrowedData {
    pub vertex_data_ptr: *mut c_void,
    pub index_data_ptr: *mut c_void,
    pub instance_data_ptr: *mut c_void,
    pub vertex_count: u32,
    pub index_count: u32,
    pub instance_count: u32,
}

extern "C" {
    pub fn division_engine_vertex_buffer_alloc(
        ctx: *mut DivisionContext,
        desriptor: *const DivisionVertexBufferDescriptor,
        out_vertex_buffer_id: *mut u32,
    ) -> bool;

    pub fn division_engine_vertex_buffer_free(
        ctx: *mut DivisionContext,
        vertex_buffer_id: u32,
    );

    pub fn division_engine_vertex_buffer_borrow_data(
        ctx: *mut DivisionContext,
        vertex_buffer_id: u32,
        out_borrow_info: *mut DivisionVertexBufferBorrowedData,
    ) -> bool;

    pub fn division_engine_vertex_buffer_return_data(
        ctx: *mut DivisionContext,
        vertex_buffer: u32,
        data_pointer: *mut DivisionVertexBufferBorrowedData,
    );
}
