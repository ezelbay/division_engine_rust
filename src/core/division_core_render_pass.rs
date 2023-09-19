use std::{ptr::null, usize};

use division_math::Vector4;

use super::{
    c_interface::{
        context::DivisionContext,
        render_pass::{
            division_engine_render_pass_alloc, division_engine_render_pass_borrow,
            division_engine_render_pass_free, division_engine_render_pass_return,
            AlphaBlendingOptions, ColorMask, RenderPassCapabilityMask, RenderPassDescriptor,
        },
    },
    DivisionCore, DivisionError, DivisionId,
};

pub use super::c_interface::render_pass::AlphaBlend;
pub use super::c_interface::render_pass::AlphaBlendOperation;
pub use super::c_interface::render_pass::IdWithBinding;

pub struct RenderPassBuilder {
    ctx: *mut DivisionContext,
    descriptor: RenderPassDescriptor,
}

pub struct BorrowedRenderPass<'a> {
    ctx: *mut DivisionContext,
    render_pass_id: u32,
    pub render_pass: &'a mut RenderPassDescriptor,
}

impl DivisionCore {
    pub fn render_pass_builder(&self) -> RenderPassBuilder {
        RenderPassBuilder {
            ctx: self.ctx,
            descriptor: RenderPassDescriptor {
                alpha_blending_options: AlphaBlendingOptions {
                    src: AlphaBlend::One,
                    dst: AlphaBlend::Zero,
                    operation: AlphaBlendOperation::Add,
                    constant_blend_color: [0., 0., 0., 0.],
                },
                first_vertex: 0,
                vertex_count: 0,
                index_count: 0,
                instance_count: 0,
                uniform_vertex_buffers: null(),
                uniform_vertex_buffer_count: 0,
                uniform_fragment_buffers: null(),
                uniform_fragment_buffer_count: 0,
                fragment_textures: null(),
                fragment_texture_count: 0,
                vertex_buffer: 0,
                shader_program: 0,
                capabilities_mask: RenderPassCapabilityMask::None,
                color_mask: ColorMask::RGB,
            },
        }
    }

    pub fn borrow_render_pass_mut_ptr(&self, render_pass_id: DivisionId) -> BorrowedRenderPass {
        unsafe {
            BorrowedRenderPass {
                ctx: self.ctx,
                render_pass_id,
                render_pass: &mut *division_engine_render_pass_borrow(self.ctx, render_pass_id),
            }
        }
    }

    #[inline(always)]
    pub fn delete_render_pass(&mut self, render_pass_id: DivisionId) {
        unsafe {
            division_engine_render_pass_free(self.ctx, render_pass_id);
        }
    }
}

impl RenderPassBuilder {
    pub fn shader(mut self, shader_id: DivisionId) -> Self {
        self.descriptor.shader_program = shader_id;
        self
    }

    pub fn vertex_buffer(
        mut self,
        vertex_buffer_id: DivisionId,
        vertex_count: usize,
        index_count: usize,
    ) -> Self {
        self.descriptor.vertex_count = vertex_count as u64;
        self.descriptor.index_count = index_count as u64;
        self.descriptor.vertex_buffer = vertex_buffer_id;

        self
    }

    pub fn first_vertex(mut self, first_vertex: usize) -> Self {
        self.descriptor.first_vertex = first_vertex as u64;

        self
    }

    pub fn enable_instancing(mut self) -> Self {
        self.descriptor.capabilities_mask |= RenderPassCapabilityMask::InstancedRendering;

        self
    }

    pub fn instances(#[allow(unused_mut)] mut self, instance_count: usize) -> Self {
        self.descriptor.instance_count = instance_count as u64;
        self.descriptor.capabilities_mask |= RenderPassCapabilityMask::InstancedRendering;

        self
    }

    pub fn alpha_blending(
        mut self,
        src: AlphaBlend,
        dst: AlphaBlend,
        operation: AlphaBlendOperation,
    ) -> Self {
        assert!(!has_constant_color(src) && !has_constant_color(dst));

        let blend_options = &mut self.descriptor.alpha_blending_options;
        blend_options.src = src;
        blend_options.dst = dst;
        blend_options.operation = operation;

        self.descriptor.capabilities_mask |= RenderPassCapabilityMask::AlphaBlend;
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

        let blend_options = &mut self.descriptor.alpha_blending_options;
        blend_options.src = src;
        blend_options.dst = dst;
        blend_options.operation = operation;
        blend_options.constant_blend_color = [color.r(), color.g(), color.b(), color.a()];

        self.descriptor.capabilities_mask |= RenderPassCapabilityMask::AlphaBlend;
        self
    }

    pub fn vertex_uniform_buffers(mut self, vertex_uniforms: &[IdWithBinding]) -> Self {
        self.descriptor.uniform_vertex_buffers = vertex_uniforms.as_ptr();
        self.descriptor.uniform_vertex_buffer_count = vertex_uniforms.len() as i32;

        self
    }

    pub fn fragment_uniform_buffers(mut self, fragment_uniforms: &[IdWithBinding]) -> Self {
        self.descriptor.uniform_fragment_buffers = fragment_uniforms.as_ptr();
        self.descriptor.uniform_fragment_buffer_count = fragment_uniforms.len() as i32;

        self
    }

    pub fn fragment_textures(mut self, texture_ids: &[IdWithBinding]) -> Self {
        self.descriptor.fragment_textures = texture_ids.as_ptr();
        self.descriptor.fragment_texture_count = texture_ids.len() as i32;

        self
    }

    pub fn build(#[allow(unused_mut)] mut self) -> Result<DivisionId, DivisionError> {
        let mut pass_id = 0;
        unsafe {
            if !division_engine_render_pass_alloc(self.ctx, self.descriptor, &mut pass_id) {
                return Err(DivisionError::Core(
                    "Failed to create a render pass".to_string(),
                ));
            }
        }

        Ok(pass_id)
    }
}

impl IdWithBinding {
    pub fn new(id: u32, shader_binding: u32) -> IdWithBinding {
        IdWithBinding { id, shader_binding }
    }
}

fn has_constant_color(blend: AlphaBlend) -> bool {
    blend != AlphaBlend::ConstantAlpha
        && blend != AlphaBlend::ConstantColor
        && blend != AlphaBlend::OneMinusConstantAlpha
        && blend != AlphaBlend::OneMinusConstantColor
}

impl<'a> Drop for BorrowedRenderPass<'a> {
    fn drop(&mut self) {
        unsafe {
            division_engine_render_pass_return(self.ctx, self.render_pass_id, self.render_pass);
        }
    }
}
