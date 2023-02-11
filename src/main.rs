use std::ffi::{CString, CStr, c_char, c_ulong, c_long, c_float};
use std::mem::size_of;
use division_engine_rust::division_engine::bridge::window::*;
use division_engine_rust::division_engine::bridge::shader::*;
use division_engine_rust::division_engine::bridge::vertex_buffer::*;
use division_engine_rust::division_engine::rendering::shader_program;

static VERTICES: [f32; 9] = [
    -0.9, -0.9, 0.,
    0.85, -0.9, 0.,
    -0.9, 0.85, 0.
];

static mut BUFFER_ID: c_long = -1;

fn main() {
    unsafe {
        init_engine();
    }
}

unsafe fn init_engine() {
    if division_engine_start_session(error_callback) == false {
        return;
    }

    let title = CString::new("Hey").unwrap();
    let settings = DivisionEngineSettings {
        window_width: 512,
        window_height: 512,
        title: title.as_ptr(),
    };

    let window_id = division_engine_window_create(&settings);
    let shader_id = shader_program::ShaderProgramBuilder::new()
        .add_shader_source("resources/shaders/default_ui.vert", ShaderType::Vertex)
        .add_shader_source("resources/shaders/default_ui.frag", ShaderType::Fragment)
        .compile();
    shader_program::use_shader_program(shader_id);

    BUFFER_ID = division_engine_vertex_buffer_create(size_of::<c_float>() as c_ulong * 9);
    {
        let buffer_ptr = division_engine_vertex_buffer_access_ptr_begin(BUFFER_ID);
        assert!(!buffer_ptr.is_null());
        let buffer_ptr = buffer_ptr as *mut c_float;
        buffer_ptr.copy_from(VERTICES.as_ptr(), VERTICES.len());
        division_engine_vertex_buffer_access_ptr_end(BUFFER_ID);
    }
    division_engine_vertex_buffer_define_attribute(BUFFER_ID, VertexAttribute {
        index: 0,
        offset: 0,
        stride: 0,
        attribute_type: AttributeType::Float,
        normalized: false,
        size_of_components: 3,
    });

    division_engine_window_run_event_loop(window_id, update_callback);

    division_engine_window_destroy(window_id);

    division_engine_finish_session();
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!("Error code:{}, error message: {}", error_code, c_message);
}

unsafe extern "C" fn update_callback(_: DivisionEngineState) {
    division_engine_vertex_buffer_draw_triangles(BUFFER_ID, 0, 9);
}
