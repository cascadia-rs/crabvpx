use std::ffi::c_void;
unsafe extern "Rust" {}
pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_intra_recon(mut ybf: *mut YV12_BUFFER_CONFIG) {
    unsafe {
        let mut i: i32 = 0;
        core::ptr::write_bytes(
            (*ybf)
                .y_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).y_width + 5 as i32) as size_t,
        );
        i = 0 as i32;
        while i < (*ybf).y_height {
            *(*ybf)
                .y_buffer
                .offset(((*ybf).y_stride * i - 1 as i32) as isize) = 129 as uint8_t;
            i += 1;
        }
        core::ptr::write_bytes(
            (*ybf)
                .u_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as size_t,
        );
        i = 0 as i32;
        while i < (*ybf).uv_height {
            *(*ybf)
                .u_buffer
                .offset(((*ybf).uv_stride * i - 1 as i32) as isize) = 129 as uint8_t;
            i += 1;
        }
        core::ptr::write_bytes(
            (*ybf)
                .v_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as size_t,
        );
        i = 0 as i32;
        while i < (*ybf).uv_height {
            *(*ybf)
                .v_buffer
                .offset(((*ybf).uv_stride * i - 1 as i32) as isize) = 129 as uint8_t;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_intra_recon_top_line(mut ybf: *mut YV12_BUFFER_CONFIG) {
    unsafe {
        core::ptr::write_bytes(
            (*ybf)
                .y_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).y_width + 5 as i32) as size_t,
        );
        core::ptr::write_bytes(
            (*ybf)
                .u_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as size_t,
        );
        core::ptr::write_bytes(
            (*ybf)
                .v_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as size_t,
        );
    }
}
