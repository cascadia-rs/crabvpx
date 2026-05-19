use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: u32,
    pub mv: IntMv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
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
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
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
pub type ModeInfo = Modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
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

pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const SIMPLE_LOOPFILTER: u32 = 1;
pub const NORMAL_LOOPFILTER: u32 = 0;
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
pub const RECON_CLAMP_NOTREQUIRED: u32 = 1;
pub const RECON_CLAMP_REQUIRED: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8TokenStruct {
    pub value: i32,
    pub len: i32,
}
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const ZEROMV: C2RustUnnamed = 7;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub type Vp8Common = VP8Common;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_0 = u32;
pub const SUBMVREF_LEFT_ABOVE_ZED: C2RustUnnamed_0 = 4;
pub const SUBMVREF_LEFT_ABOVE_SAME: C2RustUnnamed_0 = 3;
pub const SUBMVREF_ABOVE_ZED: C2RustUnnamed_0 = 2;
pub const SUBMVREF_LEFT_ZED: C2RustUnnamed_0 = 1;
pub const SUBMVREF_NORMAL: C2RustUnnamed_0 = 0;
pub type Vp8Mbsplit = [i32; 16];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_encodings: [Vp8TokenStruct; 10] = [
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
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
        value: 30 as i32,
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
        value: 62 as i32,
        len: 6 as i32,
    },
    Vp8TokenStruct {
        value: 126 as i32,
        len: 7 as i32,
    },
    Vp8TokenStruct {
        value: 127 as i32,
        len: 7 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_encodings: [Vp8TokenStruct; 5] = [
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
    Vp8TokenStruct {
        value: 4 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 5 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_encodings: [Vp8TokenStruct; 5] = [
    Vp8TokenStruct {
        value: 4 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 5 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_encodings: [Vp8TokenStruct; 4] = [
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
    Vp8TokenStruct {
        value: 2 as i32,
        len: 2 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_encodings: [Vp8TokenStruct; 4] = [
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 2 as i32,
        len: 2 as i32,
    },
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_encoding_array: [Vp8TokenStruct; 5] = [
    Vp8TokenStruct {
        value: 2 as i32,
        len: 2 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
    Vp8TokenStruct {
        value: 14 as i32,
        len: 4 as i32,
    },
    Vp8TokenStruct {
        value: 15 as i32,
        len: 4 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_encoding_array: [Vp8TokenStruct; 4] = [
    Vp8TokenStruct {
        value: 0 as i32,
        len: 1 as i32,
    },
    Vp8TokenStruct {
        value: 2 as i32,
        len: 2 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvencodings: [Vp8TokenStruct; 8] = [
    Vp8TokenStruct {
        value: 0 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 1 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 2 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 3 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 4 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 5 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 6 as i32,
        len: 3 as i32,
    },
    Vp8TokenStruct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_prob: [u8; 4] = [112 as u8, 86 as u8, 140 as u8, 37 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_prob: [u8; 4] = [145 as u8, 156 as u8, 163 as u8, 128 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_prob: [u8; 3] = [162 as u8, 101 as u8, 204 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_kf_uv_mode_prob: [u8; 3] = [142 as u8, 114 as u8, 183 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_prob: [u8; 9] = [
    120 as u8, 90 as u8, 79 as u8, 133 as u8, 87 as u8, 85 as u8, 80 as u8, 111 as u8, 151 as u8,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_bmode_prob: [[[u8; 9]; 10]; 10] = [
    [
        [
            231 as u8, 120 as u8, 48 as u8, 89 as u8, 115 as u8, 113 as u8, 120 as u8, 152 as u8,
            112 as u8,
        ],
        [
            152 as u8, 179 as u8, 64 as u8, 126 as u8, 170 as u8, 118 as u8, 46 as u8, 70 as u8,
            95 as u8,
        ],
        [
            175 as u8, 69 as u8, 143 as u8, 80 as u8, 85 as u8, 82 as u8, 72 as u8, 155 as u8,
            103 as u8,
        ],
        [
            56 as u8, 58 as u8, 10 as u8, 171 as u8, 218 as u8, 189 as u8, 17 as u8, 13 as u8,
            152 as u8,
        ],
        [
            144 as u8, 71 as u8, 10 as u8, 38 as u8, 171 as u8, 213 as u8, 144 as u8, 34 as u8,
            26 as u8,
        ],
        [
            114 as u8, 26 as u8, 17 as u8, 163 as u8, 44 as u8, 195 as u8, 21 as u8, 10 as u8,
            173 as u8,
        ],
        [
            121 as u8, 24 as u8, 80 as u8, 195 as u8, 26 as u8, 62 as u8, 44 as u8, 64 as u8,
            85 as u8,
        ],
        [
            170 as u8, 46 as u8, 55 as u8, 19 as u8, 136 as u8, 160 as u8, 33 as u8, 206 as u8,
            71 as u8,
        ],
        [
            63 as u8, 20 as u8, 8 as u8, 114 as u8, 114 as u8, 208 as u8, 12 as u8, 9 as u8,
            226 as u8,
        ],
        [
            81 as u8, 40 as u8, 11 as u8, 96 as u8, 182 as u8, 84 as u8, 29 as u8, 16 as u8,
            36 as u8,
        ],
    ],
    [
        [
            134 as u8, 183 as u8, 89 as u8, 137 as u8, 98 as u8, 101 as u8, 106 as u8, 165 as u8,
            148 as u8,
        ],
        [
            72 as u8, 187 as u8, 100 as u8, 130 as u8, 157 as u8, 111 as u8, 32 as u8, 75 as u8,
            80 as u8,
        ],
        [
            66 as u8, 102 as u8, 167 as u8, 99 as u8, 74 as u8, 62 as u8, 40 as u8, 234 as u8,
            128 as u8,
        ],
        [
            41 as u8, 53 as u8, 9 as u8, 178 as u8, 241 as u8, 141 as u8, 26 as u8, 8 as u8,
            107 as u8,
        ],
        [
            104 as u8, 79 as u8, 12 as u8, 27 as u8, 217 as u8, 255 as u8, 87 as u8, 17 as u8,
            7 as u8,
        ],
        [
            74 as u8, 43 as u8, 26 as u8, 146 as u8, 73 as u8, 166 as u8, 49 as u8, 23 as u8,
            157 as u8,
        ],
        [
            65 as u8, 38 as u8, 105 as u8, 160 as u8, 51 as u8, 52 as u8, 31 as u8, 115 as u8,
            128 as u8,
        ],
        [
            87 as u8, 68 as u8, 71 as u8, 44 as u8, 114 as u8, 51 as u8, 15 as u8, 186 as u8,
            23 as u8,
        ],
        [
            47 as u8, 41 as u8, 14 as u8, 110 as u8, 182 as u8, 183 as u8, 21 as u8, 17 as u8,
            194 as u8,
        ],
        [
            66 as u8, 45 as u8, 25 as u8, 102 as u8, 197 as u8, 189 as u8, 23 as u8, 18 as u8,
            22 as u8,
        ],
    ],
    [
        [
            88 as u8, 88 as u8, 147 as u8, 150 as u8, 42 as u8, 46 as u8, 45 as u8, 196 as u8,
            205 as u8,
        ],
        [
            43 as u8, 97 as u8, 183 as u8, 117 as u8, 85 as u8, 38 as u8, 35 as u8, 179 as u8,
            61 as u8,
        ],
        [
            39 as u8, 53 as u8, 200 as u8, 87 as u8, 26 as u8, 21 as u8, 43 as u8, 232 as u8,
            171 as u8,
        ],
        [
            56 as u8, 34 as u8, 51 as u8, 104 as u8, 114 as u8, 102 as u8, 29 as u8, 93 as u8,
            77 as u8,
        ],
        [
            107 as u8, 54 as u8, 32 as u8, 26 as u8, 51 as u8, 1 as u8, 81 as u8, 43 as u8,
            31 as u8,
        ],
        [
            39 as u8, 28 as u8, 85 as u8, 171 as u8, 58 as u8, 165 as u8, 90 as u8, 98 as u8,
            64 as u8,
        ],
        [
            34 as u8, 22 as u8, 116 as u8, 206 as u8, 23 as u8, 34 as u8, 43 as u8, 166 as u8,
            73 as u8,
        ],
        [
            68 as u8, 25 as u8, 106 as u8, 22 as u8, 64 as u8, 171 as u8, 36 as u8, 225 as u8,
            114 as u8,
        ],
        [
            34 as u8, 19 as u8, 21 as u8, 102 as u8, 132 as u8, 188 as u8, 16 as u8, 76 as u8,
            124 as u8,
        ],
        [
            62 as u8, 18 as u8, 78 as u8, 95 as u8, 85 as u8, 57 as u8, 50 as u8, 48 as u8,
            51 as u8,
        ],
    ],
    [
        [
            193 as u8, 101 as u8, 35 as u8, 159 as u8, 215 as u8, 111 as u8, 89 as u8, 46 as u8,
            111 as u8,
        ],
        [
            60 as u8, 148 as u8, 31 as u8, 172 as u8, 219 as u8, 228 as u8, 21 as u8, 18 as u8,
            111 as u8,
        ],
        [
            112 as u8, 113 as u8, 77 as u8, 85 as u8, 179 as u8, 255 as u8, 38 as u8, 120 as u8,
            114 as u8,
        ],
        [
            40 as u8, 42 as u8, 1 as u8, 196 as u8, 245 as u8, 209 as u8, 10 as u8, 25 as u8,
            109 as u8,
        ],
        [
            100 as u8, 80 as u8, 8 as u8, 43 as u8, 154 as u8, 1 as u8, 51 as u8, 26 as u8,
            71 as u8,
        ],
        [
            88 as u8, 43 as u8, 29 as u8, 140 as u8, 166 as u8, 213 as u8, 37 as u8, 43 as u8,
            154 as u8,
        ],
        [
            61 as u8, 63 as u8, 30 as u8, 155 as u8, 67 as u8, 45 as u8, 68 as u8, 1 as u8,
            209 as u8,
        ],
        [
            142 as u8, 78 as u8, 78 as u8, 16 as u8, 255 as u8, 128 as u8, 34 as u8, 197 as u8,
            171 as u8,
        ],
        [
            41 as u8, 40 as u8, 5 as u8, 102 as u8, 211 as u8, 183 as u8, 4 as u8, 1 as u8,
            221 as u8,
        ],
        [
            51 as u8, 50 as u8, 17 as u8, 168 as u8, 209 as u8, 192 as u8, 23 as u8, 25 as u8,
            82 as u8,
        ],
    ],
    [
        [
            125 as u8, 98 as u8, 42 as u8, 88 as u8, 104 as u8, 85 as u8, 117 as u8, 175 as u8,
            82 as u8,
        ],
        [
            95 as u8, 84 as u8, 53 as u8, 89 as u8, 128 as u8, 100 as u8, 113 as u8, 101 as u8,
            45 as u8,
        ],
        [
            75 as u8, 79 as u8, 123 as u8, 47 as u8, 51 as u8, 128 as u8, 81 as u8, 171 as u8,
            1 as u8,
        ],
        [
            57 as u8, 17 as u8, 5 as u8, 71 as u8, 102 as u8, 57 as u8, 53 as u8, 41 as u8,
            49 as u8,
        ],
        [
            115 as u8, 21 as u8, 2 as u8, 10 as u8, 102 as u8, 255 as u8, 166 as u8, 23 as u8,
            6 as u8,
        ],
        [
            38 as u8, 33 as u8, 13 as u8, 121 as u8, 57 as u8, 73 as u8, 26 as u8, 1 as u8,
            85 as u8,
        ],
        [
            41 as u8, 10 as u8, 67 as u8, 138 as u8, 77 as u8, 110 as u8, 90 as u8, 47 as u8,
            114 as u8,
        ],
        [
            101 as u8, 29 as u8, 16 as u8, 10 as u8, 85 as u8, 128 as u8, 101 as u8, 196 as u8,
            26 as u8,
        ],
        [
            57 as u8, 18 as u8, 10 as u8, 102 as u8, 102 as u8, 213 as u8, 34 as u8, 20 as u8,
            43 as u8,
        ],
        [
            117 as u8, 20 as u8, 15 as u8, 36 as u8, 163 as u8, 128 as u8, 68 as u8, 1 as u8,
            26 as u8,
        ],
    ],
    [
        [
            138 as u8, 31 as u8, 36 as u8, 171 as u8, 27 as u8, 166 as u8, 38 as u8, 44 as u8,
            229 as u8,
        ],
        [
            67 as u8, 87 as u8, 58 as u8, 169 as u8, 82 as u8, 115 as u8, 26 as u8, 59 as u8,
            179 as u8,
        ],
        [
            63 as u8, 59 as u8, 90 as u8, 180 as u8, 59 as u8, 166 as u8, 93 as u8, 73 as u8,
            154 as u8,
        ],
        [
            40 as u8, 40 as u8, 21 as u8, 116 as u8, 143 as u8, 209 as u8, 34 as u8, 39 as u8,
            175 as u8,
        ],
        [
            57 as u8, 46 as u8, 22 as u8, 24 as u8, 128 as u8, 1 as u8, 54 as u8, 17 as u8,
            37 as u8,
        ],
        [
            47 as u8, 15 as u8, 16 as u8, 183 as u8, 34 as u8, 223 as u8, 49 as u8, 45 as u8,
            183 as u8,
        ],
        [
            46 as u8, 17 as u8, 33 as u8, 183 as u8, 6 as u8, 98 as u8, 15 as u8, 32 as u8,
            183 as u8,
        ],
        [
            65 as u8, 32 as u8, 73 as u8, 115 as u8, 28 as u8, 128 as u8, 23 as u8, 128 as u8,
            205 as u8,
        ],
        [
            40 as u8, 3 as u8, 9 as u8, 115 as u8, 51 as u8, 192 as u8, 18 as u8, 6 as u8,
            223 as u8,
        ],
        [
            87 as u8, 37 as u8, 9 as u8, 115 as u8, 59 as u8, 77 as u8, 64 as u8, 21 as u8,
            47 as u8,
        ],
    ],
    [
        [
            104 as u8, 55 as u8, 44 as u8, 218 as u8, 9 as u8, 54 as u8, 53 as u8, 130 as u8,
            226 as u8,
        ],
        [
            64 as u8, 90 as u8, 70 as u8, 205 as u8, 40 as u8, 41 as u8, 23 as u8, 26 as u8,
            57 as u8,
        ],
        [
            54 as u8, 57 as u8, 112 as u8, 184 as u8, 5 as u8, 41 as u8, 38 as u8, 166 as u8,
            213 as u8,
        ],
        [
            30 as u8, 34 as u8, 26 as u8, 133 as u8, 152 as u8, 116 as u8, 10 as u8, 32 as u8,
            134 as u8,
        ],
        [
            75 as u8, 32 as u8, 12 as u8, 51 as u8, 192 as u8, 255 as u8, 160 as u8, 43 as u8,
            51 as u8,
        ],
        [
            39 as u8, 19 as u8, 53 as u8, 221 as u8, 26 as u8, 114 as u8, 32 as u8, 73 as u8,
            255 as u8,
        ],
        [
            31 as u8, 9 as u8, 65 as u8, 234 as u8, 2 as u8, 15 as u8, 1 as u8, 118 as u8, 73 as u8,
        ],
        [
            88 as u8, 31 as u8, 35 as u8, 67 as u8, 102 as u8, 85 as u8, 55 as u8, 186 as u8,
            85 as u8,
        ],
        [
            56 as u8, 21 as u8, 23 as u8, 111 as u8, 59 as u8, 205 as u8, 45 as u8, 37 as u8,
            192 as u8,
        ],
        [
            55 as u8, 38 as u8, 70 as u8, 124 as u8, 73 as u8, 102 as u8, 1 as u8, 34 as u8,
            98 as u8,
        ],
    ],
    [
        [
            102 as u8, 61 as u8, 71 as u8, 37 as u8, 34 as u8, 53 as u8, 31 as u8, 243 as u8,
            192 as u8,
        ],
        [
            69 as u8, 60 as u8, 71 as u8, 38 as u8, 73 as u8, 119 as u8, 28 as u8, 222 as u8,
            37 as u8,
        ],
        [
            68 as u8, 45 as u8, 128 as u8, 34 as u8, 1 as u8, 47 as u8, 11 as u8, 245 as u8,
            171 as u8,
        ],
        [
            62 as u8, 17 as u8, 19 as u8, 70 as u8, 146 as u8, 85 as u8, 55 as u8, 62 as u8,
            70 as u8,
        ],
        [
            75 as u8, 15 as u8, 9 as u8, 9 as u8, 64 as u8, 255 as u8, 184 as u8, 119 as u8,
            16 as u8,
        ],
        [
            37 as u8, 43 as u8, 37 as u8, 154 as u8, 100 as u8, 163 as u8, 85 as u8, 160 as u8,
            1 as u8,
        ],
        [
            63 as u8, 9 as u8, 92 as u8, 136 as u8, 28 as u8, 64 as u8, 32 as u8, 201 as u8,
            85 as u8,
        ],
        [
            86 as u8, 6 as u8, 28 as u8, 5 as u8, 64 as u8, 255 as u8, 25 as u8, 248 as u8, 1 as u8,
        ],
        [
            56 as u8, 8 as u8, 17 as u8, 132 as u8, 137 as u8, 255 as u8, 55 as u8, 116 as u8,
            128 as u8,
        ],
        [
            58 as u8, 15 as u8, 20 as u8, 82 as u8, 135 as u8, 57 as u8, 26 as u8, 121 as u8,
            40 as u8,
        ],
    ],
    [
        [
            164 as u8, 50 as u8, 31 as u8, 137 as u8, 154 as u8, 133 as u8, 25 as u8, 35 as u8,
            218 as u8,
        ],
        [
            51 as u8, 103 as u8, 44 as u8, 131 as u8, 131 as u8, 123 as u8, 31 as u8, 6 as u8,
            158 as u8,
        ],
        [
            86 as u8, 40 as u8, 64 as u8, 135 as u8, 148 as u8, 224 as u8, 45 as u8, 183 as u8,
            128 as u8,
        ],
        [
            22 as u8, 26 as u8, 17 as u8, 131 as u8, 240 as u8, 154 as u8, 14 as u8, 1 as u8,
            209 as u8,
        ],
        [
            83 as u8, 12 as u8, 13 as u8, 54 as u8, 192 as u8, 255 as u8, 68 as u8, 47 as u8,
            28 as u8,
        ],
        [
            45 as u8, 16 as u8, 21 as u8, 91 as u8, 64 as u8, 222 as u8, 7 as u8, 1 as u8,
            197 as u8,
        ],
        [
            56 as u8, 21 as u8, 39 as u8, 155 as u8, 60 as u8, 138 as u8, 23 as u8, 102 as u8,
            213 as u8,
        ],
        [
            85 as u8, 26 as u8, 85 as u8, 85 as u8, 128 as u8, 128 as u8, 32 as u8, 146 as u8,
            171 as u8,
        ],
        [
            18 as u8, 11 as u8, 7 as u8, 63 as u8, 144 as u8, 171 as u8, 4 as u8, 4 as u8,
            246 as u8,
        ],
        [
            35 as u8, 27 as u8, 10 as u8, 146 as u8, 174 as u8, 171 as u8, 12 as u8, 26 as u8,
            128 as u8,
        ],
    ],
    [
        [
            190 as u8, 80 as u8, 35 as u8, 99 as u8, 180 as u8, 80 as u8, 126 as u8, 54 as u8,
            45 as u8,
        ],
        [
            85 as u8, 126 as u8, 47 as u8, 87 as u8, 176 as u8, 51 as u8, 41 as u8, 20 as u8,
            32 as u8,
        ],
        [
            101 as u8, 75 as u8, 128 as u8, 139 as u8, 118 as u8, 146 as u8, 116 as u8, 128 as u8,
            85 as u8,
        ],
        [
            56 as u8, 41 as u8, 15 as u8, 176 as u8, 236 as u8, 85 as u8, 37 as u8, 9 as u8,
            62 as u8,
        ],
        [
            146 as u8, 36 as u8, 19 as u8, 30 as u8, 171 as u8, 255 as u8, 97 as u8, 27 as u8,
            20 as u8,
        ],
        [
            71 as u8, 30 as u8, 17 as u8, 119 as u8, 118 as u8, 255 as u8, 17 as u8, 18 as u8,
            138 as u8,
        ],
        [
            101 as u8, 38 as u8, 60 as u8, 138 as u8, 55 as u8, 70 as u8, 43 as u8, 26 as u8,
            142 as u8,
        ],
        [
            138 as u8, 45 as u8, 61 as u8, 62 as u8, 219 as u8, 1 as u8, 81 as u8, 188 as u8,
            64 as u8,
        ],
        [
            32 as u8, 41 as u8, 20 as u8, 117 as u8, 151 as u8, 142 as u8, 20 as u8, 21 as u8,
            163 as u8,
        ],
        [
            112 as u8, 19 as u8, 12 as u8, 61 as u8, 195 as u8, 128 as u8, 48 as u8, 4 as u8,
            24 as u8,
        ],
    ],
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_mv_cont(mut l: *const IntMv, mut a: *const IntMv) -> i32 {
    unsafe {
        let mut lez: i32 = ((*l).as_int == 0 as u32) as i32;
        let mut aez: i32 = ((*a).as_int == 0 as u32) as i32;
        let mut lea: i32 = ((*l).as_int == (*a).as_int) as i32;
        if lea != 0 && lez != 0 {
            return SUBMVREF_LEFT_ABOVE_ZED as i32;
        }
        if lea != 0 {
            return SUBMVREF_LEFT_ABOVE_SAME as i32;
        }
        if aez != 0 {
            return SUBMVREF_ABOVE_ZED as i32;
        }
        if lez != 0 {
            return SUBMVREF_LEFT_ZED as i32;
        }
        SUBMVREF_NORMAL as i32
    }
}
static mut sub_mv_ref_prob: [u8; 3] = [180 as u8, 162 as u8, 25 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_prob2: [[u8; 3]; 5] = [
    [147 as u8, 136 as u8, 18 as u8],
    [106 as u8, 145 as u8, 1 as u8],
    [179 as u8, 121 as u8, 1 as u8],
    [223 as u8, 1 as u8, 34 as u8],
    [208 as u8, 1 as u8, 1 as u8],
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplits: [Vp8Mbsplit; 4] = [
    [
        0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 1 as i32,
        1 as i32, 1 as i32, 1 as i32, 1 as i32, 1 as i32, 1 as i32, 1 as i32,
    ],
    [
        0 as i32, 0 as i32, 1 as i32, 1 as i32, 0 as i32, 0 as i32, 1 as i32, 1 as i32, 0 as i32,
        0 as i32, 1 as i32, 1 as i32, 0 as i32, 0 as i32, 1 as i32, 1 as i32,
    ],
    [
        0 as i32, 0 as i32, 1 as i32, 1 as i32, 0 as i32, 0 as i32, 1 as i32, 1 as i32, 2 as i32,
        2 as i32, 3 as i32, 3 as i32, 2 as i32, 2 as i32, 3 as i32, 3 as i32,
    ],
    [
        0 as i32, 1 as i32, 2 as i32, 3 as i32, 4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32,
        9 as i32, 10 as i32, 11 as i32, 12 as i32, 13 as i32, 14 as i32, 15 as i32,
    ],
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_count: [i32; 4] = [2 as i32, 2 as i32, 4 as i32, 16 as i32];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_probs: [u8; 3] = [110 as u8, 111 as u8, 150 as u8];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_tree: [i8; 18] = [
    -(B_DC_PRED as i32) as i8,
    2 as i8,
    -(B_TM_PRED as i32) as i8,
    4 as i8,
    -(B_VE_PRED as i32) as i8,
    6 as i8,
    8 as i8,
    12 as i8,
    -(B_HE_PRED as i32) as i8,
    10 as i8,
    -(B_RD_PRED as i32) as i8,
    -(B_VR_PRED as i32) as i8,
    -(B_LD_PRED as i32) as i8,
    14 as i8,
    -(B_VL_PRED as i32) as i8,
    16 as i8,
    -(B_HD_PRED as i32) as i8,
    -(B_HU_PRED as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_tree: [i8; 8] = [
    -(DC_PRED as i32) as i8,
    2 as i8,
    4 as i8,
    6 as i8,
    -(V_PRED as i32) as i8,
    -(H_PRED as i32) as i8,
    -(TM_PRED as i32) as i8,
    -(B_PRED as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_tree: [i8; 8] = [
    -(B_PRED as i32) as i8,
    2 as i8,
    4 as i8,
    6 as i8,
    -(DC_PRED as i32) as i8,
    -(V_PRED as i32) as i8,
    -(H_PRED as i32) as i8,
    -(TM_PRED as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_tree: [i8; 6] = [
    -(DC_PRED as i32) as i8,
    2 as i8,
    -(V_PRED as i32) as i8,
    4 as i8,
    -(H_PRED as i32) as i8,
    -(TM_PRED as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_tree: [i8; 6] = [
    -(3 as i32) as i8,
    2 as i8,
    -(2 as i32) as i8,
    4 as i8,
    -(0 as i32) as i8,
    -(1 as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_tree: [i8; 8] = [
    -(ZEROMV as i32) as i8,
    2 as i8,
    -(NEARESTMV as i32) as i8,
    4 as i8,
    -(NEARMV as i32) as i8,
    6 as i8,
    -(NEWMV as i32) as i8,
    -(SPLITMV as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_tree: [i8; 6] = [
    -(LEFT4X4 as i32) as i8,
    2 as i8,
    -(ABOVE4X4 as i32) as i8,
    4 as i8,
    -(ZERO4X4 as i32) as i8,
    -(NEW4X4 as i32) as i8,
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvtree: [i8; 14] = [
    2 as i8,
    8 as i8,
    4 as i8,
    6 as i8,
    -(0 as i32) as i8,
    -(1 as i32) as i8,
    -(2 as i32) as i8,
    -(3 as i32) as i8,
    10 as i8,
    12 as i8,
    -(4 as i32) as i8,
    -(5 as i32) as i8,
    -(6 as i32) as i8,
    -(7 as i32) as i8,
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_mbmode_probs(mut x: *mut Vp8Common) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_ymode_prob as *const u8 as *const c_void as *const u8,
            &raw mut (*x).fc.ymode_prob as *mut u8 as *mut c_void as *mut u8,
            ::core::mem::size_of::<[u8; 4]>() as usize,
        );
        core::ptr::copy_nonoverlapping(
            &raw const vp8_uv_mode_prob as *const u8 as *const c_void as *const u8,
            &raw mut (*x).fc.uv_mode_prob as *mut u8 as *mut c_void as *mut u8,
            ::core::mem::size_of::<[u8; 3]>() as usize,
        );
        core::ptr::copy_nonoverlapping(
            &raw const sub_mv_ref_prob as *const u8 as *const c_void as *const u8,
            &raw mut (*x).fc.sub_mv_ref_prob as *mut u8 as *mut c_void as *mut u8,
            ::core::mem::size_of::<[u8; 3]>() as usize,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_default_bmode_probs(mut dest: *mut u8) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_bmode_prob as *const u8 as *const c_void as *const u8,
            dest as *mut c_void as *mut u8,
            ::core::mem::size_of::<[u8; 9]>() as usize,
        );
    }
}
