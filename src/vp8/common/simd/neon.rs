//! NEON backend (aarch64): the [`Simd`] primitive impl, the public per-kernel
//! entry points, and the NEON-specific sixtap filter.
//!
//! All arithmetic is safe inside `#[target_feature(enable = "neon")]` (Rust >=
//! 1.87); the only `unsafe` is register loads/stores. NEON is ARMv8-A baseline,
//! so the feature precondition always holds on the targets this compiles for.

use super::Simd;
use core::arch::aarch64::*;

pub(crate) struct Neon;

// SAFETY (whole impl): every method body calls NEON intrinsics, which are
// `unsafe` but sound here because NEON is mandatory in the ARMv8-A baseline that
// this `#[cfg(target_arch = "aarch64")]` module compiles for. The loads/stores
// additionally require the caller-guaranteed in-bounds pointers documented on
// `load_u8`/`store_u8`.
impl Simd for Neon {
    type U8 = uint8x8_t;
    type I16 = int16x8_t;

    #[inline(always)]
    unsafe fn load_u8(p: *const u8) -> uint8x8_t {
        unsafe { vld1_u8(p) }
    }
    #[inline(always)]
    unsafe fn store_u8(p: *mut u8, v: uint8x8_t) {
        unsafe { vst1_u8(p, v) }
    }

    #[inline(always)]
    fn splat_u8(x: u8) -> uint8x8_t {
        unsafe { vdup_n_u8(x) }
    }
    #[inline(always)]
    fn abd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
        unsafe { vabd_u8(a, b) }
    }
    #[inline(always)]
    fn cgt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
        unsafe { vcgt_u8(a, b) }
    }
    #[inline(always)]
    fn or_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
        unsafe { vorr_u8(a, b) }
    }
    #[inline(always)]
    fn not_u8(a: uint8x8_t) -> uint8x8_t {
        unsafe { vmvn_u8(a) }
    }

    #[inline(always)]
    fn widen_u8(a: uint8x8_t) -> int16x8_t {
        unsafe { vreinterpretq_s16_u16(vmovl_u8(a)) }
    }
    #[inline(always)]
    fn widen_mask(a: uint8x8_t) -> int16x8_t {
        unsafe { vmovl_s8(vreinterpret_s8_u8(a)) }
    }
    #[inline(always)]
    fn narrow_mask(a: int16x8_t) -> uint8x8_t {
        unsafe { vmovn_u16(vreinterpretq_u16_s16(a)) }
    }
    #[inline(always)]
    fn to_signed_i16(a: uint8x8_t) -> int16x8_t {
        unsafe { vmovl_s8(vreinterpret_s8_u8(veor_u8(a, vdup_n_u8(0x80)))) }
    }
    #[inline(always)]
    fn from_signed_i16(a: int16x8_t) -> uint8x8_t {
        unsafe { veor_u8(vreinterpret_u8_s8(vmovn_s16(a)), vdup_n_u8(0x80)) }
    }

    #[inline(always)]
    fn splat_i16(x: i16) -> int16x8_t {
        unsafe { vdupq_n_s16(x) }
    }
    #[inline(always)]
    fn add_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vaddq_s16(a, b) }
    }
    #[inline(always)]
    fn sub_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vsubq_s16(a, b) }
    }
    #[inline(always)]
    fn mul_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vmulq_s16(a, b) }
    }
    #[inline(always)]
    fn and_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vandq_s16(a, b) }
    }
    #[inline(always)]
    fn cgt_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vreinterpretq_s16_u16(vcgtq_s16(a, b)) }
    }
    #[inline(always)]
    fn min_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vminq_s16(a, b) }
    }
    #[inline(always)]
    fn max_i16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
        unsafe { vmaxq_s16(a, b) }
    }
    #[inline(always)]
    fn shl_i16<const N: i32>(a: int16x8_t) -> int16x8_t {
        unsafe { vshlq_n_s16::<N>(a) }
    }
    #[inline(always)]
    fn shr_i16<const N: i32>(a: int16x8_t) -> int16x8_t {
        unsafe { vshrq_n_s16::<N>(a) }
    }

    #[inline(always)]
    fn transpose8x8(a: [uint8x8_t; 8]) -> [uint8x8_t; 8] {
        unsafe {
            let b0 = vtrn_u8(a[0], a[1]);
            let b1 = vtrn_u8(a[2], a[3]);
            let b2 = vtrn_u8(a[4], a[5]);
            let b3 = vtrn_u8(a[6], a[7]);
            let c0 = vtrn_u16(vreinterpret_u16_u8(b0.0), vreinterpret_u16_u8(b1.0));
            let c1 = vtrn_u16(vreinterpret_u16_u8(b0.1), vreinterpret_u16_u8(b1.1));
            let c2 = vtrn_u16(vreinterpret_u16_u8(b2.0), vreinterpret_u16_u8(b3.0));
            let c3 = vtrn_u16(vreinterpret_u16_u8(b2.1), vreinterpret_u16_u8(b3.1));
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
    }
}

// --- Public per-kernel entry points (called from the scalar dispatchers) ---

pub(crate) fn loop_filter_horizontal_edge_neon(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    super::kernels::loop_filter_horizontal_edge::<Neon>(
        s, s_offset, p, blimit, limit, thresh, count,
    );
}
pub(crate) fn loop_filter_vertical_edge_neon(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    super::kernels::loop_filter_vertical_edge::<Neon>(s, s_offset, p, blimit, limit, thresh, count);
}
pub(crate) fn mbloop_filter_horizontal_edge_neon(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    super::kernels::mbloop_filter_horizontal_edge::<Neon>(
        s, s_offset, p, blimit, limit, thresh, count,
    );
}
pub(crate) fn mbloop_filter_vertical_edge_neon(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    super::kernels::mbloop_filter_vertical_edge::<Neon>(
        s, s_offset, p, blimit, limit, thresh, count,
    );
}
pub(crate) fn simple_horizontal_edge_neon(
    y: &mut [u8],
    y_offset: usize,
    stride: usize,
    blimit: u8,
) {
    super::kernels::simple_horizontal_edge::<Neon>(y, y_offset, stride, blimit);
}
pub(crate) fn simple_vertical_edge_neon(y: &mut [u8], y_offset: usize, stride: usize, blimit: u8) {
    super::kernels::simple_vertical_edge::<Neon>(y, y_offset, stride, blimit);
}

// ===========================================================================
// NEON-specific sixtap sub-pixel filter. Not generic: it auto-vectorizes well
// in scalar (~1.13x), below the bar to port to other ISAs, so it stays NEON-only
// and isn't part of the `Simd` trait. Bit-exact with filter_block2d_sixtap_safe.
// ===========================================================================

const HALF: i32 = 64;
const SHIFT: i32 = 7;
const MID_LEN: usize = 21 * 16 + 8;

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
    let last_base = (width.div_ceil(8) - 1) * 8;
    let req_src = ih.saturating_sub(1) * src_stride + last_base + 13;
    let req_dst = height.saturating_sub(1) * dst_pitch + width;
    if src.len() < req_src || dst.len() < req_dst || width == 0 || height == 0 {
        crate::vp8::common::filter::filter_block2d_sixtap_safe(
            src, src_stride, dst, dst_pitch, width, height, h_filter, v_filter,
        );
        return;
    }
    let mut mid = [0i16; MID_LEN];
    // SAFETY: NEON baseline; the length checks above keep every load/store in
    // bounds of `src`, `dst`, and the padded `mid`.
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
    for i in 0..ih {
        let row = i * src_stride;
        let mout = i * width;
        let mut c = 0;
        while c < width {
            let base = row + c;
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            macro_rules! tap {
                ($k:literal) => {{
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
            sixtap_store_i16(&mut mid[mout + c..], res, (width - c).min(8));
            c += 8;
        }
    }
    for i in 0..height {
        let mut c = 0;
        while c < width {
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            macro_rules! vtap {
                ($k:literal) => {{
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
            let res = pack_clamped(acc_lo, acc_hi);
            let bytes = vmovn_u16(vreinterpretq_u16_s16(res));
            sixtap_store_u8(&mut dst[i * dst_pitch + c..], bytes, (width - c).min(8));
            c += 8;
        }
    }
}

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
fn sixtap_store_i16(dst: &mut [i16], v: int16x8_t, n: usize) {
    if n >= 8 {
        unsafe { vst1q_s16(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0i16; 8];
        unsafe { vst1q_s16(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

#[target_feature(enable = "neon")]
fn sixtap_store_u8(dst: &mut [u8], v: uint8x8_t, n: usize) {
    if n >= 8 {
        unsafe { vst1_u8(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0u8; 8];
        unsafe { vst1_u8(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vp8::common::filter::{filter_block2d_sixtap_safe, vp8_sub_pel_filters};
    use crate::vp8::common::loopfilter_filters::{
        loop_filter_horizontal_edge_scalar, loop_filter_vertical_edge_scalar,
        mbloop_filter_horizontal_edge_scalar, mbloop_filter_vertical_edge_scalar,
    };

    fn lcg(state: &mut u64) -> u8 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (*state >> 33) as u8
    }

    fn check_sixtap(width: usize, height: usize) {
        let stride = width + 16;
        let src_len = (height + 5) * stride + 32;
        for xo in 0..8 {
            for yo in 0..8 {
                let hf = &vp8_sub_pel_filters[xo];
                let vf = &vp8_sub_pel_filters[yo];
                let mut seed = (xo * 131 + yo * 977 + width * 17 + height) as u64 + 1;
                let mut src = vec![0u8; src_len];
                for b in src.iter_mut() {
                    *b = lcg(&mut seed);
                }
                let mut a = vec![0u8; height * width];
                let mut b = vec![0u8; height * width];
                filter_block2d_sixtap_safe(&src, stride, &mut a, width, width, height, hf, vf);
                filter_block2d_sixtap_neon(&src, stride, &mut b, width, width, height, hf, vf);
                assert_eq!(a, b, "sixtap {width}x{height} xo={xo} yo={yo}");
            }
        }
    }

    #[test]
    fn neon_sixtap_matches_scalar_bit_exact() {
        check_sixtap(4, 4);
        check_sixtap(8, 4);
        check_sixtap(8, 8);
        check_sixtap(16, 16);
    }

    #[test]
    fn neon_loopfilter_matches_scalar_bit_exact() {
        let p = 32usize;
        for trial in 0..200 {
            let mut seed = trial as u64 * 2654435761 + 1;
            let blimit = vec![lcg(&mut seed) % 40 + 1; 16];
            let limit = vec![lcg(&mut seed) % 20 + 1; 16];
            let thresh = vec![lcg(&mut seed) % 16; 16];
            let mut buf = vec![0u8; 32 * p];
            for x in buf.iter_mut() {
                *x = lcg(&mut seed);
            }
            for count in 1..=2usize {
                // block edges
                let (a, b) = run2(
                    &buf,
                    6 * p + 4,
                    p,
                    &blimit,
                    &limit,
                    &thresh,
                    count,
                    loop_filter_horizontal_edge_scalar,
                    loop_filter_horizontal_edge_neon,
                );
                assert_eq!(a, b, "lf h trial={trial} count={count}");
                let (a, b) = run2(
                    &buf,
                    2 * p + 8,
                    p,
                    &blimit,
                    &limit,
                    &thresh,
                    count,
                    loop_filter_vertical_edge_scalar,
                    loop_filter_vertical_edge_neon,
                );
                assert_eq!(a, b, "lf v trial={trial} count={count}");
                // MB edges
                let (a, b) = run2(
                    &buf,
                    6 * p + 4,
                    p,
                    &blimit,
                    &limit,
                    &thresh,
                    count,
                    mbloop_filter_horizontal_edge_scalar,
                    mbloop_filter_horizontal_edge_neon,
                );
                assert_eq!(a, b, "mb h trial={trial} count={count}");
                let (a, b) = run2(
                    &buf,
                    2 * p + 8,
                    p,
                    &blimit,
                    &limit,
                    &thresh,
                    count,
                    mbloop_filter_vertical_edge_scalar,
                    mbloop_filter_vertical_edge_neon,
                );
                assert_eq!(a, b, "mb v trial={trial} count={count}");
            }
        }
    }

    #[test]
    fn neon_simple_loopfilter_matches_scalar_bit_exact() {
        use crate::vp8::common::loopfilter_filters::{
            vp8_loop_filter_simple_horizontal_edge_scalar,
            vp8_loop_filter_simple_vertical_edge_scalar,
        };
        let stride = 32usize;
        for trial in 0..200 {
            let mut seed = trial as u64 * 1099087573 + 3;
            let blimit = lcg(&mut seed) % 60 + 1;
            let mut buf = vec![0u8; 24 * stride];
            for x in buf.iter_mut() {
                *x = lcg(&mut seed);
            }
            // horizontal (off has >= 2 rows above, 16 cols)
            let mut a = buf.clone();
            let mut b = buf.clone();
            vp8_loop_filter_simple_horizontal_edge_scalar(&mut a, 6 * stride, stride, blimit);
            simple_horizontal_edge_neon(&mut b, 6 * stride, stride, blimit);
            assert_eq!(a, b, "simple h trial={trial}");
            // vertical
            let mut a = buf.clone();
            let mut b = buf.clone();
            vp8_loop_filter_simple_vertical_edge_scalar(&mut a, 2 * stride + 8, stride, blimit);
            simple_vertical_edge_neon(&mut b, 2 * stride + 8, stride, blimit);
            assert_eq!(a, b, "simple v trial={trial}");
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn run2(
        buf: &[u8],
        off: usize,
        p: usize,
        bl: &[u8],
        li: &[u8],
        th: &[u8],
        count: usize,
        scalar: fn(&mut [u8], usize, usize, &[u8], &[u8], &[u8], usize),
        neon: fn(&mut [u8], usize, usize, u8, u8, u8, usize),
    ) -> (Vec<u8>, Vec<u8>) {
        let mut a = buf.to_vec();
        let mut b = buf.to_vec();
        scalar(&mut a, off, p, bl, li, th, count);
        neon(&mut b, off, p, bl[0], li[0], th[0], count);
        (a, b)
    }
}
