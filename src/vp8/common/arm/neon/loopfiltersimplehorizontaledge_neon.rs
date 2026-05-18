use std::arch::aarch64::*;
#[no_mangle]
pub unsafe fn vp8_loop_filter_bhs_neon(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    y_ptr = y_ptr.offset((y_stride * 4 as i32) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset((y_stride * 4 as i32) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset((y_stride * 4 as i32) as isize);
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_mbhs_neon(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    vp8_loop_filter_simple_horizontal_edge_neon(y_ptr, y_stride, blimit);
}
