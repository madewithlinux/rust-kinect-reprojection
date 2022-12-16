fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&[
            "-Wno-pragma-pack",
            "-Wno-extra-tokens",
            "-fms-extensions",
            "-fms-compatibility",
            "-fdelayed-template-parsing",
            // &format!(
            //     "-I{}",
            //     std::path::PathBuf::from("src")
            //         .canonicalize()
            //         .unwrap()
            //         .to_str()
            //         .unwrap()
            // ),
        ])
        .build()?;
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14")
        .include(std::path::PathBuf::from("src").canonicalize().unwrap())
        .compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    let kinect_sdk_10_dir = std::path::PathBuf::from(std::env::var("KINECTSDK10_DIR").unwrap());
    println!(
        "cargo:rustc-link-search={}",
        kinect_sdk_10_dir
            .join("lib")
            .join("amd64")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-lib=Kinect10");
    Ok(())
}
