static mut cospi8sqrt2minus1: ::core::ffi::c_int = 20091 as ::core::ffi::c_int;
static mut sinpi8sqrt2: ::core::ffi::c_int = 35468 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_idct4x4llm_c(
    mut input: *mut ::core::ffi::c_short,
    mut pred_ptr: *mut ::core::ffi::c_uchar,
    mut pred_stride: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    let mut b1: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut d1: ::core::ffi::c_int = 0;
    let mut output: [::core::ffi::c_short; 16] = [0; 16];
    let mut ip: *mut ::core::ffi::c_short = input;
    let mut op: *mut ::core::ffi::c_short = &raw mut output as *mut ::core::ffi::c_short;
    let mut temp1: ::core::ffi::c_int = 0;
    let mut temp2: ::core::ffi::c_int = 0;
    let mut shortpitch: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        b1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        temp1 = *ip.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * sinpi8sqrt2
            >> 16 as ::core::ffi::c_int;
        temp2 = *ip.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (*ip.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * cospi8sqrt2minus1
                >> 16 as ::core::ffi::c_int);
        c1 = temp1 - temp2;
        temp1 = *ip.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (*ip.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * cospi8sqrt2minus1
                >> 16 as ::core::ffi::c_int);
        temp2 = *ip.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * sinpi8sqrt2
            >> 16 as ::core::ffi::c_int;
        d1 = temp1 + temp2;
        *op.offset((shortpitch * 0 as ::core::ffi::c_int) as isize) =
            (a1 + d1) as ::core::ffi::c_short;
        *op.offset((shortpitch * 3 as ::core::ffi::c_int) as isize) =
            (a1 - d1) as ::core::ffi::c_short;
        *op.offset((shortpitch * 1 as ::core::ffi::c_int) as isize) =
            (b1 + c1) as ::core::ffi::c_short;
        *op.offset((shortpitch * 2 as ::core::ffi::c_int) as isize) =
            (b1 - c1) as ::core::ffi::c_short;
        ip = ip.offset(1);
        op = op.offset(1);
        i += 1;
    }
    ip = &raw mut output as *mut ::core::ffi::c_short;
    op = &raw mut output as *mut ::core::ffi::c_short;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        b1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        temp1 = *ip.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * sinpi8sqrt2
            >> 16 as ::core::ffi::c_int;
        temp2 = *ip.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (*ip.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * cospi8sqrt2minus1
                >> 16 as ::core::ffi::c_int);
        c1 = temp1 - temp2;
        temp1 = *ip.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (*ip.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * cospi8sqrt2minus1
                >> 16 as ::core::ffi::c_int);
        temp2 = *ip.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * sinpi8sqrt2
            >> 16 as ::core::ffi::c_int;
        d1 = temp1 + temp2;
        *op.offset(0 as ::core::ffi::c_int as isize) =
            (a1 + d1 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(3 as ::core::ffi::c_int as isize) =
            (a1 - d1 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(1 as ::core::ffi::c_int as isize) =
            (b1 + c1 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(2 as ::core::ffi::c_int as isize) =
            (b1 - c1 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        ip = ip.offset(shortpitch as isize);
        op = op.offset(shortpitch as isize);
        i += 1;
    }
    ip = &raw mut output as *mut ::core::ffi::c_short;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 4 as ::core::ffi::c_int {
            let mut a: ::core::ffi::c_int = *ip.offset(c as isize) as ::core::ffi::c_int
                + *pred_ptr.offset(c as isize) as ::core::ffi::c_int;
            if a < 0 as ::core::ffi::c_int {
                a = 0 as ::core::ffi::c_int;
            }
            if a > 255 as ::core::ffi::c_int {
                a = 255 as ::core::ffi::c_int;
            }
            *dst_ptr.offset(c as isize) = a as ::core::ffi::c_uchar;
            c += 1;
        }
        ip = ip.offset(4 as ::core::ffi::c_int as isize);
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        pred_ptr = pred_ptr.offset(pred_stride as isize);
        r += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dc_only_idct_add_c(
    mut input_dc: ::core::ffi::c_short,
    mut pred_ptr: *mut ::core::ffi::c_uchar,
    mut pred_stride: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) { unsafe {
    let mut a1: ::core::ffi::c_int =
        input_dc as ::core::ffi::c_int + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 4 as ::core::ffi::c_int {
            let mut a: ::core::ffi::c_int = a1 + *pred_ptr.offset(c as isize) as ::core::ffi::c_int;
            if a < 0 as ::core::ffi::c_int {
                a = 0 as ::core::ffi::c_int;
            }
            if a > 255 as ::core::ffi::c_int {
                a = 255 as ::core::ffi::c_int;
            }
            *dst_ptr.offset(c as isize) = a as ::core::ffi::c_uchar;
            c += 1;
        }
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        pred_ptr = pred_ptr.offset(pred_stride as isize);
        r += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_inv_walsh4x4_c(
    mut input: *mut ::core::ffi::c_short,
    mut mb_dqcoeff: *mut ::core::ffi::c_short,
) { unsafe {
    let mut output: [::core::ffi::c_short; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    let mut b1: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut d1: ::core::ffi::c_int = 0;
    let mut a2: ::core::ffi::c_int = 0;
    let mut b2: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut d2: ::core::ffi::c_int = 0;
    let mut ip: *mut ::core::ffi::c_short = input;
    let mut op: *mut ::core::ffi::c_short = &raw mut output as *mut ::core::ffi::c_short;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        b1 = *ip.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        c1 = *ip.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        d1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *op.offset(0 as ::core::ffi::c_int as isize) = (a1 + b1) as ::core::ffi::c_short;
        *op.offset(4 as ::core::ffi::c_int as isize) = (c1 + d1) as ::core::ffi::c_short;
        *op.offset(8 as ::core::ffi::c_int as isize) = (a1 - b1) as ::core::ffi::c_short;
        *op.offset(12 as ::core::ffi::c_int as isize) = (d1 - c1) as ::core::ffi::c_short;
        ip = ip.offset(1);
        op = op.offset(1);
        i += 1;
    }
    ip = &raw mut output as *mut ::core::ffi::c_short;
    op = &raw mut output as *mut ::core::ffi::c_short;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        b1 = *ip.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *ip.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        c1 = *ip.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        d1 = *ip.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *ip.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a2 = a1 + b1;
        b2 = c1 + d1;
        c2 = a1 - b1;
        d2 = d1 - c1;
        *op.offset(0 as ::core::ffi::c_int as isize) =
            (a2 + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(1 as ::core::ffi::c_int as isize) =
            (b2 + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(2 as ::core::ffi::c_int as isize) =
            (c2 + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *op.offset(3 as ::core::ffi::c_int as isize) =
            (d2 + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_short;
        ip = ip.offset(4 as ::core::ffi::c_int as isize);
        op = op.offset(4 as ::core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *mb_dqcoeff.offset((i * 16 as ::core::ffi::c_int) as isize) = output[i as usize];
        i += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_inv_walsh4x4_1_c(
    mut input: *mut ::core::ffi::c_short,
    mut mb_dqcoeff: *mut ::core::ffi::c_short,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    a1 = *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
        >> 3 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *mb_dqcoeff.offset((i * 16 as ::core::ffi::c_int) as isize) = a1 as ::core::ffi::c_short;
        i += 1;
    }
}}
