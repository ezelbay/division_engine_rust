use division_engine_rust::core::interface::context::*;
use division_engine_rust::core::interface::render_pass::division_engine_render_pass_alloc;
use division_engine_rust::core::interface::render_pass::AlphaBlend;
use division_engine_rust::core::interface::render_pass::AlphaBlendOperation;
use division_engine_rust::core::interface::render_pass::AlphaBlendingOptions;
use division_engine_rust::core::interface::render_pass::ColorMask;
use division_engine_rust::core::interface::render_pass::IdWithBinding;
use division_engine_rust::core::interface::render_pass::RenderPassCapabilityMask;
use division_engine_rust::core::interface::render_pass::RenderPassDescriptor;
use division_engine_rust::core::interface::renderer::*;
use division_engine_rust::core::interface::settings::*;
use division_engine_rust::core::interface::shader;
use division_engine_rust::core::interface::shader::*;
use division_engine_rust::core::interface::texture::division_engine_texture_alloc;
use division_engine_rust::core::interface::texture::division_engine_texture_set_data;
use division_engine_rust::core::interface::texture::TextureDescriptor;
use division_engine_rust::core::interface::texture::TextureFormat;
use division_engine_rust::core::interface::uniform_buffer::UniformBufferDescriptor;
use division_engine_rust::core::interface::vertex_buffer::division_engine_vertex_buffer_alloc;
use division_engine_rust::core::interface::vertex_buffer::division_engine_vertex_buffer_borrow_data_pointer;
use division_engine_rust::core::interface::vertex_buffer::division_engine_vertex_buffer_return_data_pointer;
use division_engine_rust::core::interface::vertex_buffer::RenderTopology;
use division_engine_rust::core::interface::vertex_buffer::VertexAttributeDescriptor;
use division_engine_rust::core::interface::vertex_buffer::VertexBufferDescriptor;
use division_math::Matrix4x4;
use division_math::Vector2;
use division_math::Vector3;
use division_math::Vector4;
use std::ffi::{c_char, c_float, c_long, c_ulong, c_void, CStr, CString};
use std::fs;
use std::mem::size_of;
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
    let vert_source: CString;
    let frag_source: CString;
    let vert_entry_point: CString;
    let frag_entry_point: CString;

    if cfg!(target_os = "macos") {
        vert_source = CString::new(
            fs::read_to_string("resources/shaders/test.vert.metal").unwrap()).unwrap();
        frag_source = CString::new(
            fs::read_to_string("resources/shaders/test.frag.metal").unwrap()).unwrap();

        vert_entry_point = CString::new("vert").unwrap_unchecked();
        frag_entry_point = CString::new("frag").unwrap_unchecked();
    } else {
        vert_source = CString::new(
            fs::read_to_string("resources/shaders/test.vert").unwrap()).unwrap();
        frag_source = CString::new(
            fs::read_to_string("resources/shaders/test.frag").unwrap()).unwrap();

        vert_entry_point = CString::new("main").unwrap_unchecked();
        frag_entry_point = CString::new("main").unwrap_unchecked();
    }

    let shader_settings = [
        DivisionShaderSourceDescriptor {
            shader_type: shader::ShaderType::Vertex,
            entry_point_name: vert_entry_point.as_ptr(),
            source: vert_source.as_ptr(),
            source_size: vert_source.as_bytes().len() as i32
        },
        DivisionShaderSourceDescriptor {
            shader_type: shader::ShaderType::Fragment,
            entry_point_name: frag_entry_point.as_ptr(),
            source: frag_source.as_ptr(),
            source_size: frag_source.as_bytes().len() as i32
        },
    ];

    let mut shader_id = 0;
    assert!(division_engine_shader_program_alloc(
        ctx,
        shader_settings.as_ptr(),
        2,
        &mut shader_id
    ));

    let vertices = [
        VertexData {
            position: Vector3::new(-50., -50., 0.),
            color: Vector4::one(),
            uv: Vector2::new(0., 1.),
        },
        VertexData {
            position: Vector3::new(0., 0., 0.),
            color: Vector4::one(),
            uv: Vector2::new(1., 0.),
        },
        VertexData {
            position: Vector3::new(-50., 0., 0.),
            color: Vector4::one(),
            uv: Vector2::new(0., 0.),
        },
        VertexData {
            position: Vector3::new(0., 0., 0.),
            color: Vector4::one(),
            uv: Vector2::new(1., 0.),
        },
        VertexData {
            position: Vector3::new(-50., -50., 0.),
            color: Vector4::one(),
            uv: Vector2::new(0., 1.),
        },
        VertexData {
            position: Vector3::new(0., -50., 0.),
            color: Vector4::one(),
            uv: Vector2::new(1., 1.),
        },
    ];

    let local_to_world_matrices = [
        Matrix4x4::ortho(-256., 256., -256., 256., -1., 1.),
        Matrix4x4::ortho(-256., 256., -256., 256., -1., 1.),
    ];

    let vertex_attr = [
        VertexAttributeDescriptor {
            field_type: ShaderVariableType::FVec3,
            location: 0,
        },
        VertexAttributeDescriptor {
            field_type: ShaderVariableType::FVec4,
            location: 1,
        },
        VertexAttributeDescriptor {
            field_type: ShaderVariableType::FVec2,
            location: 2,
        },
    ];

    let instance_attr = [VertexAttributeDescriptor {
        field_type: ShaderVariableType::FMat4x4,
        location: 3,
    }];

    let vertex_buffer_desc = VertexBufferDescriptor {
        per_vertex_attributes: vertex_attr.as_ptr(),
        per_instance_attributes: instance_attr.as_ptr(),
        per_vertex_attribute_count: vertex_attr.len() as i32,
        per_instance_attribute_count: instance_attr.len() as i32,
        vertex_count: vertices.len() as i32,
        instance_count: local_to_world_matrices.len() as i32,
        topology: RenderTopology::Triangles,
    };

    let mut vert_buffer_id = 0;
    division_engine_vertex_buffer_alloc(ctx, &vertex_buffer_desc, &mut vert_buffer_id);

    let vert_buff_ptr = division_engine_vertex_buffer_borrow_data_pointer(ctx, vert_buffer_id);

    let vert_buffer_per_vertex_ptr = vert_buff_ptr as *mut VertexData;
    let vert_buffer_per_instance_ptr =
        vert_buff_ptr.add(vertices.len() * size_of::<VertexData>()) as *mut Matrix4x4;

    vert_buffer_per_vertex_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
    division_engine_vertex_buffer_return_data_pointer(
        ctx,
        vert_buffer_id,
        vert_buff_ptr as *mut c_void,
    );

    vert_buffer_per_instance_ptr.copy_from_nonoverlapping(
        local_to_world_matrices.as_ptr(),
        local_to_world_matrices.len(),
    );

    let img = fs::read("resources/images/nevsky.jpg").expect("Failed to load nevsky");
    let (mut width, mut height) = (0, 0);
    let img = stb_image_rust::stbi_load_from_memory(
        img.as_ptr(),
        img.len() as i32,
        &mut width,
        &mut height,
        null_mut(),
        stb_image_rust::STBI_rgb,
    );

    let texture_descriptor = TextureDescriptor {
        texture_format: TextureFormat::RGB24Uint,
        width: width as u32,
        height: height as u32,
    };
    let mut texture_id = 0;
    division_engine_texture_alloc(ctx, &texture_descriptor, &mut texture_id);
    division_engine_texture_set_data(ctx, texture_id, img as *mut c_void);

    let textures = [IdWithBinding {
        id: texture_id,
        shader_location: 0,
    }];
    let render_pass_desc = RenderPassDescriptor {
        shader_program: shader_id,
        vertex_buffer: vert_buffer_id,
        uniform_vertex_buffers: null_mut(),
        uniform_vertex_buffer_count: 0,
        uniform_fragment_buffers: null_mut(),
        uniform_fragment_buffer_count: 0,
        fragment_textures: textures.as_ptr(),
        fragment_texture_count: 1,

        first_vertex: 0,
        vertex_count: vertex_buffer_desc.vertex_count as u64,
        instance_count: vertex_buffer_desc.instance_count as u64,
        color_mask: ColorMask::RGBA,
        capabilities_mask: RenderPassCapabilityMask::None,
        alpha_blending_options: AlphaBlendingOptions {
            src: AlphaBlend::Zero,
            dst: AlphaBlend::Zero,
            operation: AlphaBlendOperation::Add,
            constant_blend_color: [0., 0., 0., 0.],
        },
    };
    let mut render_pass_id = 0;
    division_engine_render_pass_alloc(ctx, render_pass_desc, &mut render_pass_id);
}

unsafe extern "C" fn update_func(ctx: *mut DivisionContext) {}
