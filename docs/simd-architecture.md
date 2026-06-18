# SIMD architecture: multi-ISA kernels with minimal duplication

Status: **proposed** · Supersedes the ad-hoc `vp8/common/neon.rs` landed in #31.

## Context

#31 added hand-written NEON kernels (sixtap sub-pixel filter, normal + MB-edge
loop filter) that took aarch64 decode from ~1.5–3× slower than libvpx to near
parity. We now want the same wins on the other instruction sets Chrome ships,
without writing every kernel three or four times.

Constraints:
- **Stable Rust** (no `std::simd`/`portable_simd`).
- **Minimal duplication** — the kernel *logic* must be written once.
- **Correctness** — every ISA path stays bit-exact with the scalar reference
  (gated by per-kernel unit tests + the 62-vector `differential_md5` suite).
- **`unsafe` stays confined** to the SIMD backends and tracked separately from
  the scalar zero-unsafe ratchet (cascadia ADR-014's up-only `[simd_ceilings]`).

## Decision

### 1. Target ISAs (all 128-bit)

| arch | ISA | detection |
| --- | --- | --- |
| aarch64 | NEON | compile-time (ARMv8-A baseline) |
| x86-64 | SSE2 | compile-time (x86-64 ABI baseline) |
| x86-64 | SSSE3 / SSE4.1 | runtime (`is_x86_feature_detected!`) |

**AVX2 is out of scope for VP8.** The blocks are tiny (4×4…16×16) and the loop
filter works on 8–16 lanes; 256-bit buys almost nothing here, which is why
libvpx's VP8 is essentially all SSE2/SSSE3. (Revisit for VP9/AV1.) Sticking to
128-bit keeps lane counts uniform and avoids width-generic kernels.

### 2. Share the logic, abstract the primitives

Kernel logic (the sixtap two-pass, the loop-filter mask→filter→select chain, the
8×8 transpose) is ISA-independent; only ~20 primitive ops differ. So:

```
src/vp8/common/simd/
  mod.rs       // `Simd` trait + the `Dsp` fn-pointer table + detect/dispatch
  kernels.rs   // kernels written ONCE, generic over `S: Simd`
  neon.rs      // impl Simd for Neon        (cfg aarch64)  ← #31's neon.rs moves here
  sse.rs       // impl Simd for Sse41 / Sse2 (cfg x86_64)
  scalar.rs    // impl Simd for Scalar (portable reference / fallback, no unsafe)
```

- **`Simd` trait**: associated vector types (`U8`, `I16`) + `#[inline(always)]`
  primitives — `load`/`store`, `abd_u8`, `cmp_gt`, `and`/`or`/`not`/`select`,
  `widen`/`narrow` i16, `min`/`max`/`shl`/`shr`/`mul` i16, and `transpose8x8`.
  The genuinely ISA-specific bits (transpose, narrowing) are one method each.
- **`kernels.rs`**: `sixtap::<S>()`, `loop_filter_edge::<S>()`, etc., written
  once. Monomorphization gives each ISA its own optimized code from one source.
- #31's NEON intrinsics become `impl Simd for Neon` — no work thrown away.

This is why a pure portable-SIMD crate was rejected: `wide`/`fearless_simd`
cover the lane math (even `abd` as `max-min`), but **cannot express the 8×8
transpose** the vertical loop filter needs — that's an ISA shuffle (`vtrn` /
`pshufb` / `punpck`). A trait method is the clean home for it.

### 3. Dispatch

A detect-once fn-pointer table (cascadia's `OnceLock` pattern), differing by
arch only at the table-selection step:

```rust
fn detect() -> &'static Dsp {
    #[cfg(target_arch = "aarch64")] { return &NEON_DSP; } // baseline, no detection
    #[cfg(target_arch = "x86_64")] {
        if is_x86_feature_detected!("sse4.1") { return &SSE41_DSP; }
        return &SSE2_DSP;                                  // x86-64 baseline
    }
    #[allow(unreachable_code)] &SCALAR_DSP
}
```

NEON kernels (baseline) need no `#[target_feature]`. The runtime-detected x86
entries do: a thin `#[target_feature(enable = "sse4.1")]` wrapper calls the
generic kernel so its `#[inline(always)]` primitives emit those instructions.
That per-(kernel × ISA) wrapper is the only repeated scaffolding, and a small
macro generates it:

```rust
simd_entry!(sixtap16, Sse41, "sse4.1"); // -> one #[target_feature] fn
```

So per new ISA you write: the ~20 primitive impls + one macro line per kernel.
Kernel bodies are untouched.

### 4. Correctness & safety

- **Scalar twin is the reference.** `impl Simd for Scalar` (or the existing
  scalar functions) is always correct and is the bit-exact target for every
  vector backend — the same gating #31 established, now run under each `Dsp`.
- **`unsafe` confined** to the backend `impl`s (loads/stores + the
  `#[target_feature]` boundary; arithmetic is safe inside `#[target_feature]`
  on Rust ≥ 1.87). Tracked in an up-only SIMD budget per ADR-014, never
  touching the scalar ratchet.
- CI gains an x86 job (e.g. `ubuntu` already x86) running the differential
  suite under the detected `Dsp`; a `CRABVPX_FORCE_SCALAR=1` (or per-`Dsp`
  test) keeps the scalar path covered too.

## Sequencing

1. **(#31, done)** NEON kernels, ad-hoc.
2. **Refactor** `neon.rs` → the `Simd` trait + `kernels.rs`, NEON as the first
   impl. Pure refactor, differential-gated, no behavior change.
3. **Add SSE** (`impl Simd for Sse41`, SSE2 fallback) — primitives only; kernels
   come for free. Gate against the scalar twin on x86 CI.
4. Extend to remaining kernels (simple loop filter, IDCT) under the same trait.

## Alternatives considered

- **Hand-write each kernel per ISA** (libvpx-style): 3–4× duplication. Rejected.
- **`std::simd`**: write-once, zero unsafe — but nightly-only. Blocked on stable.
- **`wide` crate**: write-once, stable, safe — but no transpose/shuffle for the
  vertical edge. Usable for the lane math; rejected as the *sole* mechanism.
- **AVX2**: marginal for VP8's small blocks. Deferred to VP9/AV1.
