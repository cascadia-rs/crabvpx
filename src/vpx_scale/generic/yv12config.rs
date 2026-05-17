pub use crate::vp8::common::types::*;
pub type uint8_t = u8;

pub type __darwin_size_t = usize;
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
pub type size_t = __darwin_size_t;

#[derive(Clone, Copy)]
#[repr(align(32))]
struct Align32([u8; 32]);

pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_de_alloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
) -> ::core::ffi::c_int { unsafe {
    if !ybf.is_null() {
        if (*ybf).buffer_alloc_sz > 0 as size_t {
            let alloc_size = (*ybf).buffer_alloc_sz / 32;
            let ptr = (*ybf).buffer_alloc as *mut Align32;
            let _vec = Vec::from_raw_parts(ptr, alloc_size, alloc_size);
        }
        *ybf = core::mem::zeroed();
    } else {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_realloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut border: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    if !ybf.is_null() {
        let mut aligned_width: ::core::ffi::c_int =
            width + 15 as ::core::ffi::c_int & !(15 as ::core::ffi::c_int);
        let mut aligned_height: ::core::ffi::c_int =
            height + 15 as ::core::ffi::c_int & !(15 as ::core::ffi::c_int);
        let mut y_stride: ::core::ffi::c_int =
            aligned_width + 2 as ::core::ffi::c_int * border + 31 as ::core::ffi::c_int
                & !(31 as ::core::ffi::c_int);
        let mut yplane_size: ::core::ffi::c_int =
            (aligned_height + 2 as ::core::ffi::c_int * border) * y_stride;
        let mut uv_width: ::core::ffi::c_int = aligned_width >> 1 as ::core::ffi::c_int;
        let mut uv_height: ::core::ffi::c_int = aligned_height >> 1 as ::core::ffi::c_int;
        let mut uv_stride: ::core::ffi::c_int = y_stride >> 1 as ::core::ffi::c_int;
        let mut uvplane_size: ::core::ffi::c_int = (uv_height + border) * uv_stride;
        let frame_size: size_t = (yplane_size + 2 as ::core::ffi::c_int * uvplane_size) as size_t;
        if (*ybf).buffer_alloc.is_null() {
            let alloc_size = (frame_size + 31) / 32;
            let mut vec = vec![Align32([0; 32]); alloc_size];
            (*ybf).buffer_alloc = vec.as_mut_ptr() as *mut uint8_t;
            core::mem::forget(vec);
            if (*ybf).buffer_alloc.is_null() {
                (*ybf).buffer_alloc_sz = 0 as size_t;
                return -(1 as ::core::ffi::c_int);
            }
            (*ybf).buffer_alloc_sz = alloc_size * 32;
        }
        if (*ybf).buffer_alloc_sz < frame_size {
            return -(1 as ::core::ffi::c_int);
        }
        if border & 0x1f as ::core::ffi::c_int != 0 {
            return -(3 as ::core::ffi::c_int);
        }
        (*ybf).y_crop_width = width;
        (*ybf).y_crop_height = height;
        (*ybf).y_width = aligned_width;
        (*ybf).y_height = aligned_height;
        (*ybf).y_stride = y_stride;
        (*ybf).uv_crop_width = (width + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        (*ybf).uv_crop_height = (height + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        (*ybf).uv_width = uv_width;
        (*ybf).uv_height = uv_height;
        (*ybf).uv_stride = uv_stride;
        (*ybf).alpha_width = 0 as ::core::ffi::c_int;
        (*ybf).alpha_height = 0 as ::core::ffi::c_int;
        (*ybf).alpha_stride = 0 as ::core::ffi::c_int;
        (*ybf).border = border;
        (*ybf).frame_size = frame_size;
        (*ybf).y_buffer = (*ybf)
            .buffer_alloc
            .offset((border * y_stride) as isize)
            .offset(border as isize);
        (*ybf).u_buffer = (*ybf)
            .buffer_alloc
            .offset(yplane_size as isize)
            .offset((border / 2 as ::core::ffi::c_int * uv_stride) as isize)
            .offset((border / 2 as ::core::ffi::c_int) as isize);
        (*ybf).v_buffer = (*ybf)
            .buffer_alloc
            .offset(yplane_size as isize)
            .offset(uvplane_size as isize)
            .offset((border / 2 as ::core::ffi::c_int * uv_stride) as isize)
            .offset((border / 2 as ::core::ffi::c_int) as isize);
        (*ybf).alpha_buffer = ::core::ptr::null_mut::<uint8_t>();
        (*ybf).corrupted = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    return -(2 as ::core::ffi::c_int);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_alloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut border: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    if !ybf.is_null() {
        vp8_yv12_de_alloc_frame_buffer(ybf);
        return vp8_yv12_realloc_frame_buffer(ybf, width, height, border);
    }
    return -(2 as ::core::ffi::c_int);
}}

