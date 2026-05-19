# Bitstream FFI Thrashing Mitigation Strategy

## The Core Challenge: Temporary Slice Thrashing & Per-Bit Pointer Synchronizations
In the initial transition toward safe bitstream reading, CrabVPX introduced `SafeBoolDecoder` as a safe abstraction over the legacy C-style `BOOL_DECODER` (`vp8_reader`). However, because downstream FFI callers and token decoders still operate on the raw C ABI pointer struct, the decoder experiences severe **temporary slice thrashing** across hot-path execution loops.

For every single bit or token decoded (millions of operations per video frame), the current implementation in `vp8dx_decode_bool` performs the following extremely inefficient sequence:
1. Reconstructs a temporary Rust slice by computing pointer differences (`bc.user_buffer_end.offset_from(bc.user_buffer)`).
2. Instantiates a transient `SafeBoolDecoder` on the stack.
3. Decodes exactly one boolean value.
4. Updates the raw C pointers by committing unsafe pointer addition (`bc.user_buffer.add(self.offset)`).

```
Legacy Hot-Path Flow (Thrashing per Bit):
[Token Decoder Loop] ──> [vp8dx_decode_bool] ──> (Rebuild Slice) ──> [SafeBoolDecoder] ──> (Write Pointers)
      │                                                                                           │
      └────────────────────────────< Loops millions of times <────────────────────────────────────┘
```

To eliminate this overhead and establish idiomatic zero-overhead safety, agents must push slice creation to the outermost execution boundary and maintain persistent borrowing across inner decoding loops.

---

## The Persistent Execution Roadmap (Agent Action Plan)

### Phase 1: Enable Persistent Context Passing in Native Token Decoders
Decoders operating within macroblock or token extraction loops must be refactored to take a persistent `&mut SafeBoolDecoder` rather than passing the legacy raw `*mut BOOL_DECODER` pointer.

**Legacy Function Prototype:**
```rust
// Requires bridging per token decode
pub unsafe fn decode_mb_tokens(xd: &mut MACROBLOCKD, bc: *mut BOOL_DECODER) {
    // ... calls vp8dx_decode_bool(bc, prob)
}
```

**Refactored Persistent Prototype:**
```rust
// Holds persistent slice across the entire macroblock decoding lifecycle
pub fn decode_mb_tokens_safe(xd: &mut MACROBLOCKD, bc: &mut SafeBoolDecoder) {
    // ... calls bc.read_bool(prob) directly with zero FFI overhead
}
```

**Agent Instruction:** Target a leaf token parsing function (e.g., inside `dboolhuff.rs` or `detokenize.rs`). Provide a safe overload (`_safe`) accepting `&mut SafeBoolDecoder`. Update its internal token reads to call `bc.read_bool()` directly.

---

### Phase 2: Boundary Inversion (Instantiate at the Partition Root)
Instead of converting raw pointers to slices inside leaf operations, convert the `BOOL_DECODER` instance exactly once at the beginning of partition or macroblock row decoding (`decode_mb_rows`).

```
Optimized Architectural Flow (Zero Overhead):
[Partition / Row Root] ──> (Instantiate Slice Once) ──> [SafeBoolDecoder]
                                                              │
   ┌──────────────────────────────────────────────────────────┴──────────────┐
   ▼                                                                         ▼
[Token Decoder Loop] ──> [bc.read_bool()]                         [Sub-Block Decoding]
```

**Agent Instruction:** Inside the partition decoding entry point (e.g. `decode_mb_rows` or threading shims), initialize `SafeBoolDecoder::from_bool_decoder(bc)` exactly once before initiating macroblock column loops. Pass this mutable reference down the execution chain, and call `update_bool_decoder()` only when returning control across FFI boundary barriers.

---

### Phase 3: Purge FFI Over-Head Wrappers
Once all internal call paths natively consume `SafeBoolDecoder`, remove the internal usage of the raw wrapper functions entirely.

**Agent Instruction:** Identify files still calling the legacy `vp8dx_decode_bool` FFI shim. Convert those calls to native safe slice reads. Ensure `extern "C"` declarations of `vp8dx_decode_bool` are preserved only if required by external legacy integration harnesses.

---

## Critical Rules of Engagement for Agents

1. **Keep Cursor Invariant Active:** Ensure that if execution leaves a Rust safe boundary back into an unported C-ABI function, `safe_decoder.update_bool_decoder(raw_bc)` is unconditionally called to sync `user_buffer` state.
2. **Never Splinter Bitstream Contexts:** Do not create multiple overlapping `SafeBoolDecoder` instances over the same underlying buffer space simultaneously.
3. **Continuous Build & Verification:** Any alteration to token or boolean parsing risks catastrophic bitstream corruption. After refactoring any bitstream reading path, immediately verify with `./build.sh` and execute `./scripts/compare.py`.
4. **Log State in HINTS:** Document which token decoder modules (e.g., `detokenize`, `entropymode`) have successfully transitioned to persistent `SafeBoolDecoder` instances in `safety/HINTS.md`.
