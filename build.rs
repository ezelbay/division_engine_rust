use cmake::Config;
use std::env;
use std::path::Path;

fn main() {
    let static_link_libs = [ "glfw3" ];
    let dynamic_link_libs = [ "X11" ];
    let project_name = "division_engine_c";
    let out_dir = env::var("OUT_DIR").unwrap();

    Config::new(project_name)
        .target(project_name)
        .out_dir(&out_dir)
        .build();

    let build_path = Path::new(&out_dir).join("build");
    println!("cargo:rustc-link-search=native={}", build_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=static={}", project_name);
    for lib_name in static_link_libs {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    for lib_name in dynamic_link_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    fs_extra::dir::remove(build_path.join("resources"))
        .expect("Failed to delete resources folder");

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    fs_extra::dir::copy(
        Path::new("resources"),
        build_path,
        &copy_options
    ).expect("Failed to copy resources folder");
}