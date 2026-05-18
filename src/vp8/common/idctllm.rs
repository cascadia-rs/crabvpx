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
pub fn vp8_dc_only_idct_add_safe(
    input_dc: i16,
    pred: &[u8],
    pred_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
) {
    let a1 = (input_dc as i32 + 4) >> 3;
    for r in 0..4 {
        for c in 0..4 {
            let pred_idx = r * pred_stride as usize + c;
            let dst_idx = r * dst_stride as usize + c;
            let a = a1 + pred[pred_idx] as i32;
            dst[dst_idx] = a.clamp(0, 255) as u8;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dc_only_idct_add_c(
    mut input_dc: ::core::ffi::c_short,
    mut pred_ptr: *mut ::core::ffi::c_uchar,
    mut pred_stride: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) {
    unsafe {
        let pred_len = (3 * pred_stride + 4) as usize;
        let pred_slice = std::slice::from_raw_parts(pred_ptr, pred_len);
        
        let dst_len = (3 * dst_stride + 4) as usize;
        let dst_slice = std::slice::from_raw_parts_mut(dst_ptr, dst_len);
        
        vp8_dc_only_idct_add_safe(input_dc, pred_slice, pred_stride, dst_slice, dst_stride);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_inv_walsh4x4_c(
    mut input: *mut ::core::ffi::c_short,
    mut mb_dqcoeff: *mut ::core::ffi::c_short,
) {
    unsafe {
        let input_ref = &*(input as *const [i16; 16]);
        // mb_dqcoeff is accessed up to index 15 * 16 = 240.
        // So we need a slice of at least 241 elements.
        // We use 256 as it is the size of contiguously allocated block coefficients.
        let mb_dqcoeff_slice = std::slice::from_raw_parts_mut(mb_dqcoeff, 256);
        vp8_short_inv_walsh4x4_safe(input_ref, mb_dqcoeff_slice);
    }
}
pub fn vp8_short_inv_walsh4x4_1_safe(
    input: &[i16],
    mb_dqcoeff: &mut [i16],
) {
    let a1 = (input[0] as i32 + 3) >> 3;
    for i in 0..16 {
        mb_dqcoeff[i * 16] = a1 as i16;
    }
}

pub fn vp8_short_inv_walsh4x4_safe(
    input: &[i16; 16],
    mb_dqcoeff: &mut [i16],
) {
    let mut output = [0i16; 16];
    
    // First pass: process columns
    for i in 0..4 {
        let a1 = input[i] as i32 + input[i + 12] as i32;
        let b1 = input[i + 4] as i32 + input[i + 8] as i32;
        let c1 = input[i + 4] as i32 - input[i + 8] as i32;
        let d1 = input[i] as i32 - input[i + 12] as i32;
        
        output[i] = (a1 + b1) as i16;
        output[i + 4] = (c1 + d1) as i16;
        output[i + 8] = (a1 - b1) as i16;
        output[i + 12] = (d1 - c1) as i16;
    }
    
    // Second pass: process rows
    let temp_output = output;
    for i in 0..4 {
        let row = i * 4;
        let a1 = temp_output[row] as i32 + temp_output[row + 3] as i32;
        let b1 = temp_output[row + 1] as i32 + temp_output[row + 2] as i32;
        let c1 = temp_output[row + 1] as i32 - temp_output[row + 2] as i32;
        let d1 = temp_output[row] as i32 - temp_output[row + 3] as i32;
        
        let a2 = a1 + b1;
        let b2 = c1 + d1;
        let c2 = a1 - b1;
        let d2 = d1 - c1;
        
        output[row] = ((a2 + 3) >> 3) as i16;
        output[row + 1] = ((b2 + 3) >> 3) as i16;
        output[row + 2] = ((c2 + 3) >> 3) as i16;
        output[row + 3] = ((d2 + 3) >> 3) as i16;
    }
    
    // Write to mb_dqcoeff
    for i in 0..16 {
        mb_dqcoeff[i * 16] = output[i];
    }
}


