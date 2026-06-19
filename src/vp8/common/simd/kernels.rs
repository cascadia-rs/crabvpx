//! Loop-filter kernels written once, generic over the [`Simd`] backend.
//!
//! Bit-exact with the scalar reference in `loopfilter_filters` (the filter math
//! is in widened i16 so the scalar's single i32-domain clamps are reproduced
//! exactly). The vertical edges reuse the lane-parallel core via an 8x8
//! transpose, so horizontal and vertical share all the filtering logic.

use super::Simd;

#[inline(always)]
fn clamp_s8<S: Simd>(x: S::I16) -> S::I16 {
    S::min_i16(S::max_i16(x, S::splat_i16(-128)), S::splat_i16(127))
}

/// 0xFF where the edge should be filtered. Matches `vp8_filter_mask`.
#[inline(always)]
fn filter_mask8<S: Simd>(
    limit: u8,
    blimit: u8,
    p3: S::U8,
    p2: S::U8,
    p1: S::U8,
    p0: S::U8,
    q0: S::U8,
    q1: S::U8,
    q2: S::U8,
    q3: S::U8,
) -> S::U8 {
    let lim = S::splat_u8(limit);
    let mut v = S::cgt_u8(S::abd_u8(p3, p2), lim);
    v = S::or_u8(v, S::cgt_u8(S::abd_u8(p2, p1), lim));
    v = S::or_u8(v, S::cgt_u8(S::abd_u8(p1, p0), lim));
    v = S::or_u8(v, S::cgt_u8(S::abd_u8(q1, q0), lim));
    v = S::or_u8(v, S::cgt_u8(S::abd_u8(q2, q1), lim));
    v = S::or_u8(v, S::cgt_u8(S::abd_u8(q3, q2), lim));
    // |p0-q0|*2 + |p1-q1|/2 > blimit (values fit in positive i16)
    let a = S::widen_u8(S::abd_u8(p0, q0));
    let b = S::widen_u8(S::abd_u8(p1, q1));
    let term = S::add_i16(S::shl_i16::<1>(a), S::shr_i16::<1>(b));
    let m7 = S::narrow_mask(S::cgt_i16(term, S::splat_i16(blimit as i16)));
    v = S::or_u8(v, m7);
    S::not_u8(v)
}

/// 0xFF where |p1-p0|>thresh or |q1-q0|>thresh. Matches `vp8_hevmask`.
#[inline(always)]
fn hev8<S: Simd>(thresh: u8, p1: S::U8, p0: S::U8, q0: S::U8, q1: S::U8) -> S::U8 {
    let t = S::splat_u8(thresh);
    S::or_u8(
        S::cgt_u8(S::abd_u8(p1, p0), t),
        S::cgt_u8(S::abd_u8(q1, q0), t),
    )
}

/// Bit-exact twin of `vp8_filter` (block edge): new (p1,p0,q0,q1).
#[inline(always)]
fn normal_filter8<S: Simd>(
    mask: S::U8,
    hev: S::U8,
    p1: S::U8,
    p0: S::U8,
    q0: S::U8,
    q1: S::U8,
) -> (S::U8, S::U8, S::U8, S::U8) {
    let ps1 = S::to_signed_i16(p1);
    let ps0 = S::to_signed_i16(p0);
    let qs0 = S::to_signed_i16(q0);
    let qs1 = S::to_signed_i16(q1);
    let hev16 = S::widen_mask(hev);
    let mask16 = S::widen_mask(mask);
    let nhev16 = S::widen_mask(S::not_u8(hev));

    let mut fv = clamp_s8::<S>(S::sub_i16(ps1, qs1));
    fv = S::and_i16(fv, hev16);
    let d = S::sub_i16(qs0, ps0);
    let three_d = S::add_i16(S::add_i16(d, d), d);
    fv = clamp_s8::<S>(S::add_i16(fv, three_d));
    fv = S::and_i16(fv, mask16);

    let f1 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fv, S::splat_i16(4))));
    let f2 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fv, S::splat_i16(3))));
    let n_q0 = clamp_s8::<S>(S::sub_i16(qs0, f1));
    let n_p0 = clamp_s8::<S>(S::add_i16(ps0, f2));

    let mut fv2 = S::shr_i16::<1>(S::add_i16(f1, S::splat_i16(1)));
    fv2 = S::and_i16(fv2, nhev16);
    let n_q1 = clamp_s8::<S>(S::sub_i16(qs1, fv2));
    let n_p1 = clamp_s8::<S>(S::add_i16(ps1, fv2));

    (
        S::from_signed_i16(n_p1),
        S::from_signed_i16(n_p0),
        S::from_signed_i16(n_q0),
        S::from_signed_i16(n_q1),
    )
}

/// Bit-exact twin of `vp8_mbfilter` (MB edge): new (p2,p1,p0,q0,q1,q2).
#[inline(always)]
fn mbfilter8<S: Simd>(
    mask: S::U8,
    hev: S::U8,
    p2: S::U8,
    p1: S::U8,
    p0: S::U8,
    q0: S::U8,
    q1: S::U8,
    q2: S::U8,
) -> (S::U8, S::U8, S::U8, S::U8, S::U8, S::U8) {
    let ps2 = S::to_signed_i16(p2);
    let ps1 = S::to_signed_i16(p1);
    let ps0 = S::to_signed_i16(p0);
    let qs0 = S::to_signed_i16(q0);
    let qs1 = S::to_signed_i16(q1);
    let qs2 = S::to_signed_i16(q2);
    let hev16 = S::widen_mask(hev);
    let mask16 = S::widen_mask(mask);
    let nhev16 = S::widen_mask(S::not_u8(hev));

    let mut fv = clamp_s8::<S>(S::sub_i16(ps1, qs1));
    let d = S::sub_i16(qs0, ps0);
    let three_d = S::add_i16(S::add_i16(d, d), d);
    fv = clamp_s8::<S>(S::add_i16(fv, three_d));
    fv = S::and_i16(fv, mask16);

    // hev (narrow) path on p0/q0
    let fh = S::and_i16(fv, hev16);
    let f1 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fh, S::splat_i16(4))));
    let f2 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fh, S::splat_i16(3))));
    let mut nq0 = clamp_s8::<S>(S::sub_i16(qs0, f1));
    let mut np0 = clamp_s8::<S>(S::add_i16(ps0, f2));

    // wide path (where !hev): u = clamp((63 + fw*k) >> 7)
    let fw = S::and_i16(fv, nhev16);
    let wide = |k: i16, x: S::I16| -> S::I16 {
        clamp_s8::<S>(S::shr_i16::<7>(S::add_i16(
            S::splat_i16(63),
            S::mul_i16(x, S::splat_i16(k)),
        )))
    };
    let u = wide(27, fw);
    nq0 = clamp_s8::<S>(S::sub_i16(nq0, u));
    np0 = clamp_s8::<S>(S::add_i16(np0, u));
    let u = wide(18, fw);
    let nq1 = clamp_s8::<S>(S::sub_i16(qs1, u));
    let np1 = clamp_s8::<S>(S::add_i16(ps1, u));
    let u = wide(9, fw);
    let nq2 = clamp_s8::<S>(S::sub_i16(qs2, u));
    let np2 = clamp_s8::<S>(S::add_i16(ps2, u));

    (
        S::from_signed_i16(np2),
        S::from_signed_i16(np1),
        S::from_signed_i16(np0),
        S::from_signed_i16(nq0),
        S::from_signed_i16(nq1),
        S::from_signed_i16(nq2),
    )
}

// --------------------------------------------------------------------------
// Edge entry points (load -> filter -> store). Contiguous edge pixels for the
// horizontal case; an 8x8 transpose makes the vertical case reuse the core.
// --------------------------------------------------------------------------

pub(crate) fn loop_filter_horizontal_edge<S: Simd>(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    let base = s.as_mut_ptr();
    for chunk in 0..count {
        let idx = s_offset + chunk * 8;
        // SAFETY: same elements the scalar loop indexes (idx-4p..idx+3p over 8
        // columns), guaranteed in-bounds by the caller (bordered buffer).
        unsafe {
            let p3 = S::load_u8(base.add(idx - 4 * p));
            let p2 = S::load_u8(base.add(idx - 3 * p));
            let p1 = S::load_u8(base.add(idx - 2 * p));
            let p0 = S::load_u8(base.add(idx - p));
            let q0 = S::load_u8(base.add(idx));
            let q1 = S::load_u8(base.add(idx + p));
            let q2 = S::load_u8(base.add(idx + 2 * p));
            let q3 = S::load_u8(base.add(idx + 3 * p));
            let mask = filter_mask8::<S>(limit, blimit, p3, p2, p1, p0, q0, q1, q2, q3);
            let hev = hev8::<S>(thresh, p1, p0, q0, q1);
            let (n1, n0, m0, m1) = normal_filter8::<S>(mask, hev, p1, p0, q0, q1);
            S::store_u8(base.add(idx - 2 * p), n1);
            S::store_u8(base.add(idx - p), n0);
            S::store_u8(base.add(idx), m0);
            S::store_u8(base.add(idx + p), m1);
        }
    }
}

pub(crate) fn loop_filter_vertical_edge<S: Simd>(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    let base = s.as_mut_ptr();
    for chunk in 0..count {
        let row0 = s_offset + chunk * 8 * p;
        // SAFETY: reads/writes s[row*p-4 .. +4] for 8 rows — same as scalar.
        unsafe {
            let mut r = [S::load_u8(base.add(row0 - 4)); 8];
            for i in 0..8 {
                r[i] = S::load_u8(base.add(row0 + i * p - 4));
            }
            let t = S::transpose8x8(r);
            let mask = filter_mask8::<S>(
                limit, blimit, t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7],
            );
            let hev = hev8::<S>(thresh, t[2], t[3], t[4], t[5]);
            let (n1, n0, m0, m1) = normal_filter8::<S>(mask, hev, t[2], t[3], t[4], t[5]);
            let out = S::transpose8x8([t[0], t[1], n1, n0, m0, m1, t[6], t[7]]);
            for i in 0..8 {
                S::store_u8(base.add(row0 + i * p - 4), out[i]);
            }
        }
    }
}

pub(crate) fn mbloop_filter_horizontal_edge<S: Simd>(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    let base = s.as_mut_ptr();
    for chunk in 0..count {
        let idx = s_offset + chunk * 8;
        // SAFETY: as loop_filter_horizontal_edge.
        unsafe {
            let p3 = S::load_u8(base.add(idx - 4 * p));
            let p2 = S::load_u8(base.add(idx - 3 * p));
            let p1 = S::load_u8(base.add(idx - 2 * p));
            let p0 = S::load_u8(base.add(idx - p));
            let q0 = S::load_u8(base.add(idx));
            let q1 = S::load_u8(base.add(idx + p));
            let q2 = S::load_u8(base.add(idx + 2 * p));
            let q3 = S::load_u8(base.add(idx + 3 * p));
            let mask = filter_mask8::<S>(limit, blimit, p3, p2, p1, p0, q0, q1, q2, q3);
            let hev = hev8::<S>(thresh, p1, p0, q0, q1);
            let (r2, r1, r0, t0, t1, t2) = mbfilter8::<S>(mask, hev, p2, p1, p0, q0, q1, q2);
            S::store_u8(base.add(idx - 3 * p), r2);
            S::store_u8(base.add(idx - 2 * p), r1);
            S::store_u8(base.add(idx - p), r0);
            S::store_u8(base.add(idx), t0);
            S::store_u8(base.add(idx + p), t1);
            S::store_u8(base.add(idx + 2 * p), t2);
        }
    }
}

/// 0xFF where |p0-q0|*2 + |p1-q1|/2 <= blimit. Matches `vp8_simple_filter_mask`.
#[inline(always)]
fn simple_filter_mask8<S: Simd>(blimit: u8, p1: S::U8, p0: S::U8, q0: S::U8, q1: S::U8) -> S::U8 {
    let a = S::widen_u8(S::abd_u8(p0, q0));
    let b = S::widen_u8(S::abd_u8(p1, q1));
    let term = S::add_i16(S::shl_i16::<1>(a), S::shr_i16::<1>(b));
    let gt = S::narrow_mask(S::cgt_i16(term, S::splat_i16(blimit as i16)));
    S::not_u8(gt) // 0xFF where term <= blimit
}

/// Bit-exact twin of `vp8_simple_filter`: new (p0, q0). p1/q1 are read-only.
#[inline(always)]
fn simple_filter8<S: Simd>(
    mask: S::U8,
    p1: S::U8,
    p0: S::U8,
    q0: S::U8,
    q1: S::U8,
) -> (S::U8, S::U8) {
    let ps1 = S::to_signed_i16(p1);
    let ps0 = S::to_signed_i16(p0);
    let qs0 = S::to_signed_i16(q0);
    let qs1 = S::to_signed_i16(q1);
    let mask16 = S::widen_mask(mask);

    let mut fv = clamp_s8::<S>(S::sub_i16(ps1, qs1));
    let d = S::sub_i16(qs0, ps0);
    fv = clamp_s8::<S>(S::add_i16(fv, S::add_i16(S::add_i16(d, d), d)));
    fv = S::and_i16(fv, mask16);
    let f1 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fv, S::splat_i16(4))));
    let f2 = S::shr_i16::<3>(clamp_s8::<S>(S::add_i16(fv, S::splat_i16(3))));
    let nq0 = clamp_s8::<S>(S::sub_i16(qs0, f1));
    let np0 = clamp_s8::<S>(S::add_i16(ps0, f2));
    (S::from_signed_i16(np0), S::from_signed_i16(nq0))
}

/// NEON twin of `vp8_loop_filter_simple_horizontal_edge_safe` (16 px).
pub(crate) fn simple_horizontal_edge<S: Simd>(
    y: &mut [u8],
    y_offset: usize,
    stride: usize,
    blimit: u8,
) {
    let base = y.as_mut_ptr();
    for chunk in 0..2 {
        let idx = y_offset + chunk * 8;
        // SAFETY: reads/writes y[idx-2*stride .. idx+stride] over 8 columns —
        // the same elements the scalar loop indexes; in-bounds per the caller.
        unsafe {
            let p1 = S::load_u8(base.add(idx - 2 * stride));
            let p0 = S::load_u8(base.add(idx - stride));
            let q0 = S::load_u8(base.add(idx));
            let q1 = S::load_u8(base.add(idx + stride));
            let mask = simple_filter_mask8::<S>(blimit, p1, p0, q0, q1);
            let (np0, nq0) = simple_filter8::<S>(mask, p1, p0, q0, q1);
            S::store_u8(base.add(idx - stride), np0);
            S::store_u8(base.add(idx), nq0);
        }
    }
}

/// NEON twin of `vp8_loop_filter_simple_vertical_edge_safe` (16 px).
pub(crate) fn simple_vertical_edge<S: Simd>(
    y: &mut [u8],
    y_offset: usize,
    stride: usize,
    blimit: u8,
) {
    let base = y.as_mut_ptr();
    for chunk in 0..2 {
        let row0 = y_offset + chunk * 8 * stride;
        // SAFETY: 8 rows of 8 bytes at y[row*stride-2 .. +6]; columns -2..+1 are
        // the taps the scalar uses (the extra columns are written back unchanged).
        unsafe {
            let mut r = [S::load_u8(base.add(row0 - 2)); 8];
            for i in 0..8 {
                r[i] = S::load_u8(base.add(row0 + i * stride - 2));
            }
            let t = S::transpose8x8(r); // t[0..4] = p1,p0,q0,q1
            let mask = simple_filter_mask8::<S>(blimit, t[0], t[1], t[2], t[3]);
            let (np0, nq0) = simple_filter8::<S>(mask, t[0], t[1], t[2], t[3]);
            let out = S::transpose8x8([t[0], np0, nq0, t[3], t[4], t[5], t[6], t[7]]);
            for i in 0..8 {
                S::store_u8(base.add(row0 + i * stride - 2), out[i]);
            }
        }
    }
}

pub(crate) fn mbloop_filter_vertical_edge<S: Simd>(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: u8,
    limit: u8,
    thresh: u8,
    count: usize,
) {
    let base = s.as_mut_ptr();
    for chunk in 0..count {
        let row0 = s_offset + chunk * 8 * p;
        // SAFETY: as loop_filter_vertical_edge.
        unsafe {
            let mut r = [S::load_u8(base.add(row0 - 4)); 8];
            for i in 0..8 {
                r[i] = S::load_u8(base.add(row0 + i * p - 4));
            }
            let t = S::transpose8x8(r);
            let mask = filter_mask8::<S>(
                limit, blimit, t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7],
            );
            let hev = hev8::<S>(thresh, t[2], t[3], t[4], t[5]);
            let (r2, r1, r0, s0, s1, s2) =
                mbfilter8::<S>(mask, hev, t[1], t[2], t[3], t[4], t[5], t[6]);
            let out = S::transpose8x8([t[0], r2, r1, r0, s0, s1, s2, t[7]]);
            for i in 0..8 {
                S::store_u8(base.add(row0 + i * p - 4), out[i]);
            }
        }
    }
}
