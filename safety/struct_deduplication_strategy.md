# Bottom-Up Struct Deduplication Strategy

## The Core Challenge: Transitive Struct Dependencies
During the conversion of CrabVPX from C to Rust, `c2rust` duplicated core struct definitions (such as `MACROBLOCKD`, `YV12_BUFFER_CONFIG`, and `MODE_INFO`) across dozens of files. 

While it is tempting to deduplicate a central struct like `MACROBLOCKD` to enable passing safe Rust references (`&mut MACROBLOCKD`) across module boundaries, doing so in isolation will fail. `MACROBLOCKD` contains fields that rely on other duplicated structs:
- `pre` and `dst` rely on `YV12_BUFFER_CONFIG` (duplicated in 22 files).
- `mode_info_context` relies on `MODE_INFO` (duplicated in 16 files).
- `block` relies on `BLOCKD` (duplicated in 13 files).

If `MACROBLOCKD` is deduplicated into a central location but calling modules retain their local copies of `YV12_BUFFER_CONFIG`, the Rust compiler will reject function calls and struct assignments with `mismatched types` errors.

## The Four-Phase Execution Roadmap
To fix this without breaking compilation, agents must execute a **bottom-up deduplication strategy**, starting with leaf structs that have no external dependencies and working up to top-level decoder structs.

### Phase 1: Centralize Leaf Structs
Create a central module (e.g., `src/vp8/common/structs/` or `src/vp8/common/types.rs`). Move and deduplicate the following "leaf" structs and unions across all files:
- `MV` and `int_mv`
- `vp8_prob`
- `ENTROPY_CONTEXT` and `ENTROPY_CONTEXT_PLANES`
- `B_PREDICTION_MODE` and `MB_PREDICTION_MODE`
- `MB_MODE_INFO`

**Agent Instruction**: For each leaf struct, place it in the central module, remove its duplicated definition from all dozen+ files, and update imports. Verify compilation after each struct.

### Phase 2: Centralize Mid-Tier Structs
Once leaf structs are centralized, deduplicate the mid-tier structs that depend on them:
- `b_mode_info` (union relying on `B_PREDICTION_MODE` and `int_mv`)
- `BLOCKD` (relying on `b_mode_info`)
- `MODE_INFO` / `modeinfo` (relying on `MB_MODE_INFO` and `b_mode_info`)
- `YV12_BUFFER_CONFIG` (relying on basic primitives)

### Phase 3: Centralize Top-Tier Structs
With all underlying dependencies centralized, deduplicate the primary decoder state structs:
- `MACROBLOCKD` (relying on `BLOCKD`, `MODE_INFO`, `YV12_BUFFER_CONFIG`, etc.)
- `VP8_COMMON`
- `VP8D_COMP` (relying on `MACROBLOCKD` and `VP8_COMMON`)

### Phase 4: Safe API Refactoring
Once all structs are globally unique types across the crate, inter-module function signatures can finally be refactored to use safe Rust references.
- Convert `*mut MACROBLOCKD` to `&mut MACROBLOCKD` in inter-module APIs (e.g., `vp8_reset_mb_tokens_context`, `vp8_decode_mb_tokens`, `decode_macroblock`).
- Remove `extern "C"` linkage from internal Rust-to-Rust calls.

## Critical Rules for Agents
1. **Do Not Skip Phases**: Attempting Phase 3 before Phase 2 will result in intractable compiler errors.
2. **Verify at Every Step**: Run `./build.sh` after deduplicating *each individual struct*. Do not attempt to deduplicate multiple structs in a single commit.
