use std::{
    alloc::Layout,
    ops::{Deref, DerefMut},
    ptr::null_mut,
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
        render_pass_instance::{
            division_engine_render_pass_instance_draw, DivisionColor,
        },
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

#[repr(transparent)]
pub struct RenderPassInstanceOwned {
    pub instance: RenderPassInstance,
}

pub trait RenderPassConvert<'a> {
    fn as_instances_slice(self) -> &'a [RenderPassInstance];
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

impl RenderPassInstance {
    pub fn new(descriptor_id: DivisionId) -> RenderPassInstance {
        RenderPassInstance {
            first_vertex: 0,
            first_instance: 0,
            vertex_count: 0,
            index_count: 0,
            instance_count: 0,
            uniform_vertex_buffers: null_mut(),
            uniform_fragment_buffers: null_mut(),
            fragment_textures: null_mut(),
            uniform_vertex_buffer_count: 0,
            uniform_fragment_buffer_count: 0,
            fragment_texture_count: 0,
            render_pass_descriptor_id: descriptor_id,
            capabilities_mask: RenderPassIsntanceCapabilityMask::None,
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

    pub unsafe fn set_uniform_vertex_buffers<'a>(
        &'a mut self,
        buffers: &'a [IdWithBinding],
    ) {
        self.uniform_vertex_buffer_count = buffers.len() as i32;
        self.uniform_vertex_buffers = buffers.as_ptr() as *mut IdWithBinding;
    }

    pub unsafe fn set_uniform_fragment_buffers<'a>(
        &'a mut self,
        buffers: &'a [IdWithBinding],
    ) {
        self.uniform_fragment_buffer_count = buffers.len() as i32;
        self.uniform_fragment_buffers = buffers.as_ptr() as *mut IdWithBinding;
    }

    pub unsafe fn set_uniform_fragment_textures<'a>(
        &'a mut self,
        buffers: &'a [IdWithBinding]
    ) {
        self.fragment_texture_count = buffers.len() as i32;
        self.fragment_textures = buffers.as_ptr() as *mut IdWithBinding;
    }
}

impl RenderPassInstanceOwned {
    pub fn new(instance: RenderPassInstance) -> Self {
        Self { instance }
    }

    pub fn uniform_vertex_buffers(mut self, buffers: &[IdWithBinding]) -> Self {
        self.uniform_vertex_buffers = Self::alloc_buffers(
            self.uniform_vertex_buffers,
            self.uniform_vertex_buffer_count,
            buffers,
        );
        self.uniform_vertex_buffer_count = buffers.len() as i32;

        self
    }

    pub fn uniform_fragment_buffers(mut self, buffers: &[IdWithBinding]) -> Self {
        self.uniform_fragment_buffers = Self::alloc_buffers(
            self.uniform_fragment_buffers,
            self.uniform_fragment_buffer_count,
            buffers,
        );
        self.uniform_fragment_buffer_count = buffers.len() as i32;
        self
    }

    pub fn fragment_textures(mut self, textures: &[IdWithBinding]) -> Self {
        self.fragment_textures = Self::alloc_buffers(
            self.fragment_textures,
            self.fragment_texture_count,
            textures,
        );
        self.fragment_texture_count = textures.len() as i32;

        self
    }

    fn alloc_buffers(
        ptr: *mut IdWithBinding,
        prev_size: i32,
        buffers: &[IdWithBinding],
    ) -> *mut IdWithBinding {
        unsafe {
            let new_size = buffers.len();
            let prev_size = prev_size as usize;
            let ptr = if prev_size < new_size {
                std::alloc::dealloc(
                    ptr as *mut u8,
                    Layout::array::<IdWithBinding>(prev_size).unwrap_unchecked(),
                );
                std::alloc::alloc(
                    Layout::array::<IdWithBinding>(new_size).unwrap_unchecked(),
                ) as *mut IdWithBinding
            } else {
                ptr
            };

            ptr.copy_from_nonoverlapping(buffers.as_ptr(), buffers.len());
            ptr
        }
    }

    fn dealloc_buffers(ptr: *const IdWithBinding, len: i32) {
        unsafe {
            std::alloc::dealloc(
                ptr as *mut u8,
                Layout::array::<IdWithBinding>(len as usize).unwrap_unchecked(),
            )
        }
    }
}

impl Drop for RenderPassInstanceOwned {
    fn drop(&mut self) {
        Self::dealloc_buffers(
            self.uniform_vertex_buffers,
            self.uniform_vertex_buffer_count,
        );
        Self::dealloc_buffers(
            self.uniform_fragment_buffers,
            self.uniform_fragment_buffer_count,
        );
        Self::dealloc_buffers(self.fragment_textures, self.fragment_texture_count);
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

    pub fn draw_render_passes(
        &mut self,
        clear_color: Vector4,
        instances: &[RenderPassInstance],
    ) {
        unsafe {
            division_engine_render_pass_instance_draw(
                self,
                &clear_color as *const Vector4 as *const DivisionColor,
                instances.as_ptr(),
                instances.len() as u32,
            );
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
    type Target = RenderPassInstance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl DerefMut for RenderPassInstanceOwned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

impl<'a> RenderPassConvert<'a> for &'a [RenderPassInstanceOwned] {
    fn as_instances_slice(self) -> &'a [RenderPassInstance] {
        unsafe { std::mem::transmute(self) }
    }
}
