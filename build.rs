extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=fmod");

    #[cfg(feature = "studio")]
    println!("cargo:rustc-link-lib=fmodstudio");

    let bindings = bindgen::Builder::default()
        .header("core-wrapper.h")
        .generate()
        .expect("Unable to generate core bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-core.rs"))
        .expect("Couldn't write core bindings!");

    let bindings = bindgen::Builder::default()
        .header("studio-wrapper.h")
        .generate()
        .expect("Unable to generate studio bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-studio.rs"))
        .expect("Couldn't write studio bindings!");
}
