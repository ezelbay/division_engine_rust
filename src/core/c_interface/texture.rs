use std::ffi::c_void;

use super::context::DivisionContext;

#[repr(i32)]
pub enum DivisionTextureFormat {
    R8Uint,
    RGB24Uint,
    RGBA32Uint,
}

#[repr(C)]
pub struct DivisionTextureDescriptor {
    pub texture_format: DivisionTextureFormat,
    pub width: u32,
    pub height: u32,
}

extern "C" {
    pub fn division_engine_texture_alloc(
        ctx: *mut DivisionContext,
        texture: *const DivisionTextureDescriptor,
        out_texture_id: *mut u32,
    ) -> bool;
    pub fn division_engine_texture_free(ctx: *mut DivisionContext, texture_id: u32);

    pub fn division_engine_texture_set_data(
        ctx: *mut DivisionContext,
        texture_id: u32,
        data: *const c_void,
    );
}
