use std::ffi::{c_ulong, c_void};

use super::context::DivisionContext;

#[repr(C)]
pub struct UniformBufferDescriptor {
    pub data_bytes: c_ulong,
}

extern "C" {
    pub fn division_engine_uniform_buffer_alloc(
        ctx: *mut DivisionContext,
        buffer: UniformBufferDescriptor,
        out_buffer_id: *mut u32,
    ) -> bool;

    pub fn division_engine_uniform_buffer_free(ctx: *mut DivisionContext, buffer_id: u32);

    pub fn division_engine_uniform_buffer_borrow_data_pointer(
        ctx: *mut DivisionContext,
        buffer_id: u32,
    ) -> *mut c_void;

    pub fn division_engine_uniform_buffer_return_data_pointer(
        ctx: *mut DivisionContext,
        buffer_id: u32,
        data_pointer: *mut c_void,
    );

}
