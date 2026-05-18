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
pub fn vp8_setup_block_dptrs(x: &mut MACROBLOCKD) { unsafe {
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 4 as ::core::ffi::c_int {
            x.block[(r * 4 as ::core::ffi::c_int + c) as usize].predictor =
                x.predictor.as_mut_ptr()
                    .offset((r * 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
                    .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            x.block[(16 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = x.predictor.as_mut_ptr()
                .offset(256 as ::core::ffi::c_int as isize)
                .offset((r * 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            x.block[(20 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = x.predictor.as_mut_ptr()
                .offset(320 as ::core::ffi::c_int as isize)
                .offset((r * 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 25 as ::core::ffi::c_int {
        x.block[r as usize].qcoeff = x.qcoeff.as_mut_ptr()
            .offset((r * 16 as ::core::ffi::c_int) as isize);
        x.block[r as usize].dqcoeff = x.dqcoeff.as_mut_ptr()
            .offset((r * 16 as ::core::ffi::c_int) as isize);
        x.block[r as usize].eob = x.eobs.as_mut_ptr().offset(r as isize);
        r += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_build_block_doffsets(mut x: *mut MACROBLOCKD) { unsafe {
    let mut block: ::core::ffi::c_int = 0;
    block = 0 as ::core::ffi::c_int;
    while block < 16 as ::core::ffi::c_int {
        (*x).block[block as usize].offset =
            (block >> 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * (*x).dst.y_stride
                + (block & 3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        block += 1;
    }
    block = 16 as ::core::ffi::c_int;
    while block < 20 as ::core::ffi::c_int {
        (*x).block[block as usize].offset = (block - 16 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
            * 4 as ::core::ffi::c_int
            * (*x).dst.uv_stride
            + (block & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        (*x).block[(block + 4 as ::core::ffi::c_int) as usize].offset =
            (*x).block[block as usize].offset;
        block += 1;
    }
}}
