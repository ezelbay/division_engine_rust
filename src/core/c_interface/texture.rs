use std::ffi::c_void;

use super::context::DivisionContext;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum DivisionTextureFormat {
    R8Uint = 1,
    RGB24Uint = 2,
    RGBA32Uint = 3,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum DivisionTextureChannelSwizzleVariant {
    Zero = 0,
    One = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Alpha = 5,
}

#[repr(C)]
pub struct DivisionTextureChannelsSwizzle {
    pub red: DivisionTextureChannelSwizzleVariant,
    pub green: DivisionTextureChannelSwizzleVariant,
    pub blue: DivisionTextureChannelSwizzleVariant,
    pub alpha: DivisionTextureChannelSwizzleVariant,
}

#[repr(C)]
pub struct DivisionTextureDescriptor {
    pub channels_swizzle: DivisionTextureChannelsSwizzle,
    pub texture_format: DivisionTextureFormat,
    pub width: u32,
    pub height: u32,
    pub has_channels_swizzle: bool,
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