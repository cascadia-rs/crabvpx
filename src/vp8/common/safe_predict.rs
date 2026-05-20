// Safe subpixel prediction wrappers
// This file provides safe Rust interfaces to the unsafe FFI predictor functions.
// It performs necessary bounds checks prior to calling the unsafe FFI.

use crate::vp8::common::types::vp8_subpix_fn_t;

unsafe extern "C" {
    pub fn vp8_bilinear_predict16x16_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_bilinear_predict8x8_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_bilinear_predict8x4_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_bilinear_predict4x4_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );

    pub fn vp8_sixtap_predict16x16_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_sixtap_predict8x8_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_sixtap_predict8x4_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
    pub fn vp8_sixtap_predict4x4_neon(
        src_ptr: *const u8,
        src_pixels_per_line: core::ffi::c_int,
        xoffset: core::ffi::c_int,
        yoffset: core::ffi::c_int,
        dst_ptr: *mut u8,
        dst_pitch: core::ffi::c_int,
    );
}

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
    assert!(src_len >= req_src_len, "Bilinear src out of bounds: len={}, req={}", src_len, req_src_len);
    
    // (H - 1) * dst_stride + W
    let req_dst_len = dst_offset + (h - 1) * dst_stride + w;
    assert!(dst_len >= req_dst_len, "Bilinear dst out of bounds: len={}, req={}", dst_len, req_dst_len);
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
    assert!(min_idx >= 0, "Sixtap src negative offset out of bounds: offset={}, min_idx={}", src_offset, min_idx);
    
    // max_idx = (H + 2) * stride + W + 2
    let req_src_len = src_offset + (h + 2) * src_stride + w + 3; // +3 because max_idx is size - 1
    assert!(src_len >= req_src_len, "Sixtap src out of bounds: len={}, req={}", src_len, req_src_len);
    
    // (H - 1) * dst_stride + W
    let req_dst_len = dst_offset + (h - 1) * dst_stride + w;
    assert!(dst_len >= req_dst_len, "Sixtap dst out of bounds: len={}, req={}", dst_len, req_dst_len);
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
    unsafe {
        vp8_bilinear_predict16x16_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_bilinear_predict8x8_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_bilinear_predict8x4_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_bilinear_predict4x4_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
        );
    }
}

// Safe Sixtap Wrappers

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
    unsafe {
        vp8_sixtap_predict16x16_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_sixtap_predict8x8_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_sixtap_predict8x4_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
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
    unsafe {
        vp8_sixtap_predict4x4_neon(
            src.as_ptr().add(src_offset),
            src_stride,
            xoffset,
            yoffset,
            dst.as_mut_ptr().add(dst_offset),
            dst_stride,
        );
    }
}
