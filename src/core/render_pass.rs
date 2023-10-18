use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::null,
    usize,
};

use division_math::Vector4;

use super::{
    context::Error,
    ffi::{
        context::DivisionContext,
        render_pass_descriptor::{
            division_engine_render_pass_descriptor_alloc,
            division_engine_render_pass_descriptor_borrow,
            division_engine_render_pass_descriptor_free,
            division_engine_render_pass_descriptor_return, DivisionColorMask,
            DivisionRenderPassDescriptorCapabilityMask,
        },
        render_pass_instance::division_engine_render_pass_instance_draw,
    },
    Context, DivisionId,
};

pub use super::ffi::{
    render_pass_descriptor::{
        DivisionAlphaBlend as AlphaBlend,
        DivisionAlphaBlendOperation as AlphaBlendOperation,
        DivisionAlphaBlendingOptions as AlphaBlendingOptions,
        DivisionColorMask as ColorMask,
        DivisionRenderPassDescriptor as RenderPassDescriptor,
        DivisionRenderPassDescriptorCapabilityMask as RenderPassDescriptorCapabilityMask,
    },
    render_pass_instance::{
        DivisionIdWithBinding as IdWithBinding,
        DivisionRenderPassInstance as RenderPassInstance,
        DivisionRenderPassInstanceCapabilityMask as RenderPassIsntanceCapabilityMask,
    },
};

pub struct BorrowedRenderPass<'a> {
    render_pass: &'a mut RenderPassDescriptor,
    ctx: *mut DivisionContext,
    render_pass_id: u32,
}

pub struct RenderPassInstanceOwned {
    pub instance: RenderPassInstance<'static>,
    pub textures: Vec<IdWithBinding>,
    pub vertex_uniforms: Vec<IdWithBinding>,
    pub fragment_uniforms: Vec<IdWithBinding>,
}

impl RenderPassDescriptor {
    pub fn with_shader_and_vertex_buffer(
        shader_program: DivisionId,
        vertex_buffer_id: DivisionId,
    ) -> RenderPassDescriptor {
        RenderPassDescriptor {
            alpha_blending_options: AlphaBlendingOptions {
                src: AlphaBlend::One,
                dst: AlphaBlend::Zero,
                operation: AlphaBlendOperation::Add,
                constant_blend_color: [0., 0., 0., 0.],
            },
            capabilities_mask: RenderPassDescriptorCapabilityMask::None,
            color_mask: ColorMask::RGB,
            shader_program,
            vertex_buffer_id,
        }
    }

    pub fn alpha_blending(
        mut self,
        src: AlphaBlend,
        dst: AlphaBlend,
        operation: AlphaBlendOperation,
    ) -> Self {
        assert!(!has_constant_color(src) && !has_constant_color(dst));

        let blend_options = &mut self.alpha_blending_options;
        blend_options.src = src;
        blend_options.dst = dst;
        blend_options.operation = operation;

        self.capabilities_mask |= DivisionRenderPassDescriptorCapabilityMask::AlphaBlend;
        self
    }

    pub fn alpha_blending_with_constant_color(
        mut self,
        src: AlphaBlend,
        dst: AlphaBlend,
        operation: AlphaBlendOperation,
        color: Vector4,
    ) -> Self {
        assert!(has_constant_color(src) || has_constant_color(dst));

        let blend_options = &mut self.alpha_blending_options;
        blend_options.src = src;
        blend_options.dst = dst;
        blend_options.operation = operation;
        blend_options.constant_blend_color = [color.r(), color.g(), color.b(), color.a()];

        self.capabilities_mask |= DivisionRenderPassDescriptorCapabilityMask::AlphaBlend;
        self
    }

    pub fn color_mask(&mut self, color_mask: DivisionColorMask) -> &mut Self {
        self.color_mask = color_mask;

        self
    }
}

impl<'a> RenderPassInstance<'a> {
    pub fn new(descriptor_id: DivisionId) -> RenderPassInstance<'a> {
        RenderPassInstance {
            first_vertex: 0,
            first_instance: 0,
            vertex_count: 0,
            index_count: 0,
            instance_count: 0,
            uniform_vertex_buffers: null(),
            uniform_fragment_buffers: null(),
            fragment_textures: null(),
            uniform_vertex_buffer_count: 0,
            uniform_fragment_buffer_count: 0,
            fragment_texture_count: 0,
            render_pass_descriptor_id: descriptor_id,
            capabilities_mask: RenderPassIsntanceCapabilityMask::None,
            lifetime_marker: PhantomData::default(),
        }
    }

    pub fn vertices(mut self, vertex_count: usize, index_count: usize) -> Self {
        self.vertex_count = vertex_count as u32;
        self.index_count = index_count as u32;

        self
    }

    pub fn first_vertex(mut self, first_vertex: usize) -> Self {
        self.first_vertex = first_vertex as u32;

        self
    }

    pub fn enable_instancing(mut self) -> Self {
        self.capabilities_mask |= RenderPassIsntanceCapabilityMask::InstancedRendering;

        self
    }

    pub fn instances(mut self, instance_count: usize) -> Self {
        self.instance_count = instance_count as u32;
        self.capabilities_mask |= RenderPassIsntanceCapabilityMask::InstancedRendering;

        self
    }

    pub fn set_vertex_uniform_buffers(
        &mut self,
        vertex_uniforms: &'a [IdWithBinding],
    ) -> &mut Self {
        self.uniform_vertex_buffers = vertex_uniforms.as_ptr();
        self.uniform_vertex_buffer_count = vertex_uniforms.len() as i32;

        self
    }

    pub fn set_fragment_uniform_buffers(
        &mut self,
        fragment_uniforms: &'a [IdWithBinding],
    ) -> &mut Self {
        self.uniform_fragment_buffers = fragment_uniforms.as_ptr();
        self.uniform_fragment_buffer_count = fragment_uniforms.len() as i32;

        self
    }

    pub fn set_fragment_textures(&mut self, texture_ids: &'a [IdWithBinding]) -> &mut Self {
        self.fragment_textures = texture_ids.as_ptr();
        self.fragment_texture_count = texture_ids.len() as i32;

        self
    }
}

impl RenderPassInstanceOwned {
    pub fn new(instance: RenderPassInstance<'static>) -> RenderPassInstanceOwned {
        RenderPassInstanceOwned {
            instance,
            textures: Vec::new(),
            vertex_uniforms: Vec::new(),
            fragment_uniforms: Vec::new(),
        }
    }

    pub fn vertex_uniform_buffers(mut self, uniforms: &[IdWithBinding]) -> Self {
        self.vertex_uniforms.extend_from_slice(uniforms);
        self.instance.uniform_vertex_buffers = self.vertex_uniforms.as_ptr();
        self.instance.uniform_vertex_buffer_count = self.vertex_uniforms.len() as i32;

        self
    }

    pub fn fragment_uniform_buffers(mut self, uniforms: &[IdWithBinding]) -> Self {
        self.fragment_uniforms.extend_from_slice(uniforms);
        self.instance.uniform_fragment_buffers = self.fragment_uniforms.as_ptr();
        self.instance.uniform_fragment_buffer_count = self.fragment_uniforms.len() as i32;

        self
    }

    pub fn fragment_textures(mut self, textures: &[IdWithBinding]) -> Self {
        self.textures.extend_from_slice(textures);
        self.instance.fragment_textures = self.textures.as_ptr();
        self.instance.fragment_texture_count = self.textures.len() as i32;

        self
    }
}

impl Context {
    pub fn create_render_pass_descriptor(
        &mut self,
        descriptor: &RenderPassDescriptor,
    ) -> Result<DivisionId, Error> {
        let mut pass_id = 0;
        unsafe {
            if !division_engine_render_pass_descriptor_alloc(
                self,
                descriptor,
                &mut pass_id,
            ) {
                return Err(Error::Core("Failed to create a render pass".to_string()));
            }
        }

        Ok(pass_id)
    }

    pub fn borrow_render_pass_descriptor_mut(
        &mut self,
        render_pass_id: DivisionId,
    ) -> BorrowedRenderPass {
        unsafe {
            BorrowedRenderPass {
                ctx: &mut *self,
                render_pass_id,
                render_pass: &mut *division_engine_render_pass_descriptor_borrow(
                    &mut *self,
                    render_pass_id,
                ),
            }
        }
    }

    pub fn draw_render_passes(&mut self, instances: &[RenderPassInstance]) {
        unsafe {
            division_engine_render_pass_instance_draw(
                self,
                instances.as_ptr(),
                instances.len() as u32,
            );
        }
    }

    pub fn draw_single_render_pass(&mut self, instance: &RenderPassInstance) {
        unsafe {
            division_engine_render_pass_instance_draw(self, instance, 1);
        }
    }

    #[inline(always)]
    pub fn delete_render_pass_descriptor(&mut self, render_pass_id: DivisionId) {
        unsafe {
            division_engine_render_pass_descriptor_free(&mut *self, render_pass_id);
        }
    }
}

impl IdWithBinding {
    pub fn new(id: u32, shader_binding: u32) -> IdWithBinding {
        IdWithBinding { id, shader_binding }
    }
}

fn has_constant_color(blend: AlphaBlend) -> bool {
    blend == AlphaBlend::ConstantAlpha
        || blend == AlphaBlend::ConstantColor
        || blend == AlphaBlend::OneMinusConstantAlpha
        || blend == AlphaBlend::OneMinusConstantColor
}

impl<'a> Drop for BorrowedRenderPass<'a> {
    fn drop(&mut self) {
        unsafe {
            division_engine_render_pass_descriptor_return(
                self.ctx,
                self.render_pass_id,
                self.render_pass,
            );
        }
    }
}

impl<'a> Deref for BorrowedRenderPass<'a> {
    type Target = RenderPassDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.render_pass
    }
}

impl<'a> DerefMut for BorrowedRenderPass<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render_pass
    }
}

impl Deref for RenderPassInstanceOwned {
    type Target = RenderPassInstance<'static>;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl DerefMut for RenderPassInstanceOwned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}
