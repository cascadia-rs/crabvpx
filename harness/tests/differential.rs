use harness::decoder::{CrabVpxDecoder, LibVpxOracleDecoder, VideoDecoder};
use harness::ivf::IvfParser;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_differential() {
    // Get test vectors directory from env var or default to ../libvpx-test-data
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let test_data_dir = manifest_dir.parent().unwrap().join("libvpx-test-data");

    assert!(test_data_dir.exists(), "Test vectors not downloaded");

    let mut ivf_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&test_data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ivf") {
                ivf_files.push(path);
            }
        }
    }

    assert!(!ivf_files.is_empty(), "No IVF files found");

    for file in &ivf_files {
        println!("Testing file {:?}", file);
        let mut ivf = IvfParser::new(file).expect("Failed to parse IVF");

        let mut oracle = LibVpxOracleDecoder::new();
        oracle.init().expect("Oracle init failed");

        let mut crab = CrabVpxDecoder::new();
        crab.init().expect("CrabVPX init failed");

        let mut frame_count = 0;
        while let Ok(Some(frame)) = ivf.next_frame() {
            oracle
                .decode_frame(&frame.payload)
                .expect("Oracle decode failed");
            crab.decode_frame(&frame.payload)
                .expect("CrabVPX decode failed");

            let oracle_hash = oracle.get_frame().expect("Oracle get_frame failed");
            let crab_hash = crab.get_frame().expect("CrabVPX get_frame failed");

            assert_eq!(
                oracle_hash, crab_hash,
                "Frame output mismatch at frame {} in {:?}",
                frame_count, file
            );

            frame_count += 1;
        }
    }
}
