# NEON optimization log (aarch64 decode)

A durable record of performance-phase experiments on the VP8 decoder — **what we
tried, what landed, what didn't, and why.** The point of the "didn't" entries is
so we don't re-tread dead ends. See [simd-architecture.md](simd-architecture.md)
for the kernel/trait design.

## Goal & baseline

Close the gap to libvpx 2.29 on Apple Silicon (NEON), single-threaded, decode-only.

- **Start of phase:** 1080p ~4.66 ms/frame, **~1.95× libvpx** (~2.6 ms/frame).
- **Now:** 1080p ~3.2 ms/frame, **~1.24×**.

## Methodology (read before benchmarking)

- **`HARNESS_NO_MD5=1` is mandatory** for decode-only timing. Without it the
  harness's per-frame MD5 dominates and dilutes the A/B ratio toward parity.
  (This flag was a no-op until PR #44 implemented it.)
- **Bit-exactness gate:** the 62-vector `differential_md5` suite, ST *and* MT
  (`cargo test --test differential_md5`; `CRABVPX_THREADS=4 …`). This is the
  source of truth, not the per-kernel unit tests.
- **Measure with alternating before/after binaries, ≥3×30-run trials.** Single
  runs are thermally noisy on this machine (±5%); trust the trend across trials,
  prefer comparing within one alternating run.
- **The libvpx source is checked out** under `libvpx/vp8/common/arm/neon/`. Read
  the actual kernel before guessing at its technique — twice the documented
  intuition turned out wrong (see lessons).

## The big lesson

**On Apple Silicon the lever is almost always the *arithmetic scheme*, not lane
width or loop unrolling.** Every width/unroll experiment was a wash; every win
came from changing how the math is done (narrower lanes via saturation, fused
rounding/clamp instructions). The Rust scalar paths also auto-vectorize well, so
a NEON port only wins when it does *fundamentally less work* than the compiler's
autovectorization, not merely "the same work, by hand."

## What landed

| PR | Kernel | Technique | 1080p win |
| --- | --- | --- | --- |
| #44 | harness | implement `HARNESS_NO_MD5` (decode-only timing) | — (tooling) |
| #45 | sixtap | single-pass fast paths for integer-pel offsets (skip the identity filter pass) | ~6% |
| #46 | sixtap | fuse the two-pass filter, drop the i16 `mid` scratch buffer (sliding register window) | ~2% |
| #47 | dc-only idct | NEON `vp8_dc_only_idct_add` (add DC term + saturating-narrow clamp) | ~3.5% |
| #48 | transform-add | read predictor in place from `dst`; drop per-4×4-block copy temps | ~6.5% |
| #49 | sixtap | **u8×u8→u16 MAC** (`vmull/vmlal/vmlsl_u8`): 6 mults/8-out vs 12 i32; fixed sign pattern, two-group split + `vqaddq_s16` + `vqrshrun_n_s16` | **~17%** |
| #50 | loop filter (horizontal Y) | **s8 saturating arithmetic** (`vqadd/vqsub_s8`): s8 saturation IS the `[-128,127]` clamp, free | ~3% |

Cumulative: ~1.95× → ~1.24×.

## What we tried that did NOT work (do not re-attempt without new insight)

- **Standalone NEON 4×4 IDCT (`vp8_short_idct4x4llm`)** — bit-exact (i32-lane
  transpose impl), but **perf-neutral**: the full 4×4 transform is ~0.5% of
  self-time and the scalar already auto-vectorizes. Discarded. (The *DC-only*
  path #47 and the in-place fusion #48 were the real transform wins.)
- **Const-generic size-specialization of sixtap** (monomorphize 16×16/8×8/8×4/
  4×4) — **~0%.** The compiler already unrolls the macro-tap loops well; width/
  unrolling wasn't the lever. The u8-MAC arithmetic (#49) was.
- **16-lane *i16* loop filter** (split `transpose8x8` into a `Transpose8x8`
  trait, added a `Neon16`/`int16x8x2` backend, routed Y horizontal) — **~0%,
  discarded.** i16 math at 16 lanes == two 8-lane passes; pure width does
  nothing. This directly motivated the s8 rewrite (#50), which *did* win.
- **Vertical-edge s8 loop filter** (16-row transpose → s8 core → transpose back)
  — **measured wash, deferred.** The 16-row transpose (two 8×8 transposes +
  `vcombine`/`vget` round-trips through `uint8x8`) costs more than the s8 filter
  saves. Would need a `vld4`-style deinterleaving load (how libvpx loads vertical
  edges) to win. Only the *horizontal* Y loop filter shipped in #50.
- **Fused NEON dequant+idct+add for the eob>1 path** (`vp8_dequant_idct_add`,
  faithful i16-saturating port of libvpx incl. unaligned-u32 predictor loads) —
  bit-exact (differential green), but **~0% / slightly negative.** The eob>1 idct
  isn't the bottleneck (most blocks are dc-only, already NEON; scalar
  auto-vectorizes the rest). The profile's "transform-add 2.6×" gap is **per-block
  dispatch + wrapper/UV overhead**, not idct math — closing it would need a
  full-MB, 2-blocks-at-once restructure (`idct_dequant_full_2x_neon`), payoff
  uncertain given this result. Discarded.

## Notes on bit-exactness for the saturating kernels

libvpx's NEON sixtap/idct/loop-filter use **saturating** i16/s8 arithmetic, which
can differ from a non-saturating i32 reference on synthetic out-of-range inputs.
This is fine: those inputs don't occur for valid dequantized coefficients, and
the 62-vector differential (the gate) stays green because crab's NEON == libvpx's
NEON == libvpx's C on real streams. Per-kernel unit tests therefore exercise the
*valid decode domain* (e.g. the dequant_idct test bounds coefficients so no
butterfly intermediate saturates); full-range fuzzing of those kernels is
*expected* to diverge from the scalar twin and is not a correctness signal.

## Current gap composition (~1.24×, ~0.6 ms/frame to libvpx)

The SIMD-able compute is at/near libvpx parity. The residual gap is diffuse and
mostly **not** more-kernel-SIMD:

1. **Serial token/MV decode** — the #1 single cost (~1.1+ ms/frame) and only at
   *parity* with libvpx's C. The main remaining headroom to actually *beat*
   libvpx: micro-opt the Rust bool decoder (renorm, branch layout, fewer bounds
   checks). Not SIMD.
2. **Per-block dispatch** in the transform-add path (the real "transform gap").
3. **crab-only buffer "views"/bounds overhead** (~2% of decode): per-MB
   re-derivation of frame-buffer slice views (`views_mut`, `views_with_borders`,
   `get_ref_and_dst_fb`). Hard to hoist cleanly — the *reference* frame varies
   per MB, and holding views across the loop conflicts with per-MB buffer access.

## Content caveat

All numbers are from Elephants Dream (an open Blender movie), upscaled to 720p/
1080p — not the same as Big Buck Bunny. The win magnitudes (especially the sixtap
fast-paths, which depend on the integer-pel MV distribution) are content-
dependent; re-baseline on the target clip before trusting an exact %.
