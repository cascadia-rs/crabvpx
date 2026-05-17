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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_swap_yv12_buffer(
    mut new_frame: *mut YV12_BUFFER_CONFIG,
    mut last_frame: *mut YV12_BUFFER_CONFIG,
) { unsafe {
    let mut temp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    temp = (*last_frame).buffer_alloc as *mut ::core::ffi::c_uchar;
    (*last_frame).buffer_alloc = (*new_frame).buffer_alloc;
    (*new_frame).buffer_alloc = temp as *mut uint8_t;
    temp = (*last_frame).y_buffer as *mut ::core::ffi::c_uchar;
    (*last_frame).y_buffer = (*new_frame).y_buffer;
    (*new_frame).y_buffer = temp as *mut uint8_t;
    temp = (*last_frame).u_buffer as *mut ::core::ffi::c_uchar;
    (*last_frame).u_buffer = (*new_frame).u_buffer;
    (*new_frame).u_buffer = temp as *mut uint8_t;
    temp = (*last_frame).v_buffer as *mut ::core::ffi::c_uchar;
    (*last_frame).v_buffer = (*new_frame).v_buffer;
    (*new_frame).v_buffer = temp as *mut uint8_t;
}}
