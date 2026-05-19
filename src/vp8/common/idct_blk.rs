pub use crate::vp8::common::types::*;

pub fn vp8_dequant_idct_add_y_block_safe(
    q: &mut [i16; 256],
    dq: &[i16; 16],
    dst: &mut [u8],
    stride: i32,
    eobs: &[::core::ffi::c_char; 16],
) {
    let stride_sz = stride as usize;
    assert!(dst.len() >= 15 * stride_sz + 16, "dst buffer too small");

    for i in 0..4 {
        for j in 0..4 {
            let block_idx = i * 4 + j;
            let eob = eobs[block_idx];
            let q_offset = block_idx * 16;
            let dst_offset = i * 4 * stride_sz + j * 4;

            if eob > 1 {
                let dst_sub = &mut dst[dst_offset..];
                let q_sub: &mut [i16; 16] = (&mut q[q_offset..q_offset + 16]).try_into().unwrap();
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(q_sub, dq, dst_sub, stride);
            } else {
                let input_dc = q[q_offset] * dq[0];
                let dst_sub = &mut dst[dst_offset..];
                
                // Copy predictor to a safe temporary array to avoid borrow-checker conflicts.
                let mut pred = [0u8; 16];
                for r in 0..4 {
                    for c in 0..4 {
                        pred[r * 4 + c] = dst_sub[r * stride_sz + c];
                    }
                }
                
                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(
                    input_dc,
                    &pred,
                    4,
                    dst_sub,
                    stride,
                );
                
                q[q_offset] = 0;
                q[q_offset + 1] = 0;
            }
        }
    }
}

pub fn vp8_dequant_idct_add_uv_block_safe(
    q: &mut [i16; 128],
    dq: &[i16; 16],
    dst_u: &mut [u8],
    dst_v: &mut [u8],
    stride: i32,
    eobs: &[::core::ffi::c_char; 8],
) {
    let stride_sz = stride as usize;
    assert!(dst_u.len() >= 7 * stride_sz + 8, "dst_u buffer too small");
    assert!(dst_v.len() >= 7 * stride_sz + 8, "dst_v buffer too small");

    // U plane (first 4 blocks of 16 coefficients)
    for i in 0..2 {
        for j in 0..2 {
            let block_idx = i * 2 + j;
            let eob = eobs[block_idx];
            let q_offset = block_idx * 16;
            let dst_offset = i * 4 * stride_sz + j * 4;

            if eob > 1 {
                let dst_sub = &mut dst_u[dst_offset..];
                let q_sub: &mut [i16; 16] = (&mut q[q_offset..q_offset + 16]).try_into().unwrap();
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(q_sub, dq, dst_sub, stride);
            } else {
                let input_dc = q[q_offset] * dq[0];
                let dst_sub = &mut dst_u[dst_offset..];
                
                let mut pred = [0u8; 16];
                for r in 0..4 {
                    for c in 0..4 {
                        pred[r * 4 + c] = dst_sub[r * stride_sz + c];
                    }
                }
                
                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(
                    input_dc,
                    &pred,
                    4,
                    dst_sub,
                    stride,
                );
                
                q[q_offset] = 0;
                q[q_offset + 1] = 0;
            }
        }
    }

    // V plane (next 4 blocks of 16 coefficients)
    for i in 0..2 {
        for j in 0..2 {
            let block_idx = i * 2 + j;
            let eob = eobs[4 + block_idx];
            let q_offset = (4 + block_idx) * 16;
            let dst_offset = i * 4 * stride_sz + j * 4;

            if eob > 1 {
                let dst_sub = &mut dst_v[dst_offset..];
                let q_sub: &mut [i16; 16] = (&mut q[q_offset..q_offset + 16]).try_into().unwrap();
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(q_sub, dq, dst_sub, stride);
            } else {
                let input_dc = q[q_offset] * dq[0];
                let dst_sub = &mut dst_v[dst_offset..];
                
                let mut pred = [0u8; 16];
                for r in 0..4 {
                    for c in 0..4 {
                        pred[r * 4 + c] = dst_sub[r * stride_sz + c];
                    }
                }
                
                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(
                    input_dc,
                    &pred,
                    4,
                    dst_sub,
                    stride,
                );
                
                q[q_offset] = 0;
                q[q_offset + 1] = 0;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vp8_dequant_idct_add_y_block_c(
    q: *mut ::core::ffi::c_short,
    dq: *mut ::core::ffi::c_short,
    dst: *mut ::core::ffi::c_uchar,
    stride: ::core::ffi::c_int,
    eobs: *mut ::core::ffi::c_char,
) {
    if q.is_null() || dq.is_null() || dst.is_null() || eobs.is_null() {
        return;
    }
    unsafe {
        let q_ref = &mut *(q as *mut [i16; 256]);
        let dq_ref = &*(dq as *const [i16; 16]);
        let eobs_ref = &*(eobs as *const [::core::ffi::c_char; 16]);
        
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        
        vp8_dequant_idct_add_y_block_safe(q_ref, dq_ref, dst_slice, stride, eobs_ref);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vp8_dequant_idct_add_uv_block_c(
    q: *mut ::core::ffi::c_short,
    dq: *mut ::core::ffi::c_short,
    dst_u: *mut ::core::ffi::c_uchar,
    dst_v: *mut ::core::ffi::c_uchar,
    stride: ::core::ffi::c_int,
    eobs: *mut ::core::ffi::c_char,
) {
    if q.is_null() || dq.is_null() || dst_u.is_null() || dst_v.is_null() || eobs.is_null() {
        return;
    }
    unsafe {
        let q_ref = &mut *(q as *mut [i16; 128]);
        let dq_ref = &*(dq as *const [i16; 16]);
        let eobs_ref = &*(eobs as *const [::core::ffi::c_char; 8]);
        
        let dst_len = 7 * stride as usize + 8;
        let dst_u_slice = core::slice::from_raw_parts_mut(dst_u, dst_len);
        let dst_v_slice = core::slice::from_raw_parts_mut(dst_v, dst_len);
        
        vp8_dequant_idct_add_uv_block_safe(q_ref, dq_ref, dst_u_slice, dst_v_slice, stride, eobs_ref);
    }
}
