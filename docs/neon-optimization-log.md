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
- **Per-MB `views_mut` dedup** (hoist the two per-macroblock plane-view
  constructions in `decodeframe` into one) — **wash, slightly negative.** The
  sampler attributed ~2% to these view helpers, but they were already nearly free
  (cheap slicing the compiler had optimized); the self-time was over-attributed.
  Discarded.
- **Bounds-check elision in the token decoder** (power-of-two index masking +
  `first_chunk_mut::<16>()` on `out`, so `out[j]`/`kBands[n]`/`kZigzag[]`/
  `prob[]` prove in-bounds and drop their checks) — bit-exact (masks are no-ops
  for the real value ranges), but **wash, slightly negative.** This is the key
  result: **LLVM already elides those bounds checks** — the hypothesized "safe-
  Rust bounds-check tax" in the hot loop essentially isn't there, and the added
  mask instructions cost a hair. Discarded. (Corrects an earlier hypothesis;
  see "Interpreting the gap".)

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

1. **Serial token/MV decode** — the #1 single cost (~1.1+ ms/frame), only at
   *parity* with libvpx's C. **Investigated and found already optimal-shape:**
   `read_bool` is inlined into `GetCoeffs` (state in registers; `fill` is rare),
   and the token tree is a faithful port. We *hypothesized* a safe-Rust
   bounds-check tax here and tested it (power-of-two index masking, above) — it
   was a **wash: LLVM already elides those checks.** So there's no removable
   safety tax in the hot loop; even `unsafe get_unchecked` would not help.
2. **Per-block dispatch** in the transform-add path (the real "transform gap"),
   though NEON-ing the idct math didn't move it (wash, above).
3. **Diffuse implementation-maturity overhead** — after 6 washes targeting the
   profile's apparent gaps, no single removable lever survived. The residual is
   cumulative: function-call boundaries, the multi-pass (vs fused-MB) structure,
   and libvpx being marginally tighter everywhere from 15 years of tuning. Not a
   single attributable hotspot.

## Interpreting the gap (do not quote "−24%" bare)

crabvpx decodes ~24% more CPU per frame than libvpx on this benchmark (1080p,
single-thread, decode-only, Apple Silicon NEON, Elephants Dream) — equivalently
~19% lower throughput (1 − 2.6/3.2). That number misleads without its caveats:

- **A young-Rust-port vs mature-C+asm, not "Rust vs C".** libvpx is ~15 years of
  hand-tuned C and assembly; crabvpx is a young safe-Rust port with NEON
  intrinsics on the hot kernels. The gap measures implementation maturity, not
  the language.
- **It is *not* mainly a safety tax — we tested that.** The intuitive culprit is
  Rust's bounds checks, but the elision experiment (above) showed **LLVM already
  removes them** in the hot token loop; forcing elision was a wash. So safe Rust
  is *not* meaningfully paying for bounds checks here, and `unsafe get_unchecked`
  would not help. (panic-free `Result` paths and the safe slicing cost something,
  but no single safety feature proved removable across 6 experiments.) The
  residual is **diffuse implementation maturity**, not a language or safety
  penalty.
- **Worst-isolated case.** Decode-only excludes demux/IO/color-convert/display;
  in a real pipeline decode is a fraction of the work, so end-to-end impact is
  much smaller. Single-thread; both scale with `CRABVPX_THREADS`.
- **A moving target, already moved.** This phase took it 1.95× → 1.24× (≈95% →
  ≈24%). Content/config dependent (the sixtap fast-path win depends on the clip's
  motion-vector distribution).

Fair one-liner: *"a bit-exact, panic-free, safe pure-Rust VP8 decoder runs decode
~24% slower than libvpx's hand-tuned C+asm on Apple Silicon — and that gap is
implementation maturity, not the language or its safety checks (the bounds checks
are compiled away)."*

## Bottom line (2026-06-20)

**~1.95× → ~1.24× this phase, staying bit-exact, panic-free, safe pure-Rust**
(~310 fps 1080p single-thread). The SIMD compute is at the practical floor and
the serial decoder is at parity. We chased the residual through 6 experiments
(incl. the bounds-check tax, which LLVM already elides) and found **no single
removable lever** — it's diffuse implementation maturity, not a safety or
language penalty. Treated as the natural finish line for the NEON phase. Future
ISA work (x86 SSE) should reuse the *arithmetic-scheme* lessons above, not chase
lane width.

## Appendix: lessons from libvpx & where Rust can help (for the SSE phase)

**Mining libvpx's "hand assembly" — feasibility by ISA.** For aarch64 there is
**no raw ARM assembly** in libvpx — the NEON path is C intrinsics
(`vp8/common/arm/neon/*.c`), which we read directly; the high-value lessons are
already extracted and shipped (u8-MAC, s8-saturating, `vqdmulhq` rotation
constants, fused `vqrshrun`/`vqshrn` round+clamp, two-group accumulation,
identity-pass skipping, in-place residual). What remains unported (2-block
register blocking `idct_dequant_full_2x`, `vld4` deinterleaving transposes,
manual scheduling) the compiler already matches — hence the washes. **The 68
`.asm` files are all x86 (SSE/SSSE3/MMX)** and are a genuinely untapped goldmine
**for the SSE phase only** (e.g. `vpx_subpixel_8t_ssse3.asm`, `idct_blk_sse2`).
A feasible confirmation step if ever needed: `otool -tv`/`objdump` a single crab
kernel and diff instruction count/scheduling vs libvpx's compiled kernel.

**"Things Rust is better at" — what applies.**
- *`&mut` = `noalias` for free* (C needs `restrict`, often omitted). Already
  helping (it's why `read_bool` inlines with state in registers). Opportunistic
  elsewhere.
- *Whole-crate inlining without LTO.* Already gives libvpx's hand-fused-function
  effect for free.
- *Bounds-check elision via types.* Tested (above) — **LLVM already does it**, so
  no win here, but it's why the safe code isn't paying a check tax. Keep using
  fixed-size array refs / iterators so it stays that way.
- *Monomorphization / the `Simd` trait.* Was a perf wash (const-generic sixtap)
  but is the real **parity multiplier for SSE**: one bit-exact kernel body, two
  ISAs. The biggest Rust leverage going forward is *productivity/correctness at
  parity*, not raw single-thread speed.
- *`const fn` to compute tables → store in a `static`.* Lookup tables must be
  `static` (one rodata copy, addressed); reserve `const` for register-sized
  values that lower to immediates. (crab already uses `static` for `kBands`/
  `kZigzag`/`vp8_norm`/`coef_probs`.)

## Content caveat

All numbers are from Elephants Dream (an open Blender movie), upscaled to 720p/
1080p — not the same as Big Buck Bunny. The win magnitudes (especially the sixtap
fast-paths, which depend on the integer-pel MV distribution) are content-
dependent; re-baseline on the target clip before trusting an exact %.
