unsafe extern "C" {
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
use crate::vp8::decoder::onyxd_if::{
    vp8dx_receive_compressed_data_safe,
    vp8_create_decoder_instances,
    vp8_remove_decoder_instances,
};
use crate::vp8::decoder::threading::{
    vp8_decoder_create_threads, vp8_decoder_remove_threads, vp8mt_alloc_temp_buffers,
    vp8mt_de_alloc_temp_buffers,
};
use crate::vp8::common::rtcd::vp8_rtcd;
use crate::vpx_dsp::vpx_dsp_rtcd::vpx_dsp_rtcd;
use crate::vpx_scale::vpx_scale_rtcd::vpx_scale_rtcd;
pub type int64_t = i64;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub use crate::vp8::common::types::*;
use crate::vp8::common::mbpitch::vp8_build_block_doffsets;
use crate::vp8::common::alloccommon::vp8_alloc_frame_buffers;
use crate::vp8::common::types::{arm_panic_hook, is_vp8_bail};

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
pub type vpx_img_fmt = ::core::ffi::c_uint;
pub type vpx_img_fmt_t = vpx_img_fmt;
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
pub type vpx_fixed_buf_t = vpx_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_fixed_buf {
    pub buf: *mut ::core::ffi::c_void,
    pub sz: size_t,
}
pub type vpx_rc_mode = ::core::ffi::c_uint;
pub type vpx_enc_pass = ::core::ffi::c_uint;
pub type vpx_codec_er_flags_t = uint32_t;
pub type vpx_bit_depth_t = vpx_bit_depth;
pub type vpx_bit_depth = ::core::ffi::c_uint;
pub type vpx_codec_get_preview_frame_fn_t =
    Option<unsafe extern "C" fn(*mut vpx_codec_alg_priv_t) -> *mut vpx_image_t>;
pub type vpx_codec_alg_priv_t = vpx_codec_alg_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_alg_priv {
    pub base: vpx_codec_priv_t,
    pub cfg: vpx_codec_dec_cfg_t,
    pub si: vp8_stream_info_t,
    pub decoder_init: ::core::ffi::c_int,
    pub restart_threads: ::core::ffi::c_int,
    pub postproc_cfg_set: ::core::ffi::c_int,
    pub postproc_cfg: vp8_postproc_cfg_t,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub img: vpx_image_t,
    pub img_setup: ::core::ffi::c_int,
    pub yv12_frame_buffers: frame_buffers,
    pub user_priv: *mut ::core::ffi::c_void,
    pub fragments: FRAGMENT_DATA,
}
pub type vp8_postproc_cfg_t = vp8_postproc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_postproc_cfg {
    pub post_proc_flag: ::core::ffi::c_int,
    pub deblocking_level: ::core::ffi::c_int,
    pub noise_level: ::core::ffi::c_int,
}
pub type vp8_stream_info_t = vpx_codec_stream_info_t;
pub type vpx_codec_stream_info_t = vpx_codec_stream_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_stream_info {
    pub sz: ::core::ffi::c_uint,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
    pub is_kf: ::core::ffi::c_uint,
}
pub type vpx_codec_dec_cfg_t = vpx_codec_dec_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_dec_cfg {
    pub threads: ::core::ffi::c_uint,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
}
pub type vpx_codec_priv_t = vpx_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_codec_priv {
    pub err_detail: *const ::core::ffi::c_char,
    pub init_flags: vpx_codec_flags_t,
    pub dec: C2RustUnnamed_2,
    pub enc: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub cx_data_dst_buf: vpx_fixed_buf_t,
    pub cx_data_pad_before: ::core::ffi::c_uint,
    pub cx_data_pad_after: ::core::ffi::c_uint,
    pub cx_data_pkt: vpx_codec_cx_pkt_t,
    pub total_encoders: ::core::ffi::c_uint,
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
pub struct C2RustUnnamed_1 {
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
pub type vpx_codec_control_fn_t = Option<
    unsafe extern "C" fn(*mut vpx_codec_alg_priv_t, *mut ::core::ffi::c_void) -> vpx_codec_err_t,
>;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub dec: *const vpx_codec_dec_cfg,
    pub enc: *const vpx_codec_enc_cfg,
    pub raw: *const ::core::ffi::c_void,
}
pub type vpx_codec_iface_t = vpx_codec_iface;
pub type vp8_postproc_level = ::core::ffi::c_uint;
pub const VP8_MFQE: vp8_postproc_level = 8;
pub const VP8_DEMACROBLOCK: vp8_postproc_level = 2;
pub const VP8_DEBLOCK: vp8_postproc_level = 1;
pub type vpx_ref_frame_type = ::core::ffi::c_uint;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_ppflags_t {
    pub post_proc_flag: ::core::ffi::c_int,
    pub deblocking_level: ::core::ffi::c_int,
    pub noise_level: ::core::ffi::c_int,
    pub display_ref_frame_flag: ::core::ffi::c_int,
    pub display_mb_modes_flag: ::core::ffi::c_int,
    pub display_b_modes_flag: ::core::ffi::c_int,
    pub display_mv_flag: ::core::ffi::c_int,
}
pub const LAST_FRAME: C2RustUnnamed_5 = 1;
pub const GOLDEN_FRAME: C2RustUnnamed_5 = 2;
pub const ALTREF_FRAME: C2RustUnnamed_5 = 3;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: C2RustUnnamed_5 = 4;
pub const INTRA_FRAME: C2RustUnnamed_5 = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const VPX_CODEC_USE_POSTPROC: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const VPX_CODEC_USE_ERROR_CONCEALMENT: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const VP8BORDERINPIXELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_PARTITIONS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[inline]
fn vpx_atomic_load_acquire(
    atomic: &vpx_atomic_int,
) -> ::core::ffi::c_int {
    atomic.value.load(core::sync::atomic::Ordering::Acquire)
}
unsafe extern "C" fn vp8_peek_si_internal(
    mut data: *const uint8_t,
    mut data_sz: ::core::ffi::c_uint,
    mut si: *mut vpx_codec_stream_info_t,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut ::core::ffi::c_void,
) -> vpx_codec_err_t { unsafe {
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
            let mut n: ::core::ffi::c_int =
                (if (::core::mem::size_of::<[uint8_t; 10]>() as usize) < data_sz as usize {
                    ::core::mem::size_of::<[uint8_t; 10]>() as usize
                } else {
                    data_sz as usize
                }) as ::core::ffi::c_int;
            decrypt_cb.expect("non-null function pointer")(
                decrypt_state,
                data as *const ::core::ffi::c_uchar,
                &raw mut clear_buffer as *mut ::core::ffi::c_uchar,
                n,
            );
            clear = &raw mut clear_buffer as *mut uint8_t;
        }
        (*si).is_kf = 0 as ::core::ffi::c_uint;
        if data_sz >= 10 as ::core::ffi::c_uint
            && *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x1 as ::core::ffi::c_int
                == 0
        {
            (*si).is_kf = 1 as ::core::ffi::c_uint;
            if *clear.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0x9d as ::core::ffi::c_int
                || *clear.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0x1 as ::core::ffi::c_int
                || *clear.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0x2a as ::core::ffi::c_int
            {
                return VPX_CODEC_UNSUP_BITSTREAM;
            }
            (*si).w = ((*clear.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                | (*clear.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                & 0x3fff as ::core::ffi::c_int) as ::core::ffi::c_uint;
            (*si).h = ((*clear.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                | (*clear.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                & 0x3fff as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if !((*si).h != 0 && (*si).w != 0) {
                (*si).h = 0 as ::core::ffi::c_uint;
                (*si).w = (*si).h;
                res = VPX_CODEC_CORRUPT_FRAME;
            }
        } else {
            res = VPX_CODEC_UNSUP_BITSTREAM;
        }
    }
    return res;
}}
unsafe extern "C" fn update_error_state(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut error: *const vpx_internal_error_info,
) -> vpx_codec_err_t { unsafe {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    res = (*error).error_code;
    if res as u64 != 0 {
        (*ctx).base.err_detail = if (*error).has_detail != 0 {
            &raw const (*error).detail as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    return res;
}}
unsafe extern "C" fn update_fragments(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: ::core::ffi::c_uint,
    mut res: *mut vpx_codec_err_t,
) -> ::core::ffi::c_int { unsafe {
    ::core::ptr::write_volatile(res, VPX_CODEC_OK);
    if (*ctx).fragments.count == 0 as ::core::ffi::c_uint {
        memset(
            &raw mut (*ctx).fragments.ptrs as *mut *const ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*const ::core::ffi::c_uchar; 9]>() as size_t,
        );
        memset(
            &raw mut (*ctx).fragments.sizes as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_uint; 9]>() as size_t,
        );
    }
    if (*ctx).fragments.enabled != 0
        && data.is_null()
        && data_sz == 0 as ::core::ffi::c_uint
        && (*ctx).fragments.count == 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*ctx).fragments.enabled != 0 && !(data.is_null() && data_sz == 0 as ::core::ffi::c_uint) {
        if (*ctx).fragments.count >= MAX_PARTITIONS as ::core::ffi::c_uint {
            (*ctx).fragments.count = 0 as ::core::ffi::c_uint;
            ::core::ptr::write_volatile(res, VPX_CODEC_INVALID_PARAM);
            return -(1 as ::core::ffi::c_int);
        }
        (*ctx).fragments.ptrs[(*ctx).fragments.count as usize] =
            data as *const ::core::ffi::c_uchar;
        (*ctx).fragments.sizes[(*ctx).fragments.count as usize] = data_sz;
        (*ctx).fragments.count = (*ctx).fragments.count.wrapping_add(1);
        return 0 as ::core::ffi::c_int;
    }
    if (*ctx).fragments.enabled == 0 && (data.is_null() && data_sz == 0 as ::core::ffi::c_uint) {
        return 0 as ::core::ffi::c_int;
    }
    if (*ctx).fragments.enabled == 0 {
        (*ctx).fragments.ptrs[0 as ::core::ffi::c_int as usize] =
            data as *const ::core::ffi::c_uchar;
        (*ctx).fragments.sizes[0 as ::core::ffi::c_int as usize] = data_sz;
        (*ctx).fragments.count = 1 as ::core::ffi::c_uint;
    }
    return 1 as ::core::ffi::c_int;
}}
unsafe extern "C" fn vp8_decode(
    mut ctx: *mut vpx_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: ::core::ffi::c_uint,
    mut user_priv: *mut ::core::ffi::c_void,
) -> vpx_codec_err_t { unsafe {
    let mut res: vpx_codec_err_t = VPX_CODEC_OK;
    let mut resolution_change: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut w: ::core::ffi::c_uint = 0;
    let mut h: ::core::ffi::c_uint = 0;
    if (*ctx).fragments.enabled == 0 && (data.is_null() && data_sz == 0 as ::core::ffi::c_uint) {
        return VPX_CODEC_OK;
    }
    if update_fragments(ctx, data, data_sz, &raw mut res) <= 0 as ::core::ffi::c_int {
        return res;
    }
    ::core::ptr::write_volatile(&mut w as *mut ::core::ffi::c_uint, (*ctx).si.w);
    ::core::ptr::write_volatile(&mut h as *mut ::core::ffi::c_uint, (*ctx).si.h);
    ::core::ptr::write_volatile(
        &mut res as *mut vpx_codec_err_t,
        vp8_peek_si_internal(
            (*ctx).fragments.ptrs[0 as ::core::ffi::c_int as usize],
            (*ctx).fragments.sizes[0 as ::core::ffi::c_int as usize],
            &raw mut (*ctx).si,
            (*ctx).decrypt_cb,
            (*ctx).decrypt_state,
        ),
    );
    if res as ::core::ffi::c_uint
        == VPX_CODEC_UNSUP_BITSTREAM as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*ctx).si.is_kf == 0
    {
        ::core::ptr::write_volatile(&mut res as *mut vpx_codec_err_t, VPX_CODEC_OK);
    }
    if (*ctx).decoder_init == 0 && (*ctx).si.is_kf == 0 {
        ::core::ptr::write_volatile(&mut res as *mut vpx_codec_err_t, VPX_CODEC_UNSUP_BITSTREAM);
    }
    if res as u64 == 0
        && (*ctx).decoder_init != 0
        && w == 0 as ::core::ffi::c_uint
        && h == 0 as ::core::ffi::c_uint
        && (*ctx).si.h == 0 as ::core::ffi::c_uint
        && (*ctx).si.w == 0 as ::core::ffi::c_uint
    {
        let mut pbi: *mut VP8D_COMP =
            (*ctx).yv12_frame_buffers.pbi[0 as ::core::ffi::c_int as usize];
        ::core::ptr::write_volatile(&mut res as *mut vpx_codec_err_t, VPX_CODEC_CORRUPT_FRAME);
        (*pbi).common.error.trigger(res, "Keyframe / intra-only frame required to reset decoder state");
    }
    if (*ctx).si.h != h || (*ctx).si.w != w {
        ::core::ptr::write_volatile(
            &mut resolution_change as *mut ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
        );
    }
    if res as u64 == 0 && (*ctx).restart_threads != 0 {
        let mut pbi_0: *mut VP8D_COMP =
            (*ctx).yv12_frame_buffers.pbi[0 as ::core::ffi::c_int as usize];
        let pc: *mut VP8_COMMON = &raw mut (*pbi_0).common;
        (*pbi_0).max_threads = (*ctx).cfg.threads as ::core::ffi::c_int;
        (*pbi_0).common.error.setjmp = 1 as ::core::ffi::c_int;
        let guard = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vp8_decoder_create_threads(&mut *pbi_0);
            if vpx_atomic_load_acquire(&(*pbi_0).b_multithreaded_rd) != 0 {
                vp8mt_alloc_temp_buffers(&mut *pbi_0, (*pc).Width, (*pc).mb_rows);
            }
        }));
        (*pbi_0).common.error.setjmp = 0 as ::core::ffi::c_int;
        if let Err(p) = guard {
            if !is_vp8_bail(&p) {
                std::panic::resume_unwind(p);
            }
            vp8_decoder_remove_threads(&mut *pbi_0);
            return VPX_CODEC_ERROR;
        }
        (*ctx).restart_threads = 0 as ::core::ffi::c_int;
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
        oxcf.Width = (*ctx).si.w as ::core::ffi::c_int;
        oxcf.Height = (*ctx).si.h as ::core::ffi::c_int;
        oxcf.Version = 9 as ::core::ffi::c_int;
        oxcf.postprocess = 0 as ::core::ffi::c_int;
        oxcf.max_threads = (*ctx).cfg.threads as ::core::ffi::c_int;
        oxcf.error_concealment = ((*ctx).base.init_flags
            & VPX_CODEC_USE_ERROR_CONCEALMENT as vpx_codec_flags_t)
            as ::core::ffi::c_int;
        if (*ctx).postproc_cfg_set == 0
            && (*ctx).base.init_flags & VPX_CODEC_USE_POSTPROC as vpx_codec_flags_t != 0
        {
            (*ctx).postproc_cfg.post_proc_flag = VP8_DEBLOCK as ::core::ffi::c_int
                | VP8_DEMACROBLOCK as ::core::ffi::c_int
                | VP8_MFQE as ::core::ffi::c_int;
            (*ctx).postproc_cfg.deblocking_level = 4 as ::core::ffi::c_int;
            (*ctx).postproc_cfg.noise_level = 0 as ::core::ffi::c_int;
        }
        ::core::ptr::write_volatile(
            &mut res as *mut vpx_codec_err_t,
            vp8_create_decoder_instances(&mut (*ctx).yv12_frame_buffers, &oxcf)
                as vpx_codec_err_t,
        );
        if res as ::core::ffi::c_uint == VPX_CODEC_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
            (*ctx).decoder_init = 1 as ::core::ffi::c_int;
        } else {
            (*ctx).si.w = 0 as ::core::ffi::c_uint;
            (*ctx).si.h = 0 as ::core::ffi::c_uint;
        }
    }
    if (*ctx).decoder_init != 0 {
        (*(*ctx).yv12_frame_buffers.pbi[0 as ::core::ffi::c_int as usize]).decrypt_cb =
            (*ctx).decrypt_cb;
        (*(*ctx).yv12_frame_buffers.pbi[0 as ::core::ffi::c_int as usize]).decrypt_state =
            (*ctx).decrypt_state;
    }
    if res as u64 == 0 {
        let mut pbi_1: *mut VP8D_COMP =
            (*ctx).yv12_frame_buffers.pbi[0 as ::core::ffi::c_int as usize];
        let pc_0: *mut VP8_COMMON = &raw mut (*pbi_1).common;
        if resolution_change != 0 {
            let xd: *mut MACROBLOCKD = &raw mut (*pbi_1).mb;
            let mut i: ::core::ffi::c_int = 0;
            (*pc_0).Width = (*ctx).si.w as ::core::ffi::c_int;
            (*pc_0).Height = (*ctx).si.h as ::core::ffi::c_int;
            (*pbi_1).common.error.setjmp = 1 as ::core::ffi::c_int;
            let guard = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if (*pc_0).Width <= 0 as ::core::ffi::c_int {
                    (*pc_0).Width = w as ::core::ffi::c_int;
                    (*pc_0).error.trigger(VPX_CODEC_CORRUPT_FRAME, "Invalid frame width");
                }
                if (*pc_0).Height <= 0 as ::core::ffi::c_int {
                    (*pc_0).Height = h as ::core::ffi::c_int;
                    (*pc_0).error.trigger(VPX_CODEC_CORRUPT_FRAME, "Invalid frame height");
                }
                if vpx_atomic_load_acquire(&(*pbi_1).b_multithreaded_rd) != 0 {
                    vp8mt_de_alloc_temp_buffers(&mut *pbi_1, (*pc_0).mb_rows);
                }
                if vp8_alloc_frame_buffers(&mut *pc_0, (*pc_0).Width, (*pc_0).Height) != 0 {
                    (*pc_0).error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate frame buffers");
                }
                let lst_fb_idx = (*pc_0).lst_fb_idx as usize;
                let new_fb_idx = (*pc_0).new_fb_idx as usize;
                (*xd).pre_fb_idx = lst_fb_idx;
                (*xd).dst_fb_idx = new_fb_idx;
                (*xd).dst_y_stride = (*pc_0).yv12_fb[new_fb_idx].y_stride;
                (*xd).dst_uv_stride = (*pc_0).yv12_fb[new_fb_idx].uv_stride;
                (*xd).dst_border = (*pc_0).yv12_fb[new_fb_idx].border;
                (*xd).pre_y_stride = (*pc_0).yv12_fb[lst_fb_idx].y_stride;
                (*xd).pre_uv_stride = (*pc_0).yv12_fb[lst_fb_idx].uv_stride;
                (*xd).pre_border = (*pc_0).yv12_fb[lst_fb_idx].border;
                i = 0 as ::core::ffi::c_int;
                while i < (*pbi_1).allocated_decoding_thread_count {
                    let mut m_guard = (*pbi_1).mb_row_di.as_mut().unwrap()[i as usize].lock().unwrap();
                    let mbd = &mut m_guard.mbd;
                    mbd.dst_fb_idx = new_fb_idx;
                    mbd.dst_y_stride = (*pc_0).yv12_fb[new_fb_idx].y_stride;
                    mbd.dst_uv_stride = (*pc_0).yv12_fb[new_fb_idx].uv_stride;
                    mbd.dst_border = (*pc_0).yv12_fb[new_fb_idx].border;
                    vp8_build_block_doffsets(mbd);
                    i += 1;
                }
                vp8_build_block_doffsets(&mut (*pbi_1).mb);
                if vpx_atomic_load_acquire(&(*pbi_1).b_multithreaded_rd) != 0 {
                    vp8mt_alloc_temp_buffers(&mut *pbi_1, (*pc_0).Width, 0 as ::core::ffi::c_int);
                }
            }));
            (*pbi_1).common.error.setjmp = 0 as ::core::ffi::c_int;
            if let Err(p) = guard {
                if !is_vp8_bail(&p) {
                    std::panic::resume_unwind(p);
                }
                (*ctx).si.w = 0 as ::core::ffi::c_uint;
                (*ctx).si.h = 0 as ::core::ffi::c_uint;
                return 4294967295 as vpx_codec_err_t;
            }
            (*pbi_1).common.fb_idx_ref_cnt[0 as ::core::ffi::c_int as usize] =
                0 as ::core::ffi::c_int;
        }
        (*pbi_1).fragments = (*ctx).fragments;
        (*pbi_1).restart_threads = 0 as ::core::ffi::c_int;
        (*ctx).user_priv = user_priv;
        (*pbi_1).common.error.setjmp = 1 as ::core::ffi::c_int;
        let guard = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vp8dx_receive_compressed_data_safe(&mut *pbi_1)
        }));
        (*pbi_1).common.error.setjmp = 0 as ::core::ffi::c_int;
        match guard {
            Ok(rv) => {
                if rv != 0 {
                    ::core::ptr::write_volatile(
                        &mut res as *mut vpx_codec_err_t,
                        update_error_state(ctx, &raw mut (*pbi_1).common.error),
                    );
                }
                (*ctx).fragments.count = 0 as ::core::ffi::c_uint;
            }
            Err(p) => {
                if !is_vp8_bail(&p) {
                    std::panic::resume_unwind(p);
                }
                (*pc_0).yv12_fb[(*pc_0).lst_fb_idx as usize].corrupted = 1 as ::core::ffi::c_int;
                if (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] > 0 as ::core::ffi::c_int {
                    (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] -= 1;
                }
                if (*pbi_1).restart_threads != 0 {
                    (*ctx).si.w = 0 as ::core::ffi::c_uint;
                    (*ctx).si.h = 0 as ::core::ffi::c_uint;
                    (*ctx).restart_threads = 1 as ::core::ffi::c_int;
                }
                ::core::ptr::write_volatile(
                    &mut res as *mut vpx_codec_err_t,
                    update_error_state(ctx, &raw mut (*pbi_1).common.error),
                );
                return res;
            }
        }
    }
    return res;
}}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub struct Vp8DecoderInstance {
    priv_0: *mut vpx_codec_alg_priv_t,
}

impl Vp8DecoderInstance {
    pub fn new(threads: u32) -> Result<Self, crate::api::DecodeError> {
        // Keep the default panic hook quiet for the internal decode-bail
        // unwind that replaced setjmp/longjmp; all other panics still print.
        arm_panic_hook();
        unsafe {
            vp8_rtcd();
            vpx_dsp_rtcd();
            vpx_scale_rtcd();
            let priv_box = Box::new(std::mem::zeroed::<vpx_codec_alg_priv_t>());
            let priv_0 = Box::into_raw(priv_box);
            (*priv_0).cfg.threads = threads;
            (*priv_0).si.sz = ::core::mem::size_of::<vp8_stream_info_t>() as ::core::ffi::c_uint;
            (*priv_0).decrypt_cb = None;
            (*priv_0).decrypt_state = NULL;
            (*priv_0).fragments.count = 0 as ::core::ffi::c_uint;
            (*priv_0).fragments.enabled = 0 as ::core::ffi::c_int;
            Ok(Self { priv_0 })
        }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<(), crate::api::DecodeError> {
        unsafe {
            let res = vp8_decode(
                self.priv_0,
                data.as_ptr(),
                data.len() as ::core::ffi::c_uint,
                ::core::ptr::null_mut(),
            );
            if res == VPX_CODEC_OK {
                Ok(())
            } else {
                Err(crate::api::DecodeError::Decode(res))
            }
        }
    }

    pub fn get_frame(&mut self) -> Option<(&YV12_BUFFER_CONFIG, i32, i32)> {
        unsafe {
            if self.priv_0.is_null() {
                return None;
            }
            let pbi = (*self.priv_0).yv12_frame_buffers.pbi[0];
            if pbi.is_null() {
                return None;
            }
            if (*pbi).ready_for_new_data == 1 {
                return None;
            }
            if (*pbi).common.show_frame == 0 {
                return None;
            }
            (*pbi).ready_for_new_data = 1;
            if let Some(idx) = (*pbi).common.frame_to_show_idx {
                Some((&(*pbi).common.yv12_fb[idx], (*pbi).common.Width, (*pbi).common.Height))
            } else {
                None
            }
        }
    }
}

impl Drop for Vp8DecoderInstance {
    fn drop(&mut self) {
        unsafe {
            if !self.priv_0.is_null() {
                vp8_remove_decoder_instances(&mut (*self.priv_0).yv12_frame_buffers);
                let _ = Box::from_raw(self.priv_0);
            }
        }
    }
}

