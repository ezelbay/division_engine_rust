use std::path::Path;

use division_math::{Vector2, Vector4};

use crate::core::{
    AlphaBlend, AlphaBlendOperation, Context, DivisionId, IdWithBinding,
    RenderPassDescriptor, RenderPassInstance, RenderPassInstanceOwned, RenderTopology,
    ShaderVariableType, VertexAttributeDescriptor, VertexBufferData, VertexData,
};

use super::{decoration::Decoration, rect::Rect};

pub struct RectDrawSystem {
    shader_id: DivisionId,
    screen_size_uniform: DivisionId,
    render_pass_descriptor: DivisionId,
    vertex_buffer_id: DivisionId,
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
pub struct RectInstanceData {
    #[location(2)]
    size: Vector2,
    #[location(3)]
    position: Vector2,
    #[location(4)]
    color: Vector4,
    #[location(5)]
    trbl_border_radius: Vector4,
}

pub const RECT_CAPACITY: usize = 128;
pub const VERTEX_PER_RECT: usize = 4;
pub const INDEX_PER_RECT: usize = 6;

impl RectDrawSystem {
    pub fn new(context: &mut Context, screen_size_uniform: DivisionId) -> RectDrawSystem {
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources")
                    .join("shaders")
                    .join("canvas")
                    .join("rect"),
            )
            .unwrap();

        let vertex_buffer_id = make_vertex_buffer(context);
        generate_rect_drawer_vertex_data(context, vertex_buffer_id);

        let render_pass_descriptor = context
            .create_render_pass_descriptor(
                &RenderPassDescriptor::with_shader_and_vertex_buffer(
                    shader_id,
                    vertex_buffer_id,
                )
                .alpha_blending(
                    AlphaBlend::SrcAlpha,
                    AlphaBlend::OneMinusSrcAlpha,
                    AlphaBlendOperation::Add,
                ),
            )
            .unwrap();

        RectDrawSystem {
            shader_id,
            screen_size_uniform,
            render_pass_descriptor,
            vertex_buffer_id,
            instance_count: 0,
        }
    }

    pub fn begin_frame_render(&mut self) {
        self.instance_count = 0;
    }

    pub fn create_new_pass(
        &mut self,
        context: &mut Context,
        texture_buffer_id: DivisionId,
        rects: &[RectInstanceData],
    ) -> RenderPassInstanceOwned {
        let pass = RenderPassInstanceOwned::new(
            RenderPassInstance::new(self.render_pass_descriptor)
                .vertices(VERTEX_PER_RECT, INDEX_PER_RECT)
                .instances(rects.len()),
        )
        .fragment_textures(&[IdWithBinding::new(texture_buffer_id, 0)])
        .uniform_vertex_buffers(&[IdWithBinding::new(self.screen_size_uniform, 1)])
        .uniform_fragment_buffers(&[IdWithBinding::new(self.screen_size_uniform, 1)]);

        let data = get_vertex_buffer_data(context, self.vertex_buffer_id);
        let instance_count = pass.instance_count as usize;
        let start = self.instance_count;
        let end = start + instance_count;
        let data_slice = &mut data.per_instance_data[start..end];

        data_slice.copy_from_slice(rects);

        self.instance_count += instance_count;

        pass
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_uniform_buffer(self.screen_size_uniform);
        context.delete_shader_program(self.shader_id);
        context.delete_render_pass_descriptor(self.render_pass_descriptor);
        context.delete_vertex_buffer(self.vertex_buffer_id);
    }
}

impl RectInstanceData {
    pub fn new(rect: Rect, decoration: Decoration) -> RectInstanceData {
        RectInstanceData {
            size: rect.size(),
            position: rect.bottom_left(),
            color: *decoration.color,
            trbl_border_radius: decoration.border_radius.into(),
        }
    }
}

fn generate_rect_drawer_vertex_data(
    context: &mut Context,
    vertex_buffer_id: DivisionId,
) {
    let data = get_vertex_buffer_data(context, vertex_buffer_id);

    let vertex_data = [
        RectVertexData {
            vert_pos: Vector2::new(0., 1.),
            uv: Vector2::new(0., 1.),
        },
        RectVertexData {
            vert_pos: Vector2::new(0., 0.),
            uv: Vector2::new(0., 0.),
        },
        RectVertexData {
            vert_pos: Vector2::new(1., 0.),
            uv: Vector2::new(1., 0.),
        },
        RectVertexData {
            vert_pos: Vector2::new(1., 1.),
            uv: Vector2::new(1., 1.),
        },
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    data.vertex_indices.copy_from_slice(&indices);
    data.per_vertex_data.copy_from_slice(&vertex_data);
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

#[inline(always)]
fn get_vertex_buffer_data(
    context: &mut Context,
    vertex_buffer_id: DivisionId,
) -> VertexBufferData<RectVertexData, RectInstanceData> {
    context.vertex_buffer_data(vertex_buffer_id)
}