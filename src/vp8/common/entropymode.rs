use std::ffi::c_void;
unsafe extern "Rust" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
pub type uint32_t = u32;
pub type B_PREDICTION_MODE = u32;
pub const B_MODE_COUNT: B_PREDICTION_MODE = 14;
pub const NEW4X4: B_PREDICTION_MODE = 13;
pub const ZERO4X4: B_PREDICTION_MODE = 12;
pub const ABOVE4X4: B_PREDICTION_MODE = 11;
pub const LEFT4X4: B_PREDICTION_MODE = 10;
pub const B_HU_PRED: B_PREDICTION_MODE = 9;
pub const B_HD_PRED: B_PREDICTION_MODE = 8;
pub const B_VL_PRED: B_PREDICTION_MODE = 7;
pub const B_VR_PRED: B_PREDICTION_MODE = 6;
pub const B_RD_PRED: B_PREDICTION_MODE = 5;
pub const B_LD_PRED: B_PREDICTION_MODE = 4;
pub const B_HE_PRED: B_PREDICTION_MODE = 3;
pub const B_VE_PRED: B_PREDICTION_MODE = 2;
pub const B_TM_PRED: B_PREDICTION_MODE = 1;
pub const B_DC_PRED: B_PREDICTION_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [i8; 80],
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
pub type vp8_prob = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type ENTROPY_CONTEXT = i8;
pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
pub type MODE_INFO = modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub mode: uint8_t,
    pub uv_mode: uint8_t,
    pub ref_frame: uint8_t,
    pub is_4x4: uint8_t,
    pub mv: int_mv,
    pub partitioning: uint8_t,
    pub mb_skip_coeff: uint8_t,
    pub need_to_clamp_mvs: uint8_t,
    pub segment_id: uint8_t,
}
pub type uint8_t = u8;
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
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
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
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
    pub error: vpx_internal_error_info,
    pub y1dequant: [[i16; 2]; 128],
    pub y2dequant: [[i16; 2]; 128],
    pub uvdequant: [[i16; 2]; 128],
    pub width: i32,
    pub height: i32,
    pub horiz_scale: i32,
    pub vert_scale: i32,
    pub clamp_type: CLAMP_TYPE,
    pub frame_to_show: *mut YV12_BUFFER_CONFIG,
    pub yv12_fb: [YV12_BUFFER_CONFIG; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: YV12_BUFFER_CONFIG,
    pub last_frame_type: FRAME_TYPE,
    pub frame_type: FRAME_TYPE,
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
    pub mip: *mut MODE_INFO,
    pub mi: *mut MODE_INFO,
    pub show_frame_mi: *mut MODE_INFO,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: loop_filter_info_n,
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
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: i32,
}
pub type TOKEN_PARTITION = u32;
pub const EIGHT_PARTITION: TOKEN_PARTITION = 3;
pub const FOUR_PARTITION: TOKEN_PARTITION = 2;
pub const TWO_PARTITION: TOKEN_PARTITION = 1;
pub const ONE_PARTITION: TOKEN_PARTITION = 0;
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub bmode_prob: [vp8_prob; 9],
    pub ymode_prob: [vp8_prob; 4],
    pub uv_mode_prob: [vp8_prob; 3],
    pub sub_mv_ref_prob: [vp8_prob; 3],
    pub coef_probs: [[[[vp8_prob; 11]; 3]; 8]; 4],
    pub mvc: [MV_CONTEXT; 2],
}
pub type MV_CONTEXT = mv_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv_context {
    pub prob: [vp8_prob; 19],
}
pub type CLAMP_TYPE = u32;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type vp8_tree_index = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: i32,
    pub len: i32,
}
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
pub type VP8_COMMON = VP8Common;
pub type C2RustUnnamed_0 = u32;
pub const SUBMVREF_LEFT_ABOVE_ZED: C2RustUnnamed_0 = 4;
pub const SUBMVREF_LEFT_ABOVE_SAME: C2RustUnnamed_0 = 3;
pub const SUBMVREF_ABOVE_ZED: C2RustUnnamed_0 = 2;
pub const SUBMVREF_LEFT_ZED: C2RustUnnamed_0 = 1;
pub const SUBMVREF_NORMAL: C2RustUnnamed_0 = 0;
pub type vp8_mbsplit = [i32; 16];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_encodings: [vp8_token_struct; 10] = [
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
    vp8_token_struct {
        value: 2 as i32,
        len: 2 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 28 as i32,
        len: 5 as i32,
    },
    vp8_token_struct {
        value: 30 as i32,
        len: 5 as i32,
    },
    vp8_token_struct {
        value: 58 as i32,
        len: 6 as i32,
    },
    vp8_token_struct {
        value: 59 as i32,
        len: 6 as i32,
    },
    vp8_token_struct {
        value: 62 as i32,
        len: 6 as i32,
    },
    vp8_token_struct {
        value: 126 as i32,
        len: 7 as i32,
    },
    vp8_token_struct {
        value: 127 as i32,
        len: 7 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_encodings: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
    vp8_token_struct {
        value: 4 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 5 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_encodings: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 4 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 5 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_encodings: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
    vp8_token_struct {
        value: 2 as i32,
        len: 2 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_encodings: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 2 as i32,
        len: 2 as i32,
    },
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_encoding_array: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 2 as i32,
        len: 2 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
    vp8_token_struct {
        value: 14 as i32,
        len: 4 as i32,
    },
    vp8_token_struct {
        value: 15 as i32,
        len: 4 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_encoding_array: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 0 as i32,
        len: 1 as i32,
    },
    vp8_token_struct {
        value: 2 as i32,
        len: 2 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvencodings: [vp8_token_struct; 8] = [
    vp8_token_struct {
        value: 0 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 1 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 2 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 3 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 4 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 5 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 6 as i32,
        len: 3 as i32,
    },
    vp8_token_struct {
        value: 7 as i32,
        len: 3 as i32,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_prob: [vp8_prob; 4] = [
    112 as vp8_prob,
    86 as vp8_prob,
    140 as vp8_prob,
    37 as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_prob: [vp8_prob; 4] = [
    145 as vp8_prob,
    156 as vp8_prob,
    163 as vp8_prob,
    128 as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_prob: [vp8_prob; 3] =
    [162 as vp8_prob, 101 as vp8_prob, 204 as vp8_prob];
#[unsafe(no_mangle)]
pub static mut vp8_kf_uv_mode_prob: [vp8_prob; 3] =
    [142 as vp8_prob, 114 as vp8_prob, 183 as vp8_prob];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_prob: [vp8_prob; 9] = [
    120 as vp8_prob,
    90 as vp8_prob,
    79 as vp8_prob,
    133 as vp8_prob,
    87 as vp8_prob,
    85 as vp8_prob,
    80 as vp8_prob,
    111 as vp8_prob,
    151 as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_bmode_prob: [[[vp8_prob; 9]; 10]; 10] = [
    [
        [
            231 as vp8_prob,
            120 as vp8_prob,
            48 as vp8_prob,
            89 as vp8_prob,
            115 as vp8_prob,
            113 as vp8_prob,
            120 as vp8_prob,
            152 as vp8_prob,
            112 as vp8_prob,
        ],
        [
            152 as vp8_prob,
            179 as vp8_prob,
            64 as vp8_prob,
            126 as vp8_prob,
            170 as vp8_prob,
            118 as vp8_prob,
            46 as vp8_prob,
            70 as vp8_prob,
            95 as vp8_prob,
        ],
        [
            175 as vp8_prob,
            69 as vp8_prob,
            143 as vp8_prob,
            80 as vp8_prob,
            85 as vp8_prob,
            82 as vp8_prob,
            72 as vp8_prob,
            155 as vp8_prob,
            103 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            58 as vp8_prob,
            10 as vp8_prob,
            171 as vp8_prob,
            218 as vp8_prob,
            189 as vp8_prob,
            17 as vp8_prob,
            13 as vp8_prob,
            152 as vp8_prob,
        ],
        [
            144 as vp8_prob,
            71 as vp8_prob,
            10 as vp8_prob,
            38 as vp8_prob,
            171 as vp8_prob,
            213 as vp8_prob,
            144 as vp8_prob,
            34 as vp8_prob,
            26 as vp8_prob,
        ],
        [
            114 as vp8_prob,
            26 as vp8_prob,
            17 as vp8_prob,
            163 as vp8_prob,
            44 as vp8_prob,
            195 as vp8_prob,
            21 as vp8_prob,
            10 as vp8_prob,
            173 as vp8_prob,
        ],
        [
            121 as vp8_prob,
            24 as vp8_prob,
            80 as vp8_prob,
            195 as vp8_prob,
            26 as vp8_prob,
            62 as vp8_prob,
            44 as vp8_prob,
            64 as vp8_prob,
            85 as vp8_prob,
        ],
        [
            170 as vp8_prob,
            46 as vp8_prob,
            55 as vp8_prob,
            19 as vp8_prob,
            136 as vp8_prob,
            160 as vp8_prob,
            33 as vp8_prob,
            206 as vp8_prob,
            71 as vp8_prob,
        ],
        [
            63 as vp8_prob,
            20 as vp8_prob,
            8 as vp8_prob,
            114 as vp8_prob,
            114 as vp8_prob,
            208 as vp8_prob,
            12 as vp8_prob,
            9 as vp8_prob,
            226 as vp8_prob,
        ],
        [
            81 as vp8_prob,
            40 as vp8_prob,
            11 as vp8_prob,
            96 as vp8_prob,
            182 as vp8_prob,
            84 as vp8_prob,
            29 as vp8_prob,
            16 as vp8_prob,
            36 as vp8_prob,
        ],
    ],
    [
        [
            134 as vp8_prob,
            183 as vp8_prob,
            89 as vp8_prob,
            137 as vp8_prob,
            98 as vp8_prob,
            101 as vp8_prob,
            106 as vp8_prob,
            165 as vp8_prob,
            148 as vp8_prob,
        ],
        [
            72 as vp8_prob,
            187 as vp8_prob,
            100 as vp8_prob,
            130 as vp8_prob,
            157 as vp8_prob,
            111 as vp8_prob,
            32 as vp8_prob,
            75 as vp8_prob,
            80 as vp8_prob,
        ],
        [
            66 as vp8_prob,
            102 as vp8_prob,
            167 as vp8_prob,
            99 as vp8_prob,
            74 as vp8_prob,
            62 as vp8_prob,
            40 as vp8_prob,
            234 as vp8_prob,
            128 as vp8_prob,
        ],
        [
            41 as vp8_prob,
            53 as vp8_prob,
            9 as vp8_prob,
            178 as vp8_prob,
            241 as vp8_prob,
            141 as vp8_prob,
            26 as vp8_prob,
            8 as vp8_prob,
            107 as vp8_prob,
        ],
        [
            104 as vp8_prob,
            79 as vp8_prob,
            12 as vp8_prob,
            27 as vp8_prob,
            217 as vp8_prob,
            255 as vp8_prob,
            87 as vp8_prob,
            17 as vp8_prob,
            7 as vp8_prob,
        ],
        [
            74 as vp8_prob,
            43 as vp8_prob,
            26 as vp8_prob,
            146 as vp8_prob,
            73 as vp8_prob,
            166 as vp8_prob,
            49 as vp8_prob,
            23 as vp8_prob,
            157 as vp8_prob,
        ],
        [
            65 as vp8_prob,
            38 as vp8_prob,
            105 as vp8_prob,
            160 as vp8_prob,
            51 as vp8_prob,
            52 as vp8_prob,
            31 as vp8_prob,
            115 as vp8_prob,
            128 as vp8_prob,
        ],
        [
            87 as vp8_prob,
            68 as vp8_prob,
            71 as vp8_prob,
            44 as vp8_prob,
            114 as vp8_prob,
            51 as vp8_prob,
            15 as vp8_prob,
            186 as vp8_prob,
            23 as vp8_prob,
        ],
        [
            47 as vp8_prob,
            41 as vp8_prob,
            14 as vp8_prob,
            110 as vp8_prob,
            182 as vp8_prob,
            183 as vp8_prob,
            21 as vp8_prob,
            17 as vp8_prob,
            194 as vp8_prob,
        ],
        [
            66 as vp8_prob,
            45 as vp8_prob,
            25 as vp8_prob,
            102 as vp8_prob,
            197 as vp8_prob,
            189 as vp8_prob,
            23 as vp8_prob,
            18 as vp8_prob,
            22 as vp8_prob,
        ],
    ],
    [
        [
            88 as vp8_prob,
            88 as vp8_prob,
            147 as vp8_prob,
            150 as vp8_prob,
            42 as vp8_prob,
            46 as vp8_prob,
            45 as vp8_prob,
            196 as vp8_prob,
            205 as vp8_prob,
        ],
        [
            43 as vp8_prob,
            97 as vp8_prob,
            183 as vp8_prob,
            117 as vp8_prob,
            85 as vp8_prob,
            38 as vp8_prob,
            35 as vp8_prob,
            179 as vp8_prob,
            61 as vp8_prob,
        ],
        [
            39 as vp8_prob,
            53 as vp8_prob,
            200 as vp8_prob,
            87 as vp8_prob,
            26 as vp8_prob,
            21 as vp8_prob,
            43 as vp8_prob,
            232 as vp8_prob,
            171 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            34 as vp8_prob,
            51 as vp8_prob,
            104 as vp8_prob,
            114 as vp8_prob,
            102 as vp8_prob,
            29 as vp8_prob,
            93 as vp8_prob,
            77 as vp8_prob,
        ],
        [
            107 as vp8_prob,
            54 as vp8_prob,
            32 as vp8_prob,
            26 as vp8_prob,
            51 as vp8_prob,
            1 as vp8_prob,
            81 as vp8_prob,
            43 as vp8_prob,
            31 as vp8_prob,
        ],
        [
            39 as vp8_prob,
            28 as vp8_prob,
            85 as vp8_prob,
            171 as vp8_prob,
            58 as vp8_prob,
            165 as vp8_prob,
            90 as vp8_prob,
            98 as vp8_prob,
            64 as vp8_prob,
        ],
        [
            34 as vp8_prob,
            22 as vp8_prob,
            116 as vp8_prob,
            206 as vp8_prob,
            23 as vp8_prob,
            34 as vp8_prob,
            43 as vp8_prob,
            166 as vp8_prob,
            73 as vp8_prob,
        ],
        [
            68 as vp8_prob,
            25 as vp8_prob,
            106 as vp8_prob,
            22 as vp8_prob,
            64 as vp8_prob,
            171 as vp8_prob,
            36 as vp8_prob,
            225 as vp8_prob,
            114 as vp8_prob,
        ],
        [
            34 as vp8_prob,
            19 as vp8_prob,
            21 as vp8_prob,
            102 as vp8_prob,
            132 as vp8_prob,
            188 as vp8_prob,
            16 as vp8_prob,
            76 as vp8_prob,
            124 as vp8_prob,
        ],
        [
            62 as vp8_prob,
            18 as vp8_prob,
            78 as vp8_prob,
            95 as vp8_prob,
            85 as vp8_prob,
            57 as vp8_prob,
            50 as vp8_prob,
            48 as vp8_prob,
            51 as vp8_prob,
        ],
    ],
    [
        [
            193 as vp8_prob,
            101 as vp8_prob,
            35 as vp8_prob,
            159 as vp8_prob,
            215 as vp8_prob,
            111 as vp8_prob,
            89 as vp8_prob,
            46 as vp8_prob,
            111 as vp8_prob,
        ],
        [
            60 as vp8_prob,
            148 as vp8_prob,
            31 as vp8_prob,
            172 as vp8_prob,
            219 as vp8_prob,
            228 as vp8_prob,
            21 as vp8_prob,
            18 as vp8_prob,
            111 as vp8_prob,
        ],
        [
            112 as vp8_prob,
            113 as vp8_prob,
            77 as vp8_prob,
            85 as vp8_prob,
            179 as vp8_prob,
            255 as vp8_prob,
            38 as vp8_prob,
            120 as vp8_prob,
            114 as vp8_prob,
        ],
        [
            40 as vp8_prob,
            42 as vp8_prob,
            1 as vp8_prob,
            196 as vp8_prob,
            245 as vp8_prob,
            209 as vp8_prob,
            10 as vp8_prob,
            25 as vp8_prob,
            109 as vp8_prob,
        ],
        [
            100 as vp8_prob,
            80 as vp8_prob,
            8 as vp8_prob,
            43 as vp8_prob,
            154 as vp8_prob,
            1 as vp8_prob,
            51 as vp8_prob,
            26 as vp8_prob,
            71 as vp8_prob,
        ],
        [
            88 as vp8_prob,
            43 as vp8_prob,
            29 as vp8_prob,
            140 as vp8_prob,
            166 as vp8_prob,
            213 as vp8_prob,
            37 as vp8_prob,
            43 as vp8_prob,
            154 as vp8_prob,
        ],
        [
            61 as vp8_prob,
            63 as vp8_prob,
            30 as vp8_prob,
            155 as vp8_prob,
            67 as vp8_prob,
            45 as vp8_prob,
            68 as vp8_prob,
            1 as vp8_prob,
            209 as vp8_prob,
        ],
        [
            142 as vp8_prob,
            78 as vp8_prob,
            78 as vp8_prob,
            16 as vp8_prob,
            255 as vp8_prob,
            128 as vp8_prob,
            34 as vp8_prob,
            197 as vp8_prob,
            171 as vp8_prob,
        ],
        [
            41 as vp8_prob,
            40 as vp8_prob,
            5 as vp8_prob,
            102 as vp8_prob,
            211 as vp8_prob,
            183 as vp8_prob,
            4 as vp8_prob,
            1 as vp8_prob,
            221 as vp8_prob,
        ],
        [
            51 as vp8_prob,
            50 as vp8_prob,
            17 as vp8_prob,
            168 as vp8_prob,
            209 as vp8_prob,
            192 as vp8_prob,
            23 as vp8_prob,
            25 as vp8_prob,
            82 as vp8_prob,
        ],
    ],
    [
        [
            125 as vp8_prob,
            98 as vp8_prob,
            42 as vp8_prob,
            88 as vp8_prob,
            104 as vp8_prob,
            85 as vp8_prob,
            117 as vp8_prob,
            175 as vp8_prob,
            82 as vp8_prob,
        ],
        [
            95 as vp8_prob,
            84 as vp8_prob,
            53 as vp8_prob,
            89 as vp8_prob,
            128 as vp8_prob,
            100 as vp8_prob,
            113 as vp8_prob,
            101 as vp8_prob,
            45 as vp8_prob,
        ],
        [
            75 as vp8_prob,
            79 as vp8_prob,
            123 as vp8_prob,
            47 as vp8_prob,
            51 as vp8_prob,
            128 as vp8_prob,
            81 as vp8_prob,
            171 as vp8_prob,
            1 as vp8_prob,
        ],
        [
            57 as vp8_prob,
            17 as vp8_prob,
            5 as vp8_prob,
            71 as vp8_prob,
            102 as vp8_prob,
            57 as vp8_prob,
            53 as vp8_prob,
            41 as vp8_prob,
            49 as vp8_prob,
        ],
        [
            115 as vp8_prob,
            21 as vp8_prob,
            2 as vp8_prob,
            10 as vp8_prob,
            102 as vp8_prob,
            255 as vp8_prob,
            166 as vp8_prob,
            23 as vp8_prob,
            6 as vp8_prob,
        ],
        [
            38 as vp8_prob,
            33 as vp8_prob,
            13 as vp8_prob,
            121 as vp8_prob,
            57 as vp8_prob,
            73 as vp8_prob,
            26 as vp8_prob,
            1 as vp8_prob,
            85 as vp8_prob,
        ],
        [
            41 as vp8_prob,
            10 as vp8_prob,
            67 as vp8_prob,
            138 as vp8_prob,
            77 as vp8_prob,
            110 as vp8_prob,
            90 as vp8_prob,
            47 as vp8_prob,
            114 as vp8_prob,
        ],
        [
            101 as vp8_prob,
            29 as vp8_prob,
            16 as vp8_prob,
            10 as vp8_prob,
            85 as vp8_prob,
            128 as vp8_prob,
            101 as vp8_prob,
            196 as vp8_prob,
            26 as vp8_prob,
        ],
        [
            57 as vp8_prob,
            18 as vp8_prob,
            10 as vp8_prob,
            102 as vp8_prob,
            102 as vp8_prob,
            213 as vp8_prob,
            34 as vp8_prob,
            20 as vp8_prob,
            43 as vp8_prob,
        ],
        [
            117 as vp8_prob,
            20 as vp8_prob,
            15 as vp8_prob,
            36 as vp8_prob,
            163 as vp8_prob,
            128 as vp8_prob,
            68 as vp8_prob,
            1 as vp8_prob,
            26 as vp8_prob,
        ],
    ],
    [
        [
            138 as vp8_prob,
            31 as vp8_prob,
            36 as vp8_prob,
            171 as vp8_prob,
            27 as vp8_prob,
            166 as vp8_prob,
            38 as vp8_prob,
            44 as vp8_prob,
            229 as vp8_prob,
        ],
        [
            67 as vp8_prob,
            87 as vp8_prob,
            58 as vp8_prob,
            169 as vp8_prob,
            82 as vp8_prob,
            115 as vp8_prob,
            26 as vp8_prob,
            59 as vp8_prob,
            179 as vp8_prob,
        ],
        [
            63 as vp8_prob,
            59 as vp8_prob,
            90 as vp8_prob,
            180 as vp8_prob,
            59 as vp8_prob,
            166 as vp8_prob,
            93 as vp8_prob,
            73 as vp8_prob,
            154 as vp8_prob,
        ],
        [
            40 as vp8_prob,
            40 as vp8_prob,
            21 as vp8_prob,
            116 as vp8_prob,
            143 as vp8_prob,
            209 as vp8_prob,
            34 as vp8_prob,
            39 as vp8_prob,
            175 as vp8_prob,
        ],
        [
            57 as vp8_prob,
            46 as vp8_prob,
            22 as vp8_prob,
            24 as vp8_prob,
            128 as vp8_prob,
            1 as vp8_prob,
            54 as vp8_prob,
            17 as vp8_prob,
            37 as vp8_prob,
        ],
        [
            47 as vp8_prob,
            15 as vp8_prob,
            16 as vp8_prob,
            183 as vp8_prob,
            34 as vp8_prob,
            223 as vp8_prob,
            49 as vp8_prob,
            45 as vp8_prob,
            183 as vp8_prob,
        ],
        [
            46 as vp8_prob,
            17 as vp8_prob,
            33 as vp8_prob,
            183 as vp8_prob,
            6 as vp8_prob,
            98 as vp8_prob,
            15 as vp8_prob,
            32 as vp8_prob,
            183 as vp8_prob,
        ],
        [
            65 as vp8_prob,
            32 as vp8_prob,
            73 as vp8_prob,
            115 as vp8_prob,
            28 as vp8_prob,
            128 as vp8_prob,
            23 as vp8_prob,
            128 as vp8_prob,
            205 as vp8_prob,
        ],
        [
            40 as vp8_prob,
            3 as vp8_prob,
            9 as vp8_prob,
            115 as vp8_prob,
            51 as vp8_prob,
            192 as vp8_prob,
            18 as vp8_prob,
            6 as vp8_prob,
            223 as vp8_prob,
        ],
        [
            87 as vp8_prob,
            37 as vp8_prob,
            9 as vp8_prob,
            115 as vp8_prob,
            59 as vp8_prob,
            77 as vp8_prob,
            64 as vp8_prob,
            21 as vp8_prob,
            47 as vp8_prob,
        ],
    ],
    [
        [
            104 as vp8_prob,
            55 as vp8_prob,
            44 as vp8_prob,
            218 as vp8_prob,
            9 as vp8_prob,
            54 as vp8_prob,
            53 as vp8_prob,
            130 as vp8_prob,
            226 as vp8_prob,
        ],
        [
            64 as vp8_prob,
            90 as vp8_prob,
            70 as vp8_prob,
            205 as vp8_prob,
            40 as vp8_prob,
            41 as vp8_prob,
            23 as vp8_prob,
            26 as vp8_prob,
            57 as vp8_prob,
        ],
        [
            54 as vp8_prob,
            57 as vp8_prob,
            112 as vp8_prob,
            184 as vp8_prob,
            5 as vp8_prob,
            41 as vp8_prob,
            38 as vp8_prob,
            166 as vp8_prob,
            213 as vp8_prob,
        ],
        [
            30 as vp8_prob,
            34 as vp8_prob,
            26 as vp8_prob,
            133 as vp8_prob,
            152 as vp8_prob,
            116 as vp8_prob,
            10 as vp8_prob,
            32 as vp8_prob,
            134 as vp8_prob,
        ],
        [
            75 as vp8_prob,
            32 as vp8_prob,
            12 as vp8_prob,
            51 as vp8_prob,
            192 as vp8_prob,
            255 as vp8_prob,
            160 as vp8_prob,
            43 as vp8_prob,
            51 as vp8_prob,
        ],
        [
            39 as vp8_prob,
            19 as vp8_prob,
            53 as vp8_prob,
            221 as vp8_prob,
            26 as vp8_prob,
            114 as vp8_prob,
            32 as vp8_prob,
            73 as vp8_prob,
            255 as vp8_prob,
        ],
        [
            31 as vp8_prob,
            9 as vp8_prob,
            65 as vp8_prob,
            234 as vp8_prob,
            2 as vp8_prob,
            15 as vp8_prob,
            1 as vp8_prob,
            118 as vp8_prob,
            73 as vp8_prob,
        ],
        [
            88 as vp8_prob,
            31 as vp8_prob,
            35 as vp8_prob,
            67 as vp8_prob,
            102 as vp8_prob,
            85 as vp8_prob,
            55 as vp8_prob,
            186 as vp8_prob,
            85 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            21 as vp8_prob,
            23 as vp8_prob,
            111 as vp8_prob,
            59 as vp8_prob,
            205 as vp8_prob,
            45 as vp8_prob,
            37 as vp8_prob,
            192 as vp8_prob,
        ],
        [
            55 as vp8_prob,
            38 as vp8_prob,
            70 as vp8_prob,
            124 as vp8_prob,
            73 as vp8_prob,
            102 as vp8_prob,
            1 as vp8_prob,
            34 as vp8_prob,
            98 as vp8_prob,
        ],
    ],
    [
        [
            102 as vp8_prob,
            61 as vp8_prob,
            71 as vp8_prob,
            37 as vp8_prob,
            34 as vp8_prob,
            53 as vp8_prob,
            31 as vp8_prob,
            243 as vp8_prob,
            192 as vp8_prob,
        ],
        [
            69 as vp8_prob,
            60 as vp8_prob,
            71 as vp8_prob,
            38 as vp8_prob,
            73 as vp8_prob,
            119 as vp8_prob,
            28 as vp8_prob,
            222 as vp8_prob,
            37 as vp8_prob,
        ],
        [
            68 as vp8_prob,
            45 as vp8_prob,
            128 as vp8_prob,
            34 as vp8_prob,
            1 as vp8_prob,
            47 as vp8_prob,
            11 as vp8_prob,
            245 as vp8_prob,
            171 as vp8_prob,
        ],
        [
            62 as vp8_prob,
            17 as vp8_prob,
            19 as vp8_prob,
            70 as vp8_prob,
            146 as vp8_prob,
            85 as vp8_prob,
            55 as vp8_prob,
            62 as vp8_prob,
            70 as vp8_prob,
        ],
        [
            75 as vp8_prob,
            15 as vp8_prob,
            9 as vp8_prob,
            9 as vp8_prob,
            64 as vp8_prob,
            255 as vp8_prob,
            184 as vp8_prob,
            119 as vp8_prob,
            16 as vp8_prob,
        ],
        [
            37 as vp8_prob,
            43 as vp8_prob,
            37 as vp8_prob,
            154 as vp8_prob,
            100 as vp8_prob,
            163 as vp8_prob,
            85 as vp8_prob,
            160 as vp8_prob,
            1 as vp8_prob,
        ],
        [
            63 as vp8_prob,
            9 as vp8_prob,
            92 as vp8_prob,
            136 as vp8_prob,
            28 as vp8_prob,
            64 as vp8_prob,
            32 as vp8_prob,
            201 as vp8_prob,
            85 as vp8_prob,
        ],
        [
            86 as vp8_prob,
            6 as vp8_prob,
            28 as vp8_prob,
            5 as vp8_prob,
            64 as vp8_prob,
            255 as vp8_prob,
            25 as vp8_prob,
            248 as vp8_prob,
            1 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            8 as vp8_prob,
            17 as vp8_prob,
            132 as vp8_prob,
            137 as vp8_prob,
            255 as vp8_prob,
            55 as vp8_prob,
            116 as vp8_prob,
            128 as vp8_prob,
        ],
        [
            58 as vp8_prob,
            15 as vp8_prob,
            20 as vp8_prob,
            82 as vp8_prob,
            135 as vp8_prob,
            57 as vp8_prob,
            26 as vp8_prob,
            121 as vp8_prob,
            40 as vp8_prob,
        ],
    ],
    [
        [
            164 as vp8_prob,
            50 as vp8_prob,
            31 as vp8_prob,
            137 as vp8_prob,
            154 as vp8_prob,
            133 as vp8_prob,
            25 as vp8_prob,
            35 as vp8_prob,
            218 as vp8_prob,
        ],
        [
            51 as vp8_prob,
            103 as vp8_prob,
            44 as vp8_prob,
            131 as vp8_prob,
            131 as vp8_prob,
            123 as vp8_prob,
            31 as vp8_prob,
            6 as vp8_prob,
            158 as vp8_prob,
        ],
        [
            86 as vp8_prob,
            40 as vp8_prob,
            64 as vp8_prob,
            135 as vp8_prob,
            148 as vp8_prob,
            224 as vp8_prob,
            45 as vp8_prob,
            183 as vp8_prob,
            128 as vp8_prob,
        ],
        [
            22 as vp8_prob,
            26 as vp8_prob,
            17 as vp8_prob,
            131 as vp8_prob,
            240 as vp8_prob,
            154 as vp8_prob,
            14 as vp8_prob,
            1 as vp8_prob,
            209 as vp8_prob,
        ],
        [
            83 as vp8_prob,
            12 as vp8_prob,
            13 as vp8_prob,
            54 as vp8_prob,
            192 as vp8_prob,
            255 as vp8_prob,
            68 as vp8_prob,
            47 as vp8_prob,
            28 as vp8_prob,
        ],
        [
            45 as vp8_prob,
            16 as vp8_prob,
            21 as vp8_prob,
            91 as vp8_prob,
            64 as vp8_prob,
            222 as vp8_prob,
            7 as vp8_prob,
            1 as vp8_prob,
            197 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            21 as vp8_prob,
            39 as vp8_prob,
            155 as vp8_prob,
            60 as vp8_prob,
            138 as vp8_prob,
            23 as vp8_prob,
            102 as vp8_prob,
            213 as vp8_prob,
        ],
        [
            85 as vp8_prob,
            26 as vp8_prob,
            85 as vp8_prob,
            85 as vp8_prob,
            128 as vp8_prob,
            128 as vp8_prob,
            32 as vp8_prob,
            146 as vp8_prob,
            171 as vp8_prob,
        ],
        [
            18 as vp8_prob,
            11 as vp8_prob,
            7 as vp8_prob,
            63 as vp8_prob,
            144 as vp8_prob,
            171 as vp8_prob,
            4 as vp8_prob,
            4 as vp8_prob,
            246 as vp8_prob,
        ],
        [
            35 as vp8_prob,
            27 as vp8_prob,
            10 as vp8_prob,
            146 as vp8_prob,
            174 as vp8_prob,
            171 as vp8_prob,
            12 as vp8_prob,
            26 as vp8_prob,
            128 as vp8_prob,
        ],
    ],
    [
        [
            190 as vp8_prob,
            80 as vp8_prob,
            35 as vp8_prob,
            99 as vp8_prob,
            180 as vp8_prob,
            80 as vp8_prob,
            126 as vp8_prob,
            54 as vp8_prob,
            45 as vp8_prob,
        ],
        [
            85 as vp8_prob,
            126 as vp8_prob,
            47 as vp8_prob,
            87 as vp8_prob,
            176 as vp8_prob,
            51 as vp8_prob,
            41 as vp8_prob,
            20 as vp8_prob,
            32 as vp8_prob,
        ],
        [
            101 as vp8_prob,
            75 as vp8_prob,
            128 as vp8_prob,
            139 as vp8_prob,
            118 as vp8_prob,
            146 as vp8_prob,
            116 as vp8_prob,
            128 as vp8_prob,
            85 as vp8_prob,
        ],
        [
            56 as vp8_prob,
            41 as vp8_prob,
            15 as vp8_prob,
            176 as vp8_prob,
            236 as vp8_prob,
            85 as vp8_prob,
            37 as vp8_prob,
            9 as vp8_prob,
            62 as vp8_prob,
        ],
        [
            146 as vp8_prob,
            36 as vp8_prob,
            19 as vp8_prob,
            30 as vp8_prob,
            171 as vp8_prob,
            255 as vp8_prob,
            97 as vp8_prob,
            27 as vp8_prob,
            20 as vp8_prob,
        ],
        [
            71 as vp8_prob,
            30 as vp8_prob,
            17 as vp8_prob,
            119 as vp8_prob,
            118 as vp8_prob,
            255 as vp8_prob,
            17 as vp8_prob,
            18 as vp8_prob,
            138 as vp8_prob,
        ],
        [
            101 as vp8_prob,
            38 as vp8_prob,
            60 as vp8_prob,
            138 as vp8_prob,
            55 as vp8_prob,
            70 as vp8_prob,
            43 as vp8_prob,
            26 as vp8_prob,
            142 as vp8_prob,
        ],
        [
            138 as vp8_prob,
            45 as vp8_prob,
            61 as vp8_prob,
            62 as vp8_prob,
            219 as vp8_prob,
            1 as vp8_prob,
            81 as vp8_prob,
            188 as vp8_prob,
            64 as vp8_prob,
        ],
        [
            32 as vp8_prob,
            41 as vp8_prob,
            20 as vp8_prob,
            117 as vp8_prob,
            151 as vp8_prob,
            142 as vp8_prob,
            20 as vp8_prob,
            21 as vp8_prob,
            163 as vp8_prob,
        ],
        [
            112 as vp8_prob,
            19 as vp8_prob,
            12 as vp8_prob,
            61 as vp8_prob,
            195 as vp8_prob,
            128 as vp8_prob,
            48 as vp8_prob,
            4 as vp8_prob,
            24 as vp8_prob,
        ],
    ],
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_mv_cont(mut l: *const int_mv, mut a: *const int_mv) -> i32 {
    unsafe {
        let mut lez: i32 = ((*l).as_int == 0 as uint32_t) as i32;
        let mut aez: i32 = ((*a).as_int == 0 as uint32_t) as i32;
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
static mut sub_mv_ref_prob: [vp8_prob; 3] = [180 as vp8_prob, 162 as vp8_prob, 25 as vp8_prob];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_prob2: [[vp8_prob; 3]; 5] = [
    [147 as vp8_prob, 136 as vp8_prob, 18 as vp8_prob],
    [106 as vp8_prob, 145 as vp8_prob, 1 as vp8_prob],
    [179 as vp8_prob, 121 as vp8_prob, 1 as vp8_prob],
    [223 as vp8_prob, 1 as vp8_prob, 34 as vp8_prob],
    [208 as vp8_prob, 1 as vp8_prob, 1 as vp8_prob],
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplits: [vp8_mbsplit; 4] = [
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
pub static mut vp8_mbsplit_probs: [vp8_prob; 3] =
    [110 as vp8_prob, 111 as vp8_prob, 150 as vp8_prob];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_tree: [vp8_tree_index; 18] = [
    -(B_DC_PRED as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    -(B_TM_PRED as i32) as vp8_tree_index,
    4 as vp8_tree_index,
    -(B_VE_PRED as i32) as vp8_tree_index,
    6 as vp8_tree_index,
    8 as vp8_tree_index,
    12 as vp8_tree_index,
    -(B_HE_PRED as i32) as vp8_tree_index,
    10 as vp8_tree_index,
    -(B_RD_PRED as i32) as vp8_tree_index,
    -(B_VR_PRED as i32) as vp8_tree_index,
    -(B_LD_PRED as i32) as vp8_tree_index,
    14 as vp8_tree_index,
    -(B_VL_PRED as i32) as vp8_tree_index,
    16 as vp8_tree_index,
    -(B_HD_PRED as i32) as vp8_tree_index,
    -(B_HU_PRED as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_tree: [vp8_tree_index; 8] = [
    -(DC_PRED as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    4 as vp8_tree_index,
    6 as vp8_tree_index,
    -(V_PRED as i32) as vp8_tree_index,
    -(H_PRED as i32) as vp8_tree_index,
    -(TM_PRED as i32) as vp8_tree_index,
    -(B_PRED as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_tree: [vp8_tree_index; 8] = [
    -(B_PRED as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    4 as vp8_tree_index,
    6 as vp8_tree_index,
    -(DC_PRED as i32) as vp8_tree_index,
    -(V_PRED as i32) as vp8_tree_index,
    -(H_PRED as i32) as vp8_tree_index,
    -(TM_PRED as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_tree: [vp8_tree_index; 6] = [
    -(DC_PRED as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    -(V_PRED as i32) as vp8_tree_index,
    4 as vp8_tree_index,
    -(H_PRED as i32) as vp8_tree_index,
    -(TM_PRED as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_tree: [vp8_tree_index; 6] = [
    -(3 as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    -(2 as i32) as vp8_tree_index,
    4 as vp8_tree_index,
    -(0 as i32) as vp8_tree_index,
    -(1 as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_tree: [vp8_tree_index; 8] = [
    -(ZEROMV as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    -(NEARESTMV as i32) as vp8_tree_index,
    4 as vp8_tree_index,
    -(NEARMV as i32) as vp8_tree_index,
    6 as vp8_tree_index,
    -(NEWMV as i32) as vp8_tree_index,
    -(SPLITMV as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_tree: [vp8_tree_index; 6] = [
    -(LEFT4X4 as i32) as vp8_tree_index,
    2 as vp8_tree_index,
    -(ABOVE4X4 as i32) as vp8_tree_index,
    4 as vp8_tree_index,
    -(ZERO4X4 as i32) as vp8_tree_index,
    -(NEW4X4 as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvtree: [vp8_tree_index; 14] = [
    2 as vp8_tree_index,
    8 as vp8_tree_index,
    4 as vp8_tree_index,
    6 as vp8_tree_index,
    -(0 as i32) as vp8_tree_index,
    -(1 as i32) as vp8_tree_index,
    -(2 as i32) as vp8_tree_index,
    -(3 as i32) as vp8_tree_index,
    10 as vp8_tree_index,
    12 as vp8_tree_index,
    -(4 as i32) as vp8_tree_index,
    -(5 as i32) as vp8_tree_index,
    -(6 as i32) as vp8_tree_index,
    -(7 as i32) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_mbmode_probs(mut x: *mut VP8_COMMON) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_ymode_prob as *const vp8_prob as *const c_void as *const u8,
            &raw mut (*x).fc.ymode_prob as *mut vp8_prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[vp8_prob; 4]>() as size_t,
        );
        core::ptr::copy_nonoverlapping(
            &raw const vp8_uv_mode_prob as *const vp8_prob as *const c_void as *const u8,
            &raw mut (*x).fc.uv_mode_prob as *mut vp8_prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
        );
        core::ptr::copy_nonoverlapping(
            &raw const sub_mv_ref_prob as *const vp8_prob as *const c_void as *const u8,
            &raw mut (*x).fc.sub_mv_ref_prob as *mut vp8_prob as *mut c_void as *mut u8,
            ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_default_bmode_probs(mut dest: *mut vp8_prob) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            &raw const vp8_bmode_prob as *const vp8_prob as *const c_void as *const u8,
            dest as *mut c_void as *mut u8,
            ::core::mem::size_of::<[vp8_prob; 9]>() as size_t,
        );
    }
}
