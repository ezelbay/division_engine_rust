use std::path::Path;

use division_math::{Vector2, Vector4};

use crate::core::{
    AlphaBlend, AlphaBlendOperation, Context, DivisionId, IdWithBinding, RenderTopology,
    ShaderVariableType, TextureDescriptor, TextureFormat, VertexAttributeDescriptor,
    VertexBufferData, VertexData,
};

use super::{decoration::Decoration, rect::Rect};

pub struct RectDrawSystem {
    shader_id: DivisionId,
    white_pixel_texture_id: DivisionId,
    uniform_buffer_id: DivisionId,
    render_pass_ids: Vec<DivisionId>,
    render_pass_to_texture_ids: Vec<DivisionId>,
    render_pass_rects: Vec<RenderPassRectsView>,

    rects: Vec<DrawableRect>,
    free_rects: Vec<usize>,
}

pub struct DrawableRect {
    pub rect: Rect,
    pub decoration: Decoration,
}

#[repr(transparent)]
struct RenderPassRectsView {
    rects: Vec<usize>,
}

#[repr(C, packed)]
#[derive(Clone, Copy, VertexData)]
struct RectVertexData {
    #[location(0)]
    vert_pos: Vector2,
    #[location(1)]
    uv: Vector2,
}

#[repr(C, packed)]
#[derive(Clone, Copy, VertexData)]
struct RectInstanceData {
    #[location(2)]
    size: Vector2,
    #[location(3)]
    position: Vector2,
    #[location(4)]
    color: Vector4,
    #[location(5)]
    trbl_border_radius: Vector4,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct UniformData {
    size: Vector2,
}

pub const RECT_CAPACITY: usize = 128;
pub const VERTEX_PER_RECT: usize = 4;
pub const INDEX_PER_RECT: usize = 6;

impl RectDrawSystem {
    pub fn new(context: &mut Context) -> RectDrawSystem {
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources")
                    .join("shaders")
                    .join("canvas")
                    .join("rect"),
            )
            .unwrap();

        let white_pixel_texture_id = context
            .create_texture_buffer_from_data(
                &TextureDescriptor::new(1, 1, TextureFormat::RGBA32Uint),
                &[255u8; 4],
            )
            .unwrap();

        let uniform_buffer_id = context
            .create_uniform_buffer_with_size_of::<UniformData>()
            .unwrap();

        RectDrawSystem {
            shader_id,
            white_pixel_texture_id,
            uniform_buffer_id,
            render_pass_ids: Vec::new(),
            render_pass_rects: Vec::new(),
            render_pass_to_texture_ids: Vec::new(),
            rects: Vec::new(),
            free_rects: Vec::new(),
        }
    }

    pub fn white_texture_id(&self) -> DivisionId {
        self.white_pixel_texture_id
    }

    pub fn add_rect(
        &mut self,
        context: &mut Context,
        drawable_rect: DrawableRect,
    ) -> DivisionId {
        let decoration = drawable_rect.decoration;

        let rect_index = if let Some(idx) = self.free_rects.pop() {
            idx
        } else {
            self.rects.len()
        };
        self.rects.insert(rect_index, drawable_rect);

        let rect_texture = decoration.texture;
        let target_pass_index = match self
            .render_pass_to_texture_ids
            .binary_search(&rect_texture)
        {
            Ok(target_pass_idx) => target_pass_idx,
            Err(target_pass_idx) => {
                let pass_id = self.create_new_render_pass_with_vertex_buffer(
                    context,
                    false,
                    self.white_pixel_texture_id,
                );

                self.render_pass_ids.insert(target_pass_idx, pass_id);
                self.render_pass_rects
                    .insert(target_pass_idx, RenderPassRectsView { rects: Vec::new() });
                self.render_pass_to_texture_ids
                    .insert(target_pass_idx, rect_texture);
                target_pass_idx
            }
        };

        let render_pass_rects = &mut self.render_pass_rects[target_pass_index].rects;
        let render_pass_rect_idx = match render_pass_rects.binary_search(&rect_index) {
            Ok(idx) | Err(idx) => idx,
        };
        render_pass_rects.insert(render_pass_rect_idx, rect_index);

        rect_index as u32
    }

    pub fn remove_rect(&mut self, rect_id: DivisionId) {
        let rect_id = rect_id as usize;

        for render_pass_idx in 0..self.render_pass_ids.len()
        {
            let rects = &mut self.render_pass_rects[render_pass_idx].rects;

            if let Ok(idx_to_remove) = rects.binary_search(&rect_id) {
                rects.remove(idx_to_remove);
            }
        }

        match self.free_rects.binary_search(&rect_id) {
            Err(idx) => self.free_rects.insert(idx, rect_id),
            Ok(_) => panic!("Id {rect_id} is already free!")
        }
    }

    pub fn get_rect(&self, rect_id: DivisionId) -> &DrawableRect {
        &self.rects[rect_id as usize]
    }

    pub fn get_rect_mut(&mut self, rect_id: DivisionId) -> &mut DrawableRect {
        &mut self.rects[rect_id as usize]
    }

    pub fn create_new_render_pass_with_vertex_buffer(
        &self,
        context: &mut Context,
        flip_vertical: bool,
        texture_buffer_id: DivisionId,
    ) -> DivisionId {
        let vertex_buffer_id = Self::make_vertex_buffer(context);
        Self::generate_rect_drawer_vertex_data(context, vertex_buffer_id, flip_vertical);

        context
            .render_pass_builder()
            .shader(self.shader_id)
            .fragment_textures(&[IdWithBinding::new(texture_buffer_id, 0)])
            .vertex_uniform_buffers(&[IdWithBinding::new(self.uniform_buffer_id, 1)])
            .fragment_uniform_buffers(&[IdWithBinding::new(self.uniform_buffer_id, 1)])
            .vertex_buffer(vertex_buffer_id, VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing()
            .alpha_blending(
                AlphaBlend::SrcAlpha,
                AlphaBlend::OneMinusSrcAlpha,
                AlphaBlendOperation::Add,
            )
            .build()
            .unwrap()
    }

    pub fn update(&mut self, context: &mut Context, canvas_size: Vector2) {
        self.update_canvas_size_uniform(context, canvas_size);

        for (render_pass_idx, rect_views) in self.render_pass_rects.iter().enumerate() {
            let render_pass_id = self.render_pass_ids[render_pass_idx];
            let vertex_buffer_id = {
                let render_pass = context.borrow_render_pass_mut(render_pass_id);
                render_pass.vertex_buffer
            };

            let instance_count = {
                let mut instance_count = 0;
                let vertex_buffer = Self::get_rect_drawer_data(context, vertex_buffer_id);

                for draw_rect in rect_views.rects.iter().map(|r| &self.rects[*r]) {
                    let instance_data =
                        &mut vertex_buffer.per_instance_data[instance_count];
                    instance_data.color = draw_rect.decoration.color.into();
                    instance_data.position = draw_rect.rect.bottom_left();
                    instance_data.size = draw_rect.rect.size();
                    instance_data.trbl_border_radius =
                        draw_rect.decoration.border_radius.into();

                    instance_count += 1;
                }

                instance_count
            };

            let mut render_pass = context.borrow_render_pass_mut(render_pass_id);
            render_pass.instance_count = instance_count as u64;
        }
    }

    fn update_canvas_size_uniform(&self, context: &mut Context, size: Vector2) {
        let ub_data = context.uniform_buffer_data::<UniformData>(self.uniform_buffer_id);
        *(ub_data.data) = UniformData { size };
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_texture_buffer(self.white_pixel_texture_id);
        context.delete_uniform_buffer(self.uniform_buffer_id);
        context.delete_shader_program(self.shader_id);

        for pass_id in self.render_pass_ids.iter() {
            let (vertex_buffer, texture_id) = {
                let pass = context.borrow_render_pass_mut(*pass_id);
                (pass.vertex_buffer, pass.fragment_textures_slice()[0].id)
            };

            context.delete_vertex_buffer(vertex_buffer);

            if texture_id != self.white_pixel_texture_id {
                context.delete_texture_buffer(texture_id);
            }
        }
    }

    fn make_vertex_buffer(context: &mut Context) -> DivisionId {
        context
            .create_vertex_buffer::<RectVertexData, RectInstanceData>(
                VERTEX_PER_RECT,
                INDEX_PER_RECT,
                RECT_CAPACITY,
                RenderTopology::Triangles,
            )
            .unwrap()
    }

    fn generate_rect_drawer_vertex_data(
        context: &mut Context,
        vertex_buffer_id: DivisionId,
        flip_vertical: bool,
    ) {
        let data = Self::get_rect_drawer_data(context, vertex_buffer_id);
        let (uv_top, uv_bottom) = if flip_vertical { (0., 1.) } else { (1., 0.) };

        let vertex_data = [
            RectVertexData {
                vert_pos: Vector2::new(0., 1.),
                uv: Vector2::new(0., uv_top),
            },
            RectVertexData {
                vert_pos: Vector2::new(0., 0.),
                uv: Vector2::new(0., uv_bottom),
            },
            RectVertexData {
                vert_pos: Vector2::new(1., 0.),
                uv: Vector2::new(1., uv_bottom),
            },
            RectVertexData {
                vert_pos: Vector2::new(1., 1.),
                uv: Vector2::new(1., uv_top),
            },
        ];
        let indices = [0, 1, 2, 2, 3, 0];

        data.vertex_indices.copy_from_slice(&indices);
        data.per_vertex_data.copy_from_slice(&vertex_data);
    }

    #[inline(always)]
    fn get_rect_drawer_data(
        context: &mut Context,
        vertex_buffer_id: DivisionId,
    ) -> VertexBufferData<RectVertexData, RectInstanceData> {
        context.vertex_buffer_data(vertex_buffer_id)
    }
}

impl DrawableRect {
    pub fn new(rect: Rect, decoration: Decoration) -> DrawableRect {
        DrawableRect { rect, decoration }
    }
}
