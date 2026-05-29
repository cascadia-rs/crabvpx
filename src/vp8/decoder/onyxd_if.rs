use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(info: *mut VpxInternalErrorInfo, error: u32, fmt: *const i8);
    fn vp8_loop_filter_init(cm: *mut VP8Common);
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
    fn pthread_once(_: *mut PthreadOnceT, _: Option<unsafe fn() -> ()>) -> i32;
    fn vp8cx_init_de_quantizer(pbi: *mut Vp8dComp);
    fn vp8_decode_frame(pbi: *mut Vp8dComp) -> i32;
    fn vpx_memalign(align: usize, size: usize) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8_create_common(oci: *mut Vp8Common);
    fn vp8_remove_common(oci: *mut Vp8Common);
    fn vp8_decoder_remove_threads(pbi: *mut Vp8dComp);
    fn vp8_decoder_create_threads(pbi: *mut Vp8dComp);
    fn vp8_init_intra_predictors();
    fn vpx_dsp_rtcd();
    fn vp8_yv12_copy_frame_c(src_ybc: *const Yv12BufferConfig, dst_ybc: *mut Yv12BufferConfig);
}
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

pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};
pub type BLOCKD = Blockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DarwinPthreadHandlerRec {
    pub __routine: Option<unsafe fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut DarwinPthreadHandlerRec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadOnceT {
    pub __sig: i64,
    pub __opaque: [i8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadT {
    pub __sig: i64,
    pub __cleanup_stack: *mut DarwinPthreadHandlerRec,
    pub __opaque: [i8; 8176],
}
pub type DarwinPthreadOnceT = OpaquePthreadOnceT;
pub type DarwinPthreadT = *mut OpaquePthreadT;
pub const SIMPLE_LOOPFILTER: u32 = 1;
pub const NORMAL_LOOPFILTER: u32 = 0;
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
pub const RECON_CLAMP_NOTREQUIRED: u32 = 1;
pub const RECON_CLAMP_REQUIRED: u32 = 0;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed = 4;
pub const ALTREF_FRAME: C2RustUnnamed = 3;
pub const GOLDEN_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
pub type MACROBLOCKD = Macroblockd;
pub type Vp8Common = VP8Common;
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
pub const VP8_ALTR_FRAME: u32 = 4;
pub const VP8_GOLD_FRAME: u32 = 2;
pub const VP8_LAST_FRAME: u32 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxAtomicInt {
    pub value: i32,
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
pub type PthreadOnceT = *mut c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameBuffers {
    pub pbi: [*mut Vp8dComp; 32],
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
        vpx_dsp_rtcd();
        vp8_init_intra_predictors();
    }
}
unsafe fn remove_decompressor(mut pbi: *mut Vp8dComp) {
    unsafe {
        vp8_remove_common(&raw mut (*pbi).common);
        vpx_free(pbi as *mut c_void);
    }
}
unsafe fn create_decompressor(_oxcf: *mut Vp8dConfig) -> *mut Vp8dComp {
    unsafe {
        let mut pbi: *mut Vp8dComp =
            vpx_memalign(32 as usize, ::core::mem::size_of::<Vp8dComp>() as usize) as *mut Vp8dComp;
        if pbi.is_null() {
            return ::core::ptr::null_mut::<Vp8dComp>();
        }
        core::ptr::write_bytes(
            pbi as *mut c_void as *mut u8,
            0 as u8,
            ::core::mem::size_of::<Vp8dComp>() as usize,
        );
        if setjmp(&raw mut (*pbi).common.error.jmp as *mut i32) != 0 {
            (*pbi).common.error.setjmp = false;
            remove_decompressor(pbi);
            return ::core::ptr::null_mut::<Vp8dComp>();
        }
        (*pbi).common.error.setjmp = true;
        vp8_create_common(&raw mut (*pbi).common);
        (*pbi).common.current_video_frame = 0 as u32;
        (*pbi).ready_for_new_data = true;
        vp8cx_init_de_quantizer(pbi);
        vp8_loop_filter_init(&raw mut (*pbi).common);
        (*pbi).common.error.setjmp = false;
        (*pbi).ec_enabled = false;
        (*pbi).ec_active = false;
        (*pbi).decoded_key_frame = false;
        (*pbi).independent_partitions = false;
        vp8_setup_block_dptrs(&raw mut (*pbi).mb);
        once(Some(initialize_dec as unsafe fn() -> ()));
        pbi as *mut Vp8dComp
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_reference(
    mut pbi: *mut Vp8dComp,
    mut ref_frame_flag: u32,
    mut sd: *mut Yv12BufferConfig,
) -> u32 {
    unsafe {
        let mut cm: *mut Vp8Common = &raw mut (*pbi).common;
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
                (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig).offset(ref_fb_idx as isize)
                    as *mut Yv12BufferConfig,
                sd as *mut Yv12BufferConfig,
            );
        }
        (*pbi).common.error.error_code
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_set_reference(
    mut pbi: *mut Vp8dComp,
    mut ref_frame_flag: u32,
    mut sd: *mut Yv12BufferConfig,
) -> u32 {
    unsafe {
        let mut cm: *mut Vp8Common = &raw mut (*pbi).common;
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
                (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig).offset(*ref_fb_ptr as isize)
                    as *mut Yv12BufferConfig,
            );
        }
        (*pbi).common.error.error_code
    }
}
unsafe fn get_free_fb(mut cm: *mut Vp8Common) -> i32 {
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
unsafe fn swap_frame_buffers(mut cm: *mut Vp8Common) -> i32 {
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
            (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
                .offset((*cm).lst_fb_idx as isize)
                as *mut Yv12BufferConfig;
        } else {
            (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
                .offset((*cm).new_fb_idx as isize)
                as *mut Yv12BufferConfig;
        }
        (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
        err
    }
}
unsafe fn check_fragments_for_errors(mut pbi: *mut Vp8dComp) -> i32 {
    unsafe {
        if !(*pbi).ec_active
            && (*pbi).fragments.count <= 1 as u32
            && (*pbi).fragments.sizes[0 as usize] == 0 as u32
        {
            let mut cm: *mut Vp8Common = &raw mut (*pbi).common;
            if (*cm).fb_idx_ref_cnt[(*cm).lst_fb_idx as usize] > 1 as i32 {
                let prev_idx: i32 = (*cm).lst_fb_idx;
                (*cm).fb_idx_ref_cnt[prev_idx as usize] -= 1;
                (*cm).lst_fb_idx = get_free_fb(cm);
                vp8_yv12_copy_frame_c(
                    (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig).offset(prev_idx as isize)
                        as *mut Yv12BufferConfig,
                    (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
                        .offset((*cm).lst_fb_idx as isize)
                        as *mut Yv12BufferConfig,
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
pub unsafe fn vp8dx_receive_compressed_data(mut pbi: *mut Vp8dComp) -> i32 {
    unsafe {
        let mut cm: *mut Vp8Common = &raw mut (*pbi).common;
        let mut retcode: i32 = -(1 as i32);
        (*pbi).common.error.error_code = VPX_CODEC_OK;
        retcode = check_fragments_for_errors(pbi);
        if retcode <= 0 as i32 {
            return retcode;
        }
        (*cm).new_fb_idx = get_free_fb(cm);
        (*pbi).dec_fb_ref[INTRA_FRAME as usize] = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
            .offset((*cm).new_fb_idx as isize)
            as *mut Yv12BufferConfig;
        (*pbi).dec_fb_ref[LAST_FRAME as usize] = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
            .offset((*cm).lst_fb_idx as isize)
            as *mut Yv12BufferConfig;
        (*pbi).dec_fb_ref[GOLDEN_FRAME as usize] = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
            .offset((*cm).gld_fb_idx as isize)
            as *mut Yv12BufferConfig;
        (*pbi).dec_fb_ref[ALTREF_FRAME as usize] = (&raw mut (*cm).yv12_fb as *mut Yv12BufferConfig)
            .offset((*cm).alt_fb_idx as isize)
            as *mut Yv12BufferConfig;
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
                    ::core::mem::size_of::<[i8; 80]>() as usize,
                );
            }
        } else if swap_frame_buffers(cm) != 0 {
            (*pbi).common.error.error_code = VPX_CODEC_ERROR;
        } else {
            if (*cm).show_frame != 0 {
                (*cm).current_video_frame = (*cm).current_video_frame.wrapping_add(1);
                (*cm).show_frame_mi = (*cm).mi;
            }
            (*pbi).ready_for_new_data = false;
        }
        retcode
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_raw_frame(
    mut pbi: *mut Vp8dComp,
    mut sd: *mut Yv12BufferConfig,
    _flags: *mut Vp8PpflagsT,
) -> i32 {
    unsafe {
        let mut ret: i32 = -(1 as i32);
        if (*pbi).ready_for_new_data {
            return ret;
        }
        if (*pbi).common.show_frame == 0 as i32 {
            return ret;
        }
        (*pbi).ready_for_new_data = true;
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
pub unsafe fn vp8dx_references_buffer(mut oci: *mut Vp8Common, mut ref_frame: i32) -> i32 {
    unsafe {
        let mut mi: *const ModeInfo = (*oci).mi;
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
    mut fb: *mut FrameBuffers,
    mut oxcf: *mut Vp8dConfig,
) -> i32 {
    unsafe {
        (*fb).pbi[0 as usize] = create_decompressor(oxcf);
        if (*fb).pbi[0 as usize].is_null() {
            return VPX_CODEC_ERROR as i32;
        }
        if setjmp(
            &raw mut (**(&raw mut (*fb).pbi as *mut *mut Vp8dComp).offset(0 as isize))
                .common
                .error
                .jmp as *mut i32,
        ) != 0
        {
            (*(*fb).pbi[0 as usize]).common.error.setjmp = false;
            vp8_remove_decoder_instances(fb);
            core::ptr::write_bytes(
                &raw mut (*fb).pbi as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[*mut Vp8dComp; 32]>() as usize,
            );
            return VPX_CODEC_ERROR as i32;
        }
        (*(*fb).pbi[0 as usize]).common.error.setjmp = true;
        (*(*fb).pbi[0 as usize]).max_threads = (*oxcf).max_threads;
        vp8_decoder_create_threads((*fb).pbi[0 as usize]);
        (*(*fb).pbi[0 as usize]).common.error.setjmp = false;
        VPX_CODEC_OK as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_remove_decoder_instances(mut fb: *mut FrameBuffers) -> i32 {
    unsafe {
        let mut pbi: *mut Vp8dComp = (*fb).pbi[0 as usize];
        if pbi.is_null() {
            return VPX_CODEC_ERROR as i32;
        }
        vp8_decoder_remove_threads(pbi);
        remove_decompressor(pbi);
        (*fb).pbi[0 as usize] = ::core::ptr::null_mut::<Vp8dComp>();
        VPX_CODEC_OK as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_get_quantizer(mut pbi: *const Vp8dComp) -> i32 {
    unsafe { (*pbi).common.base_qindex }
}
pub const NULL: *mut c_void = __DARWIN_NULL;
