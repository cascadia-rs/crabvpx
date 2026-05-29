use crate::vpx_dsp::tables::VPX_NORM as vp8_norm;
use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_bilinear_predict16x16_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_bilinear_predict4x4_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_bilinear_predict8x4_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_bilinear_predict8x8_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
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
    fn vp8_short_inv_walsh4x4_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn vp8_short_inv_walsh4x4_1_c(input: *mut i16, mb_dqcoeff: *mut i16);
    fn vp8_sixtap_predict16x16_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_sixtap_predict4x4_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_sixtap_predict8x4_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_sixtap_predict8x8_c(
        src_ptr: *mut u8,
        src_pixels_per_line: i32,
        xoffset: i32,
        yoffset: i32,
        dst_ptr: *mut u8,
        dst_pitch: i32,
    );
    fn vp8_yv12_extend_frame_borders_c(ybf: *mut Yv12BufferConfig);
    fn vp8dx_start_decode(
        br: *mut BoolDecoder,
        source: *const u8,
        source_sz: u32,
        decrypt_cb: VpxDecryptCb,
        decrypt_state: *mut c_void,
    ) -> i32;
    fn vp8dx_bool_decoder_fill(br: *mut BoolDecoder);
    fn vpx_internal_error(info: *mut VpxInternalErrorInfo, error: u32, fmt: *const i8);
    fn vp8_loop_filter_frame_init(cm: *mut VP8Common, mbd: *mut Macroblockd, default_filt_lvl: i32);
    fn vp8_loop_filter_row_normal(
        cm: *mut VP8Common,
        mode_info_context: *mut Modeinfo,
        mb_row: i32,
        post_ystride: i32,
        post_uvstride: i32,
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
    );
    fn vp8_loop_filter_row_simple(
        cm: *mut VP8Common,
        mode_info_context: *mut Modeinfo,
        mb_row: i32,
        post_ystride: i32,
        y_ptr: *mut u8,
    );
    static vp8_default_mv_context: [MvContext; 2];
    static vp8_coef_update_probs: [[[[u8; 11]; 3]; 8]; 4];
    fn vp8_default_coef_probs(_: *mut VP8Common);
    static vp8_mb_feature_data_bits: [i32; 2];
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
    fn vp8_reset_mb_tokens_context(x: *mut MACROBLOCKD);
    fn vp8_decode_mb_tokens(_: *mut Vp8dComp, _: *mut MACROBLOCKD) -> i32;
    fn vp8_setup_version(cm: *mut Vp8Common);
    fn vp8_init_mbmode_probs(x: *mut Vp8Common);
    fn vp8_ac_yquant(qindex: i32) -> i32;
    fn vp8_dc_quant(qindex: i32, delta: i32) -> i32;
    fn vp8_dc2quant(qindex: i32, delta: i32) -> i32;
    fn vp8_ac2quant(qindex: i32, delta: i32) -> i32;
    fn vp8_dc_uv_quant(qindex: i32, delta: i32) -> i32;
    fn vp8_ac_uv_quant(qindex: i32, delta: i32) -> i32;
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
    fn vp8_decode_mode_mvs(_: *mut Vp8dComp);
    fn vp8_extend_mb_row(ybf: *mut Yv12BufferConfig, yptr: *mut u8, uptr: *mut u8, vptr: *mut u8);
    fn vp8mt_decode_mb_rows(pbi: *mut Vp8dComp, xd: *mut MACROBLOCKD) -> i32;
    fn vp8_decoder_remove_threads(pbi: *mut Vp8dComp);
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
pub struct OpaquePthreadT {
    pub __sig: i64,
    pub __cleanup_stack: *mut DarwinPthreadHandlerRec,
    pub __opaque: [i8; 8176],
}
pub type DarwinPthreadT = *mut OpaquePthreadT;
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
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MB_LVL_MAX: C2RustUnnamed = 2;
pub const MB_LVL_ALT_LF: C2RustUnnamed = 1;
pub const MB_LVL_ALT_Q: C2RustUnnamed = 0;
pub const MAX_REF_FRAMES: u32 = 4;
pub const ALTREF_FRAME: u32 = 3;
pub const GOLDEN_FRAME: u32 = 2;
pub const LAST_FRAME: u32 = 1;
pub const INTRA_FRAME: u32 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const CHAR_BIT: i32 = 8 as i32;
pub const vp8_prob_half: u8 = 128 as u8;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<usize>() as i32 * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: i32 = 0x40000000 as i32;
unsafe fn vp8dx_decode_bool(mut br: *mut BoolDecoder, mut probability: i32) -> i32 {
    unsafe {
        let mut bit: u32 = 0 as u32;
        let mut value: usize = 0;
        let mut split: u32 = 0;
        let mut bigsplit: usize = 0;
        let mut count: i32 = 0;
        let mut range: u32 = 0;
        split = (1 as u32).wrapping_add(
            (*br)
                .range
                .wrapping_sub(1 as u32)
                .wrapping_mul(probability as u32)
                >> 8 as i32,
        );
        if (*br).count < 0 as i32 {
            vp8dx_bool_decoder_fill(br);
        }
        value = (*br).value;
        count = (*br).count;
        bigsplit = (split as usize) << (VP8_BD_VALUE_SIZE - 8 as i32);
        range = split;
        if value >= bigsplit {
            range = (*br).range.wrapping_sub(split);
            value = value.wrapping_sub(bigsplit);
            bit = 1 as u32;
        }
        let shift: u8 = vp8_norm[range as usize];
        range <<= shift as i32;
        value <<= shift as i32;
        count -= shift as i32;
        (*br).value = value;
        (*br).count = count;
        (*br).range = range;
        bit as i32
    }
}
#[inline]
unsafe fn vp8_decode_value(mut br: *mut BoolDecoder, mut bits: i32) -> i32 {
    unsafe {
        let mut z: i32 = 0 as i32;
        let mut bit: i32 = 0;
        bit = bits - 1 as i32;
        while bit >= 0 as i32 {
            z |= vp8dx_decode_bool(br, 0x80 as i32) << bit;
            bit -= 1;
        }
        z
    }
}
#[inline]
unsafe fn vp8dx_bool_error(mut br: *mut BoolDecoder) -> i32 {
    unsafe {
        if (*br).count > VP8_BD_VALUE_SIZE && (*br).count < VP8_LOTS_OF_BITS {
            return 1 as i32;
        }
        0 as i32
    }
}
pub const MB_FEATURE_TREE_PROBS: i32 = 3 as i32;
pub const MAX_MB_SEGMENTS: i32 = 4 as i32;
pub const MAX_REF_LF_DELTAS: i32 = 4 as i32;
pub const MAX_MODE_LF_DELTAS: i32 = 4 as i32;
pub const SEGMENT_DELTADATA: i32 = 0 as i32;
pub const SEGMENT_ABSDATA: i32 = 1 as i32;
pub const ENTROPY_NODES: i32 = 11 as i32;
pub const BLOCK_TYPES: i32 = 4 as i32;
pub const COEF_BANDS: i32 = 8 as i32;
pub const PREV_COEF_CONTEXTS: i32 = 3 as i32;
pub const MAXQ: i32 = 127 as i32;
pub const QINDEX_RANGE: i32 = MAXQ + 1 as i32;
#[inline]
unsafe fn vpx_atomic_load_acquire(mut atomic: *const VpxAtomicInt) -> i32 {
    unsafe {
        (*((&raw const (*atomic).value) as *const core::sync::atomic::AtomicI32))
            .load(core::sync::atomic::Ordering::Acquire)
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
    let y_stride = y_stride as usize;
    let uv_stride = uv_stride as usize;

    // Convert raw pointers into safe mutable slices
    let y_slice = unsafe { core::slice::from_raw_parts_mut(y_buffer, y_stride * 16) };
    let u_slice = unsafe { core::slice::from_raw_parts_mut(u_buffer, uv_stride * 8) };
    let v_slice = unsafe { core::slice::from_raw_parts_mut(v_buffer, uv_stride * 8) };

    // Use safe iterators to set the values
    for chunk in y_slice.chunks_mut(y_stride).take(16) {
        chunk[0] = 129;
    }
    for chunk in u_slice.chunks_mut(uv_stride).take(8) {
        chunk[0] = 129;
    }
    for chunk in v_slice.chunks_mut(uv_stride).take(8) {
        chunk[0] = 129;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8cx_init_de_quantizer(mut pbi: *mut Vp8dComp) {
    unsafe {
        let mut Q: i32 = 0;
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
        Q = 0 as i32;
        while Q < QINDEX_RANGE {
            (*pc).y1dequant[Q as usize][0 as usize] = vp8_dc_quant(Q, (*pc).y1dc_delta_q) as i16;
            (*pc).y2dequant[Q as usize][0 as usize] = vp8_dc2quant(Q, (*pc).y2dc_delta_q) as i16;
            (*pc).uvdequant[Q as usize][0 as usize] = vp8_dc_uv_quant(Q, (*pc).uvdc_delta_q) as i16;
            (*pc).y1dequant[Q as usize][1 as usize] = vp8_ac_yquant(Q) as i16;
            (*pc).y2dequant[Q as usize][1 as usize] = vp8_ac2quant(Q, (*pc).y2ac_delta_q) as i16;
            (*pc).uvdequant[Q as usize][1 as usize] = vp8_ac_uv_quant(Q, (*pc).uvac_delta_q) as i16;
            Q += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_mb_init_dequantizer(mut pbi: *mut Vp8dComp, mut xd: *mut MACROBLOCKD) {
    unsafe {
        let mut i: i32 = 0;
        let mut qindex: i32 = 0;
        let mut mbmi: *mut MbModeInfo = &raw mut (*(*xd).mode_info_context).mbmi;
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
        if (*xd).segmentation_enabled != 0 {
            if (*xd).mb_segment_abs_delta as i32 == SEGMENT_ABSDATA {
                qindex = (*xd).segment_feature_data[MB_LVL_ALT_Q as usize]
                    [(*mbmi).segment_id as usize] as i32;
            } else {
                qindex = (*pc).base_qindex
                    + (*xd).segment_feature_data[MB_LVL_ALT_Q as usize][(*mbmi).segment_id as usize]
                        as i32;
            }
            qindex = if qindex >= 0 as i32 {
                if qindex <= MAXQ { qindex } else { MAXQ }
            } else {
                0 as i32
            };
        } else {
            qindex = (*pc).base_qindex;
        }
        (*xd).dequant_y1_dc[0 as usize] = 1 as i16;
        (*xd).dequant_y1[0 as usize] = (*pc).y1dequant[qindex as usize][0 as usize];
        (*xd).dequant_y2[0 as usize] = (*pc).y2dequant[qindex as usize][0 as usize];
        (*xd).dequant_uv[0 as usize] = (*pc).uvdequant[qindex as usize][0 as usize];
        i = 1 as i32;
        while i < 16 as i32 {
            (*xd).dequant_y1[i as usize] = (*pc).y1dequant[qindex as usize][1 as usize];
            (*xd).dequant_y1_dc[i as usize] = (*xd).dequant_y1[i as usize];
            (*xd).dequant_y2[i as usize] = (*pc).y2dequant[qindex as usize][1 as usize];
            (*xd).dequant_uv[i as usize] = (*pc).uvdequant[qindex as usize][1 as usize];
            i += 1;
        }
    }
}
unsafe fn decode_macroblock(mut pbi: *mut Vp8dComp, mut xd: *mut MACROBLOCKD, _mb_idx: u32) {
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
                    let mut Above: *mut u8 = dst.offset(-(dst_stride as isize));
                    let mut yleft: *mut u8 = dst.offset(-(1 as isize));
                    let mut left_stride: i32 = dst_stride;
                    let mut top_left: u8 = *Above.offset(-(1 as i32) as isize);
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
unsafe fn get_delta_q(mut bc: *mut Vp8Reader, mut prev: i32, mut q_update: *mut i32) -> i32 {
    unsafe {
        let mut ret_val: i32 = 0 as i32;
        if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
            ret_val = vp8_decode_value(bc as *mut BoolDecoder, 4 as i32);
            if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                ret_val = -ret_val;
            }
        }
        if ret_val != prev {
            *q_update = 1 as i32;
        }
        ret_val
    }
}
unsafe fn yv12_extend_frame_top_c(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        let mut i: i32 = 0;
        let mut src_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut border: u32 = 0;
        let mut plane_stride: i32 = 0;
        border = (*ybf).border as u32;
        plane_stride = (*ybf).y_stride;
        src_ptr1 = (*ybf).y_buffer.offset(-(border as isize)) as *mut u8;
        dest_ptr1 = src_ptr1.offset(-(border.wrapping_mul(plane_stride as u32) as isize));
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr1 as *const c_void as *const u8,
                dest_ptr1 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            i += 1;
        }
        plane_stride = (*ybf).uv_stride;
        border = border.wrapping_div(2 as u32);
        src_ptr1 = (*ybf).u_buffer.offset(-(border as isize)) as *mut u8;
        dest_ptr1 = src_ptr1.offset(-(border.wrapping_mul(plane_stride as u32) as isize));
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr1 as *const c_void as *const u8,
                dest_ptr1 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            i += 1;
        }
        src_ptr1 = (*ybf).v_buffer.offset(-(border as isize)) as *mut u8;
        dest_ptr1 = src_ptr1.offset(-(border.wrapping_mul(plane_stride as u32) as isize));
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr1 as *const c_void as *const u8,
                dest_ptr1 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            i += 1;
        }
    }
}
unsafe fn yv12_extend_frame_bottom_c(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        let mut i: i32 = 0;
        let mut src_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut src_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut border: u32 = 0;
        let mut plane_stride: i32 = 0;
        let mut plane_height: i32 = 0;
        border = (*ybf).border as u32;
        plane_stride = (*ybf).y_stride;
        plane_height = (*ybf).y_height;
        src_ptr1 = (*ybf).y_buffer.offset(-(border as isize)) as *mut u8;
        src_ptr2 = src_ptr1
            .offset((plane_height * plane_stride) as isize)
            .offset(-(plane_stride as isize));
        dest_ptr2 = src_ptr2.offset(plane_stride as isize);
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dest_ptr2 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
        plane_stride = (*ybf).uv_stride;
        plane_height = (*ybf).uv_height;
        border = border.wrapping_div(2 as u32);
        src_ptr1 = (*ybf).u_buffer.offset(-(border as isize)) as *mut u8;
        src_ptr2 = src_ptr1
            .offset((plane_height * plane_stride) as isize)
            .offset(-(plane_stride as isize));
        dest_ptr2 = src_ptr2.offset(plane_stride as isize);
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dest_ptr2 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
        src_ptr1 = (*ybf).v_buffer.offset(-(border as isize)) as *mut u8;
        src_ptr2 = src_ptr1
            .offset((plane_height * plane_stride) as isize)
            .offset(-(plane_stride as isize));
        dest_ptr2 = src_ptr2.offset(plane_stride as isize);
        i = 0 as i32;
        while i < border as i32 {
            core::ptr::copy_nonoverlapping(
                src_ptr2 as *const c_void as *const u8,
                dest_ptr2 as *mut c_void as *mut u8,
                plane_stride as usize,
            );
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
    }
}
unsafe fn yv12_extend_frame_left_right_c(
    mut ybf: *mut Yv12BufferConfig,
    mut y_src: *mut u8,
    mut u_src: *mut u8,
    mut v_src: *mut u8,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut src_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut src_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr1: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut dest_ptr2: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut border: u32 = 0;
        let mut plane_stride: i32 = 0;
        let mut plane_height: i32 = 0;
        let mut plane_width: i32 = 0;
        border = (*ybf).border as u32;
        plane_stride = (*ybf).y_stride;
        plane_height = 16 as i32;
        plane_width = (*ybf).y_width;
        src_ptr1 = y_src;
        src_ptr2 = src_ptr1.offset(plane_width as isize).offset(-(1 as isize));
        dest_ptr1 = src_ptr1.offset(-(border as isize));
        dest_ptr2 = src_ptr2.offset(1 as isize);
        i = 0 as i32;
        while i < plane_height {
            core::ptr::write_bytes(
                dest_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as u8,
                border as usize,
            );
            core::ptr::write_bytes(
                dest_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as u8,
                border as usize,
            );
            src_ptr1 = src_ptr1.offset(plane_stride as isize);
            src_ptr2 = src_ptr2.offset(plane_stride as isize);
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
        plane_stride = (*ybf).uv_stride;
        plane_height = 8 as i32;
        plane_width = (*ybf).uv_width;
        border = border.wrapping_div(2 as u32);
        src_ptr1 = u_src;
        src_ptr2 = src_ptr1.offset(plane_width as isize).offset(-(1 as isize));
        dest_ptr1 = src_ptr1.offset(-(border as isize));
        dest_ptr2 = src_ptr2.offset(1 as isize);
        i = 0 as i32;
        while i < plane_height {
            core::ptr::write_bytes(
                dest_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as u8,
                border as usize,
            );
            core::ptr::write_bytes(
                dest_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as u8,
                border as usize,
            );
            src_ptr1 = src_ptr1.offset(plane_stride as isize);
            src_ptr2 = src_ptr2.offset(plane_stride as isize);
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
        src_ptr1 = v_src;
        src_ptr2 = src_ptr1.offset(plane_width as isize).offset(-(1 as isize));
        dest_ptr1 = src_ptr1.offset(-(border as isize));
        dest_ptr2 = src_ptr2.offset(1 as isize);
        i = 0 as i32;
        while i < plane_height {
            core::ptr::write_bytes(
                dest_ptr1 as *mut c_void as *mut u8,
                *src_ptr1.offset(0 as isize) as u8,
                border as usize,
            );
            core::ptr::write_bytes(
                dest_ptr2 as *mut c_void as *mut u8,
                *src_ptr2.offset(0 as isize) as u8,
                border as usize,
            );
            src_ptr1 = src_ptr1.offset(plane_stride as isize);
            src_ptr2 = src_ptr2.offset(plane_stride as isize);
            dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
            dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
            i += 1;
        }
    }
}
unsafe fn decode_mb_rows(mut pbi: *mut Vp8dComp) {
    unsafe {
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
        let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
        let mut lf_mic: *mut ModeInfo = (*xd).mode_info_context;
        let mut ibc: i32 = 0 as i32;
        let mut num_part: i32 = (1 as i32) << (*pc).multi_token_partition as u32;
        let mut recon_yoffset: i32 = 0;
        let mut recon_uvoffset: i32 = 0;
        let mut mb_row: i32 = 0;
        let mut mb_col: i32 = 0;
        let mut mb_idx: i32 = 0 as i32;
        let mut yv12_fb_new: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        let mut recon_y_stride: i32 = (*yv12_fb_new).y_stride;
        let mut recon_uv_stride: i32 = (*yv12_fb_new).uv_stride;
        let mut ref_buffer: [[*mut u8; 3]; 4] = [[::core::ptr::null_mut::<u8>(); 3]; 4];
        let mut dst_buffer: [*mut u8; 3] = [::core::ptr::null_mut::<u8>(); 3];
        let mut lf_dst: [*mut u8; 3] = [::core::ptr::null_mut::<u8>(); 3];
        let mut eb_dst: [*mut u8; 3] = [::core::ptr::null_mut::<u8>(); 3];
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
        lf_dst[0 as usize] = dst_buffer[0 as usize];
        eb_dst[0 as usize] = lf_dst[0 as usize];
        dst_buffer[1 as usize] = (*yv12_fb_new).u_buffer as *mut u8;
        lf_dst[1 as usize] = dst_buffer[1 as usize];
        eb_dst[1 as usize] = lf_dst[1 as usize];
        dst_buffer[2 as usize] = (*yv12_fb_new).v_buffer as *mut u8;
        lf_dst[2 as usize] = dst_buffer[2 as usize];
        eb_dst[2 as usize] = lf_dst[2 as usize];
        (*xd).up_available = false;
        if (*pc).filter_level != 0 {
            vp8_loop_filter_frame_init(
                pc as *mut VP8Common,
                xd as *mut Macroblockd,
                (*pc).filter_level,
            );
        }
        vp8_setup_intra_recon_top_line(yv12_fb_new);
        mb_row = 0 as i32;
        while mb_row < (*pc).mb_rows {
            if num_part > 1 as i32 {
                (*xd).current_bc = (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(ibc as isize)
                    as *mut Vp8Reader as *mut c_void;
                ibc += 1;
                if ibc == num_part {
                    ibc = 0 as i32;
                }
            }
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
            (*xd).recon_above[0 as usize] = dst_buffer[0 as usize].offset(recon_yoffset as isize);
            (*xd).recon_above[1 as usize] = dst_buffer[1 as usize].offset(recon_uvoffset as isize);
            (*xd).recon_above[2 as usize] = dst_buffer[2 as usize].offset(recon_uvoffset as isize);
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
            mb_col = 0 as i32;
            while mb_col < (*pc).mb_cols {
                (*xd).mb_to_left_edge = -((mb_col * 16 as i32) << 3 as i32);
                (*xd).mb_to_right_edge =
                    (((*pc).mb_cols - 1 as i32 - mb_col) * 16 as i32) << 3 as i32;
                (*xd).dst.y_buffer =
                    dst_buffer[0 as usize].offset(recon_yoffset as isize) as *mut u8;
                (*xd).dst.u_buffer =
                    dst_buffer[1 as usize].offset(recon_uvoffset as isize) as *mut u8;
                (*xd).dst.v_buffer =
                    dst_buffer[2 as usize].offset(recon_uvoffset as isize) as *mut u8;
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
                (*xd).corrupted |=
                    ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
                decode_macroblock(pbi, xd, mb_idx as u32);
                mb_idx += 1;
                (*xd).left_available = true;
                (*xd).corrupted |= vp8dx_bool_error((*xd).current_bc as *mut BoolDecoder);
                (*xd).recon_above[0 as usize] = (*xd).recon_above[0 as usize].offset(16 as isize);
                (*xd).recon_above[1 as usize] = (*xd).recon_above[1 as usize].offset(8 as isize);
                (*xd).recon_above[2 as usize] = (*xd).recon_above[2 as usize].offset(8 as isize);
                (*xd).recon_left[0 as usize] = (*xd).recon_left[0 as usize].offset(16 as isize);
                (*xd).recon_left[1 as usize] = (*xd).recon_left[1 as usize].offset(8 as isize);
                (*xd).recon_left[2 as usize] = (*xd).recon_left[2 as usize].offset(8 as isize);
                recon_yoffset += 16 as i32;
                recon_uvoffset += 8 as i32;
                (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
                (*xd).above_context = (*xd).above_context.offset(1);
                mb_col += 1;
            }
            vp8_extend_mb_row(
                yv12_fb_new,
                (*xd).dst.y_buffer.offset(16 as isize),
                (*xd).dst.u_buffer.offset(8 as isize),
                (*xd).dst.v_buffer.offset(8 as isize),
            );
            (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
            (*xd).up_available = true;
            if (*pc).filter_level != 0 {
                if mb_row > 0 as i32 {
                    if (*pc).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
                        vp8_loop_filter_row_normal(
                            pc as *mut VP8Common,
                            lf_mic as *mut Modeinfo,
                            mb_row - 1 as i32,
                            recon_y_stride,
                            recon_uv_stride,
                            lf_dst[0 as usize],
                            lf_dst[1 as usize],
                            lf_dst[2 as usize],
                        );
                    } else {
                        vp8_loop_filter_row_simple(
                            pc as *mut VP8Common,
                            lf_mic as *mut Modeinfo,
                            mb_row - 1 as i32,
                            recon_y_stride,
                            lf_dst[0 as usize],
                        );
                    }
                    if mb_row > 1 as i32 {
                        yv12_extend_frame_left_right_c(
                            yv12_fb_new,
                            eb_dst[0 as usize],
                            eb_dst[1 as usize],
                            eb_dst[2 as usize],
                        );
                        eb_dst[0 as usize] =
                            eb_dst[0 as usize].offset((recon_y_stride * 16 as i32) as isize);
                        eb_dst[1 as usize] =
                            eb_dst[1 as usize].offset((recon_uv_stride * 8 as i32) as isize);
                        eb_dst[2 as usize] =
                            eb_dst[2 as usize].offset((recon_uv_stride * 8 as i32) as isize);
                    }
                    lf_dst[0 as usize] =
                        lf_dst[0 as usize].offset((recon_y_stride * 16 as i32) as isize);
                    lf_dst[1 as usize] =
                        lf_dst[1 as usize].offset((recon_uv_stride * 8 as i32) as isize);
                    lf_dst[2 as usize] =
                        lf_dst[2 as usize].offset((recon_uv_stride * 8 as i32) as isize);
                    lf_mic = lf_mic.offset((*pc).mb_cols as isize);
                    lf_mic = lf_mic.offset(1);
                }
            } else if mb_row > 0 as i32 {
                yv12_extend_frame_left_right_c(
                    yv12_fb_new,
                    eb_dst[0 as usize],
                    eb_dst[1 as usize],
                    eb_dst[2 as usize],
                );
                eb_dst[0 as usize] =
                    eb_dst[0 as usize].offset((recon_y_stride * 16 as i32) as isize);
                eb_dst[1 as usize] =
                    eb_dst[1 as usize].offset((recon_uv_stride * 8 as i32) as isize);
                eb_dst[2 as usize] =
                    eb_dst[2 as usize].offset((recon_uv_stride * 8 as i32) as isize);
            }
            mb_row += 1;
        }
        if (*pc).filter_level != 0 {
            if (*pc).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
                vp8_loop_filter_row_normal(
                    pc as *mut VP8Common,
                    lf_mic as *mut Modeinfo,
                    mb_row - 1 as i32,
                    recon_y_stride,
                    recon_uv_stride,
                    lf_dst[0 as usize],
                    lf_dst[1 as usize],
                    lf_dst[2 as usize],
                );
            } else {
                vp8_loop_filter_row_simple(
                    pc as *mut VP8Common,
                    lf_mic as *mut Modeinfo,
                    mb_row - 1 as i32,
                    recon_y_stride,
                    lf_dst[0 as usize],
                );
            }
            yv12_extend_frame_left_right_c(
                yv12_fb_new,
                eb_dst[0 as usize],
                eb_dst[1 as usize],
                eb_dst[2 as usize],
            );
            eb_dst[0 as usize] = eb_dst[0 as usize].offset((recon_y_stride * 16 as i32) as isize);
            eb_dst[1 as usize] = eb_dst[1 as usize].offset((recon_uv_stride * 8 as i32) as isize);
            eb_dst[2 as usize] = eb_dst[2 as usize].offset((recon_uv_stride * 8 as i32) as isize);
        }
        yv12_extend_frame_left_right_c(
            yv12_fb_new,
            eb_dst[0 as usize],
            eb_dst[1 as usize],
            eb_dst[2 as usize],
        );
        yv12_extend_frame_top_c(yv12_fb_new);
        yv12_extend_frame_bottom_c(yv12_fb_new);
    }
}
unsafe fn read_partition_size(mut pbi: *mut Vp8dComp, mut cx_size: *const u8) -> u32 {
    unsafe {
        let mut temp: [u8; 3] = [0; 3];
        if (*pbi).decrypt_cb.is_some() {
            (*pbi).decrypt_cb.expect("non-null function pointer")(
                (*pbi).decrypt_state,
                cx_size,
                &raw mut temp as *mut u8,
                3 as i32,
            );
            cx_size = &raw mut temp as *mut u8;
        }
        (*cx_size.offset(0 as isize) as i32
            + ((*cx_size.offset(1 as isize) as i32) << 8 as i32)
            + ((*cx_size.offset(2 as isize) as i32) << 16 as i32)) as u32
    }
}
unsafe fn read_is_valid(mut start: *const u8, mut len: usize, mut end: *const u8) -> i32 {
    unsafe { (len != 0 as usize && end > start && len <= end.offset_from(start) as usize) as i32 }
}
unsafe fn read_available_partition_size(
    mut pbi: *mut Vp8dComp,
    mut token_part_sizes: *const u8,
    mut fragment_start: *const u8,
    mut first_fragment_end: *const u8,
    mut fragment_end: *const u8,
    mut i: i32,
    mut num_part: i32,
) -> u32 {
    unsafe {
        let mut pc: *mut Vp8Common = &raw mut (*pbi).common;
        let mut partition_size_ptr: *const u8 = token_part_sizes.offset((i * 3 as i32) as isize);
        let mut partition_size: u32 = 0 as u32;
        let mut bytes_left: isize = fragment_end.offset_from(fragment_start) as isize;
        if bytes_left < 0 as isize {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet or corrupt partition. No bytes left %d.\0" as *const u8
                    as *const i8,
            );
        }
        if i < num_part - 1 as i32 {
            if read_is_valid(partition_size_ptr, 3 as usize, first_fragment_end) != 0 {
                partition_size = read_partition_size(pbi, partition_size_ptr);
            } else if (*pbi).ec_active {
                partition_size = bytes_left as u32;
            } else {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Truncated partition size data\0" as *const u8 as *const i8,
                );
            }
        } else {
            partition_size = bytes_left as u32;
        }
        if read_is_valid(fragment_start, partition_size as usize, fragment_end) == 0 {
            if (*pbi).ec_active {
                partition_size = bytes_left as u32;
            } else {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Truncated packet or corrupt partition %d length\0" as *const u8 as *const i8,
                );
            }
        }
        partition_size
    }
}
unsafe fn setup_token_decoder(mut pbi: *mut Vp8dComp, mut token_part_sizes: *const u8) {
    unsafe {
        let mut bool_decoder: *mut Vp8Reader =
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(0 as isize) as *mut Vp8Reader;
        let mut partition_idx: u32 = 0;
        let mut fragment_idx: u32 = 0;
        let mut num_token_partitions: u32 = 0;
        let mut first_fragment_end: *const u8 =
            (*pbi).fragments.ptrs[0 as usize].offset((*pbi).fragments.sizes[0 as usize] as isize);
        let mut multi_token_partition: u32 = vp8_decode_value(
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut BoolDecoder,
            2 as i32,
        ) as u32;
        if vp8dx_bool_error(
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut BoolDecoder
        ) == 0
        {
            (*pbi).common.multi_token_partition = multi_token_partition;
        }
        num_token_partitions = ((1 as i32) << (*pbi).common.multi_token_partition as u32) as u32;
        fragment_idx = 0 as u32;
        while fragment_idx < (*pbi).fragments.count {
            let mut fragment_size: u32 = (*pbi).fragments.sizes[fragment_idx as usize];
            let mut fragment_end: *const u8 =
                (*pbi).fragments.ptrs[fragment_idx as usize].offset(fragment_size as isize);
            if fragment_idx == 0 as u32 {
                let mut ext_first_part_size: isize = token_part_sizes
                    .offset_from((*pbi).fragments.ptrs[0 as usize])
                    as isize
                    + (3 as u32).wrapping_mul(num_token_partitions.wrapping_sub(1 as u32)) as isize;
                if fragment_size < ext_first_part_size as u32 {
                    vpx_internal_error(
                        &raw mut (*pbi).common.error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Corrupted fragment size %d\0" as *const u8 as *const i8,
                    );
                }
                fragment_size = fragment_size.wrapping_sub(ext_first_part_size as u32);
                if fragment_size > 0 as u32 {
                    (*pbi).fragments.sizes[0 as usize] = ext_first_part_size as u32;
                    fragment_idx = fragment_idx.wrapping_add(1);
                    (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                        [0 as usize]
                        .offset((*pbi).fragments.sizes[0 as usize] as isize);
                }
            }
            while fragment_size > 0 as u32 {
                let mut partition_size: isize = read_available_partition_size(
                    pbi,
                    token_part_sizes,
                    (*pbi).fragments.ptrs[fragment_idx as usize],
                    first_fragment_end,
                    fragment_end,
                    fragment_idx.wrapping_sub(1 as u32) as i32,
                    num_token_partitions as i32,
                ) as isize;
                (*pbi).fragments.sizes[fragment_idx as usize] = partition_size as u32;
                if fragment_size < partition_size as u32 {
                    vpx_internal_error(
                        &raw mut (*pbi).common.error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Corrupted fragment size %d\0" as *const u8 as *const i8,
                    );
                }
                fragment_size = fragment_size.wrapping_sub(partition_size as u32);
                if fragment_size > 0 as u32 {
                    fragment_idx = fragment_idx.wrapping_add(1);
                    (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                        [fragment_idx.wrapping_sub(1 as u32) as usize]
                        .offset(partition_size as isize);
                }
            }
            fragment_idx = fragment_idx.wrapping_add(1);
        }
        (*pbi).fragments.count = num_token_partitions.wrapping_add(1 as u32);
        partition_idx = 1 as u32;
        while partition_idx < (*pbi).fragments.count {
            if vp8dx_start_decode(
                bool_decoder as *mut BoolDecoder,
                (*pbi).fragments.ptrs[partition_idx as usize],
                (*pbi).fragments.sizes[partition_idx as usize],
                (*pbi).decrypt_cb,
                (*pbi).decrypt_state,
            ) != 0
            {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate bool decoder %d\0" as *const u8 as *const i8,
                );
            }
            bool_decoder = bool_decoder.offset(1);
            partition_idx = partition_idx.wrapping_add(1);
        }
        if (*pbi).decoding_thread_count > num_token_partitions.wrapping_sub(1 as u32) {
            (*pbi).decoding_thread_count = num_token_partitions.wrapping_sub(1 as u32);
        }
        if (*pbi).decoding_thread_count as i32 > (*pbi).common.mb_rows - 1 as i32 {
            (*pbi).decoding_thread_count = ((*pbi).common.mb_rows - 1 as i32) as u32;
        }
    }
}
unsafe fn init_frame(mut pbi: *mut Vp8dComp) {
    unsafe {
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
        let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
        if (*pc).frame_type as u32 == KEY_FRAME as u32 {
            core::ptr::copy_nonoverlapping(
                &raw const vp8_default_mv_context as *const MvContext as *const c_void as *const u8,
                &raw mut (*pc).fc.mvc as *mut MvContext as *mut c_void as *mut u8,
                ::core::mem::size_of::<[MvContext; 2]>() as usize,
            );
            vp8_init_mbmode_probs(pc);
            vp8_default_coef_probs(pc as *mut VP8Common);
            core::ptr::write_bytes(
                &raw mut (*xd).segment_feature_data as *mut [i8; 4] as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[[i8; 4]; 2]>() as usize,
            );
            (*xd).mb_segment_abs_delta = SEGMENT_DELTADATA as u8;
            core::ptr::write_bytes(
                &raw mut (*xd).ref_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[i8; 4]>() as usize,
            );
            core::ptr::write_bytes(
                &raw mut (*xd).mode_lf_deltas as *mut i8 as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<[i8; 4]>() as usize,
            );
            (*pc).refresh_golden_frame = 1 as i32;
            (*pc).refresh_alt_ref_frame = 1 as i32;
            (*pc).copy_buffer_to_gf = 0 as i32;
            (*pc).copy_buffer_to_arf = 0 as i32;
            (*pc).ref_frame_sign_bias[GOLDEN_FRAME as usize] = 0 as i32;
            (*pc).ref_frame_sign_bias[ALTREF_FRAME as usize] = 0 as i32;
        } else {
            if !(*pc).use_bilinear_mc_filter {
                (*xd).subpixel_predict = Some(
                    vp8_sixtap_predict4x4_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict8x4 = Some(
                    vp8_sixtap_predict8x4_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict8x8 = Some(
                    vp8_sixtap_predict8x8_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict16x16 = Some(
                    vp8_sixtap_predict16x16_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
            } else {
                (*xd).subpixel_predict = Some(
                    vp8_bilinear_predict4x4_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict8x4 = Some(
                    vp8_bilinear_predict8x4_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict8x8 = Some(
                    vp8_bilinear_predict8x8_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
                (*xd).subpixel_predict16x16 = Some(
                    vp8_bilinear_predict16x16_c
                        as unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> (),
                ) as Vp8SubpixFnT;
            }
            if (*pbi).decoded_key_frame && (*pbi).ec_enabled && !(*pbi).ec_active {
                (*pbi).ec_active = true;
            }
        }
        (*xd).left_context = &raw mut (*pc).left_context;
        (*xd).mode_info_context = (*pc).mi;
        (*xd).frame_type = (*pc).frame_type;
        (*(*xd).mode_info_context).mbmi.mode = DC_PRED as u8;
        (*xd).mode_info_stride = (*pc).mode_info_stride;
        (*xd).corrupted = 0 as i32;
        (*xd).fullpixel_mask = !(0 as i32);
        if (*pc).full_pixel {
            (*xd).fullpixel_mask = !(7 as i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decode_frame(mut pbi: *mut Vp8dComp) -> i32 {
    unsafe {
        let bc: *mut Vp8Reader =
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut Vp8Reader;
        let pc: *mut Vp8Common = &raw mut (*pbi).common;
        let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
        let mut data: *const u8 = (*pbi).fragments.ptrs[0 as usize];
        let data_sz: u32 = (*pbi).fragments.sizes[0 as usize];
        let mut data_end: *const u8 = data.offset(data_sz as isize);
        let mut first_partition_length_in_bytes: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mb_feature_data_bits: *const i32 = &raw const vp8_mb_feature_data_bits as *const i32;
        let mut corrupt_tokens: i32 = 0 as i32;
        let mut prev_independent_partitions: bool = (*pbi).independent_partitions;
        let mut yv12_fb_new: *mut Yv12BufferConfig = (*pbi).dec_fb_ref[INTRA_FRAME as usize];
        (*xd).corrupted = 0 as i32;
        (*yv12_fb_new).corrupted = 0 as i32;
        if (data_end.offset_from(data) as i64) < 3 as i64 {
            if !(*pbi).ec_active {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Truncated packet\0" as *const u8 as *const i8,
                );
            }
            (*pc).frame_type = INTER_FRAME;
            (*pc).version = 0 as i32;
            (*pc).show_frame = 1 as i32;
            first_partition_length_in_bytes = 0 as i32;
        } else {
            let mut clear_buffer: [u8; 10] = [0; 10];
            let mut clear: *const u8 = data;
            if (*pbi).decrypt_cb.is_some() {
                let mut n: i32 =
                    (if (::core::mem::size_of::<[u8; 10]>() as usize) < data_sz as usize {
                        ::core::mem::size_of::<[u8; 10]>() as usize
                    } else {
                        data_sz as usize
                    }) as i32;
                (*pbi).decrypt_cb.expect("non-null function pointer")(
                    (*pbi).decrypt_state,
                    data,
                    &raw mut clear_buffer as *mut u8,
                    n,
                );
                clear = &raw mut clear_buffer as *mut u8;
            }
            (*pc).frame_type = (*clear.offset(0 as isize) as i32 & 1 as i32) as u32;
            (*pc).version = *clear.offset(0 as isize) as i32 >> 1 as i32 & 7 as i32;
            (*pc).show_frame = *clear.offset(0 as isize) as i32 >> 4 as i32 & 1 as i32;
            first_partition_length_in_bytes = (*clear.offset(0 as isize) as i32
                | (*clear.offset(1 as isize) as i32) << 8 as i32
                | (*clear.offset(2 as isize) as i32) << 16 as i32)
                >> 5 as i32;
            if !(*pbi).ec_active && first_partition_length_in_bytes == 0 as i32 {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupt partition 0 length\0" as *const u8 as *const i8,
                );
            }
            data = data.offset(3 as isize);
            clear = clear.offset(3 as isize);
            vp8_setup_version(pc);
            if (*pc).frame_type as u32 == KEY_FRAME as u32 {
                if data_end.offset_from(data) as i64 >= 7 as i64 {
                    if *clear.offset(0 as isize) as i32 != 0x9d as i32
                        || *clear.offset(1 as isize) as i32 != 0x1 as i32
                        || *clear.offset(2 as isize) as i32 != 0x2a as i32
                    {
                        vpx_internal_error(
                            &raw mut (*pc).error,
                            VPX_CODEC_UNSUP_BITSTREAM,
                            b"Invalid frame sync code\0" as *const u8 as *const i8,
                        );
                    }
                    (*pc).width = (*clear.offset(3 as isize) as i32
                        | (*clear.offset(4 as isize) as i32) << 8 as i32)
                        & 0x3fff as i32;
                    (*pc).horiz_scale = *clear.offset(4 as isize) as i32 >> 6 as i32;
                    (*pc).height = (*clear.offset(5 as isize) as i32
                        | (*clear.offset(6 as isize) as i32) << 8 as i32)
                        & 0x3fff as i32;
                    (*pc).vert_scale = *clear.offset(6 as isize) as i32 >> 6 as i32;
                    data = data.offset(7 as isize);
                } else if !(*pbi).ec_active {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_CORRUPT_FRAME,
                        b"Truncated key frame header\0" as *const u8 as *const i8,
                    );
                } else {
                    data = data_end;
                }
            } else {
                (*xd).pre = *yv12_fb_new;
                (*xd).dst = *yv12_fb_new;
            }
        }
        if !(*pbi).decoded_key_frame && (*pc).frame_type as u32 != KEY_FRAME as u32 {
            return -(1 as i32);
        }
        if !(*pbi).ec_active
            && (data_end.offset_from(data) as i64) < first_partition_length_in_bytes as i64
        {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet or corrupt partition 0 length\0" as *const u8 as *const i8,
            );
        }
        init_frame(pbi);
        if vp8dx_start_decode(
            bc as *mut BoolDecoder,
            data,
            data_end.offset_from(data) as u32,
            (*pbi).decrypt_cb,
            (*pbi).decrypt_state,
        ) != 0
        {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate bool decoder 0\0" as *const u8 as *const i8,
            );
        }
        if (*pc).frame_type as u32 == KEY_FRAME as u32 {
            vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
            (*pc).clamp_type =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u32;
        }
        (*xd).segmentation_enabled =
            vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
        if (*xd).segmentation_enabled != 0 {
            (*xd).update_mb_segmentation_map =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
            (*xd).update_mb_segmentation_data =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
            if (*xd).update_mb_segmentation_data != 0 {
                (*xd).mb_segment_abs_delta =
                    vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
                core::ptr::write_bytes(
                    &raw mut (*xd).segment_feature_data as *mut [i8; 4] as *mut c_void as *mut u8,
                    0 as u8,
                    ::core::mem::size_of::<[[i8; 4]; 2]>() as usize,
                );
                i = 0 as i32;
                while i < MB_LVL_MAX as i32 {
                    j = 0 as i32;
                    while j < MAX_MB_SEGMENTS {
                        if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                            (*xd).segment_feature_data[i as usize][j as usize] = vp8_decode_value(
                                bc as *mut BoolDecoder,
                                *mb_feature_data_bits.offset(i as isize),
                            )
                                as i8;
                            if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0
                            {
                                (*xd).segment_feature_data[i as usize][j as usize] =
                                    -((*xd).segment_feature_data[i as usize][j as usize] as i32)
                                        as i8;
                            }
                        } else {
                            (*xd).segment_feature_data[i as usize][j as usize] = 0 as i8;
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
            if (*xd).update_mb_segmentation_map != 0 {
                core::ptr::write_bytes(
                    &raw mut (*xd).mb_segment_tree_probs as *mut u8 as *mut c_void as *mut u8,
                    255 as u8,
                    ::core::mem::size_of::<[u8; 3]>() as usize,
                );
                i = 0 as i32;
                while i < MB_FEATURE_TREE_PROBS {
                    if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                        (*xd).mb_segment_tree_probs[i as usize] =
                            vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as u8;
                    }
                    i += 1;
                }
            }
        } else {
            (*xd).update_mb_segmentation_map = 0 as u8;
            (*xd).update_mb_segmentation_data = 0 as u8;
        }
        (*pc).filter_type = vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u32;
        (*pc).filter_level = vp8_decode_value(bc as *mut BoolDecoder, 6 as i32);
        (*pc).sharpness_level = vp8_decode_value(bc as *mut BoolDecoder, 3 as i32);
        (*xd).mode_ref_lf_delta_update = 0 as u8;
        (*xd).mode_ref_lf_delta_enabled =
            vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
        if (*xd).mode_ref_lf_delta_enabled != 0 {
            (*xd).mode_ref_lf_delta_update =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) as u8;
            if (*xd).mode_ref_lf_delta_update != 0 {
                i = 0 as i32;
                while i < MAX_REF_LF_DELTAS {
                    if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                        (*xd).ref_lf_deltas[i as usize] =
                            vp8_decode_value(bc as *mut BoolDecoder, 6 as i32) as i8;
                        if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                            (*xd).ref_lf_deltas[i as usize] =
                                ((*xd).ref_lf_deltas[i as usize] as i32 * -(1 as i32)) as i8;
                        }
                    }
                    i += 1;
                }
                i = 0 as i32;
                while i < MAX_MODE_LF_DELTAS {
                    if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                        (*xd).mode_lf_deltas[i as usize] =
                            vp8_decode_value(bc as *mut BoolDecoder, 6 as i32) as i8;
                        if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                            (*xd).mode_lf_deltas[i as usize] =
                                ((*xd).mode_lf_deltas[i as usize] as i32 * -(1 as i32)) as i8;
                        }
                    }
                    i += 1;
                }
            }
        }
        setup_token_decoder(pbi, data.offset(first_partition_length_in_bytes as isize));
        (*xd).current_bc = (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(0 as isize)
            as *mut Vp8Reader as *mut c_void;
        let mut Q: i32 = 0;
        let mut q_update: i32 = 0;
        Q = vp8_decode_value(bc as *mut BoolDecoder, 7 as i32);
        (*pc).base_qindex = Q;
        q_update = 0 as i32;
        (*pc).y1dc_delta_q = get_delta_q(bc, (*pc).y1dc_delta_q, &raw mut q_update);
        (*pc).y2dc_delta_q = get_delta_q(bc, (*pc).y2dc_delta_q, &raw mut q_update);
        (*pc).y2ac_delta_q = get_delta_q(bc, (*pc).y2ac_delta_q, &raw mut q_update);
        (*pc).uvdc_delta_q = get_delta_q(bc, (*pc).uvdc_delta_q, &raw mut q_update);
        (*pc).uvac_delta_q = get_delta_q(bc, (*pc).uvac_delta_q, &raw mut q_update);
        if q_update != 0 {
            vp8cx_init_de_quantizer(pbi);
        }
        vp8_mb_init_dequantizer(pbi, &raw mut (*pbi).mb);
        if (*pc).frame_type as u32 != KEY_FRAME as u32 {
            (*pc).refresh_golden_frame =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
            (*pc).refresh_alt_ref_frame =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
            (*pc).copy_buffer_to_gf = 0 as i32;
            if (*pc).refresh_golden_frame == 0 {
                (*pc).copy_buffer_to_gf = vp8_decode_value(bc as *mut BoolDecoder, 2 as i32);
            }
            (*pc).copy_buffer_to_arf = 0 as i32;
            if (*pc).refresh_alt_ref_frame == 0 {
                (*pc).copy_buffer_to_arf = vp8_decode_value(bc as *mut BoolDecoder, 2 as i32);
            }
            (*pc).ref_frame_sign_bias[GOLDEN_FRAME as usize] =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
            (*pc).ref_frame_sign_bias[ALTREF_FRAME as usize] =
                vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
        }
        (*pc).refresh_entropy_probs =
            vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0;
        if !(*pc).refresh_entropy_probs {
            (*pc).lfc = (*pc).fc;
        }
        (*pc).refresh_last_frame = ((*pc).frame_type as u32 == KEY_FRAME as u32
            || vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0)
            as i32;
        (*pbi).independent_partitions = true;
        i = 0 as i32;
        while i < BLOCK_TYPES {
            j = 0 as i32;
            while j < COEF_BANDS {
                k = 0 as i32;
                while k < PREV_COEF_CONTEXTS {
                    l = 0 as i32;
                    while l < ENTROPY_NODES {
                        let p: *mut u8 =
                            (&raw mut *(&raw mut *(&raw mut *(&raw mut (*pc).fc.coef_probs
                                as *mut [[[u8; 11]; 3]; 8])
                                .offset(i as isize)
                                as *mut [[u8; 11]; 3])
                                .offset(j as isize)
                                as *mut [u8; 11])
                                .offset(k as isize) as *mut u8)
                                .offset(l as isize);
                        if vp8dx_decode_bool(
                            bc as *mut BoolDecoder,
                            vp8_coef_update_probs[i as usize][j as usize][k as usize][l as usize]
                                as i32,
                        ) != 0
                        {
                            *p = vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as u8;
                        }
                        if k > 0 as i32
                            && *p as i32
                                != (*pc).fc.coef_probs[i as usize][j as usize]
                                    [(k - 1 as i32) as usize][l as usize]
                                    as i32
                        {
                            (*pbi).independent_partitions = false;
                        }
                        l += 1;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        core::ptr::write_bytes(
            &raw mut (*xd).qcoeff as *mut i16 as *mut c_void as *mut u8,
            0 as u8,
            ::core::mem::size_of::<[i16; 400]>() as usize,
        );
        vp8_decode_mode_mvs(pbi);
        core::ptr::write_bytes(
            (*pc).above_context as *mut c_void as *mut u8,
            0 as u8,
            (::core::mem::size_of::<EntropyContextPlanes>() as usize)
                .wrapping_mul((*pc).mb_cols as usize),
        );
        (*pbi).frame_corrupt_residual = 0 as i32;
        if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0
            && (*pc).multi_token_partition as u32 != ONE_PARTITION as u32
        {
            let mut thread: u32 = 0;
            if vp8mt_decode_mb_rows(pbi, xd) != 0 {
                vp8_decoder_remove_threads(pbi);
                (*pbi).restart_threads = true;
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    ::core::ptr::null::<i8>(),
                );
            }
            vp8_yv12_extend_frame_borders_c(yv12_fb_new as *mut Yv12BufferConfig);
            thread = 0 as u32;
            while thread < (*pbi).decoding_thread_count {
                corrupt_tokens |= (*(*pbi).mb_row_di.offset(thread as isize)).mbd.corrupted;
                thread = thread.wrapping_add(1);
            }
        } else {
            decode_mb_rows(pbi);
            corrupt_tokens |= (*xd).corrupted;
        }
        (*yv12_fb_new).corrupted = vp8dx_bool_error(bc as *mut BoolDecoder);
        (*yv12_fb_new).corrupted |= corrupt_tokens;
        if !(*pbi).decoded_key_frame {
            if (*pc).frame_type as u32 == KEY_FRAME as u32 && (*yv12_fb_new).corrupted == 0 {
                (*pbi).decoded_key_frame = true;
            } else {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"A stream must start with a complete key frame\0" as *const u8 as *const i8,
                );
            }
        }
        if !(*pc).refresh_entropy_probs {
            (*pc).fc = (*pc).lfc;
            (*pbi).independent_partitions = prev_independent_partitions;
        }
        0 as i32
    }
}
pub const __ATOMIC_ACQUIRE: i32 = 2 as i32;
