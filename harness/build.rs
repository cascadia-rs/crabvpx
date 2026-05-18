use std::env;
use std::path::PathBuf;

fn main() {
    let libvpx_dir = PathBuf::from("../libvpx");

    // Tell cargo to invalidate the built crate whenever headers change
    println!("cargo:rerun-if-changed=../libvpx/vpx/vpx_decoder.h");
    println!("cargo:rerun-if-changed=../libvpx/vpx/vp8dx.h");

    if env::var("CARGO_FEATURE_RUST").is_err() {
        println!("cargo:rustc-link-search=native=../libvpx");
        println!("cargo:rustc-link-lib=static=vpx");
    }

    // The bindgen builder
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::nightly())
        // The input header we would like to generate bindings for.
        .header(libvpx_dir.join("vpx/vpx_decoder.h").to_str().unwrap())
        .header(libvpx_dir.join("vpx/vp8dx.h").to_str().unwrap())
        .clang_arg(format!("-I{}", libvpx_dir.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
