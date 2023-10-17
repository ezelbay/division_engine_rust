/* 
   TODO: change internal representation from DrawableRect to RectInstanceData 
         to simplified copying to the memcpy
*/

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
    render_pass_to_rects: Vec<RenderPassRects>,

    rects: Vec<RectInRenderPass>,

    free_rect_indices: Vec<usize>,
    free_render_pass_indices: Vec<usize>,
}

pub struct DrawableRect {
    pub rect: Rect,
    pub decoration: Decoration,
}

#[derive(Clone, Copy)]
struct RectInRenderPass {
    render_pass_idx: usize,
    rect_idx: usize,
}

struct RenderPassRects {
    rects: Vec<DrawableRect>,
    free_rects: Vec<usize>,
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

type RectVertexBufferData<'a> = VertexBufferData<'a, RectVertexData, RectInstanceData>;

impl RectDrawSystem {
    const NULL_ID: DivisionId = DivisionId::MAX;

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
            render_pass_to_rects: Vec::new(),
            render_pass_to_texture_ids: Vec::new(),
            rects: Vec::new(),
            free_rect_indices: Vec::new(),
            free_render_pass_indices: Vec::new(),
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
        let rect_texture = decoration.texture;

        let render_pass_idx =
            self.find_or_add_suitable_render_pass(context, rect_texture);

        let render_pass_rects = &mut self.render_pass_to_rects[render_pass_idx];
        let rect_idx = if let Some(idx) = render_pass_rects.free_rects.pop() {
            render_pass_rects.rects[idx] = drawable_rect;
            idx
        } else {
            render_pass_rects.rects.push(drawable_rect);
            render_pass_rects.rects.len() - 1
        };

        let rect_in_render_pass = RectInRenderPass {
            render_pass_idx,
            rect_idx,
        };
        let user_rect_id = if let Some(idx) = self.free_rect_indices.pop() {
            self.rects[idx] = rect_in_render_pass;
            idx
        } else {
            self.rects.push(rect_in_render_pass);
            self.rects.len() - 1
        };

        user_rect_id as u32
    }

    fn find_or_add_suitable_render_pass(
        &mut self,
        context: &mut Context,
        texture_buffer_id: DivisionId,
    ) -> usize {
        match self
            .render_pass_to_texture_ids
            .binary_search(&texture_buffer_id)
        {
            Ok(target_pass_idx) => target_pass_idx,
            Err(target_pass_idx) => {
                let target_pass_idx =
                    if let Some(idx) = self.free_render_pass_indices.pop() {
                        idx
                    } else {
                        target_pass_idx
                    };

                let pass_id = self.create_new_render_pass_with_vertex_buffer(
                    context,
                    false,
                    texture_buffer_id,
                );

                self.render_pass_ids.insert(target_pass_idx, pass_id);
                self.render_pass_to_rects.insert(
                    target_pass_idx,
                    RenderPassRects {
                        rects: Vec::new(),
                        free_rects: Vec::new(),
                    },
                );
                self.render_pass_to_texture_ids
                    .insert(target_pass_idx, texture_buffer_id);
                target_pass_idx
            }
        }
    }

    pub fn remove_rect(&mut self, context: &mut Context, rect_id: DivisionId) {
        let rect_id = rect_id as usize;
        let rect_in_render_pass = self.rects[rect_id];

        let render_pass_index = rect_in_render_pass.render_pass_idx;
        let pass_rects = &mut self.render_pass_to_rects[render_pass_index];

        let render_pass_free_rect_idx = match pass_rects
            .free_rects
            .binary_search(&rect_in_render_pass.rect_idx)
        {
            Ok(idx) | Err(idx) => idx,
        };

        pass_rects
            .free_rects
            .insert(render_pass_free_rect_idx, rect_in_render_pass.rect_idx);

        let actual_pass_rect = pass_rects.rects.len() - pass_rects.free_rects.len();
        if actual_pass_rect <= 0 {
            self.delete_render_pass_with_local_buffers(
                context,
                self.render_pass_ids[render_pass_index],
            );
            self.render_pass_to_texture_ids[render_pass_index] = Self::NULL_ID;

            let rp_insert_idx = match self
                .free_render_pass_indices
                .binary_search(&render_pass_index)
            {
                Ok(idx) | Err(idx) => idx,
            };

            self.free_render_pass_indices.insert(rp_insert_idx, render_pass_index);
        }

        let user_rect_idx = match self.free_rect_indices.binary_search(&rect_id) {
            Ok(idx) | Err(idx) => idx,
        };
        self.free_rect_indices.insert(user_rect_idx, rect_id);
    }

    pub fn get_rect(&self, rect_id: DivisionId) -> &DrawableRect {
        let rect_in_render_pass = self.rects[rect_id as usize];
        let rects = &self.render_pass_to_rects[rect_in_render_pass.render_pass_idx];
        &rects.rects[rect_in_render_pass.rect_idx]
    }

    pub fn get_rect_mut(&mut self, rect_id: DivisionId) -> &mut DrawableRect {
        let rect_in_render_pass = self.rects[rect_id as usize];
        let rects = &mut self.render_pass_to_rects[rect_in_render_pass.render_pass_idx];
        &mut rects.rects[rect_in_render_pass.rect_idx]
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

        for (render_pass_idx, render_pass_rects) in
            self.render_pass_to_rects.iter().enumerate()
        {
            let render_pass_id = self.render_pass_ids[render_pass_idx];
            let vertex_buffer_id = {
                let render_pass = context.borrow_render_pass_mut(render_pass_id);
                render_pass.vertex_buffer
            };

            let instance_count = {
                let mut instance_count = 0;
                let vertex_buffer = Self::get_rect_drawer_data(context, vertex_buffer_id);
                let render_pass_free_rects = &render_pass_rects.free_rects;
                let mut curr_free_idx = 0;
                let mut left_index = 0;

                while left_index < render_pass_rects.rects.len() {
                    let right_index = if curr_free_idx < render_pass_free_rects.len() {
                        render_pass_free_rects[curr_free_idx]
                    } else {
                        render_pass_rects.rects.len()
                    };

                    let span_len = right_index - left_index;
                    for i in 0..span_len {
                        let src_index = left_index + i;
                        let dst_index = instance_count + i;
                        let rect = &render_pass_rects.rects[src_index];
                        vertex_buffer.per_instance_data[dst_index] = rect.into();
                    }
                    instance_count += span_len;

                    left_index = right_index + 1;
                    curr_free_idx += 1;
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

        for (pass_index, pass_id) in self.render_pass_ids.iter().enumerate() {
            if let Ok(_) = self.free_render_pass_indices.binary_search(&pass_index) {
                continue;
            }

            self.delete_render_pass_with_local_buffers(context, *pass_id);
        }
    }

    fn delete_render_pass_with_local_buffers(
        &self,
        context: &mut Context,
        pass_id: DivisionId,
    ) {
        let (vertex_buffer, texture_id) = {
            let pass = context.borrow_render_pass_mut(pass_id);
            (pass.vertex_buffer, pass.fragment_textures_slice()[0].id)
        };

        context.delete_vertex_buffer(vertex_buffer);

        if texture_id != self.white_pixel_texture_id {
            context.delete_texture_buffer(texture_id);
        }

        context.delete_render_pass(pass_id);
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
    ) -> RectVertexBufferData {
        context.vertex_buffer_data(vertex_buffer_id)
    }
}

impl DrawableRect {
    pub fn new(rect: Rect, decoration: Decoration) -> DrawableRect {
        DrawableRect { rect, decoration }
    }
}

impl From<&DrawableRect> for RectInstanceData {
    fn from(value: &DrawableRect) -> Self {
        RectInstanceData {
            size: value.rect.size(),
            position: value.rect.bottom_left(),
            color: value.decoration.color.into(),
            trbl_border_radius: value.decoration.border_radius.into(),
        }
    }
}
