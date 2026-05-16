use crate::vp8::common::entropymv::vp8_mv_update_probs;
use crate::vp8::decoder::dboolhuff::SafeBoolDecoder;
use crate::vp8::common::entropymode::{vp8_bmode_tree, vp8_ymode_tree, vp8_kf_ymode_tree, vp8_uv_mode_tree, vp8_small_mvtree, vp8_kf_bmode_prob, vp8_kf_uv_mode_prob, vp8_kf_ymode_prob};

unsafe extern "C" {
    static vp8_norm: [::core::ffi::c_uchar; 256];
    static vp8_mode_contexts: [[::core::ffi::c_int; 4]; 6];
    static vp8_mbsplit_offset: [[::core::ffi::c_uchar; 16]; 4];
}
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type size_t = __darwin_size_t;
pub type pthread_t = *mut ::core::ffi::c_void;
pub type mach_port_t = __darwin_mach_port_t;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_COMP {
    pub mb: MACROBLOCKD,
    pub dec_fb_ref: [*mut YV12_BUFFER_CONFIG; 4],
    pub common: VP8_COMMON,
    pub mbc: [vp8_reader; 9],
    pub oxcf: VP8D_CONFIG,
    pub fragments: FRAGMENT_DATA,
    pub b_multithreaded_rd: vpx_atomic_int,
    pub max_threads: ::core::ffi::c_int,
    pub current_mb_col_main: ::core::ffi::c_int,
    pub decoding_thread_count: ::core::ffi::c_uint,
    pub allocated_decoding_thread_count: ::core::ffi::c_int,
    pub mt_baseline_filter_level: [::core::ffi::c_int; 4],
    pub sync_range: ::core::ffi::c_int,
    pub mt_current_mb_col: *mut vpx_atomic_int,
    pub mt_yabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_uabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_vabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_yleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_uleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_vleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mb_row_di: *mut MB_ROW_DEC,
    pub de_thread_data: *mut DECODETHREAD_DATA,
    pub h_decoding_thread: *mut pthread_t,
    pub h_event_start_decoding: *mut semaphore_t,
    pub h_event_end_decoding: semaphore_t,
    pub ready_for_new_data: ::core::ffi::c_int,
    pub prob_intra: vp8_prob,
    pub prob_last: vp8_prob,
    pub prob_gf: vp8_prob,
    pub prob_skip_false: vp8_prob,
    pub ec_enabled: ::core::ffi::c_int,
    pub ec_active: ::core::ffi::c_int,
    pub decoded_key_frame: ::core::ffi::c_int,
    pub independent_partitions: ::core::ffi::c_int,
    pub frame_corrupt_residual: ::core::ffi::c_int,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub restart_threads: ::core::ffi::c_int,
}
pub type vpx_decrypt_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type vp8_prob = ::core::ffi::c_uchar;
pub type semaphore_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODETHREAD_DATA {
    pub ithread: ::core::ffi::c_int,
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_ROW_DEC {
    pub mbd: MACROBLOCKD,
}
pub type MACROBLOCKD = macroblockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub predictor: [::core::ffi::c_uchar; 384],
    pub qcoeff: [::core::ffi::c_short; 400],
    pub dqcoeff: [::core::ffi::c_short; 400],
    pub eobs: [::core::ffi::c_char; 25],
    pub dequant_y1: [::core::ffi::c_short; 16],
    pub dequant_y1_dc: [::core::ffi::c_short; 16],
    pub dequant_y2: [::core::ffi::c_short; 16],
    pub dequant_uv: [::core::ffi::c_short; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: ::core::ffi::c_int,
    pub pre: YV12_BUFFER_CONFIG,
    pub dst: YV12_BUFFER_CONFIG,
    pub mode_info_context: *mut MODE_INFO,
    pub mode_info_stride: ::core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub up_available: ::core::ffi::c_int,
    pub left_available: ::core::ffi::c_int,
    pub recon_above: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left_stride: [::core::ffi::c_int; 2],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: *mut ENTROPY_CONTEXT_PLANES,
    pub segmentation_enabled: ::core::ffi::c_uchar,
    pub update_mb_segmentation_map: ::core::ffi::c_uchar,
    pub update_mb_segmentation_data: ::core::ffi::c_uchar,
    pub mb_segment_abs_delta: ::core::ffi::c_uchar,
    pub mb_segment_tree_probs: [vp8_prob; 3],
    pub segment_feature_data: [[::core::ffi::c_schar; 4]; 2],
    pub mode_ref_lf_delta_enabled: ::core::ffi::c_uchar,
    pub mode_ref_lf_delta_update: ::core::ffi::c_uchar,
    pub last_ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub last_mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mb_to_left_edge: ::core::ffi::c_int,
    pub mb_to_right_edge: ::core::ffi::c_int,
    pub mb_to_top_edge: ::core::ffi::c_int,
    pub mb_to_bottom_edge: ::core::ffi::c_int,
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc: *mut ::core::ffi::c_void,
    pub corrupted: ::core::ffi::c_int,
    pub error_info: vpx_internal_error_info,
}
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
pub type vp8_subpix_fn_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
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
pub type BLOCKD = blockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut ::core::ffi::c_short,
    pub dqcoeff: *mut ::core::ffi::c_short,
    pub predictor: *mut ::core::ffi::c_uchar,
    pub dequant: *mut ::core::ffi::c_short,
    pub offset: ::core::ffi::c_int,
    pub eob: *mut ::core::ffi::c_char,
    pub bmi: b_mode_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_atomic_int {
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAGMENT_DATA {
    pub enabled: ::core::ffi::c_int,
    pub count: ::core::ffi::c_uint,
    pub ptrs: [*const ::core::ffi::c_uchar; 9],
    pub sizes: [::core::ffi::c_uint; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_CONFIG {
    pub Width: ::core::ffi::c_int,
    pub Height: ::core::ffi::c_int,
    pub Version: ::core::ffi::c_int,
    pub postprocess: ::core::ffi::c_int,
    pub max_threads: ::core::ffi::c_int,
    pub error_concealment: ::core::ffi::c_int,
}
pub type BOOL_DECODER = vp8_reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_reader {
    pub user_buffer_end: *const ::core::ffi::c_uchar,
    pub user_buffer: *const ::core::ffi::c_uchar,
    pub value: VP8_BD_VALUE,
    pub count: ::core::ffi::c_int,
    pub range: ::core::ffi::c_uint,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
}
pub type VP8_BD_VALUE = size_t;
pub type VP8_COMMON = VP8Common;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[::core::ffi::c_uchar; 1]; 64],
    pub blim: [[::core::ffi::c_uchar; 1]; 64],
    pub lim: [[::core::ffi::c_uchar; 1]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 1]; 4],
    pub lvl: [[[::core::ffi::c_uchar; 4]; 4]; 4],
    pub hev_thr_lut: [[::core::ffi::c_uchar; 64]; 2],
    pub mode_lf_lut: [::core::ffi::c_uchar; 10],
}
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type vp8_tree_index = ::core::ffi::c_schar;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MVPcount: C2RustUnnamed = 19;
pub const MVPbits: C2RustUnnamed = 9;
pub const MVPshort: C2RustUnnamed = 2;
pub const MVPsign: C2RustUnnamed = 1;
pub const mvpis_short: C2RustUnnamed = 0;
pub const mvnum_short: C2RustUnnamed = 8;
pub const mvlong_width: C2RustUnnamed = 10;
pub const MVfpvals: C2RustUnnamed = 511;
pub const mvfp_max: C2RustUnnamed = 255;
pub const MVvals: C2RustUnnamed = 2047;
pub const mv_max: C2RustUnnamed = 1023;
pub type MB_PREDICTION_MODE = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: MB_PREDICTION_MODE = 10;
pub const SPLITMV: MB_PREDICTION_MODE = 9;
pub const NEWMV: MB_PREDICTION_MODE = 8;
pub const ZEROMV: MB_PREDICTION_MODE = 7;
pub const NEARMV: MB_PREDICTION_MODE = 6;
pub const NEARESTMV: MB_PREDICTION_MODE = 5;
pub const B_PRED: MB_PREDICTION_MODE = 4;
pub const TM_PRED: MB_PREDICTION_MODE = 3;
pub const H_PRED: MB_PREDICTION_MODE = 2;
pub const V_PRED: MB_PREDICTION_MODE = 1;
pub const DC_PRED: MB_PREDICTION_MODE = 0;
pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub const CNT_NEAREST: C2RustUnnamed_0 = 1;
pub const CNT_NEAR: C2RustUnnamed_0 = 2;
pub const CNT_SPLITMV: C2RustUnnamed_0 = 3;
pub const CNT_INTRA: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;


#[inline]
unsafe extern "C" fn mv_bias(
    mut refmb_ref_frame_sign_bias: ::core::ffi::c_int,
    mut refframe: ::core::ffi::c_int,
    mut mvp: *mut int_mv,
    mut ref_frame_sign_bias: *const ::core::ffi::c_int,
) { unsafe {
    if refmb_ref_frame_sign_bias != *ref_frame_sign_bias.offset(refframe as isize) {
        (*mvp).as_mv.row = ((*mvp).as_mv.row as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
        (*mvp).as_mv.col = ((*mvp).as_mv.col as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    }
}}
pub const LEFT_TOP_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const RIGHT_BOTTOM_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vp8_clamp_mv2(mut mv: *mut int_mv, mut xd: *const MACROBLOCKD) { unsafe {
    if ((*mv).as_mv.col as ::core::ffi::c_int) < (*xd).mb_to_left_edge - LEFT_TOP_MARGIN {
        (*mv).as_mv.col = ((*xd).mb_to_left_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if (*mv).as_mv.col as ::core::ffi::c_int > (*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN {
        (*mv).as_mv.col = ((*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
    if ((*mv).as_mv.row as ::core::ffi::c_int) < (*xd).mb_to_top_edge - LEFT_TOP_MARGIN {
        (*mv).as_mv.row = ((*xd).mb_to_top_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if (*mv).as_mv.row as ::core::ffi::c_int > (*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN
    {
        (*mv).as_mv.row = ((*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
}}
#[inline]
fn vp8_check_mv_bounds(
    mv: &MV,
    mb_to_left_edge: ::core::ffi::c_int,
    mb_to_right_edge: ::core::ffi::c_int,
    mb_to_top_edge: ::core::ffi::c_int,
    mb_to_bottom_edge: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut need_to_clamp: ::core::ffi::c_uint = 0;
    need_to_clamp = ((mv.col as ::core::ffi::c_int) < mb_to_left_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= (mv.col as ::core::ffi::c_int > mb_to_right_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= ((mv.row as ::core::ffi::c_int) < mb_to_top_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= (mv.row as ::core::ffi::c_int > mb_to_bottom_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    return need_to_clamp;
}
#[inline]
unsafe extern "C" fn left_block_mode(
    mut cur_mb: *const MODE_INFO,
    mut b: ::core::ffi::c_int,
) -> B_PREDICTION_MODE { unsafe {
    if b & 3 as ::core::ffi::c_int == 0 {
        cur_mb = cur_mb.offset(-1);
        match (*cur_mb).mbmi.mode as ::core::ffi::c_int {
            4 => {
                return (*(&raw const (*cur_mb).bmi as *const b_mode_info)
                    .offset(b as isize)
                    .offset(3 as ::core::ffi::c_int as isize))
                .as_mode;
            }
            0 => return B_DC_PRED,
            1 => return B_VE_PRED,
            2 => return B_HE_PRED,
            3 => return B_TM_PRED,
            _ => return B_DC_PRED,
        }
    }
    return (*(&raw const (*cur_mb).bmi as *const b_mode_info)
        .offset(b as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)))
    .as_mode;
}}
#[inline]
unsafe extern "C" fn above_block_mode(
    mut cur_mb: *const MODE_INFO,
    mut b: ::core::ffi::c_int,
    mut mi_stride: ::core::ffi::c_int,
) -> B_PREDICTION_MODE { unsafe {
    if b >> 2 as ::core::ffi::c_int == 0 {
        cur_mb = cur_mb.offset(-(mi_stride as isize));
        match (*cur_mb).mbmi.mode as ::core::ffi::c_int {
            4 => {
                return (*(&raw const (*cur_mb).bmi as *const b_mode_info)
                    .offset(b as isize)
                    .offset(12 as ::core::ffi::c_int as isize))
                .as_mode;
            }
            0 => return B_DC_PRED,
            1 => return B_VE_PRED,
            2 => return B_HE_PRED,
            3 => return B_TM_PRED,
            _ => return B_DC_PRED,
        }
    }
    return (*(&raw const (*cur_mb).bmi as *const b_mode_info)
        .offset(b as isize)
        .offset(-(4 as ::core::ffi::c_int as isize)))
    .as_mode;
}}
fn safe_treed_read(
    r: &mut SafeBoolDecoder,
    t: &[vp8_tree_index],
    p: &[vp8_prob],
) -> i32 {
    let mut i: usize = 0;
    loop {
        let prob = p[i >> 1];
        let bit = r.read_bool(prob as i32);
        let next_node = t[i + (bit as usize)];
        if next_node <= 0 {
            return -next_node as i32;
        }
        i = next_node as usize;
    }
}

fn read_bmode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> B_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_bmode_tree, p) as B_PREDICTION_MODE
}

fn read_ymode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_ymode_tree, p) as MB_PREDICTION_MODE
}

fn read_kf_ymode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_kf_ymode_tree, p) as MB_PREDICTION_MODE
}

fn read_uv_mode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_uv_mode_tree, p) as MB_PREDICTION_MODE
}

unsafe extern "C" fn read_kf_modes(
    mut pbi: *mut VP8D_COMP,
    mut mi: *mut MODE_INFO,
    safe_decoder: &mut SafeBoolDecoder,
) { unsafe {
    let mis: ::core::ffi::c_int = (*pbi).common.mode_info_stride;

    (*mi).mbmi.ref_frame = INTRA_FRAME as ::core::ffi::c_int as uint8_t;
    (*mi).mbmi.mode = read_kf_ymode(safe_decoder, &vp8_kf_ymode_prob) as uint8_t;
    if (*mi).mbmi.mode as ::core::ffi::c_int == B_PRED as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        (*mi).mbmi.is_4x4 = 1 as uint8_t;
        loop {
            let A: B_PREDICTION_MODE = above_block_mode(mi, i, mis) as B_PREDICTION_MODE;
            let L: B_PREDICTION_MODE = left_block_mode(mi, i) as B_PREDICTION_MODE;
            (*mi).bmi[i as usize].as_mode = read_bmode(
                safe_decoder,
                &vp8_kf_bmode_prob[A as usize][L as usize],
            );
            i += 1;
            if !(i < 16 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    (*mi).mbmi.uv_mode =
        read_uv_mode(safe_decoder, &vp8_kf_uv_mode_prob) as uint8_t;
}}
fn read_mvcomponent(
    r: &mut SafeBoolDecoder,
    mvc: &MV_CONTEXT,
) -> i32 {
    let p = &mvc.prob;
    let mut x: i32 = 0;
    if r.read_bool(p[mvpis_short as usize] as i32) != 0 {
        for i in 0..3usize {
            x += r.read_bool(p[MVPbits as usize + i] as i32) << i;
        }
        for i in (4..mvlong_width as usize).rev() {
            x += r.read_bool(p[MVPbits as usize + i] as i32) << i;
        }
        if x & 0xfff0 == 0 || r.read_bool(p[MVPbits as usize + 3] as i32) != 0 {
            x += 8;
        }
    } else {
        x = safe_treed_read(r, &vp8_small_mvtree, &p[MVPshort as usize..]);
    }
    if x != 0 && r.read_bool(p[MVPsign as usize] as i32) != 0 {
        x = -x;
    }
    x
}
fn read_mv(r: &mut SafeBoolDecoder, mv: &mut MV, mvc: &[MV_CONTEXT; 2]) {
    mv.row = (read_mvcomponent(r, &mvc[0]) * 2) as i16;
    mv.col = (read_mvcomponent(r, &mvc[1]) * 2) as i16;
}
fn read_mvcontexts(bc: &mut SafeBoolDecoder, mvc: &mut [MV_CONTEXT; 2]) {
    for i in 0..2 {
        let up_probs = &vp8_mv_update_probs[i].prob;
        let p_probs = &mut mvc[i].prob;
        for j in 0..MVPcount as usize {
            let prob_to_decode = up_probs[j];
            if bc.read_bool(prob_to_decode as i32) != 0 {
                let x = bc.read_literal(7) as vp8_prob;
                p_probs[j] = if x != 0 { x << 1 } else { 1 };
            }
        }
    }
}

static mbsplit_fill_count: [::core::ffi::c_uchar; 4] = [
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mbsplit_fill_offset: [[::core::ffi::c_uchar; 16]; 4] = [
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
];
unsafe extern "C" fn mb_mode_mv_init(mut pbi: *mut VP8D_COMP, safe_decoder: &mut SafeBoolDecoder) { unsafe {
    let mvc = &mut (*pbi).common.fc.mvc;

    (*pbi).common.mb_no_coeff_skip = safe_decoder.read_bool(vp8_prob_half as i32);
    (*pbi).prob_skip_false = 0 as vp8_prob;
    if (*pbi).common.mb_no_coeff_skip != 0 {
        (*pbi).prob_skip_false = safe_decoder.read_literal(8) as vp8_prob;
    }
    if (*pbi).common.frame_type as ::core::ffi::c_uint
        != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*pbi).prob_intra = safe_decoder.read_literal(8) as vp8_prob;
        (*pbi).prob_last = safe_decoder.read_literal(8) as vp8_prob;
        (*pbi).prob_gf = safe_decoder.read_literal(8) as vp8_prob;
        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
            for i in 0..4 {
                (*pbi).common.fc.ymode_prob[i] = safe_decoder.read_literal(8) as vp8_prob;
            }
        }
        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
            for i in 0..3 {
                (*pbi).common.fc.uv_mode_prob[i] = safe_decoder.read_literal(8) as vp8_prob;
            }
        }
        read_mvcontexts(safe_decoder, mvc);
    }
}}
#[unsafe(no_mangle)]
pub static vp8_sub_mv_ref_prob3: [[vp8_prob; 3]; 8] = [
    [
        147 as ::core::ffi::c_int as vp8_prob,
        136 as ::core::ffi::c_int as vp8_prob,
        18 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        223 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        34 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        106 as ::core::ffi::c_int as vp8_prob,
        145 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        208 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
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
        179 as ::core::ffi::c_int as vp8_prob,
        121 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        208 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
];
fn get_sub_mv_ref_prob(left: uint32_t, above: uint32_t) -> &'static [vp8_prob; 3] {
    let lez = (left == 0) as usize;
    let aez = (above == 0) as usize;
    let lea = (left == above) as usize;
    &vp8_sub_mv_ref_prob3[(aez << 2) | (lez << 1) | lea]
}
unsafe extern "C" fn decode_split_mv(
    safe_decoder: &mut SafeBoolDecoder,
    mut mi: *mut MODE_INFO,
    mut left_mb: *const MODE_INFO,
    mut above_mb: *const MODE_INFO,
    mut mbmi: *mut MB_MODE_INFO,
    mut best_mv: int_mv,
    mvc: &[MV_CONTEXT; 2],
    mut mb_to_left_edge: ::core::ffi::c_int,
    mut mb_to_right_edge: ::core::ffi::c_int,
    mut mb_to_top_edge: ::core::ffi::c_int,
    mut mb_to_bottom_edge: ::core::ffi::c_int,
) { unsafe {
    let mut s: ::core::ffi::c_int = 0;
    let mut num_p: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    s = 3 as ::core::ffi::c_int;
    num_p = 16 as ::core::ffi::c_int;
    if safe_decoder.read_bool(110) != 0 {
        s = 2 as ::core::ffi::c_int;
        num_p = 4 as ::core::ffi::c_int;
        if safe_decoder.read_bool(111) != 0 {
            s = safe_decoder.read_bool(150);
            num_p = 2 as ::core::ffi::c_int;
        }
    }
    loop {
        let mut leftmv: int_mv = int_mv { as_int: 0 };
        let mut abovemv: int_mv = int_mv { as_int: 0 };
        let mut blockmv: int_mv = int_mv { as_int: 0 };
        let mut k: ::core::ffi::c_int = 0;
        k = vp8_mbsplit_offset[s as usize][j as usize] as ::core::ffi::c_int;
        if k & 3 as ::core::ffi::c_int == 0 {
            if (*left_mb).mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int {
                leftmv.as_int = (*left_mb).mbmi.mv.as_int;
            } else {
                leftmv.as_int = (*(&raw const (*left_mb).bmi as *const b_mode_info)
                    .offset(k as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)))
                .mv
                .as_int;
            }
        } else {
            leftmv.as_int = (*(&raw mut (*mi).bmi as *mut b_mode_info)
                .offset(k as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)))
            .mv
            .as_int;
        }
        if k >> 2 as ::core::ffi::c_int == 0 {
            if (*above_mb).mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int {
                abovemv.as_int = (*above_mb).mbmi.mv.as_int;
            } else {
                abovemv.as_int = (*(&raw const (*above_mb).bmi as *const b_mode_info)
                    .offset(k as isize)
                    .offset(16 as ::core::ffi::c_int as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)))
                .mv
                .as_int;
            }
        } else {
            abovemv.as_int = (*(&raw mut (*mi).bmi as *mut b_mode_info)
                .offset(k as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)))
            .mv
            .as_int;
        }
        let prob = get_sub_mv_ref_prob(leftmv.as_int, abovemv.as_int);
        if safe_decoder.read_bool(prob[0] as i32) != 0 {
            if safe_decoder.read_bool(prob[1] as i32) != 0 {
                blockmv.as_int = 0 as uint32_t;
                if safe_decoder.read_bool(prob[2] as i32) != 0 {
                    blockmv.as_mv.row = (read_mvcomponent(
                        safe_decoder,
                        &mvc[0],
                    ) * 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    blockmv.as_mv.row = (blockmv.as_mv.row as ::core::ffi::c_int
                        + best_mv.as_mv.row as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    blockmv.as_mv.col = (read_mvcomponent(
                        safe_decoder,
                        &mvc[1],
                    ) * 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    blockmv.as_mv.col = (blockmv.as_mv.col as ::core::ffi::c_int
                        + best_mv.as_mv.col as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                }
            } else {
                blockmv.as_int = abovemv.as_int;
            }
        } else {
            blockmv.as_int = leftmv.as_int;
        }
        (*mbmi).need_to_clamp_mvs = ((*mbmi).need_to_clamp_mvs as ::core::ffi::c_uint
            | vp8_check_mv_bounds(
                &blockmv.as_mv,
                mb_to_left_edge,
                mb_to_right_edge,
                mb_to_top_edge,
                mb_to_bottom_edge,
            )) as uint8_t;
        let fill_count = mbsplit_fill_count[s as usize] as usize;
        let offset_start = (j as usize) * fill_count;
        let fill_offsets = &mbsplit_fill_offset[s as usize][offset_start..offset_start + fill_count];
        for &idx in fill_offsets {
            (*mi).bmi[idx as usize].mv.as_int = blockmv.as_int;
        }
        j += 1;
        if !(j < num_p) {
            break;
        }
    }
    (*mbmi).partitioning = s as uint8_t;
}}
unsafe extern "C" fn read_mb_modes_mv(
    mut pbi: *mut VP8D_COMP,
    mut mi: *mut MODE_INFO,
    mut mbmi: *mut MB_MODE_INFO,
    safe_decoder: &mut SafeBoolDecoder,
) { unsafe {
    (*mbmi).ref_frame = safe_decoder.read_bool(
        (*pbi).prob_intra as i32,
    ) as MV_REFERENCE_FRAME as uint8_t;
    if (*mbmi).ref_frame != 0 {
        let mut cnt: [::core::ffi::c_int; 4] = [0; 4];
        let mut cntx: *mut ::core::ffi::c_int = &raw mut cnt as *mut ::core::ffi::c_int;
        let mut near_mvs: [int_mv; 4] = [int_mv { as_int: 0 }; 4];
        let mut nmv: *mut int_mv = &raw mut near_mvs as *mut int_mv;
        let mis: ::core::ffi::c_int = (*pbi).mb.mode_info_stride;
        let mut above: *const MODE_INFO = mi.offset(-(mis as isize));
        let mut left: *const MODE_INFO = mi.offset(-(1 as ::core::ffi::c_int as isize));
        let mut aboveleft: *const MODE_INFO = above.offset(-(1 as ::core::ffi::c_int as isize));
        let mut ref_frame_sign_bias: *mut ::core::ffi::c_int =
            &raw mut (*pbi).common.ref_frame_sign_bias as *mut ::core::ffi::c_int;
        (*mbmi).need_to_clamp_mvs = 0 as uint8_t;
        if safe_decoder.read_bool(
            (*pbi).prob_last as i32,
        ) != 0
        {
            (*mbmi).ref_frame = (2 as ::core::ffi::c_int
                + safe_decoder.read_bool(
                    (*pbi).prob_gf as i32,
                )) as MV_REFERENCE_FRAME as uint8_t;
        }
        let ref mut fresh0 = (*nmv.offset(2 as ::core::ffi::c_int as isize)).as_int;
        *fresh0 = 0 as uint32_t;
        let ref mut fresh1 = (*nmv.offset(1 as ::core::ffi::c_int as isize)).as_int;
        *fresh1 = *fresh0;
        (*nmv.offset(0 as ::core::ffi::c_int as isize)).as_int = *fresh1;
        cnt[3 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        cnt[2 as ::core::ffi::c_int as usize] = cnt[3 as ::core::ffi::c_int as usize];
        cnt[1 as ::core::ffi::c_int as usize] = cnt[2 as ::core::ffi::c_int as usize];
        cnt[0 as ::core::ffi::c_int as usize] = cnt[1 as ::core::ffi::c_int as usize];
        if (*above).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if (*above).mbmi.mv.as_int != 0 {
                nmv = nmv.offset(1);
                (*nmv).as_int = (*above).mbmi.mv.as_int;
                mv_bias(
                    *ref_frame_sign_bias.offset((*above).mbmi.ref_frame as isize),
                    (*mbmi).ref_frame as ::core::ffi::c_int,
                    nmv,
                    ref_frame_sign_bias,
                );
                cntx = cntx.offset(1);
            }
            *cntx += 2 as ::core::ffi::c_int;
        }
        if (*left).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if (*left).mbmi.mv.as_int != 0 {
                let mut this_mv: int_mv = int_mv { as_int: 0 };
                this_mv.as_int = (*left).mbmi.mv.as_int;
                mv_bias(
                    *ref_frame_sign_bias.offset((*left).mbmi.ref_frame as isize),
                    (*mbmi).ref_frame as ::core::ffi::c_int,
                    &raw mut this_mv,
                    ref_frame_sign_bias,
                );
                if this_mv.as_int != (*nmv).as_int {
                    nmv = nmv.offset(1);
                    (*nmv).as_int = this_mv.as_int;
                    cntx = cntx.offset(1);
                }
                *cntx += 2 as ::core::ffi::c_int;
            } else {
                cnt[CNT_INTRA as ::core::ffi::c_int as usize] += 2 as ::core::ffi::c_int;
            }
        }
        if (*aboveleft).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if (*aboveleft).mbmi.mv.as_int != 0 {
                let mut this_mv_0: int_mv = int_mv { as_int: 0 };
                this_mv_0.as_int = (*aboveleft).mbmi.mv.as_int;
                mv_bias(
                    *ref_frame_sign_bias.offset((*aboveleft).mbmi.ref_frame as isize),
                    (*mbmi).ref_frame as ::core::ffi::c_int,
                    &raw mut this_mv_0,
                    ref_frame_sign_bias,
                );
                if this_mv_0.as_int != (*nmv).as_int {
                    nmv = nmv.offset(1);
                    (*nmv).as_int = this_mv_0.as_int;
                    cntx = cntx.offset(1);
                }
                *cntx += 1 as ::core::ffi::c_int;
            } else {
                cnt[CNT_INTRA as ::core::ffi::c_int as usize] += 1 as ::core::ffi::c_int;
            }
        }
        if safe_decoder.read_bool(
            vp8_mode_contexts[cnt[CNT_INTRA as ::core::ffi::c_int as usize] as usize]
                [0 as ::core::ffi::c_int as usize] as i32,
        ) != 0
        {
            cnt[CNT_NEAREST as ::core::ffi::c_int as usize] +=
                (cnt[CNT_SPLITMV as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    & ((*nmv).as_int == near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int)
                        as ::core::ffi::c_int;
            if cnt[CNT_NEAR as ::core::ffi::c_int as usize]
                > cnt[CNT_NEAREST as ::core::ffi::c_int as usize]
            {
                let mut tmp: ::core::ffi::c_int = 0;
                tmp = cnt[CNT_NEAREST as ::core::ffi::c_int as usize];
                cnt[CNT_NEAREST as ::core::ffi::c_int as usize] =
                    cnt[CNT_NEAR as ::core::ffi::c_int as usize];
                cnt[CNT_NEAR as ::core::ffi::c_int as usize] = tmp;
                tmp = near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int
                    as ::core::ffi::c_int;
                near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int =
                    near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int;
                near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int = tmp as uint32_t;
            }
            if safe_decoder.read_bool(
                vp8_mode_contexts[cnt[CNT_NEAREST as ::core::ffi::c_int as usize] as usize]
                    [1 as ::core::ffi::c_int as usize] as i32,
            ) != 0
            {
                if safe_decoder.read_bool(
                    vp8_mode_contexts[cnt[CNT_NEAR as ::core::ffi::c_int as usize] as usize]
                        [2 as ::core::ffi::c_int as usize] as i32,
                ) != 0
                {
                    let mut mb_to_top_edge: ::core::ffi::c_int = 0;
                    let mut mb_to_bottom_edge: ::core::ffi::c_int = 0;
                    let mut mb_to_left_edge: ::core::ffi::c_int = 0;
                    let mut mb_to_right_edge: ::core::ffi::c_int = 0;
                    let mvc = &(*pbi).common.fc.mvc;
                    let mut near_index: ::core::ffi::c_int = 0;
                    mb_to_top_edge = (*pbi).mb.mb_to_top_edge;
                    mb_to_bottom_edge = (*pbi).mb.mb_to_bottom_edge;
                    mb_to_top_edge -= LEFT_TOP_MARGIN;
                    mb_to_bottom_edge += RIGHT_BOTTOM_MARGIN;
                    mb_to_right_edge = (*pbi).mb.mb_to_right_edge;
                    mb_to_right_edge += RIGHT_BOTTOM_MARGIN;
                    mb_to_left_edge = (*pbi).mb.mb_to_left_edge;
                    mb_to_left_edge -= LEFT_TOP_MARGIN;
                    near_index = CNT_INTRA as ::core::ffi::c_int
                        + (cnt[CNT_NEAREST as ::core::ffi::c_int as usize]
                            >= cnt[CNT_INTRA as ::core::ffi::c_int as usize])
                            as ::core::ffi::c_int;
                    vp8_clamp_mv2(
                        (&raw mut near_mvs as *mut int_mv).offset(near_index as isize)
                            as *mut int_mv,
                        &raw mut (*pbi).mb,
                    );
                    cnt[CNT_SPLITMV as ::core::ffi::c_int as usize] = (((*above).mbmi.mode
                        as ::core::ffi::c_int
                        == SPLITMV as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        + ((*left).mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int)
                            as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int
                        + ((*aboveleft).mbmi.mode as ::core::ffi::c_int
                            == SPLITMV as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    if safe_decoder.read_bool(
                        vp8_mode_contexts[cnt[CNT_SPLITMV as ::core::ffi::c_int as usize] as usize]
                            [3 as ::core::ffi::c_int as usize] as i32,
                    ) != 0
                    {
                        decode_split_mv(
                            safe_decoder,
                            mi,
                            left,
                            above,
                            mbmi,
                            near_mvs[near_index as usize],
                            mvc,
                            mb_to_left_edge,
                            mb_to_right_edge,
                            mb_to_top_edge,
                            mb_to_bottom_edge,
                        );
                        (*mbmi).mv.as_int = (*mi).bmi[15 as ::core::ffi::c_int as usize].mv.as_int;
                        (*mbmi).mode = SPLITMV as ::core::ffi::c_int as uint8_t;
                        (*mbmi).is_4x4 = 1 as uint8_t;
                    } else {
                        let mbmi_mv: *mut int_mv = &raw mut (*mbmi).mv;
                        read_mv(safe_decoder, &mut (*mbmi_mv).as_mv, mvc);
                        (*mbmi_mv).as_mv.row = ((*mbmi_mv).as_mv.row as ::core::ffi::c_int
                            + near_mvs[near_index as usize].as_mv.row as ::core::ffi::c_int)
                            as ::core::ffi::c_short;
                        (*mbmi_mv).as_mv.col = ((*mbmi_mv).as_mv.col as ::core::ffi::c_int
                            + near_mvs[near_index as usize].as_mv.col as ::core::ffi::c_int)
                            as ::core::ffi::c_short;
                        (*mbmi).need_to_clamp_mvs = vp8_check_mv_bounds(
                            &(*mbmi_mv).as_mv,
                            mb_to_left_edge,
                            mb_to_right_edge,
                            mb_to_top_edge,
                            mb_to_bottom_edge,
                        ) as uint8_t;
                        (*mbmi).mode = NEWMV as ::core::ffi::c_int as uint8_t;
                    }
                } else {
                    (*mbmi).mode = NEARMV as ::core::ffi::c_int as uint8_t;
                    (*mbmi).mv.as_int = near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int;
                    vp8_clamp_mv2(&raw mut (*mbmi).mv, &raw mut (*pbi).mb);
                }
            } else {
                (*mbmi).mode = NEARESTMV as ::core::ffi::c_int as uint8_t;
                (*mbmi).mv.as_int = near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int;
                vp8_clamp_mv2(&raw mut (*mbmi).mv, &raw mut (*pbi).mb);
            }
        } else {
            (*mbmi).mode = ZEROMV as ::core::ffi::c_int as uint8_t;
            (*mbmi).mv.as_int = 0 as uint32_t;
        }
    } else {
        (*mbmi).mv.as_int = 0 as uint32_t;

        (*mbmi).mode =
            read_ymode(safe_decoder, &(*pbi).common.fc.ymode_prob) as uint8_t;
        if (*mbmi).mode as ::core::ffi::c_int == B_PRED as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            (*mbmi).is_4x4 = 1 as uint8_t;
            loop {
                (*mi).bmi[j as usize].as_mode =
                    read_bmode(safe_decoder, &(*pbi).common.fc.bmode_prob);
                j += 1;
                if !(j < 16 as ::core::ffi::c_int) {
                    break;
                }
            }
        }
        (*mbmi).uv_mode =
            read_uv_mode(safe_decoder, &(*pbi).common.fc.uv_mode_prob) as uint8_t;
    };
}}
unsafe extern "C" fn read_mb_features(
    safe_decoder: &mut SafeBoolDecoder,
    mut mi: *mut MB_MODE_INFO,
    mut x: *mut MACROBLOCKD,
) { unsafe {
    if (*x).segmentation_enabled as ::core::ffi::c_int != 0
        && (*x).update_mb_segmentation_map as ::core::ffi::c_int != 0
    {
        if safe_decoder.read_bool((*x).mb_segment_tree_probs[0] as i32) != 0 {
            (*mi).segment_id = (2 + safe_decoder.read_bool((*x).mb_segment_tree_probs[2] as i32)) as uint8_t;
        } else {
            (*mi).segment_id = safe_decoder.read_bool((*x).mb_segment_tree_probs[1] as i32) as uint8_t;
        }
    }
}}
unsafe extern "C" fn decode_mb_mode_mvs(
    mut pbi: *mut VP8D_COMP,
    mut mi: *mut MODE_INFO,
    safe_decoder: &mut SafeBoolDecoder,
) { unsafe {
    if (*pbi).mb.update_mb_segmentation_map != 0 {
        read_mb_features(
            safe_decoder,
            &raw mut (*mi).mbmi,
            &raw mut (*pbi).mb,
        );
    } else if (*pbi).common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*mi).mbmi.segment_id = 0 as uint8_t;
    }
    if (*pbi).common.mb_no_coeff_skip != 0 {
        (*mi).mbmi.mb_skip_coeff = safe_decoder.read_bool((*pbi).prob_skip_false as i32) as uint8_t;
    } else {
        (*mi).mbmi.mb_skip_coeff = 0 as uint8_t;
    }
    (*mi).mbmi.is_4x4 = 0 as uint8_t;
    if (*pbi).common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        read_kf_modes(pbi, mi, safe_decoder);
    } else {
        read_mb_modes_mv(pbi, mi, &raw mut (*mi).mbmi, safe_decoder);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_decode_mode_mvs(mut pbi: *mut VP8D_COMP) { unsafe {
    let mut mi: *mut MODE_INFO = (*pbi).common.mi;
    let mut mb_row: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut mb_to_right_edge_start: ::core::ffi::c_int = 0;

    let bc: *mut vp8_reader = (&raw mut (*pbi).mbc as *mut vp8_reader)
        .offset(8 as ::core::ffi::c_int as isize) as *mut vp8_reader;
    let len = (*bc).user_buffer_end.offset_from((*bc).user_buffer) as usize;
    let slice = if len == 0 {
        &[]
    } else {
        core::slice::from_raw_parts((*bc).user_buffer, len)
    };
    let mut safe_decoder = SafeBoolDecoder {
        buffer: slice,
        offset: 0,
        value: (*bc).value,
        count: (*bc).count,
        range: (*bc).range,
        decrypt_cb: (*bc).decrypt_cb,
        decrypt_state: (*bc).decrypt_state,
    };

    mb_mode_mv_init(pbi, &mut safe_decoder);
    (*pbi).mb.mb_to_top_edge = 0 as ::core::ffi::c_int;
    (*pbi).mb.mb_to_bottom_edge = (((*pbi).common.mb_rows - 1 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int;
    mb_to_right_edge_start = (((*pbi).common.mb_cols - 1 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int;
    loop {
        mb_row += 1;
        if !(mb_row < (*pbi).common.mb_rows) {
            break;
        }
        let mut mb_col: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        (*pbi).mb.mb_to_left_edge = 0 as ::core::ffi::c_int;
        (*pbi).mb.mb_to_right_edge = mb_to_right_edge_start;
        loop {
            mb_col += 1;
            if !(mb_col < (*pbi).common.mb_cols) {
                break;
            }
            decode_mb_mode_mvs(pbi, mi, &mut safe_decoder);
            (*pbi).mb.mb_to_left_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            (*pbi).mb.mb_to_right_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            mi = mi.offset(1);
        }
        (*pbi).mb.mb_to_top_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
        (*pbi).mb.mb_to_bottom_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
        mi = mi.offset(1);
    }

    (*bc).user_buffer = (*bc).user_buffer.add(safe_decoder.offset);
    (*bc).value = safe_decoder.value;
    (*bc).count = safe_decoder.count;
    (*bc).range = safe_decoder.range;
}}
