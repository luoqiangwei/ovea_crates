// build.rs
use std::path::PathBuf;

fn main() {
    // 1. Trigger CMake build
    // This returns the installation path (usually in target/debug/build/...)
    let dst: PathBuf = cmake::Config::new("cpp_lib")
        .build();

    // 2. Tell Cargo where to find the library
    // The 'lib' folder is where CMake usually puts build artifacts
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());

    // 3. Link the library. 
    // Here we link against the static version for easier testing
    // println!("cargo:rustc-link-lib=static=my_math");
    // dynamically link instead to test dynamic loading
    println!("cargo:rustc-link-lib=dylib=my_math_dyn");

    // Re-run if any C file changes
    println!("cargo:rerun-if-changed=cpp_lib/src/my_math.c");
    println!("cargo:rerun-if-changed=cpp_lib/include/my_math.h");
    println!("cargo:rerun-if-changed=cpp_lib/CMakeLists.txt");
}
