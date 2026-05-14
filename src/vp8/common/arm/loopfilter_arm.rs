extern "C" {
    fn vp8_loop_filter_horizontal_edge_y_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_vertical_edge_y_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_horizontal_edge_uv_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_vertical_edge_uv_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_uchar,
    );
    fn vp8_mbloop_filter_horizontal_edge_y_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
    );
    fn vp8_mbloop_filter_vertical_edge_y_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
    );
    fn vp8_mbloop_filter_horizontal_edge_uv_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_uchar,
    );
    fn vp8_mbloop_filter_vertical_edge_uv_neon(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_uchar,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
pub type loopfilter_uv_neon = unsafe extern "C" fn(
    *mut ::core::ffi::c_uchar,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    *mut ::core::ffi::c_uchar,
) -> ();
pub type loopfilter_y_neon = unsafe extern "C" fn(
    *mut ::core::ffi::c_uchar,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
) -> ();
#[no_mangle]
pub unsafe extern "C" fn vp8_loop_filter_mbh_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    let mut mblim: ::core::ffi::c_uchar = *(*lfi).mblim;
    let mut lim: ::core::ffi::c_uchar = *(*lfi).lim;
    let mut hev_thr: ::core::ffi::c_uchar = *(*lfi).hev_thr;
    vp8_mbloop_filter_horizontal_edge_y_neon(y_ptr, y_stride, mblim, lim, hev_thr);
    if !u_ptr.is_null() {
        vp8_mbloop_filter_horizontal_edge_uv_neon(u_ptr, uv_stride, mblim, lim, hev_thr, v_ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_loop_filter_mbv_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    let mut mblim: ::core::ffi::c_uchar = *(*lfi).mblim;
    let mut lim: ::core::ffi::c_uchar = *(*lfi).lim;
    let mut hev_thr: ::core::ffi::c_uchar = *(*lfi).hev_thr;
    vp8_mbloop_filter_vertical_edge_y_neon(y_ptr, y_stride, mblim, lim, hev_thr);
    if !u_ptr.is_null() {
        vp8_mbloop_filter_vertical_edge_uv_neon(u_ptr, uv_stride, mblim, lim, hev_thr, v_ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_loop_filter_bh_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    let mut blim: ::core::ffi::c_uchar = *(*lfi).blim;
    let mut lim: ::core::ffi::c_uchar = *(*lfi).lim;
    let mut hev_thr: ::core::ffi::c_uchar = *(*lfi).hev_thr;
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((4 as ::core::ffi::c_int * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((8 as ::core::ffi::c_int * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_horizontal_edge_y_neon(
        y_ptr.offset((12 as ::core::ffi::c_int * y_stride) as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    if !u_ptr.is_null() {
        vp8_loop_filter_horizontal_edge_uv_neon(
            u_ptr.offset((4 as ::core::ffi::c_int * uv_stride) as isize),
            uv_stride,
            blim,
            lim,
            hev_thr,
            v_ptr.offset((4 as ::core::ffi::c_int * uv_stride) as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_loop_filter_bv_neon(
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
    mut lfi: *mut loop_filter_info,
) {
    let mut blim: ::core::ffi::c_uchar = *(*lfi).blim;
    let mut lim: ::core::ffi::c_uchar = *(*lfi).lim;
    let mut hev_thr: ::core::ffi::c_uchar = *(*lfi).hev_thr;
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(4 as ::core::ffi::c_int as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(8 as ::core::ffi::c_int as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    vp8_loop_filter_vertical_edge_y_neon(
        y_ptr.offset(12 as ::core::ffi::c_int as isize),
        y_stride,
        blim,
        lim,
        hev_thr,
    );
    if !u_ptr.is_null() {
        vp8_loop_filter_vertical_edge_uv_neon(
            u_ptr.offset(4 as ::core::ffi::c_int as isize),
            uv_stride,
            blim,
            lim,
            hev_thr,
            v_ptr.offset(4 as ::core::ffi::c_int as isize),
        );
    }
}
