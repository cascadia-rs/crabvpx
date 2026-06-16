#![no_main]
//! Fuzz the VP8 frame decoder directly: the entire input is treated as one
//! compressed frame payload. This is the most direct surface and needs no
//! seed corpus -- any byte string is a candidate (usually invalid) frame.
//!
//! Contract under test: decoding arbitrary/hostile bytes must never panic or
//! exhibit undefined behavior; it may only return an error.

use libfuzzer_sys::fuzz_target;

use crabvpx::api::{Decoder, Vp8Decoder};

fuzz_target!(|data: &[u8]| {
    let mut dec = Vp8Decoder::new();
    if dec.init().is_err() {
        return;
    }
    if dec.decode(data).is_err() {
        return;
    }
    while let Ok(Some(_frame)) = dec.get_frame() {}
});
