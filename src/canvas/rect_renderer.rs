use std::path::Path;

use division_math::{Vector2, Vector4};

use crate::core::{
    AlphaBlend, AlphaBlendOperation, Context, DivisionId, IdWithBinding,
    RenderPassDescriptor, RenderPassInstance, RenderTopology,
    ShaderVariableType, VertexAttributeDescriptor, VertexBufferData, VertexData,
};

use super::{
    renderable_rect::RenderableRect,
    renderer::{Renderer, RenderQueue},
};

pub struct RectRenderer {
    shader_id: DivisionId,
    vertex_buffer_id: DivisionId,
    render_pass_descriptor: DivisionId,
    screen_size_uniform: IdWithBinding,
    textures_heap: Vec<IdWithBinding>,
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

pub const RECT_CAPACITY: usize = 128;
pub const VERTEX_PER_RECT: usize = 4;
pub const INDEX_PER_RECT: usize = 6;

impl RectRenderer {
    pub fn new(
        context: &mut Context,
        screen_size_uniform_id: DivisionId,
    ) -> RectRenderer {
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

        RectRenderer {
            shader_id,
            screen_size_uniform: IdWithBinding {
                id: screen_size_uniform_id,
                shader_binding: 1,
            },
            render_pass_descriptor,
            vertex_buffer_id,
            textures_heap: Vec::new(),
            instance_count: 0,
        }
    }

    fn create_new_pass(&mut self, texture_id: DivisionId) -> RenderPassInstance {
        let mut pass = RenderPassInstance::new(self.render_pass_descriptor)
            .vertices(VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing();
        pass.first_instance = self.instance_count as u32;

        self.textures_heap.push(IdWithBinding::new(texture_id, 0));

        unsafe {
            pass.set_uniform_vertex_buffers(std::slice::from_ref(
                &self.screen_size_uniform,
            ));
            pass.set_uniform_fragment_buffers(std::slice::from_ref(
                &self.screen_size_uniform
            ));
            pass.set_uniform_fragment_textures(std::slice::from_ref(
                &self.textures_heap.last().unwrap_unchecked(),
            ));
        }

        pass
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_shader_program(self.shader_id);
        context.delete_render_pass_descriptor(self.render_pass_descriptor);
        context.delete_vertex_buffer(self.vertex_buffer_id);
    }
}

impl Renderer for RectRenderer {
    type RenderableData = RenderableRect;

    fn before_render_frame(&mut self, _: &mut Context) {
        self.instance_count = 0;
        self.textures_heap.clear();
    }

    fn enqueue_render_passes(
        &mut self,
        context: &mut Context,
        renderables: &[Self::RenderableData],
        render_queue: &mut RenderQueue,
    ) {
        if renderables.len() == 0 {
            return;
        }

        let mut curr_pass_tex = renderables[0].decoration.texture_id;
        let mut pass = self.create_new_pass(curr_pass_tex);

        let vertex_buffer_data = get_vertex_buffer_data(context, self.vertex_buffer_id);

        for r in renderables {
            let renderable_texture_id = r.decoration.texture_id;

            if r.decoration.texture_id != curr_pass_tex {
                render_queue.enqueue_render_pass(pass);
                pass = self.create_new_pass(r.decoration.texture_id);
                curr_pass_tex = renderable_texture_id;
            }

            let d = &mut vertex_buffer_data.per_instance_data[self.instance_count];
            d.position = r.rect.bottom_left();
            d.size = r.rect.size();
            d.color = *r.decoration.color;
            d.trbl_border_radius = *r.decoration.border_radius;

            pass.instance_count += 1;
            self.instance_count += 1;
        }

        render_queue.enqueue_render_pass(pass);
    }

    fn after_render_frame(&mut self, _: &mut Context) {}
}

fn generate_rect_drawer_vertex_data(context: &mut Context, vertex_buffer_id: DivisionId) {
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
