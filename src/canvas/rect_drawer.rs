use std::path::Path;

use division_math::{Matrix4x4, Vector2, Vector4};

use crate::core::{
    Core, Error, DivisionId, RenderTopology,
    VertexAttributeDescriptor, VertexBufferData, ShaderVariableType,
};

use super::rect::Rect;

pub struct SolidRect {
    pub rect: Rect,
    pub color: Vector4,
}

pub struct RectDrawer<'a> {
    core: &'a mut Core,
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
    pos: Vector2
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

impl Core {
    pub fn create_rect_drawer<'a>(&'a mut self, view_matrix: Matrix4x4) -> RectDrawer<'a> {
        let shader_id = self
            .create_bundled_shader_program(Path::new("resources/shaders/canvas/solid_shape"))
            .unwrap();

        let vertex_buffer_id = Self::make_vertex_buffer(self);
        self.generate_rect_drawer_vertex_data(vertex_buffer_id);

        let render_pass_id = self
            .render_pass_builder()
            .shader(shader_id)
            .vertex_buffer(vertex_buffer_id, VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing()
            .build()
            .unwrap();

        RectDrawer {
            core: self,
            shader_id,
            vertex_buffer_id,
            render_pass_id,
            instance_count: 0,
            view_matrix,
        }
    }

    fn make_vertex_buffer(core: &mut Core) -> DivisionId {
        core.create_vertex_buffer(
            &[
                VertexAttributeDescriptor {
                    location: 1,
                    field_type: ShaderVariableType::FVec2,
                },
            ],
            &[
                VertexAttributeDescriptor {
                    location: 2,
                    field_type: ShaderVariableType::FMat4x4,
                },
            ],
            VERTEX_PER_RECT,
            INDEX_PER_RECT,
            RECT_CAPACITY,
            RenderTopology::Triangles,
        )
        .unwrap()
    }

    fn generate_rect_drawer_vertex_data(&mut self, vertex_buffer_id: DivisionId) {
        let data = self.get_rect_drawer_data(vertex_buffer_id);
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
        &mut self,
        vertex_buffer_id: DivisionId,
    ) -> VertexBufferData<VertexData, InstanceData> {
        self.vertex_buffer_data(vertex_buffer_id)
    }
}

impl<'a> RectDrawer<'a> {
    pub fn draw_rect(&mut self, solid_rect: SolidRect) -> Result<(), Error> {
        if self.instance_count >= RECT_CAPACITY {
            return Err(Error::Core(
                "Rect capacity limit exceed".to_string(),
            ));
        }

        self.write_rect_data(solid_rect, self.instance_count);

        self.instance_count += 1;

        let borrowed_pass = self.core.borrow_render_pass_mut_ptr(self.render_pass_id);
        borrowed_pass.render_pass.instance_count = self.instance_count as u64;

        Ok(())
    }

    fn write_rect_data(&mut self, solid_rect: SolidRect, instance_index: usize) {
        let data = self.core.get_rect_drawer_data(self.vertex_buffer_id);
        let size = solid_rect.rect.size();
        let center = solid_rect.rect.center;
        let transform = Matrix4x4::from_columns(
            Vector4::new(size.x, 0., 0., 0.), 
            Vector4::new(0., size.y, 0., 0.), 
            Vector4::new(0., 0., 1.0, 1.), 
            Vector4::new(center.x, center.y, 0., 1.)
        );

        let local_to_world = self.view_matrix * transform;

        data.per_instance_data[instance_index] = InstanceData { local_to_world };
    }
}

// TODO: Add drop functionality and proper ownment
