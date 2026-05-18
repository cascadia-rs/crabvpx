const COSPI8SQRT2MINUS1: i32 = 20091;
const SINPI8SQRT2: i32 = 35468;

pub fn vp8_short_idct4x4llm_safe(
    input: &[i16; 16],
    pred: [u8; 16],
    dst: &mut [u8],
    dst_stride: i32,
) {
    let mut output = [0i16; 16];
    
    // First pass: process columns
    for i in 0..4 {
        let a1 = input[i] as i32 + input[i + 8] as i32;
        let b1 = input[i] as i32 - input[i + 8] as i32;
        
        let temp1 = (input[i + 4] as i32 * SINPI8SQRT2) >> 16;
        let temp2 = input[i + 12] as i32 + ((input[i + 12] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let c1 = temp1 - temp2;
        
        let temp1 = input[i + 4] as i32 + ((input[i + 4] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let temp2 = (input[i + 12] as i32 * SINPI8SQRT2) >> 16;
        let d1 = temp1 + temp2;
        
        output[i] = (a1 + d1) as i16;
        output[i + 12] = (a1 - d1) as i16;
        output[i + 4] = (b1 + c1) as i16;
        output[i + 8] = (b1 - c1) as i16;
    }
    
    // Second pass: process rows
    let temp_output = output;
    for i in 0..4 {
        let a1 = temp_output[i * 4] as i32 + temp_output[i * 4 + 2] as i32;
        let b1 = temp_output[i * 4] as i32 - temp_output[i * 4 + 2] as i32;
        
        let temp1 = (temp_output[i * 4 + 1] as i32 * SINPI8SQRT2) >> 16;
        let temp2 = temp_output[i * 4 + 3] as i32 + ((temp_output[i * 4 + 3] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let c1 = temp1 - temp2;
        
        let temp1 = temp_output[i * 4 + 1] as i32 + ((temp_output[i * 4 + 1] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let temp2 = (temp_output[i * 4 + 3] as i32 * SINPI8SQRT2) >> 16;
        let d1 = temp1 + temp2;
        
        output[i * 4] = ((a1 + d1 + 4) >> 3) as i16;
        output[i * 4 + 3] = ((a1 - d1 + 4) >> 3) as i16;
        output[i * 4 + 1] = ((b1 + c1 + 4) >> 3) as i16;
        output[i * 4 + 2] = ((b1 - c1 + 4) >> 3) as i16;
    }
    
    // Add predictor and clamp
    for r in 0..4 {
        for c in 0..4 {
            let a = output[r * 4 + c] as i32 + pred[r * 4 + c] as i32;
            let clamped = a.clamp(0, 255) as u8;
            dst[r * dst_stride as usize + c] = clamped;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_idct4x4llm_c(
    mut input: *mut ::core::ffi::c_short,
    mut pred_ptr: *mut ::core::ffi::c_uchar,
    mut pred_stride: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) {
    unsafe {
        let input_ref = &*(input as *const [i16; 16]);
        
        let mut pred = [0u8; 16];
        for r in 0..4 {
            for c in 0..4 {
                pred[r * 4 + c] = *pred_ptr.offset((r as i32 * pred_stride + c as i32) as isize);
            }
        }
        
        let dst_len = (3 * dst_stride + 4) as usize;
        let dst_slice = std::slice::from_raw_parts_mut(dst_ptr, dst_len);
        
        vp8_short_idct4x4llm_safe(input_ref, pred, dst_slice, dst_stride);
    }
}
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
