# CrabVPX Refactoring Plan

The `crabvpx` project successfully decodes VP8 video using a transpiled, `unsafe` Rust codebase (upgraded to Rust 2024). The next major goal is to incrementally convert this highly unsafe, C-like architecture into an idiomatic, safe Rust library without causing performance regressions.

## Refactoring Philosophy
- **Outside-In Approach:** Start by refactoring the public API boundary (the interface the user interacts with) into safe Rust. Then, systematically move deeper into the internal decoding pipeline.
- **Test-Driven:** Rely heavily on the existing `harness` to ensure decoding correctness after every single PR or logical change. 
- **Type Safety over Pointers:** Replace raw pointers (`*mut u8`) and length arguments with safe Rust slices (`&mut [u8]`).
- **Memory Management:** Eliminate manual `malloc`/`free` or `calloc` calls (via `vpx_mem.rs`) in favor of Rust's `Vec`, `Box`, and standard allocation mechanisms.

## Metrics
- **Initial Baseline `unsafe` occurrences:** 1340
- **Current `unsafe` occurrences:** (Run `./scripts/count_unsafe.sh` to update)

## Phase 5: Incremental Refactoring Strategy

### Step 1: The Safe Public API Boundary
Currently, the decoder is initialized and used via the C API exported in `src/vpx/vpx_decoder.rs` and `src/vpx/vpx_codec.rs`.
- **Goal:** Create a clean, safe wrapper around the `vpx_codec_ctx_t` lifecycle.
- **Tasks:**
  - Create a new `Decoder` struct in safe Rust.
  - Implement `Drop` for `Decoder` to safely call `vpx_codec_destroy`, eliminating manual memory leaks.
  - Wrap the `vpx_codec_decode` function to accept `&[u8]` (the compressed frame payload) instead of `*const u8` + `size`.
  - Wrap `vpx_codec_get_frame` to return an `Option<Image>` where `Image` safely encapsulates the `vpx_image_t` planes.

### Step 2: Safe Memory Allocation
The core of `libvpx` relies heavily on custom memory allocators defined in `vpx_mem`.
- **Goal:** Replace C-style manual allocations with Rust's global allocator.
- **Tasks:**
  - Identify the primary buffer structs (like `YV12_BUFFER_CONFIG` used for frame buffers).
  - Modify their initialization to use `Vec::with_capacity` or `Box::new` instead of `vpx_memalign` or `vpx_calloc`.
  - Ensure that the alignment requirements of the SIMD instructions (NEON requires specific byte alignments) are respected when allocating via Rust.

### Step 3: Eliminating Unsafe Threading
The multithreaded decoding currently relies on `pthread` implementations transpiled into `vpx_thread.rs`.
- **Goal:** Replace `pthread` primitives with standard Rust concurrency (`std::thread`, `std::sync::Mutex`, `std::sync::mpsc`).
- **Tasks:**
  - Audit `vpx_thread.rs` and understand the `VPxWorker` lifecycle.
  - Replace the `pthread_mutex_t` and `pthread_cond_t` with `std::sync::Mutex` and `std::sync::Condvar`.
  - Migrate the worker threads to use `std::thread::spawn` and safe closures instead of raw `unsafe extern "C"` function pointers.

### Step 4: Core Decoding Pipeline (The Hard Part)
Once the perimeter is safe and memory is managed by Rust, we tackle the core logic (e.g., `decodeframe.rs`, `decodemv.rs`, `reconinter.rs`).
- **Goal:** Incrementally reduce `unsafe` blocks within the dense math algorithms.
- **Tasks:**
  - Convert pointer arithmetic inside loops to slice iterators (`.iter()`, `.chunks_mut()`).
  - Isolate the hardware intrinsic calls (`std::arch::aarch64`) into specific, small `unsafe` functions with safe wrappers, rather than leaving entire modules marked as `unsafe`.

## Git Workflow
For every sub-step completed:
1. Run `cargo check` and `cargo fmt`.
2. Run `./run_harness.sh` to ensure all 35 IVF test vectors still pass.
3. Commit with a descriptive message outlining which `unsafe` patterns were removed.