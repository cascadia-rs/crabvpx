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
pub use crate::vp8::common::types::*;

pub fn vp8_build_block_doffsets(x: &mut MACROBLOCKD) {
    let mut block: i32 = 0;
    block = 0_i32;
    while block < 16_i32 {
        x.block[block as usize].offset =
            (block >> 2_i32) * 4_i32 * x.dst_y_stride + (block & 3_i32) * 4_i32;
        block += 1;
    }
    block = 16_i32;
    while block < 20_i32 {
        x.block[block as usize].offset =
            ((block - 16_i32) >> 1_i32) * 4_i32 * x.dst_uv_stride + (block & 1_i32) * 4_i32;
        x.block[(block + 4_i32) as usize].offset = x.block[block as usize].offset;
        block += 1;
    }
}
