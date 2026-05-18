unsafe extern "C" {
    static vp8_norm: [::core::ffi::c_uchar; 256];
    fn vp8dx_bool_decoder_fill(br: *mut BOOL_DECODER);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
pub type int16_t = i16;
pub type size_t = __darwin_size_t;
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
pub type jmp_buf = [::core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: ::core::ffi::c_int,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: ::core::ffi::c_int,
    pub jmp: jmp_buf,
}
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
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
pub type vp8_prob = ::core::ffi::c_uchar;
pub type ENTROPY_CONTEXT = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type FRAME_TYPE = ::core::ffi::c_uint;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
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
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}
pub type MODE_INFO = modeinfo;
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
pub type BLOCKD = blockd;
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
pub type MACROBLOCKD = macroblockd;
pub type pthread_t = *mut ::core::ffi::c_void;
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
    pub mblim: [[::core::ffi::c_uchar; 16]; 64],
    pub blim: [[::core::ffi::c_uchar; 16]; 64],
    pub lim: [[::core::ffi::c_uchar; 16]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 16]; 4],
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
pub type ProbaArray = *const [[uint8_t; 11]; 3];
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
unsafe extern "C" fn vp8dx_decode_bool(
    mut br: *mut BOOL_DECODER,
    mut probability: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
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
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_reset_mb_tokens_context(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut a_ctx: *mut ENTROPY_CONTEXT = (*x).above_context as *mut ENTROPY_CONTEXT;
        let mut l_ctx: *mut ENTROPY_CONTEXT = (*x).left_context as *mut ENTROPY_CONTEXT;
        memset(
            a_ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t).wrapping_sub(1 as size_t),
        );
        memset(
            l_ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t).wrapping_sub(1 as size_t),
        );
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            let ref mut fresh0 = *l_ctx.offset(8 as ::core::ffi::c_int as isize);
            *fresh0 = 0 as ENTROPY_CONTEXT;
            *a_ctx.offset(8 as ::core::ffi::c_int as isize) = *fresh0;
        }
    }
}
static mut kBands: [uint8_t; 17] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat3: [uint8_t; 4] = [
    173 as ::core::ffi::c_int as uint8_t,
    148 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat4: [uint8_t; 5] = [
    176 as ::core::ffi::c_int as uint8_t,
    155 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat5: [uint8_t; 6] = [
    180 as ::core::ffi::c_int as uint8_t,
    157 as ::core::ffi::c_int as uint8_t,
    141 as ::core::ffi::c_int as uint8_t,
    134 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat6: [uint8_t; 12] = [
    254 as ::core::ffi::c_int as uint8_t,
    254 as ::core::ffi::c_int as uint8_t,
    243 as ::core::ffi::c_int as uint8_t,
    230 as ::core::ffi::c_int as uint8_t,
    196 as ::core::ffi::c_int as uint8_t,
    177 as ::core::ffi::c_int as uint8_t,
    153 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    133 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    129 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat3456: [*const uint8_t; 4] = {
    [
        &raw const kCat3 as *const uint8_t,
        &raw const kCat4 as *const uint8_t,
        &raw const kCat5 as *const uint8_t,
        &raw const kCat6 as *const uint8_t,
    ]
};
static mut kZigzag: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn GetSigned(
    mut br: *mut BOOL_DECODER,
    mut value_to_sign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut split: ::core::ffi::c_int = ((*br).range.wrapping_add(1 as ::core::ffi::c_uint)
            >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut bigsplit: VP8_BD_VALUE =
            (split as VP8_BD_VALUE) << VP8_BD_VALUE_SIZE - 8 as ::core::ffi::c_int;
        let mut v: ::core::ffi::c_int = 0;
        if (*br).count < 0 as ::core::ffi::c_int {
            vp8dx_bool_decoder_fill(br);
        }
        if (*br).value < bigsplit {
            (*br).range = split as ::core::ffi::c_uint;
            v = value_to_sign;
        } else {
            (*br).range = (*br).range.wrapping_sub(split as ::core::ffi::c_uint);
            (*br).value = (*br).value.wrapping_sub(bigsplit);
            v = -value_to_sign;
        }
        (*br).range = (*br).range.wrapping_add((*br).range);
        (*br).value = (*br).value.wrapping_add((*br).value);
        (*br).count -= 1;
        return v;
    }
}
unsafe extern "C" fn GetCoeffs(
    mut br: *mut BOOL_DECODER,
    mut prob: ProbaArray,
    mut ctx: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut out: *mut int16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *const uint8_t = &raw const *(&raw const *prob.offset(n as isize)
            as *const [uint8_t; 11])
            .offset(ctx as isize) as *const uint8_t;
        if vp8dx_decode_bool(
            br,
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        loop {
            n += 1;
            if vp8dx_decode_bool(
                br,
                *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) == 0
            {
                p = &raw const *(&raw const *prob
                    .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                    as *const [uint8_t; 11])
                    .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t;
            } else {
                let mut v: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                if vp8dx_decode_bool(
                    br,
                    *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) == 0
                {
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                        as *const [uint8_t; 11])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const uint8_t;
                    v = 1 as ::core::ffi::c_int;
                } else {
                    if vp8dx_decode_bool(
                        br,
                        *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) == 0
                    {
                        if vp8dx_decode_bool(
                            br,
                            *p.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        ) == 0
                        {
                            v = 2 as ::core::ffi::c_int;
                        } else {
                            v = 3 as ::core::ffi::c_int
                                + vp8dx_decode_bool(
                                    br,
                                    *p.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int,
                                );
                        }
                    } else if vp8dx_decode_bool(
                        br,
                        *p.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) == 0
                    {
                        if vp8dx_decode_bool(
                            br,
                            *p.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        ) == 0
                        {
                            v = 5 as ::core::ffi::c_int
                                + vp8dx_decode_bool(br, 159 as ::core::ffi::c_int);
                        } else {
                            v = 7 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                    * vp8dx_decode_bool(br, 165 as ::core::ffi::c_int);
                            v += vp8dx_decode_bool(br, 145 as ::core::ffi::c_int);
                        }
                    } else {
                        let mut tab: *const uint8_t = ::core::ptr::null::<uint8_t>();
                        let bit1: ::core::ffi::c_int = vp8dx_decode_bool(
                            br,
                            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        )
                            as ::core::ffi::c_int;
                        let bit0: ::core::ffi::c_int = vp8dx_decode_bool(
                            br,
                            *p.offset((9 as ::core::ffi::c_int + bit1) as isize)
                                as ::core::ffi::c_int,
                        )
                            as ::core::ffi::c_int;
                        let cat: ::core::ffi::c_int = 2 as ::core::ffi::c_int * bit1 + bit0;
                        v = 0 as ::core::ffi::c_int;
                        tab = kCat3456[cat as usize];
                        while *tab != 0 {
                            v += v + vp8dx_decode_bool(br, *tab as ::core::ffi::c_int);
                            tab = tab.offset(1);
                        }
                        v += 3 as ::core::ffi::c_int + ((8 as ::core::ffi::c_int) << cat);
                    }
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                        as *const [uint8_t; 11])
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const uint8_t;
                }
                j = kZigzag[(n - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
                *out.offset(j as isize) = GetSigned(br, v) as int16_t;
                if n == 16 as ::core::ffi::c_int
                    || vp8dx_decode_bool(
                        br,
                        *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) == 0
                {
                    return n;
                }
            }
            if n == 16 as ::core::ffi::c_int {
                return 16 as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_decode_mb_tokens(
    mut dx: *mut VP8D_COMP,
    mut x: *mut MACROBLOCKD,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bc: *mut BOOL_DECODER = (*x).current_bc as *mut BOOL_DECODER;
        let fc: *const FRAME_CONTEXT = &raw mut (*dx).common.fc;
        let mut eobs: *mut ::core::ffi::c_char = &raw mut (*x).eobs as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let mut nonzeros: ::core::ffi::c_int = 0;
        let mut eobtotal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut qcoeff_ptr: *mut ::core::ffi::c_short =
            ::core::ptr::null_mut::<::core::ffi::c_short>();
        let mut coef_probs: ProbaArray = ::core::ptr::null::<[[uint8_t; 11]; 3]>();
        let mut a_ctx: *mut ENTROPY_CONTEXT = (*x).above_context as *mut ENTROPY_CONTEXT;
        let mut l_ctx: *mut ENTROPY_CONTEXT = (*x).left_context as *mut ENTROPY_CONTEXT;
        let mut a: *mut ENTROPY_CONTEXT = ::core::ptr::null_mut::<ENTROPY_CONTEXT>();
        let mut l: *mut ENTROPY_CONTEXT = ::core::ptr::null_mut::<ENTROPY_CONTEXT>();
        let mut skip_dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        qcoeff_ptr = (&raw mut (*x).qcoeff as *mut ::core::ffi::c_short)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_short;
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            a = a_ctx.offset(8 as ::core::ffi::c_int as isize);
            l = l_ctx.offset(8 as ::core::ffi::c_int as isize);
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(1 as ::core::ffi::c_int as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as ::core::ffi::c_int + *l as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                qcoeff_ptr.offset((24 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            );
            *l = (nonzeros > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ENTROPY_CONTEXT;
            *a = *l;
            *eobs.offset(24 as ::core::ffi::c_int as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros - 16 as ::core::ffi::c_int;
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(0 as ::core::ffi::c_int as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            skip_dc = 1 as ::core::ffi::c_int;
        } else {
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(3 as ::core::ffi::c_int as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            skip_dc = 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            a = a_ctx.offset((i & 3 as ::core::ffi::c_int) as isize);
            l = l_ctx.offset(((i & 0xc as ::core::ffi::c_int) >> 2 as ::core::ffi::c_int) as isize);
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as ::core::ffi::c_int + *l as ::core::ffi::c_int,
                skip_dc,
                qcoeff_ptr as *mut int16_t,
            );
            *l = (nonzeros > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ENTROPY_CONTEXT;
            *a = *l;
            nonzeros += skip_dc;
            *eobs.offset(i as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as ::core::ffi::c_int as isize);
            i += 1;
        }
        coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[vp8_prob; 11]; 3]; 8])
            .offset(2 as ::core::ffi::c_int as isize)
            as *const [[vp8_prob; 11]; 3] as ProbaArray;
        a_ctx = a_ctx.offset(4 as ::core::ffi::c_int as isize);
        l_ctx = l_ctx.offset(4 as ::core::ffi::c_int as isize);
        i = 16 as ::core::ffi::c_int;
        while i < 24 as ::core::ffi::c_int {
            a = a_ctx
                .offset(
                    (((i > 19 as ::core::ffi::c_int) as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as isize,
                )
                .offset((i & 1 as ::core::ffi::c_int) as isize);
            l = l_ctx
                .offset(
                    (((i > 19 as ::core::ffi::c_int) as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as isize,
                )
                .offset(
                    (i & 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as isize,
                );
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as ::core::ffi::c_int + *l as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                qcoeff_ptr as *mut int16_t,
            );
            *l = (nonzeros > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ENTROPY_CONTEXT;
            *a = *l;
            *eobs.offset(i as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as ::core::ffi::c_int as isize);
            i += 1;
        }
        return eobtotal;
    }
}
