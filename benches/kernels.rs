//! VP8 hot-kernel microbenchmarks.
//!
//! Complements the end-to-end `decode` bench with per-kernel timings, so a
//! perf change can be localized to a specific reconstruction primitive rather
//! than read only as a whole-frame delta. Inputs are fixed, tiny, and
//! deterministic; correctness is owned by the differential tests, so this file
//! is purely a relative speed signal.
//!
//! Scope is the kernels reachable through the public API: the inverse
//! transforms (IDCT / DC-only IDCT / inverse Walsh) and the macroblock intra
//! predictors. The sixtap inter-predictor and the loop filter are `pub(crate)`
//! and are only exercised here indirectly, via the `decode` bench.
//!
//! Run: `cargo bench --bench kernels`

use crabvpx::vp8::common::idctllm::{
    vp8_dc_only_idct_add_safe, vp8_short_idct4x4llm_safe, vp8_short_inv_walsh4x4_safe,
};
use crabvpx::vp8::common::reconintra::{
    vp8_build_intra_predictors_mbuv_safe, vp8_build_intra_predictors_mby_safe,
};
use crabvpx::vp8::common::types::{DC_PRED, TM_PRED};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

/// A representative non-trivial 4x4 residual block.
const COEFFS: [i16; 16] = [
    128, -64, 32, -16, 48, -24, 12, -6, 20, -10, 5, -2, 8, -4, 2, -1,
];
/// A representative 4x4 predictor.
const PRED4: [u8; 16] = [
    100, 110, 120, 130, 105, 115, 125, 135, 108, 118, 128, 138, 112, 122, 132, 142,
];

fn bench_transforms(c: &mut Criterion) {
    let mut g = c.benchmark_group("transform");

    g.bench_function("short_idct4x4llm", |b| {
        b.iter(|| {
            let mut dst = [0u8; 16];
            vp8_short_idct4x4llm_safe(black_box(&COEFFS), black_box(PRED4), &mut dst, 4);
            black_box(dst)
        })
    });

    g.bench_function("dc_only_idct_add", |b| {
        b.iter(|| {
            let mut dst = [0u8; 16];
            vp8_dc_only_idct_add_safe(black_box(96), black_box(&PRED4), 4, &mut dst, 4);
            black_box(dst)
        })
    });

    g.bench_function("short_inv_walsh4x4", |b| {
        b.iter(|| {
            // The kernel scatters the 16 DCs across a macroblock dqcoeff array
            // at stride 16 (last write is index 240).
            let mut dqcoeff = [0i16; 256];
            vp8_short_inv_walsh4x4_safe(black_box(&COEFFS), &mut dqcoeff);
            black_box(dqcoeff)
        })
    });

    g.finish();
}

fn bench_intra(c: &mut Criterion) {
    let mut g = c.benchmark_group("intra_pred");

    // 16 above samples + the top-left corner; 16 left samples.
    let yabove: [u8; 17] = std::array::from_fn(|i| (90 + i * 3) as u8);
    let yleft: [u8; 16] = std::array::from_fn(|i| (70 + i * 5) as u8);

    for (label, mode) in [("dc_16x16", DC_PRED), ("tm_16x16", TM_PRED)] {
        g.bench_function(label, |b| {
            b.iter(|| {
                let mut ypred = [0u8; 16 * 16];
                vp8_build_intra_predictors_mby_safe(
                    black_box(mode),
                    1,
                    1,
                    black_box(&yabove),
                    black_box(&yleft),
                    &mut ypred,
                    16,
                );
                black_box(ypred)
            })
        });
    }

    // Chroma: 8 above + corner per plane, 8 left per plane.
    let uabove: [u8; 9] = std::array::from_fn(|i| (95 + i * 4) as u8);
    let vabove: [u8; 9] = std::array::from_fn(|i| (100 + i * 4) as u8);
    let uleft: [u8; 8] = std::array::from_fn(|i| (80 + i * 6) as u8);
    let vleft: [u8; 8] = std::array::from_fn(|i| (85 + i * 6) as u8);

    g.bench_function("dc_8x8_uv", |b| {
        b.iter(|| {
            let mut upred = [0u8; 8 * 8];
            let mut vpred = [0u8; 8 * 8];
            vp8_build_intra_predictors_mbuv_safe(
                black_box(DC_PRED),
                1,
                1,
                black_box(&uabove),
                black_box(&vabove),
                black_box(&uleft),
                black_box(&vleft),
                &mut upred,
                &mut vpred,
                8,
            );
            black_box((upred, vpred))
        })
    });

    g.finish();
}

criterion_group!(benches, bench_transforms, bench_intra);
criterion_main!(benches);
