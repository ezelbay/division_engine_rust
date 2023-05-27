use std::ffi::c_ulong;
use bitflags::bitflags;

use super::context::DivisionContext;

#[repr(C)]
pub struct IdWithBinding {
    pub id: u32,
    pub shader_location: u32,
}

bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Clone, Copy)]
    pub struct ColorMask: i32 {
        const None = 0;
        const R = 1 << 0;
        const G = 1 << 1;
        const B = 1 << 2;
        const A = 1 << 3;
        const RGB = Self::R.bits() | Self::G.bits() | Self::B.bits();
        const RGBA = Self::RGB.bits() | Self::A.bits();
    }
}


bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Clone, Copy)]
    pub struct RenderPassCapabilityMask: i32 {
        const None = 0;
        const AlphaBlend = 1;
    }
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum AlphaBlend {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    SrcAlpha = 3,
    SrcAlphaSaturate = 4,
    DstColor = 5,
    DstAlpha = 6,
    ConstantColor = 7,
    ConstantAlpha = 8,
    OneMinusSrcColor = 9,
    OneMinusSrcAlpha = 10,
    OneMinusDstColor = 11,
    OneMinusDstAlpha = 12,
    OneMinusConstantColor = 13,
    OneMinusConstantAlpha = 14,
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum AlphaBlendOperation {
    Add = 1,
    Subtract = 2,
    ReverseSubtract = 3,
    Min = 4,
    Max = 5,
}

#[repr(C)]
pub struct AlphaBlendingOptions {
    pub src: AlphaBlend,
    pub dst: AlphaBlend,
    pub operation: AlphaBlendOperation,
    pub constant_blend_color: [f32; 4],
}

#[repr(C)]
pub struct RenderPassDescriptor {
    pub alpha_blending_options: AlphaBlendingOptions,

    pub first_vertex: c_ulong,
    pub vertex_count: c_ulong,
    pub instance_count: c_ulong,
    pub uniform_vertex_buffers: *const u32,
    pub uniform_vertex_buffer_count: i32,
    pub uniform_fragment_buffers: *const u32,
    pub uniform_fragment_buffer_count: i32,
    pub fragment_textures: *const IdWithBinding,
    pub fragment_texture_count: i32,
    pub vertex_buffer: u32,
    pub shader_program: u32,
    pub capabilities_mask: RenderPassCapabilityMask,
    pub color_mask: ColorMask,
}

extern "C" {
    pub fn division_engine_render_pass_alloc(
        ctx: *mut DivisionContext,
        descriptor: RenderPassDescriptor,
        out_render_pass_id: *mut u32,
    ) -> bool;

    pub fn division_engine_render_pass_free(ctx: *mut DivisionContext, render_pass_id: u32);
}
