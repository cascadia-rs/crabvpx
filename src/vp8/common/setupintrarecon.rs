unsafe extern "C" {
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub y_width: ::core::ffi::c_int,
    pub y_height: ::core::ffi::c_int,
    pub y_crop_width: ::core::ffi::c_int,
    pub y_crop_height: ::core::ffi::c_int,
    pub y_stride: ::core::ffi::c_int,
    pub uv_width: ::core::ffi::c_int,
    pub uv_height: ::core::ffi::c_int,
    pub uv_crop_width: ::core::ffi::c_int,
    pub uv_crop_height: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub alpha_width: ::core::ffi::c_int,
    pub alpha_height: ::core::ffi::c_int,
    pub alpha_stride: ::core::ffi::c_int,
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: ::core::ffi::c_int,
    pub frame_size: size_t,
    pub subsampling_x: ::core::ffi::c_int,
    pub subsampling_y: ::core::ffi::c_int,
    pub bit_depth: ::core::ffi::c_uint,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: ::core::ffi::c_int,
    pub render_height: ::core::ffi::c_int,
    pub corrupted: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_setup_intra_recon(mut ybf: *mut YV12_BUFFER_CONFIG) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        memset(
            (*ybf)
                .y_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).y_width + 5 as ::core::ffi::c_int) as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*ybf).y_height {
            *(*ybf)
                .y_buffer
                .offset(((*ybf).y_stride * i - 1 as ::core::ffi::c_int) as isize) =
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as uint8_t;
            i += 1;
        }
        memset(
            (*ybf)
                .u_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).uv_width + 5 as ::core::ffi::c_int) as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*ybf).uv_height {
            *(*ybf)
                .u_buffer
                .offset(((*ybf).uv_stride * i - 1 as ::core::ffi::c_int) as isize) =
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as uint8_t;
            i += 1;
        }
        memset(
            (*ybf)
                .v_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).uv_width + 5 as ::core::ffi::c_int) as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*ybf).uv_height {
            *(*ybf)
                .v_buffer
                .offset(((*ybf).uv_stride * i - 1 as ::core::ffi::c_int) as isize) =
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as uint8_t;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_setup_intra_recon_top_line(mut ybf: *mut YV12_BUFFER_CONFIG) {
    unsafe {
        memset(
            (*ybf)
                .y_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).y_width + 5 as ::core::ffi::c_int) as size_t,
        );
        memset(
            (*ybf)
                .u_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).uv_width + 5 as ::core::ffi::c_int) as size_t,
        );
        memset(
            (*ybf)
                .v_buffer
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*ybf).uv_width + 5 as ::core::ffi::c_int) as size_t,
        );
    }
}
