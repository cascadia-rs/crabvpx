extern "C" {
    pub type vpx_codec_alg_priv;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vpx_codec_destroy(ctx: *mut vpx_codec_ctx_t) -> vpx_codec_err_t;
}
pub type __builtin_va_list = *mut ::core::ffi::c_char;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type vpx_img_fmt = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_image {
    pub fmt: vpx_img_fmt_t,
    pub cs: vpx_color_space_t,
    pub range: vpx_color_range_t,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
    pub bit_depth: ::core::ffi::c_uint,
    pub d_w: ::core::ffi::c_uint,
    pub d_h: ::core::ffi::c_uint,
    pub r_w: ::core::ffi::c_uint,
    pub r_h: ::core::ffi::c_uint,
    pub x_chroma_shift: ::core::ffi::c_uint,
    pub y_chroma_shift: ::core::ffi::c_uint,
    pub planes: [*mut ::core::ffi::c_uchar; 4],
    pub stride: [::core::ffi::c_int; 4],
    pub bps: ::core::ffi::c_int,
    pub user_priv: *mut ::core::ffi::c_void,
    pub img_data: *mut ::core::ffi::c_uchar,
    pub img_data_owner: ::core::ffi::c_int,
    pub self_allocd: ::core::ffi::c_int,
    pub fb_priv: *mut ::core::ffi::c_void,
}
pub type vpx_image_t = vpx_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_image_rect {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
}
pub type vpx_image_rect_t = vpx_image_rect;
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
pub type vpx_codec_caps_t = ::core::ffi::c_long;
pub type vpx_codec_flags_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_iface {
    pub name: *const ::core::ffi::c_char,
    pub abi_version: ::core::ffi::c_int,
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
    pub cfg_map_count: ::core::ffi::c_int,
    pub cfg_maps: *const vpx_codec_enc_cfg_map_t,
    pub encode: vpx_codec_encode_fn_t,
    pub get_cx_data: vpx_codec_get_cx_data_fn_t,
    pub cfg_set: vpx_codec_enc_config_set_fn_t,
    pub get_glob_hdrs: vpx_codec_get_global_headers_fn_t,
    pub get_preview: vpx_codec_get_preview_frame_fn_t,
    pub mr_get_mem_loc: vpx_codec_enc_mr_get_mem_loc_fn_t,
    pub mr_free_mem_loc: vpx_codec_enc_mr_free_mem_loc_fn_t,
}
pub type vpx_codec_enc_mr_free_mem_loc_fn_t =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type vpx_codec_enc_mr_get_mem_loc_fn_t = Option<
    unsafe extern "C" fn(
        *const vpx_codec_enc_cfg_t,
        *mut *mut ::core::ffi::c_void,
    ) -> vpx_codec_err_t,
>;
pub type vpx_codec_enc_cfg_t = vpx_codec_enc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_enc_cfg {
    pub g_usage: ::core::ffi::c_uint,
    pub g_threads: ::core::ffi::c_uint,
    pub g_profile: ::core::ffi::c_uint,
    pub g_w: ::core::ffi::c_uint,
    pub g_h: ::core::ffi::c_uint,
    pub g_bit_depth: vpx_bit_depth_t,
    pub g_input_bit_depth: ::core::ffi::c_uint,
    pub g_timebase: vpx_rational,
    pub g_error_resilient: vpx_codec_er_flags_t,
    pub g_pass: vpx_enc_pass,
    pub g_lag_in_frames: ::core::ffi::c_uint,
    pub rc_dropframe_thresh: ::core::ffi::c_uint,
    pub rc_resize_allowed: ::core::ffi::c_uint,
    pub rc_scaled_width: ::core::ffi::c_uint,
    pub rc_scaled_height: ::core::ffi::c_uint,
    pub rc_resize_up_thresh: ::core::ffi::c_uint,
    pub rc_resize_down_thresh: ::core::ffi::c_uint,
    pub rc_end_usage: vpx_rc_mode,
    pub rc_twopass_stats_in: vpx_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: vpx_fixed_buf_t,
    pub rc_target_bitrate: ::core::ffi::c_uint,
    pub rc_min_quantizer: ::core::ffi::c_uint,
    pub rc_max_quantizer: ::core::ffi::c_uint,
    pub rc_undershoot_pct: ::core::ffi::c_uint,
    pub rc_overshoot_pct: ::core::ffi::c_uint,
    pub rc_buf_sz: ::core::ffi::c_uint,
    pub rc_buf_initial_sz: ::core::ffi::c_uint,
    pub rc_buf_optimal_sz: ::core::ffi::c_uint,
    pub rc_2pass_vbr_bias_pct: ::core::ffi::c_uint,
    pub rc_2pass_vbr_minsection_pct: ::core::ffi::c_uint,
    pub rc_2pass_vbr_maxsection_pct: ::core::ffi::c_uint,
    pub rc_2pass_vbr_corpus_complexity: ::core::ffi::c_uint,
    pub kf_mode: vpx_kf_mode,
    pub kf_min_dist: ::core::ffi::c_uint,
    pub kf_max_dist: ::core::ffi::c_uint,
    pub ss_number_layers: ::core::ffi::c_uint,
    pub ss_enable_auto_alt_ref: [::core::ffi::c_int; 5],
    pub ss_target_bitrate: [::core::ffi::c_uint; 5],
    pub ts_number_layers: ::core::ffi::c_uint,
    pub ts_target_bitrate: [::core::ffi::c_uint; 5],
    pub ts_rate_decimator: [::core::ffi::c_uint; 5],
    pub ts_periodicity: ::core::ffi::c_uint,
    pub ts_layer_id: [::core::ffi::c_uint; 16],
    pub layer_target_bitrate: [::core::ffi::c_uint; 12],
    pub temporal_layering_mode: ::core::ffi::c_int,
    pub use_vizier_rc_params: ::core::ffi::c_int,
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
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type vpx_kf_mode = ::core::ffi::c_uint;
pub const VPX_KF_DISABLED: vpx_kf_mode = 0;
pub const VPX_KF_AUTO: vpx_kf_mode = 1;
pub const VPX_KF_FIXED: vpx_kf_mode = 0;
pub type vpx_fixed_buf_t = vpx_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_fixed_buf {
    pub buf: *mut ::core::ffi::c_void,
    pub sz: size_t,
}
pub type vpx_rc_mode = ::core::ffi::c_uint;
pub const VPX_Q: vpx_rc_mode = 3;
pub const VPX_CQ: vpx_rc_mode = 2;
pub const VPX_CBR: vpx_rc_mode = 1;
pub const VPX_VBR: vpx_rc_mode = 0;
pub type vpx_enc_pass = ::core::ffi::c_uint;
pub const VPX_RC_LAST_PASS: vpx_enc_pass = 2;
pub const VPX_RC_FIRST_PASS: vpx_enc_pass = 1;
pub const VPX_RC_ONE_PASS: vpx_enc_pass = 0;
pub type vpx_codec_er_flags_t = uint32_t;
pub type vpx_bit_depth_t = vpx_bit_depth;
pub type vpx_bit_depth = ::core::ffi::c_uint;
pub const VPX_BITS_12: vpx_bit_depth = 12;
pub const VPX_BITS_10: vpx_bit_depth = 10;
pub const VPX_BITS_8: vpx_bit_depth = 8;
pub type vpx_codec_get_preview_frame_fn_t =
    Option<unsafe extern "C" fn(*mut vpx_codec_alg_priv_t) -> *mut vpx_image_t>;
pub type vpx_codec_alg_priv_t = vpx_codec_alg_priv;
pub type vpx_codec_get_global_headers_fn_t =
    Option<unsafe extern "C" fn(*mut vpx_codec_alg_priv_t) -> *mut vpx_fixed_buf_t>;
pub type vpx_codec_enc_config_set_fn_t = Option<
    unsafe extern "C" fn(*mut vpx_codec_alg_priv_t, *const vpx_codec_enc_cfg_t) -> vpx_codec_err_t,
>;
pub type vpx_codec_get_cx_data_fn_t = Option<
    unsafe extern "C" fn(
        *mut vpx_codec_alg_priv_t,
        *mut vpx_codec_iter_t,
    ) -> *const vpx_codec_cx_pkt_t,
>;
pub type vpx_codec_iter_t = *const ::core::ffi::c_void;
pub type vpx_codec_cx_pkt_t = vpx_codec_cx_pkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_cx_pkt {
    pub kind: vpx_codec_cx_pkt_kind,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub frame: C2RustUnnamed_0,
    pub twopass_stats: vpx_fixed_buf_t,
    pub firstpass_mb_stats: vpx_fixed_buf_t,
    pub psnr: vpx_psnr_pkt,
    pub raw: vpx_fixed_buf_t,
    pub pad: [::core::ffi::c_char; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_psnr_pkt {
    pub samples: [::core::ffi::c_uint; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [::core::ffi::c_double; 4],
    pub spatial_layer_id: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub buf: *mut ::core::ffi::c_void,
    pub sz: size_t,
    pub pts: vpx_codec_pts_t,
    pub duration: ::core::ffi::c_ulong,
    pub flags: vpx_codec_frame_flags_t,
    pub partition_id: ::core::ffi::c_int,
    pub width: [::core::ffi::c_uint; 5],
    pub height: [::core::ffi::c_uint; 5],
    pub spatial_layer_encoded: [uint8_t; 5],
}
pub type vpx_codec_frame_flags_t = uint32_t;
pub type vpx_codec_pts_t = int64_t;
pub type vpx_codec_cx_pkt_kind = ::core::ffi::c_uint;
pub const VPX_CODEC_CUSTOM_PKT: vpx_codec_cx_pkt_kind = 256;
pub const VPX_CODEC_PSNR_PKT: vpx_codec_cx_pkt_kind = 3;
pub const VPX_CODEC_FPMB_STATS_PKT: vpx_codec_cx_pkt_kind = 2;
pub const VPX_CODEC_STATS_PKT: vpx_codec_cx_pkt_kind = 1;
pub const VPX_CODEC_CX_FRAME_PKT: vpx_codec_cx_pkt_kind = 0;
pub type vpx_codec_encode_fn_t = Option<
    unsafe extern "C" fn(
        *mut vpx_codec_alg_priv_t,
        *const vpx_image_t,
        vpx_codec_pts_t,
        ::core::ffi::c_ulong,
        vpx_enc_frame_flags_t,
        vpx_enc_deadline_t,
    ) -> vpx_codec_err_t,
>;
pub type vpx_enc_deadline_t = ::core::ffi::c_ulong;
pub type vpx_enc_frame_flags_t = ::core::ffi::c_long;
pub type vpx_codec_enc_cfg_map_t = vpx_codec_enc_cfg_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_enc_cfg_map {
    pub usage: ::core::ffi::c_int,
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
    unsafe extern "C" fn(
        *mut vpx_codec_alg_priv_t,
        vpx_get_frame_buffer_cb_fn_t,
        vpx_release_frame_buffer_cb_fn_t,
        *mut ::core::ffi::c_void,
    ) -> vpx_codec_err_t,
>;
pub type vpx_release_frame_buffer_cb_fn_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut vpx_codec_frame_buffer_t,
    ) -> ::core::ffi::c_int,
>;
pub type vpx_codec_frame_buffer_t = vpx_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut ::core::ffi::c_void,
}
pub type vpx_get_frame_buffer_cb_fn_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        size_t,
        *mut vpx_codec_frame_buffer_t,
    ) -> ::core::ffi::c_int,
>;
pub type vpx_codec_get_frame_fn_t = Option<
    unsafe extern "C" fn(*mut vpx_codec_alg_priv_t, *mut vpx_codec_iter_t) -> *mut vpx_image_t,
>;
pub type vpx_codec_decode_fn_t = Option<
    unsafe extern "C" fn(
        *mut vpx_codec_alg_priv_t,
        *const uint8_t,
        ::core::ffi::c_uint,
        *mut ::core::ffi::c_void,
    ) -> vpx_codec_err_t,
>;
pub type vpx_codec_get_si_fn_t = Option<
    unsafe extern "C" fn(
        *mut vpx_codec_alg_priv_t,
        *mut vpx_codec_stream_info_t,
    ) -> vpx_codec_err_t,
>;
pub type vpx_codec_stream_info_t = vpx_codec_stream_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_stream_info {
    pub sz: ::core::ffi::c_uint,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
    pub is_kf: ::core::ffi::c_uint,
}
pub type vpx_codec_peek_si_fn_t = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_uint,
        *mut vpx_codec_stream_info_t,
    ) -> vpx_codec_err_t,
>;
pub type vpx_codec_ctrl_fn_map_t = vpx_codec_ctrl_fn_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_ctrl_fn_map {
    pub ctrl_id: ::core::ffi::c_int,
    pub fn_0: vpx_codec_control_fn_t,
}
pub type vpx_codec_control_fn_t =
    Option<unsafe extern "C" fn(*mut vpx_codec_alg_priv_t, ::core::ffi::VaList) -> vpx_codec_err_t>;
pub type va_list = __builtin_va_list;
pub type vpx_codec_destroy_fn_t =
    Option<unsafe extern "C" fn(*mut vpx_codec_alg_priv_t) -> vpx_codec_err_t>;
pub type vpx_codec_init_fn_t = Option<
    unsafe extern "C" fn(*mut vpx_codec_ctx_t, *mut vpx_codec_priv_enc_mr_cfg_t) -> vpx_codec_err_t,
>;
pub type vpx_codec_priv_enc_mr_cfg_t = vpx_codec_priv_enc_mr_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv_enc_mr_cfg {
    pub mr_total_resolutions: ::core::ffi::c_uint,
    pub mr_encoder_id: ::core::ffi::c_uint,
    pub mr_down_sampling_factor: vpx_rational,
    pub mr_low_res_mode_info: *mut ::core::ffi::c_void,
}
pub type vpx_codec_ctx_t = vpx_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_ctx {
    pub name: *const ::core::ffi::c_char,
    pub iface: *const vpx_codec_iface_t,
    pub err: vpx_codec_err_t,
    pub err_detail: *const ::core::ffi::c_char,
    pub init_flags: vpx_codec_flags_t,
    pub config: C2RustUnnamed_4,
    pub priv_0: *mut vpx_codec_priv_t,
}
pub type vpx_codec_priv_t = vpx_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv {
    pub err_detail: *const ::core::ffi::c_char,
    pub init_flags: vpx_codec_flags_t,
    pub dec: C2RustUnnamed_2,
    pub enc: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub cx_data_dst_buf: vpx_fixed_buf_t,
    pub cx_data_pad_before: ::core::ffi::c_uint,
    pub cx_data_pad_after: ::core::ffi::c_uint,
    pub cx_data_pkt: vpx_codec_cx_pkt_t,
    pub total_encoders: ::core::ffi::c_uint,
}
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
    pub user_priv: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub put_frame: vpx_codec_put_frame_cb_fn_t,
    pub put_slice: vpx_codec_put_slice_cb_fn_t,
}
pub type vpx_codec_put_slice_cb_fn_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const vpx_image_t,
        *const vpx_image_rect_t,
        *const vpx_image_rect_t,
    ) -> (),
>;
pub type vpx_codec_put_frame_cb_fn_t =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const vpx_image_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub dec: *const vpx_codec_dec_cfg,
    pub enc: *const vpx_codec_enc_cfg,
    pub raw: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_dec_cfg {
    pub threads: ::core::ffi::c_uint,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
}
pub type vpx_codec_iface_t = vpx_codec_iface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_pkt_list {
    pub cnt: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub pkts: [vpx_codec_cx_pkt; 1],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const VPX_IMAGE_ABI_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const VPX_CODEC_ABI_VERSION: ::core::ffi::c_int =
    4 as ::core::ffi::c_int + VPX_IMAGE_ABI_VERSION;
pub const VPX_CODEC_CAP_ENCODER: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const VPX_TPL_ABI_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const VPX_EXT_RATECTRL_ABI_VERSION: ::core::ffi::c_int =
    7 as ::core::ffi::c_int + VPX_TPL_ABI_VERSION;
pub const VPX_ENCODER_ABI_VERSION: ::core::ffi::c_int =
    18 as ::core::ffi::c_int + VPX_CODEC_ABI_VERSION + VPX_EXT_RATECTRL_ABI_VERSION;
pub const VPX_CODEC_CAP_PSNR: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const VPX_CODEC_CAP_OUTPUT_PARTITION: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const VPX_CODEC_USE_PSNR: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const VPX_CODEC_USE_OUTPUT_PARTITION: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const VPX_CODEC_INTERNAL_ABI_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn get_alg_priv(mut ctx: *mut vpx_codec_ctx_t) -> *mut vpx_codec_alg_priv_t {
    return (*ctx).priv_0 as *mut vpx_codec_alg_priv_t;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_enc_init_ver(
    mut ctx: *mut vpx_codec_ctx_t,
    mut iface: *const vpx_codec_iface_t,
    mut cfg: *const vpx_codec_enc_cfg_t,
    mut flags: vpx_codec_flags_t,
    mut ver: ::core::ffi::c_int,
) -> vpx_codec_err_t {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    if ver != VPX_ENCODER_ABI_VERSION {
        res = VPX_CODEC_ABI_MISMATCH;
    } else if ctx.is_null() || iface.is_null() || cfg.is_null() {
        res = VPX_CODEC_INVALID_PARAM;
    } else if (*iface).abi_version != VPX_CODEC_INTERNAL_ABI_VERSION {
        res = VPX_CODEC_ABI_MISMATCH;
    } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
        res = VPX_CODEC_INCAPABLE;
    } else if flags & VPX_CODEC_USE_PSNR as vpx_codec_flags_t != 0
        && (*iface).caps & VPX_CODEC_CAP_PSNR as vpx_codec_caps_t == 0
    {
        res = VPX_CODEC_INCAPABLE;
    } else if flags & VPX_CODEC_USE_OUTPUT_PARTITION as vpx_codec_flags_t != 0
        && (*iface).caps & VPX_CODEC_CAP_OUTPUT_PARTITION as vpx_codec_caps_t == 0
    {
        res = VPX_CODEC_INCAPABLE;
    } else {
        (*ctx).iface = iface;
        (*ctx).name = (*iface).name;
        (*ctx).priv_0 = ::core::ptr::null_mut::<vpx_codec_priv_t>();
        (*ctx).init_flags = flags;
        (*ctx).config.enc = cfg as *const vpx_codec_enc_cfg;
        res = (*(*ctx).iface).init.expect("non-null function pointer")(
            ctx,
            ::core::ptr::null_mut::<vpx_codec_priv_enc_mr_cfg_t>(),
        );
        if res as u64 != 0 {
            (*ctx).err_detail = if !(*ctx).priv_0.is_null() {
                (*(*ctx).priv_0).err_detail
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
            vpx_codec_destroy(ctx);
        }
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as ::core::ffi::c_uint
    } else {
        res as ::core::ffi::c_uint
    }) as vpx_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_enc_init_multi_ver(
    mut ctx: *mut vpx_codec_ctx_t,
    mut iface: *const vpx_codec_iface_t,
    mut cfg: *const vpx_codec_enc_cfg_t,
    mut num_enc: ::core::ffi::c_int,
    mut flags: vpx_codec_flags_t,
    mut dsf: *const vpx_rational_t,
    mut ver: ::core::ffi::c_int,
) -> vpx_codec_err_t {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    if ver != VPX_ENCODER_ABI_VERSION {
        res = VPX_CODEC_ABI_MISMATCH;
    } else if ctx.is_null()
        || iface.is_null()
        || cfg.is_null()
        || (num_enc > 16 as ::core::ffi::c_int || num_enc < 1 as ::core::ffi::c_int)
        || dsf.is_null()
    {
        res = VPX_CODEC_INVALID_PARAM;
    } else if (*iface).abi_version != VPX_CODEC_INTERNAL_ABI_VERSION {
        res = VPX_CODEC_ABI_MISMATCH;
    } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
        res = VPX_CODEC_INCAPABLE;
    } else if flags & VPX_CODEC_USE_PSNR as vpx_codec_flags_t != 0
        && (*iface).caps & VPX_CODEC_CAP_PSNR as vpx_codec_caps_t == 0
    {
        res = VPX_CODEC_INCAPABLE;
    } else if flags & VPX_CODEC_USE_OUTPUT_PARTITION as vpx_codec_flags_t != 0
        && (*iface).caps & VPX_CODEC_CAP_OUTPUT_PARTITION as vpx_codec_caps_t == 0
    {
        res = VPX_CODEC_INCAPABLE;
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut mem_loc: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*iface).enc.mr_get_mem_loc.is_none() {
            return VPX_CODEC_INCAPABLE;
        }
        res = (*iface)
            .enc
            .mr_get_mem_loc
            .expect("non-null function pointer")(cfg, &raw mut mem_loc);
        if res as u64 == 0 {
            i = 0 as ::core::ffi::c_int;
            while i < num_enc {
                let mut mr_cfg: vpx_codec_priv_enc_mr_cfg_t = vpx_codec_priv_enc_mr_cfg {
                    mr_total_resolutions: 0,
                    mr_encoder_id: 0,
                    mr_down_sampling_factor: vpx_rational { num: 0, den: 0 },
                    mr_low_res_mode_info: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                };
                if (*dsf).num < 1 as ::core::ffi::c_int
                    || (*dsf).num > 4096 as ::core::ffi::c_int
                    || (*dsf).den < 1 as ::core::ffi::c_int
                    || (*dsf).den > (*dsf).num
                {
                    res = VPX_CODEC_INVALID_PARAM;
                } else {
                    mr_cfg.mr_low_res_mode_info = mem_loc;
                    mr_cfg.mr_total_resolutions = num_enc as ::core::ffi::c_uint;
                    mr_cfg.mr_encoder_id =
                        (num_enc - 1 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
                    mr_cfg.mr_down_sampling_factor = *dsf as vpx_rational;
                    (*ctx).iface = iface;
                    (*ctx).name = (*iface).name;
                    (*ctx).priv_0 = ::core::ptr::null_mut::<vpx_codec_priv_t>();
                    (*ctx).init_flags = flags;
                    (*ctx).config.enc = cfg as *const vpx_codec_enc_cfg;
                    res = (*(*ctx).iface).init.expect("non-null function pointer")(
                        ctx,
                        &raw mut mr_cfg,
                    );
                }
                if res as u64 != 0 {
                    let mut error_detail: *const ::core::ffi::c_char = if !(*ctx).priv_0.is_null() {
                        (*(*ctx).priv_0).err_detail
                    } else {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    };
                    (*ctx).err_detail = error_detail;
                    vpx_codec_destroy(ctx);
                    while i != 0 {
                        ctx = ctx.offset(-1);
                        (*ctx).err_detail = error_detail;
                        vpx_codec_destroy(ctx);
                        i -= 1;
                    }
                    return (if !ctx.is_null() {
                        (*ctx).err = res;
                        (*ctx).err as ::core::ffi::c_uint
                    } else {
                        res as ::core::ffi::c_uint
                    }) as vpx_codec_err_t;
                }
                ctx = ctx.offset(1);
                cfg = cfg.offset(1);
                dsf = dsf.offset(1);
                i += 1;
            }
            ctx = ctx.offset(-1);
        }
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as ::core::ffi::c_uint
    } else {
        res as ::core::ffi::c_uint
    }) as vpx_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_enc_config_default(
    mut iface: *const vpx_codec_iface_t,
    mut cfg: *mut vpx_codec_enc_cfg_t,
    mut usage: ::core::ffi::c_uint,
) -> vpx_codec_err_t {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    if iface.is_null() || cfg.is_null() || usage != 0 as ::core::ffi::c_uint {
        res = VPX_CODEC_INVALID_PARAM;
    } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
        res = VPX_CODEC_INCAPABLE;
    } else {
        *cfg = (*(*iface).enc.cfg_maps).cfg;
        res = VPX_CODEC_OK;
    }
    return res;
}
unsafe extern "C" fn FLOATING_POINT_INIT() {}
unsafe extern "C" fn FLOATING_POINT_RESTORE() {}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_encode(
    mut ctx: *mut vpx_codec_ctx_t,
    mut img: *const vpx_image_t,
    mut pts: vpx_codec_pts_t,
    mut duration: ::core::ffi::c_ulong,
    mut flags: vpx_enc_frame_flags_t,
    mut deadline: vpx_enc_deadline_t,
) -> vpx_codec_err_t {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    if ctx.is_null() || !img.is_null() && duration == 0 {
        res = VPX_CODEC_INVALID_PARAM;
    } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
        res = VPX_CODEC_ERROR;
    } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
        res = VPX_CODEC_INCAPABLE;
    } else if duration > UINT32_MAX as ::core::ffi::c_ulong
        || deadline > UINT32_MAX as vpx_enc_deadline_t
    {
        res = VPX_CODEC_INVALID_PARAM;
    } else {
        let mut num_enc: ::core::ffi::c_uint = (*(*ctx).priv_0).enc.total_encoders;
        FLOATING_POINT_INIT();
        if num_enc == 1 as ::core::ffi::c_uint {
            res = (*(*ctx).iface)
                .enc
                .encode
                .expect("non-null function pointer")(
                get_alg_priv(ctx),
                img,
                pts,
                duration,
                flags,
                deadline,
            );
        } else {
            let mut i: ::core::ffi::c_int = 0;
            ctx = ctx.offset(num_enc.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            if !img.is_null() {
                img = img.offset(num_enc.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            }
            i = num_enc.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                res = (*(*ctx).iface)
                    .enc
                    .encode
                    .expect("non-null function pointer")(
                    get_alg_priv(ctx),
                    img,
                    pts,
                    duration,
                    flags,
                    deadline,
                );
                if res as u64 != 0 {
                    break;
                }
                ctx = ctx.offset(-1);
                if !img.is_null() {
                    img = img.offset(-1);
                }
                i -= 1;
            }
            ctx = ctx.offset(1);
        }
        FLOATING_POINT_RESTORE();
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as ::core::ffi::c_uint
    } else {
        res as ::core::ffi::c_uint
    }) as vpx_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_get_cx_data(
    mut ctx: *mut vpx_codec_ctx_t,
    mut iter: *mut vpx_codec_iter_t,
) -> *const vpx_codec_cx_pkt_t {
    let mut pkt: *const vpx_codec_cx_pkt_t = ::core::ptr::null::<vpx_codec_cx_pkt_t>();
    if !ctx.is_null() {
        if iter.is_null() {
            (*ctx).err = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            (*ctx).err = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
            (*ctx).err = VPX_CODEC_INCAPABLE;
        } else {
            pkt = (*(*ctx).iface)
                .enc
                .get_cx_data
                .expect("non-null function pointer")(get_alg_priv(ctx), iter);
        }
    }
    if !pkt.is_null()
        && (*pkt).kind as ::core::ffi::c_uint
            == VPX_CODEC_CX_FRAME_PKT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let priv_0: *mut vpx_codec_priv_t = (*ctx).priv_0;
        let dst_buf: *mut ::core::ffi::c_char =
            (*priv_0).enc.cx_data_dst_buf.buf as *mut ::core::ffi::c_char;
        if !dst_buf.is_null()
            && (*pkt).data.raw.buf != dst_buf as *mut ::core::ffi::c_void
            && (*pkt)
                .data
                .raw
                .sz
                .wrapping_add((*priv_0).enc.cx_data_pad_before as size_t)
                .wrapping_add((*priv_0).enc.cx_data_pad_after as size_t)
                <= (*priv_0).enc.cx_data_dst_buf.sz
        {
            let mut modified_pkt: *mut vpx_codec_cx_pkt_t = &raw mut (*priv_0).enc.cx_data_pkt;
            memcpy(
                dst_buf.offset((*priv_0).enc.cx_data_pad_before as isize)
                    as *mut ::core::ffi::c_void,
                (*pkt).data.raw.buf,
                (*pkt).data.raw.sz,
            );
            *modified_pkt = *pkt;
            (*modified_pkt).data.raw.buf = dst_buf as *mut ::core::ffi::c_void;
            (*modified_pkt).data.raw.sz = (*modified_pkt).data.raw.sz.wrapping_add(
                (*priv_0)
                    .enc
                    .cx_data_pad_before
                    .wrapping_add((*priv_0).enc.cx_data_pad_after) as size_t,
            );
            pkt = modified_pkt;
        }
        if dst_buf == (*pkt).data.raw.buf as *mut ::core::ffi::c_char {
            (*priv_0).enc.cx_data_dst_buf.buf =
                dst_buf.offset((*pkt).data.raw.sz as isize) as *mut ::core::ffi::c_void;
            (*priv_0).enc.cx_data_dst_buf.sz = (*priv_0)
                .enc
                .cx_data_dst_buf
                .sz
                .wrapping_sub((*pkt).data.raw.sz);
        }
    }
    return pkt;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_set_cx_data_buf(
    mut ctx: *mut vpx_codec_ctx_t,
    mut buf: *const vpx_fixed_buf_t,
    mut pad_before: ::core::ffi::c_uint,
    mut pad_after: ::core::ffi::c_uint,
) -> vpx_codec_err_t {
    if ctx.is_null() || (*ctx).priv_0.is_null() {
        return VPX_CODEC_INVALID_PARAM;
    }
    if !buf.is_null() {
        (*(*ctx).priv_0).enc.cx_data_dst_buf = *buf;
        (*(*ctx).priv_0).enc.cx_data_pad_before = pad_before;
        (*(*ctx).priv_0).enc.cx_data_pad_after = pad_after;
    } else {
        (*(*ctx).priv_0).enc.cx_data_dst_buf.buf = NULL;
        (*(*ctx).priv_0).enc.cx_data_dst_buf.sz = 0 as size_t;
        (*(*ctx).priv_0).enc.cx_data_pad_before = 0 as ::core::ffi::c_uint;
        (*(*ctx).priv_0).enc.cx_data_pad_after = 0 as ::core::ffi::c_uint;
    }
    return VPX_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_get_preview_frame(
    mut ctx: *mut vpx_codec_ctx_t,
) -> *const vpx_image_t {
    let mut img: *mut vpx_image_t = ::core::ptr::null_mut::<vpx_image_t>();
    if !ctx.is_null() {
        if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            (*ctx).err = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
            (*ctx).err = VPX_CODEC_INCAPABLE;
        } else if (*(*ctx).iface).enc.get_preview.is_none() {
            (*ctx).err = VPX_CODEC_INCAPABLE;
        } else {
            img = (*(*ctx).iface)
                .enc
                .get_preview
                .expect("non-null function pointer")(get_alg_priv(ctx));
        }
    }
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_get_global_headers(
    mut ctx: *mut vpx_codec_ctx_t,
) -> *mut vpx_fixed_buf_t {
    let mut buf: *mut vpx_fixed_buf_t = ::core::ptr::null_mut::<vpx_fixed_buf_t>();
    if !ctx.is_null() {
        if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            (*ctx).err = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
            (*ctx).err = VPX_CODEC_INCAPABLE;
        } else if (*(*ctx).iface).enc.get_glob_hdrs.is_none() {
            (*ctx).err = VPX_CODEC_INCAPABLE;
        } else {
            buf = (*(*ctx).iface)
                .enc
                .get_glob_hdrs
                .expect("non-null function pointer")(get_alg_priv(ctx));
        }
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_enc_config_set(
    mut ctx: *mut vpx_codec_ctx_t,
    mut cfg: *const vpx_codec_enc_cfg_t,
) -> vpx_codec_err_t {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    if ctx.is_null() || (*ctx).iface.is_null() || (*ctx).priv_0.is_null() || cfg.is_null() {
        res = VPX_CODEC_INVALID_PARAM;
    } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as vpx_codec_caps_t == 0 {
        res = VPX_CODEC_INCAPABLE;
    } else {
        res = (*(*ctx).iface)
            .enc
            .cfg_set
            .expect("non-null function pointer")(get_alg_priv(ctx), cfg);
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as ::core::ffi::c_uint
    } else {
        res as ::core::ffi::c_uint
    }) as vpx_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_pkt_list_add(
    mut list: *mut vpx_codec_pkt_list,
    mut pkt: *const vpx_codec_cx_pkt,
) -> ::core::ffi::c_int {
    if (*list).cnt < (*list).max {
        let fresh0 = (*list).cnt;
        (*list).cnt = (*list).cnt.wrapping_add(1);
        *(&raw mut (*list).pkts as *mut vpx_codec_cx_pkt).offset(fresh0 as isize) = *pkt;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_codec_pkt_list_get(
    mut list: *mut vpx_codec_pkt_list,
    mut iter: *mut vpx_codec_iter_t,
) -> *const vpx_codec_cx_pkt_t {
    let mut pkt: *const vpx_codec_cx_pkt_t = ::core::ptr::null::<vpx_codec_cx_pkt_t>();
    if (*iter).is_null() {
        *iter = &raw mut (*list).pkts as *mut vpx_codec_cx_pkt as vpx_codec_iter_t;
    }
    pkt = *iter as *const vpx_codec_cx_pkt_t;
    if (pkt.offset_from(&raw mut (*list).pkts as *mut vpx_codec_cx_pkt) as ::core::ffi::c_long
        as size_t)
        < (*list).cnt as size_t
    {
        *iter = pkt.offset(1 as ::core::ffi::c_int as isize) as vpx_codec_iter_t;
    } else {
        pkt = ::core::ptr::null::<vpx_codec_cx_pkt_t>();
    }
    return pkt;
}
