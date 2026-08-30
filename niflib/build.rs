use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = Config::new("niflib");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    // if profile == "debug" {
    //     config.define("CMAKE_BUILD_TYPE", "Debug");
    // } else {
    //     // Keeps optimizations on but adds -g / debug symbols
    //     config.define("CMAKE_BUILD_TYPE", "RelWithDebInfo");
    // }

    // 1. Tell Cargo to run CMake on our C++ directory
    // Assumes your C++ files and CMakeLists.txt are in a folder named 'cpp_src'
    let dst = config.build();

    std::fs::write(
        PathBuf::from(&dst).join("niflib_build_path.txt"),
        dst.display().to_string(),
    )
    .expect("Failed to write niflib_build_path.txt");

    // 2. Add the CMake output directory to the linker search path
    println!(
        "cargo:rustc-link-search=native={}/build/debug/",
        dst.display()
    );

    // 3. Link against the import library (my_cpp_lib.lib on Windows or libmy_cpp_lib.so on Linux)
    println!("cargo:rustc-link-lib=static=niflib_static");
    println!("cargo:rustc-link-lib=static=niflib");

    // // Compile bridge.cpp using a C++ compiler
    cxx_build::bridge("src/lib.rs")
        .file("src/lib.cpp")
        .include("include")
        .include("niflib/include")
        .compile("cxxbridge");

    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.h");
}
