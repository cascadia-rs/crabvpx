unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub use crate::vp8::common::types::*;
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type uint8_t = u8;
unsafe extern "C" fn extend_plane(
    src: *mut uint8_t,
    mut src_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut extend_top: ::core::ffi::c_int,
    mut extend_left: ::core::ffi::c_int,
    mut extend_bottom: ::core::ffi::c_int,
    mut extend_right: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let linesize: ::core::ffi::c_int = extend_left + extend_right + width;
    let mut src_ptr1: *mut uint8_t = src;
    let mut src_ptr2: *mut uint8_t = src
        .offset(width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut dst_ptr1: *mut uint8_t = src.offset(-(extend_left as isize));
    let mut dst_ptr2: *mut uint8_t = src.offset(width as isize);
    i = 0 as ::core::ffi::c_int;
    while i < height {
        memset(
            dst_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            extend_left as size_t,
        );
        memset(
            dst_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            extend_right as size_t,
        );
        src_ptr1 = src_ptr1.offset(src_stride as isize);
        src_ptr2 = src_ptr2.offset(src_stride as isize);
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        i += 1;
    }
    src_ptr1 = src.offset(-(extend_left as isize));
    src_ptr2 = src
        .offset((src_stride * (height - 1 as ::core::ffi::c_int)) as isize)
        .offset(-(extend_left as isize));
    dst_ptr1 = src
        .offset((src_stride * -extend_top) as isize)
        .offset(-(extend_left as isize));
    dst_ptr2 = src
        .offset((src_stride * height) as isize)
        .offset(-(extend_left as isize));
    i = 0 as ::core::ffi::c_int;
    while i < extend_top {
        memcpy(
            dst_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            linesize as size_t,
        );
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < extend_bottom {
        memcpy(
            dst_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            linesize as size_t,
        );
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        i += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_extend_frame_borders_c(mut ybf: *mut YV12_BUFFER_CONFIG) { unsafe {
    let uv_border: ::core::ffi::c_int = (*ybf).border / 2 as ::core::ffi::c_int;
    extend_plane(
        (*ybf).y_buffer,
        (*ybf).y_stride,
        (*ybf).y_crop_width,
        (*ybf).y_crop_height,
        (*ybf).border,
        (*ybf).border,
        (*ybf).border + (*ybf).y_height - (*ybf).y_crop_height,
        (*ybf).border + (*ybf).y_width - (*ybf).y_crop_width,
    );
    extend_plane(
        (*ybf).u_buffer,
        (*ybf).uv_stride,
        (*ybf).uv_crop_width,
        (*ybf).uv_crop_height,
        uv_border,
        uv_border,
        uv_border + (*ybf).uv_height - (*ybf).uv_crop_height,
        uv_border + (*ybf).uv_width - (*ybf).uv_crop_width,
    );
    extend_plane(
        (*ybf).v_buffer,
        (*ybf).uv_stride,
        (*ybf).uv_crop_width,
        (*ybf).uv_crop_height,
        uv_border,
        uv_border,
        uv_border + (*ybf).uv_height - (*ybf).uv_crop_height,
        uv_border + (*ybf).uv_width - (*ybf).uv_crop_width,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_copy_frame_c(
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
) { unsafe {
    let mut row: ::core::ffi::c_int = 0;
    let mut src: *const uint8_t = (*src_ybc).y_buffer;
    let mut dst: *mut uint8_t = (*dst_ybc).y_buffer;
    row = 0 as ::core::ffi::c_int;
    while row < (*src_ybc).y_height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (*src_ybc).y_width as size_t,
        );
        src = src.offset((*src_ybc).y_stride as isize);
        dst = dst.offset((*dst_ybc).y_stride as isize);
        row += 1;
    }
    src = (*src_ybc).u_buffer;
    dst = (*dst_ybc).u_buffer;
    row = 0 as ::core::ffi::c_int;
    while row < (*src_ybc).uv_height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (*src_ybc).uv_width as size_t,
        );
        src = src.offset((*src_ybc).uv_stride as isize);
        dst = dst.offset((*dst_ybc).uv_stride as isize);
        row += 1;
    }
    src = (*src_ybc).v_buffer;
    dst = (*dst_ybc).v_buffer;
    row = 0 as ::core::ffi::c_int;
    while row < (*src_ybc).uv_height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (*src_ybc).uv_width as size_t,
        );
        src = src.offset((*src_ybc).uv_stride as isize);
        dst = dst.offset((*dst_ybc).uv_stride as isize);
        row += 1;
    }
    vp8_yv12_extend_frame_borders_c(dst_ybc);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_yv12_copy_y_c(
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
) { unsafe {
    let mut row: ::core::ffi::c_int = 0;
    let mut src: *const uint8_t = (*src_ybc).y_buffer;
    let mut dst: *mut uint8_t = (*dst_ybc).y_buffer;
    row = 0 as ::core::ffi::c_int;
    while row < (*src_ybc).y_height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (*src_ybc).y_width as size_t,
        );
        src = src.offset((*src_ybc).y_stride as isize);
        dst = dst.offset((*dst_ybc).y_stride as isize);
        row += 1;
    }
}}
