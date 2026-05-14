extern "C" {
    fn vp8_bilinear_predict16x16_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict4x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict8x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict8x8_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_dc_only_idct_add_neon(
        input_dc: ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_neon(
        input: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dest: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_uv_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst_u: *mut ::core::ffi::c_uchar,
        dst_v: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequant_idct_add_y_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequantize_b_neon(_: *mut blockd, DQC: *mut ::core::ffi::c_short);
    fn vp8_short_inv_walsh4x4_neon(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn vp8_short_inv_walsh4x4_1_c(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn vp8_sixtap_predict16x16_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict4x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict8x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict8x8_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_yv12_extend_frame_borders_c(ybf: *mut yv12_buffer_config);
    static vp8_norm: [::core::ffi::c_uchar; 256];
    fn vp8dx_start_decode(
        br: *mut BOOL_DECODER,
        source: *const ::core::ffi::c_uchar,
        source_sz: ::core::ffi::c_uint,
        decrypt_cb: vpx_decrypt_cb,
        decrypt_state: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn vp8dx_bool_decoder_fill(br: *mut BOOL_DECODER);
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn vp8_loop_filter_frame_init(
        cm: *mut VP8Common,
        mbd: *mut macroblockd,
        default_filt_lvl: ::core::ffi::c_int,
    );
    fn vp8_loop_filter_row_normal(
        cm: *mut VP8Common,
        mode_info_context: *mut modeinfo,
        mb_row: ::core::ffi::c_int,
        post_ystride: ::core::ffi::c_int,
        post_uvstride: ::core::ffi::c_int,
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_row_simple(
        cm: *mut VP8Common,
        mode_info_context: *mut modeinfo,
        mb_row: ::core::ffi::c_int,
        post_ystride: ::core::ffi::c_int,
        y_ptr: *mut ::core::ffi::c_uchar,
    );
    static vp8_default_mv_context: [MV_CONTEXT; 2];
    static vp8_coef_update_probs: [[[[vp8_prob; 11]; 3]; 8]; 4];
    fn vp8_default_coef_probs(_: *mut VP8Common);
    static vp8_mb_feature_data_bits: [::core::ffi::c_int; 2];
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
    fn vp8_intra4x4_predict(
        above: *mut ::core::ffi::c_uchar,
        yleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        b_mode: B_PREDICTION_MODE,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
        top_left: ::core::ffi::c_uchar,
    );
    fn vp8_build_inter_predictors_mb(xd: *mut MACROBLOCKD);
    fn vp8_reset_mb_tokens_context(x: *mut MACROBLOCKD);
    fn vp8_decode_mb_tokens(_: *mut VP8D_COMP, _: *mut MACROBLOCKD) -> ::core::ffi::c_int;
    fn vp8_setup_version(cm: *mut VP8_COMMON);
    fn vp8_init_mbmode_probs(x: *mut VP8_COMMON);
    fn vp8_ac_yquant(QIndex: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vp8_dc_quant(QIndex: ::core::ffi::c_int, Delta: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vp8_dc2quant(QIndex: ::core::ffi::c_int, Delta: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vp8_ac2quant(QIndex: ::core::ffi::c_int, Delta: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vp8_dc_uv_quant(QIndex: ::core::ffi::c_int, Delta: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn vp8_ac_uv_quant(QIndex: ::core::ffi::c_int, Delta: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn vp8_build_intra_predictors_mby_s(
        x: *mut MACROBLOCKD,
        yabove_row: *mut ::core::ffi::c_uchar,
        yleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        ypred_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
    );
    fn vp8_build_intra_predictors_mbuv_s(
        x: *mut MACROBLOCKD,
        uabove_row: *mut ::core::ffi::c_uchar,
        vabove_row: *mut ::core::ffi::c_uchar,
        uleft: *mut ::core::ffi::c_uchar,
        vleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        upred_ptr: *mut ::core::ffi::c_uchar,
        vpred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
    );
    fn vp8_setup_intra_recon_top_line(ybf: *mut YV12_BUFFER_CONFIG);
    fn vp8_decode_mode_mvs(_: *mut VP8D_COMP);
    fn vp8_extend_mb_row(
        ybf: *mut YV12_BUFFER_CONFIG,
        YPtr: *mut ::core::ffi::c_uchar,
        UPtr: *mut ::core::ffi::c_uchar,
        VPtr: *mut ::core::ffi::c_uchar,
    );
    fn vp8mt_decode_mb_rows(pbi: *mut VP8D_COMP, xd: *mut MACROBLOCKD) -> ::core::ffi::c_int;
    fn vp8_decoder_remove_threads(pbi: *mut VP8D_COMP);
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
pub type __darwin_ptrdiff_t = isize;
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
pub type pthread_t = __darwin_pthread_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_LVL_MAX: C2RustUnnamed = 2;
pub const MB_LVL_ALT_LF: C2RustUnnamed = 1;
pub const MB_LVL_ALT_Q: C2RustUnnamed = 0;
pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
unsafe extern "C" fn vp8dx_decode_bool(
    mut br: *mut BOOL_DECODER,
    mut probability: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bit: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut value: VP8_BD_VALUE = 0;
    let mut split: ::core::ffi::c_uint = 0;
    let mut bigsplit: VP8_BD_VALUE = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut range: ::core::ffi::c_uint = 0;
    split = (1 as ::core::ffi::c_uint).wrapping_add(
        (*br)
            .range
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_mul(probability as ::core::ffi::c_uint)
            >> 8 as ::core::ffi::c_int,
    );
    if (*br).count < 0 as ::core::ffi::c_int {
        vp8dx_bool_decoder_fill(br);
    }
    value = (*br).value;
    count = (*br).count;
    bigsplit = (split as VP8_BD_VALUE) << VP8_BD_VALUE_SIZE - 8 as ::core::ffi::c_int;
    range = split;
    if value >= bigsplit {
        range = (*br).range.wrapping_sub(split);
        value = value.wrapping_sub(bigsplit);
        bit = 1 as ::core::ffi::c_uint;
    }
    let shift: ::core::ffi::c_uchar = vp8_norm[range as ::core::ffi::c_uchar as usize];
    range <<= shift as ::core::ffi::c_int;
    value <<= shift as ::core::ffi::c_int;
    count -= shift as ::core::ffi::c_int;
    (*br).value = value;
    (*br).count = count;
    (*br).range = range;
    return bit as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn vp8_decode_value(
    mut br: *mut BOOL_DECODER,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bit: ::core::ffi::c_int = 0;
    bit = bits - 1 as ::core::ffi::c_int;
    while bit >= 0 as ::core::ffi::c_int {
        z |= vp8dx_decode_bool(br, 0x80 as ::core::ffi::c_int) << bit;
        bit -= 1;
    }
    return z;
}
#[inline]
unsafe extern "C" fn vp8dx_bool_error(mut br: *mut BOOL_DECODER) -> ::core::ffi::c_int {
    if (*br).count > VP8_BD_VALUE_SIZE && (*br).count < VP8_LOTS_OF_BITS {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const MB_FEATURE_TREE_PROBS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_MB_SEGMENTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_REF_LF_DELTAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_MODE_LF_DELTAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEGMENT_DELTADATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEGMENT_ABSDATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENTROPY_NODES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const BLOCK_TYPES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COEF_BANDS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PREV_COEF_CONTEXTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXQ: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const QINDEX_RANGE: ::core::ffi::c_int = MAXQ + 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vpx_atomic_load_acquire(
    mut atomic: *const vpx_atomic_int,
) -> ::core::ffi::c_int {
    return (*(&raw const (*atomic).value as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::Acquire);
}
#[inline]
unsafe extern "C" fn intra_prediction_down_copy(
    mut xd: *mut MACROBLOCKD,
    mut above_right_src: *mut ::core::ffi::c_uchar,
) {
    let mut dst_stride: ::core::ffi::c_int = (*xd).dst.y_stride;
    let mut above_right_dst: *mut ::core::ffi::c_uchar = (*xd)
        .dst
        .y_buffer
        .offset(-(dst_stride as isize))
        .offset(16 as ::core::ffi::c_int as isize);
    let mut src_ptr: *mut ::core::ffi::c_uint = above_right_src as *mut ::core::ffi::c_uint;
    let mut dst_ptr0: *mut ::core::ffi::c_uint = above_right_dst
        .offset((4 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr1: *mut ::core::ffi::c_uint = above_right_dst
        .offset((8 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr2: *mut ::core::ffi::c_uint = above_right_dst
        .offset((12 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    *dst_ptr0 = *src_ptr;
    *dst_ptr1 = *src_ptr;
    *dst_ptr2 = *src_ptr;
}
#[inline]
unsafe extern "C" fn setup_intra_recon_left(
    mut y_buffer: *mut ::core::ffi::c_uchar,
    mut u_buffer: *mut ::core::ffi::c_uchar,
    mut v_buffer: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *y_buffer.offset((y_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *u_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *v_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8cx_init_de_quantizer(mut pbi: *mut VP8D_COMP) {
    let mut Q: ::core::ffi::c_int = 0;
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    Q = 0 as ::core::ffi::c_int;
    while Q < QINDEX_RANGE {
        (*pc).Y1dequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc_quant(Q, (*pc).y1dc_delta_q) as ::core::ffi::c_short;
        (*pc).Y2dequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc2quant(Q, (*pc).y2dc_delta_q) as ::core::ffi::c_short;
        (*pc).UVdequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc_uv_quant(Q, (*pc).uvdc_delta_q) as ::core::ffi::c_short;
        (*pc).Y1dequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac_yquant(Q) as ::core::ffi::c_short;
        (*pc).Y2dequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac2quant(Q, (*pc).y2ac_delta_q) as ::core::ffi::c_short;
        (*pc).UVdequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac_uv_quant(Q, (*pc).uvac_delta_q) as ::core::ffi::c_short;
        Q += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_mb_init_dequantizer(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut QIndex: ::core::ffi::c_int = 0;
    let mut mbmi: *mut MB_MODE_INFO = &raw mut (*(*xd).mode_info_context).mbmi;
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    if (*xd).segmentation_enabled != 0 {
        if (*xd).mb_segment_abs_delta as ::core::ffi::c_int == SEGMENT_ABSDATA {
            QIndex = (*xd).segment_feature_data[MB_LVL_ALT_Q as ::core::ffi::c_int as usize]
                [(*mbmi).segment_id as usize] as ::core::ffi::c_int;
        } else {
            QIndex = (*pc).base_qindex
                + (*xd).segment_feature_data[MB_LVL_ALT_Q as ::core::ffi::c_int as usize]
                    [(*mbmi).segment_id as usize] as ::core::ffi::c_int;
        }
        QIndex = if QIndex >= 0 as ::core::ffi::c_int {
            if QIndex <= MAXQ {
                QIndex
            } else {
                MAXQ
            }
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        QIndex = (*pc).base_qindex;
    }
    (*xd).dequant_y1_dc[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_short;
    (*xd).dequant_y1[0 as ::core::ffi::c_int as usize] =
        (*pc).Y1dequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    (*xd).dequant_y2[0 as ::core::ffi::c_int as usize] =
        (*pc).Y2dequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    (*xd).dequant_uv[0 as ::core::ffi::c_int as usize] =
        (*pc).UVdequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    i = 1 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*xd).dequant_y1[i as usize] =
            (*pc).Y1dequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        (*xd).dequant_y1_dc[i as usize] = (*xd).dequant_y1[i as usize];
        (*xd).dequant_y2[i as usize] =
            (*pc).Y2dequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        (*xd).dequant_uv[i as usize] =
            (*pc).UVdequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        i += 1;
    }
}
unsafe extern "C" fn decode_macroblock(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut mb_idx: ::core::ffi::c_uint,
) {
    let mut mode: MB_PREDICTION_MODE = DC_PRED;
    let mut i: ::core::ffi::c_int = 0;
    if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
        vp8_reset_mb_tokens_context(xd);
    } else if vp8dx_bool_error((*xd).current_bc as *mut BOOL_DECODER) == 0 {
        let mut eobtotal: ::core::ffi::c_int = 0;
        eobtotal = vp8_decode_mb_tokens(pbi, xd);
        (*(*xd).mode_info_context).mbmi.mb_skip_coeff =
            (eobtotal == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
    }
    mode = (*(*xd).mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
    if (*xd).segmentation_enabled != 0 {
        vp8_mb_init_dequantizer(pbi, xd);
    }
    if (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
        == INTRA_FRAME as ::core::ffi::c_int
    {
        vp8_build_intra_predictors_mbuv_s(
            xd,
            (*xd).recon_above[1 as ::core::ffi::c_int as usize],
            (*xd).recon_above[2 as ::core::ffi::c_int as usize],
            (*xd).recon_left[1 as ::core::ffi::c_int as usize],
            (*xd).recon_left[2 as ::core::ffi::c_int as usize],
            (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize],
            (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.uv_stride,
        );
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            vp8_build_intra_predictors_mby_s(
                xd,
                (*xd).recon_above[0 as ::core::ffi::c_int as usize],
                (*xd).recon_left[0 as ::core::ffi::c_int as usize],
                (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize],
                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                (*xd).dst.y_stride,
            );
        } else {
            let mut DQC: *mut ::core::ffi::c_short =
                &raw mut (*xd).dequant_y1 as *mut ::core::ffi::c_short;
            let mut dst_stride: ::core::ffi::c_int = (*xd).dst.y_stride;
            if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
                memset(
                    &raw mut (*xd).eobs as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    25 as size_t,
                );
            }
            intra_prediction_down_copy(
                xd,
                (*xd).recon_above[0 as ::core::ffi::c_int as usize]
                    .offset(16 as ::core::ffi::c_int as isize),
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut b: *mut BLOCKD =
                    (&raw mut (*xd).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                let mut dst: *mut ::core::ffi::c_uchar =
                    (*xd).dst.y_buffer.offset((*b).offset as isize);
                let mut b_mode: B_PREDICTION_MODE =
                    (*(*xd).mode_info_context).bmi[i as usize].as_mode;
                let mut Above: *mut ::core::ffi::c_uchar = dst.offset(-(dst_stride as isize));
                let mut yleft: *mut ::core::ffi::c_uchar =
                    dst.offset(-(1 as ::core::ffi::c_int as isize));
                let mut left_stride: ::core::ffi::c_int = dst_stride;
                let mut top_left: ::core::ffi::c_uchar =
                    *Above.offset(-(1 as ::core::ffi::c_int) as isize);
                vp8_intra4x4_predict(Above, yleft, left_stride, b_mode, dst, dst_stride, top_left);
                if (*xd).eobs[i as usize] != 0 {
                    if (*xd).eobs[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        vp8_dequant_idct_add_neon((*b).qcoeff, DQC, dst, dst_stride);
                    } else {
                        vp8_dc_only_idct_add_neon(
                            (*(*b).qcoeff.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                * *DQC.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)
                                as ::core::ffi::c_short,
                            dst,
                            dst_stride,
                            dst,
                            dst_stride,
                        );
                        memset(
                            (*b).qcoeff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_short>() as size_t,
                            ),
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
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            let mut DQC_0: *mut ::core::ffi::c_short =
                &raw mut (*xd).dequant_y1 as *mut ::core::ffi::c_short;
            if mode as ::core::ffi::c_uint != SPLITMV as ::core::ffi::c_int as ::core::ffi::c_uint {
                let mut b_0: *mut BLOCKD = (&raw mut (*xd).block as *mut BLOCKD)
                    .offset(24 as ::core::ffi::c_int as isize)
                    as *mut BLOCKD;
                if (*xd).eobs[24 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                {
                    vp8_dequantize_b_neon(
                        b_0 as *mut blockd,
                        &raw mut (*xd).dequant_y2 as *mut ::core::ffi::c_short,
                    );
                    vp8_short_inv_walsh4x4_neon(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (16 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                } else {
                    *(*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize) = (*(*b_0)
                        .qcoeff
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        * (*xd).dequant_y2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    vp8_short_inv_walsh4x4_1_c(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                DQC_0 = &raw mut (*xd).dequant_y1_dc as *mut ::core::ffi::c_short;
            }
            vp8_dequant_idct_add_y_block_neon(
                &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                DQC_0,
                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                (*xd).dst.y_stride,
                &raw mut (*xd).eobs as *mut ::core::ffi::c_char,
            );
        }
        vp8_dequant_idct_add_uv_block_neon(
            (&raw mut (*xd).qcoeff as *mut ::core::ffi::c_short)
                .offset((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            &raw mut (*xd).dequant_uv as *mut ::core::ffi::c_short,
            (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.uv_stride,
            (&raw mut (*xd).eobs as *mut ::core::ffi::c_char)
                .offset(16 as ::core::ffi::c_int as isize),
        );
    }
}
unsafe extern "C" fn get_delta_q(
    mut bc: *mut vp8_reader,
    mut prev: ::core::ffi::c_int,
    mut q_update: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret_val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int) != 0 {
        ret_val = vp8_decode_value(bc as *mut BOOL_DECODER, 4 as ::core::ffi::c_int);
        if vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int) != 0 {
            ret_val = -ret_val;
        }
    }
    if ret_val != prev {
        *q_update = 1 as ::core::ffi::c_int;
    }
    return ret_val;
}
unsafe extern "C" fn yv12_extend_frame_top_c(mut ybf: *mut YV12_BUFFER_CONFIG) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    src_ptr1 = (*ybf).y_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = (*ybf).u_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = (*ybf).v_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
}
unsafe extern "C" fn yv12_extend_frame_bottom_c(mut ybf: *mut YV12_BUFFER_CONFIG) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut src_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    let mut plane_height: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    plane_height = (*ybf).y_height;
    src_ptr1 = (*ybf).y_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    plane_height = (*ybf).uv_height;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = (*ybf).u_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = (*ybf).v_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
}
unsafe extern "C" fn yv12_extend_frame_left_right_c(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut y_src: *mut ::core::ffi::c_uchar,
    mut u_src: *mut ::core::ffi::c_uchar,
    mut v_src: *mut ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut src_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    let mut plane_height: ::core::ffi::c_int = 0;
    let mut plane_width: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    plane_height = 16 as ::core::ffi::c_int;
    plane_width = (*ybf).y_width;
    src_ptr1 = y_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    plane_height = 8 as ::core::ffi::c_int;
    plane_width = (*ybf).uv_width;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = u_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = v_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
}
unsafe extern "C" fn decode_mb_rows(mut pbi: *mut VP8D_COMP) {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    let mut lf_mic: *mut MODE_INFO = (*xd).mode_info_context;
    let mut ibc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_part: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << (*pc).multi_token_partition as ::core::ffi::c_uint;
    let mut recon_yoffset: ::core::ffi::c_int = 0;
    let mut recon_uvoffset: ::core::ffi::c_int = 0;
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut mb_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    let mut recon_y_stride: ::core::ffi::c_int = (*yv12_fb_new).y_stride;
    let mut recon_uv_stride: ::core::ffi::c_int = (*yv12_fb_new).uv_stride;
    let mut ref_buffer: [[*mut ::core::ffi::c_uchar; 3]; 4] =
        [[::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3]; 4];
    let mut dst_buffer: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut lf_dst: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut eb_dst: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut i: ::core::ffi::c_int = 0;
    let mut ref_fb_corrupted: [::core::ffi::c_int; 4] = [0; 4];
    ref_fb_corrupted[INTRA_FRAME as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < MAX_REF_FRAMES as ::core::ffi::c_int {
        let mut this_fb: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[i as usize];
        ref_buffer[i as usize][0 as ::core::ffi::c_int as usize] =
            (*this_fb).y_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][1 as ::core::ffi::c_int as usize] =
            (*this_fb).u_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][2 as ::core::ffi::c_int as usize] =
            (*this_fb).v_buffer as *mut ::core::ffi::c_uchar;
        ref_fb_corrupted[i as usize] = (*this_fb).corrupted;
        i += 1;
    }
    dst_buffer[0 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).y_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[0 as ::core::ffi::c_int as usize] = dst_buffer[0 as ::core::ffi::c_int as usize];
    eb_dst[0 as ::core::ffi::c_int as usize] = lf_dst[0 as ::core::ffi::c_int as usize];
    dst_buffer[1 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).u_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[1 as ::core::ffi::c_int as usize] = dst_buffer[1 as ::core::ffi::c_int as usize];
    eb_dst[1 as ::core::ffi::c_int as usize] = lf_dst[1 as ::core::ffi::c_int as usize];
    dst_buffer[2 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).v_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[2 as ::core::ffi::c_int as usize] = dst_buffer[2 as ::core::ffi::c_int as usize];
    eb_dst[2 as ::core::ffi::c_int as usize] = lf_dst[2 as ::core::ffi::c_int as usize];
    (*xd).up_available = 0 as ::core::ffi::c_int;
    if (*pc).filter_level != 0 {
        vp8_loop_filter_frame_init(
            pc as *mut VP8Common,
            xd as *mut macroblockd,
            (*pc).filter_level,
        );
    }
    vp8_setup_intra_recon_top_line(yv12_fb_new);
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < (*pc).mb_rows {
        if num_part > 1 as ::core::ffi::c_int {
            (*xd).current_bc = (&raw mut (*pbi).mbc as *mut vp8_reader).offset(ibc as isize)
                as *mut vp8_reader as *mut ::core::ffi::c_void;
            ibc += 1;
            if ibc == num_part {
                ibc = 0 as ::core::ffi::c_int;
            }
        }
        recon_yoffset = mb_row * recon_y_stride * 16 as ::core::ffi::c_int;
        recon_uvoffset = mb_row * recon_uv_stride * 8 as ::core::ffi::c_int;
        (*xd).above_context = (*pc).above_context;
        memset(
            (*xd).left_context as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t,
        );
        (*xd).left_available = 0 as ::core::ffi::c_int;
        (*xd).mb_to_top_edge = -((mb_row * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
        (*xd).mb_to_bottom_edge = (((*pc).mb_rows - 1 as ::core::ffi::c_int - mb_row)
            * 16 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int;
        (*xd).recon_above[0 as ::core::ffi::c_int as usize] =
            dst_buffer[0 as ::core::ffi::c_int as usize].offset(recon_yoffset as isize);
        (*xd).recon_above[1 as ::core::ffi::c_int as usize] =
            dst_buffer[1 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
        (*xd).recon_above[2 as ::core::ffi::c_int as usize] =
            dst_buffer[2 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
        (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [0 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [1 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [2 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [0 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.y_stride as isize));
        (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [1 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.uv_stride as isize));
        (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [2 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.uv_stride as isize));
        (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize] = (*xd).dst.y_stride;
        (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize] = (*xd).dst.uv_stride;
        setup_intra_recon_left(
            (*xd).recon_left[0 as ::core::ffi::c_int as usize],
            (*xd).recon_left[1 as ::core::ffi::c_int as usize],
            (*xd).recon_left[2 as ::core::ffi::c_int as usize],
            (*xd).dst.y_stride,
            (*xd).dst.uv_stride,
        );
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*pc).mb_cols {
            (*xd).mb_to_left_edge =
                -((mb_col * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
            (*xd).mb_to_right_edge = (((*pc).mb_cols - 1 as ::core::ffi::c_int - mb_col)
                * 16 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int;
            (*xd).dst.y_buffer = dst_buffer[0 as ::core::ffi::c_int as usize]
                .offset(recon_yoffset as isize) as *mut uint8_t;
            (*xd).dst.u_buffer = dst_buffer[1 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            (*xd).dst.v_buffer = dst_buffer[2 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            if (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
                >= LAST_FRAME as ::core::ffi::c_int
            {
                let ref_0: MV_REFERENCE_FRAME =
                    (*(*xd).mode_info_context).mbmi.ref_frame as MV_REFERENCE_FRAME;
                (*xd).pre.y_buffer = ref_buffer[ref_0 as usize][0 as ::core::ffi::c_int as usize]
                    .offset(recon_yoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.u_buffer = ref_buffer[ref_0 as usize][1 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.v_buffer = ref_buffer[ref_0 as usize][2 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
            } else {
                (*xd).pre.y_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.u_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.v_buffer = ::core::ptr::null_mut::<uint8_t>();
            }
            (*xd).corrupted |= ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
            decode_macroblock(pbi, xd, mb_idx as ::core::ffi::c_uint);
            mb_idx += 1;
            (*xd).left_available = 1 as ::core::ffi::c_int;
            (*xd).corrupted |= vp8dx_bool_error((*xd).current_bc as *mut BOOL_DECODER);
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [0 as ::core::ffi::c_int as usize]
                .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [1 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [2 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [0 as ::core::ffi::c_int as usize]
                .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [1 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [2 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            recon_yoffset += 16 as ::core::ffi::c_int;
            recon_uvoffset += 8 as ::core::ffi::c_int;
            (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
            (*xd).above_context = (*xd).above_context.offset(1);
            mb_col += 1;
        }
        vp8_extend_mb_row(
            yv12_fb_new,
            (*xd).dst.y_buffer.offset(16 as ::core::ffi::c_int as isize),
            (*xd).dst.u_buffer.offset(8 as ::core::ffi::c_int as isize),
            (*xd).dst.v_buffer.offset(8 as ::core::ffi::c_int as isize),
        );
        (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
        (*xd).up_available = 1 as ::core::ffi::c_int;
        if (*pc).filter_level != 0 {
            if mb_row > 0 as ::core::ffi::c_int {
                if (*pc).filter_type as ::core::ffi::c_uint
                    == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    vp8_loop_filter_row_normal(
                        pc as *mut VP8Common,
                        lf_mic as *mut modeinfo,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        recon_uv_stride,
                        lf_dst[0 as ::core::ffi::c_int as usize],
                        lf_dst[1 as ::core::ffi::c_int as usize],
                        lf_dst[2 as ::core::ffi::c_int as usize],
                    );
                } else {
                    vp8_loop_filter_row_simple(
                        pc as *mut VP8Common,
                        lf_mic as *mut modeinfo,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        lf_dst[0 as ::core::ffi::c_int as usize],
                    );
                }
                if mb_row > 1 as ::core::ffi::c_int {
                    yv12_extend_frame_left_right_c(
                        yv12_fb_new,
                        eb_dst[0 as ::core::ffi::c_int as usize],
                        eb_dst[1 as ::core::ffi::c_int as usize],
                        eb_dst[2 as ::core::ffi::c_int as usize],
                    );
                    eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst
                        [0 as ::core::ffi::c_int as usize]
                        .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
                    eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst
                        [1 as ::core::ffi::c_int as usize]
                        .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                    eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst
                        [2 as ::core::ffi::c_int as usize]
                        .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                }
                lf_dst[0 as ::core::ffi::c_int as usize] = lf_dst[0 as ::core::ffi::c_int as usize]
                    .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
                lf_dst[1 as ::core::ffi::c_int as usize] = lf_dst[1 as ::core::ffi::c_int as usize]
                    .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                lf_dst[2 as ::core::ffi::c_int as usize] = lf_dst[2 as ::core::ffi::c_int as usize]
                    .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                lf_mic = lf_mic.offset((*pc).mb_cols as isize);
                lf_mic = lf_mic.offset(1);
            }
        } else if mb_row > 0 as ::core::ffi::c_int {
            yv12_extend_frame_left_right_c(
                yv12_fb_new,
                eb_dst[0 as ::core::ffi::c_int as usize],
                eb_dst[1 as ::core::ffi::c_int as usize],
                eb_dst[2 as ::core::ffi::c_int as usize],
            );
            eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst[0 as ::core::ffi::c_int as usize]
                .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
            eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst[1 as ::core::ffi::c_int as usize]
                .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
            eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst[2 as ::core::ffi::c_int as usize]
                .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
        }
        mb_row += 1;
    }
    if (*pc).filter_level != 0 {
        if (*pc).filter_type as ::core::ffi::c_uint
            == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            vp8_loop_filter_row_normal(
                pc as *mut VP8Common,
                lf_mic as *mut modeinfo,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                recon_uv_stride,
                lf_dst[0 as ::core::ffi::c_int as usize],
                lf_dst[1 as ::core::ffi::c_int as usize],
                lf_dst[2 as ::core::ffi::c_int as usize],
            );
        } else {
            vp8_loop_filter_row_simple(
                pc as *mut VP8Common,
                lf_mic as *mut modeinfo,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                lf_dst[0 as ::core::ffi::c_int as usize],
            );
        }
        yv12_extend_frame_left_right_c(
            yv12_fb_new,
            eb_dst[0 as ::core::ffi::c_int as usize],
            eb_dst[1 as ::core::ffi::c_int as usize],
            eb_dst[2 as ::core::ffi::c_int as usize],
        );
        eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst[0 as ::core::ffi::c_int as usize]
            .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
        eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst[1 as ::core::ffi::c_int as usize]
            .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
        eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst[2 as ::core::ffi::c_int as usize]
            .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
    }
    yv12_extend_frame_left_right_c(
        yv12_fb_new,
        eb_dst[0 as ::core::ffi::c_int as usize],
        eb_dst[1 as ::core::ffi::c_int as usize],
        eb_dst[2 as ::core::ffi::c_int as usize],
    );
    yv12_extend_frame_top_c(yv12_fb_new);
    yv12_extend_frame_bottom_c(yv12_fb_new);
}
unsafe extern "C" fn read_partition_size(
    mut pbi: *mut VP8D_COMP,
    mut cx_size: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut temp: [::core::ffi::c_uchar; 3] = [0; 3];
    if (*pbi).decrypt_cb.is_some() {
        (*pbi).decrypt_cb.expect("non-null function pointer")(
            (*pbi).decrypt_state,
            cx_size,
            &raw mut temp as *mut ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int,
        );
        cx_size = &raw mut temp as *mut ::core::ffi::c_uchar;
    }
    return (*cx_size.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*cx_size.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
        + ((*cx_size.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
}
unsafe extern "C" fn read_is_valid(
    mut start: *const ::core::ffi::c_uchar,
    mut len: size_t,
    mut end: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (len != 0 as size_t
        && end > start
        && len <= end.offset_from(start) as ::core::ffi::c_long as size_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn read_available_partition_size(
    mut pbi: *mut VP8D_COMP,
    mut token_part_sizes: *const ::core::ffi::c_uchar,
    mut fragment_start: *const ::core::ffi::c_uchar,
    mut first_fragment_end: *const ::core::ffi::c_uchar,
    mut fragment_end: *const ::core::ffi::c_uchar,
    mut i: ::core::ffi::c_int,
    mut num_part: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut partition_size_ptr: *const ::core::ffi::c_uchar =
        token_part_sizes.offset((i * 3 as ::core::ffi::c_int) as isize);
    let mut partition_size: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bytes_left: ptrdiff_t = fragment_end.offset_from(fragment_start) as ptrdiff_t;
    if bytes_left < 0 as ptrdiff_t {
        vpx_internal_error(
            &raw mut (*pc).error,
            VPX_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt partition. No bytes left %d.\0" as *const u8
                as *const ::core::ffi::c_char,
            bytes_left as ::core::ffi::c_int,
        );
    }
    if i < num_part - 1 as ::core::ffi::c_int {
        if read_is_valid(partition_size_ptr, 3 as size_t, first_fragment_end) != 0 {
            partition_size = read_partition_size(pbi, partition_size_ptr);
        } else if (*pbi).ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated partition size data\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        partition_size = bytes_left as ::core::ffi::c_uint;
    }
    if read_is_valid(fragment_start, partition_size as size_t, fragment_end) == 0 {
        if (*pbi).ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet or corrupt partition %d length\0" as *const u8
                    as *const ::core::ffi::c_char,
                i + 1 as ::core::ffi::c_int,
            );
        }
    }
    return partition_size;
}
unsafe extern "C" fn setup_token_decoder(
    mut pbi: *mut VP8D_COMP,
    mut token_part_sizes: *const ::core::ffi::c_uchar,
) {
    let mut bool_decoder: *mut vp8_reader = (&raw mut (*pbi).mbc as *mut vp8_reader)
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut vp8_reader;
    let mut partition_idx: ::core::ffi::c_uint = 0;
    let mut fragment_idx: ::core::ffi::c_uint = 0;
    let mut num_token_partitions: ::core::ffi::c_uint = 0;
    let mut first_fragment_end: *const ::core::ffi::c_uchar = (*pbi).fragments.ptrs
        [0 as ::core::ffi::c_int as usize]
        .offset((*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] as isize);
    let mut multi_token_partition: TOKEN_PARTITION = vp8_decode_value(
        (&raw mut (*pbi).mbc as *mut vp8_reader).offset(8 as ::core::ffi::c_int as isize)
            as *mut BOOL_DECODER,
        2 as ::core::ffi::c_int,
    ) as TOKEN_PARTITION;
    if vp8dx_bool_error(
        (&raw mut (*pbi).mbc as *mut vp8_reader).offset(8 as ::core::ffi::c_int as isize)
            as *mut BOOL_DECODER,
    ) == 0
    {
        (*pbi).common.multi_token_partition = multi_token_partition;
    }
    num_token_partitions = ((1 as ::core::ffi::c_int)
        << (*pbi).common.multi_token_partition as ::core::ffi::c_uint)
        as ::core::ffi::c_uint;
    fragment_idx = 0 as ::core::ffi::c_uint;
    while fragment_idx < (*pbi).fragments.count {
        let mut fragment_size: ::core::ffi::c_uint = (*pbi).fragments.sizes[fragment_idx as usize];
        let mut fragment_end: *const ::core::ffi::c_uchar =
            (*pbi).fragments.ptrs[fragment_idx as usize].offset(fragment_size as isize);
        if fragment_idx == 0 as ::core::ffi::c_uint {
            let mut ext_first_part_size: ptrdiff_t = token_part_sizes
                .offset_from((*pbi).fragments.ptrs[0 as ::core::ffi::c_int as usize])
                as ptrdiff_t
                + (3 as ::core::ffi::c_uint)
                    .wrapping_mul(num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint))
                    as ptrdiff_t;
            if fragment_size < ext_first_part_size as ::core::ffi::c_uint {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupted fragment size %d\0" as *const u8 as *const ::core::ffi::c_char,
                    fragment_size,
                );
            }
            fragment_size = fragment_size.wrapping_sub(ext_first_part_size as ::core::ffi::c_uint);
            if fragment_size > 0 as ::core::ffi::c_uint {
                (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] =
                    ext_first_part_size as ::core::ffi::c_uint;
                fragment_idx = fragment_idx.wrapping_add(1);
                (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                    [0 as ::core::ffi::c_int as usize]
                    .offset((*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] as isize);
            }
        }
        while fragment_size > 0 as ::core::ffi::c_uint {
            let mut partition_size: ptrdiff_t = read_available_partition_size(
                pbi,
                token_part_sizes,
                (*pbi).fragments.ptrs[fragment_idx as usize],
                first_fragment_end,
                fragment_end,
                fragment_idx.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                num_token_partitions as ::core::ffi::c_int,
            ) as ptrdiff_t;
            (*pbi).fragments.sizes[fragment_idx as usize] = partition_size as ::core::ffi::c_uint;
            if fragment_size < partition_size as ::core::ffi::c_uint {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupted fragment size %d\0" as *const u8 as *const ::core::ffi::c_char,
                    fragment_size,
                );
            }
            fragment_size = fragment_size.wrapping_sub(partition_size as ::core::ffi::c_uint);
            if fragment_size > 0 as ::core::ffi::c_uint {
                fragment_idx = fragment_idx.wrapping_add(1);
                (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                    [fragment_idx.wrapping_sub(1 as ::core::ffi::c_uint) as usize]
                    .offset(partition_size as isize);
            }
        }
        fragment_idx = fragment_idx.wrapping_add(1);
    }
    (*pbi).fragments.count = num_token_partitions.wrapping_add(1 as ::core::ffi::c_uint);
    partition_idx = 1 as ::core::ffi::c_uint;
    while partition_idx < (*pbi).fragments.count {
        if vp8dx_start_decode(
            bool_decoder as *mut BOOL_DECODER,
            (*pbi).fragments.ptrs[partition_idx as usize],
            (*pbi).fragments.sizes[partition_idx as usize],
            (*pbi).decrypt_cb,
            (*pbi).decrypt_state,
        ) != 0
        {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate bool decoder %d\0" as *const u8 as *const ::core::ffi::c_char,
                partition_idx,
            );
        }
        bool_decoder = bool_decoder.offset(1);
        partition_idx = partition_idx.wrapping_add(1);
    }
    if (*pbi).decoding_thread_count > num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint) {
        (*pbi).decoding_thread_count = num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint);
    }
    if (*pbi).decoding_thread_count as ::core::ffi::c_int
        > (*pbi).common.mb_rows - 1 as ::core::ffi::c_int
    {
        (*pbi).decoding_thread_count =
            ((*pbi).common.mb_rows - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn init_frame(mut pbi: *mut VP8D_COMP) {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    if (*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memcpy(
            &raw mut (*pc).fc.mvc as *mut MV_CONTEXT as *mut ::core::ffi::c_void,
            &raw const vp8_default_mv_context as *const MV_CONTEXT as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[MV_CONTEXT; 2]>() as size_t,
        );
        vp8_init_mbmode_probs(pc);
        vp8_default_coef_probs(pc as *mut VP8Common);
        memset(
            &raw mut (*xd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[::core::ffi::c_schar; 4]; 2]>() as size_t,
        );
        (*xd).mb_segment_abs_delta = SEGMENT_DELTADATA as ::core::ffi::c_uchar;
        memset(
            &raw mut (*xd).ref_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        memset(
            &raw mut (*xd).mode_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        (*pc).refresh_golden_frame = 1 as ::core::ffi::c_int;
        (*pc).refresh_alt_ref_frame = 1 as ::core::ffi::c_int;
        (*pc).copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        (*pc).copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        (*pc).ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
        (*pc).ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
    } else {
        if (*pc).use_bilinear_mc_filter == 0 {
            (*xd).subpixel_predict = Some(
                vp8_sixtap_predict4x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x4 = Some(
                vp8_sixtap_predict8x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x8 = Some(
                vp8_sixtap_predict8x8_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict16x16 = Some(
                vp8_sixtap_predict16x16_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
        } else {
            (*xd).subpixel_predict = Some(
                vp8_bilinear_predict4x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x4 = Some(
                vp8_bilinear_predict8x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x8 = Some(
                vp8_bilinear_predict8x8_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict16x16 = Some(
                vp8_bilinear_predict16x16_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
        }
        if (*pbi).decoded_key_frame != 0 && (*pbi).ec_enabled != 0 && (*pbi).ec_active == 0 {
            (*pbi).ec_active = 1 as ::core::ffi::c_int;
        }
    }
    (*xd).left_context = &raw mut (*pc).left_context;
    (*xd).mode_info_context = (*pc).mi;
    (*xd).frame_type = (*pc).frame_type;
    (*(*xd).mode_info_context).mbmi.mode = DC_PRED as ::core::ffi::c_int as uint8_t;
    (*xd).mode_info_stride = (*pc).mode_info_stride;
    (*xd).corrupted = 0 as ::core::ffi::c_int;
    (*xd).fullpixel_mask = !(0 as ::core::ffi::c_int);
    if (*pc).full_pixel != 0 {
        (*xd).fullpixel_mask = !(7 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_decode_frame(mut pbi: *mut VP8D_COMP) -> ::core::ffi::c_int {
    let bc: *mut vp8_reader = (&raw mut (*pbi).mbc as *mut vp8_reader)
        .offset(8 as ::core::ffi::c_int as isize) as *mut vp8_reader;
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    let mut data: *const ::core::ffi::c_uchar =
        (*pbi).fragments.ptrs[0 as ::core::ffi::c_int as usize];
    let data_sz: ::core::ffi::c_uint = (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize];
    let mut data_end: *const ::core::ffi::c_uchar = data.offset(data_sz as isize);
    let mut first_partition_length_in_bytes: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mb_feature_data_bits: *const ::core::ffi::c_int =
        &raw const vp8_mb_feature_data_bits as *const ::core::ffi::c_int;
    let mut corrupt_tokens: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prev_independent_partitions: ::core::ffi::c_int = (*pbi).independent_partitions;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    (*xd).corrupted = 0 as ::core::ffi::c_int;
    (*yv12_fb_new).corrupted = 0 as ::core::ffi::c_int;
    if (data_end.offset_from(data) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
        if (*pbi).ec_active == 0 {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*pc).frame_type = INTER_FRAME;
        (*pc).version = 0 as ::core::ffi::c_int;
        (*pc).show_frame = 1 as ::core::ffi::c_int;
        first_partition_length_in_bytes = 0 as ::core::ffi::c_int;
    } else {
        let mut clear_buffer: [::core::ffi::c_uchar; 10] = [0; 10];
        let mut clear: *const ::core::ffi::c_uchar = data;
        if (*pbi).decrypt_cb.is_some() {
            let mut n: ::core::ffi::c_int =
                (if (::core::mem::size_of::<[::core::ffi::c_uchar; 10]>() as usize)
                    < data_sz as usize
                {
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 10]>() as usize
                } else {
                    data_sz as usize
                }) as ::core::ffi::c_int;
            (*pbi).decrypt_cb.expect("non-null function pointer")(
                (*pbi).decrypt_state,
                data,
                &raw mut clear_buffer as *mut ::core::ffi::c_uchar,
                n,
            );
            clear = &raw mut clear_buffer as *mut ::core::ffi::c_uchar;
        }
        (*pc).frame_type = (*clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int) as FRAME_TYPE;
        (*pc).version = *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int;
        (*pc).show_frame = *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int;
        first_partition_length_in_bytes = (*clear.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            | (*clear.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
            | (*clear.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int)
            >> 5 as ::core::ffi::c_int;
        if (*pbi).ec_active == 0 && first_partition_length_in_bytes == 0 as ::core::ffi::c_int {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Corrupt partition 0 length\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        data = data.offset(3 as ::core::ffi::c_int as isize);
        clear = clear.offset(3 as ::core::ffi::c_int as isize);
        vp8_setup_version(pc);
        if (*pc).frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if data_end.offset_from(data) as ::core::ffi::c_long >= 7 as ::core::ffi::c_long {
                if *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0x9d as ::core::ffi::c_int
                    || *clear.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 0x1 as ::core::ffi::c_int
                    || *clear.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 0x2a as ::core::ffi::c_int
                {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_UNSUP_BITSTREAM,
                        b"Invalid frame sync code\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*pc).Width = (*clear.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    | (*clear.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    & 0x3fff as ::core::ffi::c_int;
                (*pc).horiz_scale = *clear.offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int;
                (*pc).Height = (*clear.offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    | (*clear.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    & 0x3fff as ::core::ffi::c_int;
                (*pc).vert_scale = *clear.offset(6 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int;
                data = data.offset(7 as ::core::ffi::c_int as isize);
            } else if (*pbi).ec_active == 0 {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Truncated key frame header\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                data = data_end;
            }
        } else {
            (*xd).pre = *yv12_fb_new;
            (*xd).dst = *yv12_fb_new;
        }
    }
    if (*pbi).decoded_key_frame == 0
        && (*pc).frame_type as ::core::ffi::c_uint
            != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*pbi).ec_active == 0
        && (data_end.offset_from(data) as ::core::ffi::c_long)
            < first_partition_length_in_bytes as ::core::ffi::c_long
    {
        vpx_internal_error(
            &raw mut (*pc).error,
            VPX_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt partition 0 length\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    init_frame(pbi);
    if vp8dx_start_decode(
        bc as *mut BOOL_DECODER,
        data,
        data_end.offset_from(data) as ::core::ffi::c_long as ::core::ffi::c_uint,
        (*pbi).decrypt_cb,
        (*pbi).decrypt_state,
    ) != 0
    {
        vpx_internal_error(
            &raw mut (*pc).error,
            VPX_CODEC_MEM_ERROR,
            b"Failed to allocate bool decoder 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
        (*pc).clamp_type =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                as CLAMP_TYPE;
    }
    (*xd).segmentation_enabled =
        vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
    if (*xd).segmentation_enabled != 0 {
        (*xd).update_mb_segmentation_map =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        (*xd).update_mb_segmentation_data =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        if (*xd).update_mb_segmentation_data != 0 {
            (*xd).mb_segment_abs_delta =
                vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            memset(
                &raw mut (*xd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[[::core::ffi::c_schar; 4]; 2]>() as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while i < MB_LVL_MAX as ::core::ffi::c_int {
                j = 0 as ::core::ffi::c_int;
                while j < MAX_MB_SEGMENTS {
                    if vp8dx_decode_bool(
                        bc as *mut BOOL_DECODER,
                        vp8_prob_half as ::core::ffi::c_int,
                    ) != 0
                    {
                        (*xd).segment_feature_data[i as usize][j as usize] = vp8_decode_value(
                            bc as *mut BOOL_DECODER,
                            *mb_feature_data_bits.offset(i as isize),
                        )
                            as ::core::ffi::c_schar;
                        if vp8dx_decode_bool(
                            bc as *mut BOOL_DECODER,
                            vp8_prob_half as ::core::ffi::c_int,
                        ) != 0
                        {
                            (*xd).segment_feature_data[i as usize][j as usize] = -((*xd)
                                .segment_feature_data[i as usize][j as usize]
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_schar;
                        }
                    } else {
                        (*xd).segment_feature_data[i as usize][j as usize] =
                            0 as ::core::ffi::c_schar;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        if (*xd).update_mb_segmentation_map != 0 {
            memset(
                &raw mut (*xd).mb_segment_tree_probs as *mut vp8_prob as *mut ::core::ffi::c_void,
                255 as ::core::ffi::c_int,
                ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while i < MB_FEATURE_TREE_PROBS {
                if vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                    != 0
                {
                    (*xd).mb_segment_tree_probs[i as usize] =
                        vp8_decode_value(bc as *mut BOOL_DECODER, 8 as ::core::ffi::c_int)
                            as vp8_prob;
                }
                i += 1;
            }
        }
    } else {
        (*xd).update_mb_segmentation_map = 0 as ::core::ffi::c_uchar;
        (*xd).update_mb_segmentation_data = 0 as ::core::ffi::c_uchar;
    }
    (*pc).filter_type =
        vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
            as LOOPFILTERTYPE;
    (*pc).filter_level = vp8_decode_value(bc as *mut BOOL_DECODER, 6 as ::core::ffi::c_int);
    (*pc).sharpness_level = vp8_decode_value(bc as *mut BOOL_DECODER, 3 as ::core::ffi::c_int);
    (*xd).mode_ref_lf_delta_update = 0 as ::core::ffi::c_uchar;
    (*xd).mode_ref_lf_delta_enabled =
        vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
    if (*xd).mode_ref_lf_delta_enabled != 0 {
        (*xd).mode_ref_lf_delta_update =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        if (*xd).mode_ref_lf_delta_update != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < MAX_REF_LF_DELTAS {
                if vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                    != 0
                {
                    (*xd).ref_lf_deltas[i as usize] =
                        vp8_decode_value(bc as *mut BOOL_DECODER, 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_schar;
                    if vp8dx_decode_bool(
                        bc as *mut BOOL_DECODER,
                        vp8_prob_half as ::core::ffi::c_int,
                    ) != 0
                    {
                        (*xd).ref_lf_deltas[i as usize] = ((*xd).ref_lf_deltas[i as usize]
                            as ::core::ffi::c_int
                            * -(1 as ::core::ffi::c_int))
                            as ::core::ffi::c_schar;
                    }
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < MAX_MODE_LF_DELTAS {
                if vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int)
                    != 0
                {
                    (*xd).mode_lf_deltas[i as usize] =
                        vp8_decode_value(bc as *mut BOOL_DECODER, 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_schar;
                    if vp8dx_decode_bool(
                        bc as *mut BOOL_DECODER,
                        vp8_prob_half as ::core::ffi::c_int,
                    ) != 0
                    {
                        (*xd).mode_lf_deltas[i as usize] = ((*xd).mode_lf_deltas[i as usize]
                            as ::core::ffi::c_int
                            * -(1 as ::core::ffi::c_int))
                            as ::core::ffi::c_schar;
                    }
                }
                i += 1;
            }
        }
    }
    setup_token_decoder(pbi, data.offset(first_partition_length_in_bytes as isize));
    (*xd).current_bc = (&raw mut (*pbi).mbc as *mut vp8_reader)
        .offset(0 as ::core::ffi::c_int as isize) as *mut vp8_reader
        as *mut ::core::ffi::c_void;
    let mut Q: ::core::ffi::c_int = 0;
    let mut q_update: ::core::ffi::c_int = 0;
    Q = vp8_decode_value(bc as *mut BOOL_DECODER, 7 as ::core::ffi::c_int);
    (*pc).base_qindex = Q;
    q_update = 0 as ::core::ffi::c_int;
    (*pc).y1dc_delta_q = get_delta_q(bc, (*pc).y1dc_delta_q, &raw mut q_update);
    (*pc).y2dc_delta_q = get_delta_q(bc, (*pc).y2dc_delta_q, &raw mut q_update);
    (*pc).y2ac_delta_q = get_delta_q(bc, (*pc).y2ac_delta_q, &raw mut q_update);
    (*pc).uvdc_delta_q = get_delta_q(bc, (*pc).uvdc_delta_q, &raw mut q_update);
    (*pc).uvac_delta_q = get_delta_q(bc, (*pc).uvac_delta_q, &raw mut q_update);
    if q_update != 0 {
        vp8cx_init_de_quantizer(pbi);
    }
    vp8_mb_init_dequantizer(pbi, &raw mut (*pbi).mb);
    if (*pc).frame_type as ::core::ffi::c_uint
        != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*pc).refresh_golden_frame =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
        (*pc).refresh_alt_ref_frame =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
        (*pc).copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        if (*pc).refresh_golden_frame == 0 {
            (*pc).copy_buffer_to_gf =
                vp8_decode_value(bc as *mut BOOL_DECODER, 2 as ::core::ffi::c_int);
        }
        (*pc).copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        if (*pc).refresh_alt_ref_frame == 0 {
            (*pc).copy_buffer_to_arf =
                vp8_decode_value(bc as *mut BOOL_DECODER, 2 as ::core::ffi::c_int);
        }
        (*pc).ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
        (*pc).ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] =
            vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
    }
    (*pc).refresh_entropy_probs =
        vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int);
    if (*pc).refresh_entropy_probs == 0 as ::core::ffi::c_int {
        (*pc).lfc = (*pc).fc;
    }
    (*pc).refresh_last_frame = ((*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        || vp8dx_decode_bool(bc as *mut BOOL_DECODER, vp8_prob_half as ::core::ffi::c_int) != 0)
        as ::core::ffi::c_int;
    (*pbi).independent_partitions = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BLOCK_TYPES {
        j = 0 as ::core::ffi::c_int;
        while j < COEF_BANDS {
            k = 0 as ::core::ffi::c_int;
            while k < PREV_COEF_CONTEXTS {
                l = 0 as ::core::ffi::c_int;
                while l < ENTROPY_NODES {
                    let p: *mut vp8_prob =
                        (&raw mut *(&raw mut *(&raw mut *(&raw mut (*pc).fc.coef_probs
                            as *mut [[[vp8_prob; 11]; 3]; 8])
                            .offset(i as isize)
                            as *mut [[vp8_prob; 11]; 3])
                            .offset(j as isize)
                            as *mut [vp8_prob; 11])
                            .offset(k as isize) as *mut vp8_prob)
                            .offset(l as isize);
                    if vp8dx_decode_bool(
                        bc as *mut BOOL_DECODER,
                        vp8_coef_update_probs[i as usize][j as usize][k as usize][l as usize]
                            as ::core::ffi::c_int,
                    ) != 0
                    {
                        *p = vp8_decode_value(bc as *mut BOOL_DECODER, 8 as ::core::ffi::c_int)
                            as vp8_prob;
                    }
                    if k > 0 as ::core::ffi::c_int
                        && *p as ::core::ffi::c_int
                            != (*pc).fc.coef_probs[i as usize][j as usize]
                                [(k - 1 as ::core::ffi::c_int) as usize]
                                [l as usize] as ::core::ffi::c_int
                    {
                        (*pbi).independent_partitions = 0 as ::core::ffi::c_int;
                    }
                    l += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    memset(
        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_short; 400]>() as size_t,
    );
    vp8_decode_mode_mvs(pbi);
    memset(
        (*pc).above_context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t)
            .wrapping_mul((*pc).mb_cols as size_t),
    );
    (*pbi).frame_corrupt_residual = 0 as ::core::ffi::c_int;
    if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0
        && (*pc).multi_token_partition as ::core::ffi::c_uint
            != ONE_PARTITION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut thread: ::core::ffi::c_uint = 0;
        if vp8mt_decode_mb_rows(pbi, xd) != 0 {
            vp8_decoder_remove_threads(pbi);
            (*pbi).restart_threads = 1 as ::core::ffi::c_int;
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_CORRUPT_FRAME,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        vp8_yv12_extend_frame_borders_c(yv12_fb_new as *mut yv12_buffer_config);
        thread = 0 as ::core::ffi::c_uint;
        while thread < (*pbi).decoding_thread_count {
            corrupt_tokens |= (*(*pbi).mb_row_di.offset(thread as isize)).mbd.corrupted;
            thread = thread.wrapping_add(1);
        }
    } else {
        decode_mb_rows(pbi);
        corrupt_tokens |= (*xd).corrupted;
    }
    (*yv12_fb_new).corrupted = vp8dx_bool_error(bc as *mut BOOL_DECODER);
    (*yv12_fb_new).corrupted |= corrupt_tokens;
    if (*pbi).decoded_key_frame == 0 {
        if (*pc).frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*yv12_fb_new).corrupted == 0
        {
            (*pbi).decoded_key_frame = 1 as ::core::ffi::c_int;
        } else {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_CORRUPT_FRAME,
                b"A stream must start with a complete key frame\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if (*pc).refresh_entropy_probs == 0 as ::core::ffi::c_int {
        (*pc).fc = (*pc).lfc;
        (*pbi).independent_partitions = prev_independent_partitions;
    }
    return 0 as ::core::ffi::c_int;
}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
