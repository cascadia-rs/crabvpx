# CrabVPX

Lifting and shifting `libvpx` (specifically VP8 decoding) to Rust using `c2rust`, while retaining performance-critical Assembly optimizations via FFI.

## Project Status Checklist

### Phase 1: Preparation & Configuration
- [x] Clone upstream `libvpx` repository.
- [x] Configure `libvpx` build strictly for VP8 decoding (e.g., `--disable-vp9`, `--disable-vp8-encoder`).
- [x] Generate `compile_commands.json` using `bear`.

### Phase 2: Transpilation (The Lift)
- [x] Run `c2rust transpile` on the compilation database.
- [x] Initialize `crabvpx` Cargo project.
- [x] Integrate generated `.rs` files into the Cargo structure.

### Phase 3: Assembly FFI Integration (The Shift)
- [x] Identify necessary Assembly (`.asm`/`.S`) files.
- [x] Setup `build.rs` to compile and link ASM files using the `cc` crate (and `yasm`/`nasm`).
- [x] Verify `extern "C"` linkages in the Rust code.

### Phase 4: Stabilization & Refactoring
- [ ] Fix compilation errors in the transpiled Rust code.
- [ ] Port/integrate tests to verify decoding correctness.
- [ ] Incrementally refactor to Safe Rust APIs.
