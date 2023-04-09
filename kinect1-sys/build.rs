extern crate bindgen;
// extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let kinect_sdk_10_dir = PathBuf::from(env::var("KINECTSDK10_DIR").unwrap());

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        kinect_sdk_10_dir
            .join("lib/amd64")
            .to_str()
            .unwrap()
    );

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=Kinect10");

    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_file(".*wrapper.h")
        .allowlist_file(".*Nui.*.h")
        .clang_arg(format!(
            "-I{}",
            PathBuf::from("../include")
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
        ))
        .clang_arg("-Wno-extra-tokens")
        .clang_arg("-fms-extensions")
        .clang_arg("-fms-compatibility")
        .clang_arg("-fdelayed-template-parsing")
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
