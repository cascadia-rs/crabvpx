unsafe extern "C" {
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn vp8_loop_filter_init(cm: *mut VP8Common);
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vp8cx_init_de_quantizer(pbi: *mut VP8D_COMP);
    fn vp8_decode_frame(pbi: *mut VP8D_COMP) -> ::core::ffi::c_int;
    fn vpx_memalign(align: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
    fn vp8_create_common(oci: *mut VP8_COMMON);
    fn vp8_remove_common(oci: *mut VP8_COMMON);
    fn vp8_decoder_remove_threads(pbi: *mut VP8D_COMP);
    fn vp8_decoder_create_threads(pbi: *mut VP8D_COMP);
    fn vp8_init_intra_predictors();
    fn vpx_dsp_rtcd();
    fn vp8_yv12_copy_frame_c(src_ybc: *const yv12_buffer_config, dst_ybc: *mut yv12_buffer_config);
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
pub type __darwin_natural_t = ::core::ffi::c_uint;
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
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: C2RustUnnamed = 4;
pub const ALTREF_FRAME: C2RustUnnamed = 3;
pub const GOLDEN_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
pub type MACROBLOCKD = macroblockd;
pub type VP8_COMMON = VP8Common;
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
pub type vpx_ref_frame_type = ::core::ffi::c_uint;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
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
pub type semaphore_t = mach_port_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type pthread_t = __darwin_pthread_t;
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
pub type pthread_once_t = __darwin_pthread_once_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_buffers {
    pub pbi: [*mut VP8D_COMP; 32],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
pub const NUM_YV12_BUFFERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) { unsafe {
    static mut lock: pthread_once_t = _opaque_pthread_once_t {
        __sig: _PTHREAD_ONCE_SIG_init as ::core::ffi::c_long,
        __opaque: [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };
    pthread_once(&raw mut lock, func as Option<unsafe extern "C" fn() -> ()>);
}}
unsafe extern "C" fn initialize_dec() { unsafe {
    static mut init_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if init_done == 0 {
        vpx_dsp_rtcd();
        vp8_init_intra_predictors();
        ::core::ptr::write_volatile(
            &raw mut init_done as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}}
unsafe extern "C" fn remove_decompressor(mut pbi: *mut VP8D_COMP) { unsafe {
    vp8_remove_common(&raw mut (*pbi).common);
    vpx_free(pbi as *mut ::core::ffi::c_void);
}}
unsafe extern "C" fn create_decompressor(mut oxcf: *mut VP8D_CONFIG) -> *mut VP8D_COMP { unsafe {
    let mut pbi: *mut VP8D_COMP =
        vpx_memalign(32 as size_t, ::core::mem::size_of::<VP8D_COMP>() as size_t) as *mut VP8D_COMP;
    if pbi.is_null() {
        return ::core::ptr::null_mut::<VP8D_COMP>();
    }
    memset(
        pbi as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8D_COMP>() as size_t,
    );
    if setjmp(&raw mut (*pbi).common.error.jmp as *mut ::core::ffi::c_int) != 0 {
        (*pbi).common.error.setjmp = 0 as ::core::ffi::c_int;
        remove_decompressor(pbi);
        return ::core::ptr::null_mut::<VP8D_COMP>();
    }
    (*pbi).common.error.setjmp = 1 as ::core::ffi::c_int;
    vp8_create_common(&raw mut (*pbi).common);
    (*pbi).common.current_video_frame = 0 as ::core::ffi::c_uint;
    (*pbi).ready_for_new_data = 1 as ::core::ffi::c_int;
    vp8cx_init_de_quantizer(pbi);
    vp8_loop_filter_init(&raw mut (*pbi).common);
    (*pbi).common.error.setjmp = 0 as ::core::ffi::c_int;
    (*pbi).ec_enabled = 0 as ::core::ffi::c_int;
    (*pbi).ec_active = 0 as ::core::ffi::c_int;
    (*pbi).decoded_key_frame = 0 as ::core::ffi::c_int;
    (*pbi).independent_partitions = 0 as ::core::ffi::c_int;
    vp8_setup_block_dptrs(&raw mut (*pbi).mb);
    once(Some(initialize_dec as unsafe extern "C" fn() -> ()));
    return pbi as *mut VP8D_COMP;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut ref_fb_idx: ::core::ffi::c_int = 0;
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).lst_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).gld_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).alt_fb_idx;
    } else {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Invalid reference frame\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*pbi).common.error.error_code;
    }
    if (*cm).yv12_fb[ref_fb_idx as usize].y_height != (*sd).y_height
        || (*cm).yv12_fb[ref_fb_idx as usize].y_width != (*sd).y_width
        || (*cm).yv12_fb[ref_fb_idx as usize].uv_height != (*sd).uv_height
        || (*cm).yv12_fb[ref_fb_idx as usize].uv_width != (*sd).uv_width
    {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        vp8_yv12_copy_frame_c(
            (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(ref_fb_idx as isize)
                as *mut YV12_BUFFER_CONFIG,
            sd as *mut yv12_buffer_config,
        );
    }
    return (*pbi).common.error.error_code;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_set_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut ref_fb_ptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut free_fb: ::core::ffi::c_int = 0;
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).lst_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).gld_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).alt_fb_idx;
    } else {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Invalid reference frame\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*pbi).common.error.error_code;
    }
    if (*cm).yv12_fb[*ref_fb_ptr as usize].y_height != (*sd).y_height
        || (*cm).yv12_fb[*ref_fb_ptr as usize].y_width != (*sd).y_width
        || (*cm).yv12_fb[*ref_fb_ptr as usize].uv_height != (*sd).uv_height
        || (*cm).yv12_fb[*ref_fb_ptr as usize].uv_width != (*sd).uv_width
    {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        free_fb = get_free_fb(cm);
        (*cm).fb_idx_ref_cnt[free_fb as usize] -= 1;
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            ref_fb_ptr,
            free_fb,
        );
        vp8_yv12_copy_frame_c(
            sd,
            (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(*ref_fb_ptr as isize)
                as *mut yv12_buffer_config,
        );
    }
    return (*pbi).common.error.error_code;
}}
unsafe extern "C" fn get_free_fb(mut cm: *mut VP8_COMMON) -> ::core::ffi::c_int { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_YV12_BUFFERS {
        if (*cm).fb_idx_ref_cnt[i as usize] == 0 as ::core::ffi::c_int {
            break;
        }
        i += 1;
    }
    (*cm).fb_idx_ref_cnt[i as usize] = 1 as ::core::ffi::c_int;
    return i;
}}
unsafe extern "C" fn ref_cnt_fb(
    mut buf: *mut ::core::ffi::c_int,
    mut idx: *mut ::core::ffi::c_int,
    mut new_idx: ::core::ffi::c_int,
) { unsafe {
    if *buf.offset(*idx as isize) > 0 as ::core::ffi::c_int {
        let ref mut fresh0 = *buf.offset(*idx as isize);
        *fresh0 -= 1;
    }
    *idx = new_idx;
    let ref mut fresh1 = *buf.offset(new_idx as isize);
    *fresh1 += 1;
}}
unsafe extern "C" fn swap_frame_buffers(mut cm: *mut VP8_COMMON) -> ::core::ffi::c_int { unsafe {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cm).copy_buffer_to_arf != 0 {
        let mut new_fb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*cm).copy_buffer_to_arf == 1 as ::core::ffi::c_int {
            new_fb = (*cm).lst_fb_idx;
        } else if (*cm).copy_buffer_to_arf == 2 as ::core::ffi::c_int {
            new_fb = (*cm).gld_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).alt_fb_idx,
            new_fb,
        );
    }
    if (*cm).copy_buffer_to_gf != 0 {
        let mut new_fb_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*cm).copy_buffer_to_gf == 1 as ::core::ffi::c_int {
            new_fb_0 = (*cm).lst_fb_idx;
        } else if (*cm).copy_buffer_to_gf == 2 as ::core::ffi::c_int {
            new_fb_0 = (*cm).alt_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).gld_fb_idx,
            new_fb_0,
        );
    }
    if (*cm).refresh_golden_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).gld_fb_idx,
            (*cm).new_fb_idx,
        );
    }
    if (*cm).refresh_alt_ref_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).alt_fb_idx,
            (*cm).new_fb_idx,
        );
    }
    if (*cm).refresh_last_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).lst_fb_idx,
            (*cm).new_fb_idx,
        );
        (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
            .offset((*cm).lst_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    } else {
        (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
            .offset((*cm).new_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    }
    (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
    return err;
}}
unsafe extern "C" fn check_fragments_for_errors(mut pbi: *mut VP8D_COMP) -> ::core::ffi::c_int { unsafe {
    if (*pbi).ec_active == 0
        && (*pbi).fragments.count <= 1 as ::core::ffi::c_uint
        && (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_uint
    {
        let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
        if (*cm).fb_idx_ref_cnt[(*cm).lst_fb_idx as usize] > 1 as ::core::ffi::c_int {
            let prev_idx: ::core::ffi::c_int = (*cm).lst_fb_idx;
            (*cm).fb_idx_ref_cnt[prev_idx as usize] -= 1;
            (*cm).lst_fb_idx = get_free_fb(cm);
            vp8_yv12_copy_frame_c(
                (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(prev_idx as isize)
                    as *mut YV12_BUFFER_CONFIG,
                (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
                    .offset((*cm).lst_fb_idx as isize) as *mut yv12_buffer_config,
            );
        }
        (*cm).yv12_fb[(*cm).lst_fb_idx as usize].corrupted = 1 as ::core::ffi::c_int;
        (*cm).show_frame = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_receive_compressed_data(
    mut pbi: *mut VP8D_COMP,
) -> ::core::ffi::c_int { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut retcode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    (*pbi).common.error.error_code = VPX_CODEC_OK;
    retcode = check_fragments_for_errors(pbi);
    if retcode <= 0 as ::core::ffi::c_int {
        return retcode;
    }
    (*cm).new_fb_idx = get_free_fb(cm);
    (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).new_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[LAST_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).lst_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).gld_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[ALTREF_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).alt_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    retcode = vp8_decode_frame(pbi);
    if retcode < 0 as ::core::ffi::c_int {
        if (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] > 0 as ::core::ffi::c_int {
            (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
        }
        (*pbi).common.error.error_code = VPX_CODEC_ERROR;
        if (*pbi).mb.error_info.error_code as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            (*pbi).common.error.error_code = (*pbi).mb.error_info.error_code;
            memcpy(
                &raw mut (*pbi).common.error.detail as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw mut (*pbi).mb.error_info.detail as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            );
        }
    } else if swap_frame_buffers(cm) != 0 {
        (*pbi).common.error.error_code = VPX_CODEC_ERROR;
    } else {
        if (*cm).show_frame != 0 {
            (*cm).current_video_frame = (*cm).current_video_frame.wrapping_add(1);
            (*cm).show_frame_mi = (*cm).mi;
        }
        (*pbi).ready_for_new_data = 0 as ::core::ffi::c_int;
    }
    return retcode;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_raw_frame(
    mut pbi: *mut VP8D_COMP,
    mut sd: *mut YV12_BUFFER_CONFIG,
    mut flags: *mut vp8_ppflags_t,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*pbi).ready_for_new_data == 1 as ::core::ffi::c_int {
        return ret;
    }
    if (*pbi).common.show_frame == 0 as ::core::ffi::c_int {
        return ret;
    }
    (*pbi).ready_for_new_data = 1 as ::core::ffi::c_int;
    if !(*pbi).common.frame_to_show.is_null() {
        *sd = *(*pbi).common.frame_to_show;
        (*sd).y_width = (*pbi).common.Width;
        (*sd).y_height = (*pbi).common.Height;
        (*sd).uv_height = (*pbi).common.Height / 2 as ::core::ffi::c_int;
        ret = 0 as ::core::ffi::c_int;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_references_buffer(
    mut oci: *mut VP8_COMMON,
    mut ref_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut mi: *const MODE_INFO = (*oci).mi;
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < (*oci).mb_rows {
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*oci).mb_cols {
            if (*mi).mbmi.ref_frame as ::core::ffi::c_int == ref_frame {
                return 1 as ::core::ffi::c_int;
            }
            mb_col += 1;
            mi = mi.offset(1);
        }
        mi = mi.offset(1);
        mb_row += 1;
    }
    return 0 as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_create_decoder_instances(
    mut fb: *mut frame_buffers,
    mut oxcf: *mut VP8D_CONFIG,
) -> ::core::ffi::c_int { unsafe {
    (*fb).pbi[0 as ::core::ffi::c_int as usize] = create_decompressor(oxcf);
    if (*fb).pbi[0 as ::core::ffi::c_int as usize].is_null() {
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    if setjmp(
        &raw mut (**(&raw mut (*fb).pbi as *mut *mut VP8D_COMP)
            .offset(0 as ::core::ffi::c_int as isize))
        .common
        .error
        .jmp as *mut ::core::ffi::c_int,
    ) != 0
    {
        (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
            .common
            .error
            .setjmp = 0 as ::core::ffi::c_int;
        vp8_remove_decoder_instances(fb);
        memset(
            &raw mut (*fb).pbi as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut VP8D_COMP; 32]>() as size_t,
        );
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
        .common
        .error
        .setjmp = 1 as ::core::ffi::c_int;
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize]).max_threads = (*oxcf).max_threads;
    vp8_decoder_create_threads((*fb).pbi[0 as ::core::ffi::c_int as usize]);
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
        .common
        .error
        .setjmp = 0 as ::core::ffi::c_int;
    return VPX_CODEC_OK as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_remove_decoder_instances(
    mut fb: *mut frame_buffers,
) -> ::core::ffi::c_int { unsafe {
    let mut pbi: *mut VP8D_COMP = (*fb).pbi[0 as ::core::ffi::c_int as usize];
    if pbi.is_null() {
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    vp8_decoder_remove_threads(pbi);
    remove_decompressor(pbi);
    (*fb).pbi[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<VP8D_COMP>();
    return VPX_CODEC_OK as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_quantizer(mut pbi: *const VP8D_COMP) -> ::core::ffi::c_int { unsafe {
    return (*pbi).common.base_qindex;
}}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
