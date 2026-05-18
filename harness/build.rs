use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const VP8_VECTORS: &[&str] = &[
    "vp80-00-comprehensive-001.ivf",
    "vp80-00-comprehensive-002.ivf",
    "vp80-00-comprehensive-003.ivf",
    "vp80-00-comprehensive-004.ivf",
    "vp80-00-comprehensive-005.ivf",
    "vp80-00-comprehensive-006.ivf",
    "vp80-00-comprehensive-007.ivf",
    "vp80-00-comprehensive-008.ivf",
    "vp80-00-comprehensive-009.ivf",
    "vp80-00-comprehensive-010.ivf",
    "vp80-00-comprehensive-011.ivf",
    "vp80-00-comprehensive-012.ivf",
    "vp80-00-comprehensive-013.ivf",
    "vp80-00-comprehensive-014.ivf",
    "vp80-00-comprehensive-015.ivf",
    "vp80-00-comprehensive-016.ivf",
    "vp80-00-comprehensive-017.ivf",
    "vp80-00-comprehensive-018.ivf",
    "vp80-01-intra-1400.ivf",
    "vp80-01-intra-1411.ivf",
    "vp80-01-intra-1416.ivf",
    "vp80-01-intra-1417.ivf",
    "vp80-02-inter-1402.ivf",
    "vp80-02-inter-1412.ivf",
    "vp80-02-inter-1418.ivf",
    "vp80-02-inter-1424.ivf",
    "vp80-03-segmentation-01.ivf",
    "vp80-03-segmentation-02.ivf",
    "vp80-03-segmentation-03.ivf",
    "vp80-03-segmentation-04.ivf",
    "vp80-03-segmentation-1401.ivf",
    "vp80-03-segmentation-1403.ivf",
    "vp80-03-segmentation-1407.ivf",
    "vp80-03-segmentation-1408.ivf",
    "vp80-03-segmentation-1409.ivf",
];

fn download_test_vectors() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let test_data_dir = manifest_dir.parent().unwrap().join("test_data");

    fs::create_dir_all(&test_data_dir).unwrap();
    let base_url = "https://storage.googleapis.com/downloads.webmproject.org/test_data/libvpx";

    for vector in VP8_VECTORS {
        let target = test_data_dir.join(vector);
        if !target.exists() {
            println!("cargo:warning=Downloading test vector {}...", vector);
            let url = format!("{}/{}", base_url, vector);
            let status = Command::new("curl")
                .args(["-S", "-s", "-L", "-o", target.to_str().unwrap(), &url])
                .status()
                .expect("Failed to run curl");
            assert!(status.success(), "Failed to download {}", vector);
        }
    }
}

fn main() {
    download_test_vectors();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let libvpx_dir = PathBuf::from(manifest_dir).parent().unwrap().join("libvpx");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Tell cargo to invalidate the built crate whenever headers or the static lib change
    println!(
        "cargo:rerun-if-changed={}",
        libvpx_dir.join("vpx/vpx_decoder.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        libvpx_dir.join("vpx/vp8dx.h").display()
    );

    // The bindgen builder
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::nightly())
        // The input header we would like to generate bindings for.
        .header(libvpx_dir.join("vpx/vpx_decoder.h").to_str().unwrap())
        .header(libvpx_dir.join("vpx/vp8dx.h").to_str().unwrap())
        .clang_arg(format!("-I{}", libvpx_dir.display()))
        .dynamic_library_name("Vpx")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
