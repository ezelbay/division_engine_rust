use division_engine_rust::core::interface::context::*;
use division_engine_rust::core::interface::renderer::*;
use division_engine_rust::core::interface::settings::*;
use division_engine_rust::core::interface::shader;
use division_engine_rust::core::interface::shader::*;
use division_engine_rust::core::interface::vertex_buffer::RenderTopology;
use division_engine_rust::core::interface::vertex_buffer::VertexAttributeInputDescriptor;
use division_engine_rust::core::interface::vertex_buffer::VertexBufferInputDescriptor;
use division_math::Matrix4x4;
use division_math::Vector2;
use division_math::Vector3;
use division_math::Vector4;
use std::ffi::{c_char, c_float, c_long, c_ulong, c_void, CStr, CString};
use std::ptr::null_mut;

static VERTICES: [f32; 9] = [-0.9, -0.9, 0., 0.85, -0.9, 0., -0.9, 0.85, 0.];

static mut BUFFER_ID: c_long = -1;

#[repr(C)]
pub struct VertexData {
    position: Vector3,
    color: Vector4,
    uv: Vector2,
}

fn main() {
    unsafe {
        init_engine();
    }
}

unsafe fn init_engine() {
    let window_title = CString::new("Hello window").unwrap();
    let settings: DivisionSettings = DivisionSettings {
        window_width: 512,
        window_height: 512,
        window_title: window_title.as_ptr(),
        error_callback: error_func,
        init_callback: init_func,
        update_callback: update_func,
    };
    let mut context: *mut DivisionContext = null_mut();
    division_engine_context_alloc(&settings, (&mut context) as *mut *mut DivisionContext);

    division_engine_renderer_run_loop(context, &settings);

    division_engine_context_free(context);
}

unsafe extern "C" fn error_func(error_code: i32, error_msg: *const c_char) {
    let msg = CStr::from_ptr(error_msg);
    println!(
        "Error code: {}, error message:\n {}\n",
        error_code,
        msg.to_str().unwrap()
    );
}

unsafe extern "C" fn init_func(ctx: *mut DivisionContext) {
    let vert_path: CString;
    let frag_path: CString;
    let vert_entry_point: CString;
    let frag_entry_point: CString;

    if cfg!(target_os = "macos") {
        vert_path = CString::new("resources/shaders/test.vert.metal").unwrap_unchecked();
        frag_path = CString::new("resources/shaders/test.frag.metal").unwrap_unchecked();
        vert_entry_point = CString::new("vert").unwrap_unchecked();
        frag_entry_point = CString::new("frag").unwrap_unchecked();
    } else {
        vert_path = CString::new("resources/shaders/test.vert").unwrap_unchecked();
        frag_path = CString::new("resources/shaders/test.frag").unwrap_unchecked();
        vert_entry_point = CString::new("main").unwrap_unchecked();
        frag_entry_point = CString::new("main").unwrap_unchecked();
    }

    let shader_settings = [
        DivisionShaderFileDescriptor {
            shader_type: shader::ShaderType::Vertex,
            file_path: vert_path.as_ptr(),
            entry_point_name: vert_entry_point.as_ptr(),
        },
        DivisionShaderFileDescriptor {
            shader_type: shader::ShaderType::Fragment,
            file_path: frag_path.as_ptr(),
            entry_point_name: frag_entry_point.as_ptr(),
        },
    ];

    let mut shader_id = 0u32;
    assert!(division_engine_shader_program_alloc(ctx, shader_settings.as_ptr(), 2, &mut shader_id));

    let vertices = [
        VertexData { position: Vector3::new(-0.5, -0.5, 0.), color: Vector4::one(), uv: Vector2::new(0., 1.) },
        VertexData { position: Vector3::new(0., 0., 0.), color: Vector4::one(), uv: Vector2::new(1., 0.) },
        VertexData { position: Vector3::new(-0.5, 0., 0.), color: Vector4::one(), uv: Vector2::new(0., 0.) },
        VertexData { position: Vector3::new(0., 0., 0.), color: Vector4::one(), uv: Vector2::new(1., 0.) },
        VertexData { position: Vector3::new(-0.5, -0.5, 0.), color: Vector4::one(), uv: Vector2::new(0., 1.)},
        VertexData { position: Vector3::new(0., -0.5, 0.), color: Vector4::one(), uv: Vector2::new(1., 1.)},
    ];

    let localToWorldMatrices = [
        Matrix4x4::identity(),
        Matrix4x4::identity(),
    ];

    let vertex_attr = [
        VertexAttributeInputDescriptor { field_type: ShaderVariableType::FVec3, location: 0 },
        VertexAttributeInputDescriptor { field_type: ShaderVariableType::FVec4, location: 1 },
        VertexAttributeInputDescriptor { field_type: ShaderVariableType::FVec2, location: 2 },
    ];

    let instance_attr = [
        VertexAttributeInputDescriptor { field_type: ShaderVariableType::FMat4x4, location: 3 }
    ];

    let vertex_buffer_desc = VertexBufferInputDescriptor {
        per_vertex_attributes: vertex_attr.as_ptr(),
        per_instance_attributes: instance_attr.as_ptr(),
        per_vertex_attribute_count: vertex_attr.len() as i32,
        per_instance_attribute_count: instance_attr.len() as i32,
        vertex_count: vertices.len() as i32,
        instance_count: localToWorldMatrices.len() as i32,
        topology: RenderTopology::Triangles
    };
}

unsafe extern "C" fn update_func(ctx: *mut DivisionContext) {}
