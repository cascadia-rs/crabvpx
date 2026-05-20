# VP8 Decoder Safety Hints

See remaining_refactoring_work_items.md for an overview of particular unsafe blocks.
## Current Status (May 2026)
* **Dead Darwin Pthread and System Types Removal (decodeframe.rs, decodemv.rs, detokenize.rs, threading.rs, vp8_dx_iface.rs)**:
  - Identified and removed unused Darwin-specific pthread and system type definitions (e.g., `__darwin_pthread_handler_rec`, `_opaque_pthread_t`, `__darwin_pthread_t`, `mach_port_t`, `size_t` where unused, etc.) from multiple decoder files.
  - These types were transpiled leftovers that were completely unreferenced in the Rust implementation (which uses safe standard Rust threading and sync).
  - Removing `__darwin_pthread_handler_rec` (which contained an `unsafe extern "C" fn` pointer) successfully eliminated **5 unsafe keywords** globally, reducing the remaining unsafe count from 360 to 355.
  - Verified 100% bit-identical correctness across all 1160 test frames.

* **Obsolete #[unsafe(no_mangle)] Removal in Mode MVs (decodemv.rs)**:
  - Removed obsolete `#[unsafe(no_mangle)]` and `pub` attributes from the static table `vp8_sub_mv_ref_prob3` in `src/vp8/decoder/decodemv.rs`.
  - Since this table is only accessed internally via standard Rust imports (in `decodemv.rs` itself), the FFI export attribute and public visibility were completely obsolete.
  - This successfully eliminated **1 unsafe keyword** globally, reducing the remaining unsafe count from 361 to 360.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Dead Code & Unsafe Sync Impl Removal in Entropy (entropy.rs)**:
  - Identified `vp8_extra_bits` array and `vp8_extra_bit_struct` as completely unused dead code in the decoder (entropy decoding of categories is fully inlined/optimized in `detokenize.rs` using local tables).
  - Completely removed `vp8_extra_bits` static array, `vp8_extra_bit_struct` definition, `vp8_tree_p` type definition, and the `unsafe impl Sync for vp8_extra_bit_struct` from `src/vp8/common/entropy.rs`.
  - This successfully eliminated **1 unsafe keyword** globally, reducing the remaining unsafe count from 362 to 361.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Obsolete #[unsafe(no_mangle)] Removal in Entropy (entropy.rs)**:
  - Removed 11 obsolete `#[unsafe(no_mangle)]` attributes from static tables, probability arrays, and trees in `src/vp8/common/entropy.rs`.
  - Since CrabVPX is built as a Rust library and these tables are only accessed internally via standard Rust imports, these FFI export attributes were completely obsolete.
  - This successfully eliminated **11 unsafe keywords** globally, reducing the remaining unsafe count from 373 to 362.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Dead Code Elimination in blockd.rs**:
  - Audited `blockd.rs` and found C static tables `vp8_block2left` and `vp8_block2above` were completely unused in the decoder.
  - Removed these definitions from `src/vp8/common/blockd.rs` (emptying the file) and their extern declarations from `src/vp8/common/blockd.h`.
  - This successfully eliminated **2 unsafe keywords** globally (from `#[unsafe(no_mangle)]` attributes), reducing the remaining unsafe count from 375 to 373.
  - Verified 100% correctness with all 1160 differential test frames passing successfully.
* **Safe Wrapper for Decoded Images & Odd-Dimension Support (api.rs)**:
  - Refactored `Image` struct in `src/api.rs` to hold a safe Rust reference `&'a vpx_image_t` instead of a raw pointer `*const vpx_image_t`.
  - This enabled making `width()`, `height()`, and `bit_depth()` entirely safe methods, removing 3 `unsafe` blocks.
  - Completely rewrote `md5()` to use the safe `plane()` slice-based accessor, eliminating another `unsafe` block.
  - Fixed a critical bug in `plane()` height calculation for odd-dimension video vectors (e.g. 175x143). Height of U/V planes was previously calculated as `d_h >> shift` which rounded down (e.g. 143 >> 1 = 71). This was corrected to round up `(d_h + (1 << shift) - 1) >> shift` (e.g. (143 + 1) >> 1 = 72) to match the actual buffer allocations, preventing silent row-skipping in MD5 calculation for odd-sized frames.
  - Centralized the necessary unsafety in `Vp8Decoder::get_frame` where the raw pointer returned by the decoder instance is safely dereferenced after null-checking.
  - Net reduction of **3 unsafe blocks** globally (remaining count: 375).
  - Verified 100% correctness with all 1160 differential test frames passing successfully.
* **Obsolete #[unsafe(no_mangle)] Removal in Entropy MV (entropymv.rs)**:
  - Removed obsolete `#[unsafe(no_mangle)]` attribute from the static table `vp8_default_mv_context` in `src/vp8/common/entropymv.rs`.
  - Since this table is only accessed internally via safe Rust imports (in `decodeframe.rs`), the FFI export attribute was completely obsolete.
  - This successfully eliminated **1 unsafe keyword** globally, reducing the remaining unsafe count from 379 to 378.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Obsolete #[unsafe(no_mangle)] Removal in Mode Contexts (modecont.rs)**:
  - Removed obsolete `#[unsafe(no_mangle)]` attribute from the static table `vp8_mode_contexts` in `src/vp8/common/modecont.rs`.
  - Since this table is only accessed internally via safe Rust imports (in `findnearmv.rs` and `decodemv.rs`), the FFI export attribute was completely obsolete.
  - This successfully eliminated **1 unsafe keyword** globally, reducing the remaining unsafe count from 380 to 379.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Dead FFI/Unused Code Elimination in Tree Coder (treecoder.rs)**:
  - Completely eliminated unused transpiled FFI functions `vp8_tokens_from_tree`, `vp8_tokens_from_tree_offset`, and `vp8_tree_probs_from_distribution` along with their local safe helpers `detect_tree_slice`, `tree2tok_safe`, and `branch_counts_safe` from `src/vp8/common/treecoder.rs`.
  - Since CrabVPX is strictly a VP8 decoder, these encoder-only routines were completely dead code and not referenced anywhere in the codebase or test harness.
  - This successfully eliminated **9 unsafe blocks/keywords** globally, reducing the remaining unsafe count from 389 to 380.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Dead FFI Wrappers Cleanup in Boolean Decoder (dboolhuff.rs)**:
  - Completely eliminated the unused FFI wrappers `vp8dx_start_decode` and `vp8dx_bool_decoder_fill` from `src/vp8/decoder/dboolhuff.rs`.
  - These wrappers were legacy C-ABI entry points that were no longer used by the internal Rust decoder (which uses `vp8dx_start_decode_safe` and `SafeBoolDecoder` directly).
  - Updated `tests/bool_decoder_test.rs` to use `vp8dx_start_decode_safe` to maintain test coverage.
  - This successfully eliminated **2 unsafe blocks** and **2 unsafe attributes** (4 `unsafe` keywords total) globally, reducing the remaining unsafe count from 393 to 389.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Obsolete #[unsafe(no_mangle)] Removal in Entropy Mode (entropymode.rs)**:
  - Removed 26 obsolete `#[unsafe(no_mangle)]` attributes from all static tables, probability arrays, and trees in `src/vp8/common/entropymode.rs`.
  - Since CrabVPX is built as a Rust library (rlib) and all these tables are only accessed internally via standard Rust imports (e.g., in `decodemv.rs` and `treecoder.rs`), these FFI export attributes were completely obsolete.
  - This successfully eliminated **26 unsafe keywords** globally, reducing the remaining unsafe count from 419 to 393 (crossing the 70% cleanup milestone!).
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Dead Code and FFI Cleanup in Entropy Mode (entropymode.rs)**:
  - Completely eliminated the unused FFI wrapper `vp8_mv_cont` and its internal helper `vp8_mv_cont_safe` from `src/vp8/common/entropymode.rs`.
  - This successfully eliminated **1 unsafe block** and **1 unsafe attribute** globally, reducing the remaining unsafe count from 421 to 419.
  - Verified 100% bit-identical correctness across all 1160 test frames.
* **Test Harness and Unnecessary Attribute Cleanup (findnearmv.rs, alloccommon.rs, bool_decoder_test.rs)**:
  - Fixed `tests/bool_decoder_test.rs` to compile with modernized `BOOL_DECODER` by using `BOOL_DECODER::default()`.
  - Removed unnecessary `unsafe` block around `vp8dx_start_decode` call in `tests/bool_decoder_test.rs`.
  - Removed obsolete `#[unsafe(no_mangle)]` attribute from internal static table `vp8_mbsplit_offset` in `src/vp8/common/findnearmv.rs` as it is only used internally via safe Rust imports.
  - Removed obsolete `#[unsafe(no_mangle)]` attribute from internal function `vp8_alloc_frame_buffers` in `src/vp8/common/alloccommon.rs` as it is only called internally.
  - This successfully reduced the remaining unsafe count from 423 to 421 (2 unsafe keywords removed) while keeping 100% bit-identical correctness across all 1160 test frames.
* **FFI Wrapper and Attribute Cleanup in Common Modules (extend.rs, vp8_loopfilter.rs, filter.rs, reconintra.rs)**:
  - Completely eliminated unused FFI wrappers `vp8_copy_and_extend_frame` and `vp8_copy_and_extend_frame_with_rect` from `src/vp8/common/extend.rs`.
  - Completely eliminated unused FFI wrappers `vp8_loop_filter_frame`, `vp8_loop_filter_frame_yonly`, and `vp8_loop_filter_partial_frame` from `src/vp8/common/vp8_loopfilter.rs`.
  - Removed obsolete `#[unsafe(no_mangle)]` attributes from internal-only functions `vp8_loop_filter_update_sharpness`, `vp8_loop_filter_frame_init`, and `vp8_extend_mb_row`.
  - Removed obsolete `#[unsafe(no_mangle)]` from internal static tables `vp8_bilinear_filters` and `vp8_sub_pel_filters` in `src/vp8/common/filter.rs`.
  - Removed obsolete `#[unsafe(no_mangle)]` and `extern "C"` FFI boundary from `vp8_init_intra_predictors` in `src/vp8/common/reconintra.rs`.
  - This successfully reduced the remaining unsafe count from 439 to 423 (16 unsafe keywords removed) while keeping 100% bit-identical correctness across all 1160 test frames.
* **Safe Pointer Offset Elimination in Frame Buffer Reallocation (yv12config.rs)**:
  - Refactored `vp8_yv12_realloc_frame_buffer_safe` in `src/vpx_scale/generic/yv12config.rs` to eliminate an `unsafe` block.
  - Replaced unsafe raw pointer `offset` arithmetic with safe `usize` casts for calculating `y_buffer`, `u_buffer`, and `v_buffer` addresses.
  - This successfully reduced the remaining unsafe block count from 440 to 439, while maintaining 100% bit-identical correctness across all 1160 test frames.
* **Obsolete legacy `vp8dx_decode_bool` shims removed (dboolhuff.rs)**:
  - Completely eliminated the unused `vp8dx_decode_bool` FFI wrapper and the `vp8dx_decode_bool_safe` helper from `src/vp8/decoder/dboolhuff.rs`.
  - Since the entire internal Rust decoder was previously migrated to outer-row `SafeBoolDecoder` slice instantiations, these legacy shims were completely dead code.
  - Removing them eliminated 1 raw pointer dereference `unsafe` block and 1 `#[unsafe(no_mangle)]` attribute globally, bringing the remaining unsafe block count from 442 down to 440.
  - Differential tests continue to pass 100% perfectly for all 1160 frames.
* **Safe YV12 Buffer Allocation Refactoring (yv12config.rs)**:
  - Refactored `vp8_yv12_de_alloc_frame_buffer_safe` and `vp8_yv12_realloc_frame_buffer_safe` in `src/vpx_scale/generic/yv12config.rs` to use `AlignedBox` (from `vpx_mem.rs`) instead of the fragile `Vec::from_raw_parts` / `core::mem::forget` hack.
  - Removed the obsolete `Align32` struct from `yv12config.rs` as it is now superseded by `AlignedBox`.
  - This replacement keeps FFI compatibility while using a much safer and unified allocation abstraction. The remaining global unsafe block count is stable at 440, and all 1160 differential test frames pass perfectly.
* **Modernized BOOL_DECODER & Safe Fat Pointer Slicing (dboolhuff.rs)**:
  - Refactored `vp8_reader` (alias `BOOL_DECODER`) struct in `src/vp8/common/types.rs` to use a single fat slice raw pointer `user_buffer: *const [u8]` instead of the `user_buffer: *const u8` and `user_buffer_end: *const u8` pointer pair.
  - Replaced `offset_from` and `from_raw_parts` inside `SafeBoolDecoder::from_bool_decoder` with a direct, safe-checked dereference of `user_buffer` fat pointer.
  - Completely eliminated the raw pointer arithmetic (`bc.user_buffer.add(self.offset)`) and its `unsafe` block in `SafeBoolDecoder::update_bool_decoder` by performing safe Rust slice-indexing (`&self.buffer[self.offset..]`) and casting the remaining slice back to `*const [u8]`.
  - Reduced the global `unsafe` block count from 443 to 442, and verified 100% bit-identical correctness across all 1160 test frames.
* **Safe Multithreaded Loop Filtering (Milestone 3 - Unit 7 Complete)**:
  - Refactored multithreaded loop filtering in `src/vp8/decoder/threading.rs` to use safe disjoint row views, fully resolving the concurrency race conditions in parallel decoding.
  - Implemented `get_row_view_mut` and `get_disjoint_row_views_mut` on `YV12_BUFFER_CONFIG` in `src/vp8/common/types.rs` to dynamically partition the destination frame buffer into disjoint mutable slices per row without any heap allocation overhead (avoiding global allocator lock contention that caused hangs).
  - Designed and implemented `mbloop_filter_horizontal_edge_split_safe` and `vp8_loop_filter_simple_horizontal_edge_split_safe` in `src/vp8/common/loopfilter_filters.rs` to perform horizontal boundary filtering across disjoint row slices safely.
  - Made the boundary horizontal filter dynamic to support both Y (16 lines) and U/V (8 lines) row heights, fixing a critical out-of-bounds panic in chroma filtering.
  - Verified that both single-threaded and multithreaded (tested with `CRABVPX_THREADS=4`) decoding pass 100% bit-identical differential tests for all 1160 frames.
* **FFI Declaration Cleanup & Redundant Unsafe Removal**:
  - Identified and removed dead FFI declarations (`memcpy`, `vpx_memalign`, `vpx_free` in `src/vp8/decoder/onyxd_if.rs`; and `memcpy`, `memset`, `vpx_memalign`, `vpx_malloc`, `vpx_calloc`, `vpx_free` in `src/vp8/decoder/threading.rs`) which were unused in the Rust implementation.
  - Replaced a `memset` call in `vp8_create_decoder_instances` inside `onyxd_if.rs` with a safe Rust null-pointer array assignment.
  - Removed redundant inner `unsafe {` blocks from `vp8_create_decoder_instances` and `vp8_remove_decoder_instances` in `onyxd_if.rs`.
  - This successfully eliminated **2 unsafe keywords** globally, reducing the remaining unsafe count from 445 to 443. All 1160 differential test frames pass perfectly.
* **Safe Decryption FFI Wrapper**:
  - Created `vpx_decrypt_safe` in `src/vp8/common/types.rs` to safely wrap the FFI decryption callback (`vpx_decrypt_cb`).
  - Refactored call sites in `src/vp8/decoder/dboolhuff.rs` (inside `SafeBoolDecoder::fill`) and `src/vp8/decoder/decodeframe.rs` (inside `read_partition_size` and `decode_frame`) to use this safe wrapper.
  - This successfully eliminated **3 unsafe blocks** from `dboolhuff.rs` and `decodeframe.rs`, while adding **1 unsafe block** inside the new helper in `types.rs`, resulting in a net reduction of **2 unsafe blocks** globally (remaining count: 445). All 1160 differential test frames pass perfectly.
* **Dead FFI Predictor Cleanup & Unsafe Reduction**:
  - Identified that the transpiled C reference predictors (`vp8_sixtap_predict..._c` and `vp8_bilinear_predict..._c`) in `src/vp8/common/filter.rs` were completely unused in our Rust implementation.
  - Our Rust implementation uses `filter_block2d_sixtap_safe` and `filter_block2d_bil_safe` directly via safe wrappers in `safe_predict.rs` (on non-aarch64), or uses NEON assembly (on aarch64), completely bypassing these transpiled C versions.
  - Completely removed these 8 dead FFI predictor functions and their 2 internal unsafe helpers (`get_src_slice`, `get_bil_src_slice`) from `filter.rs`.
  - This successfully eliminated **26 unsafe blocks/keywords** globally, reducing the remaining unsafe count from 473 to 447. All 1160 differential test frames pass perfectly.
* **Safe Tree Pointer Detection & Unsafe Fallback Elimination**:
  - Implemented a 100% safe `detect_tree_slice` helper in `src/vp8/common/treecoder.rs` that matches FFI raw tree pointers (`*const vp8_tree_index`) against the known, safe static VP8 tree arrays (such as `vp8_coef_tree`, `vp8_bmode_tree`, etc.).
  - Exported the private extra bit trees `cat1` to `cat6` from `entropy.rs` as `pub` so they can be matched in `treecoder.rs`.
  - Completely removed the `get_tree_bounds` unsafe recursive function (which did unsafe pointer arithmetic to dynamically find tree sizes) and removed all unsafe fallback paths in `vp8_tokens_from_tree`, `vp8_tokens_from_tree_offset`, and `vp8_tree_probs_from_distribution`.
  - This successfully eliminated **1 unsafe block** overall (since the dead unsafe `get_tree_bounds` function was deleted), reducing the remaining unsafe count from 474 to 473. All 1160 differential test frames pass perfectly.
* **Safe Frame Buffer Slice Refactoring & Pointer Mismatch Fix**:
  - Refactored all 15 slice helper methods in `yv12_buffer_config` in `src/vp8/common/types.rs` (such as `y_slice_safe`, `u_slice_mut_safe`, and `_with_offset_safe` variants) to use safe bounds-checked slicing on `full_buffer_safe()` and `full_buffer_mut_safe()`.
  - Implemented a static `safe_len` helper to dynamically truncate slice lengths to the remaining allocation size, preventing out-of-bounds slice creation panics.
  - Discovered and fixed a critical memory safety bug where `xd.pre` and `xd.dst` `buffer_alloc` fields were mismatched (pointing to incorrect allocations) when their `y_buffer` pointers were mutated to point to different frame buffers.
  - Fixed `xd.pre` mismatch in `decodeframe.rs` and `threading.rs` by explicitly copying `buffer_alloc` and `buffer_alloc_sz` from the reference frame.
  - Fixed `xd.dst` mismatch by initializing `pbi.mb.dst` and `pbi.mb.pre` early in `vp8_decode_frame` on every frame (including key frames), resolving a hidden resolution-change bug.
  - This successfully eliminated **13 unsafe blocks** from `types.rs`, reducing the remaining unsafe count from 487 to 474. All 1160 frames pass differential testing perfectly.
* **Dead Unsafe Code Elimination & Safety Scope Minimization**:
  - Identified and removed 15 unused unsafe methods from `yv12_buffer_config` in `src/vp8/common/types.rs` (e.g., `y_slice`, `y_slice_mut`, and their `_with_offset`/`_view` counterparts). These were dead code left over after the transition to their safe `_safe` counterparts. This reduced the remaining unsafe block count from 502 to 487.
  - Shrunk the scope of the `unsafe` block in `thread_decoding_proc` inside `src/vp8/decoder/threading.rs` to only cover the FFI `setjmp` call. The rest of the function, including `split_mt_mut` and `mt_decode_mb_rows` calls, now executes safely.
* **Safe Fragment Access & Threading Safety Scope Improvement Complete**:
  - Added a safe `get_slice` method to `FRAGMENT_DATA` in `src/vp8/common/types.rs` to encapsulate raw pointer access. Refactored `setup_token_decoder` and `vp8_decode_frame` in `src/vp8/decoder/decodeframe.rs` to use this safe method, completely removing two `unsafe` blocks. Resolved borrow checker issues by copying `pbi.fragments` to local variables, allowing safe disjoint borrowing of `pbi` elsewhere. Reduced unsafe count by 1.
  - Shrunk the giant `unsafe` block in `vp8mt_decode_mb_rows` in `src/vp8/decoder/threading.rs` to only cover the `setjmp` FFI call, making the rest of the function safe and using disjoint borrows.
* **Safe Threading Refactoring (Phase 4 Extension) Complete**: Refactored `h_decoding_thread` in `VP8D_MT_SYNC` to use safe Rust thread handles (`std::thread::JoinHandle`) instead of `pthread_t` raw pointers. Eliminated `vp8_pthread_create` and `vp8_pthread_join` FFI calls entirely, spawning standard Rust threads via `std::thread::Builder` instead. Removed the `unsafe` block in `vp8_decoder_remove_threads` and cleaned up `thread_shim.rs` by removing the obsolete FFI shims, reducing the unsafe count by 9 in total!
* **Dead Pointer Field Elimination (MACROBLOCKD)**: Audited `MACROBLOCKD` struct and identified `recon_above` and `recon_left` raw pointer fields as completely dead (initialized and updated but never read). Removed these fields from `MACROBLOCKD` in `src/vp8/common/types.rs` and their `Default` implementation. Cleaned up their initialization and pointer arithmetic in `src/vp8/decoder/threading.rs` (`mt_decode_mb_rows`), which allowed fully removing one `unsafe` block that was performing raw pointer offsets. Reduced unsafe count by 1.
* **Safe Error Handling Refactoring (vpx_internal_error to trigger)**: Refactored multiple call sites of `vpx_internal_error` in `src/vp8/decoder/threading.rs` to use the safe `vpx_internal_error_info::trigger` method. Removed the `unsafe` block around the `vpx_internal_error` call in `mt_decode_mb_rows`, and fully removed the `vpx_internal_error` FFI extern declaration from `threading.rs`. This reduced the unsafe count by 1.
* **Top-Level FFI Initialization Refactoring (Phase 4 Extension)**: Refactored `vp8_decoder_create_threads` in `src/vp8/decoder/threading.rs` to be a safe Rust function taking `&mut VP8D_COMP` instead of `*mut VP8D_COMP`. Updated call sites in `onyxd_if.rs` and `vp8_dx_iface.rs` to pass safe mutable references (`&mut *pbi`). Cleaned up obsolete declarations. This reduced the unsafe count by 1.
* **Multithreaded Root Destructuring (Phase 3 Extension) Complete**: Successfully refactored `mt_decode_mb_rows` in `src/vp8/decoder/threading.rs` to use safe references and disjoint borrows, fully removing raw pointer aliasing. Introduced `VP8D_MT_SYNC` struct in `src/vp8/common/types.rs` to group multithreaded synchronization fields (`mt_current_mb_col`, `mt_yabove_row`, etc.) and implemented `split_mt_mut` on `VP8D_COMP` to allow safe disjoint borrowing. Removed the giant `unsafe` block in `mt_decode_mb_rows` and replaced it with safe slice indexing for `xd.dst`, `xd.pre`, and `recon_above` setup, reducing overall unsafe usage to only FFI calls.
* **Top-Level FFI Initialization Refactoring (Phase 4 Partial)**: Refactored `create_decompressor` in `src/vp8/decoder/onyxd_if.rs` to be safe and return `Option<Box<VP8D_COMP>>` instead of a raw pointer. Used `ManuallyDrop` to ensure memory safety during `setjmp` setup. Simplified the FFI boundary in `vp8_create_decoder_instances` by removing complex raw pointer offsetting.
* **Top-Level Struct Decoupling Complete**: All three core decoupling phases outlined in [top_level_struct_decoupling_strategy.md](top_level_struct_decoupling_strategy.md) have been successfully achieved! Legacy pointer caches (`mode_info_context`, `above_context`, `left_context`) have been fully eliminated from `MACROBLOCKD` and replaced with absolute slice indices (`mode_info_idx`, `above_context_idx`). Safe destructuring via `split_mut()` is successfully implemented on `VP8D_COMP`.
* **Bitstream FFI Thrashing Mitigation Complete**: All four implementation milestones outlined in [bitstream_thrashing_mitigation_strategy.md](bitstream_thrashing_mitigation_strategy.md) have been successfully achieved! Both single-threaded (`decode_mb_rows`) and multithreaded (`mt_decode_mb_rows`) partition pipelines have been fully converted to outer row root slice instantiations. Per-macroblock temporary slice wrapping, raw pointer syncing, and obsolete C-ABI wrapper declarations (`vp8dx_decode_bool`) have been completely eliminated from internal token decoding paths (`GetCoeffs`, `vp8_decode_mb_tokens`, `vp8_decode_mode_mvs`).
* **Completed Work Items Archive**: For a complete historical audit trail of all completed structural decoupling, dead code cleanup, and memory safety refactoring milestones, see [COMPLETED_WORK_ITEMS.md](COMPLETED_WORK_ITEMS.md).
* **Test Verification**: Differential testing via `./scripts/compare.py` passes with perfect 100% bit-identical matching across all 1160 frames. Unsafe blocks remaining: 506 (fully bypassed FFI for all subpixel predictors including 16x16, 8x8, 8x4, and 4x4 bilinear and sixtap on non-aarch64 by calling native safe Rust implementations directly, bypassed FFI for all 8 loop filters on non-aarch64, and fully refactored multithreaded synchronization to use safe Rust `Arc<Semaphore>` instead of raw `semaphore_t` FFI!).
* **Milestone 1 Complete (Global Mutable State Deprecation)**: All units across `entropymode.rs`, `entropy.rs`, `blockd.rs`, `vpx_config.rs`, `threading.rs`, and ARM NEON helper shims have been successfully converted from legacy `static mut` to immutable safe definitions.
* **Milestone 2 Complete (Filter Pointer Indexing & Sub-Slicing)**: Refactored second-pass prediction multi-tap filters to bounds-checked zipped iterators. Exhaustive verification confirmed all edge loop filtering subroutines (`loopfilter_filters.rs`) are fully utilizing safe disjoint reborrowing.
* **Milestone 3 - Unit 8 & 9 Complete (Safe Predictor Wrappers & Shim Elimination)**: Refactored all subpixel predictor FFI calls to go through safe Rust wrappers in `safe_predict.rs`. Redefined `vp8_subpix_fn_t` to be a safe `fn` pointer taking parent slices and offsets. Fully eliminated `unsafe` blocks and raw pointer slicing from `reconinter.rs`'s `call_subpixel_predict` and all its macroblock-level callers. Enforced strict bounds checks (including negative offsets for sixtap) inside the safe wrappers prior to FFI execution. Bypassed all bilinear and sixtap FFI calls on non-aarch64 with safe Rust equivalents (`filter_block2d_bil_safe` and `filter_block2d_sixtap_safe`) and purged their shims from `simd_shim.rs`.
* **Milestone 3 - Unit 10 Complete (Loop Filter Shim Elimination)**: Audited loop filter shims in `simd_shim.rs` (e.g., `vp8_loop_filter_bh_neon`, `vp8_loop_filter_bv_neon`, etc.). Bypassed FFI for them on non-aarch64 by directly calling safe Rust native implementations in `src/vp8/common/loopfilter_filters.rs` using safe bounds-checked slice views (`y_slice_mut_safe()`, etc.) inside `threading.rs`. Completely cleared `simd_shim.rs` of all loop filter shims!
* **Initialization Refactoring (create_decompressor)**: Refactored `create_decompressor` in `src/vp8/decoder/onyxd_if.rs` to shrink the `unsafe` block scope. Moved safe initialization steps (like `vp8_create_common`, `vp8cx_init_de_quantizer`, `vp8_loop_filter_init`, `vp8_setup_block_dptrs`, and various field assignments) out of the `unsafe` block, leaving only the FFI `setjmp` call as unsafe. Verified that compilation and 100% bit-identical differential tests pass successfully.
* **Safe Semaphore Refactoring (h_event_start_decoding & h_event_end_decoding)**: Refactored the multithreaded synchronization to use safe Rust `Semaphore` (implemented with `Mutex` and `Condvar`) instead of raw pointer FFI semaphores (`semaphore_t`). Updated `VP8D_MT_SYNC` fields `h_event_start_decoding` and `h_event_end_decoding` to use `Arc<Semaphore>`. Eliminated 4 dead FFI functions `vp8_semaphore_create`, `vp8_semaphore_destroy`, `vp8_semaphore_wait`, `vp8_semaphore_signal` from `src/thread_shim.rs` and removed their type aliases. This reduced the unsafe count by 10 in total (including FFI call sites in `threading.rs`).
* **Safe Top-Level Initialization Refactoring (assume_init elimination)**: Refactored `create_decompressor` in `src/vp8/decoder/onyxd_if.rs` to use safe `Box::try_new(VP8D_COMP::default())` instead of `Box::try_new_zeroed()` followed by unsafe `assume_init()`. To support this, derived `Default` for `VP8D_COMP` and all its nested structures in `src/vp8/common/types.rs`. This required implementing `Default` manually for `VP8Common` and `loop_filter_info_n` due to compiler limitations regarding `Default` implementation for arrays larger than 32 elements. This completely eliminated one `unsafe` block in `onyxd_if.rs` and reduced the remaining unsafe count to 509.
* **Safe Tree Coder Refactoring (Safe Slice Pre-scanning)**: Refactored `src/vp8/common/treecoder.rs` to completely eliminate all unsafe pointer arithmetic and raw memory offset writes in the recursive token population (`tree2tok`) and probability calculation (`branch_counts`, `vp8_tree_probs_from_distribution`) logic. Designed a safe `get_tree_bounds` pre-scan algorithm to compute the exact token and tree array boundaries, enabling 100% safe slice-based traversal. This reduced the remaining unsafe count to 506.
* **Safe Context & Mode Info Slice Refactoring (Eliminated unsafe pointer arithmetic in macroblockd)**:
  - Refactored `macroblockd` methods `mode_info`, `mode_info_mut`, `contexts_mut`, and `decode_tokens_inputs_mut` in `src/vp8/common/types.rs` to accept safe Rust slices (`&[MODE_INFO]` and `&mut [ENTROPY_CONTEXT_PLANES]`) instead of raw pointers. This completely eliminated 4 `unsafe` blocks that were doing raw pointer offsetting inside these methods!
  - Updated `VP8Common` with safe `mip_slice`, `mip_slice_mut`, `above_context_slice`, and `above_context_slice_mut` helpers, replacing the raw pointer-yielding `mip_ptr`, `above_context_ptr`, etc.
  - Propagated the slice-based signatures down through `src/vp8/common/reconinter.rs` (`vp8_build_inter_predictors_mb`, `vp8_build_inter16x16_predictors_mb`, `build_inter4x4_predictors_mb`, and `build_4x4uvmvs`), `src/vp8/decoder/decodeframe.rs`, and `src/vp8/decoder/threading.rs`.
  - Resolved complex borrow checker issues in multithreaded contexts (`threading.rs`) by: 1) destructuring the slice variables mutably to satisfy Rust's disjoint borrowing rules (allowing simultaneous immutable borrow of `common.fc`), and 2) moving the immutable `lfi_n` reference scope to be strictly local inside the loop filter execution block to avoid overlapping borrows.
  - This successfully reduced the remaining unsafe block count from 506 to 502. Differential differential testing passes perfectly across all 1160 frames.
* **Milestone 1 Complete (Structural Decoupling & Index-Based Referencing)**:
  - Successfully decoupled `macroblockd` from `YV12_BUFFER_CONFIG` by replacing `pre` and `dst` struct fields with index-based referencing fields (`pre_fb_idx`, `dst_fb_idx`) and explicit stride/border metadata fields.
  - Updated all top-level assignments in `vp8_dx_iface.rs`, `decodeframe.rs`, and `threading.rs` to initialize these indices and metadata.
  - Refactored inter-prediction reconstruction functions in `reconinter.rs` to accept `dst_fb` and `pre_fb` references and pass `0` as the slice offset, letting the caller handle macroblock offsetting via the absolute index.
  - Refactored `intra_prediction_down_copy` in `reconintra.rs` to accept slice and stride parameters directly, completely removing its dependency on `MACROBLOCKD`.
  - Resolved complex borrow-checker conflicts in `decodeframe.rs` and `threading.rs` (arising from long-lived `mip` slices locking `common`) by copying the `MODE_INFO` struct early in `decode_macroblock` and using the local copied values for all prediction decisions.
  - Corrected a critical offset mismatch bug where Y/UV reconstruction slices were writing to the top-left of the frame instead of the current macroblock because the base `views_mut()` were not offset by `recon_yoffset` / `recon_uvoffset`.
  - Compilation and all 1160 differential test frames pass perfectly! Remaining unsafe count: 443.
* **Milestone 2 Complete (Refactor Inter-Prediction & Motion Compensation with Explicit Dataflow)**:
  - Successfully implemented 100% safe `views_mut_with_borders` and `views_with_borders` on `YV12_BUFFER_CONFIG` in `types.rs` using pure safe contiguous slicing (`split_at_mut`), completely bypassing FFI/unsafe.
  - Refactored inter-prediction orchestrator functions in `reconinter.rs` (`vp8_build_inter_predictors_mb`, `vp8_build_inter16x16_predictors_mb`, and `build_inter4x4_predictors_mb`) to accept separate Y, U, V plane slices for both destination and reference planes explicitly, rather than passing whole `YV12_BUFFER_CONFIG` structs.
  - Updated call sites in both single-threaded (`decodeframe.rs`) and multithreaded (`threading.rs`) row/macroblock decoding pipelines to extract disjoint plane slices at the orchestrator level using the new border-inclusive views and pass them to the prediction engine.
  - All 1160 differential test frames pass perfectly with bit-identical matching, and remaining unsafe count remains stable at 443.
* **Milestone 3 - Unit 6 Complete (Safe Frame Buffer Row Chunking)**:
  - Implemented `split_rows_mut` on `YV12_BUFFER_CONFIG` in `src/vp8/common/types.rs`.
  - This method uses safe `chunks_mut` to split the active Y, U, and V planes into disjoint mutable slices for each macroblock row.
  - This provides the foundational data structure for disjoint parallel loop filtering (Unit 7).
  - Compilation and all 1160 differential test frames pass perfectly! Remaining unsafe count: 443.


## Architectural Quirks to Watch Out For
* **c2rust Duplication**: Functions that were `static inline` in C headers (specifically `vp8dx_decode_bool` from `dboolhuff.h`) were duplicated by `c2rust` into every Rust module that called them. (Resolved for `vp8dx_decode_bool`).
* **Hardcoded NEON Calls**: The transpiled Rust code explicitly calls `_neon` function names (e.g., `vp8_bilinear_predict16x16_neon` in `decodeframe.rs`), because it was transpiled from C code where NEON macros were active. This causes link errors on x86_64 when building the integration harness. (Forwarded via `simd_shim.rs`).
* **Duplicated Structs (Resolved)**: Struct definitions like `YV12_BUFFER_CONFIG` and `VP8Common` were previously duplicated by `c2rust` across dozens of files. They have now been centralized into `src/vp8/common/types.rs` (Phase 3 complete). Agents can externalize to Phase 4 (Safe API Refactoring).


## Next Steps for Future Agents
1. **Milestone 3: Refactor Loop Filtering Slicing (Disjoint Borrows) - ALL COMPLETE**:
   - [x] Unit 6: Implement a `split_rows_mut` or safe chunking method on `YV12_BUFFER_CONFIG` that yields disjoint mutable slices for individual macroblock rows. (Completed!)
   - [x] Unit 7: Refactor multithreaded loop filtering in `threading.rs` to assign each thread a strictly disjoint mutable slice of the frame, proving to the borrow checker that parallel loop filtering is 100% race-free. (Completed!)
2. **Future Safety Milestones**:
   - [x] **Modernize `BOOL_DECODER` (`src/vp8/decoder/dboolhuff.rs`)**: Eliminate residual raw pointer arithmetic (`user_buffer` additions) inside `SafeBoolDecoder` and fully leverage slice boundaries. (Completed!)
   - [x] **Address remaining `unsafe` blocks in `src/vp8/common/vp8_loopfilter.rs` and `extend.rs`** by replacing FFI boundary styles with native safe Rust patterns where possible (excluding assembly RTCD paths). (Completed! Unused FFI wrappers were deleted, and internal functions were cleaned of `#[unsafe(no_mangle)]` and `extern "C"`).
   - [x] **Remove obsolete `#[unsafe(no_mangle)]` from `vp8_default_mv_context` in `src/vp8/common/entropymv.rs`**: This static table is only used internally in `decodeframe.rs` and can have its `#[unsafe(no_mangle)]` attribute removed safely to reduce unsafe keyword count. (Completed!)
   - [x] **Audit `blockd.rs` for dead code**: `vp8_block2left` and `vp8_block2above` in `src/vp8/common/blockd.rs` appear to be encoder-only and completely unused in CrabVPX. They can likely be removed entirely to clean up the codebase and remove 2 more unsafe keywords. (Completed!)
   - **Audit other FFI wrappers in `src/vp8/decoder/dboolhuff.rs`** (`vp8dx_start_decode` and `vp8dx_bool_decoder_fill`) to see if they can also be deprecated/removed or if they are required for external ABI linkage.
    - **Audit other dead tables in `src/vp8/common/entropy.rs`**: `vp8_coef_encodings` also appears to be completely unused in the decoder and can be removed to clean up the codebase. (Already successfully removed `vp8_extra_bits`, `vp8_extra_bit_struct`, and `vp8_tree_p` in this turn!).

