use std::ffi::c_void;
unsafe extern "Rust" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = u32;
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
unsafe fn extend_plane(
    src: *mut uint8_t,
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
        let mut src_ptr1: *mut uint8_t = src;
        let mut src_ptr2: *mut uint8_t = src.offset(width as isize).offset(-(1 as isize));
        let mut dst_ptr1: *mut uint8_t = src.offset(-(extend_left as isize));
        let mut dst_ptr2: *mut uint8_t = src.offset(width as isize);
        i = 0 as i32;
        while i < height {
            core::ptr::write_bytes(
                dst_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as i32 as u8,
                extend_left as size_t,
            );
            core::ptr::write_bytes(
                dst_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as i32 as u8,
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
                linesize as size_t,
            );
            dst_ptr1 = dst_ptr1.offset(src_stride as isize);
            i += 1;
        }
        i = 0 as i32;
        while i < extend_bottom {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dst_ptr2 as *mut c_void as *mut u8,
                linesize as size_t,
            );
            dst_ptr2 = dst_ptr2.offset(src_stride as isize);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_extend_frame_borders_c(mut ybf: *mut YV12_BUFFER_CONFIG) {
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
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
) {
    unsafe {
        let mut row: i32 = 0;
        let mut src: *const uint8_t = (*src_ybc).y_buffer;
        let mut dst: *mut uint8_t = (*dst_ybc).y_buffer;
        row = 0 as i32;
        while row < (*src_ybc).y_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).y_width as size_t,
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
                (*src_ybc).uv_width as size_t,
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
                (*src_ybc).uv_width as size_t,
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
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
) {
    unsafe {
        let mut row: i32 = 0;
        let mut src: *const uint8_t = (*src_ybc).y_buffer;
        let mut dst: *mut uint8_t = (*dst_ybc).y_buffer;
        row = 0 as i32;
        while row < (*src_ybc).y_height {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                (*src_ybc).y_width as size_t,
            );
            src = src.offset((*src_ybc).y_stride as isize);
            dst = dst.offset((*dst_ybc).y_stride as isize);
            row += 1;
        }
    }
}
