use std::env;
use std::path::PathBuf;

fn main() {
    // === 1. CMake Build Section ===
    let dst = cmake::Config::new("cpp_lib").build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=my_math");

    // === 2. Bindgen automatically generates parts ===
    // Tell bindgen which header file we want to scan
    let header_path = "cpp_lib/include/my_math.h";

    let bindings = bindgen::Builder::default()
        .header(header_path)
        // Tell bindgen to regenerate if the header file changes
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Start generating bindings
        .generate()
        .expect("Unable to generate bindings");

    // Write the generated code to OUT_DIR in the target directory
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // === 3. Monitor changes ===
    println!("cargo:rerun-if-changed={}", header_path);
    println!("cargo:rerun-if-changed=cpp_lib/src/my_math.c");
}
