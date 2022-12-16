extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let kinect_sdk_10_dir = PathBuf::from(env::var("KINECTSDK10_DIR").unwrap());
    // println!("cargo:rustc-link-search={}", kinect_sdk_10_dir.to_str().unwrap());
    // println!("cargo:rustc-link-search={}", kinect_sdk_10_dir.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-search={}", kinect_sdk_10_dir.join("lib").join("amd64").to_str().unwrap());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=Kinect10");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("wrapper.hpp")
        .header("wrapper.h")
        // .clang_arg(format!("-I{}", kinect_sdk_10_dir.join("inc").to_str().unwrap()))
        .clang_arg(format!("-I{}", PathBuf::from("include").canonicalize().unwrap().to_str().unwrap()))
        .clang_arg("-fms-extensions")
        .clang_arg("-fms-compatibility")
        .clang_arg("-fdelayed-template-parsing")
        // .clang_arg("-x c++")
        .allowlist_file(".*NuiApi.h")
        .allowlist_file(".*NuiImageCamera.h")
        .allowlist_file(".*NuiSensor.h")
        .allowlist_file(".*NuiSkeleton.h")
        // type info
        .must_use_type("HRESULT")
        .no_copy("^INui.*")
        .no_default("^INui.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
