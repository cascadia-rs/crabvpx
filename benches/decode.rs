//! Decode throughput benchmarks.
//!
//! Measures end-to-end VP8 decode speed through the public `crabvpx::api`
//! against representative test vectors (intra-heavy, inter, segmentation).
//! Frame payloads are demuxed up front so the benchmark times only decoding.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crabvpx::api::{Decoder, Vp8Decoder};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Demux an IVF file into its compressed frame payloads.
fn read_ivf_frames(path: &Path) -> Vec<Vec<u8>> {
    let mut file = File::open(path).expect("open ivf");
    let mut header = [0u8; 32];
    file.read_exact(&mut header).expect("ivf header");
    assert_eq!(&header[0..4], b"DKIF", "not an IVF file");

    let mut frames = Vec::new();
    loop {
        let mut fh = [0u8; 12];
        if file.read_exact(&mut fh).is_err() {
            break;
        }
        let size = u32::from_le_bytes(fh[0..4].try_into().unwrap()) as usize;
        let mut payload = vec![0u8; size];
        file.read_exact(&mut payload).expect("frame payload");
        frames.push(payload);
    }
    frames
}

/// Decode every frame with a fresh decoder, draining displayed frames.
fn decode_all(frames: &[Vec<u8>], threads: u32) {
    // Safety/config knob honored by the decoder's init path.
    unsafe { std::env::set_var("CRABVPX_THREADS", threads.to_string()) };
    let mut dec = Vp8Decoder::new();
    dec.init().expect("init");
    for payload in frames {
        // Malformed frames are out of scope here; vectors are valid.
        dec.decode(payload).expect("decode");
        while let Some(frame) = dec.get_frame().expect("get_frame") {
            std::hint::black_box(frame.width());
        }
    }
}

fn vector_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join(name)
}

fn bench_decode(c: &mut Criterion) {
    // One representative vector per content class.
    let vectors = [
        ("intra", "vp80-01-intra-1400.ivf"),
        ("inter", "vp80-02-inter-1402.ivf"),
        ("segmentation", "vp80-03-segmentation-1401.ivf"),
    ];

    let mut group = c.benchmark_group("decode");
    for (label, file) in vectors {
        let path = vector_path(file);
        if !path.exists() {
            continue;
        }
        let frames = read_ivf_frames(&path);
        let total_bytes: u64 = frames.iter().map(|f| f.len() as u64).sum();
        group.throughput(Throughput::Bytes(total_bytes));
        group.bench_with_input(BenchmarkId::new("st", label), &frames, |b, frames| {
            b.iter(|| decode_all(frames, 1));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
