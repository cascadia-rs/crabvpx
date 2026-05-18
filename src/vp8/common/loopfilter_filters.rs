unsafe extern "C" {
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
pub type uc = ::core::ffi::c_uchar;
unsafe extern "C" fn vp8_signed_char_clamp(mut t: ::core::ffi::c_int) -> ::core::ffi::c_schar {
    t = if t < -(128 as ::core::ffi::c_int) {
        -(128 as ::core::ffi::c_int)
    } else {
        t
    };
    t = if t > 127 as ::core::ffi::c_int {
        127 as ::core::ffi::c_int
    } else {
        t
    };
    t as ::core::ffi::c_schar
}
unsafe extern "C" fn vp8_filter_mask(
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
) -> ::core::ffi::c_schar {
    unsafe {
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(p3 as ::core::ffi::c_int - p2 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(p2 as ::core::ffi::c_int - p1 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(p1 as ::core::ffi::c_int - p0 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(q1 as ::core::ffi::c_int - q0 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(q2 as ::core::ffi::c_int - q1 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(q3 as ::core::ffi::c_int - q2 as ::core::ffi::c_int)
                > limit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        mask = (mask as ::core::ffi::c_int
            | (abs(p0 as ::core::ffi::c_int - q0 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                + abs(p1 as ::core::ffi::c_int - q1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int
                > blimit as ::core::ffi::c_int) as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        (mask as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_schar
    }
}
unsafe extern "C" fn vp8_hevmask(
    mut thresh: uc,
    mut p1: uc,
    mut p0: uc,
    mut q0: uc,
    mut q1: uc,
) -> ::core::ffi::c_schar {
    unsafe {
        let mut hev: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        hev = (hev as ::core::ffi::c_int
            | ((abs(p1 as ::core::ffi::c_int - p0 as ::core::ffi::c_int)
                > thresh as ::core::ffi::c_int) as ::core::ffi::c_int
                * -(1 as ::core::ffi::c_int))) as ::core::ffi::c_schar;
        hev = (hev as ::core::ffi::c_int
            | ((abs(q1 as ::core::ffi::c_int - q0 as ::core::ffi::c_int)
                > thresh as ::core::ffi::c_int) as ::core::ffi::c_int
                * -(1 as ::core::ffi::c_int))) as ::core::ffi::c_schar;
        hev
    }
}
unsafe extern "C" fn vp8_filter(
    mut mask: ::core::ffi::c_schar,
    mut hev: uc,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
) {
    unsafe {
        let mut ps0: ::core::ffi::c_schar = 0;
        let mut qs0: ::core::ffi::c_schar = 0;
        let mut ps1: ::core::ffi::c_schar = 0;
        let mut qs1: ::core::ffi::c_schar = 0;
        let mut filter_value: ::core::ffi::c_schar = 0;
        let mut Filter1: ::core::ffi::c_schar = 0;
        let mut Filter2: ::core::ffi::c_schar = 0;
        let mut u: ::core::ffi::c_schar = 0;
        ps1 = (*op1 as ::core::ffi::c_schar as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        ps0 = (*op0 as ::core::ffi::c_schar as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        qs0 = (*oq0 as ::core::ffi::c_schar as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        qs1 = (*oq1 as ::core::ffi::c_schar as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        filter_value = vp8_signed_char_clamp(ps1 as ::core::ffi::c_int - qs1 as ::core::ffi::c_int);
        filter_value = (filter_value as ::core::ffi::c_int & hev as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        filter_value = vp8_signed_char_clamp(
            filter_value as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * (qs0 as ::core::ffi::c_int - ps0 as ::core::ffi::c_int),
        );
        filter_value = (filter_value as ::core::ffi::c_int & mask as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        Filter1 =
            vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 4 as ::core::ffi::c_int);
        Filter2 =
            vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 3 as ::core::ffi::c_int);
        Filter1 =
            (Filter1 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        Filter2 =
            (Filter2 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        u = vp8_signed_char_clamp(qs0 as ::core::ffi::c_int - Filter1 as ::core::ffi::c_int);
        *oq0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        u = vp8_signed_char_clamp(ps0 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int);
        *op0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        filter_value = Filter1;
        filter_value =
            (filter_value as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        filter_value =
            (filter_value as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        filter_value = (filter_value as ::core::ffi::c_int & !(hev as ::core::ffi::c_int))
            as ::core::ffi::c_schar;
        u = vp8_signed_char_clamp(qs1 as ::core::ffi::c_int - filter_value as ::core::ffi::c_int);
        *oq1 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        u = vp8_signed_char_clamp(ps1 as ::core::ffi::c_int + filter_value as ::core::ffi::c_int);
        *op1 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
    }
}
unsafe extern "C" fn loop_filter_horizontal_edge_c(
    mut s: *mut ::core::ffi::c_uchar,
    mut p: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
    mut limit: *const ::core::ffi::c_uchar,
    mut thresh: *const ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut hev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset((-(4 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(3 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(2 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(1 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((0 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((1 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((2 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((3 as ::core::ffi::c_int * p) as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset((-(2 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(1 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((0 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((1 as ::core::ffi::c_int * p) as isize) as uc,
            ) as ::core::ffi::c_int;
            vp8_filter(
                mask,
                hev as uc,
                s.offset(-((2 as ::core::ffi::c_int * p) as isize)),
                s.offset(-((1 as ::core::ffi::c_int * p) as isize)),
                s as *mut uc,
                s.offset((1 as ::core::ffi::c_int * p) as isize),
            );
            s = s.offset(1);
            i += 1;
            if !(i < count * 8 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn loop_filter_vertical_edge_c(
    mut s: *mut ::core::ffi::c_uchar,
    mut p: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
    mut limit: *const ::core::ffi::c_uchar,
    mut thresh: *const ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut hev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(-(4 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(3 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(2 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(1 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(1 as ::core::ffi::c_int as isize) as uc,
                *s.offset(2 as ::core::ffi::c_int as isize) as uc,
                *s.offset(3 as ::core::ffi::c_int as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(-(2 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(1 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(1 as ::core::ffi::c_int as isize) as uc,
            ) as ::core::ffi::c_int;
            vp8_filter(
                mask,
                hev as uc,
                s.offset(-(2 as ::core::ffi::c_int as isize)),
                s.offset(-(1 as ::core::ffi::c_int as isize)),
                s as *mut uc,
                s.offset(1 as ::core::ffi::c_int as isize),
            );
            s = s.offset(p as isize);
            i += 1;
            if !(i < count * 8 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn vp8_mbfilter(
    mut mask: ::core::ffi::c_schar,
    mut hev: uc,
    mut op2: *mut uc,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
    mut oq2: *mut uc,
) {
    unsafe {
        let mut s: ::core::ffi::c_schar = 0;
        let mut u: ::core::ffi::c_schar = 0;
        let mut filter_value: ::core::ffi::c_schar = 0;
        let mut Filter1: ::core::ffi::c_schar = 0;
        let mut Filter2: ::core::ffi::c_schar = 0;
        let mut ps2: ::core::ffi::c_schar = (*op2 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut ps1: ::core::ffi::c_schar = (*op1 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut ps0: ::core::ffi::c_schar = (*op0 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut qs0: ::core::ffi::c_schar = (*oq0 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut qs1: ::core::ffi::c_schar = (*oq1 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut qs2: ::core::ffi::c_schar = (*oq2 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        filter_value = vp8_signed_char_clamp(ps1 as ::core::ffi::c_int - qs1 as ::core::ffi::c_int);
        filter_value = vp8_signed_char_clamp(
            filter_value as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * (qs0 as ::core::ffi::c_int - ps0 as ::core::ffi::c_int),
        );
        filter_value = (filter_value as ::core::ffi::c_int & mask as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        Filter2 = filter_value;
        Filter2 =
            (Filter2 as ::core::ffi::c_int & hev as ::core::ffi::c_int) as ::core::ffi::c_schar;
        Filter1 = vp8_signed_char_clamp(Filter2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int);
        Filter2 = vp8_signed_char_clamp(Filter2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int);
        Filter1 =
            (Filter1 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        Filter2 =
            (Filter2 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        qs0 = vp8_signed_char_clamp(qs0 as ::core::ffi::c_int - Filter1 as ::core::ffi::c_int);
        ps0 = vp8_signed_char_clamp(ps0 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int);
        filter_value = (filter_value as ::core::ffi::c_int & !(hev as ::core::ffi::c_int))
            as ::core::ffi::c_schar;
        Filter2 = filter_value;
        u = vp8_signed_char_clamp(
            (63 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int * 27 as ::core::ffi::c_int)
                >> 7 as ::core::ffi::c_int,
        );
        s = vp8_signed_char_clamp(qs0 as ::core::ffi::c_int - u as ::core::ffi::c_int);
        *oq0 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        s = vp8_signed_char_clamp(ps0 as ::core::ffi::c_int + u as ::core::ffi::c_int);
        *op0 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        u = vp8_signed_char_clamp(
            (63 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int * 18 as ::core::ffi::c_int)
                >> 7 as ::core::ffi::c_int,
        );
        s = vp8_signed_char_clamp(qs1 as ::core::ffi::c_int - u as ::core::ffi::c_int);
        *oq1 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        s = vp8_signed_char_clamp(ps1 as ::core::ffi::c_int + u as ::core::ffi::c_int);
        *op1 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        u = vp8_signed_char_clamp(
            (63 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int * 9 as ::core::ffi::c_int)
                >> 7 as ::core::ffi::c_int,
        );
        s = vp8_signed_char_clamp(qs2 as ::core::ffi::c_int - u as ::core::ffi::c_int);
        *oq2 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        s = vp8_signed_char_clamp(ps2 as ::core::ffi::c_int + u as ::core::ffi::c_int);
        *op2 = (s as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
    }
}
unsafe extern "C" fn mbloop_filter_horizontal_edge_c(
    mut s: *mut ::core::ffi::c_uchar,
    mut p: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
    mut limit: *const ::core::ffi::c_uchar,
    mut thresh: *const ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut hev: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset((-(4 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(3 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(2 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(1 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((0 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((1 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((2 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((3 as ::core::ffi::c_int * p) as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset((-(2 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((-(1 as ::core::ffi::c_int) * p) as isize) as uc,
                *s.offset((0 as ::core::ffi::c_int * p) as isize) as uc,
                *s.offset((1 as ::core::ffi::c_int * p) as isize) as uc,
            );
            vp8_mbfilter(
                mask,
                hev as uc,
                s.offset(-((3 as ::core::ffi::c_int * p) as isize)),
                s.offset(-((2 as ::core::ffi::c_int * p) as isize)),
                s.offset(-((1 as ::core::ffi::c_int * p) as isize)),
                s as *mut uc,
                s.offset((1 as ::core::ffi::c_int * p) as isize),
                s.offset((2 as ::core::ffi::c_int * p) as isize),
            );
            s = s.offset(1);
            i += 1;
            if !(i < count * 8 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn mbloop_filter_vertical_edge_c(
    mut s: *mut ::core::ffi::c_uchar,
    mut p: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
    mut limit: *const ::core::ffi::c_uchar,
    mut thresh: *const ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut hev: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_filter_mask(
                *limit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(-(4 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(3 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(2 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(1 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(1 as ::core::ffi::c_int as isize) as uc,
                *s.offset(2 as ::core::ffi::c_int as isize) as uc,
                *s.offset(3 as ::core::ffi::c_int as isize) as uc,
            );
            hev = vp8_hevmask(
                *thresh.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(-(2 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(-(1 as ::core::ffi::c_int) as isize) as uc,
                *s.offset(0 as ::core::ffi::c_int as isize) as uc,
                *s.offset(1 as ::core::ffi::c_int as isize) as uc,
            );
            vp8_mbfilter(
                mask,
                hev as uc,
                s.offset(-(3 as ::core::ffi::c_int as isize)),
                s.offset(-(2 as ::core::ffi::c_int as isize)),
                s.offset(-(1 as ::core::ffi::c_int as isize)),
                s as *mut uc,
                s.offset(1 as ::core::ffi::c_int as isize),
                s.offset(2 as ::core::ffi::c_int as isize),
            );
            s = s.offset(p as isize);
            i += 1;
            if !(i < count * 8 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn vp8_simple_filter_mask(
    mut blimit: uc,
    mut p1: uc,
    mut p0: uc,
    mut q0: uc,
    mut q1: uc,
) -> ::core::ffi::c_schar {
    unsafe {
        let mut mask: ::core::ffi::c_schar =
            ((abs(p0 as ::core::ffi::c_int - q0 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                + abs(p1 as ::core::ffi::c_int - q1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int
                <= blimit as ::core::ffi::c_int) as ::core::ffi::c_int
                * -(1 as ::core::ffi::c_int)) as ::core::ffi::c_schar;
        mask
    }
}
unsafe extern "C" fn vp8_simple_filter(
    mut mask: ::core::ffi::c_schar,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
) {
    unsafe {
        let mut filter_value: ::core::ffi::c_schar = 0;
        let mut Filter1: ::core::ffi::c_schar = 0;
        let mut Filter2: ::core::ffi::c_schar = 0;
        let mut p1: ::core::ffi::c_schar = (*op1 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut p0: ::core::ffi::c_schar = (*op0 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut q0: ::core::ffi::c_schar = (*oq0 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut q1: ::core::ffi::c_schar = (*oq1 as ::core::ffi::c_schar as ::core::ffi::c_int
            ^ 0x80 as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        let mut u: ::core::ffi::c_schar = 0;
        filter_value = vp8_signed_char_clamp(p1 as ::core::ffi::c_int - q1 as ::core::ffi::c_int);
        filter_value = vp8_signed_char_clamp(
            filter_value as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * (q0 as ::core::ffi::c_int - p0 as ::core::ffi::c_int),
        );
        filter_value = (filter_value as ::core::ffi::c_int & mask as ::core::ffi::c_int)
            as ::core::ffi::c_schar;
        Filter1 =
            vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 4 as ::core::ffi::c_int);
        Filter1 =
            (Filter1 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        u = vp8_signed_char_clamp(q0 as ::core::ffi::c_int - Filter1 as ::core::ffi::c_int);
        *oq0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
        Filter2 =
            vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 3 as ::core::ffi::c_int);
        Filter2 =
            (Filter2 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
        u = vp8_signed_char_clamp(p0 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int);
        *op0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_horizontal_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    unsafe {
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_simple_filter_mask(
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *y_ptr.offset((-(2 as ::core::ffi::c_int) * y_stride) as isize) as uc,
                *y_ptr.offset((-(1 as ::core::ffi::c_int) * y_stride) as isize) as uc,
                *y_ptr.offset((0 as ::core::ffi::c_int * y_stride) as isize) as uc,
                *y_ptr.offset((1 as ::core::ffi::c_int * y_stride) as isize) as uc,
            );
            vp8_simple_filter(
                mask,
                y_ptr.offset(-((2 as ::core::ffi::c_int * y_stride) as isize)),
                y_ptr.offset(-((1 as ::core::ffi::c_int * y_stride) as isize)),
                y_ptr as *mut uc,
                y_ptr.offset((1 as ::core::ffi::c_int * y_stride) as isize),
            );
            y_ptr = y_ptr.offset(1);
            i += 1;
            if !(i < 16 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_vertical_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    unsafe {
        let mut mask: ::core::ffi::c_schar = 0 as ::core::ffi::c_schar;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            mask = vp8_simple_filter_mask(
                *blimit.offset(0 as ::core::ffi::c_int as isize) as uc,
                *y_ptr.offset(-(2 as ::core::ffi::c_int) as isize) as uc,
                *y_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as uc,
                *y_ptr.offset(0 as ::core::ffi::c_int as isize) as uc,
                *y_ptr.offset(1 as ::core::ffi::c_int as isize) as uc,
            );
            vp8_simple_filter(
                mask,
                y_ptr.offset(-(2 as ::core::ffi::c_int as isize)),
                y_ptr.offset(-(1 as ::core::ffi::c_int as isize)),
                y_ptr as *mut uc,
                y_ptr.offset(1 as ::core::ffi::c_int as isize),
            );
            y_ptr = y_ptr.offset(y_stride as isize);
            i += 1;
            if !(i < 16 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbh_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        mbloop_filter_horizontal_edge_c(
            y_ptr,
            y_stride,
            (*lfi).mblim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        if !u_ptr.is_null() {
            mbloop_filter_horizontal_edge_c(
                u_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
        if !v_ptr.is_null() {
            mbloop_filter_horizontal_edge_c(
                v_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbv_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        mbloop_filter_vertical_edge_c(
            y_ptr,
            y_stride,
            (*lfi).mblim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        if !u_ptr.is_null() {
            mbloop_filter_vertical_edge_c(
                u_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
        if !v_ptr.is_null() {
            mbloop_filter_vertical_edge_c(
                v_ptr,
                uv_stride,
                (*lfi).mblim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bh_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        loop_filter_horizontal_edge_c(
            y_ptr.offset((4 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        loop_filter_horizontal_edge_c(
            y_ptr.offset((8 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        loop_filter_horizontal_edge_c(
            y_ptr.offset((12 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        if !u_ptr.is_null() {
            loop_filter_horizontal_edge_c(
                u_ptr.offset((4 as ::core::ffi::c_int * uv_stride) as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
        if !v_ptr.is_null() {
            loop_filter_horizontal_edge_c(
                v_ptr.offset((4 as ::core::ffi::c_int * uv_stride) as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bhs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    unsafe {
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((4 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((8 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_horizontal_edge_c(
            y_ptr.offset((12 as ::core::ffi::c_int * y_stride) as isize),
            y_stride,
            blimit,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bv_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    unsafe {
        loop_filter_vertical_edge_c(
            y_ptr.offset(4 as ::core::ffi::c_int as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        loop_filter_vertical_edge_c(
            y_ptr.offset(8 as ::core::ffi::c_int as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        loop_filter_vertical_edge_c(
            y_ptr.offset(12 as ::core::ffi::c_int as isize),
            y_stride,
            (*lfi).blim,
            (*lfi).lim,
            (*lfi).hev_thr,
            2 as ::core::ffi::c_int,
        );
        if !u_ptr.is_null() {
            loop_filter_vertical_edge_c(
                u_ptr.offset(4 as ::core::ffi::c_int as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
        if !v_ptr.is_null() {
            loop_filter_vertical_edge_c(
                v_ptr.offset(4 as ::core::ffi::c_int as isize),
                uv_stride,
                (*lfi).blim,
                (*lfi).lim,
                (*lfi).hev_thr,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bvs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    unsafe {
        vp8_loop_filter_simple_vertical_edge_c(
            y_ptr.offset(4 as ::core::ffi::c_int as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_vertical_edge_c(
            y_ptr.offset(8 as ::core::ffi::c_int as isize),
            y_stride,
            blimit,
        );
        vp8_loop_filter_simple_vertical_edge_c(
            y_ptr.offset(12 as ::core::ffi::c_int as isize),
            y_stride,
            blimit,
        );
    }
}
