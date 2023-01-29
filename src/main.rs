use std::ffi::{CString, CStr, c_char, c_ulong, c_long, c_float};
use std::mem::size_of;
use division_engine_rust::division_engine_c::window::*;
use division_engine_rust::division_engine_c::shader::*;
use division_engine_rust::division_engine_c::vertex_buffer::*;

static VERTICES: [f32; 9] = [
    -0.9, -0.9, 0.,
    0.85, -0.9, 0.,
    -0.9, 0.85, 0.
];

static mut BUFFER_ID: c_long = 0;

fn main() {
    unsafe {
        init_engine();
    }
}

unsafe fn init_engine() {
    let title = CString::new("Hey").unwrap();
    let settings = DivisionEngineSettings {
        window_width: 512,
        window_height: 512,
        title: title.as_ptr(),
        error_callback,
    };

    let mut handler = DivisionEngineHandler::new();
    division_engine_window_create(&settings, &mut handler);
    let vert_shader_path = CString::new("resources/shaders/default_ui.vert").unwrap();
    let frag_shader_path = CString::new("resources/shaders/default_ui.frag").unwrap();

    let shader_program_id = division_engine_shader_create_program();
    division_engine_shader_attach_to_program(
        vert_shader_path.as_ptr(), ShaderType::Vertex, shader_program_id);
    division_engine_shader_attach_to_program(
        frag_shader_path.as_ptr(), ShaderType::Fragment, shader_program_id);
    division_engine_shader_link_program(shader_program_id);
    division_engine_shader_use_program(shader_program_id);

    BUFFER_ID = division_engine_vertex_buffer_create(size_of::<c_float>() as c_ulong * 9);
    {
        let buffer_ptr = division_engine_vertex_buffer_access_ptr_begin(BUFFER_ID);
        assert!(!buffer_ptr.is_null());
        let buffer_ptr = buffer_ptr as *mut c_float;
        for i in 0..8 {
            buffer_ptr.offset(i).write(VERTICES[i as usize]);
        }
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

    division_engine_window_run_event_loop(handler.clone(), update_callback);

    division_engine_window_destroy(handler);
}

unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
    let c_message = CStr::from_ptr(message).to_str().unwrap();
    eprintln!("Error code:{}, error message: {}", error_code, c_message);
}

unsafe extern "C" fn update_callback(_: DivisionEngineState) {
    division_engine_vertex_buffer_draw_triangles(BUFFER_ID, 0, 9);
}
