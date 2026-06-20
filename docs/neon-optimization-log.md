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
| #52 | transform-add | **batched 2-block** dequant+IDCT+add (`idct_dequant_full_2x`): two adjacent blocks in the full int16x8 width. The per-block shape was a wash — the batching is the win | ~2.7% |
| #54 | loop filter (vertical Y) | s8 16-wide via `vtrnq` load-transpose + `vst4_lane` store (normal) / transpose-back (MB). The earlier vertical-s8 wash used a slower transpose-back | ~2.6% |
| #55 | loop filter (chroma) | U+V packed into one 16-wide s8 pass (U low lanes, V high), reusing the Y s8 cores + transpose | ~2% |

Cumulative: ~1.95× → ~1.14×.

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
- **Sixtap `vext` load reduction** (one 16-byte `vld1q_u8` per row + `vext` for
  the 6 taps, vs 6 overlapping 8-byte loads) — bit-exact, but **wash.** The
  horizontal loads are L1 hits and weren't the bottleneck (same lesson as the
  const-generic sixtap). Discarded.
- **Sixtap 4-rows-at-once vertical** (libvpx's 16x16 shape: a 9-row window
  feeding four independent vertical MACs per iteration, for ILP + load reuse)
  — bit-exact, but **wash / slightly negative at 1080p.** The compiler already
  schedules the 1-row sliding-window loop well; the wider window added register
  pressure. Discarded.

**Sixtap conclusion: at its practical floor.** Three structural restructures
(const-generic, `vext`, 4-rows-at-once) all washed on top of the #49 u8-MAC
kernel. The residual ~1.3× vs libvpx is not reachable by these reshapings — it's
likely instruction-scheduling that Apple Silicon's pipeline rewards in libvpx's
exact hand-written sequence, i.e. it would need assembly-level matching, not
intrinsic restructuring. Stop here unless a fundamentally different idea appears.

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

**Measured** per-component (sample-share × each decoder's ms/frame; crab 3.2,
libvpx 2.6, totals ~equal). The gap is **not diffuse — it's concentrated in three
SIMD kernels still behind libvpx's deeper hand-tuning, plus control overhead:**

| component | crab | libvpx | gap |
| --- | --- | --- | --- |
| Transform-add (idct/dequant block) | ~0.26 | ~0.10 | **+0.15 (2.5×)** |
| Sixtap | ~0.73 | ~0.55 | **+0.17 (1.3×)** |
| Loop filter | ~0.71 | ~0.59 | **+0.12 (1.2×)** |
| decode_frame / MB control | ~0.11 | ~0.06 | +0.06 (2×) |
| Inter-pred + buffer views | ~0.12 | ~0.07 | +0.05 |
| Token decode | ~0.80 | ~0.76 | +0.04 (≈parity) |
| MV/mode decode | ~0.48 | ~0.48 | parity |

**Correction to an earlier draft of this doc:** the residual is *not* "diffuse
maturity with no removable lever." The six washes were failures to find the
*easy* wins, not evidence of parity. libvpx's transform/sixtap/loop-filter
kernels are still measurably faster from hand-tuning we labeled
diminishing-returns and walked away from — those are concrete, identifiable
targets (roadmap below). Token/MV decode genuinely are at parity (the
bounds-check tax is elided by LLVM, tested above).

## Path to parity (rav1d territory, ~5–10%)

Not a polish — real runway, but with measured ceilings. Ordered by ratio:

1. ~~Transform-add: batched 2-block NEON~~ **DONE (#52, ~2.7%).** Confirmed the
   thesis: the per-block shape was a wash, the batched 2-block version won. Closed
   ~0.09 of the ~0.15 transform gap; the rest is per-pair dispatch / chroma.
2. ~~Loop filter: finish it~~ **DONE (#54 vertical Y ~2.6%, #55 chroma U+V ~2%).**
   `vtrnq` load-transpose + `vst4_lane` store (the earlier vertical wash used a
   slower transpose-back); chroma packs U+V into one 16-wide pass.
3. ~~Sixtap restructure~~ **EXHAUSTED (3 washes: const-generic, `vext`,
   4-rows-at-once).** At its practical floor on the u8-MAC kernel; residual ~1.3×
   likely needs assembly-level scheduling, not intrinsic reshaping. See the
   sixtap conclusion above.
4. **De-c2rust the MB control loop** — `decode_frame`/per-MB dispatch is ~2×
   libvpx; idiomatic rewrite so it optimizes like the C. ~0.05–0.1 + diffuse.
   The last untried lever; uncertain whether the 2× is c2rust verbosity or
   inherent dispatch.

### Why parity is plausible here (vs the rav1d comparison)

rav1d (c2rust port of dav1d) lands ~5% slower — but it **links dav1d's hand-
written assembly**; its 5% is the cost of Rust *glue* with identical kernels.
crabvpx writes kernels in Rust, so our comparison includes a kernel gap rav1d's
doesn't. On **aarch64 that's winnable**, because libvpx has *no ARM assembly* —
its NEON is intrinsics we can match in Rust (the arithmetic-scheme wins prove
it). Our kernels trail only because the deeper intrinsic tuning isn't finished,
not a language ceiling. Our control overhead (~2× on decode_frame) is the
analogue of rav1d's glue tax, and ours is worse — the c2rust-derived code is less
idiomatic than hand-translated Rust, so it's addressable.

**x86 caveat:** libvpx *does* ship hand asm for SSE/SSSE3 (68 `.asm` files). Pure-
Rust intrinsics may genuinely trail it there; the rav1d move (link the reference
asm for the hottest kernels) may be worth considering if pure-Rust can't reach
parity on x86.

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
  residual is **unfinished kernel hand-tuning + c2rust control overhead** (see
  the measured breakdown above) — addressable work, not a language/safety
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

**~1.95× → ~1.14× and counting, staying bit-exact, panic-free, safe pure-Rust**
(~330 fps 1080p single-thread). The path-to-parity thesis held up: #52 (batched
2-block transform), #54 (vertical Y LF), and #55 (chroma U+V LF) all landed as
real wins where the *easy* shape had washed — the gap was unfinished kernel
tuning, not a wall. **Sixtap is now the exception: ~1.3× behind and exhausted**
(3 restructures washed — it's at the floor for intrinsic-level work, likely
needs assembly to match libvpx's scheduling). Other remaining: token/MV decode
(at parity, serial) and the ~2× `decode_frame` control overhead (the last untried
lever, de-c2rust). So aarch64 is close to as far as pure-Rust intrinsics
practically go (~1.14×); reaching rav1d's ~5% would likely require either the
de-c2rust control work paying off or assembly for sixtap. x86 is a separate,
harder story (libvpx has hand asm). Reuse the *arithmetic-scheme* lessons; don't
chase lane width.

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
