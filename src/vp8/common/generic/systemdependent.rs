pub use crate::vp8::common::types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: i32,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [i32; 48];
pub type vpx_codec_err_t = u32;
pub const VPX_CODEC_LIST_END: vpx_codec_err_t = 9;
pub const VPX_CODEC_INVALID_PARAM: vpx_codec_err_t = 8;
pub const VPX_CODEC_CORRUPT_FRAME: vpx_codec_err_t = 7;
pub const VPX_CODEC_UNSUP_FEATURE: vpx_codec_err_t = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: vpx_codec_err_t = 5;
pub const VPX_CODEC_INCAPABLE: vpx_codec_err_t = 4;
pub const VPX_CODEC_ABI_MISMATCH: vpx_codec_err_t = 3;
pub const VPX_CODEC_MEM_ERROR: vpx_codec_err_t = 2;
pub const VPX_CODEC_ERROR: vpx_codec_err_t = 1;
pub const VPX_CODEC_OK: vpx_codec_err_t = 0;

pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;

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
fn get_cpu_count() -> i32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as i32)
        .unwrap_or(1)
}
pub fn vp8_machine_specific_config(ctx: &mut VP8_COMMON) {
    ctx.processor_core_count = get_cpu_count();
}
