use division_engine_rust::core::{
    DivisionCore, DivisionCoreDelegate, RenderTopology, ShaderSourceDescriptor, ShaderType,
    ShaderVariableType, VertexAttributeDescriptor, TextureFormat, IdWithBinding
};
use division_math::{Vector3, Vector4, Vector2, Matrix4x4};
use stb_image_rust::stbi_set_flip_vertically_on_load;
use std::{fs, ptr::{null_mut}};

pub struct MyDelegate {
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vert {
    pos: Vector3,
    color: Vector4,
    uv: Vector2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Inst {
    local_to_world: Matrix4x4,
}

fn main() {
    let delegate = Box::new(MyDelegate{});
    let core = DivisionCore::builder()
        .window_size(1024, 1024)
        .window_title("Oh, my world")
        .build(delegate)
        .unwrap();

    core.run();
}

impl DivisionCoreDelegate for MyDelegate {
    fn init(&mut self, core: &mut DivisionCore) {
        let (vert_entry, vert_path, frag_entry, frag_path) = if cfg!(target_os = "macos") {
            ("vert", "resources/shaders/test.vert.metal", "frag", "resources/shaders/test.frag.metal")
        } else {
            ("main", "resources/shaders/test.vert", "main", "resources/shaders/test.frag")
        };

        let shader_id = core.create_shader_program(&[
            ShaderSourceDescriptor::new(
                ShaderType::Vertex,
                vert_entry,
                &fs::read_to_string(vert_path).unwrap(),
            ),
            ShaderSourceDescriptor::new(
                ShaderType::Fragment,
                frag_entry,
                &fs::read_to_string(frag_path).unwrap(),
            ),
        ])
        .unwrap();

        let vertex_buffer_id = core.create_vertex_buffer(
            &[
                VertexAttributeDescriptor { location: 0, field_type: ShaderVariableType::FVec3, },
                VertexAttributeDescriptor { location: 1, field_type: ShaderVariableType::FVec4, },
                VertexAttributeDescriptor { location: 2, field_type: ShaderVariableType::FVec2, },
            ],
            &[VertexAttributeDescriptor {
                location: 3,
                field_type: ShaderVariableType::FMat4x4,
            }],
            6,
            2,
            RenderTopology::Triangles,
        )
        .unwrap();

        {
            let vertices_data = [
                Vert { pos: Vector3::new(0.,100.,0.), color: Vector4::one(), uv: Vector2::new(0., 1.) },
                Vert { pos: Vector3::new(0., 0., 0.), color: Vector4::one(), uv: Vector2::new(0., 0.) },
                Vert { pos: Vector3::new(100., 0., 0.), color: Vector4::one(), uv: Vector2::new(1., 0.) },
                Vert { pos: Vector3::new(0., 100., 0.), color: Vector4::one(), uv: Vector2::new(0., 1.) },
                Vert { pos: Vector3::new(100., 100., 0.), color: Vector4::one(), uv: Vector2::new(1., 1.) },
                Vert { pos: Vector3::new(100., 0., 0.), color: Vector4::one(), uv: Vector2::new(1., 0.) },
            ];
            let instances_data = [
                Inst { local_to_world: Matrix4x4::ortho(0., 1024., 0., 1024., 0., 1.) },
                Inst { local_to_world: Matrix4x4::ortho(0., 1024., 0., 1024., 0., 1.) },
            ];

            let data = core.vertex_buffer_data::<Vert, Inst>(vertex_buffer_id);
            data.per_vertex_data.copy_from_slice(&vertices_data);
            data.per_instance_data.copy_from_slice(&instances_data);
        }

        let image = fs::read("resources/images/nevsky.jpg").unwrap();
        let (mut width,mut height) = (0,0);
        let texture_id = unsafe {
            stbi_set_flip_vertically_on_load(1);

            let data = stb_image_rust::stbi_load_from_memory(
                image.as_ptr(), image.len() as i32, &mut width, &mut height, null_mut(), 3);

            core.create_texture_buffer_with_data(width as u32, height as u32, TextureFormat::RGB24Uint, 
                std::slice::from_raw_parts(data, (width * height) as usize)).unwrap()
        };

        let buff_id = core.create_uniform_buffer_with_size_of::<Vector4>().unwrap();

        {
            let buff_data = core.uniform_buffer_data(buff_id);
            *buff_data.data = Vector4::one() * 0.5;
        }

        core.render_pass_builder()
            .vertex_buffer(vertex_buffer_id, 0..6)
            .fragment_textures(&[IdWithBinding::new(texture_id, 0)])
            .vertex_uniform_buffers(&[IdWithBinding::new(buff_id, 1)])
            .shader(shader_id)
            .build()
            .unwrap();
    }

    fn update(&mut self, _core: &mut DivisionCore) {}
}
