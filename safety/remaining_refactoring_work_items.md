# Final Refactoring & Stabilization Work Items

With top-level struct decoupling and bitstream thrashing mitigation completely resolved across CrabVPX, this document establishes the definitive backlog for autonomous agents addressing the remaining 558 `unsafe` blocks.

---

## 🚫 OUT OF SCOPE: Public C-ABI Dispatchers & FFI Interfaces
To preserve exact drop-in ABI linkage for external unported C consumers, public FFI wrapper modules and external C dispatcher contexts are **explicitly out of scope** for safe Rust conversion.

**Agents are strictly forbidden from attempting to remove `unsafe` blocks or raw pointer arguments in the following files:**
* `src/vp8/vp8_dx_iface.rs`
* `src/vpx/src/vpx_encoder.rs`
* `src/vpx/src/vpx_codec.rs`
* `src/vpx/src/vpx_decoder.rs`
* `src/vpx/src/vpx_image.rs`
* `src/api.rs`

*Rationale:* These modules act as external entry points that must validate raw C pointers (`*mut vpx_codec_ctx`) and conform to external C calling conventions. Attempting to make them safe breaks downstream dynamic linkage.

---

## 📋 Active Backlog: Task Units for Agents

### Milestone 1: Global Mutable State Deprecation (25 Instances)
In Rust 2024, accessing `static mut` variables requires `unsafe` blocks. Because almost all global states transpiled by `c2rust` are precalculated constant lookup tables or probability distributions, they must be modernized.

#### Work Units:
* **[x] Unit 1 (`entropymode.rs`):** Convert the 12 `static mut` probability tables to immutable `static` definitions. Remove `unsafe` blocks at read sites.
* **[x] Unit 2 (`entropy.rs`):** Convert the 7 `static mut` entropy distribution arrays to immutable `static` definitions.
* **[x] Unit 3 (`blockd.rs`):** Remove the 2 legacy mutable statics in block structures by converting to `const` or thread-local storage.
* **[x] Unit 4 (`vpx_config.rs` & `threading.rs`):** Audit and eliminate residual mutable static flags used for global build/thread configuration.

---

### Milestone 2: Filter Pointer Indexing & Sub-Slicing (`filter.rs`)
Sub-pixel prediction algorithms currently compute tap offsets by executing unsafe pointer arithmetic across unaligned image planes.

#### Work Units:
* **[ ] Unit 5 (`filter.rs`):** Replace raw pointer addition (`predictor.add(offset)`) with standard bounds-checked sub-slicing (`&slice[offset..]`).
* **[ ] Unit 6 (`filter.rs`):** Convert unaligned multi-tap reads into safe windowing methods or explicit fixed-size chunk iterators.
* **[ ] Unit 7 (`loopfilter_filters.rs`):** Purge residual raw pointers in edge filtering subroutines, fully relying on disjoint safe reborrowing.

---

### Milestone 3: Unsafe Intrinsic Shim Wrapping (`simd_shim.rs`)
Hardware assembly intrinsics must be fully isolated behind safe boundary contracts.

#### Work Units:
* **[ ] Unit 8 (`simd_shim.rs`):** Refactor public wrappers around NEON shims to take safe Rust slices. Enforce exact length pre-conditions (`assert!(slice.len() >= 16)`) prior to internal unsafe vector executions.
* **[ ] Unit 9 (`simd_shim.rs`):** Eliminate dead or duplicate C-fallback shims that have already been fully superseded by native Rust safe replacements.

---

## 🛡️ Critical Rules of Engagement for Agents

1. **Adhere Strictly to Exclusions:** Do not waste execution turns trying to refactor public C dispatchers listed in the Out of Scope section.
2. **Atomic Work Units:** Complete exactly one numbered work unit per turn. Do not attempt to refactor multiple modules in a single commit.
3. **Validate at Every Turn:** Every modification to memory models or lookup tables risks silent video output degradation. Execute `./build.sh` and `./scripts/compare.py` unconditionally prior to completing a work unit.
4. **Track Hand-Off Progress:** Update your progress on this backlog inside `safety/HINTS.md` so subsequent agents know exactly which work unit to pick up next.
