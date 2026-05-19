#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
pub type uc = ::core::ffi::c_uchar;

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
    mask |= (((p0 as i32 - q0 as i32).abs() * 2 + (p1 as i32 - q1 as i32).abs() / 2) > blimit as i32) as i32;
    (mask - 1) as i8
}

fn vp8_hevmask(
    thresh: u8,
    p1: u8,
    p0: u8,
    q0: u8,
    q1: u8,
) -> i8 {
    let mut hev: i32 = 0;
    hev |= (((p1 as i32 - p0 as i32).abs() > thresh as i32) as i32) * -1;
    hev |= (((q1 as i32 - q0 as i32).abs() > thresh as i32) as i32) * -1;
    hev as i8
}
fn vp8_filter(
    mask: i8,
    hev: u8,
    op1: &mut u8,
    op0: &mut u8,
    oq0: &mut u8,
    oq1: &mut u8,
) {
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
    filter_value = vp8_signed_char_clamp(
        filter_value as i32 + 3 * (qs0 as i32 - ps0 as i32),
    );
    filter_value = (filter_value as i32 & mask as i32) as i8;
    
    filter1 = vp8_signed_char_clamp(filter_value as i32 + 4);
    filter2 = vp8_signed_char_clamp(filter_value as i32 + 3);
    
    filter1 = (filter1 >> 3) as i8;
    filter2 = (filter2 >> 3) as i8;
    
    u = vp8_signed_char_clamp(qs0 as i32 - filter1 as i32);
    *oq0 = (u as u8 ^ 0x80);
    
    u = vp8_signed_char_clamp(ps0 as i32 + filter2 as i32);
    *op0 = (u as u8 ^ 0x80);
    
    filter_value = filter1;
    filter_value = (filter_value as i32 + 1) as i8;
    filter_value = (filter_value >> 1) as i8;
    filter_value = (filter_value as i32 & !(hev as i32)) as i8;
    
    u = vp8_signed_char_clamp(qs1 as i32 - filter_value as i32);
    *oq1 = (u as u8 ^ 0x80);
    
    u = vp8_signed_char_clamp(ps1 as i32 + filter_value as i32);
    *op1 = (u as u8 ^ 0x80);
}
fn loop_filter_horizontal_edge_safe(
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
        hev = vp8_hevmask(
            thresh[0],
            s[idx - 2 * p],
            s[idx - p],
            s[idx],
            s[idx + p],
        ) as uc;
        
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
fn loop_filter_vertical_edge_safe(
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
        hev = vp8_hevmask(
            thresh[0],
            s[idx - 2],
            s[idx - 1],
            s[idx],
            s[idx + 1],
        ) as uc;
        
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
    filter_value = vp8_signed_char_clamp(
        filter_value as i32 + 3 * (qs0 as i32 - ps0 as i32),
    );
    filter_value = (filter_value as i32 & mask as i32) as i8;
    
    filter2 = filter_value;
    filter2 = (filter2 as i32 & hev as i32) as i8;
    
    filter1 = vp8_signed_char_clamp(filter2 as i32 + 4);
    filter2 = vp8_signed_char_clamp(filter2 as i32 + 3);
    
    filter1 = (filter1 >> 3) as i8;
    filter2 = (filter2 >> 3) as i8;
    
    qs0 = vp8_signed_char_clamp(qs0 as i32 - filter1 as i32);
    ps0 = vp8_signed_char_clamp(ps0 as i32 + filter2 as i32);
    
    filter_value = (filter_value as i32 & !(hev as i32)) as i8;
    filter2 = filter_value;
    
    u = vp8_signed_char_clamp(
        (63 + filter2 as i32 * 27) >> 7,
    );
    s = vp8_signed_char_clamp(qs0 as i32 - u as i32);
    *oq0 = (s as u8 ^ 0x80);
    s = vp8_signed_char_clamp(ps0 as i32 + u as i32);
    *op0 = (s as u8 ^ 0x80);
    
    u = vp8_signed_char_clamp(
        (63 + filter2 as i32 * 18) >> 7,
    );
    s = vp8_signed_char_clamp(qs1 as i32 - u as i32);
    *oq1 = (s as u8 ^ 0x80);
    s = vp8_signed_char_clamp(ps1 as i32 + u as i32);
    *op1 = (s as u8 ^ 0x80);
    
    u = vp8_signed_char_clamp(
        (63 + filter2 as i32 * 9) >> 7,
    );
    s = vp8_signed_char_clamp(qs2 as i32 - u as i32);
    *oq2 = (s as u8 ^ 0x80);
    s = vp8_signed_char_clamp(ps2 as i32 + u as i32);
    *op2 = (s as u8 ^ 0x80);
}
fn mbloop_filter_horizontal_edge_safe(
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
        hev = vp8_hevmask(
            thresh[0],
            s[idx - 2 * p],
            s[idx - p],
            s[idx],
            s[idx + p],
        ) as uc;

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
unsafe extern "C" fn mbloop_filter_vertical_edge_c(
    mut s: *mut ::core::ffi::c_uchar,
    mut p: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
    mut limit: *const ::core::ffi::c_uchar,
    mut thresh: *const ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_int,
) { unsafe {
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
            &mut *s.offset(-3),
            &mut *s.offset(-2),
            &mut *s.offset(-1),
            &mut *s,
            &mut *s.offset(1),
            &mut *s.offset(2),
        );
        s = s.offset(p as isize);
        i += 1;
        if !(i < count * 8 as ::core::ffi::c_int) {
            break;
        }
    }
}}
fn vp8_simple_filter_mask(
    blimit: u8,
    p1: u8,
    p0: u8,
    q0: u8,
    q1: u8,
) -> i8 {
    (((p0 as i32 - q0 as i32).abs() * 2 + (p1 as i32 - q1 as i32).abs() / 2 <= blimit as i32) as i32 * -1) as i8
}
unsafe extern "C" fn vp8_simple_filter(
    mut mask: ::core::ffi::c_schar,
    mut op1: *mut uc,
    mut op0: *mut uc,
    mut oq0: *mut uc,
    mut oq1: *mut uc,
) { unsafe {
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
    filter_value =
        (filter_value as ::core::ffi::c_int & mask as ::core::ffi::c_int) as ::core::ffi::c_schar;
    Filter1 = vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 4 as ::core::ffi::c_int);
    Filter1 = (Filter1 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
    u = vp8_signed_char_clamp(q0 as ::core::ffi::c_int - Filter1 as ::core::ffi::c_int);
    *oq0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
    Filter2 = vp8_signed_char_clamp(filter_value as ::core::ffi::c_int + 3 as ::core::ffi::c_int);
    Filter2 = (Filter2 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_schar;
    u = vp8_signed_char_clamp(p0 as ::core::ffi::c_int + Filter2 as ::core::ffi::c_int);
    *op0 = (u as ::core::ffi::c_int ^ 0x80 as ::core::ffi::c_int) as uc;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_horizontal_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_vertical_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbh_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) { unsafe {
    let blimit_slice = core::slice::from_raw_parts((*lfi).mblim, 1);
    let limit_slice = core::slice::from_raw_parts((*lfi).lim, 1);
    let thresh_slice = core::slice::from_raw_parts((*lfi).hev_thr, 1);

    let y_stride_usize = y_stride as usize;
    let y_slice = core::slice::from_raw_parts_mut(y_ptr.offset(-4 * y_stride as isize), 8 * y_stride_usize);

    mbloop_filter_horizontal_edge_safe(
        y_slice,
        4 * y_stride_usize,
        y_stride_usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );

    let uv_stride_usize = uv_stride as usize;
    let uv_len = 8 * uv_stride_usize;
    if !u_ptr.is_null() {
        let u_slice = core::slice::from_raw_parts_mut(u_ptr.offset(-4 * uv_stride as isize), uv_len);
        mbloop_filter_horizontal_edge_safe(
            u_slice,
            4 * uv_stride_usize,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
    if !v_ptr.is_null() {
        let v_slice = core::slice::from_raw_parts_mut(v_ptr.offset(-4 * uv_stride as isize), uv_len);
        mbloop_filter_horizontal_edge_safe(
            v_slice,
            4 * uv_stride_usize,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbv_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) { unsafe {
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bh_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) { unsafe {
    let blimit_slice = core::slice::from_raw_parts((*lfi).blim, 1);
    let limit_slice = core::slice::from_raw_parts((*lfi).lim, 1);
    let thresh_slice = core::slice::from_raw_parts((*lfi).hev_thr, 1);

    let y_len = 16 * y_stride as usize;
    let y_slice = core::slice::from_raw_parts_mut(y_ptr, y_len);

    loop_filter_horizontal_edge_safe(
        y_slice,
        (4 * y_stride) as usize,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_horizontal_edge_safe(
        y_slice,
        (8 * y_stride) as usize,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_horizontal_edge_safe(
        y_slice,
        (12 * y_stride) as usize,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );

    let uv_len = 8 * uv_stride as usize;
    if !u_ptr.is_null() {
        let u_slice = core::slice::from_raw_parts_mut(u_ptr, uv_len);
        loop_filter_horizontal_edge_safe(
            u_slice,
            (4 * uv_stride) as usize,
            uv_stride as usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
    if !v_ptr.is_null() {
        let v_slice = core::slice::from_raw_parts_mut(v_ptr, uv_len);
        loop_filter_horizontal_edge_safe(
            v_slice,
            (4 * uv_stride) as usize,
            uv_stride as usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bhs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bv_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) { unsafe {
    let blimit_slice = core::slice::from_raw_parts((*lfi).blim, 1);
    let limit_slice = core::slice::from_raw_parts((*lfi).lim, 1);
    let thresh_slice = core::slice::from_raw_parts((*lfi).hev_thr, 1);

    let y_len = 16 * y_stride as usize;
    let y_slice = core::slice::from_raw_parts_mut(y_ptr, y_len);

    loop_filter_vertical_edge_safe(
        y_slice,
        4,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_vertical_edge_safe(
        y_slice,
        8,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_vertical_edge_safe(
        y_slice,
        12,
        y_stride as usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );

    let uv_len = 8 * uv_stride as usize;
    if !u_ptr.is_null() {
        let u_slice = core::slice::from_raw_parts_mut(u_ptr, uv_len);
        loop_filter_vertical_edge_safe(
            u_slice,
            4,
            uv_stride as usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
    if !v_ptr.is_null() {
        let v_slice = core::slice::from_raw_parts_mut(v_ptr, uv_len);
        loop_filter_vertical_edge_safe(
            v_slice,
            4,
            uv_stride as usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bvs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
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
}}
