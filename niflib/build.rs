use cmake::Config;
use std::path::PathBuf;

fn main() {
    let mut config = Config::new("niflib");

    config.define("CMAKE_CXX_FLAGS_DEBUG", "/D_ITERATOR_DEBUG_LEVEL=0");
    config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");
    config.profile("Release");

    let dst = config.build();

    std::fs::write(
        PathBuf::from(&dst).join("niflib_build_path.txt"),
        dst.display().to_string(),
    )
    .expect("Failed to write niflib_build_path.txt");

    // 2. Add the CMake output directory to the linker search path
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // 3. Link against the import library (my_cpp_lib.lib on Windows or libmy_cpp_lib.so on Linux)
    println!("cargo:rustc-link-lib=static=niflib_static");

    // // Compile bridge.cpp using a C++ compiler
    // cc::Build::new()
    //     .cpp(true) // Tell the compiler to treat this as C++
    //     .define("NIFLIB_STATIC_LINK", None) // Define a macro for static linking
    //     .file("src/lib.cpp") // Path to your C++ file
    //     .include("niflib/include") // Include path for your C++ headers
    //     .compile("niflib-cpp-bridge"); // Name of the output static library

    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("src/lib.cpp")
        .std("c++17")
        .define("NIFLIB_STATIC_LINK", None)
        .include("include")
        .include("niflib/include")
        .compile("niflib_cxxbridge");

    println!("cargo:rerun-if-changed=niflib/");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=include/");
}
