extern "Rust" {
    fn vp8_loop_filter_horizontal_edge_y_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
    );
    fn vp8_loop_filter_vertical_edge_y_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
    );
    fn vp8_loop_filter_horizontal_edge_uv_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
        _: *mut u8,
    );
    fn vp8_loop_filter_vertical_edge_uv_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
        _: *mut u8,
    );
    fn vp8_mbloop_filter_horizontal_edge_y_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
    );
    fn vp8_mbloop_filter_vertical_edge_y_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
    );
    fn vp8_mbloop_filter_horizontal_edge_uv_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
        _: *mut u8,
    );
    fn vp8_mbloop_filter_vertical_edge_uv_neon(
        _: *mut u8,
        _: i32,
        _: u8,
        _: u8,
        _: u8,
        _: *mut u8,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterInfo {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
pub type LoopfilterUvNeon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
    *mut u8,
) -> ();
pub type LoopfilterYNeon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
) -> ();
#[no_mangle]
pub unsafe fn vp8_loop_filter_mbh_neon(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut LoopFilterInfo,
) {
    let mut mblim: u8 = *(*lfi).mblim;
    let mut lim: u8 = *(*lfi).lim;
    let mut hev_thr: u8 = *(*lfi).hev_thr;
    vp8_mbloop_filter_horizontal_edge_y_neon(y_ptr, y_stride, mblim, lim, hev_thr);
    if !u_ptr.is_null() {
        vp8_mbloop_filter_horizontal_edge_uv_neon(u_ptr, uv_stride, mblim, lim, hev_thr, v_ptr);
    }
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_mbv_neon(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut LoopFilterInfo,
) {
    let mut mblim: u8 = *(*lfi).mblim;
    let mut lim: u8 = *(*lfi).lim;
    let mut hev_thr: u8 = *(*lfi).hev_thr;
    vp8_mbloop_filter_vertical_edge_y_neon(y_ptr, y_stride, mblim, lim, hev_thr);
    if !u_ptr.is_null() {
        vp8_mbloop_filter_vertical_edge_uv_neon(u_ptr, uv_stride, mblim, lim, hev_thr, v_ptr);
    }
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_bh_neon(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut LoopFilterInfo,
) {
    let mut blim: u8 = *(*lfi).blim;
    let mut lim: u8 = *(*lfi).lim;
    let mut hev_thr: u8 = *(*lfi).hev_thr;
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((4 as i32 * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((8 as i32 * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((12 as i32 * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    if !u_ptr.is_null() {
        vp8_loop_filter_horizontal_edge_uv_neon(
            u_ptr.offset((4 as i32 * uv_stride) as isize),
            uv_stride,
            blim,
            lim,
            hev_thr,
            v_ptr.offset((4 as i32 * uv_stride) as isize),
        );
    }
}
#[no_mangle]
pub unsafe fn vp8_loop_filter_bv_neon(
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
    mut lfi: *mut LoopFilterInfo,
) {
    let mut blim: u8 = *(*lfi).blim;
    let mut lim: u8 = *(*lfi).lim;
    let mut hev_thr: u8 = *(*lfi).hev_thr;
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(4 as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(8 as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(12 as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    if !u_ptr.is_null() {
        vp8_loop_filter_vertical_edge_uv_neon(
            u_ptr.offset(4 as isize),
            uv_stride,
            blim,
            lim,
            hev_thr,
            v_ptr.offset(4 as isize),
        );
    }
}
