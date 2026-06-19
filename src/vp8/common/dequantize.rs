pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;

pub fn vp8_dequantize_b_safe(q: &[i16], dq: &mut [i16], DQC: &[i16]) {
    for i in 0..16 {
        dq[i] = (q[i] as i32 * DQC[i] as i32) as i16;
    }
}

pub fn vp8_dequant_idct_add_safe(
    input: &mut [i16; 16],
    dq: &[i16; 16],
    dest: &mut [u8],
    stride: i32,
) {
    // dequantize input in-place
    for i in 0..16 {
        input[i] = (dq[i] as i32 * input[i] as i32) as i16;
    }

    // copy pred from dest
    let mut pred = [0u8; 16];
    for r in 0..4 {
        for c in 0..4 {
            pred[r * 4 + c] = dest[r * stride as usize + c];
        }
    }

    // Call safe IDCT
    crate::vp8::common::idctllm::vp8_short_idct4x4llm_safe(input, pred, dest, stride);

    // Clear input
    input.fill(0);
}
