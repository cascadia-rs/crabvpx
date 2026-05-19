use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_rtcd();
    fn vpx_dsp_rtcd();
    fn vpx_scale_rtcd();
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(info: *mut VpxInternalErrorInfo, error: u32, fmt: *const i8);
    fn vp8_build_block_doffsets(x: *mut MACROBLOCKD);
    fn vp8_alloc_frame_buffers(oci: *mut Vp8Common, width: i32, height: i32) -> i32;
    fn vpx_calloc(num: usize, size: usize) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8dx_receive_compressed_data(pbi: *mut Vp8dComp) -> i32;
    fn vp8dx_get_raw_frame(
        pbi: *mut Vp8dComp,
        sd: *mut Yv12BufferConfig,
        flags: *mut Vp8PpflagsT,
    ) -> i32;
    fn vp8dx_references_buffer(oci: *mut VP8Common, ref_frame: i32) -> i32;
    fn vp8dx_get_reference(
        pbi: *mut Vp8dComp,
        ref_frame_flag: u32,
        sd: *mut Yv12BufferConfig,
    ) -> u32;
    fn vp8dx_set_reference(
        pbi: *mut Vp8dComp,
        ref_frame_flag: u32,
        sd: *mut Yv12BufferConfig,
    ) -> u32;
    fn vp8dx_get_quantizer(pbi: *const Vp8dComp) -> i32;
    fn vp8_create_decoder_instances(fb: *mut FrameBuffers, oxcf: *mut Vp8dConfig) -> i32;
    fn vp8_remove_decoder_instances(fb: *mut FrameBuffers) -> i32;
    fn vp8_decoder_remove_threads(pbi: *mut Vp8dComp);
    fn vp8_decoder_create_threads(pbi: *mut Vp8dComp);
    fn vp8mt_alloc_temp_buffers(pbi: *mut Vp8dComp, width: i32, prev_mb_rows: i32);
    fn vp8mt_de_alloc_temp_buffers(pbi: *mut Vp8dComp, mb_rows: i32);
}
pub type BuiltinVaList = *mut i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DarwinPthreadHandlerRec {
    pub __routine: Option<unsafe fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut DarwinPthreadHandlerRec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadT {
    pub __sig: i64,
    pub __cleanup_stack: *mut DarwinPthreadHandlerRec,
    pub __opaque: [i8; 8176],
}
pub type DarwinPthreadT = *mut OpaquePthreadT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
    pub bmi: BModeInfo,
}
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
pub struct Macroblockd {
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
    pub pre: Yv12BufferConfig,
    pub dst: Yv12BufferConfig,
    pub mode_info_context: *mut ModeInfo,
    pub mode_info_stride: i32,
    pub frame_type: u32,
    pub up_available: bool,
    pub left_available: bool,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: *mut EntropyContextPlanes,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [u8; 3],
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
    pub subpixel_predict: Vp8SubpixFnT,
    pub subpixel_predict8x4: Vp8SubpixFnT,
    pub subpixel_predict8x8: Vp8SubpixFnT,
    pub subpixel_predict16x16: Vp8SubpixFnT,
    pub current_bc: *mut c_void,
    pub corrupted: i32,
    pub error_info: VpxInternalErrorInfo,
}
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
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
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
pub type BLOCKD = Blockd;
pub const VPX_IMG_FMT_I44016: u32 = 2311;
pub const VPX_IMG_FMT_I44416: u32 = 2310;
pub const VPX_IMG_FMT_I42216: u32 = 2309;
pub const VPX_IMG_FMT_I42016: u32 = 2306;
pub const VPX_IMG_FMT_NV12: u32 = 265;
pub const VPX_IMG_FMT_I440: u32 = 263;
pub const VPX_IMG_FMT_I444: u32 = 262;
pub const VPX_IMG_FMT_I422: u32 = 261;
pub const VPX_IMG_FMT_I420: u32 = 258;
pub const VPX_IMG_FMT_YV12: u32 = 769;
pub const VPX_IMG_FMT_NONE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxImage {
    pub fmt: u32,
    pub cs: u32,
    pub range: u32,
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
    pub self_allocd: bool,
    pub fb_priv: *mut c_void,
}
pub type VpxImageT = VpxImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxImageRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
pub type VpxImageRectT = VpxImageRect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecIface {
    pub name: *const i8,
    pub abi_version: i32,
    pub caps: i64,
    pub init: VpxCodecInitFnT,
    pub destroy: VpxCodecDestroyFnT,
    pub ctrl_maps: *const VpxCodecCtrlFnMapT,
    pub dec: VpxCodecDecIface,
    pub enc: VpxCodecEncIface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncIface {
    pub cfg_map_count: i32,
    pub cfg_maps: *const VpxCodecEncCfgMapT,
    pub encode: VpxCodecEncodeFnT,
    pub get_cx_data: VpxCodecGetCxDataFnT,
    pub cfg_set: VpxCodecEncConfigSetFnT,
    pub get_glob_hdrs: VpxCodecGetGlobalHeadersFnT,
    pub get_preview: VpxCodecGetPreviewFrameFnT,
    pub mr_get_mem_loc: VpxCodecEncMrGetMemLocFnT,
    pub mr_free_mem_loc: VpxCodecEncMrFreeMemLocFnT,
}
pub type VpxCodecEncMrFreeMemLocFnT = Option<unsafe fn(*mut c_void) -> ()>;
pub type VpxCodecEncMrGetMemLocFnT =
    Option<unsafe fn(*const VpxCodecEncCfgT, *mut *mut c_void) -> u32>;
pub type VpxCodecEncCfgT = VpxCodecEncCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncCfg {
    pub g_usage: u32,
    pub g_threads: u32,
    pub g_profile: u32,
    pub g_w: u32,
    pub g_h: u32,
    pub g_bit_depth: u32,
    pub g_input_bit_depth: u32,
    pub g_timebase: VpxRational,
    pub g_error_resilient: u32,
    pub g_pass: u32,
    pub g_lag_in_frames: u32,
    pub rc_dropframe_thresh: u32,
    pub rc_resize_allowed: u32,
    pub rc_scaled_width: u32,
    pub rc_scaled_height: u32,
    pub rc_resize_up_thresh: u32,
    pub rc_resize_down_thresh: u32,
    pub rc_end_usage: u32,
    pub rc_twopass_stats_in: VpxFixedBufT,
    pub rc_firstpass_mb_stats_in: VpxFixedBufT,
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
    pub kf_mode: u32,
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
    pub active_wq_factor: VpxRationalT,
    pub err_per_mb_factor: VpxRationalT,
    pub sr_default_decay_limit: VpxRationalT,
    pub sr_diff_factor: VpxRationalT,
    pub kf_err_per_mb_factor: VpxRationalT,
    pub kf_frame_min_boost_factor: VpxRationalT,
    pub kf_frame_max_boost_first_factor: VpxRationalT,
    pub kf_frame_max_boost_subs_factor: VpxRationalT,
    pub kf_max_total_boost_factor: VpxRationalT,
    pub gf_max_total_boost_factor: VpxRationalT,
    pub gf_frame_max_boost_factor: VpxRationalT,
    pub zm_factor: VpxRationalT,
    pub rd_mult_inter_qp_fac: VpxRationalT,
    pub rd_mult_arf_qp_fac: VpxRationalT,
    pub rd_mult_key_qp_fac: VpxRationalT,
}
pub type VpxRationalT = VpxRational;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxRational {
    pub num: i32,
    pub den: i32,
}
pub const VPX_KF_DISABLED: u32 = 0;
pub const VPX_KF_AUTO: u32 = 1;
pub const VPX_KF_FIXED: u32 = 0;
pub type VpxFixedBufT = VpxFixedBuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxFixedBuf {
    pub buf: *mut c_void,
    pub sz: usize,
}
pub const VPX_Q: u32 = 3;
pub const VPX_CQ: u32 = 2;
pub const VPX_CBR: u32 = 1;
pub const VPX_VBR: u32 = 0;
pub const VPX_RC_LAST_PASS: u32 = 2;
pub const VPX_RC_FIRST_PASS: u32 = 1;
pub const VPX_RC_ONE_PASS: u32 = 0;
pub const VPX_BITS_12: u32 = 12;
pub const VPX_BITS_10: u32 = 10;
pub const VPX_BITS_8: u32 = 8;
pub type VpxCodecGetPreviewFrameFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxImageT>;
pub type VpxCodecAlgPrivT = VpxCodecAlgPriv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecAlgPriv {
    pub base: VpxCodecPrivT,
    pub cfg: VpxCodecDecCfgT,
    pub si: Vp8StreamInfoT,
    pub decoder_init: bool,
    pub restart_threads: bool,
    pub postproc_cfg_set: bool,
    pub postproc_cfg: Vp8PostprocCfgT,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
    pub img: VpxImageT,
    pub img_setup: bool,
    pub yv12_frame_buffers: FrameBuffers,
    pub user_priv: *mut c_void,
    pub fragments: FragmentData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FragmentData {
    pub enabled: bool,
    pub count: u32,
    pub ptrs: [*const u8; 9],
    pub sizes: [u32; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameBuffers {
    pub pbi: [*mut Vp8dComp; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8dComp {
    pub mb: MACROBLOCKD,
    pub dec_fb_ref: [*mut Yv12BufferConfig; 4],
    pub common: Vp8Common,
    pub mbc: [Vp8Reader; 9],
    pub oxcf: Vp8dConfig,
    pub fragments: FragmentData,
    pub b_multithreaded_rd: VpxAtomicInt,
    pub max_threads: i32,
    pub current_mb_col_main: i32,
    pub decoding_thread_count: u32,
    pub allocated_decoding_thread_count: i32,
    pub mt_baseline_filter_level: [i32; 4],
    pub sync_range: i32,
    pub mt_current_mb_col: *mut VpxAtomicInt,
    pub mt_yabove_row: *mut *mut u8,
    pub mt_uabove_row: *mut *mut u8,
    pub mt_vabove_row: *mut *mut u8,
    pub mt_yleft_col: *mut *mut u8,
    pub mt_uleft_col: *mut *mut u8,
    pub mt_vleft_col: *mut *mut u8,
    pub mb_row_di: *mut MbRowDec,
    pub de_thread_data: *mut DecodethreadData,
    pub h_decoding_thread: *mut PthreadT,
    pub h_event_start_decoding: *mut SemaphoreT,
    pub h_event_end_decoding: SemaphoreT,
    pub ready_for_new_data: bool,
    pub prob_intra: u8,
    pub prob_last: u8,
    pub prob_gf: u8,
    pub prob_skip_false: u8,
    pub ec_enabled: bool,
    pub ec_active: bool,
    pub decoded_key_frame: bool,
    pub independent_partitions: bool,
    pub frame_corrupt_residual: i32,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
    pub restart_threads: bool,
}
pub type VpxDecryptCb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type SemaphoreT = *mut c_void;
pub type PthreadT = *mut c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecodethreadData {
    pub ithread: i32,
    pub ptr1: *mut c_void,
    pub ptr2: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbRowDec {
    pub mbd: MACROBLOCKD,
}
pub type MACROBLOCKD = Macroblockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxAtomicInt {
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8dConfig {
    pub width: i32,
    pub height: i32,
    pub version: i32,
    pub postprocess: i32,
    pub max_threads: i32,
    pub error_concealment: i32,
}
pub type BoolDecoder = Vp8Reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8Reader {
    pub user_buffer_end: *const u8,
    pub user_buffer: *const u8,
    pub value: usize,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
}
pub type Vp8Common = VP8Common;
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
pub type Vp8PostprocCfgT = Vp8PostprocCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8PostprocCfg {
    pub post_proc_flag: i32,
    pub deblocking_level: i32,
    pub noise_level: i32,
}
pub type Vp8StreamInfoT = VpxCodecStreamInfoT;
pub type VpxCodecStreamInfoT = VpxCodecStreamInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecStreamInfo {
    pub sz: u32,
    pub w: u32,
    pub h: u32,
    pub is_kf: u32,
}
pub type VpxCodecDecCfgT = VpxCodecDecCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecDecCfg {
    pub threads: u32,
    pub w: u32,
    pub h: u32,
}
pub type VpxCodecPrivT = VpxCodecPriv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPriv {
    pub err_detail: *const i8,
    pub init_flags: i64,
    pub dec: C2RustUnnamed_2,
    pub enc: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub cx_data_dst_buf: VpxFixedBufT,
    pub cx_data_pad_before: u32,
    pub cx_data_pad_after: u32,
    pub cx_data_pkt: VpxCodecCxPktT,
    pub total_encoders: u32,
}
pub type VpxCodecCxPktT = VpxCodecCxPkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCxPkt {
    pub kind: u32,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub frame: C2RustUnnamed_1,
    pub twopass_stats: VpxFixedBufT,
    pub firstpass_mb_stats: VpxFixedBufT,
    pub psnr: VpxPsnrPkt,
    pub raw: VpxFixedBufT,
    pub pad: [i8; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxPsnrPkt {
    pub samples: [u32; 4],
    pub sse: [u64; 4],
    pub psnr: [f64; 4],
    pub spatial_layer_id: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub buf: *mut c_void,
    pub sz: usize,
    pub pts: i64,
    pub duration: u64,
    pub flags: u32,
    pub partition_id: i32,
    pub width: [u32; 5],
    pub height: [u32; 5],
    pub spatial_layer_encoded: [u8; 5],
}
pub const VPX_CODEC_CUSTOM_PKT: u32 = 256;
pub const VPX_CODEC_PSNR_PKT: u32 = 3;
pub const VPX_CODEC_FPMB_STATS_PKT: u32 = 2;
pub const VPX_CODEC_STATS_PKT: u32 = 1;
pub const VPX_CODEC_CX_FRAME_PKT: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub put_frame_cb: VpxCodecPrivCbPairT,
    pub put_slice_cb: VpxCodecPrivCbPairT,
}
pub type VpxCodecPrivCbPairT = VpxCodecPrivCbPair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPrivCbPair {
    pub u: C2RustUnnamed_3,
    pub user_priv: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub put_frame: VpxCodecPutFrameCbFnT,
    pub put_slice: VpxCodecPutSliceCbFnT,
}
pub type VpxCodecPutSliceCbFnT = Option<
    unsafe fn(*mut c_void, *const VpxImageT, *const VpxImageRectT, *const VpxImageRectT) -> (),
>;
pub type VpxCodecPutFrameCbFnT = Option<unsafe fn(*mut c_void, *const VpxImageT) -> ()>;
pub type VpxCodecGetGlobalHeadersFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxFixedBufT>;
pub type VpxCodecEncConfigSetFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const VpxCodecEncCfgT) -> u32>;
pub type VpxCodecGetCxDataFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *const VpxCodecCxPktT>;
pub type VpxCodecIterT = *const c_void;
pub type VpxCodecEncodeFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const VpxImageT, i64, u64, i64, u64) -> u32>;
pub type VpxCodecEncCfgMapT = VpxCodecEncCfgMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncCfgMap {
    pub usage: i32,
    pub cfg: VpxCodecEncCfgT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecDecIface {
    pub peek_si: VpxCodecPeekSiFnT,
    pub get_si: VpxCodecGetSiFnT,
    pub decode: VpxCodecDecodeFnT,
    pub get_frame: VpxCodecGetFrameFnT,
    pub set_fb_fn: VpxCodecSetFbFnT,
}
pub type VpxCodecSetFbFnT = Option<
    unsafe fn(
        *mut VpxCodecAlgPrivT,
        VpxGetFrameBufferCbFnT,
        VpxReleaseFrameBufferCbFnT,
        *mut c_void,
    ) -> u32,
>;
pub type VpxReleaseFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecFrameBufferT = VpxCodecFrameBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecFrameBuffer {
    pub data: *mut u8,
    pub size: usize,
    pub priv_0: *mut c_void,
}
pub type VpxGetFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, usize, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecGetFrameFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *mut VpxImageT>;
pub type VpxCodecDecodeFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const u8, u32, *mut c_void) -> u32>;
pub type VpxCodecGetSiFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecStreamInfoT) -> u32>;
pub type VpxCodecPeekSiFnT = Option<unsafe fn(*const u8, u32, *mut VpxCodecStreamInfoT) -> u32>;
pub type VpxCodecCtrlFnMapT = VpxCodecCtrlFnMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCtrlFnMap {
    pub ctrl_id: i32,
    pub fn_0: VpxCodecControlFnT,
}
pub type VpxCodecControlFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32>;
pub type VaList = BuiltinVaList;
pub type VpxCodecDestroyFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> u32>;
pub type VpxCodecInitFnT = Option<unsafe fn(*mut VpxCodecCtxT, *mut VpxCodecPrivEncMrCfgT) -> u32>;
pub type VpxCodecPrivEncMrCfgT = VpxCodecPrivEncMrCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPrivEncMrCfg {
    pub mr_total_resolutions: u32,
    pub mr_encoder_id: u32,
    pub mr_down_sampling_factor: VpxRational,
    pub mr_low_res_mode_info: *mut c_void,
}
pub type VpxCodecCtxT = VpxCodecCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCtx {
    pub name: *const i8,
    pub iface: *const VpxCodecIfaceT,
    pub err: u32,
    pub err_detail: *const i8,
    pub init_flags: i64,
    pub config: C2RustUnnamed_4,
    pub priv_0: *mut VpxCodecPrivT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub dec: *const VpxCodecDecCfg,
    pub enc: *const VpxCodecEncCfg,
    pub raw: *const c_void,
}
pub type VpxCodecIfaceT = VpxCodecIface;
pub const VP8_DECODER_CTRL_ID_START: u32 = 256;
pub const VP8_COMMON_CTRL_ID_MAX: u32 = 129;
pub const VP9_GET_REFERENCE: u32 = 128;
pub const VP8_SET_POSTPROC: u32 = 3;
pub const VP8_COPY_REFERENCE: u32 = 2;
pub const VP8_SET_REFERENCE: u32 = 1;
pub const VP8_MFQE: u32 = 8;
pub const VP8_ADDNOISE: u32 = 4;
pub const VP8_DEMACROBLOCK: u32 = 2;
pub const VP8_DEBLOCK: u32 = 1;
pub const VP8_NOFILTERING: u32 = 0;
pub const VP8_ALTR_FRAME: u32 = 4;
pub const VP8_GOLD_FRAME: u32 = 2;
pub const VP8_LAST_FRAME: u32 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxRefFrame {
    pub frame_type: u32,
    pub img: VpxImageT,
}
pub type VpxRefFrameT = VpxRefFrame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8PpflagsT {
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
pub struct VpxDecryptInit {
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
}
pub const VPXD_SET_DECRYPTOR: u32 = 259;
pub const VPXD_GET_LAST_QUANTIZER: u32 = 267;
pub const LAST_FRAME: C2RustUnnamed_5 = 1;
pub const GOLDEN_FRAME: C2RustUnnamed_5 = 2;
pub const ALTREF_FRAME: C2RustUnnamed_5 = 3;
pub const VP8D_GET_LAST_REF_USED: u32 = 258;
pub const VP8D_GET_FRAME_CORRUPTED: u32 = 257;
pub const VP8D_GET_LAST_REF_UPDATES: u32 = 256;
pub const VP8_DECODER_CTRL_ID_MAX: u32 = 270;
pub const VP9D_SET_LOOP_FILTER_OPT: u32 = 269;
pub const VP9D_SET_ROW_MT: u32 = 268;
pub const VP9_DECODE_SVC_SPATIAL_LAYER: u32 = 266;
pub const VP9_SET_SKIP_LOOP_FILTER: u32 = 265;
pub const VP9_INVERT_TILE_DECODE_ORDER: u32 = 264;
pub const VP9_SET_BYTE_ALIGNMENT: u32 = 263;
pub const VP9D_GET_BIT_DEPTH: u32 = 262;
pub const VP9D_GET_DISPLAY_SIZE: u32 = 261;
pub const VP9D_GET_FRAME_SIZE: u32 = 260;
pub const VP8D_SET_DECRYPTOR: u32 = 259;
#[allow(non_camel_case_types)]
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
unsafe fn vpx_atomic_load_acquire(mut atomic: *const VpxAtomicInt) -> i32 {
    unsafe {
        (*((&raw const (*atomic).value) as *const core::sync::atomic::AtomicI32))
            .load(core::sync::atomic::Ordering::Acquire)
    }
}
unsafe fn vp8_init_ctx(mut ctx: *mut VpxCodecCtxT) -> i32 {
    unsafe {
        let priv_0 = Box::into_raw(Box::new(core::mem::zeroed::<VpxCodecAlgPrivT>()));
        if priv_0.is_null() {
            return 1 as i32;
        }
        (*ctx).priv_0 = priv_0 as *mut VpxCodecPrivT;
        (*(*ctx).priv_0).init_flags = (*ctx).init_flags;
        (*priv_0).si.sz = ::core::mem::size_of::<Vp8StreamInfoT>() as u32;
        (*priv_0).decrypt_cb = None;
        (*priv_0).decrypt_state = NULL;
        if !(*ctx).config.dec.is_null() {
            (*priv_0).cfg = *(*ctx).config.dec as VpxCodecDecCfgT;
            (*ctx).config.dec = &raw mut (*priv_0).cfg;
        }
        0 as i32
    }
}
unsafe fn vp8_init(mut ctx: *mut VpxCodecCtxT, _data: *mut VpxCodecPrivEncMrCfgT) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        vp8_rtcd();
        vpx_dsp_rtcd();
        vpx_scale_rtcd();
        if (*ctx).priv_0.is_null() {
            let mut priv_0: *mut VpxCodecAlgPrivT = ::core::ptr::null_mut::<VpxCodecAlgPrivT>();
            if vp8_init_ctx(ctx) != 0 {
                return VPX_CODEC_MEM_ERROR;
            }
            priv_0 = (*ctx).priv_0 as *mut VpxCodecAlgPrivT;
            (*priv_0).fragments.count = 0 as u32;
            (*priv_0).fragments.enabled =
                ((*priv_0).base.init_flags & VPX_CODEC_USE_INPUT_FRAGMENTS as i64) != 0;
        }
        res
    }
}
unsafe fn vp8_destroy(mut ctx: *mut VpxCodecAlgPrivT) -> u32 {
    unsafe {
        vp8_remove_decoder_instances(&raw mut (*ctx).yv12_frame_buffers);
        let _ = Box::from_raw(ctx);
        VPX_CODEC_OK
    }
}
unsafe fn vp8_peek_si_internal(
    mut data: *const u8,
    mut data_sz: u32,
    mut si: *mut VpxCodecStreamInfoT,
    mut decrypt_cb: VpxDecryptCb,
    mut decrypt_state: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if data.is_null() {
            return VPX_CODEC_INVALID_PARAM;
        }
        if data.offset(data_sz as isize) <= data {
            res = VPX_CODEC_INVALID_PARAM;
        } else {
            let mut clear_buffer: [u8; 10] = [0; 10];
            let mut clear: *const u8 = data;
            if decrypt_cb.is_some() {
                let mut n: i32 =
                    (if (::core::mem::size_of::<[u8; 10]>() as usize) < data_sz as usize {
                        ::core::mem::size_of::<[u8; 10]>() as usize
                    } else {
                        data_sz as usize
                    }) as i32;
                decrypt_cb.expect("non-null function pointer")(
                    decrypt_state,
                    data as *const u8,
                    &raw mut clear_buffer as *mut u8,
                    n,
                );
                clear = &raw mut clear_buffer as *mut u8;
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
    mut data: *const u8,
    mut data_sz: u32,
    mut si: *mut VpxCodecStreamInfoT,
) -> u32 {
    unsafe { vp8_peek_si_internal(data, data_sz, si, None, NULL) }
}
unsafe fn vp8_get_si(mut ctx: *mut VpxCodecAlgPrivT, mut si: *mut VpxCodecStreamInfoT) -> u32 {
    unsafe {
        let mut sz: u32 = 0;
        if (*si).sz as usize >= ::core::mem::size_of::<Vp8StreamInfoT>() as usize {
            sz = ::core::mem::size_of::<Vp8StreamInfoT>() as u32;
        } else {
            sz = ::core::mem::size_of::<VpxCodecStreamInfoT>() as u32;
        }
        core::ptr::copy_nonoverlapping(
            &raw mut (*ctx).si as *const c_void as *const u8,
            si as *mut c_void as *mut u8,
            sz as usize,
        );
        (*si).sz = sz;
        VPX_CODEC_OK
    }
}
unsafe fn update_error_state(
    mut ctx: *mut VpxCodecAlgPrivT,
    mut error: *const VpxInternalErrorInfo,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        res = (*error).error_code;
        if res as u64 != 0 {
            (*ctx).base.err_detail = if (*error).has_detail {
                &raw const (*error).detail as *const i8
            } else {
                ::core::ptr::null::<i8>()
            };
        }
        res
    }
}
unsafe fn yuvconfig2image(
    mut img: *mut VpxImageT,
    mut yv12: *const Yv12BufferConfig,
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
        (*img).self_allocd = false;
    }
}
unsafe fn update_fragments(
    mut ctx: *mut VpxCodecAlgPrivT,
    mut data: *const u8,
    mut data_sz: u32,
    mut res: *mut u32,
) -> i32 {
    unsafe {
        ::core::ptr::write_volatile(res, VPX_CODEC_OK);
        if (*ctx).fragments.count == 0 as u32 {
            core::ptr::write_bytes(
                &raw mut (*ctx).fragments.ptrs as *mut *const u8 as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[*const u8; 9]>() as usize,
            );
            core::ptr::write_bytes(
                &raw mut (*ctx).fragments.sizes as *mut u32 as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[u32; 9]>() as usize,
            );
        }
        if (*ctx).fragments.enabled
            && data.is_null()
            && data_sz == 0 as u32
            && (*ctx).fragments.count == 0 as u32
        {
            return 0 as i32;
        }
        if (*ctx).fragments.enabled && !(data.is_null() && data_sz == 0 as u32) {
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
        if !(*ctx).fragments.enabled && (data.is_null() && data_sz == 0 as u32) {
            return 0 as i32;
        }
        if !(*ctx).fragments.enabled {
            (*ctx).fragments.ptrs[0 as usize] = data as *const u8;
            (*ctx).fragments.sizes[0 as usize] = data_sz;
            (*ctx).fragments.count = 1 as u32;
        }
        1 as i32
    }
}
unsafe fn vp8_decode(
    mut ctx: *mut VpxCodecAlgPrivT,
    mut data: *const u8,
    mut data_sz: u32,
    mut user_priv: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        let mut resolution_change: u32 = 0 as u32;
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        if !(*ctx).fragments.enabled && (data.is_null() && data_sz == 0 as u32) {
            return VPX_CODEC_OK;
        }
        if update_fragments(ctx, data, data_sz, &raw mut res) <= 0 as i32 {
            return res;
        }
        ::core::ptr::write_volatile(&mut w as *mut u32, (*ctx).si.w);
        ::core::ptr::write_volatile(&mut h as *mut u32, (*ctx).si.h);
        ::core::ptr::write_volatile(
            &mut res as *mut u32,
            vp8_peek_si_internal(
                (*ctx).fragments.ptrs[0 as usize],
                (*ctx).fragments.sizes[0 as usize],
                &raw mut (*ctx).si,
                (*ctx).decrypt_cb,
                (*ctx).decrypt_state,
            ),
        );
        if res as u32 == VPX_CODEC_UNSUP_BITSTREAM as u32 && (*ctx).si.is_kf == 0 {
            ::core::ptr::write_volatile(&mut res as *mut u32, VPX_CODEC_OK);
        }
        if !(*ctx).decoder_init && (*ctx).si.is_kf == 0 {
            ::core::ptr::write_volatile(&mut res as *mut u32, VPX_CODEC_UNSUP_BITSTREAM);
        }
        if res as u64 == 0
            && (*ctx).decoder_init
            && w == 0 as u32
            && h == 0 as u32
            && (*ctx).si.h == 0 as u32
            && (*ctx).si.w == 0 as u32
        {
            let mut pbi: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            ::core::ptr::write_volatile(&mut res as *mut u32, VPX_CODEC_CORRUPT_FRAME);
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
        if res as u64 == 0 && (*ctx).restart_threads {
            let mut pbi_0: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            let pc: *mut Vp8Common = &raw mut (*pbi_0).common;
            if setjmp(&raw mut (*pbi_0).common.error.jmp as *mut i32) != 0 {
                (*pbi_0).common.error.setjmp = false;
                vp8_decoder_remove_threads(pbi_0);
                return VPX_CODEC_ERROR;
            }
            (*pbi_0).common.error.setjmp = true;
            (*pbi_0).max_threads = (*ctx).cfg.threads as i32;
            vp8_decoder_create_threads(pbi_0);
            if vpx_atomic_load_acquire(&raw mut (*pbi_0).b_multithreaded_rd) != 0 {
                vp8mt_alloc_temp_buffers(pbi_0, (*pc).width, (*pc).mb_rows);
            }
            (*ctx).restart_threads = false;
            (*pbi_0).common.error.setjmp = false;
        }
        if res as u64 == 0 && !(*ctx).decoder_init {
            let mut oxcf: Vp8dConfig = Vp8dConfig {
                width: 0,
                height: 0,
                version: 0,
                postprocess: 0,
                max_threads: 0,
                error_concealment: 0,
            };
            oxcf.width = (*ctx).si.w as i32;
            oxcf.height = (*ctx).si.h as i32;
            oxcf.version = 9 as i32;
            oxcf.postprocess = 0 as i32;
            oxcf.max_threads = (*ctx).cfg.threads as i32;
            oxcf.error_concealment =
                ((*ctx).base.init_flags & VPX_CODEC_USE_ERROR_CONCEALMENT as i64) as i32;
            if !(*ctx).postproc_cfg_set
                && (*ctx).base.init_flags & VPX_CODEC_USE_POSTPROC as i64 != 0
            {
                (*ctx).postproc_cfg.post_proc_flag =
                    VP8_DEBLOCK as i32 | VP8_DEMACROBLOCK as i32 | VP8_MFQE as i32;
                (*ctx).postproc_cfg.deblocking_level = 4 as i32;
                (*ctx).postproc_cfg.noise_level = 0 as i32;
            }
            ::core::ptr::write_volatile(
                &mut res as *mut u32,
                vp8_create_decoder_instances(&raw mut (*ctx).yv12_frame_buffers, &raw mut oxcf)
                    as u32,
            );
            if res as u32 == VPX_CODEC_OK as u32 {
                (*ctx).decoder_init = true;
            } else {
                (*ctx).si.w = 0 as u32;
                (*ctx).si.h = 0 as u32;
            }
        }
        if (*ctx).decoder_init {
            (*(*ctx).yv12_frame_buffers.pbi[0 as usize]).decrypt_cb = (*ctx).decrypt_cb;
            (*(*ctx).yv12_frame_buffers.pbi[0 as usize]).decrypt_state = (*ctx).decrypt_state;
        }
        if res as u64 == 0 {
            let mut pbi_1: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize];
            let pc_0: *mut Vp8Common = &raw mut (*pbi_1).common;
            if resolution_change != 0 {
                let xd: *mut MACROBLOCKD = &raw mut (*pbi_1).mb;
                let mut i: i32 = 0;
                (*pc_0).width = (*ctx).si.w as i32;
                (*pc_0).height = (*ctx).si.h as i32;
                if setjmp(&raw mut (*pbi_1).common.error.jmp as *mut i32) != 0 {
                    (*pbi_1).common.error.setjmp = false;
                    (*ctx).si.w = 0 as u32;
                    (*ctx).si.h = 0 as u32;
                    return 4294967295 as u32;
                }
                (*pbi_1).common.error.setjmp = true;
                if (*pc_0).width <= 0 as i32 {
                    (*pc_0).width = w as i32;
                    vpx_internal_error(
                        &raw mut (*pc_0).error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Invalid frame width\0" as *const u8 as *const i8,
                    );
                }
                if (*pc_0).height <= 0 as i32 {
                    (*pc_0).height = h as i32;
                    vpx_internal_error(
                        &raw mut (*pc_0).error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Invalid frame height\0" as *const u8 as *const i8,
                    );
                }
                if vpx_atomic_load_acquire(&raw mut (*pbi_1).b_multithreaded_rd) != 0 {
                    vp8mt_de_alloc_temp_buffers(pbi_1, (*pc_0).mb_rows);
                }
                if vp8_alloc_frame_buffers(pc_0, (*pc_0).width, (*pc_0).height) != 0 {
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
                    vp8mt_alloc_temp_buffers(pbi_1, (*pc_0).width, 0 as i32);
                }
                (*pbi_1).common.error.setjmp = false;
                (*pbi_1).common.fb_idx_ref_cnt[0 as usize] = 0 as i32;
            }
            if setjmp(&raw mut (*pbi_1).common.error.jmp as *mut i32) != 0 {
                (*pc_0).yv12_fb[(*pc_0).lst_fb_idx as usize].corrupted = 1 as i32;
                if (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] > 0 as i32 {
                    (*pc_0).fb_idx_ref_cnt[(*pc_0).new_fb_idx as usize] -= 1;
                }
                (*pbi_1).common.error.setjmp = false;
                if (*pbi_1).restart_threads {
                    (*ctx).si.w = 0 as u32;
                    (*ctx).si.h = 0 as u32;
                    (*ctx).restart_threads = true;
                }
                ::core::ptr::write_volatile(
                    &mut res as *mut u32,
                    update_error_state(ctx, &raw mut (*pbi_1).common.error),
                );
                return res;
            }
            (*pbi_1).common.error.setjmp = true;
            (*pbi_1).fragments = (*ctx).fragments;
            (*pbi_1).restart_threads = false;
            (*ctx).user_priv = user_priv;
            if vp8dx_receive_compressed_data(pbi_1 as *mut Vp8dComp) != 0 {
                ::core::ptr::write_volatile(
                    &mut res as *mut u32,
                    update_error_state(ctx, &raw mut (*pbi_1).common.error),
                );
            }
            (*ctx).fragments.count = 0 as u32;
            (*pbi_1).common.error.setjmp = false;
        }
        res
    }
}
unsafe fn vp8_get_frame(
    mut ctx: *mut VpxCodecAlgPrivT,
    mut iter: *mut VpxCodecIterT,
) -> *mut VpxImageT {
    unsafe {
        let mut img: *mut VpxImageT = ::core::ptr::null_mut::<VpxImageT>();
        if (*iter).is_null() && !(*ctx).yv12_frame_buffers.pbi[0 as usize].is_null() {
            let mut sd: Yv12BufferConfig = Yv12BufferConfig {
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
                y_buffer: ::core::ptr::null_mut::<u8>(),
                u_buffer: ::core::ptr::null_mut::<u8>(),
                v_buffer: ::core::ptr::null_mut::<u8>(),
                alpha_buffer: ::core::ptr::null_mut::<u8>(),
                buffer_alloc: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_sz: 0,
                buffer_alloc_base: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_cap: 0,
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
            let mut flags: Vp8PpflagsT = Vp8PpflagsT {
                post_proc_flag: 0,
                deblocking_level: 0,
                noise_level: 0,
                display_ref_frame_flag: 0,
                display_mb_modes_flag: 0,
                display_b_modes_flag: 0,
                display_mv_flag: 0,
            };
            core::ptr::write_bytes(
                &raw mut flags as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<Vp8PpflagsT>() as usize,
            );
            if (*ctx).base.init_flags & VPX_CODEC_USE_POSTPROC as i64 != 0 {
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
                *iter = img as VpxCodecIterT;
            }
        }
        img
    }
}
unsafe fn image2yuvconfig(mut img: *const VpxImageT, mut yv12: *mut Yv12BufferConfig) -> u32 {
    unsafe {
        let y_w: i32 = (*img).d_w as i32;
        let y_h: i32 = (*img).d_h as i32;
        let uv_w: i32 = (*img).d_w.wrapping_add(1 as u32).wrapping_div(2 as u32) as i32;
        let uv_h: i32 = (*img).d_h.wrapping_add(1 as u32).wrapping_div(2 as u32) as i32;
        let mut res: u32 = VPX_CODEC_OK;
        (*yv12).y_buffer = (*img).planes[VPX_PLANE_Y as usize] as *mut u8;
        (*yv12).u_buffer = (*img).planes[VPX_PLANE_U as usize] as *mut u8;
        (*yv12).v_buffer = (*img).planes[VPX_PLANE_V as usize] as *mut u8;
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
unsafe fn vp8_set_reference(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut data: *mut VpxRefFrameT = data as *mut VpxRefFrameT;
        if !data.is_null() {
            let mut frame: *mut VpxRefFrameT = data;
            let mut sd: Yv12BufferConfig = Yv12BufferConfig {
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
                y_buffer: ::core::ptr::null_mut::<u8>(),
                u_buffer: ::core::ptr::null_mut::<u8>(),
                v_buffer: ::core::ptr::null_mut::<u8>(),
                alpha_buffer: ::core::ptr::null_mut::<u8>(),
                buffer_alloc: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_sz: 0,
                buffer_alloc_base: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_cap: 0,
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
                (*frame).frame_type as u32,
                &raw mut sd,
            )
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_reference(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut data: *mut VpxRefFrameT = data as *mut VpxRefFrameT;
        if !data.is_null() {
            let mut frame: *mut VpxRefFrameT = data;
            let mut sd: Yv12BufferConfig = Yv12BufferConfig {
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
                y_buffer: ::core::ptr::null_mut::<u8>(),
                u_buffer: ::core::ptr::null_mut::<u8>(),
                v_buffer: ::core::ptr::null_mut::<u8>(),
                alpha_buffer: ::core::ptr::null_mut::<u8>(),
                buffer_alloc: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_sz: 0,
                buffer_alloc_base: ::core::ptr::null_mut::<u8>(),
                buffer_alloc_cap: 0,
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
                (*frame).frame_type as u32,
                &raw mut sd,
            )
        } else {
            VPX_CODEC_INVALID_PARAM
        }
    }
}
unsafe fn vp8_get_quantizer(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let arg: *mut i32 = data as *mut i32;
        let mut pbi: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize];
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
fn vp8_set_postproc(_ctx: *mut VpxCodecAlgPrivT, _args: *mut c_void) -> u32 {
    VPX_CODEC_INCAPABLE
}
unsafe fn vp8_get_last_ref_updates(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut update_info: *mut i32 = data as *mut i32;
        if !update_info.is_null() {
            let mut pbi: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut Vp8dComp;
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
unsafe fn vp8_get_last_ref_frame(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut ref_info: *mut i32 = data as *mut i32;
        if !ref_info.is_null() {
            let mut pbi: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut Vp8dComp;
            if !pbi.is_null() {
                let mut oci: *mut Vp8Common = &raw mut (*pbi).common;
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
unsafe fn vp8_get_frame_corrupted(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut corrupted: *mut i32 = data as *mut i32;
        let mut pbi: *mut Vp8dComp = (*ctx).yv12_frame_buffers.pbi[0 as usize] as *mut Vp8dComp;
        if !corrupted.is_null() && !pbi.is_null() {
            let frame: *const Yv12BufferConfig = (*pbi).common.frame_to_show;
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
unsafe fn vp8_set_decryptor(mut ctx: *mut VpxCodecAlgPrivT, mut data: *mut c_void) -> u32 {
    unsafe {
        let mut init: *mut VpxDecryptInit = data as *mut VpxDecryptInit;
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
static mut vp8_ctf_maps: [VpxCodecCtrlFnMapT; 9] = {
    [
        VpxCodecCtrlFnMap {
            ctrl_id: VP8_SET_REFERENCE as i32,
            fn_0: Some(vp8_set_reference as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VP8_COPY_REFERENCE as i32,
            fn_0: Some(vp8_get_reference as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VP8_SET_POSTPROC as i32,
            fn_0: Some(vp8_set_postproc as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VP8D_GET_LAST_REF_UPDATES as i32,
            fn_0: Some(
                vp8_get_last_ref_updates as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32,
            ),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VP8D_GET_FRAME_CORRUPTED as i32,
            fn_0: Some(
                vp8_get_frame_corrupted as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32,
            ),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VP8D_GET_LAST_REF_USED as i32,
            fn_0: Some(
                vp8_get_last_ref_frame as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32,
            ),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VPXD_GET_LAST_QUANTIZER as i32,
            fn_0: Some(vp8_get_quantizer as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: VPXD_SET_DECRYPTOR as i32,
            fn_0: Some(vp8_set_decryptor as unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32),
        },
        VpxCodecCtrlFnMap {
            ctrl_id: -(1 as i32),
            fn_0: None,
        },
    ]
};
#[unsafe(no_mangle)]
pub static mut vpx_codec_vp8_dx_algo: VpxCodecIfaceT = VpxCodecIface {
    name: ::core::ptr::null::<i8>(),
    abi_version: 0,
    caps: 0,
    init: None,
    destroy: None,
    ctrl_maps: ::core::ptr::null::<VpxCodecCtrlFnMapT>(),
    dec: VpxCodecDecIface {
        peek_si: None,
        get_si: None,
        decode: None,
        get_frame: None,
        set_fb_fn: None,
    },
    enc: VpxCodecEncIface {
        cfg_map_count: 0,
        cfg_maps: ::core::ptr::null::<VpxCodecEncCfgMapT>(),
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
pub fn vpx_codec_vp8_dx() -> *const VpxCodecIfaceT {
    &raw const vpx_codec_vp8_dx_algo
}
pub const __ATOMIC_ACQUIRE: i32 = 2 as i32;
pub const NULL: *mut c_void = __DARWIN_NULL;
unsafe fn run_static_initializers() {
    unsafe {
        vpx_codec_vp8_dx_algo = VpxCodecIface {
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
                | VPX_CODEC_CAP_INPUT_FRAGMENTS) as i64,
            init: Some(vp8_init as unsafe fn(*mut VpxCodecCtxT, *mut VpxCodecPrivEncMrCfgT) -> u32),
            destroy: Some(vp8_destroy as unsafe fn(*mut VpxCodecAlgPrivT) -> u32),
            ctrl_maps: &raw const vp8_ctf_maps as *const VpxCodecCtrlFnMapT,
            dec: VpxCodecDecIface {
                peek_si: Some(
                    vp8_peek_si as unsafe fn(*const u8, u32, *mut VpxCodecStreamInfoT) -> u32,
                ),
                get_si: Some(
                    vp8_get_si as unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecStreamInfoT) -> u32,
                ),
                decode: Some(
                    vp8_decode
                        as unsafe fn(*mut VpxCodecAlgPrivT, *const u8, u32, *mut c_void) -> u32,
                ),
                get_frame: Some(
                    vp8_get_frame
                        as unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *mut VpxImageT,
                ),
                set_fb_fn: None,
            },
            enc: VpxCodecEncIface {
                cfg_map_count: 0 as i32,
                cfg_maps: ::core::ptr::null::<VpxCodecEncCfgMapT>(),
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
