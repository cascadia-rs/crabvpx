//! Single- vs multi-threaded decode equivalence.
//!
//! The differential gate proves each thread mode bit-exact against the libvpx
//! reference, but CI only ever exercises one multithreaded configuration
//! (`CRABVPX_THREADS=4`). Multithreaded decode partitions macroblock rows
//! across workers, so different thread counts land the partition boundaries on
//! different rows; this test pins the bit-exact-across-thread-counts invariant
//! directly, decoding every `test_data/*.ivf` vector single-threaded and at a
//! range of worker counts and asserting identical per-frame MD5s. It needs no
//! reference files — the single-threaded run is its own oracle.

use crabvpx::api::{Decoder, Vp8Decoder};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Worker counts checked against the single-threaded baseline. Includes a
/// non-power-of-two (3) to land partition splits on odd row boundaries.
const THREAD_COUNTS: &[u32] = &[2, 3, 4, 8];

/// Minimal IVF demuxer: 32-byte file header, then per-frame 12-byte headers
/// (4-byte LE size + 8-byte timestamp) followed by the payload.
fn read_ivf_frames(path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 32];
    file.read_exact(&mut header)?;
    if &header[0..4] != b"DKIF" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "not an IVF file",
        ));
    }
    let mut frames = Vec::new();
    loop {
        let mut fh = [0u8; 12];
        match file.read_exact(&mut fh) {
            Ok(()) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        let size = u32::from_le_bytes(fh[0..4].try_into().unwrap()) as usize;
        let mut payload = vec![0u8; size];
        file.read_exact(&mut payload)?;
        frames.push(payload);
    }
    Ok(frames)
}

/// Decode pre-demuxed frames with `threads` workers, returning one MD5 per
/// displayed frame.
fn decode_md5s(frames: &[Vec<u8>], threads: u32) -> Vec<String> {
    // The worker count is read from the environment at `init()`. This test has
    // a single test function, so there is no concurrent reader to race with.
    unsafe { std::env::set_var("CRABVPX_THREADS", threads.to_string()) };
    let mut dec = Vp8Decoder::new();
    dec.init().expect("init");

    let mut hashes = Vec::new();
    for payload in frames {
        dec.decode(payload).expect("decode");
        while let Some(frame) = dec.get_frame().expect("get_frame") {
            hashes.push(frame.md5());
        }
    }
    hashes
}

#[test]
fn single_and_multi_threaded_decode_match() {
    let test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
    let mut ivf_files: Vec<PathBuf> = fs::read_dir(&test_data_dir)
        .expect("read test_data/")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "ivf"))
        .collect();
    ivf_files.sort();
    assert!(!ivf_files.is_empty(), "no .ivf vectors in test_data/");

    let mut failures = Vec::new();
    for ivf in &ivf_files {
        let name = ivf.file_name().unwrap().to_string_lossy();
        let frames = read_ivf_frames(ivf).expect("demux IVF");
        let baseline = decode_md5s(&frames, 1);

        for &threads in THREAD_COUNTS {
            let got = decode_md5s(&frames, threads);
            if got.len() != baseline.len() {
                failures.push(format!(
                    "{name} @ {threads}t: frame count {} != single-threaded {}",
                    got.len(),
                    baseline.len()
                ));
                continue;
            }
            for (i, (a, b)) in got.iter().zip(baseline.iter()).enumerate() {
                if a != b {
                    failures.push(format!(
                        "{name} @ {threads}t: frame {i} mismatch ({a} != single-threaded {b})"
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "single- vs multi-threaded decode diverged:\n{}",
        failures.join("\n")
    );
}
