use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
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
    fn vp8_dequantize_b_c(_: *mut Blockd, dqc: *mut i16);
    fn vp8_loop_filter_bh_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut LoopFilterInfo,
    );
    fn vp8_loop_filter_bv_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut LoopFilterInfo,
    );
    fn vp8_loop_filter_mbh_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut LoopFilterInfo,
    );
    fn vp8_loop_filter_mbv_c(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: i32,
        uv_stride: i32,
        lfi: *mut LoopFilterInfo,
    );
    fn vp8_loop_filter_bhs_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_bvs_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_simple_horizontal_edge_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_loop_filter_simple_vertical_edge_c(y_ptr: *mut u8, y_stride: i32, blimit: *const u8);
    fn vp8_short_inv_walsh4x4_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn vp8_short_inv_walsh4x4_1_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn pthread_create(
        _: *mut PthreadT,
        _: *const PthreadAttrT,
        _: Option<unsafe fn(*mut c_void) -> *mut c_void>,
        _: *mut c_void,
    ) -> i32;
    fn pthread_join(_: PthreadT, _: *mut *mut c_void) -> i32;
    fn setjmp(_: *mut i32) -> i32;
    fn vpx_internal_error(info: *mut VpxInternalErrorInfo, error: u32, fmt: *const i8);
    fn vp8_loop_filter_frame_init(cm: *mut VP8Common, mbd: *mut Macroblockd, default_filt_lvl: i32);
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
    // static mut mach_task_self_: u32;
    fn semaphore_signal(semaphore: SemaphoreT) -> i32;
    fn semaphore_wait(semaphore: SemaphoreT) -> i32;
    fn semaphore_create(task: u32, semaphore: *mut SemaphoreT, policy: i32, value: i32) -> i32;
    fn semaphore_destroy(task: u32, semaphore: SemaphoreT) -> i32;
    fn vp8_mb_init_dequantizer(pbi: *mut Vp8dComp, xd: *mut MACROBLOCKD);
    fn vpx_malloc(size: usize) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8_extend_mb_row(ybf: *mut Yv12BufferConfig, yptr: *mut u8, uptr: *mut u8, vptr: *mut u8);
    fn vp8_reset_mb_tokens_context(x: *mut MACROBLOCKD);
    fn vp8_decode_mb_tokens(_: *mut Vp8dComp, _: *mut MACROBLOCKD) -> i32;
    fn vp8_intra4x4_predict(
        above: *mut u8,
        yleft: *mut u8,
        left_stride: i32,
        b_mode: u32,
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
    fn vp8_setup_intra_recon_top_line(ybf: *mut Yv12BufferConfig);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterInfo {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DarwinPthreadHandlerRec {
    pub __routine: Option<unsafe fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut DarwinPthreadHandlerRec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadAttrT {
    pub __sig: i64,
    pub __opaque: [i8; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadT {
    pub __sig: i64,
    pub __cleanup_stack: *mut DarwinPthreadHandlerRec,
    pub __opaque: [i8; 8176],
}
pub type DarwinPthreadAttrT = OpaquePthreadAttrT;
pub type DarwinPthreadT = *mut OpaquePthreadT;
pub type PthreadAttrT = *mut c_void;
pub type PthreadT = *mut c_void;
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
pub const MB_MODE_COUNT: u32 = 10;
pub const SPLITMV: u32 = 9;
pub const NEWMV: u32 = 8;
pub const ZEROMV: u32 = 7;
pub const NEARMV: u32 = 6;
pub const NEARESTMV: u32 = 5;
pub const B_PRED: u32 = 4;
pub const TM_PRED: u32 = 3;
pub const H_PRED: u32 = 2;
pub const V_PRED: u32 = 1;
pub const DC_PRED: u32 = 0;
pub const MAX_REF_FRAMES: u32 = 4;
pub const ALTREF_FRAME: u32 = 3;
pub const GOLDEN_FRAME: u32 = 2;
pub const LAST_FRAME: u32 = 1;
pub const INTRA_FRAME: u32 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const THREAD_EXIT_SUCCESS: *mut c_void = NULL;
pub const CHAR_BIT: i32 = 8 as i32;
pub const VP8BORDERINPIXELS: i32 = 32 as i32;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<usize>() as i32 * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: i32 = 0x40000000 as i32;
#[inline]
unsafe fn vp8dx_bool_error(mut br: *mut BoolDecoder) -> i32 {
    unsafe {
        if (*br).count > VP8_BD_VALUE_SIZE && (*br).count < VP8_LOTS_OF_BITS {
            return 1 as i32;
        }
        0 as i32
    }
}
pub const SYNC_POLICY_FIFO: i32 = 0 as i32;
#[inline]
unsafe fn vpx_atomic_init(mut atomic: *mut VpxAtomicInt, mut value: i32) {
    unsafe {
        ::core::ptr::write_volatile(&mut (*atomic).value as *mut i32, value);
    }
}
#[inline]
unsafe fn vpx_atomic_store_release(mut atomic: *mut VpxAtomicInt, mut value: i32) {
    unsafe {
        (*(&raw mut (*atomic).value as *const core::sync::atomic::AtomicI32))
            .store(value, core::sync::atomic::Ordering::Release);
    }
}
#[inline]
unsafe fn vpx_atomic_load_acquire(mut atomic: *const VpxAtomicInt) -> i32 {
    unsafe {
        (*(atomic as *const core::sync::atomic::AtomicI32))
            .load(core::sync::atomic::Ordering::Acquire)
    }
}
#[inline]
unsafe fn vp8_atomic_spin_wait(
    mut mb_col: i32,
    mut last_row_current_mb_col: *const VpxAtomicInt,
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
    mut pbi: *mut Vp8dComp,
    mut xd: *mut MACROBLOCKD,
    mut mbrd: *mut MbRowDec,
    mut count: i32,
) {
    unsafe {
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
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
                ::core::mem::size_of::<[[i8; 4]; 2]>() as usize,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).ref_lf_deltas as *mut i8 as *const c_void as *const u8,
                &raw mut (*mbd).ref_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i8; 4]>() as usize,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).mode_lf_deltas as *mut i8 as *const c_void as *const u8,
                &raw mut (*mbd).mode_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i8; 4]>() as usize,
            );
            (*mbd).mode_ref_lf_delta_enabled = (*xd).mode_ref_lf_delta_enabled;
            (*mbd).mode_ref_lf_delta_update = (*xd).mode_ref_lf_delta_update;
            (*mbd).current_bc = (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(0 as isize)
                as *mut Vp8Reader as *mut c_void;
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y1_dc as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y1_dc as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as usize,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y1 as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y1 as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as usize,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_y2 as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_y2 as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as usize,
            );
            core::ptr::copy_nonoverlapping(
                &raw mut (*xd).dequant_uv as *mut i16 as *const c_void as *const u8,
                &raw mut (*mbd).dequant_uv as *mut i16 as *mut c_void as *mut u8,
                ::core::mem::size_of::<[i16; 16]>() as usize,
            );
            (*mbd).fullpixel_mask = !(0 as i32);
            if (*pc).full_pixel {
                (*mbd).fullpixel_mask = !(7 as i32);
            }
            i += 1;
        }
        i = 0 as i32;
        while i < (*pc).mb_rows {
            vpx_atomic_store_release(
                (*pbi).mt_current_mb_col.offset(i as isize) as *mut VpxAtomicInt,
                -(1 as i32),
            );
            i += 1;
        }
    }
}
unsafe fn mt_decode_macroblock(mut pbi: *mut Vp8dComp, mut xd: *mut MACROBLOCKD, _mb_idx: u32) {
    unsafe {
        let mut mode: u32 = DC_PRED;
        let mut i: i32 = 0;
        if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
            vp8_reset_mb_tokens_context(xd);
        } else if vp8dx_bool_error((*xd).current_bc as *mut BoolDecoder) == 0 {
            let mut eobtotal: i32 = 0;
            eobtotal = vp8_decode_mb_tokens(pbi, xd);
            (*(*xd).mode_info_context).mbmi.mb_skip_coeff = (eobtotal == 0 as i32) as u8;
        }
        mode = (*(*xd).mode_info_context).mbmi.mode as u32;
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
                let mut dqc: *mut i16 = &raw mut (*xd).dequant_y1 as *mut i16;
                let mut dst_stride: i32 = (*xd).dst.y_stride;
                if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
                    core::ptr::write_bytes(
                        &raw mut (*xd).eobs as *mut i8 as *mut c_void as *mut u8,
                        0 as u8,
                        25 as usize,
                    );
                }
                intra_prediction_down_copy(xd, (*xd).recon_above[0 as usize].offset(16 as isize));
                i = 0 as i32;
                while i < 16 as i32 {
                    let mut b: *mut BLOCKD =
                        (&raw mut (*xd).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                    let mut dst: *mut u8 = (*xd).dst.y_buffer.offset((*b).offset as isize);
                    let mut b_mode: u32 = (*(*xd).mode_info_context).bmi[i as usize].as_mode;
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
                            vp8_dequant_idct_add_c((*b).qcoeff, dqc, dst, dst_stride);
                        } else {
                            vp8_dc_only_idct_add_c(
                                (*(*b).qcoeff.offset(0 as isize) as i32
                                    * *dqc.offset(0 as isize) as i32)
                                    as i16,
                                dst,
                                dst_stride,
                                dst,
                                dst_stride,
                            );
                            core::ptr::write_bytes(
                                (*b).qcoeff as *mut c_void as *mut u8,
                                0 as u8,
                                (2 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
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
                let mut dqc_0: *mut i16 = &raw mut (*xd).dequant_y1 as *mut i16;
                if mode as u32 != SPLITMV as u32 {
                    let mut b_0: *mut BLOCKD =
                        (&raw mut (*xd).block as *mut BLOCKD).offset(24 as isize) as *mut BLOCKD;
                    if (*xd).eobs[24 as usize] as i32 > 1 as i32 {
                        vp8_dequantize_b_c(
                            b_0 as *mut Blockd,
                            &raw mut (*xd).dequant_y2 as *mut i16,
                        );
                        vp8_short_inv_walsh4x4_c(
                            (*b_0).dqcoeff.offset(0 as isize) as *mut i16,
                            &raw mut (*xd).qcoeff as *mut i16,
                        );
                        core::ptr::write_bytes(
                            (*b_0).qcoeff as *mut c_void as *mut u8,
                            0 as u8,
                            (16 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
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
                            0 as u8,
                            (2 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
                        );
                    }
                    dqc_0 = &raw mut (*xd).dequant_y1_dc as *mut i16;
                }
                vp8_dequant_idct_add_y_block_c(
                    &raw mut (*xd).qcoeff as *mut i16,
                    dqc_0,
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
    mut pbi: *mut Vp8dComp,
    mut xd: *mut MACROBLOCKD,
    mut start_mb_row: i32,
) {
    unsafe {
        let mut last_row_current_mb_col: *const VpxAtomicInt = ::core::ptr::null::<VpxAtomicInt>();
        let mut current_mb_col: *mut VpxAtomicInt = ::core::ptr::null_mut::<VpxAtomicInt>();
        let mut mb_row: i32 = 0;
        let mut pc: *mut Vp8Common = &raw mut (*pbi).common;
        let nsync: i32 = (*pbi).sync_range;
        let first_row_no_sync_above: VpxAtomicInt = VpxAtomicInt {
            value: (*pc).mb_cols + nsync,
        };
        let mut num_part: i32 = (1 as i32) << (*pbi).common.multi_token_partition as u32;
        let mut last_mb_row: i32 = start_mb_row;
        let mut yv12_fb_new: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        let mut yv12_fb_lst: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[LAST_FRAME as usize];
        let mut recon_y_stride: i32 = (*yv12_fb_new).y_stride;
        let mut recon_uv_stride: i32 = (*yv12_fb_new).uv_stride;
        let mut ref_buffer: [[*mut u8; 3]; 4] = [[::core::ptr::null_mut::<u8>(); 3]; 4];
        let mut dst_buffer: [*mut u8; 3] = [::core::ptr::null_mut::<u8>(); 3];
        let mut i: i32 = 0;
        let mut ref_fb_corrupted: [i32; 4] = [0; 4];
        ref_fb_corrupted[INTRA_FRAME as usize] = 0 as i32;
        i = 1 as i32;
        while i < MAX_REF_FRAMES as i32 {
            let mut this_fb: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[i as usize];
            ref_buffer[i as usize][0 as usize] = (*this_fb).y_buffer as *mut u8;
            ref_buffer[i as usize][1 as usize] = (*this_fb).u_buffer as *mut u8;
            ref_buffer[i as usize][2 as usize] = (*this_fb).v_buffer as *mut u8;
            ref_fb_corrupted[i as usize] = (*this_fb).corrupted;
            i += 1;
        }
        dst_buffer[0 as usize] = (*yv12_fb_new).y_buffer as *mut u8;
        dst_buffer[1 as usize] = (*yv12_fb_new).u_buffer as *mut u8;
        dst_buffer[2 as usize] = (*yv12_fb_new).v_buffer as *mut u8;
        (*xd).up_available = start_mb_row != 0;
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
            let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*pc).lf_info;
            last_mb_row = mb_row;
            (*xd).current_bc = (&raw mut (*pbi).mbc as *mut Vp8Reader)
                .offset((mb_row % num_part) as isize)
                as *mut Vp8Reader as *mut c_void;
            if mb_row > 0 as i32 {
                last_row_current_mb_col = (*pbi)
                    .mt_current_mb_col
                    .offset((mb_row - 1 as i32) as isize)
                    as *mut VpxAtomicInt;
            } else {
                last_row_current_mb_col = &raw const first_row_no_sync_above;
            }
            current_mb_col = (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut VpxAtomicInt;
            recon_yoffset = mb_row * recon_y_stride * 16 as i32;
            recon_uvoffset = mb_row * recon_uv_stride * 8 as i32;
            (*xd).above_context = (*pc).above_context;
            core::ptr::write_bytes(
                (*xd).left_context as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<EntropyContextPlanes>() as usize,
            );
            (*xd).left_available = false;
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
                    dst_buffer[0 as usize].offset(recon_yoffset as isize) as *mut u8;
                (*xd).dst.u_buffer =
                    dst_buffer[1 as usize].offset(recon_uvoffset as isize) as *mut u8;
                (*xd).dst.v_buffer =
                    dst_buffer[2 as usize].offset(recon_uvoffset as isize) as *mut u8;
                (*xd).corrupted |=
                    ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
                if (*xd).corrupted != 0 {
                    while mb_row < (*pc).mb_rows {
                        current_mb_col =
                            (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut VpxAtomicInt;
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
                    let ref_0: u32 = (*(*xd).mode_info_context).mbmi.ref_frame as u32;
                    (*xd).pre.y_buffer = ref_buffer[ref_0 as usize][0 as usize]
                        .offset(recon_yoffset as isize)
                        as *mut u8;
                    (*xd).pre.u_buffer = ref_buffer[ref_0 as usize][1 as usize]
                        .offset(recon_uvoffset as isize)
                        as *mut u8;
                    (*xd).pre.v_buffer = ref_buffer[ref_0 as usize][2 as usize]
                        .offset(recon_uvoffset as isize)
                        as *mut u8;
                } else {
                    (*xd).pre.y_buffer = ::core::ptr::null_mut::<u8>();
                    (*xd).pre.u_buffer = ::core::ptr::null_mut::<u8>();
                    (*xd).pre.v_buffer = ::core::ptr::null_mut::<u8>();
                }
                mt_decode_macroblock(pbi, xd, 0 as u32);
                (*xd).left_available = true;
                (*xd).corrupted |= vp8dx_bool_error((*xd).current_bc as *mut BoolDecoder);
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
                            16 as usize,
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
                            8 as usize,
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
                            8 as usize,
                        );
                    }
                    if mb_col != (*pc).mb_cols - 1 as i32 {
                        let mut next: *mut ModeInfo = (*xd).mode_info_context.offset(1 as isize);
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
                            let mut lfi: LoopFilterInfo = LoopFilterInfo {
                                mblim: ::core::ptr::null::<u8>(),
                                blim: ::core::ptr::null::<u8>(),
                                lim: ::core::ptr::null::<u8>(),
                                hev_thr: ::core::ptr::null::<u8>(),
                            };
                            let mut frame_type: u32 = (*pc).frame_type;
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
            (*xd).up_available = true;
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
        let mut ithread: i32 = (*(p_data as *mut DecodethreadData)).ithread;
        let mut pbi: *mut Vp8dComp = (*(p_data as *mut DecodethreadData)).ptr1 as *mut Vp8dComp;
        let mut mbrd: *mut MbRowDec = (*(p_data as *mut DecodethreadData)).ptr2 as *mut MbRowDec;
        let mut mb_row_left_context: EntropyContextPlanes = EntropyContextPlanes {
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
                (*xd).error_info.setjmp = false;
                crate::thread_shim::vp8_semaphore_signal((*pbi).h_event_end_decoding);
            } else {
                (*xd).error_info.setjmp = true;
                mt_decode_mb_rows(pbi, xd, ithread + 1 as i32);
                (*xd).error_info.setjmp = false;
            }
        }
        THREAD_EXIT_SUCCESS
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decoder_create_threads(mut pbi: *mut Vp8dComp) {
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

            let thread_count = (*pbi).decoding_thread_count as usize;

            let mut h_decoding_thread_vec = Vec::<PthreadT>::with_capacity(thread_count);
            h_decoding_thread_vec.resize(thread_count, core::mem::zeroed());
            (*pbi).h_decoding_thread = h_decoding_thread_vec.as_mut_ptr();
            core::mem::forget(h_decoding_thread_vec);

            if (*pbi).h_decoding_thread.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->h_decoding_thread)\0" as *const u8 as *const i8,
                );
            }

            let mut h_event_start_decoding_vec = Vec::<SemaphoreT>::with_capacity(thread_count);
            h_event_start_decoding_vec.resize(thread_count, core::mem::zeroed());
            (*pbi).h_event_start_decoding = h_event_start_decoding_vec.as_mut_ptr();
            core::mem::forget(h_event_start_decoding_vec);

            if (*pbi).h_event_start_decoding.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->h_event_start_decoding)\0" as *const u8 as *const i8,
                );
            }

            let mut mb_row_di_vec = Vec::<MbRowDec>::with_capacity(thread_count);
            mb_row_di_vec.resize(thread_count, core::mem::zeroed());
            (*pbi).mb_row_di = mb_row_di_vec.as_mut_ptr();
            core::mem::forget(mb_row_di_vec);

            if (*pbi).mb_row_di.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mb_row_di)\0" as *const u8 as *const i8,
                );
            }

            let mut de_thread_data_vec = Vec::<DecodethreadData>::with_capacity(thread_count);
            de_thread_data_vec.resize(thread_count, core::mem::zeroed());
            (*pbi).de_thread_data = de_thread_data_vec.as_mut_ptr();
            core::mem::forget(de_thread_data_vec);

            if (*pbi).de_thread_data.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->de_thread_data)\0" as *const u8 as *const i8,
                );
            }
            if crate::thread_shim::vp8_semaphore_create(
                0 as u32,
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
                    0 as u32,
                    (*pbi).h_event_start_decoding.offset(ithread as isize) as *mut SemaphoreT,
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
                *fresh7 = (*pbi).mb_row_di.offset(ithread as isize) as *mut MbRowDec as *mut c_void;
                if crate::thread_shim::vp8_pthread_create(
                    (*pbi).h_decoding_thread.offset(ithread as isize) as *mut PthreadT,
                    ::core::ptr::null::<c_void>(),
                    Some(thread_decoding_proc as unsafe fn(*mut c_void) -> *mut c_void),
                    (*pbi).de_thread_data.offset(ithread as isize) as *mut DecodethreadData
                        as *mut c_void,
                ) != 0
                {
                    crate::thread_shim::vp8_semaphore_destroy(
                        0 as u32,
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
                        0 as u32,
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
pub unsafe fn vp8mt_de_alloc_temp_buffers(mut pbi: *mut Vp8dComp, mut mb_rows: i32) {
    unsafe {
        let mut i: i32 = 0;
        if !(*pbi).mt_current_mb_col.is_null() {
            let _ = Vec::from_raw_parts((*pbi).mt_current_mb_col, 0, mb_rows as usize);
            (*pbi).mt_current_mb_col = ::core::ptr::null_mut::<VpxAtomicInt>();
        }
        if !(*pbi).mt_yabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_yabove_row.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_yabove_row.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_yabove_row, 0, mb_rows as usize);
            (*pbi).mt_yabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_uabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_uabove_row.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_uabove_row.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_uabove_row, 0, mb_rows as usize);
            (*pbi).mt_uabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_vabove_row.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_vabove_row.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_vabove_row.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_vabove_row, 0, mb_rows as usize);
            (*pbi).mt_vabove_row = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_yleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_yleft_col.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_yleft_col.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_yleft_col, 0, mb_rows as usize);
            (*pbi).mt_yleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_uleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_uleft_col.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_uleft_col.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_uleft_col, 0, mb_rows as usize);
            (*pbi).mt_uleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
        if !(*pbi).mt_vleft_col.is_null() {
            i = 0 as i32;
            while i < mb_rows {
                let ptr = *(*pbi).mt_vleft_col.offset(i as isize);
                if !ptr.is_null() {
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }
                let fresh = &mut *(*pbi).mt_vleft_col.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            let _ = Vec::from_raw_parts((*pbi).mt_vleft_col, 0, mb_rows as usize);
            (*pbi).mt_vleft_col = ::core::ptr::null_mut::<*mut u8>();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8mt_alloc_temp_buffers(
    mut pbi: *mut Vp8dComp,
    mut width: i32,
    mut prev_mb_rows: i32,
) {
    unsafe {
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
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
                (::core::mem::size_of::<VpxAtomicInt>() as usize)
                    .wrapping_mul((*pc).mb_rows as usize),
            ) as *mut VpxAtomicInt;
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
                    (*pbi).mt_current_mb_col.offset(i as isize) as *mut VpxAtomicInt,
                    0 as i32,
                );
                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut yabove_vec = Vec::<*mut u8>::with_capacity(row_count);
            yabove_vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_yabove_row = yabove_vec.as_mut_ptr();
            core::mem::forget(yabove_vec);

            if (*pbi).mt_yabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_yabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let y_alloc_size = ((width + ((32 as i32) << 1 as i32)) as usize)
                .wrapping_mul(::core::mem::size_of::<u8>() as usize);
            while i < (*pc).mb_rows {
                let mut row_vec = Vec::<u8>::with_capacity(y_alloc_size + 15);
                let base_ptr = row_vec.as_mut_ptr();
                core::mem::forget(row_vec);
                let aligned_ptr = ((base_ptr as usize + 15) & !15) as *mut u8;

                core::ptr::write_bytes(aligned_ptr, 0 as u8, y_alloc_size);
                *(*pbi).mt_yabove_row.offset(i as isize) = aligned_ptr;

                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut uabove_vec = Vec::<*mut u8>::with_capacity(row_count);
            uabove_vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_uabove_row = uabove_vec.as_mut_ptr();
            core::mem::forget(uabove_vec);

            if (*pbi).mt_uabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_uabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let uv_alloc_size = ((uv_width + 32 as i32) as usize)
                .wrapping_mul(::core::mem::size_of::<u8>() as usize);
            while i < (*pc).mb_rows {
                let mut row_vec = Vec::<u8>::with_capacity(uv_alloc_size + 15);
                let base_ptr = row_vec.as_mut_ptr();
                core::mem::forget(row_vec);
                let aligned_ptr = ((base_ptr as usize + 15) & !15) as *mut u8;

                core::ptr::write_bytes(aligned_ptr, 0 as u8, uv_alloc_size);
                *(*pbi).mt_uabove_row.offset(i as isize) = aligned_ptr;

                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut vabove_vec = Vec::<*mut u8>::with_capacity(row_count);
            vabove_vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_vabove_row = vabove_vec.as_mut_ptr();
            core::mem::forget(vabove_vec);

            if (*pbi).mt_vabove_row.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_vabove_row)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let uv_alloc_size = ((uv_width + 32 as i32) as usize)
                .wrapping_mul(::core::mem::size_of::<u8>() as usize);
            while i < (*pc).mb_rows {
                let mut row_vec = Vec::<u8>::with_capacity(uv_alloc_size + 15);
                let base_ptr = row_vec.as_mut_ptr();
                core::mem::forget(row_vec);
                let aligned_ptr = ((base_ptr as usize + 15) & !15) as *mut u8;

                core::ptr::write_bytes(aligned_ptr, 0 as u8, uv_alloc_size);
                *(*pbi).mt_vabove_row.offset(i as isize) = aligned_ptr;

                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut vec = Vec::<*mut u8>::with_capacity(row_count);
            vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_yleft_col = vec.as_mut_ptr();
            core::mem::forget(vec);

            if (*pbi).mt_yleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_yleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(16 as usize);
            while i < (*pc).mb_rows {
                let mut col_vec = Vec::<u8>::with_capacity(col_alloc_size);
                col_vec.resize(col_alloc_size, 0);
                *(*pbi).mt_yleft_col.offset(i as isize) = col_vec.as_mut_ptr();
                core::mem::forget(col_vec);
                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut vec = Vec::<*mut u8>::with_capacity(row_count);
            vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_uleft_col = vec.as_mut_ptr();
            core::mem::forget(vec);

            if (*pbi).mt_uleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_uleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(8 as usize);
            while i < (*pc).mb_rows {
                let mut col_vec = Vec::<u8>::with_capacity(col_alloc_size);
                col_vec.resize(col_alloc_size, 0);
                *(*pbi).mt_uleft_col.offset(i as isize) = col_vec.as_mut_ptr();
                core::mem::forget(col_vec);
                i += 1;
            }
            let row_count = (*pc).mb_rows as usize;

            let mut vec = Vec::<*mut u8>::with_capacity(row_count);
            vec.resize(row_count, core::ptr::null_mut());
            (*pbi).mt_vleft_col = vec.as_mut_ptr();
            core::mem::forget(vec);

            if (*pbi).mt_vleft_col.is_null() {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->mt_vleft_col)\0" as *const u8 as *const i8,
                );
            }
            i = 0 as i32;
            let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(8 as usize);
            while i < (*pc).mb_rows {
                let mut col_vec = Vec::<u8>::with_capacity(col_alloc_size);
                col_vec.resize(col_alloc_size, 0);
                *(*pbi).mt_vleft_col.offset(i as isize) = col_vec.as_mut_ptr();
                core::mem::forget(col_vec);
                i += 1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decoder_remove_threads(mut pbi: *mut Vp8dComp) {
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
                    *(*pbi).h_decoding_thread.offset(i as isize) as PthreadT,
                    ::core::ptr::null_mut::<*mut c_void>(),
                );
                i += 1;
            }
            i = 0 as i32;
            while i < (*pbi).allocated_decoding_thread_count {
                crate::thread_shim::vp8_semaphore_destroy(
                    0 as u32,
                    *(*pbi).h_event_start_decoding.offset(i as isize),
                );
                i += 1;
            }
            if (*pbi).allocated_decoding_thread_count != 0 {
                crate::thread_shim::vp8_semaphore_destroy(0 as u32, (*pbi).h_event_end_decoding);
            }
            let thread_count = (*pbi).decoding_thread_count as usize;
            if !(*pbi).h_decoding_thread.is_null() {
                let _ = Vec::from_raw_parts((*pbi).h_decoding_thread, 0, thread_count);
            }
            (*pbi).h_decoding_thread = ::core::ptr::null_mut::<PthreadT>();
            if !(*pbi).h_event_start_decoding.is_null() {
                let _ = Vec::from_raw_parts((*pbi).h_event_start_decoding, 0, thread_count);
            }
            (*pbi).h_event_start_decoding = ::core::ptr::null_mut::<SemaphoreT>();
            if !(*pbi).mb_row_di.is_null() {
                let _ = Vec::from_raw_parts((*pbi).mb_row_di, 0, thread_count);
            }
            (*pbi).mb_row_di = ::core::ptr::null_mut::<MbRowDec>();
            if !(*pbi).de_thread_data.is_null() {
                let _ = Vec::from_raw_parts((*pbi).de_thread_data, 0, thread_count);
            }
            (*pbi).de_thread_data = ::core::ptr::null_mut::<DecodethreadData>();
            vp8mt_de_alloc_temp_buffers(pbi, (*pbi).common.mb_rows);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8mt_decode_mb_rows(mut pbi: *mut Vp8dComp, mut xd: *mut MACROBLOCKD) -> i32 {
    unsafe {
        let mut pc: *mut Vp8Common = &raw mut (*pbi).common;
        let mut i: u32 = 0;
        let mut j: i32 = 0;
        let mut filter_level: i32 = (*pc).filter_level;
        let mut yv12_fb_new: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        if filter_level != 0 {
            core::ptr::write_bytes(
                (*(*pbi).mt_yabove_row.offset(0 as isize))
                    .offset(VP8BORDERINPIXELS as isize)
                    .offset(-(1 as isize)) as *mut c_void as *mut u8,
                127 as u8,
                ((*yv12_fb_new).y_width + 5 as i32) as usize,
            );
            core::ptr::write_bytes(
                ((*(*pbi).mt_uabove_row.offset(0 as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                    .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                (127 as i32) as u8,
                ((((*yv12_fb_new).y_width >> 1 as i32) + 5 as i32) as usize) as usize,
            );
            core::ptr::write_bytes(
                ((*(*pbi).mt_vabove_row.offset(0 as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                    .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                (127 as i32) as u8,
                ((((*yv12_fb_new).y_width >> 1 as i32) + 5 as i32) as usize) as usize,
            );
            j = 1 as i32;
            while j < (*pc).mb_rows {
                core::ptr::write_bytes(
                    ((*(*pbi).mt_yabove_row.offset(j as isize))
                        .offset(VP8BORDERINPIXELS as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as usize) as usize,
                );
                core::ptr::write_bytes(
                    ((*(*pbi).mt_uabove_row.offset(j as isize))
                        .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as usize) as usize,
                );
                core::ptr::write_bytes(
                    ((*(*pbi).mt_vabove_row.offset(j as isize))
                        .offset((VP8BORDERINPIXELS >> 1 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (1 as usize) as usize,
                );
                j += 1;
            }
            j = 0 as i32;
            while j < (*pc).mb_rows {
                core::ptr::write_bytes(
                    (*(*pbi).mt_yleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (16 as usize) as usize,
                );
                core::ptr::write_bytes(
                    (*(*pbi).mt_uleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (8 as usize) as usize,
                );
                core::ptr::write_bytes(
                    (*(*pbi).mt_vleft_col.offset(j as isize) as *mut c_void) as *mut u8,
                    (129 as i32) as u8,
                    (8 as usize) as usize,
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
            (*xd).error_info.setjmp = false;
            (*xd).corrupted = 1 as i32;
            i = 0 as u32;
            while i < (*pbi).decoding_thread_count {
                crate::thread_shim::vp8_semaphore_wait((*pbi).h_event_end_decoding);
                i = i.wrapping_add(1);
            }
            return -(1 as i32);
        }
        (*xd).error_info.setjmp = true;
        mt_decode_mb_rows(pbi, xd, 0 as i32);
        (*xd).error_info.setjmp = false;
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
