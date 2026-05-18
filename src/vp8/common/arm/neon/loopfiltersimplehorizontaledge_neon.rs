use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint8_t = u8;
#[no_mangle]
pub unsafe fn vp8_loop_filter_bhs_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    y_ptr = y_ptr.offset((y_stride * 4 as ::core::ffi::c_int) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset((y_stride * 4 as ::core::ffi::c_int) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset((y_stride * 4 as ::core::ffi::c_int) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_mbhs_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut blimit: *const ::core::ffi::c_uchar,
) {
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
}
