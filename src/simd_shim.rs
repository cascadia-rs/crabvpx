#![cfg(not(target_arch = "aarch64"))]

use core::ffi::{c_uchar, c_int, c_short, c_void, c_char};
pub type ptrdiff_t = isize;

unsafe extern "C" {
    fn vp8_bilinear_predict16x16_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_bilinear_predict4x4_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_bilinear_predict8x4_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_bilinear_predict8x8_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_copy_mem16x16_c(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int);
    fn vp8_copy_mem8x4_c(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int);
    fn vp8_copy_mem8x8_c(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int);
    fn vp8_dc_only_idct_add_c(input_dc: c_short, pred_ptr: *mut c_uchar, pred_stride: c_int, dst_ptr: *mut c_uchar, dst_stride: c_int);
    fn vp8_loop_filter_bh_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_bv_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_mbh_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_mbv_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_bhs_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);
    fn vp8_loop_filter_bvs_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);
    fn vp8_short_idct4x4llm_c(input: *mut c_short, pred_ptr: *mut c_uchar, pred_stride: c_int, dst_ptr: *mut c_uchar, dst_stride: c_int);
    fn vp8_short_inv_walsh4x4_c(input: *mut c_short, mb_dqcoeff: *mut c_short);
    fn vp8_sixtap_predict16x16_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_sixtap_predict4x4_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_sixtap_predict8x4_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_sixtap_predict8x8_c(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int);
    fn vp8_loop_filter_simple_horizontal_edge_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);
    fn vp8_loop_filter_simple_vertical_edge_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);



}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict16x16_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_bilinear_predict16x16_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict4x4_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_bilinear_predict4x4_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x4_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_bilinear_predict8x4_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x8_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_bilinear_predict8x8_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem16x16_neon(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int) {
    vp8_copy_mem16x16_c(src, src_stride, dst, dst_stride);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x4_neon(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int) {
    vp8_copy_mem8x4_c(src, src_stride, dst, dst_stride);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x8_neon(src: *mut c_uchar, src_stride: c_int, dst: *mut c_uchar, dst_stride: c_int) {
    vp8_copy_mem8x8_c(src, src_stride, dst, dst_stride);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dc_only_idct_add_neon(input_dc: c_short, pred_ptr: *mut c_uchar, pred_stride: c_int, dst_ptr: *mut c_uchar, dst_stride: c_int) {
    vp8_dc_only_idct_add_c(input_dc, pred_ptr, pred_stride, dst_ptr, dst_stride);
}




#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bh_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_bh_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bv_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_bv_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbh_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_mbh_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbv_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_mbv_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bhs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_bhs_c(y_ptr, y_stride, blimit);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bvs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_bvs_c(y_ptr, y_stride, blimit);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_idct4x4llm_neon(input: *mut c_short, pred_ptr: *mut c_uchar, pred_stride: c_int, dst_ptr: *mut c_uchar, dst_stride: c_int) {
    vp8_short_idct4x4llm_c(input, pred_ptr, pred_stride, dst_ptr, dst_stride);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_short_inv_walsh4x4_neon(input: *mut c_short, mb_dqcoeff: *mut c_short) {
    vp8_short_inv_walsh4x4_c(input, mb_dqcoeff);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict16x16_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_sixtap_predict16x16_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict4x4_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_sixtap_predict4x4_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x4_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_sixtap_predict8x4_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x8_neon(src_ptr: *mut c_uchar, src_pixels_per_line: c_int, xoffset: c_int, yoffset: c_int, dst_ptr: *mut c_uchar, dst_pitch: c_int) {
    vp8_sixtap_predict8x8_c(src_ptr, src_pixels_per_line, xoffset, yoffset, dst_ptr, dst_pitch);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbhs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_simple_horizontal_edge_c(y_ptr, y_stride, blimit);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbvs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_simple_vertical_edge_c(y_ptr, y_stride, blimit);
}
















