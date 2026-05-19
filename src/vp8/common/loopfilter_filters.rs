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
fn mbloop_filter_vertical_edge_safe(
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
fn vp8_simple_filter_mask(
    blimit: u8,
    p1: u8,
    p0: u8,
    q0: u8,
    q1: u8,
) -> i8 {
    (((p0 as i32 - q0 as i32).abs() * 2 + (p1 as i32 - q1 as i32).abs() / 2 <= blimit as i32) as i32 * -1) as i8
}

fn vp8_simple_filter_safe(
    mask: i8,
    op1: u8,
    op0: &mut u8,
    oq0: &mut u8,
    oq1: u8,
) {
    let mut filter_value: i8;
    let filter1: i8;
    let filter2: i8;
    let p1 = ((op1 as i8 as i32) ^ 0x80) as i8;
    let p0 = ((*op0 as i8 as i32) ^ 0x80) as i8;
    let q0 = ((*oq0 as i8 as i32) ^ 0x80) as i8;
    let q1 = ((oq1 as i8 as i32) ^ 0x80) as i8;
    let mut u: i8;
    filter_value = vp8_signed_char_clamp(p1 as i32 - q1 as i32);
    filter_value = vp8_signed_char_clamp(
        filter_value as i32
            + 3 * (q0 as i32 - p0 as i32),
    );
    filter_value = (filter_value as i32 & mask as i32) as i8;
    filter1 = vp8_signed_char_clamp(filter_value as i32 + 4);
    let filter1_shifted = (filter1 as i32 >> 3) as i8;
    u = vp8_signed_char_clamp(q0 as i32 - filter1_shifted as i32);
    *oq0 = (u as i32 ^ 0x80) as u8;
    filter2 = vp8_signed_char_clamp(filter_value as i32 + 3);
    let filter2_shifted = (filter2 as i32 >> 3) as i8;
    u = vp8_signed_char_clamp(p0 as i32 + filter2_shifted as i32);
    *op0 = (u as i32 ^ 0x80) as u8;
}

fn vp8_loop_filter_simple_horizontal_edge_safe(
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

        mask = vp8_simple_filter_mask(
            blimit,
            p1_val,
            p0_val,
            q0_val,
            q1_val,
        );

        vp8_simple_filter_safe(
            mask,
            p1_val,
            &mut p0_val,
            &mut q0_val,
            q1_val,
        );

        y[idx - y_stride] = p0_val;
        y[idx] = q0_val;
    }
}

fn vp8_loop_filter_simple_vertical_edge_safe(
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

        mask = vp8_simple_filter_mask(
            blimit,
            p1_val,
            p0_val,
            q0_val,
            q1_val,
        );

        vp8_simple_filter_safe(
            mask,
            p1_val,
            &mut p0_val,
            &mut q0_val,
            q1_val,
        );

        y[idx - 1] = p0_val;
        y[idx] = q0_val;
    }
}

fn vp8_loop_filter_bhs_safe(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    vp8_loop_filter_simple_horizontal_edge_safe(
        y,
        y_offset + 4 * y_stride,
        y_stride,
        blimit,
    );
    vp8_loop_filter_simple_horizontal_edge_safe(
        y,
        y_offset + 8 * y_stride,
        y_stride,
        blimit,
    );
    vp8_loop_filter_simple_horizontal_edge_safe(
        y,
        y_offset + 12 * y_stride,
        y_stride,
        blimit,
    );
}

fn vp8_loop_filter_bvs_safe(
    y: &mut [u8],
    y_offset: usize,
    y_stride: usize,
    blimit: u8,
) {
    vp8_loop_filter_simple_vertical_edge_safe(
        y,
        y_offset + 4,
        y_stride,
        blimit,
    );
    vp8_loop_filter_simple_vertical_edge_safe(
        y,
        y_offset + 8,
        y_stride,
        blimit,
    );
    vp8_loop_filter_simple_vertical_edge_safe(
        y,
        y_offset + 12,
        y_stride,
        blimit,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_horizontal_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
    let y_stride_usize = y_stride as usize;
    let y_slice = core::slice::from_raw_parts_mut(
        y_ptr.offset(-2 * y_stride as isize),
        3 * y_stride_usize + 16,
    );
    let blimit_val = *blimit;

    vp8_loop_filter_simple_horizontal_edge_safe(
        y_slice,
        2 * y_stride_usize,
        y_stride_usize,
        blimit_val,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_simple_vertical_edge_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) { unsafe {
    let y_stride_usize = y_stride as usize;
    let y_slice = core::slice::from_raw_parts_mut(
        y_ptr.offset(-2),
        15 * y_stride_usize + 4,
    );
    let blimit_val = *blimit;

    vp8_loop_filter_simple_vertical_edge_safe(
        y_slice,
        2,
        y_stride_usize,
        blimit_val,
    );
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
    let blimit_slice = core::slice::from_raw_parts((*lfi).mblim, 1);
    let limit_slice = core::slice::from_raw_parts((*lfi).lim, 1);
    let thresh_slice = core::slice::from_raw_parts((*lfi).hev_thr, 1);

    let y_stride_usize = y_stride as usize;
    let y_len = 15 * y_stride_usize + 8;
    let y_slice = core::slice::from_raw_parts_mut(y_ptr.offset(-4), y_len);
    mbloop_filter_vertical_edge_safe(
        y_slice,
        4,
        y_stride_usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );

    let uv_stride_usize = uv_stride as usize;
    let uv_len = 7 * uv_stride_usize + 8;
    if !u_ptr.is_null() {
        let u_slice = core::slice::from_raw_parts_mut(u_ptr.offset(-4), uv_len);
        mbloop_filter_vertical_edge_safe(
            u_slice,
            4,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
    if !v_ptr.is_null() {
        let v_slice = core::slice::from_raw_parts_mut(v_ptr.offset(-4), uv_len);
        mbloop_filter_vertical_edge_safe(
            v_slice,
            4,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
}}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_loop_filter_bh_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    if y_ptr.is_null() || lfi.is_null() {
        return;
    }
    let y_stride_usize = y_stride as usize;
    let uv_stride_usize = uv_stride as usize;

    let (blimit_slice, limit_slice, thresh_slice, y_slice, u_slice, v_slice) = unsafe {
        (
            core::slice::from_raw_parts((*lfi).blim, 1),
            core::slice::from_raw_parts((*lfi).lim, 1),
            core::slice::from_raw_parts((*lfi).hev_thr, 1),
            core::slice::from_raw_parts_mut(y_ptr, 16 * y_stride_usize),
            if u_ptr.is_null() {
                None
            } else {
                Some(core::slice::from_raw_parts_mut(u_ptr, 8 * uv_stride_usize))
            },
            if v_ptr.is_null() {
                None
            } else {
                Some(core::slice::from_raw_parts_mut(v_ptr, 8 * uv_stride_usize))
            },
        )
    };

    loop_filter_horizontal_edge_safe(
        y_slice,
        4 * y_stride_usize,
        y_stride_usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_horizontal_edge_safe(
        y_slice,
        8 * y_stride_usize,
        y_stride_usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );
    loop_filter_horizontal_edge_safe(
        y_slice,
        12 * y_stride_usize,
        y_stride_usize,
        blimit_slice,
        limit_slice,
        thresh_slice,
        2,
    );

    if let Some(u_slice) = u_slice {
        loop_filter_horizontal_edge_safe(
            u_slice,
            4 * uv_stride_usize,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
    if let Some(v_slice) = v_slice {
        loop_filter_horizontal_edge_safe(
            v_slice,
            4 * uv_stride_usize,
            uv_stride_usize,
            blimit_slice,
            limit_slice,
            thresh_slice,
            1,
        );
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_loop_filter_bhs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    if y_ptr.is_null() || blimit.is_null() {
        return;
    }
    let y_stride_usize = y_stride as usize;
    let (blimit_val, y_slice) = unsafe {
        (
            *blimit,
            core::slice::from_raw_parts_mut(y_ptr, 13 * y_stride_usize + 16),
        )
    };
    vp8_loop_filter_bhs_safe(y_slice, 0, y_stride_usize, blimit_val);
}
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
pub extern "C" fn vp8_loop_filter_bvs_c(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    if y_ptr.is_null() || blimit.is_null() {
        return;
    }
    let y_stride_usize = y_stride as usize;
    let (blimit_val, y_slice) = unsafe {
        (
            *blimit,
            core::slice::from_raw_parts_mut(y_ptr, 14 + 15 * y_stride_usize),
        )
    };
    vp8_loop_filter_bvs_safe(y_slice, 0, y_stride_usize, blimit_val);
}
