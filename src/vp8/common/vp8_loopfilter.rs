unsafe extern "C" {
    fn vp8_loop_filter_bh_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bv_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbh_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbv_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bhs_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_bvs_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_simple_horizontal_edge_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_simple_vertical_edge_c(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
pub type BLOCKD = blockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
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
pub type VP8_COMMON = VP8Common;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const ZEROMV: C2RustUnnamed = 7;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub type MACROBLOCKD = macroblockd;
pub const MAX_REF_FRAMES: C2RustUnnamed_1 = 4;
pub const INTRA_FRAME: C2RustUnnamed_1 = 0;
pub const MB_LVL_ALT_LF: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MB_LVL_MAX: C2RustUnnamed_0 = 2;
pub const MB_LVL_ALT_Q: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const MAX_LOOP_FILTER: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const PARTIAL_FRAME_FRACTION: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIMD_WIDTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_MB_SEGMENTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEGMENT_ABSDATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn lf_init_lut(mut lfi: *mut loop_filter_info_n) {
    unsafe {
        let mut filt_lvl: ::core::ffi::c_int = 0;
        filt_lvl = 0 as ::core::ffi::c_int;
        while filt_lvl <= MAX_LOOP_FILTER {
            if filt_lvl >= 40 as ::core::ffi::c_int {
                (*lfi).hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    2 as ::core::ffi::c_uchar;
                (*lfi).hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    3 as ::core::ffi::c_uchar;
            } else if filt_lvl >= 20 as ::core::ffi::c_int {
                (*lfi).hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    1 as ::core::ffi::c_uchar;
                (*lfi).hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    2 as ::core::ffi::c_uchar;
            } else if filt_lvl >= 15 as ::core::ffi::c_int {
                (*lfi).hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    1 as ::core::ffi::c_uchar;
                (*lfi).hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    1 as ::core::ffi::c_uchar;
            } else {
                (*lfi).hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    0 as ::core::ffi::c_uchar;
                (*lfi).hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                    0 as ::core::ffi::c_uchar;
            }
            filt_lvl += 1;
        }
        (*lfi).mode_lf_lut[DC_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[V_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[H_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[TM_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[B_PRED as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[ZEROMV as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[NEARESTMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[NEARMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[NEWMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
        (*lfi).mode_lf_lut[SPLITMV as ::core::ffi::c_int as usize] = 3 as ::core::ffi::c_uchar;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_update_sharpness(
    mut lfi: *mut loop_filter_info_n,
    mut sharpness_lvl: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i <= MAX_LOOP_FILTER {
            let mut filt_lvl: ::core::ffi::c_int = i;
            let mut block_inside_limit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            block_inside_limit =
                filt_lvl >> (sharpness_lvl > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            block_inside_limit = block_inside_limit
                >> (sharpness_lvl > 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if sharpness_lvl > 0 as ::core::ffi::c_int {
                if block_inside_limit > 9 as ::core::ffi::c_int - sharpness_lvl {
                    block_inside_limit = 9 as ::core::ffi::c_int - sharpness_lvl;
                }
            }
            if block_inside_limit < 1 as ::core::ffi::c_int {
                block_inside_limit = 1 as ::core::ffi::c_int;
            }
            memset(
                &raw mut *(&raw mut (*lfi).lim as *mut [::core::ffi::c_uchar; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                block_inside_limit,
                SIMD_WIDTH as size_t,
            );
            memset(
                &raw mut *(&raw mut (*lfi).blim as *mut [::core::ffi::c_uchar; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                2 as ::core::ffi::c_int * filt_lvl + block_inside_limit,
                SIMD_WIDTH as size_t,
            );
            memset(
                &raw mut *(&raw mut (*lfi).mblim as *mut [::core::ffi::c_uchar; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                2 as ::core::ffi::c_int * (filt_lvl + 2 as ::core::ffi::c_int) + block_inside_limit,
                SIMD_WIDTH as size_t,
            );
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_init(mut cm: *mut VP8_COMMON) {
    unsafe {
        let mut lfi: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        let mut i: ::core::ffi::c_int = 0;
        vp8_loop_filter_update_sharpness(lfi, (*cm).sharpness_level);
        (*cm).last_sharpness_level = (*cm).sharpness_level;
        lf_init_lut(lfi);
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            memset(
                &raw mut *(&raw mut (*lfi).hev_thr as *mut [::core::ffi::c_uchar; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                i,
                SIMD_WIDTH as size_t,
            );
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_frame_init(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: ::core::ffi::c_int,
) {
    unsafe {
        let mut seg: ::core::ffi::c_int = 0;
        let mut ref_0: ::core::ffi::c_int = 0;
        let mut mode: ::core::ffi::c_int = 0;
        let mut lfi: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        if (*cm).last_sharpness_level != (*cm).sharpness_level {
            vp8_loop_filter_update_sharpness(lfi, (*cm).sharpness_level);
            (*cm).last_sharpness_level = (*cm).sharpness_level;
        }
        seg = 0 as ::core::ffi::c_int;
        while seg < MAX_MB_SEGMENTS {
            let mut lvl_seg: ::core::ffi::c_int = default_filt_lvl;
            let mut lvl_ref: ::core::ffi::c_int = 0;
            let mut lvl_mode: ::core::ffi::c_int = 0;
            if (*mbd).segmentation_enabled != 0 {
                if (*mbd).mb_segment_abs_delta as ::core::ffi::c_int == SEGMENT_ABSDATA {
                    lvl_seg = (*mbd).segment_feature_data
                        [MB_LVL_ALT_LF as ::core::ffi::c_int as usize][seg as usize]
                        as ::core::ffi::c_int;
                } else {
                    lvl_seg += (*mbd).segment_feature_data
                        [MB_LVL_ALT_LF as ::core::ffi::c_int as usize][seg as usize]
                        as ::core::ffi::c_int;
                }
                lvl_seg = if lvl_seg > 0 as ::core::ffi::c_int {
                    if lvl_seg > 63 as ::core::ffi::c_int {
                        63 as ::core::ffi::c_int
                    } else {
                        lvl_seg
                    }
                } else {
                    0 as ::core::ffi::c_int
                };
            }
            if (*mbd).mode_ref_lf_delta_enabled == 0 {
                memset(
                    &raw mut *(&raw mut *(&raw mut (*lfi).lvl
                        as *mut [[::core::ffi::c_uchar; 4]; 4])
                        .offset(seg as isize)
                        as *mut [::core::ffi::c_uchar; 4])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_void,
                    lvl_seg,
                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as size_t,
                );
            } else {
                ref_0 = INTRA_FRAME as ::core::ffi::c_int;
                lvl_ref = lvl_seg + (*mbd).ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
                mode = 0 as ::core::ffi::c_int;
                lvl_mode = lvl_ref + (*mbd).mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
                lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                    if lvl_mode > 63 as ::core::ffi::c_int {
                        63 as ::core::ffi::c_int
                    } else {
                        lvl_mode
                    }
                } else {
                    0 as ::core::ffi::c_int
                };
                (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] =
                    lvl_mode as ::core::ffi::c_uchar;
                mode = 1 as ::core::ffi::c_int;
                lvl_mode = if lvl_ref > 0 as ::core::ffi::c_int {
                    if lvl_ref > 63 as ::core::ffi::c_int {
                        63 as ::core::ffi::c_int
                    } else {
                        lvl_ref
                    }
                } else {
                    0 as ::core::ffi::c_int
                };
                (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] =
                    lvl_mode as ::core::ffi::c_uchar;
                ref_0 = 1 as ::core::ffi::c_int;
                while ref_0 < MAX_REF_FRAMES as ::core::ffi::c_int {
                    lvl_ref = lvl_seg + (*mbd).ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
                    mode = 1 as ::core::ffi::c_int;
                    while mode < 4 as ::core::ffi::c_int {
                        lvl_mode =
                            lvl_ref + (*mbd).mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
                        lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                            if lvl_mode > 63 as ::core::ffi::c_int {
                                63 as ::core::ffi::c_int
                            } else {
                                lvl_mode
                            }
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] =
                            lvl_mode as ::core::ffi::c_uchar;
                        mode += 1;
                    }
                    ref_0 += 1;
                }
            }
            seg += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_row_normal(
    mut cm: *mut VP8_COMMON,
    mut mode_info_context: *mut MODE_INFO,
    mut mb_row: ::core::ffi::c_int,
    mut post_ystride: ::core::ffi::c_int,
    mut post_uvstride: ::core::ffi::c_int,
    mut y_ptr: *mut ::core::ffi::c_uchar,
    mut u_ptr: *mut ::core::ffi::c_uchar,
    mut v_ptr: *mut ::core::ffi::c_uchar,
) {
    unsafe {
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        let mut lfi: loop_filter_info = loop_filter_info {
            mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
        };
        let mut frame_type: FRAME_TYPE = (*cm).frame_type;
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*cm).mb_cols {
            let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                as ::core::ffi::c_int
                != B_PRED as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                    != SPLITMV as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            let mode_index: ::core::ffi::c_int =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as ::core::ffi::c_int;
            let seg: ::core::ffi::c_int =
                (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
            let ref_frame: ::core::ffi::c_int =
                (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
            filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                as ::core::ffi::c_int;
            if filter_level != 0 {
                let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut[frame_type as usize]
                    [filter_level as usize]
                    as ::core::ffi::c_int;
                lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 16])
                    .offset(filter_level as isize)
                    as *mut ::core::ffi::c_uchar;
                lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 16])
                    .offset(filter_level as isize)
                    as *mut ::core::ffi::c_uchar;
                lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [::core::ffi::c_uchar; 16])
                    .offset(filter_level as isize)
                    as *mut ::core::ffi::c_uchar;
                lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                    as *mut [::core::ffi::c_uchar; 16])
                    .offset(hev_index as isize)
                    as *mut ::core::ffi::c_uchar;
                if mb_col > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_mbv_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bv_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if mb_row > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_mbh_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bh_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
            }
            y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
            u_ptr = u_ptr.offset(8 as ::core::ffi::c_int as isize);
            v_ptr = v_ptr.offset(8 as ::core::ffi::c_int as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_col += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_row_simple(
    mut cm: *mut VP8_COMMON,
    mut mode_info_context: *mut MODE_INFO,
    mut mb_row: ::core::ffi::c_int,
    mut post_ystride: ::core::ffi::c_int,
    mut y_ptr: *mut ::core::ffi::c_uchar,
) {
    unsafe {
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*cm).mb_cols {
            let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                as ::core::ffi::c_int
                != B_PRED as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                    != SPLITMV as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            let mode_index: ::core::ffi::c_int =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as ::core::ffi::c_int;
            let seg: ::core::ffi::c_int =
                (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
            let ref_frame: ::core::ffi::c_int =
                (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
            filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                as ::core::ffi::c_int;
            if filter_level != 0 {
                if mb_col > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_simple_vertical_edge_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bvs_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                }
                if mb_row > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_simple_horizontal_edge_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bhs_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                }
            }
            y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_col += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_frame(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut frame_type: ::core::ffi::c_int,
) {
    unsafe {
        let mut post: *mut YV12_BUFFER_CONFIG = (*cm).frame_to_show;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        let mut lfi: loop_filter_info = loop_filter_info {
            mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
        };
        let mut mb_row: ::core::ffi::c_int = 0;
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut mb_rows: ::core::ffi::c_int = (*cm).mb_rows;
        let mut mb_cols: ::core::ffi::c_int = (*cm).mb_cols;
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut u_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut v_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut mode_info_context: *const MODE_INFO = (*cm).mi;
        let mut post_y_stride: ::core::ffi::c_int = (*post).y_stride;
        let mut post_uv_stride: ::core::ffi::c_int = (*post).uv_stride;
        vp8_loop_filter_frame_init(cm, mbd, (*cm).filter_level);
        y_ptr = (*post).y_buffer as *mut ::core::ffi::c_uchar;
        u_ptr = (*post).u_buffer as *mut ::core::ffi::c_uchar;
        v_ptr = (*post).v_buffer as *mut ::core::ffi::c_uchar;
        if (*cm).filter_type as ::core::ffi::c_uint
            == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            mb_row = 0 as ::core::ffi::c_int;
            while mb_row < mb_rows {
                mb_col = 0 as ::core::ffi::c_int;
                while mb_col < mb_cols {
                    let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                        as ::core::ffi::c_int
                        != B_PRED as ::core::ffi::c_int
                        && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                            != SPLITMV as ::core::ffi::c_int
                        && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int;
                    let mode_index: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                        [(*mode_info_context).mbmi.mode as usize]
                        as ::core::ffi::c_int;
                    let seg: ::core::ffi::c_int =
                        (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                    let ref_frame: ::core::ffi::c_int =
                        (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                    filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize]
                        [mode_index as usize]
                        as ::core::ffi::c_int;
                    if filter_level != 0 {
                        let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut
                            [frame_type as usize][filter_level as usize]
                            as ::core::ffi::c_int;
                        lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.blim = &raw mut *(&raw mut (*lfi_n).blim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.lim = &raw mut *(&raw mut (*lfi_n).lim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(hev_index as isize)
                            as *mut ::core::ffi::c_uchar;
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbv_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbh_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                    }
                    y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                    u_ptr = u_ptr.offset(8 as ::core::ffi::c_int as isize);
                    v_ptr = v_ptr.offset(8 as ::core::ffi::c_int as isize);
                    mode_info_context = mode_info_context.offset(1);
                    mb_col += 1;
                }
                y_ptr = y_ptr
                    .offset((post_y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
                u_ptr = u_ptr
                    .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
                v_ptr = v_ptr
                    .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_row += 1;
            }
        } else {
            mb_row = 0 as ::core::ffi::c_int;
            while mb_row < mb_rows {
                mb_col = 0 as ::core::ffi::c_int;
                while mb_col < mb_cols {
                    let mut skip_lf_0: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                        as ::core::ffi::c_int
                        != B_PRED as ::core::ffi::c_int
                        && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                            != SPLITMV as ::core::ffi::c_int
                        && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int;
                    let mode_index_0: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                        [(*mode_info_context).mbmi.mode as usize]
                        as ::core::ffi::c_int;
                    let seg_0: ::core::ffi::c_int =
                        (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                    let ref_frame_0: ::core::ffi::c_int =
                        (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                    filter_level = (*lfi_n).lvl[seg_0 as usize][ref_frame_0 as usize]
                        [mode_index_0 as usize]
                        as ::core::ffi::c_int;
                    if filter_level != 0 {
                        let mut mblim: *const ::core::ffi::c_uchar =
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 16])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar;
                        let mut blim: *const ::core::ffi::c_uchar =
                            &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 16])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar;
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_simple_vertical_edge_c(y_ptr, post_y_stride, mblim);
                        }
                        if skip_lf_0 == 0 {
                            vp8_loop_filter_bvs_c(y_ptr, post_y_stride, blim);
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_simple_horizontal_edge_c(y_ptr, post_y_stride, mblim);
                        }
                        if skip_lf_0 == 0 {
                            vp8_loop_filter_bhs_c(y_ptr, post_y_stride, blim);
                        }
                    }
                    y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                    u_ptr = u_ptr.offset(8 as ::core::ffi::c_int as isize);
                    v_ptr = v_ptr.offset(8 as ::core::ffi::c_int as isize);
                    mode_info_context = mode_info_context.offset(1);
                    mb_col += 1;
                }
                y_ptr = y_ptr
                    .offset((post_y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
                u_ptr = u_ptr
                    .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
                v_ptr = v_ptr
                    .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_row += 1;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_frame_yonly(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: ::core::ffi::c_int,
) {
    unsafe {
        let mut post: *mut YV12_BUFFER_CONFIG = (*cm).frame_to_show;
        let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut mb_row: ::core::ffi::c_int = 0;
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        let mut lfi: loop_filter_info = loop_filter_info {
            mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
        };
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut frame_type: FRAME_TYPE = (*cm).frame_type;
        let mut mode_info_context: *const MODE_INFO = (*cm).mi;
        vp8_loop_filter_frame_init(cm, mbd, default_filt_lvl);
        y_ptr = (*post).y_buffer as *mut ::core::ffi::c_uchar;
        mb_row = 0 as ::core::ffi::c_int;
        while mb_row < (*cm).mb_rows {
            mb_col = 0 as ::core::ffi::c_int;
            while mb_col < (*cm).mb_cols {
                let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                    as ::core::ffi::c_int
                    != B_PRED as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                        != SPLITMV as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                let mode_index: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                    [(*mode_info_context).mbmi.mode as usize]
                    as ::core::ffi::c_int;
                let seg: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                let ref_frame: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                    as ::core::ffi::c_int;
                if filter_level != 0 {
                    if (*cm).filter_type as ::core::ffi::c_uint
                        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut
                            [frame_type as usize][filter_level as usize]
                            as ::core::ffi::c_int;
                        lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.blim = &raw mut *(&raw mut (*lfi_n).blim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.lim = &raw mut *(&raw mut (*lfi_n).lim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(hev_index as isize)
                            as *mut ::core::ffi::c_uchar;
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                    } else {
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_simple_vertical_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bvs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_simple_horizontal_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bhs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                    }
                }
                y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_col += 1;
            }
            y_ptr = y_ptr
                .offset(((*post).y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_row += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_partial_frame(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: ::core::ffi::c_int,
) {
    unsafe {
        let mut post: *mut YV12_BUFFER_CONFIG = (*cm).frame_to_show;
        let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut mb_row: ::core::ffi::c_int = 0;
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut mb_cols: ::core::ffi::c_int = (*post).y_width >> 4 as ::core::ffi::c_int;
        let mut mb_rows: ::core::ffi::c_int = (*post).y_height >> 4 as ::core::ffi::c_int;
        let mut linestocopy: ::core::ffi::c_int = 0;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
        let mut lfi: loop_filter_info = loop_filter_info {
            mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
            hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
        };
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut frame_type: FRAME_TYPE = (*cm).frame_type;
        let mut mode_info_context: *const MODE_INFO = ::core::ptr::null::<MODE_INFO>();
        vp8_loop_filter_frame_init(cm, mbd, default_filt_lvl);
        linestocopy = mb_rows / PARTIAL_FRAME_FRACTION;
        linestocopy = if linestocopy != 0 {
            linestocopy << 4 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        y_ptr = (*post).y_buffer.offset(
            (((*post).y_height >> 5 as ::core::ffi::c_int)
                * 16 as ::core::ffi::c_int
                * (*post).y_stride) as isize,
        ) as *mut ::core::ffi::c_uchar;
        mode_info_context = (*cm).mi.offset(
            (((*post).y_height >> 5 as ::core::ffi::c_int) * (mb_cols + 1 as ::core::ffi::c_int))
                as isize,
        );
        mb_row = 0 as ::core::ffi::c_int;
        while mb_row < linestocopy >> 4 as ::core::ffi::c_int {
            mb_col = 0 as ::core::ffi::c_int;
            while mb_col < mb_cols {
                let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                    as ::core::ffi::c_int
                    != B_PRED as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                        != SPLITMV as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                let mode_index: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                    [(*mode_info_context).mbmi.mode as usize]
                    as ::core::ffi::c_int;
                let seg: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                let ref_frame: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                    as ::core::ffi::c_int;
                if filter_level != 0 {
                    if (*cm).filter_type as ::core::ffi::c_uint
                        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut
                            [frame_type as usize][filter_level as usize]
                            as ::core::ffi::c_int;
                        lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.blim = &raw mut *(&raw mut (*lfi_n).blim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.lim = &raw mut *(&raw mut (*lfi_n).lim
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                            as *mut [::core::ffi::c_uchar; 16])
                            .offset(hev_index as isize)
                            as *mut ::core::ffi::c_uchar;
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                        vp8_loop_filter_mbh_c(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                (*post).y_stride,
                                0 as ::core::ffi::c_int,
                                &raw mut lfi,
                            );
                        }
                    } else {
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_simple_vertical_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bvs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        vp8_loop_filter_simple_horizontal_edge_c(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 16])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                        if skip_lf == 0 {
                            vp8_loop_filter_bhs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim
                                    as *mut [::core::ffi::c_uchar; 16])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                    }
                }
                y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                mode_info_context = mode_info_context.offset(1 as ::core::ffi::c_int as isize);
                mb_col += 1;
            }
            y_ptr = y_ptr
                .offset(((*post).y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
            mode_info_context = mode_info_context.offset(1 as ::core::ffi::c_int as isize);
            mb_row += 1;
        }
    }
}
