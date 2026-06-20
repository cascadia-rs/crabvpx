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
        let temp2 = temp_output[i * 4 + 3] as i32
            + ((temp_output[i * 4 + 3] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let c1 = temp1 - temp2;

        let temp1 = temp_output[i * 4 + 1] as i32
            + ((temp_output[i * 4 + 1] as i32 * COSPI8SQRT2MINUS1) >> 16);
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

/// Add the DC-only inverse transform (a single rounded DC term) to the
/// predictor and clamp. Dispatches to the NEON kernel on aarch64 (bit-exact
/// with the scalar twin); scalar elsewhere.
pub fn vp8_dc_only_idct_add_safe(
    input_dc: i16,
    pred: &[u8],
    pred_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::vp8_dc_only_idct_add_neon(
            input_dc,
            pred,
            pred_stride,
            dst,
            dst_stride,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    vp8_dc_only_idct_add_scalar(input_dc, pred, pred_stride, dst, dst_stride);
}

pub fn vp8_dc_only_idct_add_scalar(
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

pub fn vp8_short_inv_walsh4x4_1_safe(input: &[i16], mb_dqcoeff: &mut [i16]) {
    let a1 = (input[0] as i32 + 3) >> 3;
    for i in 0..16 {
        mb_dqcoeff[i * 16] = a1 as i16;
    }
}

pub fn vp8_short_inv_walsh4x4_safe(input: &[i16; 16], mb_dqcoeff: &mut [i16]) {
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
