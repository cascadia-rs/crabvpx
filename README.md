# CrabVPX

Lifting and shifting `libvpx` (specifically VP8 decoding) to Rust using `c2rust`.
We successfully transpiled the C code, including ARM NEON intrinsics, yielding a **pure-Rust hardware-accelerated decoder** (on Apple Silicon). The project has been upgraded to the **Rust 2024 edition**.

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
