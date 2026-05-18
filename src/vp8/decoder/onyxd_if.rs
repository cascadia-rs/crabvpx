use std::ffi::c_void;
unsafe extern "Rust" {
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const i8,
    );
    fn vp8_loop_filter_init(cm: *mut VP8Common);
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
    fn pthread_once(_: *mut pthread_once_t, _: Option<unsafe fn() -> ()>) -> i32;
    fn vp8cx_init_de_quantizer(pbi: *mut VP8D_COMP);
    fn vp8_decode_frame(pbi: *mut VP8D_COMP) -> i32;
    fn vpx_memalign(align: size_t, size: size_t) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
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
pub struct _opaque_pthread_once_t {
    pub __sig: i64,
    pub __opaque: [i8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: i64,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [i8; 8176],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub y1dequant: [[i16; 2]; 128],
    pub y2dequant: [[i16; 2]; 128],
    pub uvdequant: [[i16; 2]; 128],
    pub width: i32,
    pub height: i32,
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
    pub mbs: i32,
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
pub type CLAMP_TYPE = u32;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type C2RustUnnamed = u32;
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
    pub post_proc_flag: i32,
    pub deblocking_level: i32,
    pub noise_level: i32,
    pub display_ref_frame_flag: i32,
    pub display_mb_modes_flag: i32,
    pub display_b_modes_flag: i32,
    pub display_mv_flag: i32,
}
pub type vpx_ref_frame_type = u32;
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
    pub width: i32,
    pub height: i32,
    pub version: i32,
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
pub type pthread_once_t = *mut c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_buffers {
    pub pbi: [*mut VP8D_COMP; 32],
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const _PTHREAD_ONCE_SIG_init: i32 = 0x30b1bcba as i32;
pub const NUM_YV12_BUFFERS: i32 = 4 as i32;
unsafe fn once(mut func: Option<unsafe fn() -> ()>) {
    unsafe {
        static INIT: std::sync::Once = std::sync::Once::new();
        if let Some(f) = func {
            INIT.call_once(|| f());
        }
    }
}
unsafe fn initialize_dec() {
    unsafe {
        static mut init_done: i32 = 0 as i32;
        if init_done == 0 {
            vpx_dsp_rtcd();
            vp8_init_intra_predictors();
            ::core::ptr::write_volatile(&raw mut init_done as *mut i32, 1 as i32);
        }
    }
}
unsafe fn remove_decompressor(mut pbi: *mut VP8D_COMP) {
    unsafe {
        vp8_remove_common(&raw mut (*pbi).common);
        vpx_free(pbi as *mut c_void);
    }
}
unsafe fn create_decompressor(_oxcf: *mut VP8D_CONFIG) -> *mut VP8D_COMP {
    unsafe {
        let mut pbi: *mut VP8D_COMP =
            vpx_memalign(32 as size_t, ::core::mem::size_of::<VP8D_COMP>() as size_t)
                as *mut VP8D_COMP;
        if pbi.is_null() {
            return ::core::ptr::null_mut::<VP8D_COMP>();
        }
        core::ptr::write_bytes(
            pbi as *mut c_void as *mut u8,
            0 as i32 as u8,
            ::core::mem::size_of::<VP8D_COMP>() as size_t,
        );
        if setjmp(&raw mut (*pbi).common.error.jmp as *mut i32) != 0 {
            (*pbi).common.error.setjmp = 0 as i32;
            remove_decompressor(pbi);
            return ::core::ptr::null_mut::<VP8D_COMP>();
        }
        (*pbi).common.error.setjmp = 1 as i32;
        vp8_create_common(&raw mut (*pbi).common);
        (*pbi).common.current_video_frame = 0 as u32;
        (*pbi).ready_for_new_data = 1 as i32;
        vp8cx_init_de_quantizer(pbi);
        vp8_loop_filter_init(&raw mut (*pbi).common);
        (*pbi).common.error.setjmp = 0 as i32;
        (*pbi).ec_enabled = 0 as i32;
        (*pbi).ec_active = 0 as i32;
        (*pbi).decoded_key_frame = 0 as i32;
        (*pbi).independent_partitions = 0 as i32;
        vp8_setup_block_dptrs(&raw mut (*pbi).mb);
        once(Some(initialize_dec as unsafe fn() -> ()));
        pbi as *mut VP8D_COMP
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    unsafe {
        let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut ref_fb_idx: i32 = 0;
        if ref_frame_flag as u32 == VP8_LAST_FRAME as u32 {
            ref_fb_idx = (*cm).lst_fb_idx;
        } else if ref_frame_flag as u32 == VP8_GOLD_FRAME as u32 {
            ref_fb_idx = (*cm).gld_fb_idx;
        } else if ref_frame_flag as u32 == VP8_ALTR_FRAME as u32 {
            ref_fb_idx = (*cm).alt_fb_idx;
        } else {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_ERROR,
                b"Invalid reference frame\0" as *const u8 as *const i8,
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
                b"Incorrect buffer dimensions\0" as *const u8 as *const i8,
            );
        } else {
            vp8_yv12_copy_frame_c(
                (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(ref_fb_idx as isize)
                    as *mut YV12_BUFFER_CONFIG,
                sd as *mut yv12_buffer_config,
            );
        }
        (*pbi).common.error.error_code
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_set_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    unsafe {
        let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut ref_fb_ptr: *mut i32 = ::core::ptr::null_mut::<i32>();
        let mut free_fb: i32 = 0;
        if ref_frame_flag as u32 == VP8_LAST_FRAME as u32 {
            ref_fb_ptr = &raw mut (*cm).lst_fb_idx;
        } else if ref_frame_flag as u32 == VP8_GOLD_FRAME as u32 {
            ref_fb_ptr = &raw mut (*cm).gld_fb_idx;
        } else if ref_frame_flag as u32 == VP8_ALTR_FRAME as u32 {
            ref_fb_ptr = &raw mut (*cm).alt_fb_idx;
        } else {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_ERROR,
                b"Invalid reference frame\0" as *const u8 as *const i8,
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
                b"Incorrect buffer dimensions\0" as *const u8 as *const i8,
            );
        } else {
            free_fb = get_free_fb(cm);
            (*cm).fb_idx_ref_cnt[free_fb as usize] -= 1;
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
                ref_fb_ptr,
                free_fb,
            );
            vp8_yv12_copy_frame_c(
                sd,
                (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(*ref_fb_ptr as isize)
                    as *mut yv12_buffer_config,
            );
        }
        (*pbi).common.error.error_code
    }
}
unsafe fn get_free_fb(mut cm: *mut VP8_COMMON) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < NUM_YV12_BUFFERS {
            if (*cm).fb_idx_ref_cnt[i as usize] == 0 as i32 {
                break;
            }
            i += 1;
        }
        (*cm).fb_idx_ref_cnt[i as usize] = 1 as i32;
        i
    }
}
unsafe fn ref_cnt_fb(mut buf: *mut i32, mut idx: *mut i32, mut new_idx: i32) {
    unsafe {
        if *buf.offset(*idx as isize) > 0 as i32 {
            let fresh0 = &mut *buf.offset(*idx as isize);
            *fresh0 -= 1;
        }
        *idx = new_idx;
        let fresh1 = &mut *buf.offset(new_idx as isize);
        *fresh1 += 1;
    }
}
unsafe fn swap_frame_buffers(mut cm: *mut VP8_COMMON) -> i32 {
    unsafe {
        let mut err: i32 = 0 as i32;
        if (*cm).copy_buffer_to_arf != 0 {
            let mut new_fb: i32 = 0 as i32;
            if (*cm).copy_buffer_to_arf == 1 as i32 {
                new_fb = (*cm).lst_fb_idx;
            } else if (*cm).copy_buffer_to_arf == 2 as i32 {
                new_fb = (*cm).gld_fb_idx;
            } else {
                err = -(1 as i32);
            }
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
                &raw mut (*cm).alt_fb_idx,
                new_fb,
            );
        }
        if (*cm).copy_buffer_to_gf != 0 {
            let mut new_fb_0: i32 = 0 as i32;
            if (*cm).copy_buffer_to_gf == 1 as i32 {
                new_fb_0 = (*cm).lst_fb_idx;
            } else if (*cm).copy_buffer_to_gf == 2 as i32 {
                new_fb_0 = (*cm).alt_fb_idx;
            } else {
                err = -(1 as i32);
            }
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
                &raw mut (*cm).gld_fb_idx,
                new_fb_0,
            );
        }
        if (*cm).refresh_golden_frame != 0 {
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
                &raw mut (*cm).gld_fb_idx,
                (*cm).new_fb_idx,
            );
        }
        if (*cm).refresh_alt_ref_frame != 0 {
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
                &raw mut (*cm).alt_fb_idx,
                (*cm).new_fb_idx,
            );
        }
        if (*cm).refresh_last_frame != 0 {
            ref_cnt_fb(
                &raw mut (*cm).fb_idx_ref_cnt as *mut i32,
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
        err
    }
}
unsafe fn check_fragments_for_errors(mut pbi: *mut VP8D_COMP) -> i32 {
    unsafe {
        if (*pbi).ec_active == 0
            && (*pbi).fragments.count <= 1 as u32
            && (*pbi).fragments.sizes[0 as usize] == 0 as u32
        {
            let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
            if (*cm).fb_idx_ref_cnt[(*cm).lst_fb_idx as usize] > 1 as i32 {
                let prev_idx: i32 = (*cm).lst_fb_idx;
                (*cm).fb_idx_ref_cnt[prev_idx as usize] -= 1;
                (*cm).lst_fb_idx = get_free_fb(cm);
                vp8_yv12_copy_frame_c(
                    (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(prev_idx as isize)
                        as *mut YV12_BUFFER_CONFIG,
                    (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
                        .offset((*cm).lst_fb_idx as isize)
                        as *mut yv12_buffer_config,
                );
            }
            (*cm).yv12_fb[(*cm).lst_fb_idx as usize].corrupted = 1 as i32;
            (*cm).show_frame = 0 as i32;
            return 0 as i32;
        }
        1 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_receive_compressed_data(mut pbi: *mut VP8D_COMP) -> i32 {
    unsafe {
        let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
        let mut retcode: i32 = -(1 as i32);
        (*pbi).common.error.error_code = VPX_CODEC_OK;
        retcode = check_fragments_for_errors(pbi);
        if retcode <= 0 as i32 {
            return retcode;
        }
        (*cm).new_fb_idx = get_free_fb(cm);
        (*pbi).dec_fb_ref[INTRA_FRAME as usize] =
            (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).new_fb_idx as isize)
                as *mut YV12_BUFFER_CONFIG;
        (*pbi).dec_fb_ref[LAST_FRAME as usize] = (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
            .offset((*cm).lst_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
        (*pbi).dec_fb_ref[GOLDEN_FRAME as usize] =
            (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).gld_fb_idx as isize)
                as *mut YV12_BUFFER_CONFIG;
        (*pbi).dec_fb_ref[ALTREF_FRAME as usize] =
            (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).alt_fb_idx as isize)
                as *mut YV12_BUFFER_CONFIG;
        retcode = vp8_decode_frame(pbi);
        if retcode < 0 as i32 {
            if (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] > 0 as i32 {
                (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
            }
            (*pbi).common.error.error_code = VPX_CODEC_ERROR;
            if (*pbi).mb.error_info.error_code as u32 != 0 as u32 {
                (*pbi).common.error.error_code = (*pbi).mb.error_info.error_code;
                core::ptr::copy_nonoverlapping(
                    &raw mut (*pbi).mb.error_info.detail as *mut i8 as *const c_void as *const u8,
                    &raw mut (*pbi).common.error.detail as *mut i8 as *mut c_void as *mut u8,
                    ::core::mem::size_of::<[i8; 80]>() as size_t,
                );
            }
        } else if swap_frame_buffers(cm) != 0 {
            (*pbi).common.error.error_code = VPX_CODEC_ERROR;
        } else {
            if (*cm).show_frame != 0 {
                (*cm).current_video_frame = (*cm).current_video_frame.wrapping_add(1);
                (*cm).show_frame_mi = (*cm).mi;
            }
            (*pbi).ready_for_new_data = 0 as i32;
        }
        retcode
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_raw_frame(
    mut pbi: *mut VP8D_COMP,
    mut sd: *mut YV12_BUFFER_CONFIG,
    _flags: *mut vp8_ppflags_t,
) -> i32 {
    unsafe {
        let mut ret: i32 = -(1 as i32);
        if (*pbi).ready_for_new_data == 1 as i32 {
            return ret;
        }
        if (*pbi).common.show_frame == 0 as i32 {
            return ret;
        }
        (*pbi).ready_for_new_data = 1 as i32;
        if !(*pbi).common.frame_to_show.is_null() {
            *sd = *(*pbi).common.frame_to_show;
            (*sd).y_width = (*pbi).common.width;
            (*sd).y_height = (*pbi).common.height;
            (*sd).uv_height = (*pbi).common.height / 2 as i32;
            ret = 0 as i32;
        } else {
            ret = -(1 as i32);
        }
        ret
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_references_buffer(mut oci: *mut VP8_COMMON, mut ref_frame: i32) -> i32 {
    unsafe {
        let mut mi: *const MODE_INFO = (*oci).mi;
        let mut mb_row: i32 = 0;
        let mut mb_col: i32 = 0;
        mb_row = 0 as i32;
        while mb_row < (*oci).mb_rows {
            mb_col = 0 as i32;
            while mb_col < (*oci).mb_cols {
                if (*mi).mbmi.ref_frame as i32 == ref_frame {
                    return 1 as i32;
                }
                mb_col += 1;
                mi = mi.offset(1);
            }
            mi = mi.offset(1);
            mb_row += 1;
        }
        0 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_create_decoder_instances(
    mut fb: *mut frame_buffers,
    mut oxcf: *mut VP8D_CONFIG,
) -> i32 {
    unsafe {
        (*fb).pbi[0 as usize] = create_decompressor(oxcf);
        if (*fb).pbi[0 as usize].is_null() {
            return VPX_CODEC_ERROR as i32;
        }
        if setjmp(
            &raw mut (**(&raw mut (*fb).pbi as *mut *mut VP8D_COMP).offset(0 as isize))
                .common
                .error
                .jmp as *mut i32,
        ) != 0
        {
            (*(*fb).pbi[0 as usize]).common.error.setjmp = 0 as i32;
            vp8_remove_decoder_instances(fb);
            core::ptr::write_bytes(
                &raw mut (*fb).pbi as *mut c_void as *mut u8,
                0 as i32 as u8,
                ::core::mem::size_of::<[*mut VP8D_COMP; 32]>() as size_t,
            );
            return VPX_CODEC_ERROR as i32;
        }
        (*(*fb).pbi[0 as usize]).common.error.setjmp = 1 as i32;
        (*(*fb).pbi[0 as usize]).max_threads = (*oxcf).max_threads;
        vp8_decoder_create_threads((*fb).pbi[0 as usize]);
        (*(*fb).pbi[0 as usize]).common.error.setjmp = 0 as i32;
        VPX_CODEC_OK as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_remove_decoder_instances(mut fb: *mut frame_buffers) -> i32 {
    unsafe {
        let mut pbi: *mut VP8D_COMP = (*fb).pbi[0 as usize];
        if pbi.is_null() {
            return VPX_CODEC_ERROR as i32;
        }
        vp8_decoder_remove_threads(pbi);
        remove_decompressor(pbi);
        (*fb).pbi[0 as usize] = ::core::ptr::null_mut::<VP8D_COMP>();
        VPX_CODEC_OK as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_quantizer(mut pbi: *const VP8D_COMP) -> i32 {
    unsafe { (*pbi).common.base_qindex }
}
pub const NULL: *mut c_void = __DARWIN_NULL;
