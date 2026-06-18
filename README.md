# CrabVPX

A **memory-safe, pure-Rust VP8 video decoder**. Originally lifted from `libvpx`
with `c2rust` and progressively rewritten into safe Rust.

- **Correct:** decode output is bit-exact with `libvpx` across all 62 WebM VP8
  conformance vectors — comprehensive, intra, inter, segmentation, partitions,
  sharpness, and smallsize — single- and multi-threaded (verified in CI).
- **Fast:** on **aarch64**, NEON kernels for the loop filter and sub-pixel
  filter bring decode to roughly **1.1–1.5× of `libvpx`** on heavy content
  and *faster* than libvpx on lighter streams. Other targets use the pure-Rust
  scalar path (LLVM auto-vectorized), ~1.5–3× of libvpx. All SIMD `unsafe`
  is confined to `vp8/common/neon.rs`, each kernel bit-exact-gated against
  its scalar twin.
- **Safe:** builds on stable Rust (no `unsafe`-heavy FFI in the decode path;
  the remaining `unsafe` is confined to the low-level buffer/threading core).
- **Scope:** VP8 *decoding* only (no encoder, no VP9/AV1).

## Usage

```rust
use crabvpx::api::{Decoder, Plane, Vp8Decoder};

let mut dec = Vp8Decoder::new();
dec.init()?;
dec.decode(frame)?; // one compressed VP8 frame
while let Some(img) = dec.get_frame()? {
    let y = img.plane(Plane::Y).unwrap();
    let stride = img.stride(Plane::Y);
    // ... use img.width(), img.height(), planes ...
}
```

Multithreaded decoding is opt-in via `CRABVPX_THREADS` (default `1`).

## Provenance & license

crabvpx is a Rust port of the VP8 decoder from
[libvpx](https://github.com/webmproject/libvpx) — produced with `c2rust` and
progressively rewritten into safe Rust. It is 3-Clause BSD, retaining the
WebM Project's original copyright and patent grant. See
[`PROVENANCE.md`](PROVENANCE.md), [`LICENSE`](LICENSE), and [`PATENTS`](PATENTS).
