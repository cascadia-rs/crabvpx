use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_dc_only_idct_add_c(
        input_dc: i16,
        pred_ptr: *mut u8,
        pred_stride: i32,
        dst_ptr: *mut u8,
        dst_stride: i32,
    );
    fn vp8_dequant_idct_add_c(input: *mut i16, dq: *mut i16, dest: *mut u8, stride: i32);
    fn vp8_dequant_idct_add_uv_block_c(
        q: *mut i16,
        dq: *mut i16,
        dst_u: *mut u8,
        dst_v: *mut u8,
        stride: i32,
        eobs: *mut i8,
    );
    fn vp8_dequant_idct_add_y_block_c(
        q: *mut i16,
        dq: *mut i16,
        dst: *mut u8,
        stride: i32,
        eobs: *mut i8,
    );
    fn vp8_dequantize_b_c(_: *mut blockd, DQC: *mut i16);
    fn vp8_loop_filter_bh_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bv_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbh_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbv_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bhs_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_bvs_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_simple_horizontal_edge_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_simple_vertical_edge_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_short_inv_walsh4x4_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn vp8_short_inv_walsh4x4_1_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe fn(*mut c_void) -> *mut c_void>,
        _: *mut c_void,
    ) -> i32;
    fn pthread_join(_: pthread_t, _: *mut *mut c_void) -> i32;
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const i8,
    );
    fn vp8_loop_filter_frame_init(cm: *mut VP8Common, mbd: *mut macroblockd, default_filt_lvl: i32);
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
    // static mut mach_task_self_: mach_port_t;
    fn semaphore_signal(semaphore: semaphore_t) -> kern_return_t;
    fn semaphore_wait(semaphore: semaphore_t) -> kern_return_t;
    fn semaphore_create(
        task: task_t,
        semaphore: *mut semaphore_t,
        policy: i32,
        value: i32,
    ) -> kern_return_t;
    fn semaphore_destroy(task: task_t, semaphore: semaphore_t) -> kern_return_t;
    fn vp8_mb_init_dequantizer(pbi: *mut VP8D_COMP, xd: *mut MACROBLOCKD);
    fn vpx_memalign(align: size_t, size: size_t) -> *mut c_void;
    fn vpx_malloc(size: size_t) -> *mut c_void;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8_extend_mb_row(ybf: *mut YV12_BUFFER_CONFIG, YPtr: *mut u8, UPtr: *mut u8, VPtr: *mut u8);
    fn vp8_reset_mb_tokens_context(x: *mut MACROBLOCKD);
    fn vp8_decode_mb_tokens(_: *mut VP8D_COMP, _: *mut MACROBLOCKD) -> i32;
    fn vp8_intra4x4_predict(
        above: *mut u8,
        yleft: *mut u8,
        left_stride: i32,
        b_mode: B_PREDICTION_MODE,
        dst: *mut u8,
        dst_stride: i32,
        top_left: u8,
    );
    fn vp8_build_inter_predictors_mb(xd: *mut MACROBLOCKD);
    fn vp8_build_intra_predictors_mby_s(
        x: *mut MACROBLOCKD,
        yabove_row: *mut u8,
        yleft: *mut u8,
        left_stride: i32,
        ypred_ptr: *mut u8,
        y_stride: i32,
    );
    fn vp8_build_intra_predictors_mbuv_s(
        x: *mut MACROBLOCKD,
        uabove_row: *mut u8,
        vabove_row: *mut u8,
        uleft: *mut u8,
        vleft: *mut u8,
        left_stride: i32,
        upred_ptr: *mut u8,
        vpred_ptr: *mut u8,
        pred_stride: i32,
    );
    fn vp8_setup_intra_recon_top_line(ybf: *mut YV12_BUFFER_CONFIG);
}
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
pub type BLOCKD = blockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
pub type __darwin_natural_t = u32;
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
pub struct _opaque_pthread_attr_t {
    pub __sig: i64,
    pub __opaque: [i8; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: i64,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [i8; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type pthread_attr_t = *mut c_void;
pub type pthread_t = *mut c_void;
pub type mach_port_t = __darwin_mach_port_t;
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
pub struct FRAGMENT_DATA {
    pub enabled: i32,
    pub count: u32,
    pub ptrs: [*const u8; 9],
    pub sizes: [u32; 9],
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
pub type MB_PREDICTION_MODE = u32;
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
pub type MV_REFERENCE_FRAME = u32;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub type kern_return_t = i32;
pub type task_t = mach_port_t;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const THREAD_EXIT_SUCCESS: *mut c_void = NULL;
pub const CHAR_BIT: i32 = 8 as i32;
pub const VP8BORDERINPIXELS: i32 = 32 as i32;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<VP8_BD_VALUE>() as i32 * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: i32 = 0x40000000 as i32;
#[inline]
unsafe fn vp8dx_bool_error(mut br: *mut BOOL_DECODER) -> i32 {
    unsafe {
        if (*br).count > VP8_BD_VALUE_SIZE && (*br).count < VP8_LOTS_OF_BITS {
            return 1 as i32;
        }
        0 as i32
    }
}
pub const SYNC_POLICY_FIFO: i32 = 0 as i32;
#[inline]
unsafe fn vpx_atomic_init(mut atomic: *mut vpx_atomic_int, mut value: i32) {
    unsafe {
        ::core::ptr::write_volatile(&mut (*atomic).value as *mut i32, value);
    }
}
#[inline]
unsafe fn vpx_atomic_store_release(mut atomic: *mut vpx_atomic_int, mut value: i32) {
    unsafe {
        (*(&raw mut (*atomic).value as *const core::sync::atomic::AtomicI32))
            .store(value, core::sync::atomic::Ordering::Release);
    }
}
#[inline]
unsafe fn vpx_atomic_load_acquire(mut atomic: *const vpx_atomic_int) -> i32 {
    unsafe {
        (*(atomic as *const core::sync::atomic::AtomicI32))
            .load(core::sync::atomic::Ordering::Acquire)
    }
}
#[inline]
unsafe fn vp8_atomic_spin_wait(
    mut mb_col: i32,
    mut last_row_current_mb_col: *const vpx_atomic_int,
    nsync: i32,
) {
    unsafe {
        while mb_col > vpx_atomic_load_acquire(last_row_current_mb_col) - nsync {
            std::thread::yield_now();
        }
    }
}
#[inline]
unsafe fn intra_prediction_down_copy(mut xd: *mut MACROBLOCKD, mut above_right_src: *mut u8) {
    unsafe {
        let mut dst_stride: i32 = (*xd).dst.y_stride;
        let mut above_right_dst: *mut u8 = (*xd)
            .dst
            .y_buffer
            .offset(-(dst_stride as isize))
            .offset(16 as isize);
        let mut src_ptr: *mut u32 = above_right_src as *mut u32;
        let mut dst_ptr0: *mut u32 =
            above_right_dst.offset((4 as i32 * dst_stride) as isize) as *mut u32;
        let mut dst_ptr1: *mut u32 =
            above_right_dst.offset((8 as i32 * dst_stride) as isize) as *mut u32;
        let mut dst_ptr2: *mut u32 =
            above_right_dst.offset((12 as i32 * dst_stride) as isize) as *mut u32;
        *dst_ptr0 = *src_ptr;
        *dst_ptr1 = *src_ptr;
        *dst_ptr2 = *src_ptr;
    }
}
#[inline]
unsafe fn setup_intra_recon_left(
    mut y_buffer: *mut u8,
    mut u_buffer: *mut u8,
    mut v_buffer: *mut u8,
    mut y_stride: i32,
    mut uv_stride: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 16 as i32 {
            *y_buffer.offset((y_stride * i) as isize) = 129 as u8;
            i += 1;
        }
        i = 0 as i32;
        while i < 8 as i32 {
            *u_buffer.offset((uv_stride * i) as isize) = 129 as u8;
            i += 1;
        }
        i = 0 as i32;
        while i < 8 as i32 {
            *v_buffer.offset((uv_stride * i) as isize) = 129 as u8;
            i += 1;
        }
    }
}
unsafe fn setup_decoding_thread_data(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut mbrd: *mut MB_ROW_DEC,
    mut count: i32,
) {
    unsafe {
        let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < count {
            let mut mbd: *mut MACROBLOCKD = &raw mut (*mbrd.offset(i as isize)).mbd;
            (*mbd).subpixel_predict = (*xd).subpixel_predict;
            (*mbd).subpixel_predict8x4 = (*xd).subpixel_predict8x4;
            (*mbd).subpixel_predict8x8 = (*xd).subpixel_predict8x8;
            (*mbd).subpixel_predict16x16 = (*xd).subpixel_predict16x16;
            (*mbd).frame_type = (*pc).frame_type;
            (*mbd).pre = (*xd).pre;
            (*mbd).dst = (*xd).dst;
            (*mbd).segmentation_enabled = (*xd).segmentation_enabled;
            (*mbd).mb_segment_abs_delta = (*xd).mb_segment_abs_delta;
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).segment_feature_data as *mut [i8; 4] as *const c_void as *const u8,
                &raw mut (*mbd).segment_feature_data as *mut [i8; 4] as *mut c_void as *mut u8,
                ::core::mem::size_of::<[[i8; 4]; 2]>() as size_t,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).ref_lf_deltas as *mut i8 as *const c_void as *const u8,
                &raw mut (*mbd).ref_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i8; 4]>() as size_t,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).mode_lf_deltas as *mut i8 as *const c_void as *const u8,
                &raw mut (*mbd).mode_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i8; 4]>() as size_t,
            );
            (*mbd).mode_ref_lf_delta_enabled = (*xd).mode_ref_lf_delta_enabled;
            (*mbd).mode_ref_lf_delta_update = (*xd).mode_ref_lf_delta_update;
            (*mbd).current_bc = (&raw mut (*pbi).mbc as *mut vp8_reader).offset(0 as isize)
                as *mut vp8_reader as *mut c_void;
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y1_dc as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y1_dc as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as size_t,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y1 as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y1 as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as size_t,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y2 as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y2 as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as size_t,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_uv as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_uv as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as size_t,
            );
            (*mbd).fullpixel_mask = !(0 as i32);
            if (*pc).full_pixel != 0 {
                (*mbd).fullpixel_mask = !(7 as i32);
            }
            i += 1;
        }
        i = 0 as i32;
        while i < (*pc).mb_rows {
            vpx_atomic_store_release(
                (*pbi).mt_current_mb_col.offset(i as isize) as *mut vpx_atomic_int,
                -(1 as i32),
            );
            i += 1;
        }
    }
}
unsafe fn mt_decode_macroblock(mut pbi: *mut VP8D_COMP, mut xd: *mut MACROBLOCKD, _mb_idx: u32) {
    unsafe {
        let mut mode: MB_PREDICTION_MODE = DC_PRED;
        let mut i: i32 = 0;
        if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
            vp8_reset_mb_tokens_context(xd);
        } else if vp8dx_bool_error((*xd).current_bc as *mut BOOL_DECODER) == 0 {
            let mut eobtotal: i32 = 0;
            eobtotal = vp8_decode_mb_tokens(pbi, xd);
            (*(*xd).mode_info_context).mbmi.mb_skip_coeff = (eobtotal == 0 as i32) as uint8_t;
        }
        mode = (*(*xd).mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
        if (*xd).segmentation_enabled != 0 {
            vp8_mb_init_dequantizer(pbi, xd);
        }
        if (*(*xd).mode_info_context).mbmi.ref_frame as i32 == INTRA_FRAME as i32 {
            vp8_build_intra_predictors_mbuv_s(
                xd,
                (*xd).recon_above[1 as usize],
                (*xd).recon_above[2 as usize],
                (*xd).recon_left[1 as usize],
                (*xd).recon_left[2 as usize],
                (*xd).recon_left_stride[1 as usize],
                (*xd).dst.u_buffer as *mut u8,
                (*xd).dst.v_buffer as *mut u8,
                (*xd).dst.uv_stride,
            );
            if mode as u32 != B_PRED as u32 {
                vp8_build_intra_predictors_mby_s(
                    xd,
                    (*xd).recon_above[0 as usize],
                    (*xd).recon_left[0 as usize],
                    (*xd).recon_left_stride[0 as usize],
                    (*xd).dst.y_buffer as *mut u8,
                    (*xd).dst.y_stride,
                );
            } else {
                let mut DQC: *mut i16 = &raw mut (*xd).dequant_y1 as *mut i16;
                let mut dst_stride: i32 = (*xd).dst.y_stride;
                if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
                    core::ptr::write_bytes(
                        &raw mut (*xd).eobs as *mut i8 as *mut c_void as *mut u8,
                        0 as i32 as u8,
                        25 as size_t,
                    );
                }
                intra_prediction_down_copy(xd, (*xd).recon_above[0 as usize].offset(16 as isize));
                i = 0 as i32;
                while i < 16 as i32 {
                    let mut b: *mut BLOCKD =
                        (&raw mut (*xd).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                    let mut dst: *mut u8 = (*xd).dst.y_buffer.offset((*b).offset as isize);
                    let mut b_mode: B_PREDICTION_MODE =
                        (*(*xd).mode_info_context).bmi[i as usize].as_mode;
                    let mut Above: *mut u8 = ::core::ptr::null_mut::<u8>();
                    let mut yleft: *mut u8 = ::core::ptr::null_mut::<u8>();
                    let mut left_stride: i32 = 0;
                    let mut top_left: u8 = 0;
                    if i < 4 as i32 && (*pbi).common.filter_level != 0 {
                        Above = (*xd).recon_above[0 as usize].offset((*b).offset as isize);
                    } else {
                        Above = dst.offset(-(dst_stride as isize));
                    }
                    if i % 4 as i32 == 0 as i32 && (*pbi).common.filter_level != 0 {
                        yleft = (*xd).recon_left[0 as usize].offset(i as isize);
                        left_stride = 1 as i32;
                    } else {
                        yleft = dst.offset(-(1 as isize));
                        left_stride = dst_stride;
                    }
                    if (i == 4 as i32 || i == 8 as i32 || i == 12 as i32)
                        && (*pbi).common.filter_level != 0
                    {
                        top_left = *(*xd).recon_left[0 as usize]
                            .offset(i as isize)
                            .offset(-(1 as isize));
                    } else {
                        top_left = *Above.offset(-(1 as i32) as isize);
                    }
                    vp8_intra4x4_predict(
                        Above,
                        yleft,
                        left_stride,
                        b_mode,
                        dst,
                        dst_stride,
                        top_left,
                    );
                    if (*xd).eobs[i as usize] != 0 {
                        if (*xd).eobs[i as usize] as i32 > 1 as i32 {
                            vp8_dequant_idct_add_c((*b).qcoeff, DQC, dst, dst_stride);
                        } else {
                            vp8_dc_only_idct_add_c(
                                (*(*b).qcoeff.offset(0 as isize) as i32
                                    * *DQC.offset(0 as isize) as i32)
                                    as i16,
                                dst,
                                dst_stride,
                                dst,
                                dst_stride,
                            );
                            core::ptr::write_bytes(
                                (*b).qcoeff as *mut c_void as *mut u8,
                                0 as i32 as u8,
                                (2 as size_t).wrapping_mul(::core::mem::size_of::<i16>() as size_t),
                            );
                        }
                    }
                    i += 1;
                }
            }
        } else {
            vp8_build_inter_predictors_mb(xd);
        }
        if (*(*xd).mode_info_context).mbmi.mb_skip_coeff == 0 {
            if mode as u32 != B_PRED as u32 {
                let mut DQC_0: *mut i16 = &raw mut (*xd).dequant_y1 as *mut i16;
                if mode as u32 != SPLITMV as u32 {
                    let mut b_0: *mut BLOCKD =
                        (&raw mut (*xd).block as *mut BLOCKD).offset(24 as isize) as *mut BLOCKD;
                    if (*xd).eobs[24 as usize] as i32 > 1 as i32 {
                        vp8_dequantize_b_c(
                            b_0 as *mut blockd,
                            &raw mut (*xd).dequant_y2 as *mut i16,
                        );
                        vp8_short_inv_walsh4x4_c(
                            (*b_0).dqcoeff.offset(0 as isize) as *mut i16,
                            &raw mut (*xd).qcoeff as *mut i16,
                        );
                        core::ptr::write_bytes(
                            (*b_0).qcoeff as *mut c_void as *mut u8,
                            0 as i32 as u8,
                            (16 as size_t).wrapping_mul(::core::mem::size_of::<i16>() as size_t),
                        );
                    } else {
                        *(*b_0).dqcoeff.offset(0 as isize) = (*(*b_0).qcoeff.offset(0 as isize)
                            as i32
                            * (*xd).dequant_y2[0 as usize] as i32)
                            as i16;
                        vp8_short_inv_walsh4x4_1_c(
                            (*b_0).dqcoeff.offset(0 as isize) as *mut i16,
                            &raw mut (*xd).qcoeff as *mut i16,
                        );
                        core::ptr::write_bytes(
                            (*b_0).qcoeff as *mut c_void as *mut u8,
                            0 as i32 as u8,
                            (2 as size_t).wrapping_mul(::core::mem::size_of::<i16>() as size_t),
                        );
                    }
                    DQC_0 = &raw mut (*xd).dequant_y1_dc as *mut i16;
                }
                vp8_dequant_idct_add_y_block_c(
                    &raw mut (*xd).qcoeff as *mut i16,
                    DQC_0,
                    (*xd).dst.y_buffer as *mut u8,
                    (*xd).dst.y_stride,
                    &raw mut (*xd).eobs as *mut i8,
                );
            }
            vp8_dequant_idct_add_uv_block_c(
                (&raw mut (*xd).qcoeff as *mut i16).offset((16 as i32 * 16 as i32) as isize),
                &raw mut (*xd).dequant_uv as *mut i16,
                (*xd).dst.u_buffer as *mut u8,
                (*xd).dst.v_buffer as *mut u8,
                (*xd).dst.uv_stride,
                (&raw mut (*xd).eobs as *mut i8).offset(16 as isize),
            );
        }
    }
}
unsafe fn mt_decode_mb_rows(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut start_mb_row: i32,
) {
    unsafe {
        let mut last_row_current_mb_col: *const vpx_atomic_int =
            ::core::ptr::null::<vpx_atomic_int>();
        let mut current_mb_col: *mut vpx_atomic_int = ::core::ptr::null_mut::<vpx_atomic_int>();
        let mut mb_row: i32 = 0;
        let mut pc: *mut VP8_COMMON = &raw mut (*pbi).common;
        let nsync: i32 = (*pbi).sync_range;
        let first_row_no_sync_above: vpx_atomic_int = vpx_atomic_int {
            value: (*pc).mb_cols + nsync,
        };
        let mut num_part: i32 = (1 as i32) << (*pbi).common.multi_token_partition as u32;
        let mut last_mb_row: i32 = start_mb_row;
        let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        let mut yv12_fb_lst: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[LAST_FRAME as usize];
        let mut recon_y_stride: i32 = (*yv12_fb_new).y_stride;
        let mut recon_uv_stride: i32 = (*yv12_fb_new).uv_stride;
        let mut ref_buffer: [[*mut u8; 3]; 4] = [[::core::ptr::null_mut::<u8>(); 3]; 4];
        let mut dst_buffer: [*mut u8; 3] = [::core::ptr::null_mut::<u8>(); 3];
        let mut i: i32 = 0;
        let mut ref_fb_corrupted: [i32; 4] = [0; 4];
        ref_fb_corrupted[INTRA_FRAME as usize] = 0 as i32;
        i = 1 as i32;
        while i < MAX_REF_FRAMES as i32 {
            let mut this_fb: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[i as usize];
            ref_buffer[i as usize][0 as usize] = (*this_fb).y_buffer as *mut u8;
            ref_buffer[i as usize][1 as usize] = (*this_fb).u_buffer as *mut u8;
            ref_buffer[i as usize][2 as usize] = (*this_fb).v_buffer as *mut u8;
            ref_fb_corrupted[i as usize] = (*this_fb).corrupted;
            i += 1;
        }
        dst_buffer[0 as usize] = (*yv12_fb_new).y_buffer as *mut u8;
        dst_buffer[1 as usize] = (*yv12_fb_new).u_buffer as *mut u8;
        dst_buffer[2 as usize] = (*yv12_fb_new).v_buffer as *mut u8;
        (*xd).up_available = (start_mb_row != 0 as i32) as i32;
        (*xd).mode_info_context = (*pc)
            .mi
            .offset(((*pc).mode_info_stride * start_mb_row) as isize);
        (*xd).mode_info_stride = (*pc).mode_info_stride;
        mb_row = start_mb_row;
        while mb_row < (*pc).mb_rows {
            let mut recon_yoffset: i32 = 0;
            let mut recon_uvoffset: i32 = 0;
            let mut mb_col: i32 = 0;
            let mut filter_level: i32 = 0;
            let mut lfi_n: *mut loop_filter_info_n = &raw mut (*pc).lf_info;
            last_mb_row = mb_row;
            (*xd).current_bc = (&raw mut (*pbi).mbc as *mut vp8_reader)
                .offset((mb_row % num_part) as isize)
                as *mut vp8_reader as *mut c_void;
            if mb_row > 0 as i32 {
                last_row_current_mb_col = (*pbi)
                    .mt_current_mb_col
                    .offset((mb_row - 1 as i32) as isize)
                    as *mut vpx_atomic_int;
            } else {
                last_row_current_mb_col = &raw const first_row_no_sync_above;
            }
            current_mb_col =
                (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut vpx_atomic_int;
            recon_yoffset = mb_row * recon_y_stride * 16 as i32;
            recon_uvoffset = mb_row * recon_uv_stride * 8 as i32;
            (*xd).above_context = (*pc).above_context;
            core::ptr::write_bytes(
                (*xd).left_context as *mut c_void as *mut u8,
                0 as i32 as u8,
                ::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t,
            );
            (*xd).left_available = 0 as i32;
            (*xd).mb_to_top_edge = -((mb_row * 16 as i32) << 3 as i32);
            (*xd).mb_to_bottom_edge = (((*pc).mb_rows - 1 as i32 - mb_row) * 16 as i32) << 3 as i32;
            if (*pbi).common.filter_level != 0 {
                (*xd).recon_above[0 as usize] = (*(*pbi).mt_yabove_row.offset(mb_row as isize))
                    .offset((0 as i32 * 16 as i32) as isize)
                    .offset(32 as isize);
                (*xd).recon_above[1 as usize] = (*(*pbi).mt_uabove_row.offset(mb_row as isize))
                    .offset((0 as i32 * 8 as i32) as isize)
                    .offset(16 as isize);
                (*xd).recon_above[2 as usize] = (*(*pbi).mt_vabove_row.offset(mb_row as isize))
                    .offset((0 as i32 * 8 as i32) as isize)
                    .offset(16 as isize);
                (*xd).recon_left[0 as usize] = *(*pbi).mt_yleft_col.offset(mb_row as isize);
                (*xd).recon_left[1 as usize] = *(*pbi).mt_uleft_col.offset(mb_row as isize);
                (*xd).recon_left[2 as usize] = *(*pbi).mt_vleft_col.offset(mb_row as isize);
                (*xd).recon_left_stride[0 as usize] = 1 as i32;
                (*xd).recon_left_stride[1 as usize] = 1 as i32;
            } else {
                (*xd).recon_above[0 as usize] =
                    dst_buffer[0 as usize].offset(recon_yoffset as isize);
                (*xd).recon_above[1 as usize] =
                    dst_buffer[1 as usize].offset(recon_uvoffset as isize);
                (*xd).recon_above[2 as usize] =
                    dst_buffer[2 as usize].offset(recon_uvoffset as isize);
                (*xd).recon_left[0 as usize] = (*xd).recon_above[0 as usize].offset(-(1 as isize));
                (*xd).recon_left[1 as usize] = (*xd).recon_above[1 as usize].offset(-(1 as isize));
                (*xd).recon_left[2 as usize] = (*xd).recon_above[2 as usize].offset(-(1 as isize));
                (*xd).recon_above[0 as usize] =
                    (*xd).recon_above[0 as usize].offset(-((*xd).dst.y_stride as isize));
                (*xd).recon_above[1 as usize] =
                    (*xd).recon_above[1 as usize].offset(-((*xd).dst.uv_stride as isize));
                (*xd).recon_above[2 as usize] =
                    (*xd).recon_above[2 as usize].offset(-((*xd).dst.uv_stride as isize));
                (*xd).recon_left_stride[0 as usize] = (*xd).dst.y_stride;
                (*xd).recon_left_stride[1 as usize] = (*xd).dst.uv_stride;
                setup_intra_recon_left(
                    (*xd).recon_left[0 as usize],
                    (*xd).recon_left[1 as usize],
                    (*xd).recon_left[2 as usize],
                    (*xd).dst.y_stride,
                    (*xd).dst.uv_stride,
                );
            }
            mb_col = 0 as i32;
            while mb_col < (*pc).mb_cols {
                if (mb_col - 1 as i32) % nsync == 0 as i32 {
                    vpx_atomic_store_release(current_mb_col, mb_col - 1 as i32);
                }
                if mb_row != 0 && mb_col & (nsync - 1 as i32) == 0 {
                    vp8_atomic_spin_wait(mb_col, last_row_current_mb_col, nsync);
                }
                (*xd).mb_to_left_edge = -((mb_col * 16 as i32) << 3 as i32);
                (*xd).mb_to_right_edge =
                    (((*pc).mb_cols - 1 as i32 - mb_col) * 16 as i32) << 3 as i32;
                (*xd).dst.y_buffer =
                    dst_buffer[0 as usize].offset(recon_yoffset as isize) as *mut uint8_t;
                (*xd).dst.u_buffer =
                    dst_buffer[1 as usize].offset(recon_uvoffset as isize) as *mut uint8_t;
                (*xd).dst.v_buffer =
                    dst_buffer[2 as usize].offset(recon_uvoffset as isize) as *mut uint8_t;
                (*xd).corrupted |=
                    ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
                if (*xd).corrupted != 0 {
                    while mb_row < (*pc).mb_rows {
                        current_mb_col =
                            (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut vpx_atomic_int;
                        vpx_atomic_store_release(current_mb_col, (*pc).mb_cols + nsync);
                        mb_row = (mb_row as u32)
                            .wrapping_add((*pbi).decoding_thread_count.wrapping_add(1 as u32))
                            as i32;
                    }
                    vpx_internal_error(
                        &raw mut (*xd).error_info,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Corrupted reference frame\0" as *const u8 as *const i8,
                    );
                }
                if (*(*xd).mode_info_context).mbmi.ref_frame as i32 >= LAST_FRAME as i32 {
                    let ref_0: MV_REFERENCE_FRAME =
                        (*(*xd).mode_info_context).mbmi.ref_frame as MV_REFERENCE_FRAME;
                    (*xd).pre.y_buffer = ref_buffer[ref_0 as usize][0 as usize]
                        .offset(recon_yoffset as isize)
                        as *mut uint8_t;
                    (*xd).pre.u_buffer = ref_buffer[ref_0 as usize][1 as usize]
                        .offset(recon_uvoffset as isize)
                        as *mut uint8_t;
                    (*xd).pre.v_buffer = ref_buffer[ref_0 as usize][2 as usize]
                        .offset(recon_uvoffset as isize)
                        as *mut uint8_t;
                } else {
                    (*xd).pre.y_buffer = ::core::ptr::null_mut::<uint8_t>();
                    (*xd).pre.u_buffer = ::core::ptr::null_mut::<uint8_t>();
                    (*xd).pre.v_buffer = ::core::ptr::null_mut::<uint8_t>();
                }
                mt_decode_macroblock(pbi, xd, 0 as u32);
                (*xd).left_available = 1 as i32;
                (*xd).corrupted |= vp8dx_bool_error((*xd).current_bc as *mut BOOL_DECODER);
                (*xd).recon_above[0 as usize] = (*xd).recon_above[0 as usize].offset(16 as isize);
                (*xd).recon_above[1 as usize] = (*xd).recon_above[1 as usize].offset(8 as isize);
                (*xd).recon_above[2 as usize] = (*xd).recon_above[2 as usize].offset(8 as isize);
                if (*pbi).common.filter_level == 0 {
                    (*xd).recon_left[0 as usize] = (*xd).recon_left[0 as usize].offset(16 as isize);
                    (*xd).recon_left[1 as usize] = (*xd).recon_left[1 as usize].offset(8 as isize);
                    (*xd).recon_left[2 as usize] = (*xd).recon_left[2 as usize].offset(8 as isize);
                }
                if (*pbi).common.filter_level != 0 {
                    let mut skip_lf: i32 = ((*(*xd).mode_info_context).mbmi.mode as i32
                        != B_PRED as i32
                        && (*(*xd).mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                        && (*(*xd).mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                        as i32;
                    let mode_index: i32 =
                        (*lfi_n).mode_lf_lut[(*(*xd).mode_info_context).mbmi.mode as usize] as i32;
                    let seg: i32 = (*(*xd).mode_info_context).mbmi.segment_id as i32;
                    let ref_frame: i32 = (*(*xd).mode_info_context).mbmi.ref_frame as i32;
                    filter_level =
                        (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
                    if mb_row != (*pc).mb_rows - 1 as i32 {
                        core::ptr::copy_nonoverlapping(
                            (*xd)
                                .dst
                                .y_buffer
                                .offset((15 as i32 * recon_y_stride) as isize)
                                as *const c_void as *const u8,
                            (*(*pbi).mt_yabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset(32 as isize)
                                .offset((mb_col * 16 as i32) as isize)
                                as *mut c_void as *mut u8,
                            16 as size_t,
                        );
                        core::ptr::copy_nonoverlapping(
                            (*xd)
                                .dst
                                .u_buffer
                                .offset((7 as i32 * recon_uv_stride) as isize)
                                as *const c_void as *const u8,
                            (*(*pbi).mt_uabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset(16 as isize)
                                .offset((mb_col * 8 as i32) as isize)
                                as *mut c_void as *mut u8,
                            8 as size_t,
                        );
                        core::ptr::copy_nonoverlapping(
                            (*xd)
                                .dst
                                .v_buffer
                                .offset((7 as i32 * recon_uv_stride) as isize)
                                as *const c_void as *const u8,
                            (*(*pbi).mt_vabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset(16 as isize)
                                .offset((mb_col * 8 as i32) as isize)
                                as *mut c_void as *mut u8,
                            8 as size_t,
                        );
                    }
                    if mb_col != (*pc).mb_cols - 1 as i32 {
                        let mut next: *mut MODE_INFO = (*xd).mode_info_context.offset(1 as isize);
                        if (*next).mbmi.ref_frame as i32 == INTRA_FRAME as i32 {
                            i = 0 as i32;
                            while i < 16 as i32 {
                                *(*(*pbi).mt_yleft_col.offset(mb_row as isize))
                                    .offset(i as isize) = *(*xd)
                                    .dst
                                    .y_buffer
                                    .offset((i * recon_y_stride + 15 as i32) as isize)
                                    as u8;
                                i += 1;
                            }
                            i = 0 as i32;
                            while i < 8 as i32 {
                                *(*(*pbi).mt_uleft_col.offset(mb_row as isize))
                                    .offset(i as isize) = *(*xd)
                                    .dst
                                    .u_buffer
                                    .offset((i * recon_uv_stride + 7 as i32) as isize)
                                    as u8;
                                *(*(*pbi).mt_vleft_col.offset(mb_row as isize))
                                    .offset(i as isize) = *(*xd)
                                    .dst
                                    .v_buffer
                                    .offset((i * recon_uv_stride + 7 as i32) as isize)
                                    as u8;
                                i += 1;
                            }
                        }
                    }
                    if filter_level != 0 {
                        if (*pc).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
                            let mut lfi: loop_filter_info = loop_filter_info {
                                mblim: ::core::ptr::null::<u8>(),
                                blim: ::core::ptr::null::<u8>(),
                                lim: ::core::ptr::null::<u8>(),
                                hev_thr: ::core::ptr::null::<u8>(),
                            };
                            let mut frame_type: FRAME_TYPE = (*pc).frame_type;
                            let hev_index: i32 = (*lfi_n).hev_thr_lut[frame_type as usize]
                                [filter_level as usize]
                                as i32;
                            lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                .offset(filter_level as isize)
                                as *mut u8;
                            lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                .offset(filter_level as isize)
                                as *mut u8;
                            lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [u8; 16])
                                .offset(filter_level as isize)
                                as *mut u8;
                            lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr as *mut [u8; 16])
                                .offset(hev_index as isize)
                                as *mut u8;
                            if mb_col > 0 as i32 {
                                vp8_loop_filter_mbv_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    (*xd).dst.u_buffer as *mut u8,
                                    (*xd).dst.v_buffer as *mut u8,
                                    recon_y_stride,
                                    recon_uv_stride,
                                    &raw mut lfi,
                                );
                            }
                            if skip_lf == 0 {
                                vp8_loop_filter_bv_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    (*xd).dst.u_buffer as *mut u8,
                                    (*xd).dst.v_buffer as *mut u8,
                                    recon_y_stride,
                                    recon_uv_stride,
                                    &raw mut lfi,
                                );
                            }
                            if mb_row > 0 as i32 {
                                vp8_loop_filter_mbh_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    (*xd).dst.u_buffer as *mut u8,
                                    (*xd).dst.v_buffer as *mut u8,
                                    recon_y_stride,
                                    recon_uv_stride,
                                    &raw mut lfi,
                                );
                            }
                            if skip_lf == 0 {
                                vp8_loop_filter_bh_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    (*xd).dst.u_buffer as *mut u8,
                                    (*xd).dst.v_buffer as *mut u8,
                                    recon_y_stride,
                                    recon_uv_stride,
                                    &raw mut lfi,
                                );
                            }
                        } else {
                            if mb_col > 0 as i32 {
                                vp8_loop_filter_simple_vertical_edge_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    recon_y_stride,
                                    &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                        .offset(filter_level as isize)
                                        as *mut u8,
                                );
                            }
                            if skip_lf == 0 {
                                vp8_loop_filter_bvs_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    recon_y_stride,
                                    &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                        .offset(filter_level as isize)
                                        as *mut u8,
                                );
                            }
                            if mb_row > 0 as i32 {
                                vp8_loop_filter_simple_horizontal_edge_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    recon_y_stride,
                                    &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                        .offset(filter_level as isize)
                                        as *mut u8,
                                );
                            }
                            if skip_lf == 0 {
                                vp8_loop_filter_bhs_c(
                                    (*xd).dst.y_buffer as *mut u8,
                                    recon_y_stride,
                                    &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                        .offset(filter_level as isize)
                                        as *mut u8,
                                );
                            }
                        }
                    }
                }
                recon_yoffset += 16 as i32;
                recon_uvoffset += 8 as i32;
                (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
                (*xd).above_context = (*xd).above_context.offset(1);
                mb_col += 1;
            }
            if (*pbi).common.filter_level != 0 {
                if mb_row != (*pc).mb_rows - 1 as i32 {
                    let mut lasty: i32 = (*yv12_fb_lst).y_width + VP8BORDERINPIXELS;
                    let mut lastuv: i32 =
                        ((*yv12_fb_lst).y_width >> 1 as i32) + (VP8BORDERINPIXELS >> 1 as i32);
                    i = 0 as i32;
                    while i < 4 as i32 {
                        *(*(*pbi).mt_yabove_row.offset((mb_row + 1 as i32) as isize))
                            .offset((lasty + i) as isize) =
                            *(*(*pbi).mt_yabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset((lasty - 1 as i32) as isize);
                        *(*(*pbi).mt_uabove_row.offset((mb_row + 1 as i32) as isize))
                            .offset((lastuv + i) as isize) =
                            *(*(*pbi).mt_uabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset((lastuv - 1 as i32) as isize);
                        *(*(*pbi).mt_vabove_row.offset((mb_row + 1 as i32) as isize))
                            .offset((lastuv + i) as isize) =
                            *(*(*pbi).mt_vabove_row.offset((mb_row + 1 as i32) as isize))
                                .offset((lastuv - 1 as i32) as isize);
                        i += 1;
                    }
                }
            } else {
                vp8_extend_mb_row(
                    yv12_fb_new,
                    (*xd).dst.y_buffer.offset(16 as isize),
                    (*xd).dst.u_buffer.offset(8 as isize),
                    (*xd).dst.v_buffer.offset(8 as isize),
                );
            }
            vpx_atomic_store_release(current_mb_col, mb_col + nsync);
            (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
            (*xd).up_available = 1 as i32;
            (*xd).mode_info_context = (*xd).mode_info_context.offset(
                ((*xd).mode_info_stride as u32).wrapping_mul((*pbi).decoding_thread_count) as isize,
            );
            mb_row = (mb_row as u32)
                .wrapping_add((*pbi).decoding_thread_count.wrapping_add(1 as u32))
                as i32;
        }
        if last_mb_row + (*pbi).decoding_thread_count as i32 + 1 as i32 >= (*pc).mb_rows {
            crate::thread_shim::vp8_semaphore_signal((*pbi).h_event_end_decoding);
        }
    }
}
unsafe fn thread_decoding_proc(mut p_data: *mut c_void) -> *mut c_void {
    unsafe {
        let mut ithread: i32 = (*(p_data as *mut DECODETHREAD_DATA)).ithread;
        let mut pbi: *mut VP8D_COMP = (*(p_data as *mut DECODETHREAD_DATA)).ptr1 as *mut VP8D_COMP;
        let mut mbrd: *mut MB_ROW_DEC =
            (*(p_data as *mut DECODETHREAD_DATA)).ptr2 as *mut MB_ROW_DEC;
        let mut mb_row_left_context: ENTROPY_CONTEXT_PLANES = ENTROPY_CONTEXT_PLANES {
            y1: [0; 4],
            u: [0; 2],
            v: [0; 2],
            y2: 0,
        };
        while !(vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) == 0 as i32) {
            if !(crate::thread_shim::vp8_semaphore_wait(
                *(*pbi).h_event_start_decoding.offset(ithread as isize),
            ) == 0 as i32)
            {
                continue;
            }
            if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) == 0 as i32 {
                break;
            }
            let mut xd: *mut MACROBLOCKD = &raw mut (*mbrd).mbd;
            (*xd).left_context = &raw mut mb_row_left_context;
            if setjmp(&raw mut (*xd).error_info.jmp as *mut i32) != 0 {
                (*xd).error_info.setjmp = 0 as i32;
                crate::thread_shim::vp8_semaphore_signal((*pbi).h_event_end_decoding);
            } else {
                (*xd).error_info.setjmp = 1 as i32;
                mt_decode_mb_rows(pbi, xd, ithread + 1 as i32);
                (*xd).error_info.setjmp = 0 as i32;
            }
        }
        THREAD_EXIT_SUCCESS
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decoder_create_threads(mut pbi: *mut VP8D_COMP) {
    unsafe {
        let mut core_count: i32 = 0 as i32;
        let mut ithread: u32 = 0;
        vpx_atomic_init(&raw mut (*pbi).b_multithreaded_rd, 0 as i32);
        (*pbi).allocated_decoding_thread_count = 0 as i32;
        core_count = if (*pbi).max_threads > 8 as i32 {
            8 as i32
        } else {
            (*pbi).max_threads
        };
        if core_count > (*pbi).common.processor_core_count {
            core_count = (*pbi).common.processor_core_count;
        }
        if core_count > 1 as i32 {
            vpx_atomic_init(&raw mut (*pbi).b_multithreaded_rd, 1 as i32);
            (*pbi).decoding_thread_count = (core_count - 1 as i32) as u32;
            (*pbi).h_decoding_thread = vpx_calloc(
                ::core::mem::size_of::<pthread_t>() as size_t,
                (*pbi).decoding_thread_count as size_t,
            ) as *mut pthread_t;
            if (*pbi).h_decoding_thread.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->h_decoding_thread)\0" as *const u8 as *const i8,
                );
            }
            (*pbi).h_event_start_decoding = vpx_calloc(
                ::core::mem::size_of::<semaphore_t>() as size_t,
                (*pbi).decoding_thread_count as size_t,
            ) as *mut semaphore_t;
            if (*pbi).h_event_start_decoding.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->h_event_start_decoding)\0" as *const u8 as *const i8,
                );
            }
            (*pbi).mb_row_di = vpx_memalign(
                32 as size_t,
                (::core::mem::size_of::<MB_ROW_DEC>() as size_t)
                    .wrapping_mul((*pbi).decoding_thread_count as size_t),
            ) as *mut MB_ROW_DEC;
            if (*pbi).mb_row_di.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mb_row_di)\0" as *const u8 as *const i8,
                );
            }
            core::ptr::write_bytes(
                (*pbi).mb_row_di as *mut c_void as *mut u8,
                0 as i32 as u8,
                ((*pbi).decoding_thread_count as size_t)
                    .wrapping_mul(::core::mem::size_of::<MB_ROW_DEC>() as size_t),
            );
            (*pbi).de_thread_data = vpx_calloc(
                ::core::mem::size_of::<DECODETHREAD_DATA>() as size_t,
                (*pbi).decoding_thread_count as size_t,
            ) as *mut DECODETHREAD_DATA;
            if (*pbi).de_thread_data.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->de_thread_data)\0" as *const u8 as *const i8,
                );
            }
            if crate::thread_shim::vp8_semaphore_create(
                0 as task_t,
                &raw mut (*pbi).h_event_end_decoding,
                SYNC_POLICY_FIFO,
                0 as i32,
            ) != 0
            {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to initialize semaphore\0" as *const u8 as *const i8,
                );
            }
            ithread = 0 as u32;
            while ithread < (*pbi).decoding_thread_count {
                if crate::thread_shim::vp8_semaphore_create(
                    0 as task_t,
                    (*pbi).h_event_start_decoding.offset(ithread as isize) as *mut semaphore_t,
                    SYNC_POLICY_FIFO,
                    0 as i32,
                ) != 0
                {
                    break;
                }
                vp8_setup_block_dptrs(&raw mut (*(*pbi).mb_row_di.offset(ithread as isize)).mbd);
                (*(*pbi).de_thread_data.offset(ithread as isize)).ithread = ithread as i32;
                let fresh6 = &mut (*(*pbi).de_thread_data.offset(ithread as isize)).ptr1;
                *fresh6 = pbi as *mut c_void;
                let fresh7 = &mut (*(*pbi).de_thread_data.offset(ithread as isize)).ptr2;
                *fresh7 =
                    (*pbi).mb_row_di.offset(ithread as isize) as *mut MB_ROW_DEC as *mut c_void;
                if crate::thread_shim::vp8_pthread_create(
                    (*pbi).h_decoding_thread.offset(ithread as isize) as *mut pthread_t,
                    ::core::ptr::null::<c_void>(),
                    Some(thread_decoding_proc as unsafe fn(*mut c_void) -> *mut c_void),
                    (*pbi).de_thread_data.offset(ithread as isize) as *mut DECODETHREAD_DATA
                        as *mut c_void,
                ) != 0
                {
                    crate::thread_shim::vp8_semaphore_destroy(
                        0 as task_t,
                        *(*pbi).h_event_start_decoding.offset(ithread as isize),
                    );
                    break;
                } else {
                    ithread = ithread.wrapping_add(1);
                }
            }
            (*pbi).allocated_decoding_thread_count = ithread as i32;
            if (*pbi).allocated_decoding_thread_count != (*pbi).decoding_thread_count as i32 {
                if (*pbi).allocated_decoding_thread_count == 0 as i32 {
                    crate::thread_shim::vp8_semaphore_destroy(
                        0 as task_t,
                        (*pbi).h_event_end_decoding,
                    );
                }
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to create threads\0" as *const u8 as *const i8,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8mt_de_alloc_temp_buffers(mut pbi: *mut VP8D_COMP, mut mb_rows: i32) {
    unsafe {
        let mut i: i32 = 0;
        vpx_free((*pbi).mt_current_mb_col as *mut c_void);
        (*pbi).mt_current_mb_col = ::core::ptr::null_mut::<vpx_atomic_int>();
        if !(*pbi).mt_yabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_yabove_row.offset(i as isize) as *mut c_void);
                let fresh0 = &mut *(*pbi).mt_yabove_row.offset(i as isize);
                *fresh0 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_yabove_row as *mut c_void);
            (*pbi).mt_yabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_uabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_uabove_row.offset(i as isize) as *mut c_void);
                let fresh1 = &mut *(*pbi).mt_uabove_row.offset(i as isize);
                *fresh1 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_uabove_row as *mut c_void);
            (*pbi).mt_uabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_vabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_vabove_row.offset(i as isize) as *mut c_void);
                let fresh2 = &mut *(*pbi).mt_vabove_row.offset(i as isize);
                *fresh2 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_vabove_row as *mut c_void);
            (*pbi).mt_vabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_yleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_yleft_col.offset(i as isize) as *mut c_void);
                let fresh3 = &mut *(*pbi).mt_yleft_col.offset(i as isize);
                *fresh3 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_yleft_col as *mut c_void);
            (*pbi).mt_yleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_uleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_uleft_col.offset(i as isize) as *mut c_void);
                let fresh4 = &mut *(*pbi).mt_uleft_col.offset(i as isize);
                *fresh4 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_uleft_col as *mut c_void);
            (*pbi).mt_uleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_vleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                vpx_free(*(*pbi).mt_vleft_col.offset(i as isize) as *mut c_void);
                let fresh5 = &mut *(*pbi).mt_vleft_col.offset(i as isize);
                *fresh5 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            vpx_free((*pbi).mt_vleft_col as *mut c_void);
            (*pbi).mt_vleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8mt_alloc_temp_buffers(
    mut pbi: *mut VP8D_COMP,
    mut width: i32,
    mut prev_mb_rows: i32,
) {
    unsafe {
        let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut i: i32 = 0;
        let mut uv_width: i32 = 0;
        if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0 {
            vp8mt_de_alloc_temp_buffers(pbi, prev_mb_rows);
            if width & 0xf as i32 != 0 as i32 {
                width += 16 as i32 - (width & 0xf as i32);
            }
            if width < 640 as i32 {
                (*pbi).sync_range = 1 as i32;
            } else if width <= 1280 as i32 {
                (*pbi).sync_range = 8 as i32;
            } else if width <= 2560 as i32 {
                (*pbi).sync_range = 16 as i32;
            } else {
                (*pbi).sync_range = 32 as i32;
            }
            uv_width = width >> 1 as i32;
            (*pbi).mt_current_mb_col = vpx_malloc(
                (::core::mem::size_of::<vpx_atomic_int>() as size_t)
                    .wrapping_mul((*pc).mb_rows as size_t),
            ) as *mut vpx_atomic_int;
            if (*pbi).mt_current_mb_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_current_mb_col\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                vpx_atomic_init(
                    (*pbi).mt_current_mb_col.offset(i as isize) as *mut vpx_atomic_int,
                    0 as i32,
                );
                i += 1;
            }
            (*pbi).mt_yabove_row = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_yabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_yabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh8 = &mut *(*pbi).mt_yabove_row.offset(i as isize);
                *fresh8 = vpx_memalign(
                    16 as size_t,
                    (::core::mem::size_of::<u8>() as size_t)
                        .wrapping_mul((width + ((32 as i32) << 1 as i32)) as size_t),
                ) as *mut u8;
                if (*(*pbi).mt_yabove_row.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_yabove_row[i]\0" as *const u8 as *const i8,
                    );
                }
                core::ptr::write_bytes(
                    *(*pbi).mt_yabove_row.offset(i as isize) as *mut c_void as *mut u8,
                    0 as i32 as u8,
                    ((width + ((32 as i32) << 1 as i32)) as size_t)
                        .wrapping_mul(::core::mem::size_of::<u8>() as size_t),
                );
                i += 1;
            }
            (*pbi).mt_uabove_row = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_uabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_uabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh9 = &mut *(*pbi).mt_uabove_row.offset(i as isize);
                *fresh9 = vpx_memalign(
                    16 as size_t,
                    (::core::mem::size_of::<u8>() as size_t)
                        .wrapping_mul((uv_width + 32 as i32) as size_t),
                ) as *mut u8;
                if (*(*pbi).mt_uabove_row.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_uabove_row[i]\0" as *const u8 as *const i8,
                    );
                }
                core::ptr::write_bytes(
                    *(*pbi).mt_uabove_row.offset(i as isize) as *mut c_void as *mut u8,
                    0 as i32 as u8,
                    ((uv_width + 32 as i32) as size_t)
                        .wrapping_mul(::core::mem::size_of::<u8>() as size_t),
                );
                i += 1;
            }
            (*pbi).mt_vabove_row = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_vabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_vabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh10 = &mut *(*pbi).mt_vabove_row.offset(i as isize);
                *fresh10 = vpx_memalign(
                    16 as size_t,
                    (::core::mem::size_of::<u8>() as size_t)
                        .wrapping_mul((uv_width + 32 as i32) as size_t),
                ) as *mut u8;
                if (*(*pbi).mt_vabove_row.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_vabove_row[i]\0" as *const u8 as *const i8,
                    );
                }
                core::ptr::write_bytes(
                    *(*pbi).mt_vabove_row.offset(i as isize) as *mut c_void as *mut u8,
                    0 as i32 as u8,
                    ((uv_width + 32 as i32) as size_t)
                        .wrapping_mul(::core::mem::size_of::<u8>() as size_t),
                );
                i += 1;
            }
            (*pbi).mt_yleft_col = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_yleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_yleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh11 = &mut *(*pbi).mt_yleft_col.offset(i as isize);
                *fresh11 = vpx_calloc(
                    (::core::mem::size_of::<u8>() as size_t).wrapping_mul(16 as size_t),
                    1 as size_t,
                ) as *mut u8;
                if (*(*pbi).mt_yleft_col.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_yleft_col[i]\0" as *const u8 as *const i8,
                    );
                }
                i += 1;
            }
            (*pbi).mt_uleft_col = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_uleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_uleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh12 = &mut *(*pbi).mt_uleft_col.offset(i as isize);
                *fresh12 = vpx_calloc(
                    (::core::mem::size_of::<u8>() as size_t).wrapping_mul(8 as size_t),
                    1 as size_t,
                ) as *mut u8;
                if (*(*pbi).mt_uleft_col.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_uleft_col[i]\0" as *const u8 as *const i8,
                    );
                }
                i += 1;
            }
            (*pbi).mt_vleft_col = vpx_calloc(
                ::core::mem::size_of::<*mut u8>() as size_t,
                (*pc).mb_rows as size_t,
            ) as *mut *mut u8;
            if (*pbi).mt_vleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_vleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            while i < (*pc).mb_rows {
                let fresh13 = &mut *(*pbi).mt_vleft_col.offset(i as isize);
                *fresh13 = vpx_calloc(
                    (::core::mem::size_of::<u8>() as size_t).wrapping_mul(8 as size_t),
                    1 as size_t,
                ) as *mut u8;
                if (*(*pbi).mt_vleft_col.offset(i as isize)).is_null() {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_MEM_ERROR,
                        b"Failed to allocate pbi->mt_vleft_col[i]\0" as *const u8 as *const i8,
                    );
                }
                i += 1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decoder_remove_threads(mut pbi: *mut VP8D_COMP) {
    unsafe {
        if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0 {
            let mut i: i32 = 0;
            vpx_atomic_store_release(&raw mut (*pbi).b_multithreaded_rd, 0 as i32);
            i = 0 as i32;
            while i < (*pbi).allocated_decoding_thread_count {
                crate::thread_shim::vp8_semaphore_signal(
                    *(*pbi).h_event_start_decoding.offset(i as isize),
                );
                crate::thread_shim::vp8_pthread_join(
                    *(*pbi).h_decoding_thread.offset(i as isize) as pthread_t,
                    ::core::ptr::null_mut::<*mut c_void>(),
                );
                i += 1;
            }
            i = 0 as i32;
            while i < (*pbi).allocated_decoding_thread_count {
                crate::thread_shim::vp8_semaphore_destroy(
                    0 as task_t,
                    *(*pbi).h_event_start_decoding.offset(i as isize),
                );
                i += 1;
            }
            if (*pbi).allocated_decoding_thread_count != 0 {
                crate::thread_shim::vp8_semaphore_destroy(0 as task_t, (*pbi).h_event_end_decoding);
            }
            vpx_free((*pbi).h_decoding_thread as *mut c_void);
            (*pbi).h_decoding_thread = ::core::ptr::null_mut::<pthread_t>();
            vpx_free((*pbi).h_event_start_decoding as *mut c_void);
            (*pbi).h_event_start_decoding = ::core::ptr::null_mut::<semaphore_t>();
            vpx_free((*pbi).mb_row_di as *mut c_void);
            (*pbi).mb_row_di = ::core::ptr::null_mut::<MB_ROW_DEC>();
            vpx_free((*pbi).de_thread_data as *mut c_void);
            (*pbi).de_thread_data = ::core::ptr::null_mut::<DECODETHREAD_DATA>();
            vp8mt_de_alloc_temp_buffers(pbi, (*pbi).common.mb_rows);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8mt_decode_mb_rows(mut pbi: *mut VP8D_COMP, mut xd: *mut MACROBLOCKD) -> i32 {
    unsafe {
        let mut pc: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut i: u32 = 0;
        let mut j: i32 = 0;
        let mut filter_level: i32 = (*pc).filter_level;
        let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        if filter_level != 0 {
            core::ptr::write_bytes(
                (*(*pbi).mt_yabove_row.offset(0 as isize))
                    .offset(VP8BORDERINPIXELS as isize)
                    .offset(-(1 as isize)) as *mut c_void as *mut u8,
                127 as i32 as u8,
                ((*yv12_fb_new).y_width + 5 as i32) as size_t,
            );
            core::ptr::write_bytes(
                ((*(*pbi).mt_uabove_row.offset(0 as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                    .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                (127 as i32) as u8,
                ((((*yv12_fb_new).y_width >> 1 as i32) + 5 as i32) as size_t) as usize,
            );
            core::ptr::write_bytes(
                ((*(*pbi).mt_vabove_row.offset(0 as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                    .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                (127 as i32) as u8,
                ((((*yv12_fb_new).y_width >> 1 as i32) + 5 as i32) as size_t) as usize,
            );
            j = 1 as i32;
            while j < (*pc).mb_rows {
                core::ptr::write_bytes(
                    ((*(*pbi).mt_yabove_row.offset(j as isize))
                        .offset(VP8BORDERINPIXELS as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as size_t) as usize,
                );
                core::ptr::write_bytes(
                    ((*(*pbi).mt_uabove_row.offset(j as isize))
                        .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as size_t) as usize,
                );
                core::ptr::write_bytes(
                    ((*(*pbi).mt_vabove_row.offset(j as isize))
                        .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as size_t) as usize,
                );
                j += 1;
            }
            j = 0 as i32;
            while j < (*pc).mb_rows {
                core::ptr::write_bytes(
                    (*(*pbi).mt_yleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (16 as size_t) as usize,
                );
                core::ptr::write_bytes(
                    (*(*pbi).mt_uleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (8 as size_t) as usize,
                );
                core::ptr::write_bytes(
                    (*(*pbi).mt_vleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (8 as size_t) as usize,
                );
                j += 1;
            }
            vp8_loop_filter_frame_init(pc as *mut VP8Common, &raw mut (*pbi).mb, filter_level);
        } else {
            vp8_setup_intra_recon_top_line(yv12_fb_new);
        }
        setup_decoding_thread_data(
            pbi,
            xd,
            (*pbi).mb_row_di,
            (*pbi).decoding_thread_count as i32,
        );
        i = 0 as u32;
        while i < (*pbi).decoding_thread_count {
            crate::thread_shim::vp8_semaphore_signal(
                *(*pbi).h_event_start_decoding.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        if setjmp(&raw mut (*xd).error_info.jmp as *mut i32) != 0 {
            (*xd).error_info.setjmp = 0 as i32;
            (*xd).corrupted = 1 as i32;
            i = 0 as u32;
            while i < (*pbi).decoding_thread_count {
                crate::thread_shim::vp8_semaphore_wait((*pbi).h_event_end_decoding);
                i = i.wrapping_add(1);
            }
            return -(1 as i32);
        }
        (*xd).error_info.setjmp = 1 as i32;
        mt_decode_mb_rows(pbi, xd, 0 as i32);
        (*xd).error_info.setjmp = 0 as i32;
        i = 0 as u32;
        while i < (*pbi).decoding_thread_count.wrapping_add(1 as u32) {
            crate::thread_shim::vp8_semaphore_wait((*pbi).h_event_end_decoding);
            i = i.wrapping_add(1);
        }
        0 as i32
    }
}
pub const __ATOMIC_ACQUIRE: i32 = 2 as i32;
pub const __ATOMIC_RELEASE: i32 = 3 as i32;
pub const NULL: *mut c_void = __DARWIN_NULL;
