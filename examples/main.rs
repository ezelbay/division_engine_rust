use division_engine_rust::core::interface::context::*;
use division_engine_rust::core::interface::renderer::*;
use division_engine_rust::core::interface::settings::*;
use division_engine_rust::core::interface::state::*;
use division_engine_rust::shader_compiler::interface::*;
use std::ffi::{c_char, c_float, c_long, c_ulong, c_void, CStr, CString};
use std::fs;
use std::fs::FileType;
use std::ptr::null_mut;
use walkdir::{DirEntry, WalkDir};

static VERTICES: [f32; 9] = [-0.9, -0.9, 0., 0.85, -0.9, 0., -0.9, 0.85, 0.];

static mut BUFFER_ID: c_long = -1;

fn main() {
    unsafe {
        compile_glsl_to_metal();
        init_engine();
    }
}

struct GlslToMslShader {
    glsl_path: String,
    entry_point: String,
    shader_type: ShaderType,
    out_msl_path: String,
}

unsafe fn compile_glsl_to_metal() {
    division_shader_compiler_alloc();

    let glsl_to_msl_shaders: Vec<GlslToMslShader> = WalkDir::new("resources/shaders")
        .into_iter()
        .map(make_glsl_to_msl_shader)
        .filter(|o| o.is_some())
        .map(|o| o.unwrap_unchecked())
        .collect();

    for s in glsl_to_msl_shaders {
        let shader_src = CString::new(
            fs::read_to_string(s.glsl_path).expect("Can't read shader source from file")
        ).unwrap();
        let entry_point = CString::new(s.entry_point).unwrap();
        let mut spirv: *mut c_void = null_mut();
        let mut spirv_bytes: c_ulong = 0;
        let mut msl: *mut c_char = null_mut();
        let mut msl_size: c_ulong = 0;

        assert!(division_shader_compiler_compile_glsl_to_spirv(
            shader_src.as_ptr(), shader_src.as_bytes().len() as i32,
            s.shader_type,
            entry_point.as_ptr(),
            &mut spirv, &mut spirv_bytes
        ));

        assert!(division_shader_compiler_compile_spirv_to_metal(
            spirv, spirv_bytes,
            s.shader_type,
            entry_point.as_ptr(),
            &mut msl, &mut msl_size
        ));

        let msl_str = CStr::from_ptr(msl).to_str().unwrap();

        fs::write(s.out_msl_path, msl_str)
            .expect("Failed to write metal source to the file");

        division_shader_compiler_spirv_source_free(spirv);
        division_shader_compiler_metal_source_free(msl);
    }

    division_shader_compiler_free();
}

fn make_glsl_to_msl_shader(entry: Result<DirEntry, walkdir::Error>) -> Option<GlslToMslShader> {
    if entry.is_err() { return None; }

    let entry = entry.ok().unwrap();
    let extension = entry.path().extension();
    if extension.is_none() { return None; }

    let extension = extension.unwrap();

    let shader_type: ShaderType;
    let entry_point: String;
    if extension == "vert" {
        shader_type = ShaderType::Vertex;
        entry_point = "vert".to_string();
    } else if extension == "frag" {
        shader_type = ShaderType::Fragment;
        entry_point = "frag".to_string();
    } else {
        return None;
    }

    return Some(GlslToMslShader {
        shader_type,
        entry_point,
        glsl_path: String::from(entry.path().to_str().unwrap()),
        out_msl_path: String::from(
            entry.path().with_extension(
                format!("{}.metal", extension.to_str().unwrap())
            ).to_str().unwrap()
        ),
    });
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

unsafe extern "C" fn init_func(ctx: *const DivisionContext) {}

unsafe extern "C" fn update_func(ctx: *const DivisionContext) {}
