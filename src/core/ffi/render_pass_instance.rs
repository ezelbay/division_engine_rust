use std::marker::PhantomData;

use bitflags::bitflags;

use super::context::DivisionContext;

bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Clone, Copy)]
    pub struct DivisionRenderPassInstanceCapabilityMask: i32 {
        const None = 0;
        const InstancedRendering = 1 << 0;
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct DivisionIdWithBinding {
    pub id: u32,
    pub shader_binding: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct DivisionRenderPassInstance<'a> {
    pub first_vertex: u32,
    pub first_instance: u32,
    pub vertex_count: u32,
    pub index_count: u32,
    pub instance_count: u32,
    pub uniform_vertex_buffers: *const DivisionIdWithBinding,
    pub uniform_fragment_buffers: *const DivisionIdWithBinding,
    pub fragment_textures: *const DivisionIdWithBinding,

    pub uniform_vertex_buffer_count: i32,
    pub uniform_fragment_buffer_count: i32,
    pub fragment_texture_count: i32,
    pub render_pass_descriptor_id: u32,
    pub capabilities_mask: DivisionRenderPassInstanceCapabilityMask,

    pub(crate) lifetime_marker: PhantomData<&'a DivisionIdWithBinding>
}

extern "C" {
    pub fn division_engine_render_pass_instance_draw(
        ctx: *mut DivisionContext,
        render_pass_instances: *const DivisionRenderPassInstance,
        render_pass_instance_count: u32
    );
}