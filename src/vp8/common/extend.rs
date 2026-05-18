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
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
unsafe extern "C" fn copy_and_extend_plane(
    mut s: *mut ::core::ffi::c_uchar,
    mut sp: ::core::ffi::c_int,
    mut d: *mut ::core::ffi::c_uchar,
    mut dp: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut et: ::core::ffi::c_int,
    mut el: ::core::ffi::c_int,
    mut eb: ::core::ffi::c_int,
    mut er: ::core::ffi::c_int,
    mut interleave_step: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut src_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut linesize: ::core::ffi::c_int = 0;
    if interleave_step < 1 as ::core::ffi::c_int {
        interleave_step = 1 as ::core::ffi::c_int;
    }
    src_ptr1 = s;
    src_ptr2 = s.offset(((w - 1 as ::core::ffi::c_int) * interleave_step) as isize);
    dest_ptr1 = d.offset(-(el as isize));
    dest_ptr2 = d.offset(w as isize);
    i = 0 as ::core::ffi::c_int;
    while i < h {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            el as size_t,
        );
        if interleave_step == 1 as ::core::ffi::c_int {
            memcpy(
                dest_ptr1.offset(el as isize) as *mut ::core::ffi::c_void,
                src_ptr1 as *const ::core::ffi::c_void,
                w as size_t,
            );
        } else {
            j = 0 as ::core::ffi::c_int;
            while j < w {
                *dest_ptr1.offset((el + j) as isize) =
                    *src_ptr1.offset((interleave_step * j) as isize);
                j += 1;
            }
        }
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            er as size_t,
        );
        src_ptr1 = src_ptr1.offset(sp as isize);
        src_ptr2 = src_ptr2.offset(sp as isize);
        dest_ptr1 = dest_ptr1.offset(dp as isize);
        dest_ptr2 = dest_ptr2.offset(dp as isize);
        i += 1;
    }
    src_ptr1 = d.offset(-(el as isize));
    src_ptr2 = d
        .offset((dp * (h - 1 as ::core::ffi::c_int)) as isize)
        .offset(-(el as isize));
    dest_ptr1 = d.offset((dp * -et) as isize).offset(-(el as isize));
    dest_ptr2 = d.offset((dp * h) as isize).offset(-(el as isize));
    linesize = el + er + w;
    i = 0 as ::core::ffi::c_int;
    while i < et {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            linesize as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(dp as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < eb {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            linesize as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(dp as isize);
        i += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_and_extend_frame(
    mut src: *mut YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
) { unsafe {
    let mut et: ::core::ffi::c_int = (*dst).border;
    let mut el: ::core::ffi::c_int = (*dst).border;
    let mut eb: ::core::ffi::c_int = (*dst).border + (*dst).y_height - (*src).y_height;
    let mut er: ::core::ffi::c_int = (*dst).border + (*dst).y_width - (*src).y_width;
    let mut chroma_step: ::core::ffi::c_int = if (*src).v_buffer.offset_from((*src).u_buffer)
        as ::core::ffi::c_long
        == 1 as ::core::ffi::c_long
    {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    copy_and_extend_plane(
        (*src).y_buffer as *mut ::core::ffi::c_uchar,
        (*src).y_stride,
        (*dst).y_buffer as *mut ::core::ffi::c_uchar,
        (*dst).y_stride,
        (*src).y_height,
        (*src).y_width,
        et,
        el,
        eb,
        er,
        1 as ::core::ffi::c_int,
    );
    et = (*dst).border >> 1 as ::core::ffi::c_int;
    el = (*dst).border >> 1 as ::core::ffi::c_int;
    eb = ((*dst).border >> 1 as ::core::ffi::c_int) + (*dst).uv_height - (*src).uv_height;
    er = ((*dst).border >> 1 as ::core::ffi::c_int) + (*dst).uv_width - (*src).uv_width;
    copy_and_extend_plane(
        (*src).u_buffer as *mut ::core::ffi::c_uchar,
        (*src).uv_stride,
        (*dst).u_buffer as *mut ::core::ffi::c_uchar,
        (*dst).uv_stride,
        (*src).uv_height,
        (*src).uv_width,
        et,
        el,
        eb,
        er,
        chroma_step,
    );
    copy_and_extend_plane(
        (*src).v_buffer as *mut ::core::ffi::c_uchar,
        (*src).uv_stride,
        (*dst).v_buffer as *mut ::core::ffi::c_uchar,
        (*dst).uv_stride,
        (*src).uv_height,
        (*src).uv_width,
        et,
        el,
        eb,
        er,
        chroma_step,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_and_extend_frame_with_rect(
    mut src: *mut YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
    mut srcy: ::core::ffi::c_int,
    mut srcx: ::core::ffi::c_int,
    mut srch: ::core::ffi::c_int,
    mut srcw: ::core::ffi::c_int,
) { unsafe {
    let mut et: ::core::ffi::c_int = (*dst).border;
    let mut el: ::core::ffi::c_int = (*dst).border;
    let mut eb: ::core::ffi::c_int = (*dst).border + (*dst).y_height - (*src).y_height;
    let mut er: ::core::ffi::c_int = (*dst).border + (*dst).y_width - (*src).y_width;
    let mut src_y_offset: ::core::ffi::c_int = srcy * (*src).y_stride + srcx;
    let mut dst_y_offset: ::core::ffi::c_int = srcy * (*dst).y_stride + srcx;
    let mut src_uv_offset: ::core::ffi::c_int =
        (srcy * (*src).uv_stride >> 1 as ::core::ffi::c_int) + (srcx >> 1 as ::core::ffi::c_int);
    let mut dst_uv_offset: ::core::ffi::c_int =
        (srcy * (*dst).uv_stride >> 1 as ::core::ffi::c_int) + (srcx >> 1 as ::core::ffi::c_int);
    let mut chroma_step: ::core::ffi::c_int = if (*src).v_buffer.offset_from((*src).u_buffer)
        as ::core::ffi::c_long
        == 1 as ::core::ffi::c_long
    {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    if srcy != 0 {
        et = 0 as ::core::ffi::c_int;
    }
    if srcx != 0 {
        el = 0 as ::core::ffi::c_int;
    }
    if srcy + srch != (*src).y_height {
        eb = 0 as ::core::ffi::c_int;
    }
    if srcx + srcw != (*src).y_width {
        er = 0 as ::core::ffi::c_int;
    }
    copy_and_extend_plane(
        (*src).y_buffer.offset(src_y_offset as isize),
        (*src).y_stride,
        (*dst).y_buffer.offset(dst_y_offset as isize),
        (*dst).y_stride,
        srch,
        srcw,
        et,
        el,
        eb,
        er,
        1 as ::core::ffi::c_int,
    );
    et = et + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    el = el + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    eb = eb + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    er = er + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    srch = srch + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    srcw = srcw + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    copy_and_extend_plane(
        (*src).u_buffer.offset(src_uv_offset as isize),
        (*src).uv_stride,
        (*dst).u_buffer.offset(dst_uv_offset as isize),
        (*dst).uv_stride,
        srch,
        srcw,
        et,
        el,
        eb,
        er,
        chroma_step,
    );
    copy_and_extend_plane(
        (*src).v_buffer.offset(src_uv_offset as isize),
        (*src).uv_stride,
        (*dst).v_buffer.offset(dst_uv_offset as isize),
        (*dst).uv_stride,
        srch,
        srcw,
        et,
        el,
        eb,
        er,
        chroma_step,
    );
}}
#[unsafe(no_mangle)]
pub fn vp8_extend_mb_row(
    ybf: &mut YV12_BUFFER_CONFIG,
    mb_row: i32,
) {
    let y_stride = ybf.y_stride as usize;
    let uv_stride = ybf.uv_stride as usize;
    let y_width = ybf.y_width as usize;
    let uv_width = ybf.uv_width as usize;
    let border = ybf.border as usize;
    let mb_row = mb_row as usize;

    // Y plane border extension
    {
        let y_slice = unsafe { ybf.y_slice_mut() };
        
        // Y plane row 14
        {
            let row_idx = border + mb_row * 16 + 14;
            let row_start = row_idx * y_stride;
            let src_val = y_slice[row_start + border + y_width - 1];
            let dst_start = row_start + border + y_width;
            for i in 0..4 {
                y_slice[dst_start + i] = src_val;
            }
        }
        // Y plane row 15
        {
            let row_idx = border + mb_row * 16 + 15;
            let row_start = row_idx * y_stride;
            let src_val = y_slice[row_start + border + y_width - 1];
            let dst_start = row_start + border + y_width;
            for i in 0..4 {
                y_slice[dst_start + i] = src_val;
            }
        }
    }

    let uv_border = border / 2;

    // U plane border extension
    {
        let u_slice = unsafe { ybf.u_slice_mut() };
        
        // U plane row 6
        {
            let row_idx = uv_border + mb_row * 8 + 6;
            let row_start = row_idx * uv_stride;
            let src_val = u_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                u_slice[dst_start + i] = src_val;
            }
        }
        // U plane row 7
        {
            let row_idx = uv_border + mb_row * 8 + 7;
            let row_start = row_idx * uv_stride;
            let src_val = u_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                u_slice[dst_start + i] = src_val;
            }
        }
    }

    // V plane border extension
    {
        let v_slice = unsafe { ybf.v_slice_mut() };
        
        // V plane row 6
        {
            let row_idx = uv_border + mb_row * 8 + 6;
            let row_start = row_idx * uv_stride;
            let src_val = v_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                v_slice[dst_start + i] = src_val;
            }
        }
        // V plane row 7
        {
            let row_idx = uv_border + mb_row * 8 + 7;
            let row_start = row_idx * uv_stride;
            let src_val = v_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                v_slice[dst_start + i] = src_val;
            }
        }
    }
}
