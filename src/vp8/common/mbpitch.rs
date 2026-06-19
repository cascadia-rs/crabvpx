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
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;

pub fn vp8_build_block_doffsets(x: &mut MACROBLOCKD) {
    let mut block: ::core::ffi::c_int = 0;
    block = 0 as ::core::ffi::c_int;
    while block < 16 as ::core::ffi::c_int {
        x.block[block as usize].offset =
            (block >> 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * x.dst_y_stride
                + (block & 3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        block += 1;
    }
    block = 16 as ::core::ffi::c_int;
    while block < 20 as ::core::ffi::c_int {
        x.block[block as usize].offset = ((block - 16 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)
            * 4 as ::core::ffi::c_int
            * x.dst_uv_stride
            + (block & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        x.block[(block + 4 as ::core::ffi::c_int) as usize].offset = x.block[block as usize].offset;
        block += 1;
    }
}
