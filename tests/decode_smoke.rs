//! Minimal decode smoke test.
//!
//! Decodes the first key frame of a small in-repo vector and checks a frame
//! comes out with sane dimensions — a fast, self-contained exercise of the real
//! reconstruction path, distinct from the full differential corpus.
//!
//! It's also the intended target for `cargo miri test` once the decode path is
//! Miri-clean. With `setjmp`/`longjmp` removed, Miri now reaches this far but
//! reports a pre-existing Stacked Borrows violation in the frame-buffer pointer
//! handling (`AlignedBox`/`buffer_alloc`); fixing that provenance issue is the
//! remaining step before this can gate under Miri.

use crabvpx::api::{Decoder, Plane, Vp8Decoder};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn first_frame_payload(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join(name);
    let mut file = File::open(&path).expect("open vector");
    let mut header = [0u8; 32];
    file.read_exact(&mut header).expect("ivf header");
    assert_eq!(&header[0..4], b"DKIF", "not an IVF file");
    let mut fh = [0u8; 12];
    file.read_exact(&mut fh).expect("frame header");
    let size = u32::from_le_bytes(fh[0..4].try_into().unwrap()) as usize;
    let mut payload = vec![0u8; size];
    file.read_exact(&mut payload).expect("frame payload");
    payload
}

#[test]
fn decodes_one_keyframe() {
    let payload = first_frame_payload("vp80-00-comprehensive-001.ivf");

    let mut dec = Vp8Decoder::new();
    dec.init().expect("init");
    dec.decode(&payload).expect("decode keyframe");

    let img = dec
        .get_frame()
        .expect("get_frame")
        .expect("a displayed frame");
    assert!(img.width() > 0 && img.height() > 0, "sane dimensions");
    let y = img.plane(Plane::Y).expect("Y plane");
    assert!(
        y.len() >= img.height() as usize * img.stride(Plane::Y),
        "Y plane covers the frame"
    );
}
