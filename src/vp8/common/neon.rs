//! NEON SIMD kernels (aarch64), per ADR-014.
//!
//! This module is the only place the decoder carries SIMD `unsafe`. Each kernel
//! is a bit-exact twin of a scalar reference (here, `filter::filter_block2d_sixtap_safe`),
//! gated by `tests` below and by the end-to-end `differential_md5` suite. All
//! arithmetic is ordinary safe code: calling a NEON intrinsic from inside a
//! `#[target_feature(enable = "neon")]` fn is safe on Rust >= 1.87. The residual
//! `unsafe` is confined to register loads/stores (NEON is ARMv8-A baseline, so
//! the target-feature precondition always holds here).
#![cfg(target_arch = "aarch64")]

use core::arch::aarch64::*;

const HALF: i32 = 64; // VP8_FILTER_WEIGHT >> 1
const SHIFT: i32 = 7; // VP8_FILTER_SHIFT

/// Max intermediate size: (16 + 5) rows * 16 wide, plus slack for the 8-lane
/// vertical loads to read past a width-4 row without going out of bounds.
const MID_LEN: usize = 21 * 16 + 8;

/// Bit-exact NEON twin of [`crate::vp8::common::filter::filter_block2d_sixtap_safe`].
///
/// Two separable passes: horizontal 6-tap producing a clamped [0,255] i16
/// intermediate, then vertical 6-tap to the u8 destination. Falls back to the
/// scalar kernel if the source slice is too short for the (border-backed) NEON
/// over-reads, so it is always memory-safe.
pub(crate) fn filter_block2d_sixtap_neon(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    h_filter: &[i16; 6],
    v_filter: &[i16; 6],
) {
    let ih = height + 5;
    // Horizontal pass reads up to `base + 13` per 8-col chunk; for the last
    // chunk that can exceed width+5 by a few bytes (covered by the frame border
    // in real use). Require it; otherwise use the scalar twin.
    let last_base = ((width + 7) / 8 - 1) * 8;
    let req_src = ih.saturating_sub(1) * src_stride + last_base + 13;
    let req_dst = height.saturating_sub(1) * dst_pitch + width;
    if src.len() < req_src || dst.len() < req_dst || width == 0 || height == 0 {
        crate::vp8::common::filter::filter_block2d_sixtap_safe(
            src, src_stride, dst, dst_pitch, width, height, h_filter, v_filter,
        );
        return;
    }

    let mut mid = [0i16; MID_LEN];
    // SAFETY: NEON is ARMv8-A baseline (discharges the `#[target_feature]`
    // precondition). The length checks above guarantee every load/store below
    // stays in bounds of `src`, `dst`, and the padded `mid` buffer.
    unsafe {
        sixtap_impl(
            src, src_stride, &mut mid, dst, dst_pitch, width, height, h_filter, v_filter,
        );
    }
}

#[target_feature(enable = "neon")]
fn sixtap_impl(
    src: &[u8],
    src_stride: usize,
    mid: &mut [i16],
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    hf: &[i16; 6],
    vf: &[i16; 6],
) {
    let ih = height + 5;

    // ---- Horizontal pass: src(u8) -> mid(i16, clamped 0..255) ----
    for i in 0..ih {
        let row = i * src_stride;
        let mout = i * width;
        let mut c = 0;
        while c < width {
            let base = row + c;
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            // 6 taps: src[base+k .. base+k+8]
            macro_rules! tap {
                ($k:literal) => {{
                    // SAFETY: base + k + 8 <= req_src by construction.
                    let v = unsafe { vld1_u8(src.as_ptr().add(base + $k)) };
                    let w = vreinterpretq_s16_u16(vmovl_u8(v));
                    let f = vdup_n_s16(hf[$k]);
                    acc_lo = vmlal_s16(acc_lo, vget_low_s16(w), f);
                    acc_hi = vmlal_s16(acc_hi, vget_high_s16(w), f);
                }};
            }
            tap!(0);
            tap!(1);
            tap!(2);
            tap!(3);
            tap!(4);
            tap!(5);
            let res = pack_clamped(acc_lo, acc_hi);
            store_i16(&mut mid[mout + c..], res, (width - c).min(8));
            c += 8;
        }
    }

    // ---- Vertical pass: mid(i16) -> dst(u8) ----
    for i in 0..height {
        let mut c = 0;
        while c < width {
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            macro_rules! vtap {
                ($k:literal) => {{
                    // SAFETY: (i + k) < ih and the +8 slack on `mid` keeps the
                    // 8-lane load in bounds even when width == 4.
                    let m = unsafe { vld1q_s16(mid.as_ptr().add((i + $k) * width + c)) };
                    let f = vdup_n_s16(vf[$k]);
                    acc_lo = vmlal_s16(acc_lo, vget_low_s16(m), f);
                    acc_hi = vmlal_s16(acc_hi, vget_high_s16(m), f);
                }};
            }
            vtap!(0);
            vtap!(1);
            vtap!(2);
            vtap!(3);
            vtap!(4);
            vtap!(5);
            let res = pack_clamped(acc_lo, acc_hi); // i16x8 in [0,255]
            let bytes = vmovn_u16(vreinterpretq_u16_s16(res)); // u8x8
            store_u8(&mut dst[i * dst_pitch + c..], bytes, (width - c).min(8));
            c += 8;
        }
    }
}

/// `(acc + ... already has HALF) >> 7`, clamped to [0,255], as i16x8.
#[target_feature(enable = "neon")]
fn pack_clamped(acc_lo: int32x4_t, acc_hi: int32x4_t) -> int16x8_t {
    let lo = vshrq_n_s32::<SHIFT>(acc_lo);
    let hi = vshrq_n_s32::<SHIFT>(acc_hi);
    let zero = vdupq_n_s32(0);
    let max = vdupq_n_s32(255);
    let lo = vminq_s32(vmaxq_s32(lo, zero), max);
    let hi = vminq_s32(vmaxq_s32(hi, zero), max);
    vcombine_s16(vmovn_s32(lo), vmovn_s32(hi))
}

#[target_feature(enable = "neon")]
fn store_i16(dst: &mut [i16], v: int16x8_t, n: usize) {
    if n >= 8 {
        // SAFETY: caller guarantees dst has >= 8 elements when n >= 8.
        unsafe { vst1q_s16(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0i16; 8];
        unsafe { vst1q_s16(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

#[target_feature(enable = "neon")]
fn store_u8(dst: &mut [u8], v: uint8x8_t, n: usize) {
    if n >= 8 {
        // SAFETY: caller guarantees dst has >= 8 elements when n >= 8.
        unsafe { vst1_u8(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0u8; 8];
        unsafe { vst1_u8(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

// ===========================================================================
// Normal loop filter (block edges) — the branchy hotspot LLVM can't auto-vec.
// Bit-exact with loopfilter_filters::{vp8_filter_mask, vp8_hevmask, vp8_filter}.
// The filter math is done in widened i16 so the single i32-domain clamp of
// `fv + 3*(qs0-ps0)` in the scalar is reproduced exactly (an s8 saturating-add
// chain would round differently).
// ===========================================================================

#[target_feature(enable = "neon")]
fn clamp_s8_i16(x: int16x8_t) -> int16x8_t {
    vminq_s16(vmaxq_s16(x, vdupq_n_s16(-128)), vdupq_n_s16(127))
}

#[target_feature(enable = "neon")]
fn widen_mask(m: uint8x8_t) -> int16x8_t {
    // 0x00 -> 0x0000, 0xFF -> 0xFFFF (sign-extend the 0/-1 byte mask).
    vmovl_s8(vreinterpret_s8_u8(m))
}

/// 0xFF where the edge should be filtered (all tap deltas within `limit` and the
/// blimit term within `blimit`). Matches `vp8_filter_mask` (which returns mask-1).
#[target_feature(enable = "neon")]
fn filter_mask8(
    limit: u8, blimit: u8,
    p3: uint8x8_t, p2: uint8x8_t, p1: uint8x8_t, p0: uint8x8_t,
    q0: uint8x8_t, q1: uint8x8_t, q2: uint8x8_t, q3: uint8x8_t,
) -> uint8x8_t {
    let lim = vdup_n_u8(limit);
    let mut v = vcgt_u8(vabd_u8(p3, p2), lim);
    v = vorr_u8(v, vcgt_u8(vabd_u8(p2, p1), lim));
    v = vorr_u8(v, vcgt_u8(vabd_u8(p1, p0), lim));
    v = vorr_u8(v, vcgt_u8(vabd_u8(q1, q0), lim));
    v = vorr_u8(v, vcgt_u8(vabd_u8(q2, q1), lim));
    v = vorr_u8(v, vcgt_u8(vabd_u8(q3, q2), lim));
    // |p0-q0|*2 + |p1-q1|/2 > blimit, computed in u16 to avoid overflow.
    let a = vmovl_u8(vabd_u8(p0, q0));
    let b = vmovl_u8(vabd_u8(p1, q1));
    let term = vaddq_u16(vshlq_n_u16::<1>(a), vshrq_n_u16::<1>(b));
    let m7 = vmovn_u16(vcgtq_u16(term, vdupq_n_u16(blimit as u16)));
    v = vorr_u8(v, m7);
    vmvn_u8(v) // 0xFF where NOT violated
}

/// 0xFF where |p1-p0|>thresh or |q1-q0|>thresh. Matches `vp8_hevmask`.
#[target_feature(enable = "neon")]
fn hev8(thresh: u8, p1: uint8x8_t, p0: uint8x8_t, q0: uint8x8_t, q1: uint8x8_t) -> uint8x8_t {
    let t = vdup_n_u8(thresh);
    vorr_u8(vcgt_u8(vabd_u8(p1, p0), t), vcgt_u8(vabd_u8(q1, q0), t))
}

#[target_feature(enable = "neon")]
fn to_signed_i16(x: uint8x8_t, c80: uint8x8_t) -> int16x8_t {
    vmovl_s8(vreinterpret_s8_u8(veor_u8(x, c80)))
}

#[target_feature(enable = "neon")]
fn from_signed_i16(x: int16x8_t, c80: uint8x8_t) -> uint8x8_t {
    veor_u8(vreinterpret_u8_s8(vmovn_s16(x)), c80)
}

/// Bit-exact NEON twin of `vp8_filter` for 8 lanes. Returns new (p1,p0,q0,q1).
#[target_feature(enable = "neon")]
fn normal_filter8(
    mask: uint8x8_t, hev: uint8x8_t,
    p1: uint8x8_t, p0: uint8x8_t, q0: uint8x8_t, q1: uint8x8_t,
) -> (uint8x8_t, uint8x8_t, uint8x8_t, uint8x8_t) {
    let c80 = vdup_n_u8(0x80);
    let ps1 = to_signed_i16(p1, c80);
    let ps0 = to_signed_i16(p0, c80);
    let qs0 = to_signed_i16(q0, c80);
    let qs1 = to_signed_i16(q1, c80);
    let hev16 = widen_mask(hev);
    let mask16 = widen_mask(mask);
    let nhev16 = vmvnq_s16(hev16);

    let mut fv = clamp_s8_i16(vsubq_s16(ps1, qs1)); // clamp(ps1 - qs1)
    fv = vandq_s16(fv, hev16); // & hev
    let d = vsubq_s16(qs0, ps0);
    let three_d = vaddq_s16(vaddq_s16(d, d), d); // 3*(qs0-ps0), exact in i16
    fv = clamp_s8_i16(vaddq_s16(fv, three_d)); // clamp(fv + 3*(qs0-ps0))
    fv = vandq_s16(fv, mask16); // & mask

    let f1 = vshrq_n_s16::<3>(clamp_s8_i16(vaddq_s16(fv, vdupq_n_s16(4))));
    let f2 = vshrq_n_s16::<3>(clamp_s8_i16(vaddq_s16(fv, vdupq_n_s16(3))));

    let n_q0 = clamp_s8_i16(vsubq_s16(qs0, f1));
    let n_p0 = clamp_s8_i16(vaddq_s16(ps0, f2));

    let mut fv2 = vshrq_n_s16::<1>(vaddq_s16(f1, vdupq_n_s16(1))); // (f1+1)>>1
    fv2 = vandq_s16(fv2, nhev16); // & !hev
    let n_q1 = clamp_s8_i16(vsubq_s16(qs1, fv2));
    let n_p1 = clamp_s8_i16(vaddq_s16(ps1, fv2));

    (
        from_signed_i16(n_p1, c80),
        from_signed_i16(n_p0, c80),
        from_signed_i16(n_q0, c80),
        from_signed_i16(n_q1, c80),
    )
}

/// NEON twin of `loop_filter_horizontal_edge_safe`. Edge pixels are contiguous;
/// taps are at row stride `p`. Access range matches the scalar exactly.
pub(crate) fn loop_filter_horizontal_edge_neon(
    s: &mut [u8], s_offset: usize, p: usize,
    blimit: u8, limit: u8, thresh: u8, count: usize,
) {
    // SAFETY: NEON is aarch64 baseline. For each 8-lane chunk we read/write the
    // same elements the scalar loop indexes (idx-4p .. idx+3p over 8 columns),
    // which the caller guarantees in-bounds (bordered frame buffer).
    unsafe {
        let base = s.as_mut_ptr();
        for chunk in 0..count {
            let idx = s_offset + chunk * 8;
            let ld = |o: usize| vld1_u8(base.add(o) as *const u8);
            let p3 = ld(idx - 4 * p);
            let p2 = ld(idx - 3 * p);
            let p1 = ld(idx - 2 * p);
            let p0 = ld(idx - p);
            let q0 = ld(idx);
            let q1 = ld(idx + p);
            let q2 = ld(idx + 2 * p);
            let q3 = ld(idx + 3 * p);
            let mask = filter_mask8(limit, blimit, p3, p2, p1, p0, q0, q1, q2, q3);
            let hev = hev8(thresh, p1, p0, q0, q1);
            let (n1, n0, m0, m1) = normal_filter8(mask, hev, p1, p0, q0, q1);
            vst1_u8(base.add(idx - 2 * p), n1);
            vst1_u8(base.add(idx - p), n0);
            vst1_u8(base.add(idx), m0);
            vst1_u8(base.add(idx + p), m1);
        }
    }
}

/// Bit-exact NEON twin of `vp8_mbfilter` for 8 lanes (modifies p2..q2).
#[target_feature(enable = "neon")]
fn mbfilter8(
    mask: uint8x8_t, hev: uint8x8_t,
    p2: uint8x8_t, p1: uint8x8_t, p0: uint8x8_t,
    q0: uint8x8_t, q1: uint8x8_t, q2: uint8x8_t,
) -> (uint8x8_t, uint8x8_t, uint8x8_t, uint8x8_t, uint8x8_t, uint8x8_t) {
    let c80 = vdup_n_u8(0x80);
    let ps2 = to_signed_i16(p2, c80);
    let ps1 = to_signed_i16(p1, c80);
    let ps0 = to_signed_i16(p0, c80);
    let qs0 = to_signed_i16(q0, c80);
    let qs1 = to_signed_i16(q1, c80);
    let qs2 = to_signed_i16(q2, c80);
    let hev16 = widen_mask(hev);
    let mask16 = widen_mask(mask);
    let nhev16 = vmvnq_s16(hev16);

    let mut fv = clamp_s8_i16(vsubq_s16(ps1, qs1));
    let d = vsubq_s16(qs0, ps0);
    let three_d = vaddq_s16(vaddq_s16(d, d), d);
    fv = clamp_s8_i16(vaddq_s16(fv, three_d));
    fv = vandq_s16(fv, mask16);

    // hev path (narrow filter on p0/q0)
    let fh = vandq_s16(fv, hev16);
    let f1 = vshrq_n_s16::<3>(clamp_s8_i16(vaddq_s16(fh, vdupq_n_s16(4))));
    let f2 = vshrq_n_s16::<3>(clamp_s8_i16(vaddq_s16(fh, vdupq_n_s16(3))));
    let mut nq0 = clamp_s8_i16(vsubq_s16(qs0, f1));
    let mut np0 = clamp_s8_i16(vaddq_s16(ps0, f2));

    // wide path (where !hev): u = clamp((63 + fw*k) >> 7)
    let fw = vandq_s16(fv, nhev16);
    let wide = |k: i16, x: int16x8_t| -> int16x8_t {
        clamp_s8_i16(vshrq_n_s16::<7>(vaddq_s16(
            vdupq_n_s16(63),
            vmulq_s16(x, vdupq_n_s16(k)),
        )))
    };
    let u = wide(27, fw);
    nq0 = clamp_s8_i16(vsubq_s16(nq0, u));
    np0 = clamp_s8_i16(vaddq_s16(np0, u));
    let u = wide(18, fw);
    let nq1 = clamp_s8_i16(vsubq_s16(qs1, u));
    let np1 = clamp_s8_i16(vaddq_s16(ps1, u));
    let u = wide(9, fw);
    let nq2 = clamp_s8_i16(vsubq_s16(qs2, u));
    let np2 = clamp_s8_i16(vaddq_s16(ps2, u));

    (
        from_signed_i16(np2, c80),
        from_signed_i16(np1, c80),
        from_signed_i16(np0, c80),
        from_signed_i16(nq0, c80),
        from_signed_i16(nq1, c80),
        from_signed_i16(nq2, c80),
    )
}

/// NEON twin of `mbloop_filter_horizontal_edge_safe`.
pub(crate) fn mbloop_filter_horizontal_edge_neon(
    s: &mut [u8], s_offset: usize, p: usize,
    blimit: u8, limit: u8, thresh: u8, count: usize,
) {
    // SAFETY: see loop_filter_horizontal_edge_neon — same access pattern.
    unsafe {
        let base = s.as_mut_ptr();
        for chunk in 0..count {
            let idx = s_offset + chunk * 8;
            let ld = |o: usize| vld1_u8(base.add(o) as *const u8);
            let p3 = ld(idx - 4 * p);
            let p2 = ld(idx - 3 * p);
            let p1 = ld(idx - 2 * p);
            let p0 = ld(idx - p);
            let q0 = ld(idx);
            let q1 = ld(idx + p);
            let q2 = ld(idx + 2 * p);
            let q3 = ld(idx + 3 * p);
            let mask = filter_mask8(limit, blimit, p3, p2, p1, p0, q0, q1, q2, q3);
            let hev = hev8(thresh, p1, p0, q0, q1);
            let (r2, r1, r0, s0, s1, s2) = mbfilter8(mask, hev, p2, p1, p0, q0, q1, q2);
            vst1_u8(base.add(idx - 3 * p), r2);
            vst1_u8(base.add(idx - 2 * p), r1);
            vst1_u8(base.add(idx - p), r0);
            vst1_u8(base.add(idx), s0);
            vst1_u8(base.add(idx + p), s1);
            vst1_u8(base.add(idx + 2 * p), s2);
        }
    }
}

/// NEON twin of `mbloop_filter_vertical_edge_safe` (load 8 rows, transpose,
/// MB-filter, transpose back, store).
pub(crate) fn mbloop_filter_vertical_edge_neon(
    s: &mut [u8], s_offset: usize, p: usize,
    blimit: u8, limit: u8, thresh: u8, count: usize,
) {
    // SAFETY: see loop_filter_vertical_edge_neon — same access pattern.
    unsafe {
        let base = s.as_mut_ptr();
        for chunk in 0..count {
            let row0 = s_offset + chunk * 8 * p;
            let mut r = [vdup_n_u8(0); 8];
            for i in 0..8 {
                r[i] = vld1_u8(base.add(row0 + i * p - 4) as *const u8);
            }
            let t = transpose8x8(r);
            let mask = filter_mask8(limit, blimit, t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7]);
            let hev = hev8(thresh, t[2], t[3], t[4], t[5]);
            let (r2, r1, r0, s0, s1, s2) = mbfilter8(mask, hev, t[1], t[2], t[3], t[4], t[5], t[6]);
            let out = transpose8x8([t[0], r2, r1, r0, s0, s1, s2, t[7]]);
            for i in 0..8 {
                vst1_u8(base.add(row0 + i * p - 4), out[i]);
            }
        }
    }
}

/// NEON 8x8 byte transpose (rows -> columns). `out[j]` = column `j` of input.
#[target_feature(enable = "neon")]
fn transpose8x8(a: [uint8x8_t; 8]) -> [uint8x8_t; 8] {
    // L1: transpose adjacent 8-bit lanes.
    let b0 = vtrn_u8(a[0], a[1]);
    let b1 = vtrn_u8(a[2], a[3]);
    let b2 = vtrn_u8(a[4], a[5]);
    let b3 = vtrn_u8(a[6], a[7]);
    // L2: as u16, transpose lanes two apart.
    let c0 = vtrn_u16(vreinterpret_u16_u8(b0.0), vreinterpret_u16_u8(b1.0));
    let c1 = vtrn_u16(vreinterpret_u16_u8(b0.1), vreinterpret_u16_u8(b1.1));
    let c2 = vtrn_u16(vreinterpret_u16_u8(b2.0), vreinterpret_u16_u8(b3.0));
    let c3 = vtrn_u16(vreinterpret_u16_u8(b2.1), vreinterpret_u16_u8(b3.1));
    // L3: as u32, transpose lanes four apart.
    let d0 = vtrn_u32(vreinterpret_u32_u16(c0.0), vreinterpret_u32_u16(c2.0));
    let d1 = vtrn_u32(vreinterpret_u32_u16(c1.0), vreinterpret_u32_u16(c3.0));
    let d2 = vtrn_u32(vreinterpret_u32_u16(c0.1), vreinterpret_u32_u16(c2.1));
    let d3 = vtrn_u32(vreinterpret_u32_u16(c1.1), vreinterpret_u32_u16(c3.1));
    [
        vreinterpret_u8_u32(d0.0),
        vreinterpret_u8_u32(d1.0),
        vreinterpret_u8_u32(d2.0),
        vreinterpret_u8_u32(d3.0),
        vreinterpret_u8_u32(d0.1),
        vreinterpret_u8_u32(d1.1),
        vreinterpret_u8_u32(d2.1),
        vreinterpret_u8_u32(d3.1),
    ]
}

/// NEON twin of `loop_filter_vertical_edge_safe`. Edge pixels run down the
/// column at stride `p`; taps are horizontal. We load 8 rows of 8 bytes
/// (`s[idx-4 .. idx+4]`), transpose to lane-per-row, filter, transpose back,
/// and store whole rows (unmodified taps written back unchanged).
pub(crate) fn loop_filter_vertical_edge_neon(
    s: &mut [u8], s_offset: usize, p: usize,
    blimit: u8, limit: u8, thresh: u8, count: usize,
) {
    // SAFETY: NEON is aarch64 baseline. Each chunk reads/writes s[row*p-4 .. +4]
    // for 8 consecutive rows — the same elements the scalar loop touches, which
    // the caller guarantees in-bounds.
    unsafe {
        let base = s.as_mut_ptr();
        for chunk in 0..count {
            let row0 = s_offset + chunk * 8 * p;
            let mut r = [vdup_n_u8(0); 8];
            for i in 0..8 {
                r[i] = vld1_u8(base.add(row0 + i * p - 4) as *const u8);
            }
            let t = transpose8x8(r); // t[0..8] = p3,p2,p1,p0,q0,q1,q2,q3
            let mask = filter_mask8(limit, blimit, t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7]);
            let hev = hev8(thresh, t[2], t[3], t[4], t[5]);
            let (n1, n0, m0, m1) = normal_filter8(mask, hev, t[2], t[3], t[4], t[5]);
            let out = transpose8x8([t[0], t[1], n1, n0, m0, m1, t[6], t[7]]);
            for i in 0..8 {
                vst1_u8(base.add(row0 + i * p - 4), out[i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vp8::common::filter::{filter_block2d_sixtap_safe, vp8_sub_pel_filters};

    // Tiny deterministic PRNG so the test needs no deps.
    fn lcg(state: &mut u64) -> u8 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (*state >> 33) as u8
    }

    fn check_size(width: usize, height: usize) {
        let stride = width + 16; // generous border
        let ih = height + 5;
        let src_len = (ih) * stride + 32;
        for xo in 0..8 {
            for yo in 0..8 {
                let hf = &vp8_sub_pel_filters[xo];
                let vf = &vp8_sub_pel_filters[yo];
                let mut seed = (xo * 131 + yo * 977 + width * 17 + height) as u64 + 1;
                let mut src = vec![0u8; src_len];
                for b in src.iter_mut() {
                    *b = lcg(&mut seed);
                }
                let mut dst_scalar = vec![0u8; height * width];
                let mut dst_neon = vec![0u8; height * width];
                filter_block2d_sixtap_safe(&src, stride, &mut dst_scalar, width, width, height, hf, vf);
                filter_block2d_sixtap_neon(&src, stride, &mut dst_neon, width, width, height, hf, vf);
                assert_eq!(
                    dst_scalar, dst_neon,
                    "mismatch {width}x{height} xo={xo} yo={yo}"
                );
            }
        }
    }

    #[test]
    fn neon_sixtap_matches_scalar_bit_exact() {
        check_size(4, 4);
        check_size(8, 4);
        check_size(8, 8);
        check_size(16, 16);
    }

    // Run with: cargo test --release --lib neon_sixtap_microbench -- --nocapture --ignored
    #[test]
    fn neon_loopfilter_h_matches_scalar_bit_exact() {
        use crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_scalar;
        let p = 32usize; // stride
        let rows = 16usize;
        let buf_len = rows * p;
        let s_offset = 5 * p + 4; // room for -4p taps and left margin
        for trial in 0..200 {
            let mut seed = trial as u64 * 2654435761 + 1;
            // limits chosen to exercise both filtered and skipped lanes
            let blimit = vec![(lcg(&mut seed) % 40 + 1); 16];
            let limit = vec![(lcg(&mut seed) % 20 + 1); 16];
            let thresh = vec![(lcg(&mut seed) % 16); 16];
            let mut base = vec![0u8; buf_len];
            for b in base.iter_mut() {
                *b = lcg(&mut seed);
            }
            for count in 1..=2usize {
                let mut a = base.clone();
                let mut b = base.clone();
                loop_filter_horizontal_edge_scalar(&mut a, s_offset, p, &blimit, &limit, &thresh, count);
                loop_filter_horizontal_edge_neon(&mut b, s_offset, p, blimit[0], limit[0], thresh[0], count);
                assert_eq!(a, b, "h loopfilter mismatch trial={trial} count={count}");
            }
        }
    }

    #[test]
    fn neon_loopfilter_v_matches_scalar_bit_exact() {
        use crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_scalar;
        let p = 32usize;
        let rows = 24usize;
        let buf_len = rows * p;
        let s_offset = 2 * p + 8; // room for -4 .. +3 horizontal taps
        for trial in 0..200 {
            let mut seed = trial as u64 * 40503 + 7;
            let blimit = vec![(lcg(&mut seed) % 40 + 1); 16];
            let limit = vec![(lcg(&mut seed) % 20 + 1); 16];
            let thresh = vec![(lcg(&mut seed) % 16); 16];
            let mut bsrc = vec![0u8; buf_len];
            for b in bsrc.iter_mut() {
                *b = lcg(&mut seed);
            }
            for count in 1..=2usize {
                let mut a = bsrc.clone();
                let mut b = bsrc.clone();
                loop_filter_vertical_edge_scalar(&mut a, s_offset, p, &blimit, &limit, &thresh, count);
                loop_filter_vertical_edge_neon(&mut b, s_offset, p, blimit[0], limit[0], thresh[0], count);
                assert_eq!(a, b, "v loopfilter mismatch trial={trial} count={count}");
            }
        }
    }

    #[test]
    fn neon_mbloop_matches_scalar_bit_exact() {
        use crate::vp8::common::loopfilter_filters::{
            mbloop_filter_horizontal_edge_scalar, mbloop_filter_vertical_edge_scalar,
        };
        let p = 32usize;
        let buf_len = 32 * p;
        for trial in 0..200 {
            let mut seed = trial as u64 * 2246822519 + 13;
            let blimit = vec![(lcg(&mut seed) % 40 + 1); 16];
            let limit = vec![(lcg(&mut seed) % 20 + 1); 16];
            let thresh = vec![(lcg(&mut seed) % 16); 16];
            let mut bsrc = vec![0u8; buf_len];
            for b in bsrc.iter_mut() {
                *b = lcg(&mut seed);
            }
            for count in 1..=2usize {
                // horizontal
                let off_h = 6 * p + 4;
                let mut a = bsrc.clone();
                let mut b = bsrc.clone();
                mbloop_filter_horizontal_edge_scalar(&mut a, off_h, p, &blimit, &limit, &thresh, count);
                mbloop_filter_horizontal_edge_neon(&mut b, off_h, p, blimit[0], limit[0], thresh[0], count);
                assert_eq!(a, b, "mb h mismatch trial={trial} count={count}");
                // vertical
                let off_v = 2 * p + 8;
                let mut a = bsrc.clone();
                let mut b = bsrc.clone();
                mbloop_filter_vertical_edge_scalar(&mut a, off_v, p, &blimit, &limit, &thresh, count);
                mbloop_filter_vertical_edge_neon(&mut b, off_v, p, blimit[0], limit[0], thresh[0], count);
                assert_eq!(a, b, "mb v mismatch trial={trial} count={count}");
            }
        }
    }

    #[test]
    #[ignore]
    fn neon_sixtap_microbench() {
        use std::time::Instant;
        let (w, h) = (16usize, 16usize);
        let stride = w + 16;
        let src_len = (h + 5) * stride + 32;
        let mut seed = 12345u64;
        let mut src = vec![0u8; src_len];
        for b in src.iter_mut() {
            *b = lcg(&mut seed);
        }
        let hf = &vp8_sub_pel_filters[3];
        let vf = &vp8_sub_pel_filters[5];
        let mut dst = vec![0u8; h * w];
        let iters = 2_000_000u32;
        // scalar
        let t = Instant::now();
        for _ in 0..iters {
            filter_block2d_sixtap_safe(&src, stride, &mut dst, w, w, h, hf, vf);
            std::hint::black_box(&dst);
        }
        let scalar = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        // neon
        let t = Instant::now();
        for _ in 0..iters {
            filter_block2d_sixtap_neon(&src, stride, &mut dst, w, w, h, hf, vf);
            std::hint::black_box(&dst);
        }
        let neon = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        println!(
            "sixtap 16x16: scalar {scalar:.1} ns, neon {neon:.1} ns, speedup {:.2}x",
            scalar / neon
        );

        // Loop filter (16 px horizontal block edge)
        use crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_scalar;
        let p = 32usize;
        let mut s = vec![0u8; 16 * p];
        for b in s.iter_mut() {
            *b = lcg(&mut seed);
        }
        let off = 5 * p + 4;
        let bl = vec![20u8; 16];
        let li = vec![10u8; 16];
        let th = vec![7u8; 16];
        let mut sc = s.clone();
        let t = Instant::now();
        for _ in 0..iters {
            loop_filter_horizontal_edge_scalar(&mut sc, off, p, &bl, &li, &th, 2);
            std::hint::black_box(&sc);
        }
        let lf_scalar = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        let mut sn = s.clone();
        let t = Instant::now();
        for _ in 0..iters {
            loop_filter_horizontal_edge_neon(&mut sn, off, p, bl[0], li[0], th[0], 2);
            std::hint::black_box(&sn);
        }
        let lf_neon = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        println!(
            "loopfilter h 16px: scalar {lf_scalar:.1} ns, neon {lf_neon:.1} ns, speedup {:.2}x",
            lf_scalar / lf_neon
        );
    }
}
