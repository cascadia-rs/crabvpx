use std::ffi::c_void;
unsafe extern "Rust" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn memset(__b: *mut c_void, __c: i32, __len: size_t) -> *mut c_void;
    fn vp8_rtcd();
    fn vpx_dsp_rtcd();
    fn vpx_scale_rtcd();
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const i8,
    );
    fn vp8_build_block_doffsets(x: *mut MACROBLOCKD);
    fn vp8_alloc_frame_buffers(oci: *mut VP8_COMMON, width: i32, height: i32) -> i32;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8dx_receive_compressed_data(pbi: *mut VP8D_COMP) -> i32;
    fn vp8dx_get_raw_frame(
        pbi: *mut VP8D_COMP,
        sd: *mut YV12_BUFFER_CONFIG,
        flags: *mut vp8_ppflags_t,
    ) -> i32;
    fn vp8dx_references_buffer(oci: *mut VP8Common, ref_frame: i32) -> i32;
    fn vp8dx_get_reference(
        pbi: *mut VP8D_COMP,
        ref_frame_flag: vpx_ref_frame_type,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> vpx_codec_err_t;
    fn vp8dx_set_reference(
        pbi: *mut VP8D_COMP,
        ref_frame_flag: vpx_ref_frame_type,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> vpx_codec_err_t;
    fn vp8dx_get_quantizer(pbi: *const VP8D_COMP) -> i32;
    fn vp8_create_decoder_instances(fb: *mut frame_buffers, oxcf: *mut VP8D_CONFIG) -> i32;
    fn vp8_remove_decoder_instances(fb: *mut frame_buffers) -> i32;
    fn vp8_decoder_remove_threads(pbi: *mut VP8D_COMP);
    fn vp8_decoder_create_threads(pbi: *mut VP8D_COMP);
    fn vp8mt_alloc_temp_buffers(pbi: *mut VP8D_COMP, width: i32, prev_mb_rows: i32);
    fn vp8mt_de_alloc_temp_buffers(pbi: *mut VP8D_COMP, mb_rows: i32);
}
pub type __builtin_va_list = *mut i8;
pub type __darwin_natural_t = u32;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: i64,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [i8; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type int64_t = i64;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
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
    pub row: i16,
    pub col: i16,
}
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
pub struct macroblockd {
    pub predictor: [u8; 384],
    pub qcoeff: [i16; 400],
    pub dqcoeff: [i16; 400],
    pub eobs: [i8; 25],
    pub dequant_y1: [i16; 16],
    pub dequant_y1_dc: [i16; 16],
    pub dequant_y2: [i16; 16],
    pub dequant_uv: [i16; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: i32,
    pub pre: YV12_BUFFER_CONFIG,
    pub dst: YV12_BUFFER_CONFIG,
    pub mode_info_context: *mut MODE_INFO,
    pub mode_info_stride: i32,
    pub frame_type: FRAME_TYPE,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: *mut ENTROPY_CONTEXT_PLANES,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [vp8_prob; 3],
    pub segment_feature_data: [[i8; 4]; 2],
    pub mode_ref_lf_delta_enabled: u8,
    pub mode_ref_lf_delta_update: u8,
    pub last_ref_lf_deltas: [i8; 4],
    pub ref_lf_deltas: [i8; 4],
    pub last_mode_lf_deltas: [i8; 4],
    pub mode_lf_deltas: [i8; 4],
    pub mb_to_left_edge: i32,
    pub mb_to_right_edge: i32,
    pub mb_to_top_edge: i32,
    pub mb_to_bottom_edge: i32,
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc: *mut c_void,
    pub corrupted: i32,
    pub error_info: vpx_internal_error_info,
}
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
pub type vp8_subpix_fn_t = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
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
pub type BLOCKD = blockd;
pub type vpx_img_fmt = u32;
pub const VPX_IMG_FMT_I44016: vpx_img_fmt = 2311;
pub const VPX_IMG_FMT_I44416: vpx_img_fmt = 2310;
pub const VPX_IMG_FMT_I42216: vpx_img_fmt = 2309;
pub const VPX_IMG_FMT_I42016: vpx_img_fmt = 2306;
pub const VPX_IMG_FMT_NV12: vpx_img_fmt = 265;
pub const VPX_IMG_FMT_I440: vpx_img_fmt = 263;
pub const VPX_IMG_FMT_I444: vpx_img_fmt = 262;
pub const VPX_IMG_FMT_I422: vpx_img_fmt = 261;
pub const VPX_IMG_FMT_I420: vpx_img_fmt = 258;
pub const VPX_IMG_FMT_YV12: vpx_img_fmt = 769;
pub const VPX_IMG_FMT_NONE: vpx_img_fmt = 0;
pub type vpx_img_fmt_t = vpx_img_fmt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_image {
    pub fmt: vpx_img_fmt_t,
    pub cs: vpx_color_space_t,
    pub range: vpx_color_range_t,
    pub w: u32,
    pub h: u32,
    pub bit_depth: u32,
    pub d_w: u32,
    pub d_h: u32,
    pub r_w: u32,
    pub r_h: u32,
    pub x_chroma_shift: u32,
    pub y_chroma_shift: u32,
    pub planes: [*mut u8; 4],
    pub stride: [i32; 4],
    pub bps: i32,
    pub user_priv: *mut c_void,
    pub img_data: *mut u8,
    pub img_data_owner: i32,
    pub self_allocd: i32,
    pub fb_priv: *mut c_void,
}
pub type vpx_image_t = vpx_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_image_rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
pub type vpx_image_rect_t = vpx_image_rect;
pub type vpx_codec_caps_t = i64;
pub type vpx_codec_flags_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_iface {
    pub name: *const i8,
    pub abi_version: i32,
    pub caps: vpx_codec_caps_t,
    pub init: vpx_codec_init_fn_t,
    pub destroy: vpx_codec_destroy_fn_t,
    pub ctrl_maps: *const vpx_codec_ctrl_fn_map_t,
    pub dec: vpx_codec_dec_iface,
    pub enc: vpx_codec_enc_iface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_enc_iface {
    pub cfg_map_count: i32,
    pub cfg_maps: *const vpx_codec_enc_cfg_map_t,
    pub encode: vpx_codec_encode_fn_t,
    pub get_cx_data: vpx_codec_get_cx_data_fn_t,
    pub cfg_set: vpx_codec_enc_config_set_fn_t,
    pub get_glob_hdrs: vpx_codec_get_global_headers_fn_t,
    pub get_preview: vpx_codec_get_preview_frame_fn_t,
    pub mr_get_mem_loc: vpx_codec_enc_mr_get_mem_loc_fn_t,
    pub mr_free_mem_loc: vpx_codec_enc_mr_free_mem_loc_fn_t,
}
pub type vpx_codec_enc_mr_free_mem_loc_fn_t = Option<unsafe fn(*mut c_void) -> ()>;
pub type vpx_codec_enc_mr_get_mem_loc_fn_t =
    Option<unsafe fn(*const vpx_codec_enc_cfg_t, *mut *mut c_void) -> vpx_codec_err_t>;
pub type vpx_codec_enc_cfg_t = vpx_codec_enc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_enc_cfg {
    pub g_usage: u32,
    pub g_threads: u32,
    pub g_profile: u32,
    pub g_w: u32,
    pub g_h: u32,
    pub g_bit_depth: vpx_bit_depth_t,
    pub g_input_bit_depth: u32,
    pub g_timebase: vpx_rational,
    pub g_error_resilient: vpx_codec_er_flags_t,
    pub g_pass: vpx_enc_pass,
    pub g_lag_in_frames: u32,
    pub rc_dropframe_thresh: u32,
    pub rc_resize_allowed: u32,
    pub rc_scaled_width: u32,
    pub rc_scaled_height: u32,
    pub rc_resize_up_thresh: u32,
    pub rc_resize_down_thresh: u32,
    pub rc_end_usage: vpx_rc_mode,
    pub rc_twopass_stats_in: vpx_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: vpx_fixed_buf_t,
    pub rc_target_bitrate: u32,
    pub rc_min_quantizer: u32,
    pub rc_max_quantizer: u32,
    pub rc_undershoot_pct: u32,
    pub rc_overshoot_pct: u32,
    pub rc_buf_sz: u32,
    pub rc_buf_initial_sz: u32,
    pub rc_buf_optimal_sz: u32,
    pub rc_2pass_vbr_bias_pct: u32,
    pub rc_2pass_vbr_minsection_pct: u32,
    pub rc_2pass_vbr_maxsection_pct: u32,
    pub rc_2pass_vbr_corpus_complexity: u32,
    pub kf_mode: vpx_kf_mode,
    pub kf_min_dist: u32,
    pub kf_max_dist: u32,
    pub ss_number_layers: u32,
    pub ss_enable_auto_alt_ref: [i32; 5],
    pub ss_target_bitrate: [u32; 5],
    pub ts_number_layers: u32,
    pub ts_target_bitrate: [u32; 5],
    pub ts_rate_decimator: [u32; 5],
    pub ts_periodicity: u32,
    pub ts_layer_id: [u32; 16],
    pub layer_target_bitrate: [u32; 12],
    pub temporal_layering_mode: i32,
    pub use_vizier_rc_params: i32,
    pub active_wq_factor: vpx_rational_t,
    pub err_per_mb_factor: vpx_rational_t,
    pub sr_default_decay_limit: vpx_rational_t,
    pub sr_diff_factor: vpx_rational_t,
    pub kf_err_per_mb_factor: vpx_rational_t,
    pub kf_frame_min_boost_factor: vpx_rational_t,
    pub kf_frame_max_boost_first_factor: vpx_rational_t,
    pub kf_frame_max_boost_subs_factor: vpx_rational_t,
    pub kf_max_total_boost_factor: vpx_rational_t,
    pub gf_max_total_boost_factor: vpx_rational_t,
    pub gf_frame_max_boost_factor: vpx_rational_t,
    pub zm_factor: vpx_rational_t,
    pub rd_mult_inter_qp_fac: vpx_rational_t,
    pub rd_mult_arf_qp_fac: vpx_rational_t,
    pub rd_mult_key_qp_fac: vpx_rational_t,
}
pub type vpx_rational_t = vpx_rational;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_rational {
    pub num: i32,
    pub den: i32,
}
pub type vpx_kf_mode = u32;
pub const VPX_KF_DISABLED: vpx_kf_mode = 0;
pub const VPX_KF_AUTO: vpx_kf_mode = 1;
pub const VPX_KF_FIXED: vpx_kf_mode = 0;
pub type vpx_fixed_buf_t = vpx_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_fixed_buf {
    pub buf: *mut c_void,
    pub sz: size_t,
}
pub type vpx_rc_mode = u32;
pub const VPX_Q: vpx_rc_mode = 3;
pub const VPX_CQ: vpx_rc_mode = 2;
pub const VPX_CBR: vpx_rc_mode = 1;
pub const VPX_VBR: vpx_rc_mode = 0;
pub type vpx_enc_pass = u32;
pub const VPX_RC_LAST_PASS: vpx_enc_pass = 2;
pub const VPX_RC_FIRST_PASS: vpx_enc_pass = 1;
pub const VPX_RC_ONE_PASS: vpx_enc_pass = 0;
pub type vpx_codec_er_flags_t = uint32_t;
pub type vpx_bit_depth_t = vpx_bit_depth;
pub type vpx_bit_depth = u32;
pub const VPX_BITS_12: vpx_bit_depth = 12;
pub const VPX_BITS_10: vpx_bit_depth = 10;
pub const VPX_BITS_8: vpx_bit_depth = 8;
pub type vpx_codec_get_preview_frame_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t) -> *mut vpx_image_t>;
pub type vpx_codec_alg_priv_t = vpx_codec_alg_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_alg_priv {
    pub base: vpx_codec_priv_t,
    pub cfg: vpx_codec_dec_cfg_t,
    pub si: vp8_stream_info_t,
    pub decoder_init: i32,
    pub restart_threads: i32,
    pub postproc_cfg_set: i32,
    pub postproc_cfg: vp8_postproc_cfg_t,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
    pub img: vpx_image_t,
    pub img_setup: i32,
    pub yv12_frame_buffers: frame_buffers,
    pub user_priv: *mut c_void,
    pub fragments: FRAGMENT_DATA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAGMENT_DATA {
    pub enabled: i32,
    pub count: u32,
    pub ptrs: [*const u8; 9],
    pub sizes: [u32; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_buffers {
    pub pbi: [*mut VP8D_COMP; 32],
}
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
    pub max_threads: i32,
    pub current_mb_col_main: i32,
    pub decoding_thread_count: u32,
    pub allocated_decoding_thread_count: i32,
    pub mt_baseline_filter_level: [i32; 4],
    pub sync_range: i32,
    pub mt_current_mb_col: *mut vpx_atomic_int,
    pub mt_yabove_row: *mut *mut u8,
    pub mt_uabove_row: *mut *mut u8,
    pub mt_vabove_row: *mut *mut u8,
    pub mt_yleft_col: *mut *mut u8,
    pub mt_uleft_col: *mut *mut u8,
    pub mt_vleft_col: *mut *mut u8,
    pub mb_row_di: *mut MB_ROW_DEC,
    pub de_thread_data: *mut DECODETHREAD_DATA,
    pub h_decoding_thread: *mut pthread_t,
    pub h_event_start_decoding: *mut semaphore_t,
    pub h_event_end_decoding: semaphore_t,
    pub ready_for_new_data: i32,
    pub prob_intra: vp8_prob,
    pub prob_last: vp8_prob,
    pub prob_gf: vp8_prob,
    pub prob_skip_false: vp8_prob,
    pub ec_enabled: i32,
    pub ec_active: i32,
    pub decoded_key_frame: i32,
    pub independent_partitions: i32,
    pub frame_corrupt_residual: i32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
    pub restart_threads: i32,
}
pub type vpx_decrypt_cb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type semaphore_t = *mut c_void;
pub type mach_port_t = __darwin_mach_port_t;
pub type pthread_t = *mut c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODETHREAD_DATA {
    pub ithread: i32,
    pub ptr1: *mut c_void,
    pub ptr2: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_ROW_DEC {
    pub mbd: MACROBLOCKD,
}
pub type MACROBLOCKD = macroblockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_atomic_int {
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_CONFIG {
    pub Width: i32,
    pub Height: i32,
    pub Version: i32,
    pub postprocess: i32,
    pub max_threads: i32,
    pub error_concealment: i32,
}
pub type BOOL_DECODER = vp8_reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_reader {
    pub user_buffer_end: *const u8,
    pub user_buffer: *const u8,
    pub value: VP8_BD_VALUE,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
}
pub type VP8_BD_VALUE = size_t;
pub type VP8_COMMON = VP8Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub Y1dequant: [[i16; 2]; 128],
    pub Y2dequant: [[i16; 2]; 128],
    pub UVdequant: [[i16; 2]; 128],
    pub Width: i32,
    pub Height: i32,
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
    pub MBs: i32,
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
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
pub type CLAMP_TYPE = u32;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type vp8_postproc_cfg_t = vp8_postproc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_postproc_cfg {
    pub post_proc_flag: i32,
    pub deblocking_level: i32,
    pub noise_level: i32,
}
pub type vp8_stream_info_t = vpx_codec_stream_info_t;
pub type vpx_codec_stream_info_t = vpx_codec_stream_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_stream_info {
    pub sz: u32,
    pub w: u32,
    pub h: u32,
    pub is_kf: u32,
}
pub type vpx_codec_dec_cfg_t = vpx_codec_dec_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_dec_cfg {
    pub threads: u32,
    pub w: u32,
    pub h: u32,
}
pub type vpx_codec_priv_t = vpx_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv {
    pub err_detail: *const i8,
    pub init_flags: vpx_codec_flags_t,
    pub dec: C2RustUnnamed_2,
    pub enc: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub cx_data_dst_buf: vpx_fixed_buf_t,
    pub cx_data_pad_before: u32,
    pub cx_data_pad_after: u32,
    pub cx_data_pkt: vpx_codec_cx_pkt_t,
    pub total_encoders: u32,
}
pub type vpx_codec_cx_pkt_t = vpx_codec_cx_pkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_cx_pkt {
    pub kind: vpx_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub frame: C2RustUnnamed_1,
    pub twopass_stats: vpx_fixed_buf_t,
    pub firstpass_mb_stats: vpx_fixed_buf_t,
    pub psnr: vpx_psnr_pkt,
    pub raw: vpx_fixed_buf_t,
    pub pad: [i8; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_psnr_pkt {
    pub samples: [u32; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [f64; 4],
    pub spatial_layer_id: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub buf: *mut c_void,
    pub sz: size_t,
    pub pts: vpx_codec_pts_t,
    pub duration: u64,
    pub flags: vpx_codec_frame_flags_t,
    pub partition_id: i32,
    pub width: [u32; 5],
    pub height: [u32; 5],
    pub spatial_layer_encoded: [uint8_t; 5],
}
pub type vpx_codec_frame_flags_t = uint32_t;
pub type vpx_codec_pts_t = int64_t;
pub type vpx_codec_cx_pkt_kind = u32;
pub const VPX_CODEC_CUSTOM_PKT: vpx_codec_cx_pkt_kind = 256;
pub const VPX_CODEC_PSNR_PKT: vpx_codec_cx_pkt_kind = 3;
pub const VPX_CODEC_FPMB_STATS_PKT: vpx_codec_cx_pkt_kind = 2;
pub const VPX_CODEC_STATS_PKT: vpx_codec_cx_pkt_kind = 1;
pub const VPX_CODEC_CX_FRAME_PKT: vpx_codec_cx_pkt_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub put_frame_cb: vpx_codec_priv_cb_pair_t,
    pub put_slice_cb: vpx_codec_priv_cb_pair_t,
}
pub type vpx_codec_priv_cb_pair_t = vpx_codec_priv_cb_pair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv_cb_pair {
    pub u: C2RustUnnamed_3,
    pub user_priv: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub put_frame: vpx_codec_put_frame_cb_fn_t,
    pub put_slice: vpx_codec_put_slice_cb_fn_t,
}
pub type vpx_codec_put_slice_cb_fn_t = Option<
    unsafe fn(
        *mut c_void,
        *const vpx_image_t,
        *const vpx_image_rect_t,
        *const vpx_image_rect_t,
    ) -> (),
>;
pub type vpx_codec_put_frame_cb_fn_t = Option<unsafe fn(*mut c_void, *const vpx_image_t) -> ()>;
pub type vpx_codec_get_global_headers_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t) -> *mut vpx_fixed_buf_t>;
pub type vpx_codec_enc_config_set_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t, *const vpx_codec_enc_cfg_t) -> vpx_codec_err_t>;
pub type vpx_codec_get_cx_data_fn_t = Option<
    unsafe fn(*mut vpx_codec_alg_priv_t, *mut vpx_codec_iter_t) -> *const vpx_codec_cx_pkt_t,
>;
pub type vpx_codec_iter_t = *const c_void;
pub type vpx_codec_encode_fn_t = Option<
    unsafe fn(
        *mut vpx_codec_alg_priv_t,
        *const vpx_image_t,
        vpx_codec_pts_t,
        u64,
        vpx_enc_frame_flags_t,
        vpx_enc_deadline_t,
    ) -> vpx_codec_err_t,
>;
pub type vpx_enc_deadline_t = u64;
pub type vpx_enc_frame_flags_t = i64;
pub type vpx_codec_enc_cfg_map_t = vpx_codec_enc_cfg_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_enc_cfg_map {
    pub usage: i32,
    pub cfg: vpx_codec_enc_cfg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_dec_iface {
    pub peek_si: vpx_codec_peek_si_fn_t,
    pub get_si: vpx_codec_get_si_fn_t,
    pub decode: vpx_codec_decode_fn_t,
    pub get_frame: vpx_codec_get_frame_fn_t,
    pub set_fb_fn: vpx_codec_set_fb_fn_t,
}
pub type vpx_codec_set_fb_fn_t = Option<
    unsafe fn(
        *mut vpx_codec_alg_priv_t,
        vpx_get_frame_buffer_cb_fn_t,
        vpx_release_frame_buffer_cb_fn_t,
        *mut c_void,
    ) -> vpx_codec_err_t,
>;
pub type vpx_release_frame_buffer_cb_fn_t =
    Option<unsafe fn(*mut c_void, *mut vpx_codec_frame_buffer_t) -> i32>;
pub type vpx_codec_frame_buffer_t = vpx_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut c_void,
}
pub type vpx_get_frame_buffer_cb_fn_t =
    Option<unsafe fn(*mut c_void, size_t, *mut vpx_codec_frame_buffer_t) -> i32>;
pub type vpx_codec_get_frame_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t, *mut vpx_codec_iter_t) -> *mut vpx_image_t>;
pub type vpx_codec_decode_fn_t = Option<
    unsafe fn(*mut vpx_codec_alg_priv_t, *const uint8_t, u32, *mut c_void) -> vpx_codec_err_t,
>;
pub type vpx_codec_get_si_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t, *mut vpx_codec_stream_info_t) -> vpx_codec_err_t>;
pub type vpx_codec_peek_si_fn_t =
    Option<unsafe fn(*const uint8_t, u32, *mut vpx_codec_stream_info_t) -> vpx_codec_err_t>;
pub type vpx_codec_ctrl_fn_map_t = vpx_codec_ctrl_fn_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_ctrl_fn_map {
    pub ctrl_id: i32,
    pub fn_0: vpx_codec_control_fn_t,
}
pub type vpx_codec_control_fn_t =
    Option<unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t>;
pub type va_list = __builtin_va_list;
pub type vpx_codec_destroy_fn_t = Option<unsafe fn(*mut vpx_codec_alg_priv_t) -> vpx_codec_err_t>;
pub type vpx_codec_init_fn_t =
    Option<unsafe fn(*mut vpx_codec_ctx_t, *mut vpx_codec_priv_enc_mr_cfg_t) -> vpx_codec_err_t>;
pub type vpx_codec_priv_enc_mr_cfg_t = vpx_codec_priv_enc_mr_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv_enc_mr_cfg {
    pub mr_total_resolutions: u32,
    pub mr_encoder_id: u32,
    pub mr_down_sampling_factor: vpx_rational,
    pub mr_low_res_mode_info: *mut c_void,
}
pub type vpx_codec_ctx_t = vpx_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_ctx {
    pub name: *const i8,
    pub iface: *const vpx_codec_iface_t,
    pub err: vpx_codec_err_t,
    pub err_detail: *const i8,
    pub init_flags: vpx_codec_flags_t,
    pub config: C2RustUnnamed_4,
    pub priv_0: *mut vpx_codec_priv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub dec: *const vpx_codec_dec_cfg,
    pub enc: *const vpx_codec_enc_cfg,
    pub raw: *const c_void,
}
pub type vpx_codec_iface_t = vpx_codec_iface;
pub type vp8_com_control_id = u32;
pub const VP8_DECODER_CTRL_ID_START: vp8_com_control_id = 256;
pub const VP8_COMMON_CTRL_ID_MAX: vp8_com_control_id = 129;
pub const VP9_GET_REFERENCE: vp8_com_control_id = 128;
pub const VP8_SET_POSTPROC: vp8_com_control_id = 3;
pub const VP8_COPY_REFERENCE: vp8_com_control_id = 2;
pub const VP8_SET_REFERENCE: vp8_com_control_id = 1;
pub type vp8_postproc_level = u32;
pub const VP8_MFQE: vp8_postproc_level = 8;
pub const VP8_ADDNOISE: vp8_postproc_level = 4;
pub const VP8_DEMACROBLOCK: vp8_postproc_level = 2;
pub const VP8_DEBLOCK: vp8_postproc_level = 1;
pub const VP8_NOFILTERING: vp8_postproc_level = 0;
pub type vpx_ref_frame_type = u32;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
pub type vpx_ref_frame_type_t = vpx_ref_frame_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_ref_frame {
    pub frame_type: vpx_ref_frame_type_t,
    pub img: vpx_image_t,
}
pub type vpx_ref_frame_t = vpx_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_ppflags_t {
    pub post_proc_flag: i32,
    pub deblocking_level: i32,
    pub noise_level: i32,
    pub display_ref_frame_flag: i32,
    pub display_mb_modes_flag: i32,
    pub display_b_modes_flag: i32,
    pub display_mv_flag: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_decrypt_init {
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
}
pub const VPXD_SET_DECRYPTOR: vp8_dec_control_id = 259;
pub const VPXD_GET_LAST_QUANTIZER: vp8_dec_control_id = 267;
pub const LAST_FRAME: C2RustUnnamed_5 = 1;
pub const GOLDEN_FRAME: C2RustUnnamed_5 = 2;
pub const ALTREF_FRAME: C2RustUnnamed_5 = 3;
pub const VP8D_GET_LAST_REF_USED: vp8_dec_control_id = 258;
pub const VP8D_GET_FRAME_CORRUPTED: vp8_dec_control_id = 257;
pub const VP8D_GET_LAST_REF_UPDATES: vp8_dec_control_id = 256;
pub type vp8_dec_control_id = u32;
pub const VP8_DECODER_CTRL_ID_MAX: vp8_dec_control_id = 270;
pub const VP9D_SET_LOOP_FILTER_OPT: vp8_dec_control_id = 269;
pub const VP9D_SET_ROW_MT: vp8_dec_control_id = 268;
pub const VP9_DECODE_SVC_SPATIAL_LAYER: vp8_dec_control_id = 266;
pub const VP9_SET_SKIP_LOOP_FILTER: vp8_dec_control_id = 265;
pub const VP9_INVERT_TILE_DECODE_ORDER: vp8_dec_control_id = 264;
pub const VP9_SET_BYTE_ALIGNMENT: vp8_dec_control_id = 263;
pub const VP9D_GET_BIT_DEPTH: vp8_dec_control_id = 262;
pub const VP9D_GET_DISPLAY_SIZE: vp8_dec_control_id = 261;
pub const VP9D_GET_FRAME_SIZE: vp8_dec_control_id = 260;
pub const VP8D_SET_DECRYPTOR: vp8_dec_control_id = 259;
pub type C2RustUnnamed_5 = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed_5 = 4;
pub const INTRA_FRAME: C2RustUnnamed_5 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const CONFIG_POSTPROC: i32 = 0 as i32;
pub const CONFIG_ERROR_CONCEALMENT: i32 = 0 as i32;
pub const VPX_PLANE_Y: i32 = 0 as i32;
pub const VPX_PLANE_U: i32 = 1 as i32;
pub const VPX_PLANE_V: i32 = 2 as i32;
pub const VPX_PLANE_ALPHA: i32 = 3 as i32;
pub const VPX_CODEC_CAP_DECODER: i32 = 0x1 as i32;
pub const VPX_CODEC_CAP_POSTPROC: i32 = 0x40000 as i32;
pub const VPX_CODEC_CAP_ERROR_CONCEALMENT: i32 = 0x80000 as i32;
pub const VPX_CODEC_CAP_INPUT_FRAGMENTS: i32 = 0x100000 as i32;
pub const VPX_CODEC_USE_POSTPROC: i32 = 0x10000 as i32;
pub const VPX_CODEC_USE_ERROR_CONCEALMENT: i32 = 0x20000 as i32;
pub const VPX_CODEC_USE_INPUT_FRAGMENTS: i32 = 0x40000 as i32;
pub const VPX_CODEC_INTERNAL_ABI_VERSION: i32 = 5 as i32;
pub const VP8BORDERINPIXELS: i32 = 32 as i32;
pub const MAX_PARTITIONS: i32 = 9 as i32;
#[inline]
unsafe fn vpx_atomic_load_acquire(mut atomic: *const vpx_atomic_int) -> i32 {
    unsafe {
        (*((&raw const (*atomic).value) as *const core::sync::atomic::AtomicI32))
            .load(core::sync::atomic::Ordering::Acquire)
    }
}
unsafe fn vp8_init_ctx(mut ctx: *mut vpx_codec_ctx_t) -> i32 {
    unsafe {
        let mut priv_0: *mut vpx_codec_alg_priv_t = vpx_calloc(
            1 as size_t,
            ::core::mem::size_of::<vpx_codec_alg_priv_t>() as size_t,
        ) as *mut vpx_codec_alg_priv_t;
        if priv_0.is_null() {
            return 1 as i32;
        }
        (*ctx).priv_0 = priv_0 as *mut vpx_codec_priv_t;
        (*(*ctx).priv_0).init_flags = (*ctx).init_flags;
        (*priv_0).si.sz = ::core::mem::size_of::<vp8_stream_info_t>() as u32;
        (*priv_0).decrypt_cb = None;
        (*priv_0).decrypt_state = NULL;
        if !(*ctx).config.dec.is_null() {
            (*priv_0).cfg = *(*ctx).config.dec as vpx_codec_dec_cfg_t;
            (*ctx).config.dec = &raw mut (*priv_0).cfg;
        }
        0 as i32
    }
}
unsafe fn vp8_init(
    mut ctx: *mut vpx_codec_ctx_t,
    _data: *mut vpx_codec_priv_enc_mr_cfg_t,
) -> vpx_codec_err_t {
    unsafe {
        let mut res: vpx_codec_err_t = VPX_CODEC_OK;
        vp8_rtcd();
        vpx_dsp_rtcd();
        vpx_scale_rtcd();
        if (*ctx).priv_0.is_null() {
            let mut priv_0: *mut vpx_codec_alg_priv_t =
                ::core::ptr::null_mut::<vpx_codec_alg_priv_t>();
            if vp8_init_ctx(ctx) != 0 {
                return VPX_CODEC_MEM_ERROR;
            }
            priv_0 = (*ctx).priv_0 as *mut vpx_codec_alg_priv_t;
            (*priv_0).fragments.count = 0 as u32;
            (*priv_0).fragments.enabled = ((*priv_0).base.init_flags
                & VPX_CODEC_USE_INPUT_FRAGMENTS as vpx_codec_flags_t)
                as i32;
        }
        res
    }
}
unsafe fn vp8_destroy(mut ctx: *mut vpx_codec_alg_priv_t) -> vpx_codec_err_t {
    unsafe {
        vp8_remove_decoder_instances(&raw mut (*ctx).yv12_frame_buffers);
        vpx_free(ctx as *mut c_void);
        VPX_CODEC_OK
    }
}
unsafe fn vp8_peek_si_internal(
    mut data: *const uint8_t,
    mut data_sz: u32,
    mut si: *mut vpx_codec_stream_info_t,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut res: vpx_codec_err_t = VPX_CODEC_OK;
        if data.is_null() {
            return VPX_CODEC_INVALID_PARAM;
        }
        if data.offset(data_sz as isize) <= data {
            res = VPX_CODEC_INVALID_PARAM;
        } else {
            let mut clear_buffer: [uint8_t; 10] = [0; 10];
            let mut clear: *const uint8_t = data;
            if decrypt_cb.is_some() {
                let mut n: i32 =
                    (if (::core::mem::size_of::<[uint8_t; 10]>() as usize) < data_sz as usize {
                        ::core::mem::size_of::<[uint8_t; 10]>() as usize
                    } else {
                        data_sz as usize
                    }) as i32;
                decrypt_cb.expect("non-null function pointer")(
                    decrypt_state,
                    data as *const u8,
                    &raw mut clear_buffer as *mut u8,
                    n,
                );
                clear = &raw mut clear_buffer as *mut uint8_t;
            }
            (*si).is_kf = 0 as u32;
            if data_sz >= 10 as u32 && *clear.offset(0 as isize) as i32 & 0x1 as i32 == 0 {
                (*si).is_kf = 1 as u32;
                if *clear.offset(3 as isize) as i32 != 0x9d as i32
                    || *clear.offset(4 as isize) as i32 != 0x1 as i32
                    || *clear.offset(5 as isize) as i32 != 0x2a as i32
                {
                    return VPX_CODEC_UNSUP_BITSTREAM;
                }
                (*si).w = ((*clear.offset(6 as isize) as i32
                    | (*clear.offset(7 as isize) as i32) << 8 as i32)
                    & 0x3fff as i32) as u32;
                (*si).h = ((*clear.offset(8 as isize) as i32
                    | (*clear.offset(9 as isize) as i32) << 8 as i32)
                    & 0x3fff as i32) as u32;
                if !((*si).h != 0 && (*si).w != 0) {
                    (*si).h = 0 as u32;
                    (*si).w = (*si).h;
                    res = VPX_CODEC_CORRUPT_FRAME;
                }
            } else {
                res = VPX_CODEC_UNSUP_BITSTREAM;
            }
        }
        res
    }
}
unsafe fn vp8_peek_si(
    mut data: *const uint8_t,
    mut data_sz: u32,
    mut si: *mut vpx_codec_stream_info_t,
) -> vpx_codec_err_t {
    unsafe { vp8_peek_si_internal(data, data_sz, si, None, NULL) }
}
unsafe fn vp8_get_si(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut si: *mut vpx_codec_stream_info_t,
) -> vpx_codec_err_t {
    unsafe {
        let mut sz: u32 = 0;
        if (*si).sz as usize >= ::core::mem::size_of::<vp8_stream_info_t>() as usize {
            sz = ::core::mem::size_of::<vp8_stream_info_t>() as u32;
        } else {
            sz = ::core::mem::size_of::<vpx_codec_stream_info_t>() as u32;
        }
        memcpy(
            si as *mut c_void,
            &raw mut (*ctx).si as *const c_void,
            sz as size_t,
        );
        (*si).sz = sz;
        VPX_CODEC_OK
    }
}
unsafe fn update_error_state(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut error: *const vpx_internal_error_info,
) -> vpx_codec_err_t {
    unsafe {
        let mut res: vpx_codec_err_t = VPX_CODEC_OK;
        res = (*error).error_code;
        if res as u64 != 0 {
            (*ctx).base.err_detail = if (*error).has_detail != 0 {
                &raw const (*error).detail as *const i8
            } else {
                ::core::ptr::null::<i8>()
            };
        }
        res
    }
}
unsafe fn yuvconfig2image(
    mut img: *mut vpx_image_t,
    mut yv12: *const YV12_BUFFER_CONFIG,
    mut user_priv: *mut c_void,
) {
    unsafe {
        (*img).fmt = VPX_IMG_FMT_I420;
        (*img).w = (*yv12).y_stride as u32;
        (*img).h =
            (((*yv12).y_height + 2 as i32 * VP8BORDERINPIXELS + 15 as i32) & !(15 as i32)) as u32;
        (*img).r_w = (*yv12).y_width as u32;
        (*img).d_w = (*img).r_w;
        (*img).r_h = (*yv12).y_height as u32;
        (*img).d_h = (*img).r_h;
        (*img).x_chroma_shift = 1 as u32;
        (*img).y_chroma_shift = 1 as u32;
        (*img).planes[VPX_PLANE_Y as usize] = (*yv12).y_buffer as *mut u8;
        (*img).planes[VPX_PLANE_U as usize] = (*yv12).u_buffer as *mut u8;
        (*img).planes[VPX_PLANE_V as usize] = (*yv12).v_buffer as *mut u8;
        (*img).planes[VPX_PLANE_ALPHA as usize] = ::core::ptr::null_mut::<u8>();
        (*img).stride[VPX_PLANE_Y as usize] = (*yv12).y_stride;
        (*img).stride[VPX_PLANE_U as usize] = (*yv12).uv_stride;
        (*img).stride[VPX_PLANE_V as usize] = (*yv12).uv_stride;
        (*img).stride[VPX_PLANE_ALPHA as usize] = (*yv12).y_stride;
        (*img).bit_depth = 8 as u32;
        (*img).bps = 12 as i32;
        (*img).user_priv = user_priv;
        (*img).img_data = (*yv12).buffer_alloc as *mut u8;
        (*img).img_data_owner = 0 as i32;
        (*img).self_allocd = 0 as i32;
    }
}
unsafe fn update_fragments(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: u32,
    mut res: *mut vpx_codec_err_t,
) -> i32 {
    unsafe {
        ::core::ptr::write_volatile(res, VPX_CODEC_OK);
        if (*ctx).fragments.count == 0 as u32 {
            memset(
                &raw mut (*ctx).fragments.ptrs as *mut *const u8 as *mut c_void,
                0 as i32,
                ::core::mem::size_of::<[*const u8; 9]>() as size_t,
            );
            memset(
                &raw mut (*ctx).fragments.sizes as *mut u32 as *mut c_void,
                0 as i32,
                ::core::mem::size_of::<[u32; 9]>() as size_t,
            );
        }
        if (*ctx).fragments.enabled != 0
            && data.is_null()
            && data_sz == 0 as u32
            && (*ctx).fragments.count == 0 as u32
        {
            return 0 as i32;
        }
        if (*ctx).fragments.enabled != 0 && !(data.is_null() && data_sz == 0 as u32) {
            if (*ctx).fragments.count >= MAX_PARTITIONS as u32 {
                (*ctx).fragments.count = 0 as u32;
                ::core::ptr::write_volatile(res, VPX_CODEC_INVALID_PARAM);
                return -(1 as i32);
            }
            (*ctx).fragments.ptrs[(*ctx).fragments.count as usize] = data as *const u8;
            (*ctx).fragments.sizes[(*ctx).fragments.count as usize] = data_sz;
            (*ctx).fragments.count = (*ctx).fragments.count.wrapping_add(1);
            return 0 as i32;
        }
        if (*ctx).fragments.enabled == 0 && (data.is_null() && data_sz == 0 as u32) {
            return 0 as i32;
        }
        if (*ctx).fragments.enabled == 0 {
            (*ctx).fragments.ptrs[0 as usize] = data as *const u8;
            (*ctx).fragments.sizes[0 as usize] = data_sz;
            (*ctx).fragments.count = 1 as u32;
        }
        1 as i32
    }
}
unsafe fn vp8_decode(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: u32,
    mut user_priv: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut res: vpx_codec_err_t = VPX_CODEC_OK;
        let mut resolution_change: u32 = 0 as u32;
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        if (*ctx).fragments.enabled == 0 && (data.is_null() && data_sz == 0 as u32) {
            return VPX_CODEC_OK;
        }
        if update_fragments(ctx, data, data_sz, &raw mut res) <= 0 as i32 {
            return res;
        }
        ::core::ptr::write_volatile(&mut w as *mut u32, (*ctx).si.w);
        ::core::ptr::write_volatile(&mut h as *mut u32, (*ctx).si.h);
        ::core::ptr::write_volatile(
            &mut res as *mut vpx_codec_err_t,
            vp8_peek_si_internal(
                (*ctx).fragments.ptrs[0 as usize],
                (*ctx).fragments.sizes[0 as usize],
                &raw mut (*ctx).si,
                (*ctx).decrypt_cb,
                (*ctx).decrypt_state,
            ),
        );
        if res as u32 == VPX_CODEC_UNSUP_BITSTREAM as u32 && (*ctx).si.is_kf == 0 {
            ::core::ptr::write_volatile(&mut res as *mut vpx_codec_err_t, VPX_CODEC_OK);
        }
        if (*ctx).decoder_init == 0 && (*ctx).si.is_kf == 0 {
            ::core::ptr::write_volatile(
                &mut res as *mut vpx_codec_err_t,
                VPX_CODEC_UNSUP_BITSTREAM,
            );
        }
        if res as u64 == 0
            && (*ctx).decoder_init != 0
            && w == 0 as u32
            && h == 0 as u32
            && (*ctx).si.h == 0 as u32
            && (*ctx).si.w == 0 as u32
        {
            let mut pbi: *mut VP8D_COMP = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            ::core::ptr::write_volatile(&mut res as *mut vpx_codec_err_t, VPX_CODEC_CORRUPT_FRAME);
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                res,
                b"Keyframe / intra-only frame required to reset decoder state\0" as *const u8
                    as *const i8,
            );
        }
        if (*ctx).si.h != h || (*ctx).si.w != w {
            ::core::ptr::write_volatile(&mut resolution_change as *mut u32, 1 as u32);
        }
        if res as u64 == 0 && (*ctx).restart_threads != 0 {
            let mut pbi_0: *mut VP8D_COMP = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            let pc: *mut VP8_COMMON = &raw mut (*pbi_0).common;
            if setjmp(&raw mut (*pbi_0).common.error.jmp as *mut i32) != 0 {
                (*pbi_0).common.error.setjmp = 0 as i32;
                vp8_decoder_remove_threads(pbi_0);
                return VPX_CODEC_ERROR;
            }
            (*pbi_0).common.error.setjmp = 1 as i32;
            (*pbi_0).max_threads = (*ctx).cfg.threads as i32;
            vp8_decoder_create_threads(pbi_0);
            if vpx_atomic_load_acquire(&raw mut (*pbi_0).b_multithreaded_rd) != 0 {
                vp8mt_alloc_temp_buffers(pbi_0, (*pc).Width, (*pc).mb_rows);
            }
            (*ctx).restart_threads = 0 as i32;
            (*pbi_0).common.error.setjmp = 0 as i32;
        }
        if res as u64 == 0 && (*ctx).decoder_init == 0 {
            let mut oxcf: VP8D_CONFIG = VP8D_CONFIG {
                Width: 0,
                Height: 0,
                Version: 0,
                postprocess: 0,
                max_threads: 0,
                error_concealment: 0,
            };
            oxcf.Width = (*ctx).si.w as i32;
            oxcf.Height = (*ctx).si.h as i32;
            oxcf.Version = 9 as i32;
            oxcf.postprocess = 0 as i32;
            oxcf.max_threads = (*ctx).cfg.threads as i32;
            oxcf.error_concealment = ((*ctx).base.init_flags
                & VPX_CODEC_USE_ERROR_CONCEALMENT as vpx_codec_flags_t)
                as i32;
            if (*ctx).postproc_cfg_set == 0
                && (*ctx).base.init_flags & VPX_CODEC_USE_POSTPROC as vpx_codec_flags_t != 0
            {
                (*ctx).postproc_cfg.post_proc_flag =
                    VP8_DEBLOCK as i32 | VP8_DEMACROBLOCK as i32 | VP8_MFQE as i32;
                (*ctx).postproc_cfg.deblocking_level = 4 as i32;
                (*ctx).postproc_cfg.noise_level = 0 as i32;
            }
            ::core::ptr::write_volatile(
                &mut res as *mut vpx_codec_err_t,
                vp8_create_decoder_instances(&raw mut (*ctx).yv12_frame_buffers, &raw mut oxcf)
                    as vpx_codec_err_t,
            );
            if res as u32 == VPX_CODEC_OK as u32 {
                (*ctx).decoder_init = 1 as i32;
            } else {
                (*ctx).si.w = 0 as u32;
                (*ctx).si.h = 0 as u32;
            }
        }
        if (*ctx).decoder_init != 0 {
            (*(*ctx).yv12_frame_buffers.pbi[0 as usize]).decrypt_cb = (*ctx).decrypt_cb;
            (*(*ctx).yv12_frame_buffers.pbi[0 as usize]).decrypt_state = (*ctx).decrypt_state;
        }
        if res as u64 == 0 {
            let mut pbi_1: *mut VP8D_COMP = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            let pc_0: *mut VP8_COMMON = &raw mut (*pbi_1).common;
            if resolution_change != 0 {
                let xd: *mut MACROBLOCKD = &raw mut (*pbi_1).mb;
                let mut i: i32 = 0;
                (*pc_0).Width = (*ctx).si.w as i32;
                (*pc_0).Height = (*ctx).si.h as i32;
                if setjmp(&raw mut (*pbi_1).common.error.jmp as *mut i32) != 0 {
                    (*pbi_1).common.error.setjmp = 0 as i32;
                    (*ctx).si.w = 0 as u32;
                    (*ctx).si.h = 0 as u32;
                    return 4294967295 as vpx_codec_err_t;
                }
                (*pbi_1).common.error.setjmp = 1 as i32;
                if (*pc_0).Width <= 0 as i32 {
                    (*pc_0).Width = w as i32;
                    vpx_internal_error(
                        &raw mut (*pc_0).error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Invalid frame width\0" as *const u8 as *const i8,
                    );
                }
                if (*pc_0).Height <= 0 as i32 {
                    (*pc_0).Height = h as i32;
                    vpx_internal_error(
                        &raw mut (*pc_0).error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Invalid frame height\0" as *const u8 as *const i8,
                    );
                }
                if vpx_atomic_load_acquire(&raw mut (*pbi_1).b_multithreaded_rd) != 0 {
                    vp8mt_de_alloc_temp_buffers(pbi_1, (*pc_0).mb_rows);
                }
                if vp8_alloc_frame_buffers(pc_0, (*pc_0).Width, (*pc_0).Height) != 0 {
                    vpx_internal_error(
                        &raw mut (*pc_0).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate frame buffers\0" as *const u8 as *const i8,
                    );
                }
                (*xd).pre = (*pc_0).yv12_fb[(*pc_0).lst_fb_idx as usize];
                (*xd).dst = (*pc_0).yv12_fb[(*pc_0).new_fb_idx as usize];
                i = 0 as i32;
                while i < (*pbi_1).allocated_decoding_thread_count {
                    (*(*pbi_1).mb_row_di.offset(i as isize)).mbd.dst =
                        (*pc_0).yv12_fb[(*pc_0).new_fb_idx as usize];
                    vp8_build_block_doffsets(&raw mut (*(*pbi_1).mb_row_di.offset(i as isize)).mbd);
                    i += 1;
                }
                vp8_build_block_doffsets(&raw mut (*pbi_1).mb);
                if vpx_atomic_load_acquire(&raw mut (*pbi_1).b_multithreaded_rd) != 0 {
                    vp8mt_alloc_temp_buffers(pbi_1, (*pc_0).Width, 0 as i32);
                }
                (*pbi_1).common.error.setjmp = 0 as i32;
                (*pbi_1).common.fb_idx_ref_cnt[0 as usize] = 0 as i32;
            }
            if setjmp(&raw mut (*pbi_1).common.error.jmp as *mut i32) != 0 {
                (*pc_0).yv12_fb[(*pc_0).lst_fb_idx as usize].corrupted = 1 as i32;
                if (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] > 0 as i32 {
                    (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] -= 1;
                }
                (*pbi_1).common.error.setjmp = 0 as i32;
                if (*pbi_1).restart_threads != 0 {
                    (*ctx).si.w = 0 as u32;
                    (*ctx).si.h = 0 as u32;
                    (*ctx).restart_threads = 1 as i32;
                }
                ::core::ptr::write_volatile(
                    &mut res as *mut vpx_codec_err_t,
                    update_error_state(ctx, &raw mut (*pbi_1).common.error),
                );
                return res;
            }
            (*pbi_1).common.error.setjmp = 1 as i32;
            (*pbi_1).fragments = (*ctx).fragments;
            (*pbi_1).restart_threads = 0 as i32;
            (*ctx).user_priv = user_priv;
            if vp8dx_receive_compressed_data(pbi_1 as *mut VP8D_COMP) != 0 {
                ::core::ptr::write_volatile(
                    &mut res as *mut vpx_codec_err_t,
                    update_error_state(ctx, &raw mut (*pbi_1).common.error),
                );
            }
            (*ctx).fragments.count = 0 as u32;
            (*pbi_1).common.error.setjmp = 0 as i32;
        }
        res
    }
}
unsafe fn vp8_get_frame(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut iter: *mut vpx_codec_iter_t,
) -> *mut vpx_image_t {
    unsafe {
        let mut img: *mut vpx_image_t = ::core::ptr::null_mut::<vpx_image_t>();
        if (*iter).is_null() && !(*ctx).yv12_frame_buffers.pbi[0 as usize].is_null() {
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
                y_width: 0,
                y_height: 0,
                y_crop_width: 0,
                y_crop_height: 0,
                y_stride: 0,
                uv_width: 0,
                uv_height: 0,
                uv_crop_width: 0,
                uv_crop_height: 0,
                uv_stride: 0,
                alpha_width: 0,
                alpha_height: 0,
                alpha_stride: 0,
                y_buffer: ::core::ptr::null_mut::<uint8_t>(),
                u_buffer: ::core::ptr::null_mut::<uint8_t>(),
                v_buffer: ::core::ptr::null_mut::<uint8_t>(),
                alpha_buffer: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc_sz: 0,
                border: 0,
                frame_size: 0,
                subsampling_x: 0,
                subsampling_y: 0,
                bit_depth: 0,
                color_space: VPX_CS_UNKNOWN,
                color_range: VPX_CR_STUDIO_RANGE,
                render_width: 0,
                render_height: 0,
                corrupted: 0,
                flags: 0,
            };
            let mut flags: vp8_ppflags_t = vp8_ppflags_t {
                post_proc_flag: 0,
                deblocking_level: 0,
                noise_level: 0,
                display_ref_frame_flag: 0,
                display_mb_modes_flag: 0,
                display_b_modes_flag: 0,
                display_mv_flag: 0,
            };
            memset(
                &raw mut flags as *mut c_void,
                0 as i32,
                ::core::mem::size_of::<vp8_ppflags_t>() as size_t,
            );
            if (*ctx).base.init_flags & VPX_CODEC_USE_POSTPROC as vpx_codec_flags_t != 0 {
                flags.post_proc_flag = (*ctx).postproc_cfg.post_proc_flag;
                flags.deblocking_level = (*ctx).postproc_cfg.deblocking_level;
                flags.noise_level = (*ctx).postproc_cfg.noise_level;
            }
            if 0 as i32
                == vp8dx_get_raw_frame(
                    (*ctx).yv12_frame_buffers.pbi[0 as usize],
                    &raw mut sd,
                    &raw mut flags,
                )
            {
                yuvconfig2image(&raw mut (*ctx).img, &raw mut sd, (*ctx).user_priv);
                img = &raw mut (*ctx).img;
                *iter = img as vpx_codec_iter_t;
            }
        }
        img
    }
}
unsafe fn image2yuvconfig(
    mut img: *const vpx_image_t,
    mut yv12: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    unsafe {
        let y_w: i32 = (*img).d_w as i32;
        let y_h: i32 = (*img).d_h as i32;
        let uv_w: i32 = (*img).d_w.wrapping_add(1 as u32).wrapping_div(2 as u32) as i32;
        let uv_h: i32 = (*img).d_h.wrapping_add(1 as u32).wrapping_div(2 as u32) as i32;
        let mut res: vpx_codec_err_t = VPX_CODEC_OK;
        (*yv12).y_buffer = (*img).planes[VPX_PLANE_Y as usize] as *mut uint8_t;
        (*yv12).u_buffer = (*img).planes[VPX_PLANE_U as usize] as *mut uint8_t;
        (*yv12).v_buffer = (*img).planes[VPX_PLANE_V as usize] as *mut uint8_t;
        (*yv12).y_crop_width = y_w;
        (*yv12).y_crop_height = y_h;
        (*yv12).y_width = y_w;
        (*yv12).y_height = y_h;
        (*yv12).uv_crop_width = uv_w;
        (*yv12).uv_crop_height = uv_h;
        (*yv12).uv_width = uv_w;
        (*yv12).uv_height = uv_h;
        (*yv12).y_stride = (*img).stride[VPX_PLANE_Y as usize];
        (*yv12).uv_stride = (*img).stride[VPX_PLANE_U as usize];
        (*yv12).border = ((*img).stride[VPX_PLANE_Y as usize] as u32)
            .wrapping_sub((*img).d_w)
            .wrapping_div(2 as u32) as i32;
        res
    }
}
unsafe fn vp8_set_reference(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut data: *mut vpx_ref_frame_t = data as *mut vpx_ref_frame_t;
        if !data.is_null() {
            let mut frame: *mut vpx_ref_frame_t = data;
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
                y_width: 0,
                y_height: 0,
                y_crop_width: 0,
                y_crop_height: 0,
                y_stride: 0,
                uv_width: 0,
                uv_height: 0,
                uv_crop_width: 0,
                uv_crop_height: 0,
                uv_stride: 0,
                alpha_width: 0,
                alpha_height: 0,
                alpha_stride: 0,
                y_buffer: ::core::ptr::null_mut::<uint8_t>(),
                u_buffer: ::core::ptr::null_mut::<uint8_t>(),
                v_buffer: ::core::ptr::null_mut::<uint8_t>(),
                alpha_buffer: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc_sz: 0,
                border: 0,
                frame_size: 0,
                subsampling_x: 0,
                subsampling_y: 0,
                bit_depth: 0,
                color_space: VPX_CS_UNKNOWN,
                color_range: VPX_CR_STUDIO_RANGE,
                render_width: 0,
                render_height: 0,
                corrupted: 0,
                flags: 0,
            };
            image2yuvconfig(&raw mut (*frame).img, &raw mut sd);
            if (*ctx).yv12_frame_buffers.pbi[0 as usize].is_null() {
                return VPX_CODEC_CORRUPT_FRAME;
            }
            vp8dx_set_reference(
                (*ctx).yv12_frame_buffers.pbi[0 as usize],
                (*frame).frame_type as vpx_ref_frame_type,
                &raw mut sd,
            )
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_reference(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut data: *mut vpx_ref_frame_t = data as *mut vpx_ref_frame_t;
        if !data.is_null() {
            let mut frame: *mut vpx_ref_frame_t = data;
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
                y_width: 0,
                y_height: 0,
                y_crop_width: 0,
                y_crop_height: 0,
                y_stride: 0,
                uv_width: 0,
                uv_height: 0,
                uv_crop_width: 0,
                uv_crop_height: 0,
                uv_stride: 0,
                alpha_width: 0,
                alpha_height: 0,
                alpha_stride: 0,
                y_buffer: ::core::ptr::null_mut::<uint8_t>(),
                u_buffer: ::core::ptr::null_mut::<uint8_t>(),
                v_buffer: ::core::ptr::null_mut::<uint8_t>(),
                alpha_buffer: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc: ::core::ptr::null_mut::<uint8_t>(),
                buffer_alloc_sz: 0,
                border: 0,
                frame_size: 0,
                subsampling_x: 0,
                subsampling_y: 0,
                bit_depth: 0,
                color_space: VPX_CS_UNKNOWN,
                color_range: VPX_CR_STUDIO_RANGE,
                render_width: 0,
                render_height: 0,
                corrupted: 0,
                flags: 0,
            };
            image2yuvconfig(&raw mut (*frame).img, &raw mut sd);
            if (*ctx).yv12_frame_buffers.pbi[0 as usize].is_null() {
                return VPX_CODEC_CORRUPT_FRAME;
            }
            vp8dx_get_reference(
                (*ctx).yv12_frame_buffers.pbi[0 as usize],
                (*frame).frame_type as vpx_ref_frame_type,
                &raw mut sd,
            )
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_quantizer(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let arg: *mut i32 = data as *mut i32;
        let mut pbi: *mut VP8D_COMP = (*ctx).yv12_frame_buffers.pbi[0 as usize];
        if arg.is_null() {
            return VPX_CODEC_INVALID_PARAM;
        }
        if pbi.is_null() {
            return VPX_CODEC_CORRUPT_FRAME;
        }
        *arg = vp8dx_get_quantizer(pbi);
        VPX_CODEC_OK
    }
}
unsafe fn vp8_set_postproc(_ctx: *mut vpx_codec_alg_priv_t, _args: *mut c_void) -> vpx_codec_err_t {
    VPX_CODEC_INCAPABLE
}
unsafe fn vp8_get_last_ref_updates(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut update_info: *mut i32 = data as *mut i32;
        if !update_info.is_null() {
            let mut pbi: *mut VP8D_COMP =
                (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut VP8D_COMP;
            if pbi.is_null() {
                return VPX_CODEC_CORRUPT_FRAME;
            }
            *update_info = (*pbi).common.refresh_alt_ref_frame * VP8_ALTR_FRAME as i32
                + (*pbi).common.refresh_golden_frame * VP8_GOLD_FRAME as i32
                + (*pbi).common.refresh_last_frame * VP8_LAST_FRAME as i32;
            VPX_CODEC_OK
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_last_ref_frame(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut ref_info: *mut i32 = data as *mut i32;
        if !ref_info.is_null() {
            let mut pbi: *mut VP8D_COMP =
                (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut VP8D_COMP;
            if !pbi.is_null() {
                let mut oci: *mut VP8_COMMON = &raw mut (*pbi).common;
                *ref_info =
                    (if vp8dx_references_buffer(oci as *mut VP8Common, ALTREF_FRAME as i32) != 0 {
                        VP8_ALTR_FRAME as i32
                    } else {
                        0 as i32
                    }) | (if vp8dx_references_buffer(oci as *mut VP8Common, GOLDEN_FRAME as i32)
                        != 0
                    {
                        VP8_GOLD_FRAME as i32
                    } else {
                        0 as i32
                    }) | (if vp8dx_references_buffer(oci as *mut VP8Common, LAST_FRAME as i32) != 0
                    {
                        VP8_LAST_FRAME as i32
                    } else {
                        0 as i32
                    });
                VPX_CODEC_OK
            } else {
                VPX_CODEC_CORRUPT_FRAME
            }
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_frame_corrupted(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut corrupted: *mut i32 = data as *mut i32;
        let mut pbi: *mut VP8D_COMP = (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut VP8D_COMP;
        if !corrupted.is_null() && !pbi.is_null() {
            let frame: *const YV12_BUFFER_CONFIG = (*pbi).common.frame_to_show;
            if frame.is_null() {
                return VPX_CODEC_ERROR;
            }
            *corrupted = (*frame).corrupted;
            VPX_CODEC_OK
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_set_decryptor(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *mut c_void,
) -> vpx_codec_err_t {
    unsafe {
        let mut init: *mut vpx_decrypt_init = data as *mut vpx_decrypt_init;
        if !init.is_null() {
            (*ctx).decrypt_cb = (*init).decrypt_cb;
            (*ctx).decrypt_state = (*init).decrypt_state;
        } else {
            (*ctx).decrypt_cb = None;
            (*ctx).decrypt_state = NULL;
        }
        VPX_CODEC_OK
    }
}
static mut vp8_ctf_maps: [vpx_codec_ctrl_fn_map_t; 9] = {
    [
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8_SET_REFERENCE as i32,
            fn_0: Some(
                vp8_set_reference
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8_COPY_REFERENCE as i32,
            fn_0: Some(
                vp8_get_reference
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8_SET_POSTPROC as i32,
            fn_0: Some(
                vp8_set_postproc
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8D_GET_LAST_REF_UPDATES as i32,
            fn_0: Some(
                vp8_get_last_ref_updates
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8D_GET_FRAME_CORRUPTED as i32,
            fn_0: Some(
                vp8_get_frame_corrupted
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VP8D_GET_LAST_REF_USED as i32,
            fn_0: Some(
                vp8_get_last_ref_frame
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VPXD_GET_LAST_QUANTIZER as i32,
            fn_0: Some(
                vp8_get_quantizer
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: VPXD_SET_DECRYPTOR as i32,
            fn_0: Some(
                vp8_set_decryptor
                    as unsafe fn(*mut vpx_codec_alg_priv_t, *mut c_void) -> vpx_codec_err_t,
            ),
        },
        vpx_codec_ctrl_fn_map {
            ctrl_id: -(1 as i32),
            fn_0: None,
        },
    ]
};
#[unsafe(no_mangle)]
pub static mut vpx_codec_vp8_dx_algo: vpx_codec_iface_t = vpx_codec_iface {
    name: ::core::ptr::null::<i8>(),
    abi_version: 0,
    caps: 0,
    init: None,
    destroy: None,
    ctrl_maps: ::core::ptr::null::<vpx_codec_ctrl_fn_map_t>(),
    dec: vpx_codec_dec_iface {
        peek_si: None,
        get_si: None,
        decode: None,
        get_frame: None,
        set_fb_fn: None,
    },
    enc: vpx_codec_enc_iface {
        cfg_map_count: 0,
        cfg_maps: ::core::ptr::null::<vpx_codec_enc_cfg_map_t>(),
        encode: None,
        get_cx_data: None,
        cfg_set: None,
        get_glob_hdrs: None,
        get_preview: None,
        mr_get_mem_loc: None,
        mr_free_mem_loc: None,
    },
};
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_vp8_dx() -> *const vpx_codec_iface_t {
    &raw const vpx_codec_vp8_dx_algo
}
pub const __ATOMIC_ACQUIRE: i32 = 2 as i32;
pub const NULL: *mut c_void = __DARWIN_NULL;
unsafe fn run_static_initializers() {
    unsafe {
        vpx_codec_vp8_dx_algo = vpx_codec_iface {
            name: b"WebM Project VP8 Decoder v1.16.0-122-ge9efe034e\0" as *const u8 as *const i8,
            abi_version: VPX_CODEC_INTERNAL_ABI_VERSION,
            caps: (VPX_CODEC_CAP_DECODER
                | (if CONFIG_POSTPROC != 0 {
                    VPX_CODEC_CAP_POSTPROC
                } else {
                    0 as i32
                })
                | (if CONFIG_ERROR_CONCEALMENT != 0 {
                    VPX_CODEC_CAP_ERROR_CONCEALMENT
                } else {
                    0 as i32
                })
                | VPX_CODEC_CAP_INPUT_FRAGMENTS) as vpx_codec_caps_t,
            init: Some(
                vp8_init
                    as unsafe fn(
                        *mut vpx_codec_ctx_t,
                        *mut vpx_codec_priv_enc_mr_cfg_t,
                    ) -> vpx_codec_err_t,
            ),
            destroy: Some(vp8_destroy as unsafe fn(*mut vpx_codec_alg_priv_t) -> vpx_codec_err_t),
            ctrl_maps: &raw const vp8_ctf_maps as *const vpx_codec_ctrl_fn_map_t,
            dec: vpx_codec_dec_iface {
                peek_si: Some(
                    vp8_peek_si
                        as unsafe fn(
                            *const uint8_t,
                            u32,
                            *mut vpx_codec_stream_info_t,
                        ) -> vpx_codec_err_t,
                ),
                get_si: Some(
                    vp8_get_si
                        as unsafe fn(
                            *mut vpx_codec_alg_priv_t,
                            *mut vpx_codec_stream_info_t,
                        ) -> vpx_codec_err_t,
                ),
                decode: Some(
                    vp8_decode
                        as unsafe fn(
                            *mut vpx_codec_alg_priv_t,
                            *const uint8_t,
                            u32,
                            *mut c_void,
                        ) -> vpx_codec_err_t,
                ),
                get_frame: Some(
                    vp8_get_frame
                        as unsafe fn(
                            *mut vpx_codec_alg_priv_t,
                            *mut vpx_codec_iter_t,
                        ) -> *mut vpx_image_t,
                ),
                set_fb_fn: None,
            },
            enc: vpx_codec_enc_iface {
                cfg_map_count: 0 as i32,
                cfg_maps: ::core::ptr::null::<vpx_codec_enc_cfg_map_t>(),
                encode: None,
                get_cx_data: None,
                cfg_set: None,
                get_glob_hdrs: None,
                get_preview: None,
                mr_get_mem_loc: None,
                mr_free_mem_loc: None,
            },
        };
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XCU"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
