#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
pub type uc = u8;

fn vp8_signed_char_clamp(t: i32) -> i8 {
    t.clamp(-128, 127) as i8
}

fn vp8_filter_mask(
    limit: u8,
    blimit: u8,
    p3: u8,
    p2: u8,
    p1: u8,
    p0: u8,
    q0: u8,
    q1: u8,
    q2: u8,
    q3: u8,
) -> i8 {
    let mut mask: i32 = 0;
    mask |= ((p3 as i32 - p2 as i32).abs() > limit as i32) as i32;
    mask |= ((p2 as i32 - p1 as i32).abs() > limit as i32) as i32;
    mask |= ((p1 as i32 - p0 as i32).abs() > limit as i32) as i32;
    mask |= ((q1 as i32 - q0 as i32).abs() > limit as i32) as i32;
    mask |= ((q2 as i32 - q1 as i32).abs() > limit as i32) as i32;
    mask |= ((q3 as i32 - q2 as i32).abs() > limit as i32) as i32;
    mask |= (((p0 as i32 - q0 as i32).abs() * 2 + (p1 as i32 - q1 as i32).abs() / 2)
        > blimit as i32) as i32;
    (mask - 1) as i8
}

fn vp8_hevmask(thresh: u8, p1: u8, p0: u8, q0: u8, q1: u8) -> i8 {
    let mut hev: i32 = 0;
    hev |= -(((p1 as i32 - p0 as i32).abs() > thresh as i32) as i32);
    hev |= -(((q1 as i32 - q0 as i32).abs() > thresh as i32) as i32);
    hev as i8
}
fn vp8_filter(mask: i8, hev: u8, op1: &mut u8, op0: &mut u8, oq0: &mut u8, oq1: &mut u8) {
    let mut ps0: i8;
    let mut qs0: i8;
    let mut ps1: i8;
    let mut qs1: i8;
    let mut filter_value: i8;
    let mut filter1: i8;
    let mut filter2: i8;
    let mut u: i8;

    ps1 = (*op1 ^ 0x80) as i8;
    ps0 = (*op0 ^ 0x80) as i8;
    qs0 = (*oq0 ^ 0x80) as i8;
    qs1 = (*oq1 ^ 0x80) as i8;

    filter_value = vp8_signed_char_clamp(ps1 as i32 - qs1 as i32);
    filter_value = (filter_value as i32 & hev as i32) as i8;
    filter_value = vp8_signed_char_clamp(filter_value as i32 + 3 * (qs0 as i32 - ps0 as i32));
    filter_value = (filter_value as i32 & mask as i32) as i8;

    filter1 = vp8_signed_char_clamp(filter_value as i32 + 4);
    filter2 = vp8_signed_char_clamp(filter_value as i32 + 3);

    filter1 >>= 3;
    filter2 >>= 3;

    u = vp8_signed_char_clamp(qs0 as i32 - filter1 as i32);
    *oq0 = u as u8 ^ 0x80;

    u = vp8_signed_char_clamp(ps0 as i32 + filter2 as i32);
    *op0 = u as u8 ^ 0x80;

    filter_value = filter1;
    filter_value = (filter_value as i32 + 1) as i8;
    filter_value >>= 1;
    filter_value = (filter_value as i32 & !(hev as i32)) as i8;

    u = vp8_signed_char_clamp(qs1 as i32 - filter_value as i32);
    *oq1 = u as u8 ^ 0x80;

    u = vp8_signed_char_clamp(ps1 as i32 + filter_value as i32);
    *op1 = u as u8 ^ 0x80;
}
pub(crate) fn loop_filter_horizontal_edge_safe(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::loop_filter_horizontal_edge_neon(
            s, s_offset, p, blimit[0], limit[0], thresh[0], count,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    loop_filter_horizontal_edge_scalar(s, s_offset, p, blimit, limit, thresh, count);
}

// Chroma edges, U and V filtered together. On aarch64 they pack into one 16-wide
// s8 pass (libvpx's scheme); elsewhere they fall back to two 8-wide scalar calls.
#[allow(clippy::too_many_arguments)]
pub(crate) fn loop_filter_horizontal_edge_uv_safe(
    u: &mut [u8],
    u_off: usize,
    v: &mut [u8],
    v_off: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
) {
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::loop_filter_horizontal_edge_uv_neon(
        u, u_off, v, v_off, p, blimit[0], limit[0], thresh[0],
    );
    #[cfg(not(target_arch = "aarch64"))]
    {
        loop_filter_horizontal_edge_scalar(u, u_off, p, blimit, limit, thresh, 1);
        loop_filter_horizontal_edge_scalar(v, v_off, p, blimit, limit, thresh, 1);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn mbloop_filter_horizontal_edge_uv_safe(
    u: &mut [u8],
    u_off: usize,
    v: &mut [u8],
    v_off: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
) {
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::mbloop_filter_horizontal_edge_uv_neon(
        u, u_off, v, v_off, p, blimit[0], limit[0], thresh[0],
    );
    #[cfg(not(target_arch = "aarch64"))]
    {
        mbloop_filter_horizontal_edge_scalar(u, u_off, p, blimit, limit, thresh, 1);
        mbloop_filter_horizontal_edge_scalar(v, v_off, p, blimit, limit, thresh, 1);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn loop_filter_vertical_edge_uv_safe(
    u: &mut [u8],
    u_off: usize,
    v: &mut [u8],
    v_off: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
) {
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::loop_filter_vertical_edge_uv_neon(
        u, u_off, v, v_off, p, blimit[0], limit[0], thresh[0],
    );
    #[cfg(not(target_arch = "aarch64"))]
    {
        loop_filter_vertical_edge_scalar(u, u_off, p, blimit, limit, thresh, 1);
        loop_filter_vertical_edge_scalar(v, v_off, p, blimit, limit, thresh, 1);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn mbloop_filter_vertical_edge_uv_safe(
    u: &mut [u8],
    u_off: usize,
    v: &mut [u8],
    v_off: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
) {
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::mbloop_filter_vertical_edge_uv_neon(
        u, u_off, v, v_off, p, blimit[0], limit[0], thresh[0],
    );
    #[cfg(not(target_arch = "aarch64"))]
    {
        mbloop_filter_vertical_edge_scalar(u, u_off, p, blimit, limit, thresh, 1);
        mbloop_filter_vertical_edge_scalar(v, v_off, p, blimit, limit, thresh, 1);
    }
}

pub(crate) fn loop_filter_horizontal_edge_scalar(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    let mut hev: uc = 0;
    let mut mask: i8 = 0;
    let count_8 = count * 8;
    for i in 0..count_8 {
        let idx = s_offset + i;
        mask = vp8_filter_mask(
            limit[0],
            blimit[0],
            s[idx - 4 * p],
            s[idx - 3 * p],
            s[idx - 2 * p],
            s[idx - p],
            s[idx],
            s[idx + p],
            s[idx + 2 * p],
            s[idx + 3 * p],
        );
        hev = vp8_hevmask(thresh[0], s[idx - 2 * p], s[idx - p], s[idx], s[idx + p]) as uc;

        let mut op1_val = s[idx - 2 * p];
        let mut op0_val = s[idx - p];
        let mut oq0_val = s[idx];
        let mut oq1_val = s[idx + p];

        vp8_filter(
            mask,
            hev,
            &mut op1_val,
            &mut op0_val,
            &mut oq0_val,
            &mut oq1_val,
        );

        s[idx - 2 * p] = op1_val;
        s[idx - p] = op0_val;
        s[idx] = oq0_val;
        s[idx + p] = oq1_val;
    }
}
pub(crate) fn loop_filter_vertical_edge_safe(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::loop_filter_vertical_edge_neon(
            s, s_offset, p, blimit[0], limit[0], thresh[0], count,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    loop_filter_vertical_edge_scalar(s, s_offset, p, blimit, limit, thresh, count);
}

pub(crate) fn loop_filter_vertical_edge_scalar(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    let mut hev: uc = 0;
    let mut mask: i8 = 0;
    let count_8 = count * 8;
    for i in 0..count_8 {
        let idx = s_offset + i * p;
        mask = vp8_filter_mask(
            limit[0],
            blimit[0],
            s[idx - 4],
            s[idx - 3],
            s[idx - 2],
            s[idx - 1],
            s[idx],
            s[idx + 1],
            s[idx + 2],
            s[idx + 3],
        );
        hev = vp8_hevmask(thresh[0], s[idx - 2], s[idx - 1], s[idx], s[idx + 1]) as uc;

        let mut op1_val = s[idx - 2];
        let mut op0_val = s[idx - 1];
        let mut oq0_val = s[idx];
        let mut oq1_val = s[idx + 1];

        vp8_filter(
            mask,
            hev,
            &mut op1_val,
            &mut op0_val,
            &mut oq0_val,
            &mut oq1_val,
        );

        s[idx - 2] = op1_val;
        s[idx - 1] = op0_val;
        s[idx] = oq0_val;
        s[idx + 1] = oq1_val;
    }
}

fn vp8_mbfilter(
    mask: i8,
    hev: u8,
    op2: &mut u8,
    op1: &mut u8,
    op0: &mut u8,
    oq0: &mut u8,
    oq1: &mut u8,
    oq2: &mut u8,
) {
    let mut s: i8;
    let mut u: i8;
    let mut filter_value: i8;
    let mut filter1: i8;
    let mut filter2: i8;

    let mut ps2 = (*op2 ^ 0x80) as i8;
    let mut ps1 = (*op1 ^ 0x80) as i8;
    let mut ps0 = (*op0 ^ 0x80) as i8;
    let mut qs0 = (*oq0 ^ 0x80) as i8;
    let mut qs1 = (*oq1 ^ 0x80) as i8;
    let mut qs2 = (*oq2 ^ 0x80) as i8;

    filter_value = vp8_signed_char_clamp(ps1 as i32 - qs1 as i32);
    filter_value = vp8_signed_char_clamp(filter_value as i32 + 3 * (qs0 as i32 - ps0 as i32));
    filter_value = (filter_value as i32 & mask as i32) as i8;

    filter2 = filter_value;
    filter2 = (filter2 as i32 & hev as i32) as i8;

    filter1 = vp8_signed_char_clamp(filter2 as i32 + 4);
    filter2 = vp8_signed_char_clamp(filter2 as i32 + 3);

    filter1 >>= 3;
    filter2 >>= 3;

    qs0 = vp8_signed_char_clamp(qs0 as i32 - filter1 as i32);
    ps0 = vp8_signed_char_clamp(ps0 as i32 + filter2 as i32);

    filter_value = (filter_value as i32 & !(hev as i32)) as i8;
    filter2 = filter_value;

    u = vp8_signed_char_clamp((63 + filter2 as i32 * 27) >> 7);
    s = vp8_signed_char_clamp(qs0 as i32 - u as i32);
    *oq0 = s as u8 ^ 0x80;
    s = vp8_signed_char_clamp(ps0 as i32 + u as i32);
    *op0 = s as u8 ^ 0x80;

    u = vp8_signed_char_clamp((63 + filter2 as i32 * 18) >> 7);
    s = vp8_signed_char_clamp(qs1 as i32 - u as i32);
    *oq1 = s as u8 ^ 0x80;
    s = vp8_signed_char_clamp(ps1 as i32 + u as i32);
    *op1 = s as u8 ^ 0x80;

    u = vp8_signed_char_clamp((63 + filter2 as i32 * 9) >> 7);
    s = vp8_signed_char_clamp(qs2 as i32 - u as i32);
    *oq2 = s as u8 ^ 0x80;
    s = vp8_signed_char_clamp(ps2 as i32 + u as i32);
    *op2 = s as u8 ^ 0x80;
}
pub(crate) fn mbloop_filter_horizontal_edge_safe(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::mbloop_filter_horizontal_edge_neon(
            s, s_offset, p, blimit[0], limit[0], thresh[0], count,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    mbloop_filter_horizontal_edge_scalar(s, s_offset, p, blimit, limit, thresh, count);
}

pub(crate) fn mbloop_filter_horizontal_edge_scalar(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    let mut hev: uc = 0;
    let mut mask: i8 = 0;
    let count_8 = count * 8;
    for i in 0..count_8 {
        let idx = s_offset + i;
        mask = vp8_filter_mask(
            limit[0],
            blimit[0],
            s[idx - 4 * p],
            s[idx - 3 * p],
            s[idx - 2 * p],
            s[idx - p],
            s[idx],
            s[idx + p],
            s[idx + 2 * p],
            s[idx + 3 * p],
        );
        hev = vp8_hevmask(thresh[0], s[idx - 2 * p], s[idx - p], s[idx], s[idx + p]) as uc;

        let mut op2_val = s[idx - 3 * p];
        let mut op1_val = s[idx - 2 * p];
        let mut op0_val = s[idx - p];
        let mut oq0_val = s[idx];
        let mut oq1_val = s[idx + p];
        let mut oq2_val = s[idx + 2 * p];

        vp8_mbfilter(
            mask,
            hev,
            &mut op2_val,
            &mut op1_val,
            &mut op0_val,
            &mut oq0_val,
            &mut oq1_val,
            &mut oq2_val,
        );

        s[idx - 3 * p] = op2_val;
        s[idx - 2 * p] = op1_val;
        s[idx - p] = op0_val;
        s[idx] = oq0_val;
        s[idx + p] = oq1_val;
        s[idx + 2 * p] = oq2_val;
    }
}

pub(crate) fn mbloop_filter_horizontal_edge_split_safe(
    row_above: &mut [u8],
    row_current: &mut [u8],
    col_offset: usize,
    stride: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    let mut hev: uc = 0;
    let mut mask: i8 = 0;
    let count_8 = count * 8;
    let row_height = count_8; // row height is count * 8

    for i in 0..count_8 {
        let c = col_offset + i;

        let p1_idx = c + (row_height - 1) * stride;
        let p2_idx = c + (row_height - 2) * stride;
        let p3_idx = c + (row_height - 3) * stride;
        let p4_idx = c + (row_height - 4) * stride;

        let q0_idx = c;
        let q1_idx = c + stride;
        let q2_idx = c + 2 * stride;
        let q3_idx = c + 3 * stride;

        mask = vp8_filter_mask(
            limit[0],
            blimit[0],
            row_above[p4_idx],
            row_above[p3_idx],
            row_above[p2_idx],
            row_above[p1_idx],
            row_current[q0_idx],
            row_current[q1_idx],
            row_current[q2_idx],
            row_current[q3_idx],
        );
        hev = vp8_hevmask(
            thresh[0],
            row_above[p2_idx],
            row_above[p1_idx],
            row_current[q0_idx],
            row_current[q1_idx],
        ) as uc;

        let mut op2_val = row_above[p3_idx];
        let mut op1_val = row_above[p2_idx];
        let mut op0_val = row_above[p1_idx];
        let mut oq0_val = row_current[q0_idx];
        let mut oq1_val = row_current[q1_idx];
        let mut oq2_val = row_current[q2_idx];

        vp8_mbfilter(
            mask,
            hev,
            &mut op2_val,
            &mut op1_val,
            &mut op0_val,
            &mut oq0_val,
            &mut oq1_val,
            &mut oq2_val,
        );

        row_above[p3_idx] = op2_val;
        row_above[p2_idx] = op1_val;
        row_above[p1_idx] = op0_val;
        row_current[q0_idx] = oq0_val;
        row_current[q1_idx] = oq1_val;
        row_current[q2_idx] = oq2_val;
    }
}

pub(crate) fn mbloop_filter_vertical_edge_safe(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::mbloop_filter_vertical_edge_neon(
            s, s_offset, p, blimit[0], limit[0], thresh[0], count,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    mbloop_filter_vertical_edge_scalar(s, s_offset, p, blimit, limit, thresh, count);
}

pub(crate) fn mbloop_filter_vertical_edge_scalar(
    s: &mut [u8],
    s_offset: usize,
    p: usize,
    blimit: &[u8],
    limit: &[u8],
    thresh: &[u8],
    count: usize,
) {
    let mut hev: uc = 0;
    let mut mask: i8 = 0;
    let count_8 = count * 8;
    for i in 0..count_8 {
        let idx = s_offset + i * p;
        mask = vp8_filter_mask(
            limit[0],
            blimit[0],
            s[idx - 4],
            s[idx - 3],
            s[idx - 2],
            s[idx - 1],
            s[idx],
            s[idx + 1],
            s[idx + 2],
            s[idx + 3],
        );
        hev = vp8_hevmask(thresh[0], s[idx - 2], s[idx - 1], s[idx], s[idx + 1]) as uc;

        let mut op2_val = s[idx - 3];
        let mut op1_val = s[idx - 2];
        let mut op0_val = s[idx - 1];
        let mut oq0_val = s[idx];
        let mut oq1_val = s[idx + 1];
        let mut oq2_val = s[idx + 2];

        vp8_mbfilter(
            mask,
            hev,
            &mut op2_val,
            &mut op1_val,
            &mut op0_val,
            &mut oq0_val,
            &mut oq1_val,
            &mut oq2_val,
        );

        s[idx - 3] = op2_val;
        s[idx - 2] = op1_val;
        s[idx - 1] = op0_val;
        s[idx] = oq0_val;
        s[idx + 1] = oq1_val;
        s[idx + 2] = oq2_val;
    }
}
fn vp8_simple_filter_mask(blimit: u8, p1: u8, p0: u8, q0: u8, q1: u8) -> i8 {
    -(((p0 as i32 - q0 as i32).abs() * 2 + (p1 as i32 - q1 as i32).abs() / 2 <= blimit as i32)
        as i32) as i8
}

fn vp8_simple_filter_safe(mask: i8, op1: u8, op0: &mut u8, oq0: &mut u8, oq1: u8) {
    let mut filter_value: i8;

    let p1 = ((op1 as i8 as i32) ^ 0x80) as i8;
    let p0 = ((*op0 as i8 as i32) ^ 0x80) as i8;
    let q0 = ((*oq0 as i8 as i32) ^ 0x80) as i8;
    let q1 = ((oq1 as i8 as i32) ^ 0x80) as i8;
    let mut u: i8;
    filter_value = vp8_signed_char_clamp(p1 as i32 - q1 as i32);
    filter_value = vp8_signed_char_clamp(filter_value as i32 + 3 * (q0 as i32 - p0 as i32));
    filter_value = (filter_value as i32 & mask as i32) as i8;
    let filter1: i8 = vp8_signed_char_clamp(filter_value as i32 + 4);
    let filter1_shifted = (filter1 as i32 >> 3) as i8;
    u = vp8_signed_char_clamp(q0 as i32 - filter1_shifted as i32);
    *oq0 = (u as i32 ^ 0x80) as u8;
    let filter2: i8 = vp8_signed_char_clamp(filter_value as i32 + 3);
    let filter2_shifted = (filter2 as i32 >> 3) as i8;
    u = vp8_signed_char_clamp(p0 as i32 + filter2_shifted as i32);
    *op0 = (u as i32 ^ 0x80) as u8;
}

pub(crate) fn vp8_loop_filter_simple_horizontal_edge_safe(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::simple_horizontal_edge_neon(y, y_offset, y_stride, blimit);
    }
    #[cfg(not(target_arch = "aarch64"))]
    vp8_loop_filter_simple_horizontal_edge_scalar(y, y_offset, y_stride, blimit);
}

pub(crate) fn vp8_loop_filter_simple_horizontal_edge_scalar(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    let mut mask: i8;
    for i in 0..16 {
        let idx = y_offset + i;
        let p1_val = y[idx - 2 * y_stride];
        let mut p0_val = y[idx - y_stride];
        let mut q0_val = y[idx];
        let q1_val = y[idx + y_stride];

        mask = vp8_simple_filter_mask(blimit, p1_val, p0_val, q0_val, q1_val);

        vp8_simple_filter_safe(mask, p1_val, &mut p0_val, &mut q0_val, q1_val);

        y[idx - y_stride] = p0_val;
        y[idx] = q0_val;
    }
}

pub(crate) fn vp8_loop_filter_simple_horizontal_edge_split_safe(
    row_above: &mut [u8],
    row_current: &mut [u8],
    col_offset: usize,
    stride: usize,
    blimit: u8,
) {
    let mut mask: i8;
    for i in 0..16 {
        let c = col_offset + i;
        let p1_idx = c + 14 * stride;
        let p0_idx = c + 15 * stride;
        let q0_idx = c;
        let q1_idx = c + stride;

        let p1_val = row_above[p1_idx];
        let mut p0_val = row_above[p0_idx];
        let mut q0_val = row_current[q0_idx];
        let q1_val = row_current[q1_idx];

        mask = vp8_simple_filter_mask(blimit, p1_val, p0_val, q0_val, q1_val);

        vp8_simple_filter_safe(mask, p1_val, &mut p0_val, &mut q0_val, q1_val);

        row_above[p0_idx] = p0_val;
        row_current[q0_idx] = q0_val;
    }
}

pub(crate) fn vp8_loop_filter_simple_vertical_edge_safe(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::simple_vertical_edge_neon(y, y_offset, y_stride, blimit);
    }
    #[cfg(not(target_arch = "aarch64"))]
    vp8_loop_filter_simple_vertical_edge_scalar(y, y_offset, y_stride, blimit);
}

pub(crate) fn vp8_loop_filter_simple_vertical_edge_scalar(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    let mut mask: i8;
    for i in 0..16 {
        let idx = y_offset + i * y_stride;
        let p1_val = y[idx - 2];
        let mut p0_val = y[idx - 1];
        let mut q0_val = y[idx];
        let q1_val = y[idx + 1];

        mask = vp8_simple_filter_mask(blimit, p1_val, p0_val, q0_val, q1_val);

        vp8_simple_filter_safe(mask, p1_val, &mut p0_val, &mut q0_val, q1_val);

        y[idx - 1] = p0_val;
        y[idx] = q0_val;
    }
}

pub(crate) fn vp8_loop_filter_bhs_safe(y: &mut [u8], y_offset: usize, y_stride: usize, blimit: u8) {
    vp8_loop_filter_simple_horizontal_edge_safe(y, y_offset + 4 * y_stride, y_stride, blimit);
    vp8_loop_filter_simple_horizontal_edge_safe(y, y_offset + 8 * y_stride, y_stride, blimit);
    vp8_loop_filter_simple_horizontal_edge_safe(y, y_offset + 12 * y_stride, y_stride, blimit);
}

pub(crate) fn vp8_loop_filter_bvs_safe(y: &mut [u8], y_offset: usize, y_stride: usize, blimit: u8) {
    vp8_loop_filter_simple_vertical_edge_safe(y, y_offset + 4, y_stride, blimit);
    vp8_loop_filter_simple_vertical_edge_safe(y, y_offset + 8, y_stride, blimit);
    vp8_loop_filter_simple_vertical_edge_safe(y, y_offset + 12, y_stride, blimit);
}
