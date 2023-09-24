use division_engine_rust::core::{
    Context, IdWithBinding, Image, LifecycleManager, RenderTopology, ShaderVariableType,
    VertexAttributeDescriptor, VertexData,
};
use division_math::{Matrix4x4, Vector2, Vector3, Vector4};
use std::path::Path;

pub struct MyDelegate {}

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
    let mut delegate = MyDelegate {};

    let mut context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Oh, my world")
        .build(&mut delegate)
        .unwrap();

    context.run();
}

impl LifecycleManager for MyDelegate {
    fn init(&mut self, context: &mut Context) {
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources").join("shaders").join("test"),
            )
            .unwrap();

        let vertices_data = [
            Vert {
                pos: Vector3::new(100., 500., 0.),
                color: Vector4::one(),
                uv: Vector2::new(0., 1.),
            },
            Vert {
                pos: Vector3::new(100., 100., 0.),
                color: Vector4::one(),
                uv: Vector2::new(0., 0.),
            },
            Vert {
                pos: Vector3::new(500., 100., 0.),
                color: Vector4::one(),
                uv: Vector2::new(1., 0.),
            },
            Vert {
                pos: Vector3::new(500., 500., 0.),
                color: Vector4::one(),
                uv: Vector2::new(1., 1.),
            },
        ];
        let instances_data = [Inst {
            local_to_world: Matrix4x4::ortho_with_near_far(0., 1024., 0., 1024., -1., 1.),
        }];
        let indices = [0, 1, 2, 2, 3, 0];

        let vertex_buffer_id = context
            .create_vertex_buffer::<Vert, Inst>(
                vertices_data.len(),
                indices.len(),
                instances_data.len(),
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

        context
            .render_pass_builder()
            .vertex_buffer(vertex_buffer_id, vertices_data.len(), indices.len())
            .instances(instances_data.len())
            .fragment_textures(&[IdWithBinding::new(texture_id, 0)])
            .fragment_uniform_buffers(&[IdWithBinding::new(buff_id, 1)])
            .shader(shader_id)
            .build()
            .unwrap();
    }

    fn update(&mut self, _context: &mut Context) {}

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}")
    }

    fn cleanup(&mut self, _context: &mut Context) {}
}
