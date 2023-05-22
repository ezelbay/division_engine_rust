use std::ffi::{c_ulong, c_long, c_float, CString, CStr, c_char, c_void};
use std::ptr::null_mut;
use division_engine_rust::core::interface::settings::*;
use division_engine_rust::core::interface::renderer::*;
use division_engine_rust::core::interface::context::*;
use division_engine_rust::core::interface::state::*;
use division_engine_rust::shader_compiler::interface::*;

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
    println!("Error code: {}, error message:\n {}\n", error_code, msg.to_str().unwrap());
}

unsafe extern "C" fn init_func(ctx: *const DivisionContext) {
    division_shader_compiler_alloc();

    let shader_src = std::fs::read_to_string("resources/shaders/test.vert").unwrap();
    let c_shader_src = CString::new(shader_src).unwrap();
    let c_entry_point_name = CString::new("vert").unwrap();

    let mut spirv: *mut c_void = null_mut();
    let mut spirv_bytes = 0 as c_ulong;
    division_shader_compiler_compile_glsl_to_spirv(
        c_shader_src.as_ptr(), c_shader_src.as_bytes().len() as i32,
        SHADER_TYPE_VERTEX, c_entry_point_name.as_ptr(),
        &mut spirv , &mut spirv_bytes
    );

    println!("spirv size is: {}", spirv_bytes);

    division_shader_compiler_free();
}

unsafe extern "C" fn update_func(ctx: *const DivisionContext) {}