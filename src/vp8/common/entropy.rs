use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
pub type Vp8TreeP = *const i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8TokenStruct {
    pub value: i32,
    pub len: i32,
}
pub type Vp8Token = Vp8TokenStruct;
pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};
pub const VPX_CODEC_LIST_END: u32 = 9;
pub const VPX_CODEC_INVALID_PARAM: u32 = 8;
pub const VPX_CODEC_CORRUPT_FRAME: u32 = 7;
pub const VPX_CODEC_UNSUP_FEATURE: u32 = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: u32 = 5;
pub const VPX_CODEC_INCAPABLE: u32 = 4;
pub const VPX_CODEC_ABI_MISMATCH: u32 = 3;
pub const VPX_CODEC_MEM_ERROR: u32 = 2;
pub const VPX_CODEC_ERROR: u32 = 1;
pub const VPX_CODEC_OK: u32 = 0;
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [i8; 4],
    pub u: [i8; 2],
    pub v: [i8; 2],
    pub y2: i8,
}
pub const INTER_FRAME: u32 = 1;
pub const KEY_FRAME: u32 = 0;
pub const B_MODE_COUNT: u32 = 14;
pub const NEW4X4: u32 = 13;
pub const ZERO4X4: u32 = 12;
pub const ABOVE4X4: u32 = 11;
pub const LEFT4X4: u32 = 10;
pub const B_HU_PRED: u32 = 9;
pub const B_HD_PRED: u32 = 8;
pub const B_VL_PRED: u32 = 7;
pub const B_VR_PRED: u32 = 6;
pub const B_RD_PRED: u32 = 5;
pub const B_LD_PRED: u32 = 4;
pub const B_HE_PRED: u32 = 3;
pub const B_VE_PRED: u32 = 2;
pub const B_TM_PRED: u32 = 1;
pub const B_DC_PRED: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: u32,
    pub mv: IntMv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbModeInfo {
    pub mode: u8,
    pub uv_mode: u8,
    pub ref_frame: u8,
    pub is_4x4: u8,
    pub mv: IntMv,
    pub partitioning: u8,
    pub mb_skip_coeff: u8,
    pub need_to_clamp_mvs: u8,
    pub segment_id: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
pub type ModeInfo = Modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8ExtraBitStruct {
    pub tree: Vp8TreeP,
    pub prob: *const u8,
    pub len: i32,
    pub base_val: i32,
}
unsafe impl Sync for Vp8ExtraBitStruct {}
unsafe impl Send for Vp8ExtraBitStruct {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: VpxInternalErrorInfo,
    pub y1dequant: [[i16; 2]; 128],
    pub y2dequant: [[i16; 2]; 128],
    pub uvdequant: [[i16; 2]; 128],
    pub width: i32,
    pub height: i32,
    pub horiz_scale: i32,
    pub vert_scale: i32,
    pub clamp_type: u32,
    pub frame_to_show: *mut Yv12BufferConfig,
    pub yv12_fb: [Yv12BufferConfig; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: Yv12BufferConfig,
    pub last_frame_type: u32,
    pub frame_type: u32,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub mbs: i32,
    pub mb_rows: i32,
    pub mb_cols: i32,
    pub mode_info_stride: i32,
    pub mb_no_coeff_skip: bool,
    pub no_lpf: bool,
    pub use_bilinear_mc_filter: bool,
    pub full_pixel: bool,
    pub base_qindex: i32,
    pub y1dc_delta_q: i32,
    pub y2dc_delta_q: i32,
    pub y2ac_delta_q: i32,
    pub uvdc_delta_q: i32,
    pub uvac_delta_q: i32,
    pub mip: *mut ModeInfo,
    pub mi: *mut ModeInfo,
    pub show_frame_mi: *mut ModeInfo,
    pub filter_type: u32,
    pub lf_info: LoopFilterInfoN,
    pub filter_level: i32,
    pub last_sharpness_level: i32,
    pub sharpness_level: i32,
    pub refresh_last_frame: i32,
    pub refresh_golden_frame: i32,
    pub refresh_alt_ref_frame: i32,
    pub copy_buffer_to_gf: i32,
    pub copy_buffer_to_arf: i32,
    pub refresh_entropy_probs: bool,
    pub ref_frame_sign_bias: [i32; 4],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: EntropyContextPlanes,
    pub lfc: FrameContext,
    pub fc: FrameContext,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: u32,
    pub processor_core_count: i32,
}
pub const EIGHT_PARTITION: u32 = 3;
pub const FOUR_PARTITION: u32 = 2;
pub const TWO_PARTITION: u32 = 1;
pub const ONE_PARTITION: u32 = 0;
pub type FrameContext = FrameContexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameContexts {
    pub bmode_prob: [u8; 9],
    pub ymode_prob: [u8; 4],
    pub uv_mode_prob: [u8; 3],
    pub sub_mv_ref_prob: [u8; 3],
    pub coef_probs: [[[[u8; 11]; 3]; 8]; 4],
    pub mvc: [MvContext; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [u8; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterInfoN {
    pub mblim: [[u8; 16]; 64],
    pub blim: [[u8; 16]; 64],
    pub lim: [[u8; 16]; 64],
    pub hev_thr: [[u8; 16]; 4],
    pub lvl: [[[u8; 4]; 4]; 4],
    pub hev_thr_lut: [[u8; 64]; 2],
    pub mode_lf_lut: [u8; 10],
}
pub const SIMPLE_LOOPFILTER: u32 = 1;
pub const NORMAL_LOOPFILTER: u32 = 0;
pub const RECON_CLAMP_NOTREQUIRED: u32 = 1;
pub const RECON_CLAMP_REQUIRED: u32 = 0;
pub type Vp8Common = VP8Common;
pub use crate::vp8::common::tables::{
    DCT_EOB_TOKEN, DCT_VAL_CATEGORY1, DCT_VAL_CATEGORY2, DCT_VAL_CATEGORY3, DCT_VAL_CATEGORY4,
    DCT_VAL_CATEGORY5, DCT_VAL_CATEGORY6, ONE_TOKEN, THREE_TOKEN, TWO_TOKEN, ZERO_TOKEN,
};
pub const FOUR_TOKEN: i32 = crate::vp8::common::tables::FOUR_TOKEN;
#[unsafe(no_mangle)]
pub static vp8_coef_update_probs: [[[[u8; 11]; 3]; 8]; 4] = [
    [
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                176 as u8, 246 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                223 as u8, 241 as u8, 252 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                249 as u8, 253 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 244 as u8, 252 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                234 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 246 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                239 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 248 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                251 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                251 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 253 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                250 as u8, 255 as u8, 254 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
    ],
    [
        [
            [
                217 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                225 as u8, 252 as u8, 241 as u8, 253 as u8, 255 as u8, 255 as u8, 254 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                234 as u8, 250 as u8, 241 as u8, 250 as u8, 253 as u8, 255 as u8, 253 as u8,
                254 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                223 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                238 as u8, 253 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 248 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                249 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                247 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                252 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                250 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
    ],
    [
        [
            [
                186 as u8, 251 as u8, 250 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                234 as u8, 251 as u8, 244 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                251 as u8, 251 as u8, 243 as u8, 253 as u8, 254 as u8, 255 as u8, 254 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                236 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                251 as u8, 253 as u8, 253 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
    ],
    [
        [
            [
                248 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                250 as u8, 254 as u8, 252 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                248 as u8, 254 as u8, 249 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 253 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                246 as u8, 253 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                252 as u8, 254 as u8, 251 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 254 as u8, 252 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                248 as u8, 254 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                253 as u8, 255 as u8, 254 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 251 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                245 as u8, 251 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                253 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 251 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                252 as u8, 253 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 252 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                249 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 253 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                250 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
        [
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                254 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
            [
                255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                255 as u8, 255 as u8, 255 as u8, 255 as u8,
            ],
        ],
    ],
];
pub use crate::vp8::common::tables::{
    vp8_coef_bands, vp8_coef_tree, vp8_default_inv_zig_zag, vp8_default_zig_zag_mask,
    vp8_default_zig_zag1d, vp8_mb_feature_data_bits, vp8_prev_token_class,
};
pub use crate::vpx_dsp::tables::VPX_NORM as vp8_norm;
#[unsafe(no_mangle)]
pub static vp8_coef_encodings: [Vp8Token; 12] = [
    Vp8TokenStruct {
        value: 2 as i32,
        len: 2 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 28 as i32,
        len: 5 as i32,
    },
    Vp8TokenStruct {
        value: 58 as i32,
        len: 6 as i32,
    },
    Vp8TokenStruct {
        value: 59 as i32,
        len: 6 as i32,
    },
    Vp8TokenStruct {
        value: 60 as i32,
        len: 6 as i32,
    },
    Vp8TokenStruct {
        value: 61 as i32,
        len: 6 as i32,
    },
    Vp8TokenStruct {
        value: 124 as i32,
        len: 7 as i32,
    },
    Vp8TokenStruct {
        value: 125 as i32,
        len: 7 as i32,
    },
    Vp8TokenStruct {
        value: 126 as i32,
        len: 7 as i32,
    },
    Vp8TokenStruct {
        value: 127 as i32,
        len: 7 as i32,
    },
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
];
static Pcat1: [u8; 1] = [159 as u8];
static Pcat2: [u8; 2] = [165 as u8, 145 as u8];
static Pcat3: [u8; 3] = [173 as u8, 148 as u8, 140 as u8];
static Pcat4: [u8; 4] = [176 as u8, 155 as u8, 140 as u8, 135 as u8];
static Pcat5: [u8; 5] = [180 as u8, 157 as u8, 141 as u8, 134 as u8, 130 as u8];
static Pcat6: [u8; 11] = [
    254 as u8, 254 as u8, 243 as u8, 230 as u8, 196 as u8, 177 as u8, 153 as u8, 140 as u8,
    133 as u8, 130 as u8, 129 as u8,
];
static cat1: [i8; 2] = [0 as i8, 0 as i8];
static cat2: [i8; 4] = [2 as i8, 2 as i8, 0 as i8, 0 as i8];
static cat3: [i8; 6] = [2 as i8, 2 as i8, 4 as i8, 4 as i8, 0 as i8, 0 as i8];
static cat4: [i8; 8] = [
    2 as i8, 2 as i8, 4 as i8, 4 as i8, 6 as i8, 6 as i8, 0 as i8, 0 as i8,
];
static cat5: [i8; 10] = [
    2 as i8, 2 as i8, 4 as i8, 4 as i8, 6 as i8, 6 as i8, 8 as i8, 8 as i8, 0 as i8, 0 as i8,
];
static cat6: [i8; 22] = [
    2 as i8, 2 as i8, 4 as i8, 4 as i8, 6 as i8, 6 as i8, 8 as i8, 8 as i8, 10 as i8, 10 as i8,
    12 as i8, 12 as i8, 14 as i8, 14 as i8, 16 as i8, 16 as i8, 18 as i8, 18 as i8, 20 as i8,
    20 as i8, 0 as i8, 0 as i8,
];
#[unsafe(no_mangle)]
pub static vp8_extra_bits: [Vp8ExtraBitStruct; 12] = {
    [
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 0 as i32,
        },
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 1 as i32,
        },
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 2 as i32,
        },
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 3 as i32,
        },
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 4 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat1 as Vp8TreeP,
            prob: &raw const Pcat1 as *const u8,
            len: 1 as i32,
            base_val: 5 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat2 as Vp8TreeP,
            prob: &raw const Pcat2 as *const u8,
            len: 2 as i32,
            base_val: 7 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat3 as Vp8TreeP,
            prob: &raw const Pcat3 as *const u8,
            len: 3 as i32,
            base_val: 11 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat4 as Vp8TreeP,
            prob: &raw const Pcat4 as *const u8,
            len: 4 as i32,
            base_val: 19 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat5 as Vp8TreeP,
            prob: &raw const Pcat5 as *const u8,
            len: 5 as i32,
            base_val: 35 as i32,
        },
        Vp8ExtraBitStruct {
            tree: &raw const cat6 as Vp8TreeP,
            prob: &raw const Pcat6 as *const u8,
            len: 11 as i32,
            base_val: 67 as i32,
        },
        Vp8ExtraBitStruct {
            tree: ::core::ptr::null::<i8>(),
            prob: ::core::ptr::null::<u8>(),
            len: 0 as i32,
            base_val: 0 as i32,
        },
    ]
};
static default_coef_probs: [[[[u8; 11]; 3]; 8]; 4] = [
    [
        [
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                253 as u8, 136 as u8, 254 as u8, 255 as u8, 228 as u8, 219 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                189 as u8, 129 as u8, 242 as u8, 255 as u8, 227 as u8, 213 as u8, 255 as u8,
                219 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                106 as u8, 126 as u8, 227 as u8, 252 as u8, 214 as u8, 209 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 98 as u8, 248 as u8, 255 as u8, 236 as u8, 226 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                181 as u8, 133 as u8, 238 as u8, 254 as u8, 221 as u8, 234 as u8, 255 as u8,
                154 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                78 as u8, 134 as u8, 202 as u8, 247 as u8, 198 as u8, 180 as u8, 255 as u8,
                219 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 185 as u8, 249 as u8, 255 as u8, 243 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                184 as u8, 150 as u8, 247 as u8, 255 as u8, 236 as u8, 224 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                77 as u8, 110 as u8, 216 as u8, 255 as u8, 236 as u8, 230 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 101 as u8, 251 as u8, 255 as u8, 241 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                170 as u8, 139 as u8, 241 as u8, 252 as u8, 236 as u8, 209 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                37 as u8, 116 as u8, 196 as u8, 243 as u8, 228 as u8, 255 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 204 as u8, 254 as u8, 255 as u8, 245 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                207 as u8, 160 as u8, 250 as u8, 255 as u8, 238 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                102 as u8, 103 as u8, 231 as u8, 255 as u8, 211 as u8, 171 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 152 as u8, 252 as u8, 255 as u8, 240 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                177 as u8, 135 as u8, 243 as u8, 255 as u8, 234 as u8, 225 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                80 as u8, 129 as u8, 211 as u8, 255 as u8, 194 as u8, 224 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8,
            ],
            [
                246 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
    ],
    [
        [
            [
                198 as u8, 35 as u8, 237 as u8, 223 as u8, 193 as u8, 187 as u8, 162 as u8,
                160 as u8, 145 as u8, 155 as u8, 62 as u8,
            ],
            [
                131 as u8, 45 as u8, 198 as u8, 221 as u8, 172 as u8, 176 as u8, 220 as u8,
                157 as u8, 252 as u8, 221 as u8, 1 as u8,
            ],
            [
                68 as u8, 47 as u8, 146 as u8, 208 as u8, 149 as u8, 167 as u8, 221 as u8,
                162 as u8, 255 as u8, 223 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 149 as u8, 241 as u8, 255 as u8, 221 as u8, 224 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                184 as u8, 141 as u8, 234 as u8, 253 as u8, 222 as u8, 220 as u8, 255 as u8,
                199 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                81 as u8, 99 as u8, 181 as u8, 242 as u8, 176 as u8, 190 as u8, 249 as u8,
                202 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 129 as u8, 232 as u8, 253 as u8, 214 as u8, 197 as u8, 242 as u8,
                196 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
            [
                99 as u8, 121 as u8, 210 as u8, 250 as u8, 201 as u8, 198 as u8, 255 as u8,
                202 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                23 as u8, 91 as u8, 163 as u8, 242 as u8, 170 as u8, 187 as u8, 247 as u8,
                210 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 200 as u8, 246 as u8, 255 as u8, 234 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                109 as u8, 178 as u8, 241 as u8, 255 as u8, 231 as u8, 245 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                44 as u8, 130 as u8, 201 as u8, 253 as u8, 205 as u8, 192 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 132 as u8, 239 as u8, 251 as u8, 219 as u8, 209 as u8, 255 as u8,
                165 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                94 as u8, 136 as u8, 225 as u8, 251 as u8, 218 as u8, 190 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                22 as u8, 100 as u8, 174 as u8, 245 as u8, 186 as u8, 161 as u8, 255 as u8,
                199 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 182 as u8, 249 as u8, 255 as u8, 232 as u8, 235 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                124 as u8, 143 as u8, 241 as u8, 255 as u8, 227 as u8, 234 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                35 as u8, 77 as u8, 181 as u8, 251 as u8, 193 as u8, 211 as u8, 255 as u8,
                205 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 157 as u8, 247 as u8, 255 as u8, 236 as u8, 231 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                121 as u8, 141 as u8, 235 as u8, 255 as u8, 225 as u8, 227 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                45 as u8, 99 as u8, 188 as u8, 251 as u8, 195 as u8, 217 as u8, 255 as u8,
                224 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 1 as u8, 251 as u8, 255 as u8, 213 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8,
            ],
            [
                203 as u8, 1 as u8, 248 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                137 as u8, 1 as u8, 177 as u8, 255 as u8, 224 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
    ],
    [
        [
            [
                253 as u8, 9 as u8, 248 as u8, 251 as u8, 207 as u8, 208 as u8, 255 as u8,
                192 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                175 as u8, 13 as u8, 224 as u8, 243 as u8, 193 as u8, 185 as u8, 249 as u8,
                198 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
            [
                73 as u8, 17 as u8, 171 as u8, 221 as u8, 161 as u8, 179 as u8, 236 as u8,
                167 as u8, 255 as u8, 234 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 95 as u8, 247 as u8, 253 as u8, 212 as u8, 183 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                239 as u8, 90 as u8, 244 as u8, 250 as u8, 211 as u8, 209 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                155 as u8, 77 as u8, 195 as u8, 248 as u8, 188 as u8, 195 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 24 as u8, 239 as u8, 251 as u8, 218 as u8, 219 as u8, 255 as u8,
                205 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                201 as u8, 51 as u8, 219 as u8, 255 as u8, 196 as u8, 186 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                69 as u8, 46 as u8, 190 as u8, 239 as u8, 201 as u8, 218 as u8, 255 as u8,
                228 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 191 as u8, 251 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                223 as u8, 165 as u8, 249 as u8, 255 as u8, 213 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                141 as u8, 124 as u8, 248 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 16 as u8, 248 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                190 as u8, 36 as u8, 230 as u8, 255 as u8, 236 as u8, 255 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                149 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 226 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                247 as u8, 192 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                240 as u8, 128 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 134 as u8, 252 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                213 as u8, 62 as u8, 250 as u8, 255 as u8, 255 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                55 as u8, 93 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
    ],
    [
        [
            [
                202 as u8, 24 as u8, 213 as u8, 235 as u8, 186 as u8, 191 as u8, 220 as u8,
                160 as u8, 240 as u8, 175 as u8, 255 as u8,
            ],
            [
                126 as u8, 38 as u8, 182 as u8, 232 as u8, 169 as u8, 184 as u8, 228 as u8,
                174 as u8, 255 as u8, 187 as u8, 128 as u8,
            ],
            [
                61 as u8, 46 as u8, 138 as u8, 219 as u8, 151 as u8, 178 as u8, 240 as u8,
                170 as u8, 255 as u8, 216 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 112 as u8, 230 as u8, 250 as u8, 199 as u8, 191 as u8, 247 as u8,
                159 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
            [
                166 as u8, 109 as u8, 228 as u8, 252 as u8, 211 as u8, 215 as u8, 255 as u8,
                174 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                39 as u8, 77 as u8, 162 as u8, 232 as u8, 172 as u8, 180 as u8, 245 as u8,
                178 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 52 as u8, 220 as u8, 246 as u8, 198 as u8, 199 as u8, 249 as u8,
                220 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
            [
                124 as u8, 74 as u8, 191 as u8, 243 as u8, 183 as u8, 193 as u8, 250 as u8,
                221 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
            [
                24 as u8, 71 as u8, 130 as u8, 219 as u8, 154 as u8, 170 as u8, 243 as u8,
                182 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 182 as u8, 225 as u8, 249 as u8, 219 as u8, 240 as u8, 255 as u8,
                224 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                149 as u8, 150 as u8, 226 as u8, 252 as u8, 216 as u8, 205 as u8, 255 as u8,
                171 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                28 as u8, 108 as u8, 170 as u8, 242 as u8, 183 as u8, 194 as u8, 254 as u8,
                223 as u8, 255 as u8, 255 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 81 as u8, 230 as u8, 252 as u8, 204 as u8, 203 as u8, 255 as u8,
                192 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                123 as u8, 102 as u8, 209 as u8, 247 as u8, 188 as u8, 196 as u8, 255 as u8,
                233 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                20 as u8, 95 as u8, 153 as u8, 243 as u8, 164 as u8, 173 as u8, 255 as u8,
                203 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 222 as u8, 248 as u8, 255 as u8, 216 as u8, 213 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                168 as u8, 175 as u8, 246 as u8, 252 as u8, 235 as u8, 205 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                47 as u8, 116 as u8, 215 as u8, 255 as u8, 211 as u8, 212 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 121 as u8, 236 as u8, 253 as u8, 212 as u8, 214 as u8, 255 as u8,
                255 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                141 as u8, 84 as u8, 213 as u8, 252 as u8, 201 as u8, 202 as u8, 255 as u8,
                219 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                42 as u8, 80 as u8, 160 as u8, 240 as u8, 162 as u8, 185 as u8, 255 as u8,
                205 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
        [
            [
                1 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8,
            ],
            [
                244 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
            [
                238 as u8, 1 as u8, 255 as u8, 128 as u8, 128 as u8, 128 as u8, 128 as u8,
                128 as u8, 128 as u8, 128 as u8, 128 as u8,
            ],
        ],
    ],
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_default_coef_probs(mut pc: *mut Vp8Common) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const default_coef_probs as *const [[[u8; 11]; 3]; 8] as *const c_void
                as *const u8,
            &raw mut (*pc).fc.coef_probs as *mut [[[u8; 11]; 3]; 8] as *mut c_void as *mut u8,
            ::core::mem::size_of::<[[[[u8; 11]; 3]; 8]; 4]>() as usize,
        );
    }
}
