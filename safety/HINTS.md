# VP8 Decoder Safety Hints

## Current Progress (May 2026)
- **Merge Conflict Resolution**: Resolved all merge conflicts across `src/api.rs`, `harness/src/decoder.rs`, and `scripts/compare.py`. Integrated `Image<'a>` struct and GAT `Decoder` trait with harness MD5 verification. Differential testing via `./scripts/compare.py` passes 100% (1160 frames).
- **Build Fixes**: Modified `build.rs` to conditionalize ARM assembly files based on target architecture. `crabvpx` now successfully compiles on x86_64 (library only).
- **Boolean Decoder**: Implemented `SafeBoolDecoder<'a>` in `src/vp8/decoder/dboolhuff.rs` following `zero_unsafe_strategy.md`. Verified in isolation via `cargo +nightly test --test bool_decoder_test`.
- **Global State**: Converted `vp8_norm` in `src/vp8/common/entropy.rs` from `static mut` to immutable `static`.
- **SafeBoolDecoder Integration**: Removed duplicated `vp8dx_decode_bool` from `detokenize.rs`, `decodeframe.rs`, and `decodemv.rs`. Implemented a centralized `vp8dx_decode_bool` in `dboolhuff.rs` that delegates to `SafeBoolDecoder`.
- **x86_64 Testing Harness**: Implemented `simd_shim.rs` to forward missing `_neon` symbols to their `_c` equivalents on non-ARM targets. Also added a `pthread_once` shim in `thread_shim.rs` to fix Linux glibc compatibility issues with Darwin-transpiled `pthread_once_t` locks. Integration test harness `just compare` now runs and passes 35/35 vectors.
- **Frame Memory Management**: Refactored `YV12_BUFFER_CONFIG` memory allocation in `src/vpx_scale/generic/yv12config.rs` to use aligned Rust vectors (`Vec<Align32>`) instead of legacy `vpx_memalign`/`vpx_free`.
- **Safe Memory Allocation**: Replaced legacy `vpx_memalign` and `vpx_free` calls with `Box::try_new_zeroed()` and `Box::from_raw()` for `VP8D_COMP` allocation in `src/vp8/decoder/onyxd_if.rs`.
- **Safe Image Allocation**: Replaced `calloc` and `free` with `Box::try_new_zeroed()` and `Box::from_raw()` for `vpx_image_t` struct allocation in `src/vpx/src/vpx_image.rs`.
- **Safe Memory Allocation**: Introduced `AlignedBox` in `src/vpx_mem/vpx_mem.rs` and refactored `vpx_memalign` / `vpx_free` to use it. Replaced `vpx_memalign` and `vpx_free` FFI calls in `src/vpx/src/vpx_image.rs` with `AlignedBox::new().into_raw()` and `AlignedBox::from_raw()` for `img_data` buffer allocation.
- **Public API Boundary**: Implemented safe `Image<'a>` wrapper around `vpx_image_t` and updated `Decoder` trait in `src/api.rs` using GATs. `get_frame` now returns `Option<Image<'a>>` providing safe slice access to image planes. Updated integration harness to match.
- **Bitstream Parser**: Refactored `GetSigned` in `src/vp8/decoder/detokenize.rs` to delegate to `vp8dx_decode_bool(br, 128)`, eliminating manual C-style bitstream arithmetic and ensuring all bool decoding passes through `SafeBoolDecoder`.
- **Safe Multithreading Allocation**: Replaced legacy `vpx_calloc` and `vpx_memalign` calls with `Box::into_raw(vec![...].into_boxed_slice())` and `AlignedBox::new()` for thread management allocations (`h_decoding_thread`, `h_event_start_decoding`, `de_thread_data`, `mb_row_di`) in `src/vp8/decoder/threading.rs`.
- **Safe Temporary Buffer Allocation**: Replaced legacy `vpx_calloc`, `vpx_malloc`, and `vpx_memalign` calls with `Box::into_raw(vec![...].into_boxed_slice())` and `AlignedBox::new()` for multithreading temporary buffers (`mt_yabove_row`, `mt_uabove_row`, `mt_vabove_row`, `mt_yleft_col`, `mt_uleft_col`, `mt_vleft_col`, `mt_current_mb_col`) in `src/vp8/decoder/threading.rs`.
- **Motion Vector Decoding**: Refactored `read_mvcontexts` in `src/vp8/decoder/decodemv.rs` to use safe slice indexing and iterators instead of raw pointer arithmetic. Converted `vp8_mv_update_probs` in `src/vp8/common/entropymv.rs` to safe public static, eliminating FFI imports. Reduced unsafe count by 1.

## Architectural Quirks to Watch Out For
- **c2rust Duplication**: Functions that were `static inline` in C headers (specifically `vp8dx_decode_bool` from `dboolhuff.h`) were duplicated by `c2rust` into every Rust module that called them. (Resolved for `vp8dx_decode_bool`).
- **Hardcoded NEON Calls**: The transpiled Rust code explicitly calls `_neon` function names (e.g., `vp8_bilinear_predict16x16_neon` in `decodeframe.rs`), because it was transpiled from C code where NEON macros were active. This causes link errors on x86_64 when building the integration `harness`.
- **Duplicated Structs**: Struct definitions like `YV12_BUFFER_CONFIG` and `VP8Common` were duplicated by `c2rust` across dozens of files. Do not attempt to deduplicate them yet; maintain raw pointer boundaries between modules to avoid FFI type mismatches.

## Next Steps for Future Agents
1. **Core Decoding Pipeline**: Begin Phase 5, Step 4 of `docs/refactor_plan.md`. Convert pointer arithmetic inside loops to slice iterators in core decoding files (`decodeframe.rs`, `decodemv.rs`).

