use cmake::Config;
use division_shader_compiler_rust::{ShaderCompiler, ShaderType};
use std::path::{Path, PathBuf};
use std::vec;
use std::{env, fs};

struct DivisionBuildOptions {
    static_libs: Vec<String>,
    dynamic_libs: Vec<String>,
    frameworks: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=resources/*");
    println!("cargo:rerun-if-changed=division_engine_core");

    let build_options = get_build_options();
    let out_dir = env::var("OUT_DIR").unwrap();
    let division_engine_core = "division_engine_core";
    let build_path = Path::new(&out_dir).join("build");
    let examples_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap())
        .join("examples");

    Config::new(division_engine_core)
        .configure_arg("-DFT_DISABLE_HARFBUZZ:BOOL=ON")
        .configure_arg("-DFT_DISABLE_BZIP2:BOOL=ON")
        .configure_arg("-DFT_DISABLE_PNG:BOOL=ON")
        .configure_arg("-DFT_DISABLE_BROTLI:BOOL=ON")
        .target(division_engine_core)
        .out_dir(&out_dir)
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        build_path.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&out_dir).join("lib").to_str().unwrap()
    );

    for lib_name in build_options.static_libs {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    for lib_name in build_options.dynamic_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    for framework_name in build_options.frameworks {
        println!("cargo:rustc-link-lib=framework={}", framework_name);
    }

    println!("cargo:rustc-link-lib=static={}", division_engine_core);

    compile_shaders_to_msl();
    copy_resources_to_build(&examples_dir);
}

fn get_build_options() -> DivisionBuildOptions {
    let platform_opts = if cfg!(target_os = "macos") {
        build_with_osx_metal()
    } else {
        build_with_glfw()
    };

    let common_opts = build_common();

    DivisionBuildOptions {
        static_libs: Vec::from_iter(
            platform_opts
                .static_libs
                .into_iter()
                .chain(common_opts.static_libs.into_iter()),
        ),
        dynamic_libs: Vec::from_iter(
            platform_opts
                .dynamic_libs
                .into_iter()
                .chain(common_opts.dynamic_libs.into_iter()),
        ),
        frameworks: Vec::from_iter(platform_opts.frameworks),
    }
}

fn build_with_osx_metal() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: make_strings_vec(vec!["z"]),
        static_libs: make_strings_vec(vec!["osx_metal_internal"]),
        frameworks: make_strings_vec(vec![
            "Metal",
            "MetalKit",
            "AppKit",
            "GameController",
        ]),
    }
}

fn build_with_glfw() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: make_strings_vec(vec!["X11"]),
        static_libs: make_strings_vec(vec!["glfw3", "glfw_internal"]),
        frameworks: vec![],
    }
}

fn build_common() -> DivisionBuildOptions {
    DivisionBuildOptions {
        static_libs: vec![freetype_name()],
        dynamic_libs: make_strings_vec(vec!["z"]),
        frameworks: Vec::new(),
    }
}

fn make_strings_vec(strings: Vec<&str>) -> Vec<String> {
    strings.into_iter().map(|m| m.to_string()).collect()
}

fn freetype_name() -> String {
    let profile = env::var("PROFILE");
    match profile {
        Ok(p) if p.as_str() == "debug" => "freetyped",
        _ => "freetype",
    }
    .to_string()
}

fn compile_shaders_to_msl() {
    let shader_compiler = ShaderCompiler::new();
    for dir in walkdir::WalkDir::new("resources/shaders") {
        let dir = match dir {
            Ok(v) => v,
            Err(_) => continue,
        };

        if !dir.file_type().is_file() {
            continue;
        }

        let path = dir.path();

        let path_extension = match path.extension() {
            Some(e) => e,
            None => continue,
        };

        let entry_point;
        let shader_type;
        match path_extension.to_str() {
            Some("vert") => {
                entry_point = "vert";
                shader_type = ShaderType::Vertex
            }
            Some("frag") => {
                entry_point = "frag";
                shader_type = ShaderType::Fragment
            }
            _ => continue,
        }

        let glsl_src = std::fs::read_to_string(path);
        let glsl_src = match glsl_src {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "Failed to read a shader file by path: `{:?}` with error: `{}` ",
                    path, e
                );
                continue;
            }
        };

        let msl_src = match shader_compiler.compile_glsl_to_metal(
            &glsl_src,
            entry_point,
            shader_type,
        ) {
            Ok(v) => v,
            Err(_) => {
                panic!("Failed to compile the shader by path: {:?}", path);
            }
        };

        if fs::write(format!("{}.metal", path.to_string_lossy()), msl_src).is_err() {
            eprint!("Failed to write msl output by path: {:?}", path);
        }
    }
}

fn copy_resources_to_build(path: &PathBuf) {
    fs_extra::dir::remove(path.join("resources"))
        .expect("Failed to delete resources folder");

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    fs_extra::dir::copy(Path::new("resources"), path, &copy_options)
        .expect("Failed to copy resources folder");
}
