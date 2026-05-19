use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;

pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
unsafe fn extend_plane(
    src: *mut u8,
    mut src_stride: i32,
    mut width: i32,
    mut height: i32,
    mut extend_top: i32,
    mut extend_left: i32,
    mut extend_bottom: i32,
    mut extend_right: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        let linesize: i32 = extend_left + extend_right + width;
        let mut src_ptr1: *mut u8 = src;
        let mut src_ptr2: *mut u8 = src.offset(width as isize).offset(-(1 as isize));
        let mut dst_ptr1: *mut u8 = src.offset(-(extend_left as isize));
        let mut dst_ptr2: *mut u8 = src.offset(width as isize);
        i = 0 as i32;
        while i < height {
            core::ptr::write_bytes(
                dst_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as u8,
                extend_left as usize,
            );
            core::ptr::write_bytes(
                dst_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as u8,
                extend_right as usize,
            );
            src_ptr1 = src_ptr1.offset(src_stride as isize);
            src_ptr2 = src_ptr2.offset(src_stride as isize);
            dst_ptr1 = dst_ptr1.offset(src_stride as isize);
            dst_ptr2 = dst_ptr2.offset(src_stride as isize);
            i += 1;
        }
        src_ptr1 = src.offset(-(extend_left as isize));
        src_ptr2 = src
            .offset((src_stride * (height - 1 as i32)) as isize)
            .offset(-(extend_left as isize));
        dst_ptr1 = src
            .offset((src_stride * -extend_top) as isize)
            .offset(-(extend_left as isize));
        dst_ptr2 = src
            .offset((src_stride * height) as isize)
            .offset(-(extend_left as isize));
        i = 0 as i32;
        while i < extend_top {
            core::ptr::copy_nonoverlapping(
                src_ptr1 as *const c_void as *const u8,
                dst_ptr1 as *mut c_void as *mut u8,
                linesize as usize,
            );
            dst_ptr1 = dst_ptr1.offset(src_stride as isize);
            i += 1;
        }
        i = 0 as i32;
        while i < extend_bottom {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dst_ptr2 as *mut c_void as *mut u8,
                linesize as usize,
            );
            dst_ptr2 = dst_ptr2.offset(src_stride as isize);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_extend_frame_borders_c(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        let uv_border: i32 = (*ybf).border / 2 as i32;
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
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_copy_frame_c(
    mut src_ybc: *const Yv12BufferConfig,
    mut dst_ybc: *mut Yv12BufferConfig,
) {
    unsafe {
        let mut row: i32 = 0;
        let mut src: *const u8 = (*src_ybc).y_buffer;
        let mut dst: *mut u8 = (*dst_ybc).y_buffer;
        row = 0 as i32;
        while row < (*src_ybc).y_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).y_width as usize,
            );
            src = src.offset((*src_ybc).y_stride as isize);
            dst = dst.offset((*dst_ybc).y_stride as isize);
            row += 1;
        }
        src = (*src_ybc).u_buffer;
        dst = (*dst_ybc).u_buffer;
        row = 0 as i32;
        while row < (*src_ybc).uv_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).uv_width as usize,
            );
            src = src.offset((*src_ybc).uv_stride as isize);
            dst = dst.offset((*dst_ybc).uv_stride as isize);
            row += 1;
        }
        src = (*src_ybc).v_buffer;
        dst = (*dst_ybc).v_buffer;
        row = 0 as i32;
        while row < (*src_ybc).uv_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).uv_width as usize,
            );
            src = src.offset((*src_ybc).uv_stride as isize);
            dst = dst.offset((*dst_ybc).uv_stride as isize);
            row += 1;
        }
        vp8_yv12_extend_frame_borders_c(dst_ybc);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_yv12_copy_y_c(
    mut src_ybc: *const Yv12BufferConfig,
    mut dst_ybc: *mut Yv12BufferConfig,
) {
    unsafe {
        let mut row: i32 = 0;
        let mut src: *const u8 = (*src_ybc).y_buffer;
        let mut dst: *mut u8 = (*dst_ybc).y_buffer;
        row = 0 as i32;
        while row < (*src_ybc).y_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).y_width as usize,
            );
            src = src.offset((*src_ybc).y_stride as isize);
            dst = dst.offset((*dst_ybc).y_stride as isize);
            row += 1;
        }
    }
}
