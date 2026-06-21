# NEON performance phase — summary

**Outcome: crabvpx VP8 decode went from ~1.95× to ~1.14× libvpx** on Apple
Silicon (1080p, single-thread, decode-only) — bit-exact, panic-free, still safe
pure-Rust. ~330 fps 1080p single-threaded.

Full experiment log (every attempt, win and dead-end): [neon-optimization-log.md](neon-optimization-log.md).

## Results

| | start | end |
| --- | --- | --- |
| 1080p decode-only | 4.66 ms/frame (1.95×) | ~3.0 ms/frame (~1.14×) |
| 720p decode-only | (1.95× class) | ~1.38 ms/frame |

Measured with the A/B harness, decode-only (`HARNESS_NO_MD5=1`), real content
(Elephants Dream). Every kernel stayed bit-exact against the scalar reference:
62-vector `differential_md5` green single- and multi-threaded.

## What shipped

| Area | PRs | Technique |
| --- | --- | --- |
| Harness | #44 | implement `HARNESS_NO_MD5` so decode-only timing is real (it was a no-op; MD5 was hiding the gap) |
| Sub-pixel (sixtap) | #45, #46, #49 | integer-pel fast paths, two-pass fusion, then the big one: `u8×u8→u16` MAC (#49, ~17%) |
| Transform-add | #47, #48, #52 | NEON dc-only idct, in-place predictor (drop copies), batched 2-block dequant+idct+add |
| Loop filter | #50, #54, #55 | 8-bit saturating arithmetic; vertical via transpose+`vst4`; chroma U+V packed 16-wide |

## The one lesson worth remembering

**On Apple Silicon the lever is the *arithmetic scheme*, not lane width or loop
unrolling.** Every "make it wider / unroll it" experiment washed; every win came
from doing the math differently — narrower lanes via saturation (s8/u8 instead of
i16/i32), fused round+shift+clamp instructions (`vqrshrun`/`vqshrn`), and
batching that does *less work*, not the same work by hand. The Rust scalar paths
auto-vectorize well, so a hand NEON kernel only wins when it's fundamentally
cheaper than what the compiler already produces.

Corollary, learned the hard way (10+ washed experiments): **read libvpx's actual
kernel before porting, and measure every change** — intuition about "obvious"
speedups (lane width, bounds-check elision, load reduction) was wrong more often
than right.

## On the residual ~14% (don't quote "Rust is 14% slower")

- It's a *young pure-Rust port vs 15-year-tuned C+asm*, not "Rust vs C." On
  aarch64 libvpx itself is C intrinsics (no ARM asm), which Rust can match — and
  largely did.
- It is **not** a safety tax: we tested bounds-check elision directly and LLVM
  already removes the checks; forcing it was a wash. Token/MV decode (the #1 cost)
  is **at parity** with libvpx's C.
- The residual is: **sixtap ~1.3×** (exhausted — 3 restructures washed; needs
  hand-assembly to match libvpx's scheduling), and **diffuse per-MB control
  overhead** (no concrete hot spot; broad refactor, low expected return).
- For reference: rav1d (c2rust port of dav1d) runs ~5% slower **but links
  dav1d's hand-written assembly**; its gap is Rust *glue*, ours includes kernels
  we wrote in Rust. On that basis ~1.14× pure-Rust is a strong result.

## Recommendation & next phase

Treat aarch64 as **done at ~1.14×** — it's near the practical floor for pure-Rust
*intrinsics*. Going further means trading the pure-Rust property for targeted
assembly (sixtap), which is a deliberate product decision, not a quick win.

**x86 SSE is the next phase**, and it's a different game: libvpx ships ~68 hand-
written `.asm` files for SSE/SSSE3 there. Pure-Rust intrinsics may genuinely
trail that, so consider the rav1d approach (link the reference asm for the
hottest kernels) if parity matters. Reuse the arithmetic-scheme lessons above;
the portable `Simd` trait already lets one bit-exact kernel body target both
NEON and SSE.
