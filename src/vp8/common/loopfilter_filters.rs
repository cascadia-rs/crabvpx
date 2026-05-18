unsafe extern "Rust" {
    fn abs(_: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
pub type uc = u8;
unsafe fn vp8_signed_char_clamp(mut t: i32) -> i8 {
    t = if t < -(128 as i32) { -(128 as i32) } else { t };
    t = if t > 127 as i32 { 127 as i32 } else { t };
    t as i8
}
unsafe fn vp8_filter_mask(
    mut limit: uc,
    mut blimit: uc,
    mut p3: uc,
    mut p2: uc,
    mut p1: uc,
    mut p0: uc,
    mut q0: uc,
    mut q1: uc,
    mut q2: uc,
    mut q3: uc,
) -> i8 {
    unsafe {
        let mut mask: i8 = 0 as i8;
        mask = (mask as i32 | (abs(p3 as i32 - p2 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32 | (abs(p2 as i32 - p1 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32 | (abs(p1 as i32 - p0 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32 | (abs(q1 as i32 - q0 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32 | (abs(q2 as i32 - q1 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32 | (abs(q3 as i32 - q2 as i32) > limit as i32) as i32) as i8;
        mask = (mask as i32
            | (abs(p0 as i32 - q0 as i32) * 2 as i32 + abs(p1 as i32 - q1 as i32) / 2 as i32
                > blimit as i32) as i32) as i8;
        (mask as i32 - 1 as i32) as i8
    }
}
unsafe fn vp8_hevmask(mut thresh: uc, mut p1: uc, mut p0: uc, mut q0: uc, mut q1: uc) -> i8 {
    unsafe {
        let mut hev: i8 = 0 as i8;
        hev = (hev as i32 | ((abs(p1 as i32 - p0 as i32) > thresh as i32) as i32 * -(1 as i32)))
            as i8;
        hev = (hev as i32 | ((abs(q1 as i32 - q0 as i32) > thresh as i32) as i32 * -(1 as i32)))
            as i8;
        hev
    }
}
unsafe fn vp8_filter(
    mut mask: i8,
    mut hev: uc,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
) {
    unsafe {
        let mut ps0: i8 = 0;
        let mut qs0: i8 = 0;
        let mut ps1: i8 = 0;
        let mut qs1: i8 = 0;
        let mut filter_value: i8 = 0;
        let mut Filter1: i8 = 0;
        let mut Filter2: i8 = 0;
        let mut u: i8 = 0;
        ps1 = (*op1 as i32 ^ 0x80 as i32) as i8;
        ps0 = (*op0 as i32 ^ 0x80 as i32) as i8;
        qs0 = (*oq0 as i32 ^ 0x80 as i32) as i8;
        qs1 = (*oq1 as i32 ^ 0x80 as i32) as i8;
        filter_value = vp8_signed_char_clamp(ps1 as i32 - qs1 as i32);
        filter_value = (filter_value as i32 & hev as i32) as i8;
        filter_value =
            vp8_signed_char_clamp(filter_value as i32 + 3 as i32 * (qs0 as i32 - ps0 as i32));
        filter_value = (filter_value as i32 & mask as i32) as i8;
        Filter1 = vp8_signed_char_clamp(filter_value as i32 + 4 as i32);
        Filter2 = vp8_signed_char_clamp(filter_value as i32 + 3 as i32);
        Filter1 = (Filter1 as i32 >> 3 as i32) as i8;
        Filter2 = (Filter2 as i32 >> 3 as i32) as i8;
        u = vp8_signed_char_clamp(qs0 as i32 - Filter1 as i32);
        *oq0 = (u as i32 ^ 0x80 as i32) as uc;
        u = vp8_signed_char_clamp(ps0 as i32 + Filter2 as i32);
        *op0 = (u as i32 ^ 0x80 as i32) as uc;
        filter_value = Filter1;
        filter_value = (filter_value as i32 + 1 as i32) as i8;
        filter_value = (filter_value as i32 >> 1 as i32) as i8;
        filter_value = (filter_value as i32 & !(hev as i32)) as i8;
        u = vp8_signed_char_clamp(qs1 as i32 - filter_value as i32);
        *oq1 = (u as i32 ^ 0x80 as i32) as uc;
        u = vp8_signed_char_clamp(ps1 as i32 + filter_value as i32);
        *op1 = (u as i32 ^ 0x80 as i32) as uc;
    }
}
unsafe fn loop_filter_horizontal_edge_c(
    mut s: *mut u8,
    mut p: i32,
    mut blimit: *const u8,
    mut limit: *const u8,
    mut thresh: *const u8,
    mut count: i32,
) {
    unsafe {
        let mut hev: i32 = 0 as i32;
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as isize) as uc,
                *blimit.offset(0 as isize) as uc,
                *s.offset((-(4 as i32) * p) as isize) as uc,
                *s.offset((-(3 as i32) * p) as isize) as uc,
                *s.offset((-(2 as i32) * p) as isize) as uc,
                *s.offset((-(1 as i32) * p) as isize) as uc,
                *s.offset((0 as i32 * p) as isize) as uc,
                *s.offset((1 as i32 * p) as isize) as uc,
                *s.offset((2 as i32 * p) as isize) as uc,
                *s.offset((3 as i32 * p) as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as isize) as uc,
                *s.offset((-(2 as i32) * p) as isize) as uc,
                *s.offset((-(1 as i32) * p) as isize) as uc,
                *s.offset((0 as i32 * p) as isize) as uc,
                *s.offset((1 as i32 * p) as isize) as uc,
            ) as i32;
            vp8_filter(
                mask,
                hev as uc,
                s.offset(-((2 as i32 * p) as isize)),
                s.offset(-((1 as i32 * p) as isize)),
                s as *mut uc,
                s.offset((1 as i32 * p) as isize),
            );
            s = s.offset(1);
            i += 1;
            if !(i < count * 8 as i32) {
                break;
            }
        }
    }
}
unsafe fn loop_filter_vertical_edge_c(
    mut s: *mut u8,
    mut p: i32,
    mut blimit: *const u8,
    mut limit: *const u8,
    mut thresh: *const u8,
    mut count: i32,
) {
    unsafe {
        let mut hev: i32 = 0 as i32;
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as isize) as uc,
                *blimit.offset(0 as isize) as uc,
                *s.offset(-(4 as i32) as isize) as uc,
                *s.offset(-(3 as i32) as isize) as uc,
                *s.offset(-(2 as i32) as isize) as uc,
                *s.offset(-(1 as i32) as isize) as uc,
                *s.offset(0 as isize) as uc,
                *s.offset(1 as isize) as uc,
                *s.offset(2 as isize) as uc,
                *s.offset(3 as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as isize) as uc,
                *s.offset(-(2 as i32) as isize) as uc,
                *s.offset(-(1 as i32) as isize) as uc,
                *s.offset(0 as isize) as uc,
                *s.offset(1 as isize) as uc,
            ) as i32;
            vp8_filter(
                mask,
                hev as uc,
                s.offset(-(2 as isize)),
                s.offset(-(1 as isize)),
                s as *mut uc,
                s.offset(1 as isize),
            );
            s = s.offset(p as isize);
            i += 1;
            if !(i < count * 8 as i32) {
                break;
            }
        }
    }
}
unsafe fn vp8_mbfilter(
    mut mask: i8,
    mut hev: uc,
    mut op2: *mut uc,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
    mut oq2: *mut uc,
) {
    unsafe {
        let mut s: i8 = 0;
        let mut u: i8 = 0;
        let mut filter_value: i8 = 0;
        let mut Filter1: i8 = 0;
        let mut Filter2: i8 = 0;
        let mut ps2: i8 = (*op2 as i32 ^ 0x80 as i32) as i8;
        let mut ps1: i8 = (*op1 as i32 ^ 0x80 as i32) as i8;
        let mut ps0: i8 = (*op0 as i32 ^ 0x80 as i32) as i8;
        let mut qs0: i8 = (*oq0 as i32 ^ 0x80 as i32) as i8;
        let mut qs1: i8 = (*oq1 as i32 ^ 0x80 as i32) as i8;
        let mut qs2: i8 = (*oq2 as i32 ^ 0x80 as i32) as i8;
        filter_value = vp8_signed_char_clamp(ps1 as i32 - qs1 as i32);
        filter_value =
            vp8_signed_char_clamp(filter_value as i32 + 3 as i32 * (qs0 as i32 - ps0 as i32));
        filter_value = (filter_value as i32 & mask as i32) as i8;
        Filter2 = filter_value;
        Filter2 = (Filter2 as i32 & hev as i32) as i8;
        Filter1 = vp8_signed_char_clamp(Filter2 as i32 + 4 as i32);
        Filter2 = vp8_signed_char_clamp(Filter2 as i32 + 3 as i32);
        Filter1 = (Filter1 as i32 >> 3 as i32) as i8;
        Filter2 = (Filter2 as i32 >> 3 as i32) as i8;
        qs0 = vp8_signed_char_clamp(qs0 as i32 - Filter1 as i32);
        ps0 = vp8_signed_char_clamp(ps0 as i32 + Filter2 as i32);
        filter_value = (filter_value as i32 & !(hev as i32)) as i8;
        Filter2 = filter_value;
        u = vp8_signed_char_clamp((63 as i32 + Filter2 as i32 * 27 as i32) >> 7 as i32);
        s = vp8_signed_char_clamp(qs0 as i32 - u as i32);
        *oq0 = (s as i32 ^ 0x80 as i32) as uc;
        s = vp8_signed_char_clamp(ps0 as i32 + u as i32);
        *op0 = (s as i32 ^ 0x80 as i32) as uc;
        u = vp8_signed_char_clamp((63 as i32 + Filter2 as i32 * 18 as i32) >> 7 as i32);
        s = vp8_signed_char_clamp(qs1 as i32 - u as i32);
        *oq1 = (s as i32 ^ 0x80 as i32) as uc;
        s = vp8_signed_char_clamp(ps1 as i32 + u as i32);
        *op1 = (s as i32 ^ 0x80 as i32) as uc;
        u = vp8_signed_char_clamp((63 as i32 + Filter2 as i32 * 9 as i32) >> 7 as i32);
        s = vp8_signed_char_clamp(qs2 as i32 - u as i32);
        *oq2 = (s as i32 ^ 0x80 as i32) as uc;
        s = vp8_signed_char_clamp(ps2 as i32 + u as i32);
        *op2 = (s as i32 ^ 0x80 as i32) as uc;
    }
}
unsafe fn mbloop_filter_horizontal_edge_c(
    mut s: *mut u8,
    mut p: i32,
    mut blimit: *const u8,
    mut limit: *const u8,
    mut thresh: *const u8,
    mut count: i32,
) {
    unsafe {
        let mut hev: i8 = 0 as i8;
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as isize) as uc,
                *blimit.offset(0 as isize) as uc,
                *s.offset((-(4 as i32) * p) as isize) as uc,
                *s.offset((-(3 as i32) * p) as isize) as uc,
                *s.offset((-(2 as i32) * p) as isize) as uc,
                *s.offset((-(1 as i32) * p) as isize) as uc,
                *s.offset((0 as i32 * p) as isize) as uc,
                *s.offset((1 as i32 * p) as isize) as uc,
                *s.offset((2 as i32 * p) as isize) as uc,
                *s.offset((3 as i32 * p) as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as isize) as uc,
                *s.offset((-(2 as i32) * p) as isize) as uc,
                *s.offset((-(1 as i32) * p) as isize) as uc,
                *s.offset((0 as i32 * p) as isize) as uc,
                *s.offset((1 as i32 * p) as isize) as uc,
            );
            vp8_mbfilter(
                mask,
                hev as uc,
                s.offset(-((3 as i32 * p) as isize)),
                s.offset(-((2 as i32 * p) as isize)),
                s.offset(-((1 as i32 * p) as isize)),
                s as *mut uc,
                s.offset((1 as i32 * p) as isize),
                s.offset((2 as i32 * p) as isize),
            );
            s = s.offset(1);
            i += 1;
            if !(i < count * 8 as i32) {
                break;
            }
        }
    }
}
unsafe fn mbloop_filter_vertical_edge_c(
    mut s: *mut u8,
    mut p: i32,
    mut blimit: *const u8,
    mut limit: *const u8,
    mut thresh: *const u8,
    mut count: i32,
) {
    unsafe {
        let mut hev: i8 = 0 as i8;
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as isize) as uc,
                *blimit.offset(0 as isize) as uc,
                *s.offset(-(4 as i32) as isize) as uc,
                *s.offset(-(3 as i32) as isize) as uc,
                *s.offset(-(2 as i32) as isize) as uc,
                *s.offset(-(1 as i32) as isize) as uc,
                *s.offset(0 as isize) as uc,
                *s.offset(1 as isize) as uc,
                *s.offset(2 as isize) as uc,
                *s.offset(3 as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as isize) as uc,
                *s.offset(-(2 as i32) as isize) as uc,
                *s.offset(-(1 as i32) as isize) as uc,
                *s.offset(0 as isize) as uc,
                *s.offset(1 as isize) as uc,
            );
            vp8_mbfilter(
                mask,
                hev as uc,
                s.offset(-(3 as isize)),
                s.offset(-(2 as isize)),
                s.offset(-(1 as isize)),
                s as *mut uc,
                s.offset(1 as isize),
                s.offset(2 as isize),
            );
            s = s.offset(p as isize);
            i += 1;
            if !(i < count * 8 as i32) {
                break;
            }
        }
    }
}
unsafe fn vp8_simple_filter_mask(
    mut blimit: uc,
    mut p1: uc,
    mut p0: uc,
    mut q0: uc,
    mut q1: uc,
) -> i8 {
    unsafe {
        let mut mask: i8 = ((abs(p0 as i32 - q0 as i32) * 2 as i32
            + abs(p1 as i32 - q1 as i32) / 2 as i32
            <= blimit as i32) as i32
            * -(1 as i32)) as i8;
        mask
    }
}
unsafe fn vp8_simple_filter(
    mut mask: i8,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
) {
    unsafe {
        let mut filter_value: i8 = 0;
        let mut Filter1: i8 = 0;
        let mut Filter2: i8 = 0;
        let mut p1: i8 = (*op1 as i32 ^ 0x80 as i32) as i8;
        let mut p0: i8 = (*op0 as i32 ^ 0x80 as i32) as i8;
        let mut q0: i8 = (*oq0 as i32 ^ 0x80 as i32) as i8;
        let mut q1: i8 = (*oq1 as i32 ^ 0x80 as i32) as i8;
        let mut u: i8 = 0;
        filter_value = vp8_signed_char_clamp(p1 as i32 - q1 as i32);
        filter_value =
            vp8_signed_char_clamp(filter_value as i32 + 3 as i32 * (q0 as i32 - p0 as i32));
        filter_value = (filter_value as i32 & mask as i32) as i8;
        Filter1 = vp8_signed_char_clamp(filter_value as i32 + 4 as i32);
        Filter1 = (Filter1 as i32 >> 3 as i32) as i8;
        u = vp8_signed_char_clamp(q0 as i32 - Filter1 as i32);
        *oq0 = (u as i32 ^ 0x80 as i32) as uc;
        Filter2 = vp8_signed_char_clamp(filter_value as i32 + 3 as i32);
        Filter2 = (Filter2 as i32 >> 3 as i32) as i8;
        u = vp8_signed_char_clamp(p0 as i32 + Filter2 as i32);
        *op0 = (u as i32 ^ 0x80 as i32) as uc;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_simple_horizontal_edge_c(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    unsafe {
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_simple_filter_mask(
                *blimit.offset(0 as isize) as uc,
                *y_ptr.offset((-(2 as i32) * y_stride) as isize) as uc,
                *y_ptr.offset((-(1 as i32) * y_stride) as isize) as uc,
                *y_ptr.offset((0 as i32 * y_stride) as isize) as uc,
                *y_ptr.offset((1 as i32 * y_stride) as isize) as uc,
            );
            vp8_simple_filter(
                mask,
                y_ptr.offset(-((2 as i32 * y_stride) as isize)),
                y_ptr.offset(-((1 as i32 * y_stride) as isize)),
                y_ptr as *mut uc,
                y_ptr.offset((1 as i32 * y_stride) as isize),
            );
            y_ptr = y_ptr.offset(1);
            i += 1;
            if !(i < 16 as i32) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_simple_vertical_edge_c(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    unsafe {
        let mut mask: i8 = 0 as i8;
        let mut i: i32 = 0 as i32;
        loop {
            mask = vp8_simple_filter_mask(
                *blimit.offset(0 as isize) as uc,
                *y_ptr.offset(-(2 as i32) as isize) as uc,
                *y_ptr.offset(-(1 as i32) as isize) as uc,
                *y_ptr.offset(0 as isize) as uc,
                *y_ptr.offset(1 as isize) as uc,
            );
            vp8_simple_filter(
                mask,
                y_ptr.offset(-(2 as isize)),
                y_ptr.offset(-(1 as isize)),
                y_ptr as *mut uc,
                y_ptr.offset(1 as isize),
            );
            y_ptr = y_ptr.offset(y_stride as isize);
            i += 1;
            if !(i < 16 as i32) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_mbh_c(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        mbloop_filter_horizontal_edge_c(
            y_ptr,
            y_stride,
            (*lfi).mblim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        if !u_ptr.is_null() {
            mbloop_filter_horizontal_edge_c(
                u_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
        if !v_ptr.is_null() {
            mbloop_filter_horizontal_edge_c(
                v_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_mbv_c(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        mbloop_filter_vertical_edge_c(
            y_ptr,
            y_stride,
            (*lfi).mblim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        if !u_ptr.is_null() {
            mbloop_filter_vertical_edge_c(
                u_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
        if !v_ptr.is_null() {
            mbloop_filter_vertical_edge_c(
                v_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_bh_c(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        loop_filter_horizontal_edge_c(
            y_ptr.offset((4 as i32 * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        loop_filter_horizontal_edge_c(
            y_ptr.offset((8 as i32 * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        loop_filter_horizontal_edge_c(
            y_ptr.offset((12 as i32 * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        if !u_ptr.is_null() {
            loop_filter_horizontal_edge_c(
                u_ptr.offset((4 as i32 * uv_stride) as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
        if !v_ptr.is_null() {
            loop_filter_horizontal_edge_c(
                v_ptr.offset((4 as i32 * uv_stride) as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_bhs_c(mut y_ptr: *mut u8, mut y_stride: i32, mut blimit: *const u8) {
    unsafe {
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((4 as i32 * y_stride) as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((8 as i32 * y_stride) as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((12 as i32 * y_stride) as isize),
            y_stride,
            blimit,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_bv_c(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        loop_filter_vertical_edge_c(
            y_ptr.offset(4 as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        loop_filter_vertical_edge_c(
            y_ptr.offset(8 as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        loop_filter_vertical_edge_c(
            y_ptr.offset(12 as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as i32,
        );
        if !u_ptr.is_null() {
            loop_filter_vertical_edge_c(
                u_ptr.offset(4 as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
        if !v_ptr.is_null() {
            loop_filter_vertical_edge_c(
                v_ptr.offset(4 as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_bvs_c(mut y_ptr: *mut u8, mut y_stride: i32, mut blimit: *const u8) {
    unsafe {
        vp8_loop_filter_simple_vertical_edge_c(y_ptr.offset(4 as isize), y_stride, blimit);
        vp8_loop_filter_simple_vertical_edge_c(y_ptr.offset(8 as isize), y_stride, blimit);
        vp8_loop_filter_simple_vertical_edge_c(y_ptr.offset(12 as isize), y_stride, blimit);
    }
}
