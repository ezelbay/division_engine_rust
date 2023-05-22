use cmake::Config;
use std::env;
use std::path::Path;
use std::vec;

struct DivisionBuildOptions {
    static_libs: Vec<String>,
    dynamic_libs: Vec<String>,
    frameworks: Vec<String>,
}

fn main() {
    let build_options = get_build_options();
    let out_dir = env::var("OUT_DIR").unwrap();
    let curr_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let division_engine_core = "division_engine_core";
    env::set_var("DIVISION_ENGINE_USE_SHADER_COMPILER", "1");

    let build_path = Path::new(&out_dir).join("build");
    let cmake_build_path = Path::new(&curr_dir)
        .join("division_engine_core")
        .join("cmake-build");

    Config::new(division_engine_core)
        .target(division_engine_core)
        .out_dir(&out_dir)
        .define("CMAKE_CXX_COMPILER", "clang++")
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
    println!("cargo:rustc-link-lib=static={}", "division_engine_shader_compiler");

    fs_extra::dir::remove(build_path.join("resources")).expect("Failed to delete resources folder");

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    fs_extra::dir::copy(Path::new("resources"), build_path, &copy_options)
        .expect("Failed to copy resources folder");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=division_engine_core");
}

fn get_build_options() -> DivisionBuildOptions {
    if cfg!(target_os = "macos") {
        build_with_osx_metal()
    } else {
        build_with_glfw()
    }
}

fn build_with_osx_metal() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: make_strings_vec(vec!["c++"]),
        static_libs: make_strings_vec(vec![
            "osx_metal_internal",
            "MachineIndependent",
            "OSDependent",
            "OGLCompiler",
            "GenericCodeGen",
            "glslang-default-resource-limits",
            "glslang",
            "SPIRV",
            "spirv-cross-core",
            "spirv-cross-cpp",
            "spirv-cross-glsl",
            "spirv-cross-msl"
        ]),
        frameworks: make_strings_vec(vec!["Metal", "MetalKit", "AppKit"]),
    }
}

fn build_with_glfw() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: make_strings_vec(vec!["X11"]),
        static_libs: make_strings_vec(vec!["glfw3", "glfw_internal"]),
        frameworks: vec![],
    }
}

fn make_strings_vec(strings: Vec<&str>) -> Vec<String> {
    strings.into_iter().map(|m| m.to_string()).collect()
}
