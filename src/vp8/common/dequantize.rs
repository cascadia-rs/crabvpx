pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;

pub fn vp8_dequantize_b_c(
    d: &mut BLOCKD,
    DQC: &[i16],
) {
    assert!(!d.dqcoeff.is_null(), "dqcoeff is null");
    assert!(!d.qcoeff.is_null(), "qcoeff is null");
    let dq = unsafe { std::slice::from_raw_parts_mut(d.dqcoeff, 16) };
    let q = unsafe { std::slice::from_raw_parts(d.qcoeff, 16) };
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dequant_idct_add_c(
    mut input: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dest: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
) {
    unsafe {
        let input_ref = &mut *(input as *mut [i16; 16]);
        let dq_ref = &*(dq as *const [i16; 16]);
        
        let dest_len = (3 * stride + 4) as usize;
        let dest_slice = std::slice::from_raw_parts_mut(dest, dest_len);
        
        vp8_dequant_idct_add_safe(input_ref, dq_ref, dest_slice, stride);
    }
}
