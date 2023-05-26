use std::fs;
use division_engine_rust::core::{
    DivisionCore, DivisionCoreDelegate, ShaderSourceDescriptor, ShaderType, ShaderVariableType, VertexAttributeDescriptor, RenderTopology,
};

pub struct MyDelegate {}

fn main() {
    let delegate = Box::new(MyDelegate {});
    let core = DivisionCore::builder()
        .window_size(1024, 1024)
        .window_title("Oh, my world")
        .build(delegate)
        .unwrap();

    core.run();
}

impl DivisionCoreDelegate for MyDelegate {
    fn init(&self, core: &mut DivisionCore) {
        core.create_shader_program(&[
            ShaderSourceDescriptor::new(
                ShaderType::Vertex,
                "vert",
                &fs::read_to_string("resources/shaders/test.vert.metal").unwrap()
            ),
            ShaderSourceDescriptor::new(
                ShaderType::Fragment,
                "frag",
                &fs::read_to_string("resources/shaders/test.frag.metal").unwrap()
            )
        ]).unwrap();

        core.create_vertex_buffer(
            &[VertexAttributeDescriptor { location: 0, field_type: ShaderVariableType::FVec3 }],
            &[VertexAttributeDescriptor { location: 1, field_type: ShaderVariableType::FVec4 }],
            6,
            2,
            RenderTopology::Triangles,
        ).unwrap();
    }

    fn update(&self, core: &mut DivisionCore) {
        
    }
}
