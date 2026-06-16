#![no_main]
//! Fuzz the decoder over a multi-frame IVF stream. The input is parsed as IVF
//! (panic-free, bounds-checked here) and each frame is fed to the decoder in
//! sequence, exercising key-frame + inter-frame/reference paths.
//!
//! Seed this target with the project's `test_data/*.ivf` vectors so the fuzzer
//! starts from valid bitstreams and mutates outward.
//!
//! Contract under test: no input may cause a panic or undefined behavior.

use libfuzzer_sys::fuzz_target;

use crabvpx::api::{Decoder, Vp8Decoder};

/// Cap to keep individual fuzz iterations cheap.
const MAX_FRAMES: usize = 4096;

/// Split an IVF byte stream into frame payloads without ever panicking.
fn ivf_frames(data: &[u8]) -> Vec<&[u8]> {
    let mut frames = Vec::new();
    if data.len() < 32 || &data[0..4] != b"DKIF" {
        return frames;
    }
    let mut pos = 32usize;
    while pos + 12 <= data.len() && frames.len() < MAX_FRAMES {
        let size = match data[pos..pos + 4].try_into() {
            Ok(bytes) => u32::from_le_bytes(bytes) as usize,
            Err(_) => break,
        };
        pos += 12;
        let end = match pos.checked_add(size) {
            Some(e) if e <= data.len() => e,
            _ => break,
        };
        frames.push(&data[pos..end]);
        pos = end;
    }
    frames
}

fuzz_target!(|data: &[u8]| {
    let frames = ivf_frames(data);
    if frames.is_empty() {
        return;
    }
    let mut dec = Vp8Decoder::new();
    if dec.init().is_err() {
        return;
    }
    for frame in frames {
        if dec.decode(frame).is_err() {
            break;
        }
        while let Ok(Some(_frame)) = dec.get_frame() {}
    }
});
