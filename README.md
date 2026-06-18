# CrabVPX

A **memory-safe, pure-Rust VP8 video decoder**. Originally lifted from `libvpx`
with `c2rust` and progressively rewritten into safe Rust.

- **Correct:** decode output is bit-exact with `libvpx` across the 35 WebM VP8
  conformance vectors, single- and multi-threaded (verified in CI).
- **Fast:** decode throughput is at parity with `libvpx` (within ±3%) — LLVM
  auto-vectorizes the hot loops, so no hand-written SIMD is required.
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

## Provenance

## Project Status Checklist

### Phase 1: Preparation & Configuration
- [x] Clone upstream `libvpx` repository.
- [x] Configure `libvpx` build strictly for VP8 decoding (e.g., `--disable-vp9`, `--disable-vp8-encoder`).
- [x] Generate `compile_commands.json` using `bear`.

### Phase 2: Transpilation (The Lift)
- [x] Run `c2rust transpile` on the compilation database.
- [x] Initialize `crabvpx` Cargo project.
- [x] Integrate generated `.rs` files into the Cargo structure.

### Phase 3: Hardware Intrinsics & Compilation (The Shift)
- [x] Verify transpiled NEON intrinsics compile purely in Rust using `std::arch::aarch64`.
- [x] Fix transpilation edge cases (atomics, `c_variadic`, types).
- [x] Achieve clean `cargo check` on Rust 2021.

### Phase 4: Stabilization & Upgrade
- [x] Port/integrate tests (harness) to verify decoding correctness against 35 IVF vectors.
- [x] Upgrade codebase to Rust 2024 using `cargo fix --edition`.
- [ ] Implement Phase 5: Incremental Refactoring to Safe Rust APIs (see `docs/refactor_plan.md`).
