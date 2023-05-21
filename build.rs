use cmake::Config;
use std::env;
use std::path::Path;
use std::vec;

struct DivisionBuildOptions {
    static_libs: Vec<String>,
    dynamic_libs: Vec<String>,
    frameworks: Vec<String>
}

fn main() {
    let build_options = get_build_options();
    let out_dir = env::var("OUT_DIR").unwrap();

    Config::new("division_engine_core")
        .target("division_engine_core")
        .build_target("division_engine_core")
        .out_dir(&out_dir)
        .build();

    let build_path = Path::new(&out_dir).join("build");
    println!("cargo:rustc-link-search=native={}", build_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=static={}", "division_engine_core");
    for lib_name in build_options.static_libs {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    for lib_name in build_options.dynamic_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    for framework_name in build_options.frameworks {
        println!("cargo:rustc-link-lib=framework={}", framework_name);
    }

    fs_extra::dir::remove(build_path.join("resources"))
        .expect("Failed to delete resources folder");

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    fs_extra::dir::copy(
        Path::new("resources"),
        build_path,
        &copy_options,
    ).expect("Failed to copy resources folder");
}

fn get_build_options() -> DivisionBuildOptions {
    if cfg!(target_os = "macos") {
        build_with_osx_metal()
    }
    else {
        build_with_glfw()
    }
}

fn build_with_osx_metal() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: vec![],
        static_libs: vec![ "osx_metal_internal".to_string() ],
        frameworks: vec![
            "Metal".to_string(),
            "MetalKit".to_string(),
            "AppKit".to_string()
        ]
    }
}

fn build_with_glfw() -> DivisionBuildOptions {
    DivisionBuildOptions {
        dynamic_libs: vec!["X11".to_string()],
        static_libs: vec![
            "glfw3".to_string(),
            "glfw_internal".to_string()
        ],
        frameworks: vec![]
    }
}

