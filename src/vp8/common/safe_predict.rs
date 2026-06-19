// Safe subpixel prediction wrappers
// This file provides safe Rust interfaces to the unsafe FFI predictor functions.
// It performs necessary bounds checks prior to calling the unsafe FFI.

// Helper macros or functions for assertions

#[inline(always)]
fn assert_bilinear_bounds(
    src_len: usize,
    src_offset: usize,
    src_stride: usize,
    w: usize,
    h: usize,
    dst_len: usize,
    dst_offset: usize,
    dst_stride: usize,
) {
    // H * stride + W + 1
    let req_src_len = src_offset + h * src_stride + w + 1;
    assert!(
        src_len >= req_src_len,
        "Bilinear src out of bounds: len={}, req={}",
        src_len,
        req_src_len
    );

    // (H - 1) * dst_stride + W
    let req_dst_len = dst_offset + (h - 1) * dst_stride + w;
    assert!(
        dst_len >= req_dst_len,
        "Bilinear dst out of bounds: len={}, req={}",
        dst_len,
        req_dst_len
    );
}

#[inline(always)]
fn assert_sixtap_bounds(
    src_len: usize,
    src_offset: usize,
    src_stride: usize,
    w: usize,
    h: usize,
    dst_len: usize,
    dst_offset: usize,
    dst_stride: usize,
) {
    // min_idx = -2 * stride - 2
    let min_idx = src_offset as isize - 2 * src_stride as isize - 2;
    assert!(
        min_idx >= 0,
        "Sixtap src negative offset out of bounds: offset={}, min_idx={}",
        src_offset,
        min_idx
    );

    // max_idx = (H + 2) * stride + W + 2
    let req_src_len = src_offset + (h + 2) * src_stride + w + 3; // +3 because max_idx is size - 1
    assert!(
        src_len >= req_src_len,
        "Sixtap src out of bounds: len={}, req={}",
        src_len,
        req_src_len
    );

    // (H - 1) * dst_stride + W
    let req_dst_len = dst_offset + (h - 1) * dst_stride + w;
    assert!(
        dst_len >= req_dst_len,
        "Sixtap dst out of bounds: len={}, req={}",
        dst_len,
        req_dst_len
    );
}

// Safe Bilinear Wrappers

pub fn safe_vp8_bilinear_predict16x16_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_bilinear_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        16,
        16,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_bilinear_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_bilinear_filters[yoffset as usize];

        let src_slice = &src[src_offset..];
        let dst_slice = &mut dst[dst_offset..];

        crate::vp8::common::filter::filter_block2d_bil_safe(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            16,
            16,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_bilinear_predict8x8_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_bilinear_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        8,
        8,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_bilinear_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_bilinear_filters[yoffset as usize];

        let src_slice = &src[src_offset..];
        let dst_slice = &mut dst[dst_offset..];

        crate::vp8::common::filter::filter_block2d_bil_safe(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            8,
            8,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_bilinear_predict8x4_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_bilinear_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        8,
        4,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_bilinear_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_bilinear_filters[yoffset as usize];

        let src_slice = &src[src_offset..];
        let dst_slice = &mut dst[dst_offset..];

        crate::vp8::common::filter::filter_block2d_bil_safe(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            8,
            4,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_bilinear_predict4x4_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_bilinear_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        4,
        4,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_bilinear_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_bilinear_filters[yoffset as usize];

        let src_slice = &src[src_offset..];
        let dst_slice = &mut dst[dst_offset..];

        crate::vp8::common::filter::filter_block2d_bil_safe(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            4,
            4,
            h_filter,
            v_filter,
        );
    }
}

// Safe Sixtap Wrappers

/// Dispatch the 6-tap sub-pixel filter to the NEON kernel on aarch64, falling
/// back to the scalar twin elsewhere. Both are bit-exact (gated by `neon`'s
/// unit test and the differential suite).
#[inline]
fn sixtap_dispatch(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    h_filter: &[i16; 6],
    v_filter: &[i16; 6],
) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::filter_block2d_sixtap_neon(
            src, src_stride, dst, dst_pitch, width, height, h_filter, v_filter,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        crate::vp8::common::filter::filter_block2d_sixtap_safe(
            src, src_stride, dst, dst_pitch, width, height, h_filter, v_filter,
        );
    }
}

pub fn safe_vp8_sixtap_predict16x16_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_sixtap_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        16,
        16,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[yoffset as usize];

        let offset = src_offset as isize - 2 * stride as isize - 2;
        let src_slice = &src[offset as usize..];
        let dst_slice = &mut dst[dst_offset..];

        sixtap_dispatch(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            16,
            16,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_sixtap_predict8x8_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_sixtap_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        8,
        8,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[yoffset as usize];

        let offset = src_offset as isize - 2 * stride as isize - 2;
        let src_slice = &src[offset as usize..];
        let dst_slice = &mut dst[dst_offset..];

        sixtap_dispatch(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            8,
            8,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_sixtap_predict8x4_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_sixtap_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        8,
        4,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[yoffset as usize];

        let offset = src_offset as isize - 2 * stride as isize - 2;
        let src_slice = &src[offset as usize..];
        let dst_slice = &mut dst[dst_offset..];

        sixtap_dispatch(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            8,
            4,
            h_filter,
            v_filter,
        );
    }
}

pub fn safe_vp8_sixtap_predict4x4_neon(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    assert_sixtap_bounds(
        src.len(),
        src_offset,
        src_stride as usize,
        4,
        4,
        dst.len(),
        dst_offset,
        dst_stride as usize,
    );
    {
        let stride = src_stride as usize;
        let dst_stride_usize = dst_stride as usize;
        let h_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &crate::vp8::common::filter::vp8_sub_pel_filters[yoffset as usize];

        let offset = src_offset as isize - 2 * stride as isize - 2;
        let src_slice = &src[offset as usize..];
        let dst_slice = &mut dst[dst_offset..];

        sixtap_dispatch(
            src_slice,
            stride,
            dst_slice,
            dst_stride_usize,
            4,
            4,
            h_filter,
            v_filter,
        );
    }
}
