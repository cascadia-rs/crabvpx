use std::ffi::c_void;
unsafe extern "Rust" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: BPredictionMode,
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
pub type BPredictionMode = u32;
pub const B_MODE_COUNT: BPredictionMode = 14;
pub const NEW4X4: BPredictionMode = 13;
pub const ZERO4X4: BPredictionMode = 12;
pub const ABOVE4X4: BPredictionMode = 11;
pub const LEFT4X4: BPredictionMode = 10;
pub const B_HU_PRED: BPredictionMode = 9;
pub const B_HD_PRED: BPredictionMode = 8;
pub const B_VL_PRED: BPredictionMode = 7;
pub const B_VR_PRED: BPredictionMode = 6;
pub const B_RD_PRED: BPredictionMode = 5;
pub const B_LD_PRED: BPredictionMode = 4;
pub const B_HE_PRED: BPredictionMode = 3;
pub const B_VE_PRED: BPredictionMode = 2;
pub const B_TM_PRED: BPredictionMode = 1;
pub const B_DC_PRED: BPredictionMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
pub type VpxCodecErrT = u32;
pub const VPX_CODEC_LIST_END: VpxCodecErrT = 9;
pub const VPX_CODEC_INVALID_PARAM: VpxCodecErrT = 8;
pub const VPX_CODEC_CORRUPT_FRAME: VpxCodecErrT = 7;
pub const VPX_CODEC_UNSUP_FEATURE: VpxCodecErrT = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: VpxCodecErrT = 5;
pub const VPX_CODEC_INCAPABLE: VpxCodecErrT = 4;
pub const VPX_CODEC_ABI_MISMATCH: VpxCodecErrT = 3;
pub const VPX_CODEC_MEM_ERROR: VpxCodecErrT = 2;
pub const VPX_CODEC_ERROR: VpxCodecErrT = 1;
pub const VPX_CODEC_OK: VpxCodecErrT = 0;
pub type Vp8Prob = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type EntropyContext = i8;
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: SizeT,
    pub border: i32,
    pub frame_size: SizeT,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: VpxColorSpaceT,
    pub color_range: VpxColorRangeT,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type VpxColorRangeT = VpxColorRange;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type SizeT = DarwinSizeT;
pub type DarwinSizeT = usize;
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
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
    pub clamp_type: ClampType,
    pub frame_to_show: *mut Yv12BufferConfig,
    pub yv12_fb: [Yv12BufferConfig; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: Yv12BufferConfig,
    pub last_frame_type: FrameType,
    pub frame_type: FrameType,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub mbs: i32,
    pub mb_rows: i32,
    pub mb_cols: i32,
    pub mode_info_stride: i32,
    pub mb_no_coeff_skip: i32,
    pub no_lpf: i32,
    pub use_bilinear_mc_filter: i32,
    pub full_pixel: i32,
    pub base_qindex: i32,
    pub y1dc_delta_q: i32,
    pub y2dc_delta_q: i32,
    pub y2ac_delta_q: i32,
    pub uvdc_delta_q: i32,
    pub uvac_delta_q: i32,
    pub mip: *mut ModeInfo,
    pub mi: *mut ModeInfo,
    pub show_frame_mi: *mut ModeInfo,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: LoopFilterInfoN,
    pub filter_level: i32,
    pub last_sharpness_level: i32,
    pub sharpness_level: i32,
    pub refresh_last_frame: i32,
    pub refresh_golden_frame: i32,
    pub refresh_alt_ref_frame: i32,
    pub copy_buffer_to_gf: i32,
    pub copy_buffer_to_arf: i32,
    pub refresh_entropy_probs: i32,
    pub ref_frame_sign_bias: [i32; 4],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: EntropyContextPlanes,
    pub lfc: FrameContext,
    pub fc: FrameContext,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: TokenPartition,
    pub processor_core_count: i32,
}
pub type TokenPartition = u32;
pub const EIGHT_PARTITION: TokenPartition = 3;
pub const FOUR_PARTITION: TokenPartition = 2;
pub const TWO_PARTITION: TokenPartition = 1;
pub const ONE_PARTITION: TokenPartition = 0;
pub type FrameContext = FrameContexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameContexts {
    pub bmode_prob: [Vp8Prob; 9],
    pub ymode_prob: [Vp8Prob; 4],
    pub uv_mode_prob: [Vp8Prob; 3],
    pub sub_mv_ref_prob: [Vp8Prob; 3],
    pub coef_probs: [[[[Vp8Prob; 11]; 3]; 8]; 4],
    pub mvc: [MvContext; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [Vp8Prob; 19],
}
pub type ClampType = u32;
pub const RECON_CLAMP_NOTREQUIRED: ClampType = 1;
pub const RECON_CLAMP_REQUIRED: ClampType = 0;
pub type Vp8TreeIndex = i8;
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
pub static mut vp8_ymode_prob: [Vp8Prob; 4] =
    [112 as Vp8Prob, 86 as Vp8Prob, 140 as Vp8Prob, 37 as Vp8Prob];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_prob: [Vp8Prob; 4] = [
    145 as Vp8Prob,
    156 as Vp8Prob,
    163 as Vp8Prob,
    128 as Vp8Prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_prob: [Vp8Prob; 3] = [162 as Vp8Prob, 101 as Vp8Prob, 204 as Vp8Prob];
#[unsafe(no_mangle)]
pub static mut vp8_kf_uv_mode_prob: [Vp8Prob; 3] = [142 as Vp8Prob, 114 as Vp8Prob, 183 as Vp8Prob];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_prob: [Vp8Prob; 9] = [
    120 as Vp8Prob,
    90 as Vp8Prob,
    79 as Vp8Prob,
    133 as Vp8Prob,
    87 as Vp8Prob,
    85 as Vp8Prob,
    80 as Vp8Prob,
    111 as Vp8Prob,
    151 as Vp8Prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_bmode_prob: [[[Vp8Prob; 9]; 10]; 10] = [
    [
        [
            231 as Vp8Prob,
            120 as Vp8Prob,
            48 as Vp8Prob,
            89 as Vp8Prob,
            115 as Vp8Prob,
            113 as Vp8Prob,
            120 as Vp8Prob,
            152 as Vp8Prob,
            112 as Vp8Prob,
        ],
        [
            152 as Vp8Prob,
            179 as Vp8Prob,
            64 as Vp8Prob,
            126 as Vp8Prob,
            170 as Vp8Prob,
            118 as Vp8Prob,
            46 as Vp8Prob,
            70 as Vp8Prob,
            95 as Vp8Prob,
        ],
        [
            175 as Vp8Prob,
            69 as Vp8Prob,
            143 as Vp8Prob,
            80 as Vp8Prob,
            85 as Vp8Prob,
            82 as Vp8Prob,
            72 as Vp8Prob,
            155 as Vp8Prob,
            103 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            58 as Vp8Prob,
            10 as Vp8Prob,
            171 as Vp8Prob,
            218 as Vp8Prob,
            189 as Vp8Prob,
            17 as Vp8Prob,
            13 as Vp8Prob,
            152 as Vp8Prob,
        ],
        [
            144 as Vp8Prob,
            71 as Vp8Prob,
            10 as Vp8Prob,
            38 as Vp8Prob,
            171 as Vp8Prob,
            213 as Vp8Prob,
            144 as Vp8Prob,
            34 as Vp8Prob,
            26 as Vp8Prob,
        ],
        [
            114 as Vp8Prob,
            26 as Vp8Prob,
            17 as Vp8Prob,
            163 as Vp8Prob,
            44 as Vp8Prob,
            195 as Vp8Prob,
            21 as Vp8Prob,
            10 as Vp8Prob,
            173 as Vp8Prob,
        ],
        [
            121 as Vp8Prob,
            24 as Vp8Prob,
            80 as Vp8Prob,
            195 as Vp8Prob,
            26 as Vp8Prob,
            62 as Vp8Prob,
            44 as Vp8Prob,
            64 as Vp8Prob,
            85 as Vp8Prob,
        ],
        [
            170 as Vp8Prob,
            46 as Vp8Prob,
            55 as Vp8Prob,
            19 as Vp8Prob,
            136 as Vp8Prob,
            160 as Vp8Prob,
            33 as Vp8Prob,
            206 as Vp8Prob,
            71 as Vp8Prob,
        ],
        [
            63 as Vp8Prob,
            20 as Vp8Prob,
            8 as Vp8Prob,
            114 as Vp8Prob,
            114 as Vp8Prob,
            208 as Vp8Prob,
            12 as Vp8Prob,
            9 as Vp8Prob,
            226 as Vp8Prob,
        ],
        [
            81 as Vp8Prob,
            40 as Vp8Prob,
            11 as Vp8Prob,
            96 as Vp8Prob,
            182 as Vp8Prob,
            84 as Vp8Prob,
            29 as Vp8Prob,
            16 as Vp8Prob,
            36 as Vp8Prob,
        ],
    ],
    [
        [
            134 as Vp8Prob,
            183 as Vp8Prob,
            89 as Vp8Prob,
            137 as Vp8Prob,
            98 as Vp8Prob,
            101 as Vp8Prob,
            106 as Vp8Prob,
            165 as Vp8Prob,
            148 as Vp8Prob,
        ],
        [
            72 as Vp8Prob,
            187 as Vp8Prob,
            100 as Vp8Prob,
            130 as Vp8Prob,
            157 as Vp8Prob,
            111 as Vp8Prob,
            32 as Vp8Prob,
            75 as Vp8Prob,
            80 as Vp8Prob,
        ],
        [
            66 as Vp8Prob,
            102 as Vp8Prob,
            167 as Vp8Prob,
            99 as Vp8Prob,
            74 as Vp8Prob,
            62 as Vp8Prob,
            40 as Vp8Prob,
            234 as Vp8Prob,
            128 as Vp8Prob,
        ],
        [
            41 as Vp8Prob,
            53 as Vp8Prob,
            9 as Vp8Prob,
            178 as Vp8Prob,
            241 as Vp8Prob,
            141 as Vp8Prob,
            26 as Vp8Prob,
            8 as Vp8Prob,
            107 as Vp8Prob,
        ],
        [
            104 as Vp8Prob,
            79 as Vp8Prob,
            12 as Vp8Prob,
            27 as Vp8Prob,
            217 as Vp8Prob,
            255 as Vp8Prob,
            87 as Vp8Prob,
            17 as Vp8Prob,
            7 as Vp8Prob,
        ],
        [
            74 as Vp8Prob,
            43 as Vp8Prob,
            26 as Vp8Prob,
            146 as Vp8Prob,
            73 as Vp8Prob,
            166 as Vp8Prob,
            49 as Vp8Prob,
            23 as Vp8Prob,
            157 as Vp8Prob,
        ],
        [
            65 as Vp8Prob,
            38 as Vp8Prob,
            105 as Vp8Prob,
            160 as Vp8Prob,
            51 as Vp8Prob,
            52 as Vp8Prob,
            31 as Vp8Prob,
            115 as Vp8Prob,
            128 as Vp8Prob,
        ],
        [
            87 as Vp8Prob,
            68 as Vp8Prob,
            71 as Vp8Prob,
            44 as Vp8Prob,
            114 as Vp8Prob,
            51 as Vp8Prob,
            15 as Vp8Prob,
            186 as Vp8Prob,
            23 as Vp8Prob,
        ],
        [
            47 as Vp8Prob,
            41 as Vp8Prob,
            14 as Vp8Prob,
            110 as Vp8Prob,
            182 as Vp8Prob,
            183 as Vp8Prob,
            21 as Vp8Prob,
            17 as Vp8Prob,
            194 as Vp8Prob,
        ],
        [
            66 as Vp8Prob,
            45 as Vp8Prob,
            25 as Vp8Prob,
            102 as Vp8Prob,
            197 as Vp8Prob,
            189 as Vp8Prob,
            23 as Vp8Prob,
            18 as Vp8Prob,
            22 as Vp8Prob,
        ],
    ],
    [
        [
            88 as Vp8Prob,
            88 as Vp8Prob,
            147 as Vp8Prob,
            150 as Vp8Prob,
            42 as Vp8Prob,
            46 as Vp8Prob,
            45 as Vp8Prob,
            196 as Vp8Prob,
            205 as Vp8Prob,
        ],
        [
            43 as Vp8Prob,
            97 as Vp8Prob,
            183 as Vp8Prob,
            117 as Vp8Prob,
            85 as Vp8Prob,
            38 as Vp8Prob,
            35 as Vp8Prob,
            179 as Vp8Prob,
            61 as Vp8Prob,
        ],
        [
            39 as Vp8Prob,
            53 as Vp8Prob,
            200 as Vp8Prob,
            87 as Vp8Prob,
            26 as Vp8Prob,
            21 as Vp8Prob,
            43 as Vp8Prob,
            232 as Vp8Prob,
            171 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            34 as Vp8Prob,
            51 as Vp8Prob,
            104 as Vp8Prob,
            114 as Vp8Prob,
            102 as Vp8Prob,
            29 as Vp8Prob,
            93 as Vp8Prob,
            77 as Vp8Prob,
        ],
        [
            107 as Vp8Prob,
            54 as Vp8Prob,
            32 as Vp8Prob,
            26 as Vp8Prob,
            51 as Vp8Prob,
            1 as Vp8Prob,
            81 as Vp8Prob,
            43 as Vp8Prob,
            31 as Vp8Prob,
        ],
        [
            39 as Vp8Prob,
            28 as Vp8Prob,
            85 as Vp8Prob,
            171 as Vp8Prob,
            58 as Vp8Prob,
            165 as Vp8Prob,
            90 as Vp8Prob,
            98 as Vp8Prob,
            64 as Vp8Prob,
        ],
        [
            34 as Vp8Prob,
            22 as Vp8Prob,
            116 as Vp8Prob,
            206 as Vp8Prob,
            23 as Vp8Prob,
            34 as Vp8Prob,
            43 as Vp8Prob,
            166 as Vp8Prob,
            73 as Vp8Prob,
        ],
        [
            68 as Vp8Prob,
            25 as Vp8Prob,
            106 as Vp8Prob,
            22 as Vp8Prob,
            64 as Vp8Prob,
            171 as Vp8Prob,
            36 as Vp8Prob,
            225 as Vp8Prob,
            114 as Vp8Prob,
        ],
        [
            34 as Vp8Prob,
            19 as Vp8Prob,
            21 as Vp8Prob,
            102 as Vp8Prob,
            132 as Vp8Prob,
            188 as Vp8Prob,
            16 as Vp8Prob,
            76 as Vp8Prob,
            124 as Vp8Prob,
        ],
        [
            62 as Vp8Prob,
            18 as Vp8Prob,
            78 as Vp8Prob,
            95 as Vp8Prob,
            85 as Vp8Prob,
            57 as Vp8Prob,
            50 as Vp8Prob,
            48 as Vp8Prob,
            51 as Vp8Prob,
        ],
    ],
    [
        [
            193 as Vp8Prob,
            101 as Vp8Prob,
            35 as Vp8Prob,
            159 as Vp8Prob,
            215 as Vp8Prob,
            111 as Vp8Prob,
            89 as Vp8Prob,
            46 as Vp8Prob,
            111 as Vp8Prob,
        ],
        [
            60 as Vp8Prob,
            148 as Vp8Prob,
            31 as Vp8Prob,
            172 as Vp8Prob,
            219 as Vp8Prob,
            228 as Vp8Prob,
            21 as Vp8Prob,
            18 as Vp8Prob,
            111 as Vp8Prob,
        ],
        [
            112 as Vp8Prob,
            113 as Vp8Prob,
            77 as Vp8Prob,
            85 as Vp8Prob,
            179 as Vp8Prob,
            255 as Vp8Prob,
            38 as Vp8Prob,
            120 as Vp8Prob,
            114 as Vp8Prob,
        ],
        [
            40 as Vp8Prob,
            42 as Vp8Prob,
            1 as Vp8Prob,
            196 as Vp8Prob,
            245 as Vp8Prob,
            209 as Vp8Prob,
            10 as Vp8Prob,
            25 as Vp8Prob,
            109 as Vp8Prob,
        ],
        [
            100 as Vp8Prob,
            80 as Vp8Prob,
            8 as Vp8Prob,
            43 as Vp8Prob,
            154 as Vp8Prob,
            1 as Vp8Prob,
            51 as Vp8Prob,
            26 as Vp8Prob,
            71 as Vp8Prob,
        ],
        [
            88 as Vp8Prob,
            43 as Vp8Prob,
            29 as Vp8Prob,
            140 as Vp8Prob,
            166 as Vp8Prob,
            213 as Vp8Prob,
            37 as Vp8Prob,
            43 as Vp8Prob,
            154 as Vp8Prob,
        ],
        [
            61 as Vp8Prob,
            63 as Vp8Prob,
            30 as Vp8Prob,
            155 as Vp8Prob,
            67 as Vp8Prob,
            45 as Vp8Prob,
            68 as Vp8Prob,
            1 as Vp8Prob,
            209 as Vp8Prob,
        ],
        [
            142 as Vp8Prob,
            78 as Vp8Prob,
            78 as Vp8Prob,
            16 as Vp8Prob,
            255 as Vp8Prob,
            128 as Vp8Prob,
            34 as Vp8Prob,
            197 as Vp8Prob,
            171 as Vp8Prob,
        ],
        [
            41 as Vp8Prob,
            40 as Vp8Prob,
            5 as Vp8Prob,
            102 as Vp8Prob,
            211 as Vp8Prob,
            183 as Vp8Prob,
            4 as Vp8Prob,
            1 as Vp8Prob,
            221 as Vp8Prob,
        ],
        [
            51 as Vp8Prob,
            50 as Vp8Prob,
            17 as Vp8Prob,
            168 as Vp8Prob,
            209 as Vp8Prob,
            192 as Vp8Prob,
            23 as Vp8Prob,
            25 as Vp8Prob,
            82 as Vp8Prob,
        ],
    ],
    [
        [
            125 as Vp8Prob,
            98 as Vp8Prob,
            42 as Vp8Prob,
            88 as Vp8Prob,
            104 as Vp8Prob,
            85 as Vp8Prob,
            117 as Vp8Prob,
            175 as Vp8Prob,
            82 as Vp8Prob,
        ],
        [
            95 as Vp8Prob,
            84 as Vp8Prob,
            53 as Vp8Prob,
            89 as Vp8Prob,
            128 as Vp8Prob,
            100 as Vp8Prob,
            113 as Vp8Prob,
            101 as Vp8Prob,
            45 as Vp8Prob,
        ],
        [
            75 as Vp8Prob,
            79 as Vp8Prob,
            123 as Vp8Prob,
            47 as Vp8Prob,
            51 as Vp8Prob,
            128 as Vp8Prob,
            81 as Vp8Prob,
            171 as Vp8Prob,
            1 as Vp8Prob,
        ],
        [
            57 as Vp8Prob,
            17 as Vp8Prob,
            5 as Vp8Prob,
            71 as Vp8Prob,
            102 as Vp8Prob,
            57 as Vp8Prob,
            53 as Vp8Prob,
            41 as Vp8Prob,
            49 as Vp8Prob,
        ],
        [
            115 as Vp8Prob,
            21 as Vp8Prob,
            2 as Vp8Prob,
            10 as Vp8Prob,
            102 as Vp8Prob,
            255 as Vp8Prob,
            166 as Vp8Prob,
            23 as Vp8Prob,
            6 as Vp8Prob,
        ],
        [
            38 as Vp8Prob,
            33 as Vp8Prob,
            13 as Vp8Prob,
            121 as Vp8Prob,
            57 as Vp8Prob,
            73 as Vp8Prob,
            26 as Vp8Prob,
            1 as Vp8Prob,
            85 as Vp8Prob,
        ],
        [
            41 as Vp8Prob,
            10 as Vp8Prob,
            67 as Vp8Prob,
            138 as Vp8Prob,
            77 as Vp8Prob,
            110 as Vp8Prob,
            90 as Vp8Prob,
            47 as Vp8Prob,
            114 as Vp8Prob,
        ],
        [
            101 as Vp8Prob,
            29 as Vp8Prob,
            16 as Vp8Prob,
            10 as Vp8Prob,
            85 as Vp8Prob,
            128 as Vp8Prob,
            101 as Vp8Prob,
            196 as Vp8Prob,
            26 as Vp8Prob,
        ],
        [
            57 as Vp8Prob,
            18 as Vp8Prob,
            10 as Vp8Prob,
            102 as Vp8Prob,
            102 as Vp8Prob,
            213 as Vp8Prob,
            34 as Vp8Prob,
            20 as Vp8Prob,
            43 as Vp8Prob,
        ],
        [
            117 as Vp8Prob,
            20 as Vp8Prob,
            15 as Vp8Prob,
            36 as Vp8Prob,
            163 as Vp8Prob,
            128 as Vp8Prob,
            68 as Vp8Prob,
            1 as Vp8Prob,
            26 as Vp8Prob,
        ],
    ],
    [
        [
            138 as Vp8Prob,
            31 as Vp8Prob,
            36 as Vp8Prob,
            171 as Vp8Prob,
            27 as Vp8Prob,
            166 as Vp8Prob,
            38 as Vp8Prob,
            44 as Vp8Prob,
            229 as Vp8Prob,
        ],
        [
            67 as Vp8Prob,
            87 as Vp8Prob,
            58 as Vp8Prob,
            169 as Vp8Prob,
            82 as Vp8Prob,
            115 as Vp8Prob,
            26 as Vp8Prob,
            59 as Vp8Prob,
            179 as Vp8Prob,
        ],
        [
            63 as Vp8Prob,
            59 as Vp8Prob,
            90 as Vp8Prob,
            180 as Vp8Prob,
            59 as Vp8Prob,
            166 as Vp8Prob,
            93 as Vp8Prob,
            73 as Vp8Prob,
            154 as Vp8Prob,
        ],
        [
            40 as Vp8Prob,
            40 as Vp8Prob,
            21 as Vp8Prob,
            116 as Vp8Prob,
            143 as Vp8Prob,
            209 as Vp8Prob,
            34 as Vp8Prob,
            39 as Vp8Prob,
            175 as Vp8Prob,
        ],
        [
            57 as Vp8Prob,
            46 as Vp8Prob,
            22 as Vp8Prob,
            24 as Vp8Prob,
            128 as Vp8Prob,
            1 as Vp8Prob,
            54 as Vp8Prob,
            17 as Vp8Prob,
            37 as Vp8Prob,
        ],
        [
            47 as Vp8Prob,
            15 as Vp8Prob,
            16 as Vp8Prob,
            183 as Vp8Prob,
            34 as Vp8Prob,
            223 as Vp8Prob,
            49 as Vp8Prob,
            45 as Vp8Prob,
            183 as Vp8Prob,
        ],
        [
            46 as Vp8Prob,
            17 as Vp8Prob,
            33 as Vp8Prob,
            183 as Vp8Prob,
            6 as Vp8Prob,
            98 as Vp8Prob,
            15 as Vp8Prob,
            32 as Vp8Prob,
            183 as Vp8Prob,
        ],
        [
            65 as Vp8Prob,
            32 as Vp8Prob,
            73 as Vp8Prob,
            115 as Vp8Prob,
            28 as Vp8Prob,
            128 as Vp8Prob,
            23 as Vp8Prob,
            128 as Vp8Prob,
            205 as Vp8Prob,
        ],
        [
            40 as Vp8Prob,
            3 as Vp8Prob,
            9 as Vp8Prob,
            115 as Vp8Prob,
            51 as Vp8Prob,
            192 as Vp8Prob,
            18 as Vp8Prob,
            6 as Vp8Prob,
            223 as Vp8Prob,
        ],
        [
            87 as Vp8Prob,
            37 as Vp8Prob,
            9 as Vp8Prob,
            115 as Vp8Prob,
            59 as Vp8Prob,
            77 as Vp8Prob,
            64 as Vp8Prob,
            21 as Vp8Prob,
            47 as Vp8Prob,
        ],
    ],
    [
        [
            104 as Vp8Prob,
            55 as Vp8Prob,
            44 as Vp8Prob,
            218 as Vp8Prob,
            9 as Vp8Prob,
            54 as Vp8Prob,
            53 as Vp8Prob,
            130 as Vp8Prob,
            226 as Vp8Prob,
        ],
        [
            64 as Vp8Prob,
            90 as Vp8Prob,
            70 as Vp8Prob,
            205 as Vp8Prob,
            40 as Vp8Prob,
            41 as Vp8Prob,
            23 as Vp8Prob,
            26 as Vp8Prob,
            57 as Vp8Prob,
        ],
        [
            54 as Vp8Prob,
            57 as Vp8Prob,
            112 as Vp8Prob,
            184 as Vp8Prob,
            5 as Vp8Prob,
            41 as Vp8Prob,
            38 as Vp8Prob,
            166 as Vp8Prob,
            213 as Vp8Prob,
        ],
        [
            30 as Vp8Prob,
            34 as Vp8Prob,
            26 as Vp8Prob,
            133 as Vp8Prob,
            152 as Vp8Prob,
            116 as Vp8Prob,
            10 as Vp8Prob,
            32 as Vp8Prob,
            134 as Vp8Prob,
        ],
        [
            75 as Vp8Prob,
            32 as Vp8Prob,
            12 as Vp8Prob,
            51 as Vp8Prob,
            192 as Vp8Prob,
            255 as Vp8Prob,
            160 as Vp8Prob,
            43 as Vp8Prob,
            51 as Vp8Prob,
        ],
        [
            39 as Vp8Prob,
            19 as Vp8Prob,
            53 as Vp8Prob,
            221 as Vp8Prob,
            26 as Vp8Prob,
            114 as Vp8Prob,
            32 as Vp8Prob,
            73 as Vp8Prob,
            255 as Vp8Prob,
        ],
        [
            31 as Vp8Prob,
            9 as Vp8Prob,
            65 as Vp8Prob,
            234 as Vp8Prob,
            2 as Vp8Prob,
            15 as Vp8Prob,
            1 as Vp8Prob,
            118 as Vp8Prob,
            73 as Vp8Prob,
        ],
        [
            88 as Vp8Prob,
            31 as Vp8Prob,
            35 as Vp8Prob,
            67 as Vp8Prob,
            102 as Vp8Prob,
            85 as Vp8Prob,
            55 as Vp8Prob,
            186 as Vp8Prob,
            85 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            21 as Vp8Prob,
            23 as Vp8Prob,
            111 as Vp8Prob,
            59 as Vp8Prob,
            205 as Vp8Prob,
            45 as Vp8Prob,
            37 as Vp8Prob,
            192 as Vp8Prob,
        ],
        [
            55 as Vp8Prob,
            38 as Vp8Prob,
            70 as Vp8Prob,
            124 as Vp8Prob,
            73 as Vp8Prob,
            102 as Vp8Prob,
            1 as Vp8Prob,
            34 as Vp8Prob,
            98 as Vp8Prob,
        ],
    ],
    [
        [
            102 as Vp8Prob,
            61 as Vp8Prob,
            71 as Vp8Prob,
            37 as Vp8Prob,
            34 as Vp8Prob,
            53 as Vp8Prob,
            31 as Vp8Prob,
            243 as Vp8Prob,
            192 as Vp8Prob,
        ],
        [
            69 as Vp8Prob,
            60 as Vp8Prob,
            71 as Vp8Prob,
            38 as Vp8Prob,
            73 as Vp8Prob,
            119 as Vp8Prob,
            28 as Vp8Prob,
            222 as Vp8Prob,
            37 as Vp8Prob,
        ],
        [
            68 as Vp8Prob,
            45 as Vp8Prob,
            128 as Vp8Prob,
            34 as Vp8Prob,
            1 as Vp8Prob,
            47 as Vp8Prob,
            11 as Vp8Prob,
            245 as Vp8Prob,
            171 as Vp8Prob,
        ],
        [
            62 as Vp8Prob,
            17 as Vp8Prob,
            19 as Vp8Prob,
            70 as Vp8Prob,
            146 as Vp8Prob,
            85 as Vp8Prob,
            55 as Vp8Prob,
            62 as Vp8Prob,
            70 as Vp8Prob,
        ],
        [
            75 as Vp8Prob,
            15 as Vp8Prob,
            9 as Vp8Prob,
            9 as Vp8Prob,
            64 as Vp8Prob,
            255 as Vp8Prob,
            184 as Vp8Prob,
            119 as Vp8Prob,
            16 as Vp8Prob,
        ],
        [
            37 as Vp8Prob,
            43 as Vp8Prob,
            37 as Vp8Prob,
            154 as Vp8Prob,
            100 as Vp8Prob,
            163 as Vp8Prob,
            85 as Vp8Prob,
            160 as Vp8Prob,
            1 as Vp8Prob,
        ],
        [
            63 as Vp8Prob,
            9 as Vp8Prob,
            92 as Vp8Prob,
            136 as Vp8Prob,
            28 as Vp8Prob,
            64 as Vp8Prob,
            32 as Vp8Prob,
            201 as Vp8Prob,
            85 as Vp8Prob,
        ],
        [
            86 as Vp8Prob,
            6 as Vp8Prob,
            28 as Vp8Prob,
            5 as Vp8Prob,
            64 as Vp8Prob,
            255 as Vp8Prob,
            25 as Vp8Prob,
            248 as Vp8Prob,
            1 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            8 as Vp8Prob,
            17 as Vp8Prob,
            132 as Vp8Prob,
            137 as Vp8Prob,
            255 as Vp8Prob,
            55 as Vp8Prob,
            116 as Vp8Prob,
            128 as Vp8Prob,
        ],
        [
            58 as Vp8Prob,
            15 as Vp8Prob,
            20 as Vp8Prob,
            82 as Vp8Prob,
            135 as Vp8Prob,
            57 as Vp8Prob,
            26 as Vp8Prob,
            121 as Vp8Prob,
            40 as Vp8Prob,
        ],
    ],
    [
        [
            164 as Vp8Prob,
            50 as Vp8Prob,
            31 as Vp8Prob,
            137 as Vp8Prob,
            154 as Vp8Prob,
            133 as Vp8Prob,
            25 as Vp8Prob,
            35 as Vp8Prob,
            218 as Vp8Prob,
        ],
        [
            51 as Vp8Prob,
            103 as Vp8Prob,
            44 as Vp8Prob,
            131 as Vp8Prob,
            131 as Vp8Prob,
            123 as Vp8Prob,
            31 as Vp8Prob,
            6 as Vp8Prob,
            158 as Vp8Prob,
        ],
        [
            86 as Vp8Prob,
            40 as Vp8Prob,
            64 as Vp8Prob,
            135 as Vp8Prob,
            148 as Vp8Prob,
            224 as Vp8Prob,
            45 as Vp8Prob,
            183 as Vp8Prob,
            128 as Vp8Prob,
        ],
        [
            22 as Vp8Prob,
            26 as Vp8Prob,
            17 as Vp8Prob,
            131 as Vp8Prob,
            240 as Vp8Prob,
            154 as Vp8Prob,
            14 as Vp8Prob,
            1 as Vp8Prob,
            209 as Vp8Prob,
        ],
        [
            83 as Vp8Prob,
            12 as Vp8Prob,
            13 as Vp8Prob,
            54 as Vp8Prob,
            192 as Vp8Prob,
            255 as Vp8Prob,
            68 as Vp8Prob,
            47 as Vp8Prob,
            28 as Vp8Prob,
        ],
        [
            45 as Vp8Prob,
            16 as Vp8Prob,
            21 as Vp8Prob,
            91 as Vp8Prob,
            64 as Vp8Prob,
            222 as Vp8Prob,
            7 as Vp8Prob,
            1 as Vp8Prob,
            197 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            21 as Vp8Prob,
            39 as Vp8Prob,
            155 as Vp8Prob,
            60 as Vp8Prob,
            138 as Vp8Prob,
            23 as Vp8Prob,
            102 as Vp8Prob,
            213 as Vp8Prob,
        ],
        [
            85 as Vp8Prob,
            26 as Vp8Prob,
            85 as Vp8Prob,
            85 as Vp8Prob,
            128 as Vp8Prob,
            128 as Vp8Prob,
            32 as Vp8Prob,
            146 as Vp8Prob,
            171 as Vp8Prob,
        ],
        [
            18 as Vp8Prob,
            11 as Vp8Prob,
            7 as Vp8Prob,
            63 as Vp8Prob,
            144 as Vp8Prob,
            171 as Vp8Prob,
            4 as Vp8Prob,
            4 as Vp8Prob,
            246 as Vp8Prob,
        ],
        [
            35 as Vp8Prob,
            27 as Vp8Prob,
            10 as Vp8Prob,
            146 as Vp8Prob,
            174 as Vp8Prob,
            171 as Vp8Prob,
            12 as Vp8Prob,
            26 as Vp8Prob,
            128 as Vp8Prob,
        ],
    ],
    [
        [
            190 as Vp8Prob,
            80 as Vp8Prob,
            35 as Vp8Prob,
            99 as Vp8Prob,
            180 as Vp8Prob,
            80 as Vp8Prob,
            126 as Vp8Prob,
            54 as Vp8Prob,
            45 as Vp8Prob,
        ],
        [
            85 as Vp8Prob,
            126 as Vp8Prob,
            47 as Vp8Prob,
            87 as Vp8Prob,
            176 as Vp8Prob,
            51 as Vp8Prob,
            41 as Vp8Prob,
            20 as Vp8Prob,
            32 as Vp8Prob,
        ],
        [
            101 as Vp8Prob,
            75 as Vp8Prob,
            128 as Vp8Prob,
            139 as Vp8Prob,
            118 as Vp8Prob,
            146 as Vp8Prob,
            116 as Vp8Prob,
            128 as Vp8Prob,
            85 as Vp8Prob,
        ],
        [
            56 as Vp8Prob,
            41 as Vp8Prob,
            15 as Vp8Prob,
            176 as Vp8Prob,
            236 as Vp8Prob,
            85 as Vp8Prob,
            37 as Vp8Prob,
            9 as Vp8Prob,
            62 as Vp8Prob,
        ],
        [
            146 as Vp8Prob,
            36 as Vp8Prob,
            19 as Vp8Prob,
            30 as Vp8Prob,
            171 as Vp8Prob,
            255 as Vp8Prob,
            97 as Vp8Prob,
            27 as Vp8Prob,
            20 as Vp8Prob,
        ],
        [
            71 as Vp8Prob,
            30 as Vp8Prob,
            17 as Vp8Prob,
            119 as Vp8Prob,
            118 as Vp8Prob,
            255 as Vp8Prob,
            17 as Vp8Prob,
            18 as Vp8Prob,
            138 as Vp8Prob,
        ],
        [
            101 as Vp8Prob,
            38 as Vp8Prob,
            60 as Vp8Prob,
            138 as Vp8Prob,
            55 as Vp8Prob,
            70 as Vp8Prob,
            43 as Vp8Prob,
            26 as Vp8Prob,
            142 as Vp8Prob,
        ],
        [
            138 as Vp8Prob,
            45 as Vp8Prob,
            61 as Vp8Prob,
            62 as Vp8Prob,
            219 as Vp8Prob,
            1 as Vp8Prob,
            81 as Vp8Prob,
            188 as Vp8Prob,
            64 as Vp8Prob,
        ],
        [
            32 as Vp8Prob,
            41 as Vp8Prob,
            20 as Vp8Prob,
            117 as Vp8Prob,
            151 as Vp8Prob,
            142 as Vp8Prob,
            20 as Vp8Prob,
            21 as Vp8Prob,
            163 as Vp8Prob,
        ],
        [
            112 as Vp8Prob,
            19 as Vp8Prob,
            12 as Vp8Prob,
            61 as Vp8Prob,
            195 as Vp8Prob,
            128 as Vp8Prob,
            48 as Vp8Prob,
            4 as Vp8Prob,
            24 as Vp8Prob,
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
static mut sub_mv_ref_prob: [Vp8Prob; 3] = [180 as Vp8Prob, 162 as Vp8Prob, 25 as Vp8Prob];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_prob2: [[Vp8Prob; 3]; 5] = [
    [147 as Vp8Prob, 136 as Vp8Prob, 18 as Vp8Prob],
    [106 as Vp8Prob, 145 as Vp8Prob, 1 as Vp8Prob],
    [179 as Vp8Prob, 121 as Vp8Prob, 1 as Vp8Prob],
    [223 as Vp8Prob, 1 as Vp8Prob, 34 as Vp8Prob],
    [208 as Vp8Prob, 1 as Vp8Prob, 1 as Vp8Prob],
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
pub static mut vp8_mbsplit_probs: [Vp8Prob; 3] = [110 as Vp8Prob, 111 as Vp8Prob, 150 as Vp8Prob];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_tree: [Vp8TreeIndex; 18] = [
    -(B_DC_PRED as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    -(B_TM_PRED as i32) as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    -(B_VE_PRED as i32) as Vp8TreeIndex,
    6 as Vp8TreeIndex,
    8 as Vp8TreeIndex,
    12 as Vp8TreeIndex,
    -(B_HE_PRED as i32) as Vp8TreeIndex,
    10 as Vp8TreeIndex,
    -(B_RD_PRED as i32) as Vp8TreeIndex,
    -(B_VR_PRED as i32) as Vp8TreeIndex,
    -(B_LD_PRED as i32) as Vp8TreeIndex,
    14 as Vp8TreeIndex,
    -(B_VL_PRED as i32) as Vp8TreeIndex,
    16 as Vp8TreeIndex,
    -(B_HD_PRED as i32) as Vp8TreeIndex,
    -(B_HU_PRED as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_tree: [Vp8TreeIndex; 8] = [
    -(DC_PRED as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    6 as Vp8TreeIndex,
    -(V_PRED as i32) as Vp8TreeIndex,
    -(H_PRED as i32) as Vp8TreeIndex,
    -(TM_PRED as i32) as Vp8TreeIndex,
    -(B_PRED as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_tree: [Vp8TreeIndex; 8] = [
    -(B_PRED as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    6 as Vp8TreeIndex,
    -(DC_PRED as i32) as Vp8TreeIndex,
    -(V_PRED as i32) as Vp8TreeIndex,
    -(H_PRED as i32) as Vp8TreeIndex,
    -(TM_PRED as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_tree: [Vp8TreeIndex; 6] = [
    -(DC_PRED as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    -(V_PRED as i32) as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    -(H_PRED as i32) as Vp8TreeIndex,
    -(TM_PRED as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_tree: [Vp8TreeIndex; 6] = [
    -(3 as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    -(2 as i32) as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    -(0 as i32) as Vp8TreeIndex,
    -(1 as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_tree: [Vp8TreeIndex; 8] = [
    -(ZEROMV as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    -(NEARESTMV as i32) as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    -(NEARMV as i32) as Vp8TreeIndex,
    6 as Vp8TreeIndex,
    -(NEWMV as i32) as Vp8TreeIndex,
    -(SPLITMV as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_tree: [Vp8TreeIndex; 6] = [
    -(LEFT4X4 as i32) as Vp8TreeIndex,
    2 as Vp8TreeIndex,
    -(ABOVE4X4 as i32) as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    -(ZERO4X4 as i32) as Vp8TreeIndex,
    -(NEW4X4 as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvtree: [Vp8TreeIndex; 14] = [
    2 as Vp8TreeIndex,
    8 as Vp8TreeIndex,
    4 as Vp8TreeIndex,
    6 as Vp8TreeIndex,
    -(0 as i32) as Vp8TreeIndex,
    -(1 as i32) as Vp8TreeIndex,
    -(2 as i32) as Vp8TreeIndex,
    -(3 as i32) as Vp8TreeIndex,
    10 as Vp8TreeIndex,
    12 as Vp8TreeIndex,
    -(4 as i32) as Vp8TreeIndex,
    -(5 as i32) as Vp8TreeIndex,
    -(6 as i32) as Vp8TreeIndex,
    -(7 as i32) as Vp8TreeIndex,
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_mbmode_probs(mut x: *mut Vp8Common) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_ymode_prob as *const Vp8Prob as *const c_void as *const u8,
            &raw mut (*x).fc.ymode_prob as *mut Vp8Prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[Vp8Prob; 4]>() as SizeT,
        );
        core::ptr::copy_nonoverlapping(
            &raw const vp8_uv_mode_prob as *const Vp8Prob as *const c_void as *const u8,
            &raw mut (*x).fc.uv_mode_prob as *mut Vp8Prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[Vp8Prob; 3]>() as SizeT,
        );
        core::ptr::copy_nonoverlapping(
            &raw const sub_mv_ref_prob as *const Vp8Prob as *const c_void as *const u8,
            &raw mut (*x).fc.sub_mv_ref_prob as *mut Vp8Prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[Vp8Prob; 3]>() as SizeT,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_default_bmode_probs(mut dest: *mut Vp8Prob) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_bmode_prob as *const Vp8Prob as *const c_void as *const u8,
            dest as *mut c_void as *mut u8,
            ::core::mem::size_of::<[Vp8Prob; 9]>() as SizeT,
        );
    }
}
