use std::path::Path;

use division_math::{Matrix4x4, Vector2, Vector4};

use crate::core::{
    Context, DivisionId, Error, RenderTopology, ShaderVariableType,
    VertexAttributeDescriptor, VertexBufferData,
};

use super::rect::Rect;

pub struct SolidRect {
    pub rect: Rect,
    pub color: Vector4,
}

pub struct RectDrawer {
    shader_id: DivisionId,
    vertex_buffer_id: DivisionId,
    render_pass_id: DivisionId,

    view_matrix: Matrix4x4,
    instance_count: usize,
}

#[repr(packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct VertexData {
    pos: Vector2,
}

#[repr(packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct InstanceData {
    local_to_world: Matrix4x4,
}

pub const RECT_CAPACITY: usize = 128;
pub const VERTEX_PER_RECT: usize = 4;
pub const INDEX_PER_RECT: usize = 6;

impl RectDrawer {
    pub fn new(context: &mut Context, view_matrix: Matrix4x4) -> RectDrawer {
        let shader_id = context
            .create_bundled_shader_program(Path::new(
                "resources/shaders/canvas/solid_shape",
            ))
            .unwrap();

        let vertex_buffer_id = Self::make_vertex_buffer(context);
        Self::generate_rect_drawer_vertex_data(context, vertex_buffer_id);

        let render_pass_id = context
            .render_pass_builder()
            .shader(shader_id)
            .vertex_buffer(vertex_buffer_id, VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing()
            .build()
            .unwrap();

        RectDrawer {
            shader_id,
            vertex_buffer_id,
            render_pass_id,
            instance_count: 0,
            view_matrix,
        }
    }

    fn make_vertex_buffer(context: &mut Context) -> DivisionId {
        context
            .create_vertex_buffer(
                &[VertexAttributeDescriptor {
                    location: 1,
                    field_type: ShaderVariableType::FVec2,
                }],
                &[VertexAttributeDescriptor {
                    location: 2,
                    field_type: ShaderVariableType::FMat4x4,
                }],
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
    ) {
        let data = Self::get_rect_drawer_data(context, vertex_buffer_id);
        let vertex_data = [
            VertexData {
                pos: Vector2::new(0., 1.),
            },
            VertexData {
                pos: Vector2::new(0., 0.),
            },
            VertexData {
                pos: Vector2::new(1., 0.),
            },
            VertexData {
                pos: Vector2::new(1., 1.),
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
    ) -> VertexBufferData<VertexData, InstanceData> {
        context.vertex_buffer_data(vertex_buffer_id)
    }

    pub fn draw_rect(
        &mut self,
        context: &mut Context,
        solid_rect: SolidRect,
    ) -> Result<(), Error> {
        if self.instance_count >= RECT_CAPACITY {
            return Err(Error::Core("Rect capacity limit exceed".to_string()));
        }

        self.write_rect_data(context, solid_rect, self.instance_count);

        self.instance_count += 1;

        let borrowed_pass = context.borrow_render_pass_mut_ptr(self.render_pass_id);
        borrowed_pass.render_pass.instance_count = self.instance_count as u64;

        Ok(())
    }

    fn write_rect_data(
        &mut self,
        context: &mut Context,
        solid_rect: SolidRect,
        instance_index: usize,
    ) {
        let data = Self::get_rect_drawer_data(context, self.vertex_buffer_id);
        let size = solid_rect.rect.size();
        let center = solid_rect.rect.center;
        let transform = Matrix4x4::from_columns(
            Vector4::new(size.x, 0., 0., 0.),
            Vector4::new(0., size.y, 0., 0.),
            Vector4::new(0., 0., 1.0, 1.),
            Vector4::new(center.x, center.y, 0., 1.),
        );

        let local_to_world = self.view_matrix * transform;

        data.per_instance_data[instance_index] = InstanceData { local_to_world };
    }

    pub fn delete(&mut self, context: &mut Context) {
        context.delete_render_pass(self.render_pass_id);
        context.delete_vertex_buffer(self.vertex_buffer_id);
        context.delete_shader_program(self.shader_id);
    }
}