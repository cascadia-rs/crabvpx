use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;

unsafe fn copy_and_extend_plane(
    mut s: *mut u8,
    mut sp: i32,
    mut d: *mut u8,
    mut dp: i32,
    mut h: i32,
    mut w: i32,
    mut et: i32,
    mut el: i32,
    mut eb: i32,
    mut er: i32,
    mut interleave_step: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut src_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut src_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut linesize: i32 = 0;
        if interleave_step < 1 as i32 {
            interleave_step = 1 as i32;
        }
        src_ptr1 = s;
        src_ptr2 = s.offset(((w - 1 as i32) * interleave_step) as isize);
        dest_ptr1 = d.offset(-(el as isize));
        dest_ptr2 = d.offset(w as isize);
        i = 0 as i32;
        while i < h {
            core::ptr::write_bytes(
                dest_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as u8,
                el as usize,
            );
            if interleave_step == 1 as i32 {
                core::ptr::copy_nonoverlapping(
                    src_ptr1 as *const c_void as *const u8,
                    dest_ptr1.offset(el as isize) as *mut c_void as *mut u8,
                    w as usize,
                );
            } else {
                j = 0 as i32;
                while j < w {
                    *dest_ptr1.offset((el + j) as isize) =
                        *src_ptr1.offset((interleave_step * j) as isize);
                    j += 1;
                }
            }
            core::ptr::write_bytes(
                dest_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as u8,
                er as usize,
            );
            src_ptr1 = src_ptr1.offset(sp as isize);
            src_ptr2 = src_ptr2.offset(sp as isize);
            dest_ptr1 = dest_ptr1.offset(dp as isize);
            dest_ptr2 = dest_ptr2.offset(dp as isize);
            i += 1;
        }
        src_ptr1 = d.offset(-(el as isize));
        src_ptr2 = d
            .offset((dp * (h - 1 as i32)) as isize)
            .offset(-(el as isize));
        dest_ptr1 = d.offset((dp * -et) as isize).offset(-(el as isize));
        dest_ptr2 = d.offset((dp * h) as isize).offset(-(el as isize));
        linesize = el + er + w;
        i = 0 as i32;
        while i < et {
            core::ptr::copy_nonoverlapping(
                src_ptr1 as *const c_void as *const u8,
                dest_ptr1 as *mut c_void as *mut u8,
                linesize as usize,
            );
            dest_ptr1 = dest_ptr1.offset(dp as isize);
            i += 1;
        }
        i = 0 as i32;
        while i < eb {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dest_ptr2 as *mut c_void as *mut u8,
                linesize as usize,
            );
            dest_ptr2 = dest_ptr2.offset(dp as isize);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_copy_and_extend_frame(
    mut src: *mut Yv12BufferConfig,
    mut dst: *mut Yv12BufferConfig,
) {
    unsafe {
        let mut et: i32 = (*dst).border;
        let mut el: i32 = (*dst).border;
        let mut eb: i32 = (*dst).border + (*dst).y_height - (*src).y_height;
        let mut er: i32 = (*dst).border + (*dst).y_width - (*src).y_width;
        let mut chroma_step: i32 =
            if (*src).v_buffer.offset_from((*src).u_buffer) as i64 == 1 as i64 {
                2 as i32
            } else {
                1 as i32
            };
        copy_and_extend_plane(
            (*src).y_buffer as *mut u8,
            (*src).y_stride,
            (*dst).y_buffer as *mut u8,
            (*dst).y_stride,
            (*src).y_height,
            (*src).y_width,
            et,
            el,
            eb,
            er,
            1 as i32,
        );
        et = (*dst).border >> 1 as i32;
        el = (*dst).border >> 1 as i32;
        eb = ((*dst).border >> 1 as i32) + (*dst).uv_height - (*src).uv_height;
        er = ((*dst).border >> 1 as i32) + (*dst).uv_width - (*src).uv_width;
        copy_and_extend_plane(
            (*src).u_buffer as *mut u8,
            (*src).uv_stride,
            (*dst).u_buffer as *mut u8,
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
            (*src).v_buffer as *mut u8,
            (*src).uv_stride,
            (*dst).v_buffer as *mut u8,
            (*dst).uv_stride,
            (*src).uv_height,
            (*src).uv_width,
            et,
            el,
            eb,
            er,
            chroma_step,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_copy_and_extend_frame_with_rect(
    mut src: *mut Yv12BufferConfig,
    mut dst: *mut Yv12BufferConfig,
    mut srcy: i32,
    mut srcx: i32,
    mut srch: i32,
    mut srcw: i32,
) {
    unsafe {
        let mut et: i32 = (*dst).border;
        let mut el: i32 = (*dst).border;
        let mut eb: i32 = (*dst).border + (*dst).y_height - (*src).y_height;
        let mut er: i32 = (*dst).border + (*dst).y_width - (*src).y_width;
        let mut src_y_offset: i32 = srcy * (*src).y_stride + srcx;
        let mut dst_y_offset: i32 = srcy * (*dst).y_stride + srcx;
        let mut src_uv_offset: i32 = ((srcy * (*src).uv_stride) >> 1 as i32) + (srcx >> 1 as i32);
        let mut dst_uv_offset: i32 = ((srcy * (*dst).uv_stride) >> 1 as i32) + (srcx >> 1 as i32);
        let mut chroma_step: i32 =
            if (*src).v_buffer.offset_from((*src).u_buffer) as i64 == 1 as i64 {
                2 as i32
            } else {
                1 as i32
            };
        if srcy != 0 {
            et = 0 as i32;
        }
        if srcx != 0 {
            el = 0 as i32;
        }
        if srcy + srch != (*src).y_height {
            eb = 0 as i32;
        }
        if srcx + srcw != (*src).y_width {
            er = 0 as i32;
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
            1 as i32,
        );
        et = (et + 1 as i32) >> 1 as i32;
        el = (el + 1 as i32) >> 1 as i32;
        eb = (eb + 1 as i32) >> 1 as i32;
        er = (er + 1 as i32) >> 1 as i32;
        srch = (srch + 1 as i32) >> 1 as i32;
        srcw = (srcw + 1 as i32) >> 1 as i32;
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
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_extend_mb_row(
    mut ybf: *mut Yv12BufferConfig,
    mut yptr: *mut u8,
    mut uptr: *mut u8,
    mut vptr: *mut u8,
) {
    unsafe {
        let mut i: i32 = 0;
        yptr = yptr.offset(((*ybf).y_stride * 14 as i32) as isize);
        uptr = uptr.offset(((*ybf).uv_stride * 6 as i32) as isize);
        vptr = vptr.offset(((*ybf).uv_stride * 6 as i32) as isize);
        i = 0 as i32;
        while i < 4 as i32 {
            *yptr.offset(i as isize) = *yptr.offset(-(1 as i32) as isize);
            *uptr.offset(i as isize) = *uptr.offset(-(1 as i32) as isize);
            *vptr.offset(i as isize) = *vptr.offset(-(1 as i32) as isize);
            i += 1;
        }
        yptr = yptr.offset((*ybf).y_stride as isize);
        uptr = uptr.offset((*ybf).uv_stride as isize);
        vptr = vptr.offset((*ybf).uv_stride as isize);
        i = 0 as i32;
        while i < 4 as i32 {
            *yptr.offset(i as isize) = *yptr.offset(-(1 as i32) as isize);
            *uptr.offset(i as isize) = *uptr.offset(-(1 as i32) as isize);
            *vptr.offset(i as isize) = *vptr.offset(-(1 as i32) as isize);
            i += 1;
        }
    }
}
