use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x8x2T {
    pub val: [uint8x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x8x4T {
    pub val: [uint8x8_t; 4],
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_bvs_neon(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    y_ptr = y_ptr.offset(4 as isize);
    vp8_loop_filter_simple_vertical_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset(4 as isize);
    vp8_loop_filter_simple_vertical_edge_neon(y_ptr, y_stride, blimit);
    y_ptr = y_ptr.offset(4 as isize);
    vp8_loop_filter_simple_vertical_edge_neon(y_ptr, y_stride, blimit);
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_mbvs_neon(
    mut y_ptr: *mut u8,
    mut y_stride: i32,
    mut blimit: *const u8,
) {
    vp8_loop_filter_simple_vertical_edge_neon(y_ptr, y_stride, blimit);
}
