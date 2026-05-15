unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
pub type uint32_t = u32;
pub type B_PREDICTION_MODE = ::core::ffi::c_uint;
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
    pub has_detail: ::core::ffi::c_int,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: ::core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [::core::ffi::c_int; 48];
pub type vpx_codec_err_t = ::core::ffi::c_uint;
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
pub type vp8_prob = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type ENTROPY_CONTEXT = ::core::ffi::c_char;
pub type FRAME_TYPE = ::core::ffi::c_uint;
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
    pub y_width: ::core::ffi::c_int,
    pub y_height: ::core::ffi::c_int,
    pub y_crop_width: ::core::ffi::c_int,
    pub y_crop_height: ::core::ffi::c_int,
    pub y_stride: ::core::ffi::c_int,
    pub uv_width: ::core::ffi::c_int,
    pub uv_height: ::core::ffi::c_int,
    pub uv_crop_width: ::core::ffi::c_int,
    pub uv_crop_height: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub alpha_width: ::core::ffi::c_int,
    pub alpha_height: ::core::ffi::c_int,
    pub alpha_stride: ::core::ffi::c_int,
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: ::core::ffi::c_int,
    pub frame_size: size_t,
    pub subsampling_x: ::core::ffi::c_int,
    pub subsampling_y: ::core::ffi::c_int,
    pub bit_depth: ::core::ffi::c_uint,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: ::core::ffi::c_int,
    pub render_height: ::core::ffi::c_int,
    pub corrupted: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = ::core::ffi::c_uint;
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
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[::core::ffi::c_uchar; 16]; 64],
    pub blim: [[::core::ffi::c_uchar; 16]; 64],
    pub lim: [[::core::ffi::c_uchar; 16]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 16]; 4],
    pub lvl: [[[::core::ffi::c_uchar; 4]; 4]; 4],
    pub hev_thr_lut: [[::core::ffi::c_uchar; 64]; 2],
    pub mode_lf_lut: [::core::ffi::c_uchar; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub Y1dequant: [[::core::ffi::c_short; 2]; 128],
    pub Y2dequant: [[::core::ffi::c_short; 2]; 128],
    pub UVdequant: [[::core::ffi::c_short; 2]; 128],
    pub Width: ::core::ffi::c_int,
    pub Height: ::core::ffi::c_int,
    pub horiz_scale: ::core::ffi::c_int,
    pub vert_scale: ::core::ffi::c_int,
    pub clamp_type: CLAMP_TYPE,
    pub frame_to_show: *mut YV12_BUFFER_CONFIG,
    pub yv12_fb: [YV12_BUFFER_CONFIG; 4],
    pub fb_idx_ref_cnt: [::core::ffi::c_int; 4],
    pub new_fb_idx: ::core::ffi::c_int,
    pub lst_fb_idx: ::core::ffi::c_int,
    pub gld_fb_idx: ::core::ffi::c_int,
    pub alt_fb_idx: ::core::ffi::c_int,
    pub temp_scale_frame: YV12_BUFFER_CONFIG,
    pub last_frame_type: FRAME_TYPE,
    pub frame_type: FRAME_TYPE,
    pub show_frame: ::core::ffi::c_int,
    pub frame_flags: ::core::ffi::c_int,
    pub MBs: ::core::ffi::c_int,
    pub mb_rows: ::core::ffi::c_int,
    pub mb_cols: ::core::ffi::c_int,
    pub mode_info_stride: ::core::ffi::c_int,
    pub mb_no_coeff_skip: ::core::ffi::c_int,
    pub no_lpf: ::core::ffi::c_int,
    pub use_bilinear_mc_filter: ::core::ffi::c_int,
    pub full_pixel: ::core::ffi::c_int,
    pub base_qindex: ::core::ffi::c_int,
    pub y1dc_delta_q: ::core::ffi::c_int,
    pub y2dc_delta_q: ::core::ffi::c_int,
    pub y2ac_delta_q: ::core::ffi::c_int,
    pub uvdc_delta_q: ::core::ffi::c_int,
    pub uvac_delta_q: ::core::ffi::c_int,
    pub mip: *mut MODE_INFO,
    pub mi: *mut MODE_INFO,
    pub show_frame_mi: *mut MODE_INFO,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: loop_filter_info_n,
    pub filter_level: ::core::ffi::c_int,
    pub last_sharpness_level: ::core::ffi::c_int,
    pub sharpness_level: ::core::ffi::c_int,
    pub refresh_last_frame: ::core::ffi::c_int,
    pub refresh_golden_frame: ::core::ffi::c_int,
    pub refresh_alt_ref_frame: ::core::ffi::c_int,
    pub copy_buffer_to_gf: ::core::ffi::c_int,
    pub copy_buffer_to_arf: ::core::ffi::c_int,
    pub refresh_entropy_probs: ::core::ffi::c_int,
    pub ref_frame_sign_bias: [::core::ffi::c_int; 4],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: ::core::ffi::c_uint,
    pub version: ::core::ffi::c_int,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: ::core::ffi::c_int,
}
pub type TOKEN_PARTITION = ::core::ffi::c_uint;
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
pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type vp8_tree_index = ::core::ffi::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: ::core::ffi::c_int,
    pub Len: ::core::ffi::c_int,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SUBMVREF_LEFT_ABOVE_ZED: C2RustUnnamed_0 = 4;
pub const SUBMVREF_LEFT_ABOVE_SAME: C2RustUnnamed_0 = 3;
pub const SUBMVREF_ABOVE_ZED: C2RustUnnamed_0 = 2;
pub const SUBMVREF_LEFT_ZED: C2RustUnnamed_0 = 1;
pub const SUBMVREF_NORMAL: C2RustUnnamed_0 = 0;
pub type vp8_mbsplit = [::core::ffi::c_int; 16];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_encodings: [vp8_token_struct; 10] = [
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 2 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 28 as ::core::ffi::c_int,
        Len: 5 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 30 as ::core::ffi::c_int,
        Len: 5 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 58 as ::core::ffi::c_int,
        Len: 6 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 59 as ::core::ffi::c_int,
        Len: 6 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 62 as ::core::ffi::c_int,
        Len: 6 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 126 as ::core::ffi::c_int,
        Len: 7 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 127 as ::core::ffi::c_int,
        Len: 7 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_encodings: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 4 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 5 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_encodings: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 4 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 5 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_encodings: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 2 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_encodings: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 2 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_encoding_array: [vp8_token_struct; 5] = [
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 2 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 14 as ::core::ffi::c_int,
        Len: 4 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 15 as ::core::ffi::c_int,
        Len: 4 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_encoding_array: [vp8_token_struct; 4] = [
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 1 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 2 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvencodings: [vp8_token_struct; 8] = [
    vp8_token_struct {
        value: 0 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 1 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 2 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 3 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 4 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 5 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 6 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
    vp8_token_struct {
        value: 7 as ::core::ffi::c_int,
        Len: 3 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_prob: [vp8_prob; 4] = [
    112 as ::core::ffi::c_int as vp8_prob,
    86 as ::core::ffi::c_int as vp8_prob,
    140 as ::core::ffi::c_int as vp8_prob,
    37 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_prob: [vp8_prob; 4] = [
    145 as ::core::ffi::c_int as vp8_prob,
    156 as ::core::ffi::c_int as vp8_prob,
    163 as ::core::ffi::c_int as vp8_prob,
    128 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_prob: [vp8_prob; 3] = [
    162 as ::core::ffi::c_int as vp8_prob,
    101 as ::core::ffi::c_int as vp8_prob,
    204 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_uv_mode_prob: [vp8_prob; 3] = [
    142 as ::core::ffi::c_int as vp8_prob,
    114 as ::core::ffi::c_int as vp8_prob,
    183 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_prob: [vp8_prob; 9] = [
    120 as ::core::ffi::c_int as vp8_prob,
    90 as ::core::ffi::c_int as vp8_prob,
    79 as ::core::ffi::c_int as vp8_prob,
    133 as ::core::ffi::c_int as vp8_prob,
    87 as ::core::ffi::c_int as vp8_prob,
    85 as ::core::ffi::c_int as vp8_prob,
    80 as ::core::ffi::c_int as vp8_prob,
    111 as ::core::ffi::c_int as vp8_prob,
    151 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_bmode_prob: [[[vp8_prob; 9]; 10]; 10] = [
    [
        [
            231 as ::core::ffi::c_int as vp8_prob,
            120 as ::core::ffi::c_int as vp8_prob,
            48 as ::core::ffi::c_int as vp8_prob,
            89 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            113 as ::core::ffi::c_int as vp8_prob,
            120 as ::core::ffi::c_int as vp8_prob,
            152 as ::core::ffi::c_int as vp8_prob,
            112 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            152 as ::core::ffi::c_int as vp8_prob,
            179 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            126 as ::core::ffi::c_int as vp8_prob,
            170 as ::core::ffi::c_int as vp8_prob,
            118 as ::core::ffi::c_int as vp8_prob,
            46 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
            95 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            175 as ::core::ffi::c_int as vp8_prob,
            69 as ::core::ffi::c_int as vp8_prob,
            143 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            82 as ::core::ffi::c_int as vp8_prob,
            72 as ::core::ffi::c_int as vp8_prob,
            155 as ::core::ffi::c_int as vp8_prob,
            103 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            58 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            218 as ::core::ffi::c_int as vp8_prob,
            189 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            13 as ::core::ffi::c_int as vp8_prob,
            152 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            144 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            213 as ::core::ffi::c_int as vp8_prob,
            144 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            114 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            163 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            195 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            173 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            121 as ::core::ffi::c_int as vp8_prob,
            24 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
            195 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            62 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            170 as ::core::ffi::c_int as vp8_prob,
            46 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            136 as ::core::ffi::c_int as vp8_prob,
            160 as ::core::ffi::c_int as vp8_prob,
            33 as ::core::ffi::c_int as vp8_prob,
            206 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            63 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            8 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
            208 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            226 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            81 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            11 as ::core::ffi::c_int as vp8_prob,
            96 as ::core::ffi::c_int as vp8_prob,
            182 as ::core::ffi::c_int as vp8_prob,
            84 as ::core::ffi::c_int as vp8_prob,
            29 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            36 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            134 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            89 as ::core::ffi::c_int as vp8_prob,
            137 as ::core::ffi::c_int as vp8_prob,
            98 as ::core::ffi::c_int as vp8_prob,
            101 as ::core::ffi::c_int as vp8_prob,
            106 as ::core::ffi::c_int as vp8_prob,
            165 as ::core::ffi::c_int as vp8_prob,
            148 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            72 as ::core::ffi::c_int as vp8_prob,
            187 as ::core::ffi::c_int as vp8_prob,
            100 as ::core::ffi::c_int as vp8_prob,
            130 as ::core::ffi::c_int as vp8_prob,
            157 as ::core::ffi::c_int as vp8_prob,
            111 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            75 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            66 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            167 as ::core::ffi::c_int as vp8_prob,
            99 as ::core::ffi::c_int as vp8_prob,
            74 as ::core::ffi::c_int as vp8_prob,
            62 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            234 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            41 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            178 as ::core::ffi::c_int as vp8_prob,
            241 as ::core::ffi::c_int as vp8_prob,
            141 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            8 as ::core::ffi::c_int as vp8_prob,
            107 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            104 as ::core::ffi::c_int as vp8_prob,
            79 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            27 as ::core::ffi::c_int as vp8_prob,
            217 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            87 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            7 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            74 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            146 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            49 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            157 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            65 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            105 as ::core::ffi::c_int as vp8_prob,
            160 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            52 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            87 as ::core::ffi::c_int as vp8_prob,
            68 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            186 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            47 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            14 as ::core::ffi::c_int as vp8_prob,
            110 as ::core::ffi::c_int as vp8_prob,
            182 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            194 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            66 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            197 as ::core::ffi::c_int as vp8_prob,
            189 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            22 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            88 as ::core::ffi::c_int as vp8_prob,
            88 as ::core::ffi::c_int as vp8_prob,
            147 as ::core::ffi::c_int as vp8_prob,
            150 as ::core::ffi::c_int as vp8_prob,
            42 as ::core::ffi::c_int as vp8_prob,
            46 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            196 as ::core::ffi::c_int as vp8_prob,
            205 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            43 as ::core::ffi::c_int as vp8_prob,
            97 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            117 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            35 as ::core::ffi::c_int as vp8_prob,
            179 as ::core::ffi::c_int as vp8_prob,
            61 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            39 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            200 as ::core::ffi::c_int as vp8_prob,
            87 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            232 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            104 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            29 as ::core::ffi::c_int as vp8_prob,
            93 as ::core::ffi::c_int as vp8_prob,
            77 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            107 as ::core::ffi::c_int as vp8_prob,
            54 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            81 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            39 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            58 as ::core::ffi::c_int as vp8_prob,
            165 as ::core::ffi::c_int as vp8_prob,
            90 as ::core::ffi::c_int as vp8_prob,
            98 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            34 as ::core::ffi::c_int as vp8_prob,
            22 as ::core::ffi::c_int as vp8_prob,
            116 as ::core::ffi::c_int as vp8_prob,
            206 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            68 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            106 as ::core::ffi::c_int as vp8_prob,
            22 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            36 as ::core::ffi::c_int as vp8_prob,
            225 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            34 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            132 as ::core::ffi::c_int as vp8_prob,
            188 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            76 as ::core::ffi::c_int as vp8_prob,
            124 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            62 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            78 as ::core::ffi::c_int as vp8_prob,
            95 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
            50 as ::core::ffi::c_int as vp8_prob,
            48 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            193 as ::core::ffi::c_int as vp8_prob,
            101 as ::core::ffi::c_int as vp8_prob,
            35 as ::core::ffi::c_int as vp8_prob,
            159 as ::core::ffi::c_int as vp8_prob,
            215 as ::core::ffi::c_int as vp8_prob,
            111 as ::core::ffi::c_int as vp8_prob,
            89 as ::core::ffi::c_int as vp8_prob,
            46 as ::core::ffi::c_int as vp8_prob,
            111 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            60 as ::core::ffi::c_int as vp8_prob,
            148 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            172 as ::core::ffi::c_int as vp8_prob,
            219 as ::core::ffi::c_int as vp8_prob,
            228 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            111 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            112 as ::core::ffi::c_int as vp8_prob,
            113 as ::core::ffi::c_int as vp8_prob,
            77 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            179 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            120 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            40 as ::core::ffi::c_int as vp8_prob,
            42 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            196 as ::core::ffi::c_int as vp8_prob,
            245 as ::core::ffi::c_int as vp8_prob,
            209 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            109 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            100 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
            8 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            88 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            29 as ::core::ffi::c_int as vp8_prob,
            140 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            213 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            61 as ::core::ffi::c_int as vp8_prob,
            63 as ::core::ffi::c_int as vp8_prob,
            30 as ::core::ffi::c_int as vp8_prob,
            155 as ::core::ffi::c_int as vp8_prob,
            67 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            68 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            209 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            142 as ::core::ffi::c_int as vp8_prob,
            78 as ::core::ffi::c_int as vp8_prob,
            78 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            197 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            41 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            5 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            211 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            4 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            221 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            51 as ::core::ffi::c_int as vp8_prob,
            50 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            168 as ::core::ffi::c_int as vp8_prob,
            209 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            82 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            125 as ::core::ffi::c_int as vp8_prob,
            98 as ::core::ffi::c_int as vp8_prob,
            42 as ::core::ffi::c_int as vp8_prob,
            88 as ::core::ffi::c_int as vp8_prob,
            104 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            117 as ::core::ffi::c_int as vp8_prob,
            175 as ::core::ffi::c_int as vp8_prob,
            82 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            95 as ::core::ffi::c_int as vp8_prob,
            84 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            89 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            100 as ::core::ffi::c_int as vp8_prob,
            113 as ::core::ffi::c_int as vp8_prob,
            101 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            75 as ::core::ffi::c_int as vp8_prob,
            79 as ::core::ffi::c_int as vp8_prob,
            123 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            81 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            57 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            5 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            49 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            115 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            2 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            6 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            38 as ::core::ffi::c_int as vp8_prob,
            33 as ::core::ffi::c_int as vp8_prob,
            13 as ::core::ffi::c_int as vp8_prob,
            121 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            41 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            67 as ::core::ffi::c_int as vp8_prob,
            138 as ::core::ffi::c_int as vp8_prob,
            77 as ::core::ffi::c_int as vp8_prob,
            110 as ::core::ffi::c_int as vp8_prob,
            90 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            101 as ::core::ffi::c_int as vp8_prob,
            29 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            101 as ::core::ffi::c_int as vp8_prob,
            196 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            57 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            213 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            117 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            36 as ::core::ffi::c_int as vp8_prob,
            163 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            68 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            138 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            36 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            27 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            229 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            67 as ::core::ffi::c_int as vp8_prob,
            87 as ::core::ffi::c_int as vp8_prob,
            58 as ::core::ffi::c_int as vp8_prob,
            169 as ::core::ffi::c_int as vp8_prob,
            82 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            59 as ::core::ffi::c_int as vp8_prob,
            179 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            63 as ::core::ffi::c_int as vp8_prob,
            59 as ::core::ffi::c_int as vp8_prob,
            90 as ::core::ffi::c_int as vp8_prob,
            180 as ::core::ffi::c_int as vp8_prob,
            59 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            93 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            40 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            116 as ::core::ffi::c_int as vp8_prob,
            143 as ::core::ffi::c_int as vp8_prob,
            209 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            39 as ::core::ffi::c_int as vp8_prob,
            175 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            57 as ::core::ffi::c_int as vp8_prob,
            46 as ::core::ffi::c_int as vp8_prob,
            22 as ::core::ffi::c_int as vp8_prob,
            24 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            54 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            47 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            223 as ::core::ffi::c_int as vp8_prob,
            49 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            46 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            33 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            6 as ::core::ffi::c_int as vp8_prob,
            98 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            65 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            205 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            40 as ::core::ffi::c_int as vp8_prob,
            3 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            6 as ::core::ffi::c_int as vp8_prob,
            223 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            87 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            115 as ::core::ffi::c_int as vp8_prob,
            59 as ::core::ffi::c_int as vp8_prob,
            77 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            104 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            218 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            54 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            130 as ::core::ffi::c_int as vp8_prob,
            226 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            64 as ::core::ffi::c_int as vp8_prob,
            90 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
            205 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            54 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
            112 as ::core::ffi::c_int as vp8_prob,
            184 as ::core::ffi::c_int as vp8_prob,
            5 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            166 as ::core::ffi::c_int as vp8_prob,
            213 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            30 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            133 as ::core::ffi::c_int as vp8_prob,
            152 as ::core::ffi::c_int as vp8_prob,
            116 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            134 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            75 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            160 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            39 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            221 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            114 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            31 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            65 as ::core::ffi::c_int as vp8_prob,
            234 as ::core::ffi::c_int as vp8_prob,
            2 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            118 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            88 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            35 as ::core::ffi::c_int as vp8_prob,
            67 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            186 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            111 as ::core::ffi::c_int as vp8_prob,
            59 as ::core::ffi::c_int as vp8_prob,
            205 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            55 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
            124 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            98 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            102 as ::core::ffi::c_int as vp8_prob,
            61 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            53 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            243 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            69 as ::core::ffi::c_int as vp8_prob,
            60 as ::core::ffi::c_int as vp8_prob,
            71 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            73 as ::core::ffi::c_int as vp8_prob,
            119 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
            222 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            68 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            34 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
            11 as ::core::ffi::c_int as vp8_prob,
            245 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            62 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
            146 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            62 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            75 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            184 as ::core::ffi::c_int as vp8_prob,
            119 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            37 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
            100 as ::core::ffi::c_int as vp8_prob,
            163 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            160 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            63 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            92 as ::core::ffi::c_int as vp8_prob,
            136 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            201 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            86 as ::core::ffi::c_int as vp8_prob,
            6 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
            5 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            248 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            8 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            132 as ::core::ffi::c_int as vp8_prob,
            137 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            116 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            58 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            82 as ::core::ffi::c_int as vp8_prob,
            135 as ::core::ffi::c_int as vp8_prob,
            57 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            121 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            164 as ::core::ffi::c_int as vp8_prob,
            50 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            137 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
            133 as ::core::ffi::c_int as vp8_prob,
            25 as ::core::ffi::c_int as vp8_prob,
            35 as ::core::ffi::c_int as vp8_prob,
            218 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            51 as ::core::ffi::c_int as vp8_prob,
            103 as ::core::ffi::c_int as vp8_prob,
            44 as ::core::ffi::c_int as vp8_prob,
            131 as ::core::ffi::c_int as vp8_prob,
            131 as ::core::ffi::c_int as vp8_prob,
            123 as ::core::ffi::c_int as vp8_prob,
            31 as ::core::ffi::c_int as vp8_prob,
            6 as ::core::ffi::c_int as vp8_prob,
            158 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            86 as ::core::ffi::c_int as vp8_prob,
            40 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            135 as ::core::ffi::c_int as vp8_prob,
            148 as ::core::ffi::c_int as vp8_prob,
            224 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            183 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            22 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            131 as ::core::ffi::c_int as vp8_prob,
            240 as ::core::ffi::c_int as vp8_prob,
            154 as ::core::ffi::c_int as vp8_prob,
            14 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            209 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            83 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            13 as ::core::ffi::c_int as vp8_prob,
            54 as ::core::ffi::c_int as vp8_prob,
            192 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            68 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
            28 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            45 as ::core::ffi::c_int as vp8_prob,
            16 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            91 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
            222 as ::core::ffi::c_int as vp8_prob,
            7 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            197 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            39 as ::core::ffi::c_int as vp8_prob,
            155 as ::core::ffi::c_int as vp8_prob,
            60 as ::core::ffi::c_int as vp8_prob,
            138 as ::core::ffi::c_int as vp8_prob,
            23 as ::core::ffi::c_int as vp8_prob,
            102 as ::core::ffi::c_int as vp8_prob,
            213 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            85 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
            146 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            18 as ::core::ffi::c_int as vp8_prob,
            11 as ::core::ffi::c_int as vp8_prob,
            7 as ::core::ffi::c_int as vp8_prob,
            63 as ::core::ffi::c_int as vp8_prob,
            144 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            4 as ::core::ffi::c_int as vp8_prob,
            4 as ::core::ffi::c_int as vp8_prob,
            246 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            35 as ::core::ffi::c_int as vp8_prob,
            27 as ::core::ffi::c_int as vp8_prob,
            10 as ::core::ffi::c_int as vp8_prob,
            146 as ::core::ffi::c_int as vp8_prob,
            174 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
    [
        [
            190 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
            35 as ::core::ffi::c_int as vp8_prob,
            99 as ::core::ffi::c_int as vp8_prob,
            180 as ::core::ffi::c_int as vp8_prob,
            80 as ::core::ffi::c_int as vp8_prob,
            126 as ::core::ffi::c_int as vp8_prob,
            54 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            85 as ::core::ffi::c_int as vp8_prob,
            126 as ::core::ffi::c_int as vp8_prob,
            47 as ::core::ffi::c_int as vp8_prob,
            87 as ::core::ffi::c_int as vp8_prob,
            176 as ::core::ffi::c_int as vp8_prob,
            51 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            32 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            101 as ::core::ffi::c_int as vp8_prob,
            75 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            139 as ::core::ffi::c_int as vp8_prob,
            118 as ::core::ffi::c_int as vp8_prob,
            146 as ::core::ffi::c_int as vp8_prob,
            116 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            56 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            15 as ::core::ffi::c_int as vp8_prob,
            176 as ::core::ffi::c_int as vp8_prob,
            236 as ::core::ffi::c_int as vp8_prob,
            85 as ::core::ffi::c_int as vp8_prob,
            37 as ::core::ffi::c_int as vp8_prob,
            9 as ::core::ffi::c_int as vp8_prob,
            62 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            146 as ::core::ffi::c_int as vp8_prob,
            36 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            30 as ::core::ffi::c_int as vp8_prob,
            171 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            97 as ::core::ffi::c_int as vp8_prob,
            27 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            71 as ::core::ffi::c_int as vp8_prob,
            30 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            119 as ::core::ffi::c_int as vp8_prob,
            118 as ::core::ffi::c_int as vp8_prob,
            255 as ::core::ffi::c_int as vp8_prob,
            17 as ::core::ffi::c_int as vp8_prob,
            18 as ::core::ffi::c_int as vp8_prob,
            138 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            101 as ::core::ffi::c_int as vp8_prob,
            38 as ::core::ffi::c_int as vp8_prob,
            60 as ::core::ffi::c_int as vp8_prob,
            138 as ::core::ffi::c_int as vp8_prob,
            55 as ::core::ffi::c_int as vp8_prob,
            70 as ::core::ffi::c_int as vp8_prob,
            43 as ::core::ffi::c_int as vp8_prob,
            26 as ::core::ffi::c_int as vp8_prob,
            142 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            138 as ::core::ffi::c_int as vp8_prob,
            45 as ::core::ffi::c_int as vp8_prob,
            61 as ::core::ffi::c_int as vp8_prob,
            62 as ::core::ffi::c_int as vp8_prob,
            219 as ::core::ffi::c_int as vp8_prob,
            1 as ::core::ffi::c_int as vp8_prob,
            81 as ::core::ffi::c_int as vp8_prob,
            188 as ::core::ffi::c_int as vp8_prob,
            64 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            32 as ::core::ffi::c_int as vp8_prob,
            41 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            117 as ::core::ffi::c_int as vp8_prob,
            151 as ::core::ffi::c_int as vp8_prob,
            142 as ::core::ffi::c_int as vp8_prob,
            20 as ::core::ffi::c_int as vp8_prob,
            21 as ::core::ffi::c_int as vp8_prob,
            163 as ::core::ffi::c_int as vp8_prob,
        ],
        [
            112 as ::core::ffi::c_int as vp8_prob,
            19 as ::core::ffi::c_int as vp8_prob,
            12 as ::core::ffi::c_int as vp8_prob,
            61 as ::core::ffi::c_int as vp8_prob,
            195 as ::core::ffi::c_int as vp8_prob,
            128 as ::core::ffi::c_int as vp8_prob,
            48 as ::core::ffi::c_int as vp8_prob,
            4 as ::core::ffi::c_int as vp8_prob,
            24 as ::core::ffi::c_int as vp8_prob,
        ],
    ],
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_mv_cont(
    mut l: *const int_mv,
    mut a: *const int_mv,
) -> ::core::ffi::c_int { unsafe {
    let mut lez: ::core::ffi::c_int = ((*l).as_int == 0 as uint32_t) as ::core::ffi::c_int;
    let mut aez: ::core::ffi::c_int = ((*a).as_int == 0 as uint32_t) as ::core::ffi::c_int;
    let mut lea: ::core::ffi::c_int = ((*l).as_int == (*a).as_int) as ::core::ffi::c_int;
    if lea != 0 && lez != 0 {
        return SUBMVREF_LEFT_ABOVE_ZED as ::core::ffi::c_int;
    }
    if lea != 0 {
        return SUBMVREF_LEFT_ABOVE_SAME as ::core::ffi::c_int;
    }
    if aez != 0 {
        return SUBMVREF_ABOVE_ZED as ::core::ffi::c_int;
    }
    if lez != 0 {
        return SUBMVREF_LEFT_ZED as ::core::ffi::c_int;
    }
    return SUBMVREF_NORMAL as ::core::ffi::c_int;
}}
static mut sub_mv_ref_prob: [vp8_prob; 3] = [
    180 as ::core::ffi::c_int as vp8_prob,
    162 as ::core::ffi::c_int as vp8_prob,
    25 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_prob2: [[vp8_prob; 3]; 5] = [
    [
        147 as ::core::ffi::c_int as vp8_prob,
        136 as ::core::ffi::c_int as vp8_prob,
        18 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        106 as ::core::ffi::c_int as vp8_prob,
        145 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        179 as ::core::ffi::c_int as vp8_prob,
        121 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        223 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        34 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        208 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplits: [vp8_mbsplit; 4] = [
    [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        11 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
        13 as ::core::ffi::c_int,
        14 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    ],
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_count: [::core::ffi::c_int; 4] = [
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_probs: [vp8_prob; 3] = [
    110 as ::core::ffi::c_int as vp8_prob,
    111 as ::core::ffi::c_int as vp8_prob,
    150 as ::core::ffi::c_int as vp8_prob,
];
#[unsafe(no_mangle)]
pub static mut vp8_bmode_tree: [vp8_tree_index; 18] = [
    -(B_DC_PRED as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    -(B_TM_PRED as ::core::ffi::c_int) as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    -(B_VE_PRED as ::core::ffi::c_int) as vp8_tree_index,
    6 as ::core::ffi::c_int as vp8_tree_index,
    8 as ::core::ffi::c_int as vp8_tree_index,
    12 as ::core::ffi::c_int as vp8_tree_index,
    -(B_HE_PRED as ::core::ffi::c_int) as vp8_tree_index,
    10 as ::core::ffi::c_int as vp8_tree_index,
    -(B_RD_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(B_VR_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(B_LD_PRED as ::core::ffi::c_int) as vp8_tree_index,
    14 as ::core::ffi::c_int as vp8_tree_index,
    -(B_VL_PRED as ::core::ffi::c_int) as vp8_tree_index,
    16 as ::core::ffi::c_int as vp8_tree_index,
    -(B_HD_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(B_HU_PRED as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_ymode_tree: [vp8_tree_index; 8] = [
    -(DC_PRED as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    6 as ::core::ffi::c_int as vp8_tree_index,
    -(V_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(H_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(TM_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(B_PRED as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_kf_ymode_tree: [vp8_tree_index; 8] = [
    -(B_PRED as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    6 as ::core::ffi::c_int as vp8_tree_index,
    -(DC_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(V_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(H_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(TM_PRED as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_uv_mode_tree: [vp8_tree_index; 6] = [
    -(DC_PRED as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    -(V_PRED as ::core::ffi::c_int) as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    -(H_PRED as ::core::ffi::c_int) as vp8_tree_index,
    -(TM_PRED as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_tree: [vp8_tree_index; 6] = [
    -(3 as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    -(2 as ::core::ffi::c_int) as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    -(0 as ::core::ffi::c_int) as vp8_tree_index,
    -(1 as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_mv_ref_tree: [vp8_tree_index; 8] = [
    -(ZEROMV as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    -(NEARESTMV as ::core::ffi::c_int) as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    -(NEARMV as ::core::ffi::c_int) as vp8_tree_index,
    6 as ::core::ffi::c_int as vp8_tree_index,
    -(NEWMV as ::core::ffi::c_int) as vp8_tree_index,
    -(SPLITMV as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_tree: [vp8_tree_index; 6] = [
    -(LEFT4X4 as ::core::ffi::c_int) as vp8_tree_index,
    2 as ::core::ffi::c_int as vp8_tree_index,
    -(ABOVE4X4 as ::core::ffi::c_int) as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    -(ZERO4X4 as ::core::ffi::c_int) as vp8_tree_index,
    -(NEW4X4 as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub static mut vp8_small_mvtree: [vp8_tree_index; 14] = [
    2 as ::core::ffi::c_int as vp8_tree_index,
    8 as ::core::ffi::c_int as vp8_tree_index,
    4 as ::core::ffi::c_int as vp8_tree_index,
    6 as ::core::ffi::c_int as vp8_tree_index,
    -(0 as ::core::ffi::c_int) as vp8_tree_index,
    -(1 as ::core::ffi::c_int) as vp8_tree_index,
    -(2 as ::core::ffi::c_int) as vp8_tree_index,
    -(3 as ::core::ffi::c_int) as vp8_tree_index,
    10 as ::core::ffi::c_int as vp8_tree_index,
    12 as ::core::ffi::c_int as vp8_tree_index,
    -(4 as ::core::ffi::c_int) as vp8_tree_index,
    -(5 as ::core::ffi::c_int) as vp8_tree_index,
    -(6 as ::core::ffi::c_int) as vp8_tree_index,
    -(7 as ::core::ffi::c_int) as vp8_tree_index,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_init_mbmode_probs(mut x: *mut VP8_COMMON) { unsafe {
    memcpy(
        &raw mut (*x).fc.ymode_prob as *mut vp8_prob as *mut ::core::ffi::c_void,
        &raw const vp8_ymode_prob as *const vp8_prob as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[vp8_prob; 4]>() as size_t,
    );
    memcpy(
        &raw mut (*x).fc.uv_mode_prob as *mut vp8_prob as *mut ::core::ffi::c_void,
        &raw const vp8_uv_mode_prob as *const vp8_prob as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
    );
    memcpy(
        &raw mut (*x).fc.sub_mv_ref_prob as *mut vp8_prob as *mut ::core::ffi::c_void,
        &raw const sub_mv_ref_prob as *const vp8_prob as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_default_bmode_probs(mut dest: *mut vp8_prob) { unsafe {
    memcpy(
        dest as *mut ::core::ffi::c_void,
        &raw const vp8_bmode_prob as *const vp8_prob as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[vp8_prob; 9]>() as size_t,
    );
}}
