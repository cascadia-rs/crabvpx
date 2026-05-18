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
pub fn vp8_setup_block_dptrs(x: &mut MACROBLOCKD) {
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 4 as ::core::ffi::c_int {
            let idx = (r * 64 + c * 4) as usize;
            x.block[(r * 4 as ::core::ffi::c_int + c) as usize].predictor =
                &raw mut x.predictor[idx];
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            let idx = (256 + r * 32 + c * 4) as usize;
            x.block[(16 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = &raw mut x.predictor[idx];
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            let idx = (320 + r * 32 + c * 4) as usize;
            x.block[(20 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = &raw mut x.predictor[idx];
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 25 as ::core::ffi::c_int {
        let q_idx = (r * 16) as usize;
        x.block[r as usize].qcoeff = &raw mut x.qcoeff[q_idx];
        x.block[r as usize].dqcoeff = &raw mut x.dqcoeff[q_idx];
        x.block[r as usize].eob = &raw mut x.eobs[r as usize];
        r += 1;
    }
}
pub fn vp8_build_block_doffsets(x: &mut MACROBLOCKD) {
    let mut block: ::core::ffi::c_int = 0;
    block = 0 as ::core::ffi::c_int;
    while block < 16 as ::core::ffi::c_int {
        x.block[block as usize].offset =
            (block >> 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * x.dst.y_stride
                + (block & 3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        block += 1;
    }
    block = 16 as ::core::ffi::c_int;
    while block < 20 as ::core::ffi::c_int {
        x.block[block as usize].offset = (block - 16 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
            * 4 as ::core::ffi::c_int
            * x.dst.uv_stride
            + (block & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        x.block[(block + 4 as ::core::ffi::c_int) as usize].offset =
            x.block[block as usize].offset;
        block += 1;
    }
}
