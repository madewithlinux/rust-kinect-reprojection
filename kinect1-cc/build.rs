extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let kinect_sdk_10_dir = PathBuf::from(env::var("KINECTSDK10_DIR").unwrap());

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        kinect_sdk_10_dir.join("lib/amd64").to_str().unwrap()
    );

    // first, build the helper library
    cc::Build::new()
        .file("src/kinect_helper.c")
        .flag(
            kinect_sdk_10_dir
                .join("lib/amd64/Kinect10.lib")
                .to_str()
                .unwrap(),
        )
        // .file("src/kinect_helper.h")
        .include("../include")
        .include("src")
        // .flag("-LKinect10")
        // .shared_flag(true)
        .static_flag(false)
        // .use_plt(true)
        .cargo_metadata(true)
        .compile("kinect_cc");
    // to pick up and link the library we just compiled
    // println!("cargo:rustc-link-lib=kinect_cc");
    // println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=../include/");
    println!("cargo:rerun-if-changed=src/");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("src/kinect_helper.h")
        .header("../include/forward_declarations.h")
        .header("../include/constants.h")
        .header("src/nui_sensor_interface.h")
        .allowlist_file(".*src/kinect_helper.h")
        .allowlist_file(".*/forward_declarations.h")
        .allowlist_file(".*/constants.h")
        .allowlist_file(".*src/nui_sensor_interface.h")
        // .clang_arg(format!("-I{}", PathBuf::from("include").canonicalize().unwrap().to_str().unwrap()))
        .clang_arg(format!(
            "-I{}",
            PathBuf::from("../include")
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            PathBuf::from("src")
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
