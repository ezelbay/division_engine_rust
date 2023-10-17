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
    uniform_buffer_id: DivisionId,
    vertex_buffer_id: DivisionId,
    render_pass_id: DivisionId,
    texture_buffer_id: DivisionId,

    instance_count: usize,
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
        let white_texture = context
            .create_texture_buffer_from_data(
                &TextureDescriptor::new(1, 1, TextureFormat::RGBA32Uint),
                &[255u8; 4],
            )
            .unwrap();

        Self::with_texture(context, white_texture, false)
    }

    pub fn with_texture(
        context: &mut Context,
        texture_buffer_id: DivisionId,
        flip_vertical: bool,
    ) -> RectDrawSystem {
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources")
                    .join("shaders")
                    .join("canvas")
                    .join("rect"),
            )
            .unwrap();

        let vertex_buffer_id = Self::make_vertex_buffer(context);
        Self::generate_rect_drawer_vertex_data(context, vertex_buffer_id, flip_vertical);

        let uniform_buffer_id = context
            .create_uniform_buffer_with_size_of::<UniformData>()
            .unwrap();

        let texture_buffer_id = texture_buffer_id;

        let render_pass_id = context
            .render_pass_builder()
            .shader(shader_id)
            .fragment_textures(&[IdWithBinding::new(texture_buffer_id, 0)])
            .vertex_uniform_buffers(&[IdWithBinding::new(uniform_buffer_id, 1)])
            .fragment_uniform_buffers(&[IdWithBinding::new(uniform_buffer_id, 1)])
            .vertex_buffer(vertex_buffer_id, VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing()
            .alpha_blending(
                AlphaBlend::SrcAlpha,
                AlphaBlend::OneMinusSrcAlpha,
                AlphaBlendOperation::Add,
            )
            .build()
            .unwrap();

        RectDrawSystem {
            shader_id,
            uniform_buffer_id,
            vertex_buffer_id,
            render_pass_id,
            texture_buffer_id,
            instance_count: 0,
        }
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_render_pass(self.render_pass_id);
        context.delete_texture_buffer(self.texture_buffer_id);
        context.delete_uniform_buffer(self.uniform_buffer_id);
        context.delete_vertex_buffer(self.vertex_buffer_id);
        context.delete_shader_program(self.shader_id);
    }

    pub fn set_canvas_size(&mut self, context: &mut Context, size: Vector2) {
        let data = context.uniform_buffer_data::<UniformData>(self.uniform_buffer_id);
        *(data.data) = UniformData { size };
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

    pub fn draw_rect(
        &mut self,
        context: &mut Context,
        rect: Rect,
        decoration: Decoration,
    ) {
        assert!(self.instance_count < RECT_CAPACITY);

        self.write_rect_data(context, rect, decoration, self.instance_count);

        self.instance_count += 1;

        let mut borrowed_pass = context.borrow_render_pass_mut(self.render_pass_id);
        borrowed_pass.instance_count = self.instance_count as u64;
    }

    fn write_rect_data(
        &mut self,
        context: &mut Context,
        rect: Rect,
        decoration: Decoration,
        instance_index: usize,
    ) {
        let data = Self::get_rect_drawer_data(context, self.vertex_buffer_id);

        data.per_instance_data[instance_index] = RectInstanceData {
            size: rect.size(),
            position: rect.bottom_left(),
            color: decoration.color.into(),
            trbl_border_radius: decoration.border_radius.into(),
        };
    }

    #[inline(always)]
    fn get_rect_drawer_data(
        context: &mut Context,
        vertex_buffer_id: DivisionId,
    ) -> VertexBufferData<RectVertexData, RectInstanceData> {
        context.vertex_buffer_data(vertex_buffer_id)
    }
}
