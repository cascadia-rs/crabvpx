//! Per-macroblock dequant+IDCT+add dispatch — port of `vp8/common/idct_blk.c`.
//!
//! Walks a macroblock's blocks, routing each to the full IDCT, the DC-only
//! fast path, or skip based on its eob; dispatches to the batched NEON drivers
//! on aarch64. Per-fn details below.

pub use crate::vp8::common::types::*;

/// Dequant + IDCT + add for the 16 luma blocks. Dispatches to the batched
/// 2-block NEON driver on aarch64 (bit-exact on real streams; see `simd::neon`);
/// scalar elsewhere.
pub fn vp8_dequant_idct_add_y_block_safe(
    q: &mut [i16; 256],
    dq: &[i16; 16],
    dst: &mut [u8],
    stride: i32,
    eobs: &[::core::ffi::c_char; 16],
) {
    let stride_sz = stride as usize;
    assert!(dst.len() >= 15 * stride_sz + 16, "dst buffer too small");
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::vp8_dequant_idct_add_y_block_neon(q, dq, dst, stride_sz, eobs);
    #[cfg(not(target_arch = "aarch64"))]
    vp8_dequant_idct_add_y_block_scalar(q, dq, dst, stride, eobs);
}

#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub fn vp8_dequant_idct_add_y_block_scalar(
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
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(
                    q_sub, dq, dst_sub, stride,
                );
            } else {
                // Wrap to match libvpx's truncation of the int product to `short`.
                let input_dc = q[q_offset].wrapping_mul(dq[0]);
                let dst_sub = &mut dst[dst_offset..];

                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(input_dc, dst_sub, stride);

                q[q_offset] = 0;
                q[q_offset + 1] = 0;
            }
        }
    }
}

/// Dequant + IDCT + add for the 8 chroma blocks. Dispatches to the batched
/// 2-block NEON driver on aarch64; scalar elsewhere.
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
    #[cfg(target_arch = "aarch64")]
    crate::vp8::common::simd::neon::vp8_dequant_idct_add_uv_block_neon(
        q, dq, dst_u, dst_v, stride_sz, eobs,
    );
    #[cfg(not(target_arch = "aarch64"))]
    vp8_dequant_idct_add_uv_block_scalar(q, dq, dst_u, dst_v, stride, eobs);
}

#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub fn vp8_dequant_idct_add_uv_block_scalar(
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
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(
                    q_sub, dq, dst_sub, stride,
                );
            } else {
                // Wrap to match libvpx's truncation of the int product to `short`.
                let input_dc = q[q_offset].wrapping_mul(dq[0]);
                let dst_sub = &mut dst_u[dst_offset..];

                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(input_dc, dst_sub, stride);

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
                crate::vp8::common::dequantize::vp8_dequant_idct_add_safe(
                    q_sub, dq, dst_sub, stride,
                );
            } else {
                // Wrap to match libvpx's truncation of the int product to `short`.
                let input_dc = q[q_offset].wrapping_mul(dq[0]);
                let dst_sub = &mut dst_v[dst_offset..];

                crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe(input_dc, dst_sub, stride);

                q[q_offset] = 0;
                q[q_offset + 1] = 0;
            }
        }
    }
}
