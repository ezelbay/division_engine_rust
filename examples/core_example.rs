use division_engine_rust::{core::{
        Context, CoreRunner, IdWithBinding, Image, ImageSettings,
        LifecycleManager, LifecycleManagerBuilder, RenderPassDescriptor,
        RenderPassInstance, RenderPassInstanceOwned, RenderTopology, ShaderVariableType,
        VertexAttributeDescriptor, VertexData, VertexBufferSize,
    }, canvas::color::Color32};
use division_math::{Matrix4x4, Vector2, Vector3, Vector4};
use std::path::Path;

pub struct MyDelegateBuilder;

pub struct MyDelegate {
    render_pass_instance: RenderPassInstanceOwned,
}

#[repr(packed)]
#[derive(Clone, Copy, VertexData)]
#[allow(dead_code)]
pub struct Vert {
    #[location(0)]
    pos: Vector3,
    #[location(1)]
    color: Vector4,
    #[location(2)]
    uv: Vector2,
}

#[repr(packed)]
#[derive(Clone, Copy, VertexData)]
#[allow(dead_code)]
pub struct Inst {
    #[location(3)]
    local_to_world: Matrix4x4,
}

fn main() {
    CoreRunner::new()
        .window_size(1024, 1024)
        .window_title("Oh, my world")
        .run(MyDelegateBuilder)
        .unwrap();
}

impl LifecycleManagerBuilder for MyDelegateBuilder {
    type LifecycleManager = MyDelegate;

    fn build(&mut self, context: &mut Context) -> Self::LifecycleManager {
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources").join("shaders").join("test"),
            )
            .unwrap();

        let vertices_data = [
            Vert {
                pos: Vector3::new(100., 500., 0.),
                color: Vector4::one(),
                uv: Vector2::new(0., 0.),
            },
            Vert {
                pos: Vector3::new(100., 100., 0.),
                color: Vector4::one(),
                uv: Vector2::new(0., 1.),
            },
            Vert {
                pos: Vector3::new(500., 100., 0.),
                color: Vector4::one(),
                uv: Vector2::new(1., 1.),
            },
            Vert {
                pos: Vector3::new(500., 500., 0.),
                color: Vector4::one(),
                uv: Vector2::new(1., 0.),
            },
        ];
        let instances_data = [Inst {
            local_to_world: Matrix4x4::ortho_with_near_far(0., 1024., 0., 1024., -1., 1.),
        }];
        let indices = [0, 1, 2, 2, 3, 0];

        let vertex_buffer_id = context
            .create_vertex_buffer::<Vert, Inst>(
                VertexBufferSize {
                    vertex_count: vertices_data.len() as u32,
                    index_count: indices.len() as u32,
                    instance_count: instances_data.len() as u32
                },
                RenderTopology::Triangles,
            )
            .unwrap();

        {
            let data = context.vertex_buffer_data::<Vert, Inst>(vertex_buffer_id);
            data.per_vertex_data.copy_from_slice(&vertices_data);
            data.per_instance_data.copy_from_slice(&instances_data);
            data.vertex_indices.copy_from_slice(&indices);
        }

        let texture_id = {
            let image = Image::create_bundled_image(
                &Path::new("resources").join("images").join("nevsky.jpg"),
                ImageSettings::default(),
            )
            .unwrap();

            let texture_id = context.create_texture_buffer_from_image(&image).unwrap();

            texture_id
        };

        let buff_id = context
            .create_uniform_buffer_with_size_of::<Vector4>()
            .unwrap();

        {
            let buff_data = context.uniform_buffer_data(buff_id);
            *buff_data.data = Vector4::one() * 0.5;
        }

        let pass_desc_id = context
            .create_render_pass_descriptor(
                &RenderPassDescriptor::with_shader_and_vertex_buffer(
                    shader_id,
                    vertex_buffer_id,
                ),
            )
            .unwrap();

        let render_pass_instance = RenderPassInstanceOwned::new(
            RenderPassInstance::new(pass_desc_id)
                .vertices(vertices_data.len(), indices.len())
                .instances(instances_data.len()),
        )
        .fragment_textures(&[IdWithBinding::new(texture_id, 0)])
        .uniform_fragment_buffers(&[IdWithBinding::new(buff_id, 1)]);
        
        MyDelegate {
            render_pass_instance,
        }
    }
}

impl LifecycleManager for MyDelegate {
    fn draw(&mut self, context: &mut Context) {
        context.draw_render_passes(
            *Color32::white(),
            std::slice::from_ref(&self.render_pass_instance.instance)
        );
    }

    fn error(&mut self, _: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}")
    }

    fn cleanup(&mut self, _: &mut Context) {}
}
