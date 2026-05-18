use std::ffi::c_void;
unsafe extern "Rust" {
    static vp8_norm: [u8; 256];
    fn vp8dx_bool_decoder_fill(br: *mut BoolDecoder);
    static vp8_mv_update_probs: [MvContext; 2];
    static vp8_bmode_tree: [Vp8TreeIndex; 0];
    static vp8_ymode_tree: [Vp8TreeIndex; 0];
    static vp8_kf_ymode_tree: [Vp8TreeIndex; 0];
    static vp8_uv_mode_tree: [Vp8TreeIndex; 0];
    static vp8_small_mvtree: [Vp8TreeIndex; 0];
    static vp8_kf_bmode_prob: [[[Vp8Prob; 9]; 10]; 10];
    static vp8_kf_uv_mode_prob: [Vp8Prob; 3];
    static vp8_kf_ymode_prob: [Vp8Prob; 4];
    static vp8_mode_contexts: [[i32; 4]; 6];
    static vp8_mbsplit_offset: [[u8; 16]; 4];
}
pub type DarwinNaturalT = u32;
pub type DarwinSizeT = usize;
pub type DarwinMachPortNameT = DarwinNaturalT;
pub type DarwinMachPortT = DarwinMachPortNameT;
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
pub type SizeT = DarwinSizeT;
pub type PthreadT = *mut c_void;
pub type MachPortT = DarwinMachPortT;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorRangeT = VpxColorRange;
pub type VpxCodecErrT = u32;
pub const VPX_CODEC_LIST_END: VpxCodecErrT = 9;
pub const VPX_CODEC_INVALID_PARAM: VpxCodecErrT = 8;
pub const VPX_CODEC_CORRUPT_FRAME: VpxCodecErrT = 7;
pub const VPX_CODEC_UNSUP_FEATURE: VpxCodecErrT = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: VpxCodecErrT = 5;
pub const VPX_CODEC_INCAPABLE: VpxCodecErrT = 4;
pub const VPX_CODEC_ABI_MISMATCH: VpxCodecErrT = 3;
pub const VPX_CODEC_MEM_ERROR: VpxCodecErrT = 2;
pub const VPX_CODEC_ERROR: VpxCodecErrT = 1;
pub const VPX_CODEC_OK: VpxCodecErrT = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
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
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: SizeT,
    pub border: i32,
    pub frame_size: SizeT,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: VpxColorSpaceT,
    pub color_range: VpxColorRangeT,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
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
    pub ready_for_new_data: i32,
    pub prob_intra: Vp8Prob,
    pub prob_last: Vp8Prob,
    pub prob_gf: Vp8Prob,
    pub prob_skip_false: Vp8Prob,
    pub ec_enabled: i32,
    pub ec_active: i32,
    pub decoded_key_frame: i32,
    pub independent_partitions: i32,
    pub frame_corrupt_residual: i32,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
    pub restart_threads: i32,
}
pub type VpxDecryptCb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type Vp8Prob = u8;
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
    pub frame_type: FrameType,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: *mut EntropyContextPlanes,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [Vp8Prob; 3],
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
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type EntropyContext = i8;
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
pub type ModeInfo = Modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: BPredictionMode,
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
pub type BPredictionMode = u32;
pub const B_MODE_COUNT: BPredictionMode = 14;
pub const NEW4X4: BPredictionMode = 13;
pub const ZERO4X4: BPredictionMode = 12;
pub const ABOVE4X4: BPredictionMode = 11;
pub const LEFT4X4: BPredictionMode = 10;
pub const B_HU_PRED: BPredictionMode = 9;
pub const B_HD_PRED: BPredictionMode = 8;
pub const B_VL_PRED: BPredictionMode = 7;
pub const B_VR_PRED: BPredictionMode = 6;
pub const B_RD_PRED: BPredictionMode = 5;
pub const B_LD_PRED: BPredictionMode = 4;
pub const B_HE_PRED: BPredictionMode = 3;
pub const B_VE_PRED: BPredictionMode = 2;
pub const B_TM_PRED: BPredictionMode = 1;
pub const B_DC_PRED: BPredictionMode = 0;
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
pub type BLOCKD = Blockd;
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
pub struct VpxAtomicInt {
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FragmentData {
    pub enabled: i32,
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
    pub value: Vp8BdValue,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
}
pub type Vp8BdValue = SizeT;
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
    pub clamp_type: ClampType,
    pub frame_to_show: *mut Yv12BufferConfig,
    pub yv12_fb: [Yv12BufferConfig; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: Yv12BufferConfig,
    pub last_frame_type: FrameType,
    pub frame_type: FrameType,
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
    pub mip: *mut ModeInfo,
    pub mi: *mut ModeInfo,
    pub show_frame_mi: *mut ModeInfo,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: LoopFilterInfoN,
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
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: EntropyContextPlanes,
    pub lfc: FrameContext,
    pub fc: FrameContext,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: TokenPartition,
    pub processor_core_count: i32,
}
pub type TokenPartition = u32;
pub const EIGHT_PARTITION: TokenPartition = 3;
pub const FOUR_PARTITION: TokenPartition = 2;
pub const TWO_PARTITION: TokenPartition = 1;
pub const ONE_PARTITION: TokenPartition = 0;
pub type FrameContext = FrameContexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameContexts {
    pub bmode_prob: [Vp8Prob; 9],
    pub ymode_prob: [Vp8Prob; 4],
    pub uv_mode_prob: [Vp8Prob; 3],
    pub sub_mv_ref_prob: [Vp8Prob; 3],
    pub coef_probs: [[[[Vp8Prob; 11]; 3]; 8]; 4],
    pub mvc: [MvContext; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [Vp8Prob; 19],
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
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
pub type ClampType = u32;
pub const RECON_CLAMP_NOTREQUIRED: ClampType = 1;
pub const RECON_CLAMP_REQUIRED: ClampType = 0;
pub type Vp8TreeIndex = i8;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MVPcount: C2RustUnnamed = 19;
pub const MVPbits: C2RustUnnamed = 9;
pub const MVPshort: C2RustUnnamed = 2;
pub const MVPsign: C2RustUnnamed = 1;
pub const mvpis_short: C2RustUnnamed = 0;
pub const mvnum_short: C2RustUnnamed = 8;
pub const mvlong_width: C2RustUnnamed = 10;
pub const MVfpvals: C2RustUnnamed = 511;
pub const mvfp_max: C2RustUnnamed = 255;
pub const MVvals: C2RustUnnamed = 2047;
pub const mv_max: C2RustUnnamed = 1023;
pub type MbPredictionMode = u32;
pub const MB_MODE_COUNT: MbPredictionMode = 10;
pub const SPLITMV: MbPredictionMode = 9;
pub const NEWMV: MbPredictionMode = 8;
pub const ZEROMV: MbPredictionMode = 7;
pub const NEARMV: MbPredictionMode = 6;
pub const NEARESTMV: MbPredictionMode = 5;
pub const B_PRED: MbPredictionMode = 4;
pub const TM_PRED: MbPredictionMode = 3;
pub const H_PRED: MbPredictionMode = 2;
pub const V_PRED: MbPredictionMode = 1;
pub const DC_PRED: MbPredictionMode = 0;
pub type MvReferenceFrame = u32;
pub const MAX_REF_FRAMES: MvReferenceFrame = 4;
pub const ALTREF_FRAME: MvReferenceFrame = 3;
pub const GOLDEN_FRAME: MvReferenceFrame = 2;
pub const LAST_FRAME: MvReferenceFrame = 1;
pub const INTRA_FRAME: MvReferenceFrame = 0;
pub const CNT_NEAREST: C2RustUnnamed_0 = 1;
pub const CNT_NEAR: C2RustUnnamed_0 = 2;
pub const CNT_SPLITMV: C2RustUnnamed_0 = 3;
pub const CNT_INTRA: C2RustUnnamed_0 = 0;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_0 = u32;
pub const CHAR_BIT: i32 = 8 as i32;
pub const vp8_prob_half: Vp8Prob = 128 as Vp8Prob;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<Vp8BdValue>() as i32 * CHAR_BIT;
unsafe fn vp8dx_decode_bool(mut br: *mut BoolDecoder, mut probability: i32) -> i32 {
    unsafe {
        let mut bit: u32 = 0 as u32;
        let mut value: Vp8BdValue = 0;
        let mut split: u32 = 0;
        let mut bigsplit: Vp8BdValue = 0;
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
        bigsplit = (split as Vp8BdValue) << (VP8_BD_VALUE_SIZE - 8 as i32);
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
unsafe fn vp8_treed_read(r: *mut Vp8Reader, mut t: *const Vp8TreeIndex, p: *const Vp8Prob) -> i32 {
    unsafe {
        let mut i: Vp8TreeIndex = 0 as Vp8TreeIndex;
        loop {
            i = *t.offset(
                (i as i32
                    + vp8dx_decode_bool(
                        r as *mut BoolDecoder,
                        *p.offset((i as i32 >> 1 as i32) as isize) as i32,
                    )) as isize,
            );
            if !(i as i32 > 0 as i32) {
                break;
            }
        }
        -(i as i32)
    }
}
#[inline]
unsafe fn mv_bias(
    mut refmb_ref_frame_sign_bias: i32,
    mut refframe: i32,
    mut mvp: *mut IntMv,
    mut ref_frame_sign_bias: *const i32,
) {
    unsafe {
        if refmb_ref_frame_sign_bias != *ref_frame_sign_bias.offset(refframe as isize) {
            (*mvp).as_mv.row = ((*mvp).as_mv.row as i32 * -(1 as i32)) as i16;
            (*mvp).as_mv.col = ((*mvp).as_mv.col as i32 * -(1 as i32)) as i16;
        }
    }
}
pub const LEFT_TOP_MARGIN: i32 = (16 as i32) << 3 as i32;
pub const RIGHT_BOTTOM_MARGIN: i32 = (16 as i32) << 3 as i32;
#[inline]
unsafe fn vp8_clamp_mv2(mut mv: *mut IntMv, mut xd: *const MACROBLOCKD) {
    unsafe {
        if ((*mv).as_mv.col as i32) < (*xd).mb_to_left_edge - LEFT_TOP_MARGIN {
            (*mv).as_mv.col = ((*xd).mb_to_left_edge - LEFT_TOP_MARGIN) as i16;
        } else if (*mv).as_mv.col as i32 > (*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN {
            (*mv).as_mv.col = ((*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN) as i16;
        }
        if ((*mv).as_mv.row as i32) < (*xd).mb_to_top_edge - LEFT_TOP_MARGIN {
            (*mv).as_mv.row = ((*xd).mb_to_top_edge - LEFT_TOP_MARGIN) as i16;
        } else if (*mv).as_mv.row as i32 > (*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN {
            (*mv).as_mv.row = ((*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN) as i16;
        }
    }
}
#[inline]
unsafe fn vp8_check_mv_bounds(
    mut mv: *mut IntMv,
    mut mb_to_left_edge: i32,
    mut mb_to_right_edge: i32,
    mut mb_to_top_edge: i32,
    mut mb_to_bottom_edge: i32,
) -> u32 {
    unsafe {
        let mut need_to_clamp: u32 = 0;
        need_to_clamp = (((*mv).as_mv.col as i32) < mb_to_left_edge) as u32;
        need_to_clamp |= ((*mv).as_mv.col as i32 > mb_to_right_edge) as u32;
        need_to_clamp |= (((*mv).as_mv.row as i32) < mb_to_top_edge) as u32;
        need_to_clamp |= ((*mv).as_mv.row as i32 > mb_to_bottom_edge) as u32;
        need_to_clamp
    }
}
#[inline]
unsafe fn left_block_mode(mut cur_mb: *const ModeInfo, mut b: i32) -> BPredictionMode {
    unsafe {
        if b & 3 as i32 == 0 {
            cur_mb = cur_mb.offset(-1);
            match (*cur_mb).mbmi.mode as i32 {
                4 => {
                    return (*(&raw const (*cur_mb).bmi as *const BModeInfo)
                        .offset(b as isize)
                        .offset(3 as isize))
                    .as_mode;
                }
                0 => return B_DC_PRED,
                1 => return B_VE_PRED,
                2 => return B_HE_PRED,
                3 => return B_TM_PRED,
                _ => return B_DC_PRED,
            }
        }
        (*(&raw const (*cur_mb).bmi as *const BModeInfo)
            .offset(b as isize)
            .offset(-(1 as isize)))
        .as_mode
    }
}
#[inline]
unsafe fn above_block_mode(
    mut cur_mb: *const ModeInfo,
    mut b: i32,
    mut mi_stride: i32,
) -> BPredictionMode {
    unsafe {
        if b >> 2 as i32 == 0 {
            cur_mb = cur_mb.offset(-(mi_stride as isize));
            match (*cur_mb).mbmi.mode as i32 {
                4 => {
                    return (*(&raw const (*cur_mb).bmi as *const BModeInfo)
                        .offset(b as isize)
                        .offset(12 as isize))
                    .as_mode;
                }
                0 => return B_DC_PRED,
                1 => return B_VE_PRED,
                2 => return B_HE_PRED,
                3 => return B_TM_PRED,
                _ => return B_DC_PRED,
            }
        }
        (*(&raw const (*cur_mb).bmi as *const BModeInfo)
            .offset(b as isize)
            .offset(-(4 as isize)))
        .as_mode
    }
}
unsafe fn read_bmode(mut bc: *mut Vp8Reader, mut p: *const Vp8Prob) -> BPredictionMode {
    unsafe {
        let i: i32 = vp8_treed_read(bc, &raw const vp8_bmode_tree as *const Vp8TreeIndex, p) as i32;
        i as BPredictionMode
    }
}
unsafe fn read_ymode(mut bc: *mut Vp8Reader, mut p: *const Vp8Prob) -> MbPredictionMode {
    unsafe {
        let i: i32 = vp8_treed_read(bc, &raw const vp8_ymode_tree as *const Vp8TreeIndex, p) as i32;
        i as MbPredictionMode
    }
}
unsafe fn read_kf_ymode(mut bc: *mut Vp8Reader, mut p: *const Vp8Prob) -> MbPredictionMode {
    unsafe {
        let i: i32 =
            vp8_treed_read(bc, &raw const vp8_kf_ymode_tree as *const Vp8TreeIndex, p) as i32;
        i as MbPredictionMode
    }
}
unsafe fn read_uv_mode(mut bc: *mut Vp8Reader, mut p: *const Vp8Prob) -> MbPredictionMode {
    unsafe {
        let i: i32 =
            vp8_treed_read(bc, &raw const vp8_uv_mode_tree as *const Vp8TreeIndex, p) as i32;
        i as MbPredictionMode
    }
}
unsafe fn read_kf_modes(mut pbi: *mut Vp8dComp, mut mi: *mut ModeInfo) {
    unsafe {
        let bc: *mut Vp8Reader =
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut Vp8Reader;
        let mis: i32 = (*pbi).common.mode_info_stride;
        (*mi).mbmi.ref_frame = INTRA_FRAME as u8;
        (*mi).mbmi.mode = read_kf_ymode(bc, &raw const vp8_kf_ymode_prob as *const Vp8Prob) as u8;
        if (*mi).mbmi.mode as i32 == B_PRED as i32 {
            let mut i: i32 = 0 as i32;
            (*mi).mbmi.is_4x4 = 1 as u8;
            loop {
                let A: BPredictionMode = above_block_mode(mi, i, mis) as BPredictionMode;
                let L: BPredictionMode = left_block_mode(mi, i) as BPredictionMode;
                (*mi).bmi[i as usize].as_mode = read_bmode(
                    bc,
                    &raw const *(&raw const *(&raw const vp8_kf_bmode_prob
                        as *const [[Vp8Prob; 9]; 10])
                        .offset(A as isize) as *const [Vp8Prob; 9])
                        .offset(L as isize) as *const Vp8Prob,
                );
                i += 1;
                if !(i < 16 as i32) {
                    break;
                }
            }
        }
        (*mi).mbmi.uv_mode =
            read_uv_mode(bc, &raw const vp8_kf_uv_mode_prob as *const Vp8Prob) as u8;
    }
}
unsafe fn read_mvcomponent(mut r: *mut Vp8Reader, mut mvc: *const MvContext) -> i32 {
    unsafe {
        let p: *const Vp8Prob = mvc as *const Vp8Prob;
        let mut x: i32 = 0 as i32;
        if vp8dx_decode_bool(
            r as *mut BoolDecoder,
            *p.offset(mvpis_short as isize) as i32,
        ) != 0
        {
            let mut i: i32 = 0 as i32;
            loop {
                x += vp8dx_decode_bool(
                    r as *mut BoolDecoder,
                    *p.offset((MVPbits as i32 + i) as isize) as i32,
                ) << i;
                i += 1;
                if !(i < 3 as i32) {
                    break;
                }
            }
            i = mvlong_width as i32 - 1 as i32;
            loop {
                x += vp8dx_decode_bool(
                    r as *mut BoolDecoder,
                    *p.offset((MVPbits as i32 + i) as isize) as i32,
                ) << i;
                i -= 1;
                if !(i > 3 as i32) {
                    break;
                }
            }
            if x & 0xfff0 as i32 == 0
                || vp8dx_decode_bool(
                    r as *mut BoolDecoder,
                    *p.offset((MVPbits as i32 + 3 as i32) as isize) as i32,
                ) != 0
            {
                x += 8 as i32;
            }
        } else {
            x = vp8_treed_read(
                r,
                &raw const vp8_small_mvtree as *const Vp8TreeIndex,
                p.offset(MVPshort as isize),
            );
        }
        if x != 0
            && vp8dx_decode_bool(r as *mut BoolDecoder, *p.offset(MVPsign as isize) as i32) != 0
        {
            x = -x;
        }
        x
    }
}
unsafe fn read_mv(mut r: *mut Vp8Reader, mut mv: *mut MV, mut mvc: *const MvContext) {
    unsafe {
        (*mv).row = (read_mvcomponent(r, mvc) * 2 as i32) as i16;
        mvc = mvc.offset(1);
        (*mv).col = (read_mvcomponent(r, mvc) * 2 as i32) as i16;
    }
}
unsafe fn read_mvcontexts(mut bc: *mut Vp8Reader, mut mvc: *mut MvContext) {
    unsafe {
        let mut i: i32 = 0 as i32;
        loop {
            let mut up: *const Vp8Prob = &raw const (*(&raw const vp8_mv_update_probs
                as *const MvContext)
                .offset(i as isize))
            .prob as *const Vp8Prob;
            let mut p: *mut Vp8Prob = mvc.offset(i as isize) as *mut Vp8Prob;
            let pstop: *mut Vp8Prob = p.offset(MVPcount as isize);
            loop {
                let fresh2 = up;
                up = up.offset(1);
                if vp8dx_decode_bool(bc as *mut BoolDecoder, *fresh2 as i32) != 0 {
                    let x: Vp8Prob = vp8_decode_value(bc as *mut BoolDecoder, 7 as i32) as Vp8Prob;
                    *p = (if x as i32 != 0 {
                        (x as i32) << 1 as i32
                    } else {
                        1 as i32
                    }) as Vp8Prob;
                }
                p = p.offset(1);
                if !(p < pstop) {
                    break;
                }
            }
            i += 1;
            if !(i < 2 as i32) {
                break;
            }
        }
    }
}
static mut mbsplit_fill_count: [u8; 4] = [8 as u8, 8 as u8, 4 as u8, 1 as u8];
static mut mbsplit_fill_offset: [[u8; 16]; 4] = [
    [
        0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8,
        10 as u8, 11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8,
    ],
    [
        0 as u8, 1 as u8, 4 as u8, 5 as u8, 8 as u8, 9 as u8, 12 as u8, 13 as u8, 2 as u8, 3 as u8,
        6 as u8, 7 as u8, 10 as u8, 11 as u8, 14 as u8, 15 as u8,
    ],
    [
        0 as u8, 1 as u8, 4 as u8, 5 as u8, 2 as u8, 3 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8,
        12 as u8, 13 as u8, 10 as u8, 11 as u8, 14 as u8, 15 as u8,
    ],
    [
        0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8,
        10 as u8, 11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8,
    ],
];
unsafe fn mb_mode_mv_init(mut pbi: *mut Vp8dComp) {
    unsafe {
        let bc: *mut Vp8Reader =
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut Vp8Reader;
        let mvc: *mut MvContext = &raw mut (*pbi).common.fc.mvc as *mut MvContext;
        (*pbi).common.mb_no_coeff_skip =
            vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32);
        (*pbi).prob_skip_false = 0 as Vp8Prob;
        if (*pbi).common.mb_no_coeff_skip != 0 {
            (*pbi).prob_skip_false = vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
        }
        if (*pbi).common.frame_type as u32 != KEY_FRAME as u32 {
            (*pbi).prob_intra = vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
            (*pbi).prob_last = vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
            (*pbi).prob_gf = vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
            if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                let mut i: i32 = 0 as i32;
                loop {
                    (*pbi).common.fc.ymode_prob[i as usize] =
                        vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
                    i += 1;
                    if !(i < 4 as i32) {
                        break;
                    }
                }
            }
            if vp8dx_decode_bool(bc as *mut BoolDecoder, vp8_prob_half as i32) != 0 {
                let mut i_0: i32 = 0 as i32;
                loop {
                    (*pbi).common.fc.uv_mode_prob[i_0 as usize] =
                        vp8_decode_value(bc as *mut BoolDecoder, 8 as i32) as Vp8Prob;
                    i_0 += 1;
                    if !(i_0 < 3 as i32) {
                        break;
                    }
                }
            }
            read_mvcontexts(bc, mvc);
        }
    }
}
#[unsafe(no_mangle)]
pub static mut vp8_sub_mv_ref_prob3: [[Vp8Prob; 3]; 8] = [
    [147 as Vp8Prob, 136 as Vp8Prob, 18 as Vp8Prob],
    [223 as Vp8Prob, 1 as Vp8Prob, 34 as Vp8Prob],
    [106 as Vp8Prob, 145 as Vp8Prob, 1 as Vp8Prob],
    [208 as Vp8Prob, 1 as Vp8Prob, 1 as Vp8Prob],
    [179 as Vp8Prob, 121 as Vp8Prob, 1 as Vp8Prob],
    [223 as Vp8Prob, 1 as Vp8Prob, 34 as Vp8Prob],
    [179 as Vp8Prob, 121 as Vp8Prob, 1 as Vp8Prob],
    [208 as Vp8Prob, 1 as Vp8Prob, 1 as Vp8Prob],
];
unsafe fn get_sub_mv_ref_prob(left: u32, above: u32) -> *const Vp8Prob {
    unsafe {
        let mut lez: i32 = (left == 0 as u32) as i32;
        let mut aez: i32 = (above == 0 as u32) as i32;
        let mut lea: i32 = (left == above) as i32;
        let mut prob: *const Vp8Prob = ::core::ptr::null::<Vp8Prob>();
        prob = &raw const *(&raw const vp8_sub_mv_ref_prob3 as *const [Vp8Prob; 3])
            .offset((aez << 2 as i32 | lez << 1 as i32 | lea) as isize)
            as *const Vp8Prob;
        prob
    }
}
unsafe fn decode_split_mv(
    bc: *mut Vp8Reader,
    mut mi: *mut ModeInfo,
    mut left_mb: *const ModeInfo,
    mut above_mb: *const ModeInfo,
    mut mbmi: *mut MbModeInfo,
    mut best_mv: IntMv,
    mvc: *mut MvContext,
    mut mb_to_left_edge: i32,
    mut mb_to_right_edge: i32,
    mut mb_to_top_edge: i32,
    mut mb_to_bottom_edge: i32,
) {
    unsafe {
        let mut s: i32 = 0;
        let mut num_p: i32 = 0;
        let mut j: i32 = 0 as i32;
        s = 3 as i32;
        num_p = 16 as i32;
        if vp8dx_decode_bool(bc as *mut BoolDecoder, 110 as i32) != 0 {
            s = 2 as i32;
            num_p = 4 as i32;
            if vp8dx_decode_bool(bc as *mut BoolDecoder, 111 as i32) != 0 {
                s = vp8dx_decode_bool(bc as *mut BoolDecoder, 150 as i32);
                num_p = 2 as i32;
            }
        }
        loop {
            let mut leftmv: IntMv = IntMv { as_int: 0 };
            let mut abovemv: IntMv = IntMv { as_int: 0 };
            let mut blockmv: IntMv = IntMv { as_int: 0 };
            let mut k: i32 = 0;
            let mut prob: *const Vp8Prob = ::core::ptr::null::<Vp8Prob>();
            k = vp8_mbsplit_offset[s as usize][j as usize] as i32;
            if k & 3 as i32 == 0 {
                if (*left_mb).mbmi.mode as i32 != SPLITMV as i32 {
                    leftmv.as_int = (*left_mb).mbmi.mv.as_int;
                } else {
                    leftmv.as_int = (*(&raw const (*left_mb).bmi as *const BModeInfo)
                        .offset(k as isize)
                        .offset(4 as isize)
                        .offset(-(1 as isize)))
                    .mv
                    .as_int;
                }
            } else {
                leftmv.as_int = (*(&raw mut (*mi).bmi as *mut BModeInfo)
                    .offset(k as isize)
                    .offset(-(1 as isize)))
                .mv
                .as_int;
            }
            if k >> 2 as i32 == 0 {
                if (*above_mb).mbmi.mode as i32 != SPLITMV as i32 {
                    abovemv.as_int = (*above_mb).mbmi.mv.as_int;
                } else {
                    abovemv.as_int = (*(&raw const (*above_mb).bmi as *const BModeInfo)
                        .offset(k as isize)
                        .offset(16 as isize)
                        .offset(-(4 as isize)))
                    .mv
                    .as_int;
                }
            } else {
                abovemv.as_int = (*(&raw mut (*mi).bmi as *mut BModeInfo)
                    .offset(k as isize)
                    .offset(-(4 as isize)))
                .mv
                .as_int;
            }
            prob = get_sub_mv_ref_prob(leftmv.as_int, abovemv.as_int);
            if vp8dx_decode_bool(bc as *mut BoolDecoder, *prob.offset(0 as isize) as i32) != 0 {
                if vp8dx_decode_bool(bc as *mut BoolDecoder, *prob.offset(1 as isize) as i32) != 0 {
                    blockmv.as_int = 0 as u32;
                    if vp8dx_decode_bool(bc as *mut BoolDecoder, *prob.offset(2 as isize) as i32)
                        != 0
                    {
                        blockmv.as_mv.row =
                            (read_mvcomponent(bc, mvc.offset(0 as isize) as *mut MvContext)
                                * 2 as i32) as i16;
                        blockmv.as_mv.row =
                            (blockmv.as_mv.row as i32 + best_mv.as_mv.row as i32) as i16;
                        blockmv.as_mv.col =
                            (read_mvcomponent(bc, mvc.offset(1 as isize) as *mut MvContext)
                                * 2 as i32) as i16;
                        blockmv.as_mv.col =
                            (blockmv.as_mv.col as i32 + best_mv.as_mv.col as i32) as i16;
                    }
                } else {
                    blockmv.as_int = abovemv.as_int;
                }
            } else {
                blockmv.as_int = leftmv.as_int;
            }
            (*mbmi).need_to_clamp_mvs = ((*mbmi).need_to_clamp_mvs as u32
                | vp8_check_mv_bounds(
                    &raw mut blockmv,
                    mb_to_left_edge,
                    mb_to_right_edge,
                    mb_to_top_edge,
                    mb_to_bottom_edge,
                )) as u8;
            let mut fill_offset: *const u8 = ::core::ptr::null::<u8>();
            let mut fill_count: u32 = mbsplit_fill_count[s as usize] as u32;
            fill_offset = (&raw const *(&raw const mbsplit_fill_offset as *const [u8; 16])
                .offset(s as isize) as *const u8)
                .offset(
                    (j as i32
                        * *(&raw const mbsplit_fill_count as *const u8).offset(s as isize) as i32)
                        as isize,
                ) as *const u8;
            loop {
                (*mi).bmi[*fill_offset as usize].mv.as_int = blockmv.as_int;
                fill_offset = fill_offset.offset(1);
                fill_count = fill_count.wrapping_sub(1);
                if !(fill_count != 0) {
                    break;
                }
            }
            j += 1;
            if !(j < num_p) {
                break;
            }
        }
        (*mbmi).partitioning = s as u8;
    }
}
unsafe fn read_mb_modes_mv(
    mut pbi: *mut Vp8dComp,
    mut mi: *mut ModeInfo,
    mut mbmi: *mut MbModeInfo,
) {
    unsafe {
        let bc: *mut Vp8Reader =
            (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut Vp8Reader;
        (*mbmi).ref_frame =
            vp8dx_decode_bool(bc as *mut BoolDecoder, (*pbi).prob_intra as i32) as u8;
        if (*mbmi).ref_frame != 0 {
            let mut cnt: [i32; 4] = [0; 4];
            let mut cntx: *mut i32 = &raw mut cnt as *mut i32;
            let mut near_mvs: [IntMv; 4] = [IntMv { as_int: 0 }; 4];
            let mut nmv: *mut IntMv = &raw mut near_mvs as *mut IntMv;
            let mis: i32 = (*pbi).mb.mode_info_stride;
            let mut above: *const ModeInfo = mi.offset(-(mis as isize));
            let mut left: *const ModeInfo = mi.offset(-(1 as isize));
            let mut aboveleft: *const ModeInfo = above.offset(-(1 as isize));
            let mut ref_frame_sign_bias: *mut i32 =
                &raw mut (*pbi).common.ref_frame_sign_bias as *mut i32;
            (*mbmi).need_to_clamp_mvs = 0 as u8;
            if vp8dx_decode_bool(bc as *mut BoolDecoder, (*pbi).prob_last as i32) != 0 {
                (*mbmi).ref_frame = (2 as i32
                    + vp8dx_decode_bool(bc as *mut BoolDecoder, (*pbi).prob_gf as i32))
                    as u8;
            }
            let fresh0 = &mut (*nmv.offset(2 as isize)).as_int;
            *fresh0 = 0 as u32;
            let fresh1 = &mut (*nmv.offset(1 as isize)).as_int;
            *fresh1 = *fresh0;
            (*nmv.offset(0 as isize)).as_int = *fresh1;
            cnt[3 as usize] = 0 as i32;
            cnt[2 as usize] = cnt[3 as usize];
            cnt[1 as usize] = cnt[2 as usize];
            cnt[0 as usize] = cnt[1 as usize];
            if (*above).mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
                if (*above).mbmi.mv.as_int != 0 {
                    nmv = nmv.offset(1);
                    (*nmv).as_int = (*above).mbmi.mv.as_int;
                    mv_bias(
                        *ref_frame_sign_bias.offset((*above).mbmi.ref_frame as isize),
                        (*mbmi).ref_frame as i32,
                        nmv,
                        ref_frame_sign_bias,
                    );
                    cntx = cntx.offset(1);
                }
                *cntx += 2 as i32;
            }
            if (*left).mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
                if (*left).mbmi.mv.as_int != 0 {
                    let mut this_mv: IntMv = IntMv { as_int: 0 };
                    this_mv.as_int = (*left).mbmi.mv.as_int;
                    mv_bias(
                        *ref_frame_sign_bias.offset((*left).mbmi.ref_frame as isize),
                        (*mbmi).ref_frame as i32,
                        &raw mut this_mv,
                        ref_frame_sign_bias,
                    );
                    if this_mv.as_int != (*nmv).as_int {
                        nmv = nmv.offset(1);
                        (*nmv).as_int = this_mv.as_int;
                        cntx = cntx.offset(1);
                    }
                    *cntx += 2 as i32;
                } else {
                    cnt[CNT_INTRA as usize] += 2 as i32;
                }
            }
            if (*aboveleft).mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
                if (*aboveleft).mbmi.mv.as_int != 0 {
                    let mut this_mv_0: IntMv = IntMv { as_int: 0 };
                    this_mv_0.as_int = (*aboveleft).mbmi.mv.as_int;
                    mv_bias(
                        *ref_frame_sign_bias.offset((*aboveleft).mbmi.ref_frame as isize),
                        (*mbmi).ref_frame as i32,
                        &raw mut this_mv_0,
                        ref_frame_sign_bias,
                    );
                    if this_mv_0.as_int != (*nmv).as_int {
                        nmv = nmv.offset(1);
                        (*nmv).as_int = this_mv_0.as_int;
                        cntx = cntx.offset(1);
                    }
                    *cntx += 1 as i32;
                } else {
                    cnt[CNT_INTRA as usize] += 1 as i32;
                }
            }
            if vp8dx_decode_bool(
                bc as *mut BoolDecoder,
                vp8_mode_contexts[cnt[CNT_INTRA as usize] as usize][0 as usize],
            ) != 0
            {
                cnt[CNT_NEAREST as usize] += (cnt[CNT_SPLITMV as usize] > 0 as i32) as i32
                    & ((*nmv).as_int == near_mvs[CNT_NEAREST as usize].as_int) as i32;
                if cnt[CNT_NEAR as usize] > cnt[CNT_NEAREST as usize] {
                    let mut tmp: i32 = 0;
                    tmp = cnt[CNT_NEAREST as usize];
                    cnt[CNT_NEAREST as usize] = cnt[CNT_NEAR as usize];
                    cnt[CNT_NEAR as usize] = tmp;
                    tmp = near_mvs[CNT_NEAREST as usize].as_int as i32;
                    near_mvs[CNT_NEAREST as usize].as_int = near_mvs[CNT_NEAR as usize].as_int;
                    near_mvs[CNT_NEAR as usize].as_int = tmp as u32;
                }
                if vp8dx_decode_bool(
                    bc as *mut BoolDecoder,
                    vp8_mode_contexts[cnt[CNT_NEAREST as usize] as usize][1 as usize],
                ) != 0
                {
                    if vp8dx_decode_bool(
                        bc as *mut BoolDecoder,
                        vp8_mode_contexts[cnt[CNT_NEAR as usize] as usize][2 as usize],
                    ) != 0
                    {
                        let mut mb_to_top_edge: i32 = 0;
                        let mut mb_to_bottom_edge: i32 = 0;
                        let mut mb_to_left_edge: i32 = 0;
                        let mut mb_to_right_edge: i32 = 0;
                        let mvc: *mut MvContext = &raw mut (*pbi).common.fc.mvc as *mut MvContext;
                        let mut near_index: i32 = 0;
                        mb_to_top_edge = (*pbi).mb.mb_to_top_edge;
                        mb_to_bottom_edge = (*pbi).mb.mb_to_bottom_edge;
                        mb_to_top_edge -= LEFT_TOP_MARGIN;
                        mb_to_bottom_edge += RIGHT_BOTTOM_MARGIN;
                        mb_to_right_edge = (*pbi).mb.mb_to_right_edge;
                        mb_to_right_edge += RIGHT_BOTTOM_MARGIN;
                        mb_to_left_edge = (*pbi).mb.mb_to_left_edge;
                        mb_to_left_edge -= LEFT_TOP_MARGIN;
                        near_index = CNT_INTRA as i32
                            + (cnt[CNT_NEAREST as usize] >= cnt[CNT_INTRA as usize]) as i32;
                        vp8_clamp_mv2(
                            (&raw mut near_mvs as *mut IntMv).offset(near_index as isize)
                                as *mut IntMv,
                            &raw mut (*pbi).mb,
                        );
                        cnt[CNT_SPLITMV as usize] = (((*above).mbmi.mode as i32 == SPLITMV as i32)
                            as i32
                            + ((*left).mbmi.mode as i32 == SPLITMV as i32) as i32)
                            * 2 as i32
                            + ((*aboveleft).mbmi.mode as i32 == SPLITMV as i32) as i32;
                        if vp8dx_decode_bool(
                            bc as *mut BoolDecoder,
                            vp8_mode_contexts[cnt[CNT_SPLITMV as usize] as usize][3 as usize],
                        ) != 0
                        {
                            decode_split_mv(
                                bc,
                                mi,
                                left,
                                above,
                                mbmi,
                                near_mvs[near_index as usize],
                                mvc,
                                mb_to_left_edge,
                                mb_to_right_edge,
                                mb_to_top_edge,
                                mb_to_bottom_edge,
                            );
                            (*mbmi).mv.as_int = (*mi).bmi[15 as usize].mv.as_int;
                            (*mbmi).mode = SPLITMV as u8;
                            (*mbmi).is_4x4 = 1 as u8;
                        } else {
                            let mbmi_mv: *mut IntMv = &raw mut (*mbmi).mv;
                            read_mv(bc, &raw mut (*mbmi_mv).as_mv, mvc as *const MvContext);
                            (*mbmi_mv).as_mv.row = ((*mbmi_mv).as_mv.row as i32
                                + near_mvs[near_index as usize].as_mv.row as i32)
                                as i16;
                            (*mbmi_mv).as_mv.col = ((*mbmi_mv).as_mv.col as i32
                                + near_mvs[near_index as usize].as_mv.col as i32)
                                as i16;
                            (*mbmi).need_to_clamp_mvs = vp8_check_mv_bounds(
                                mbmi_mv,
                                mb_to_left_edge,
                                mb_to_right_edge,
                                mb_to_top_edge,
                                mb_to_bottom_edge,
                            ) as u8;
                            (*mbmi).mode = NEWMV as u8;
                        }
                    } else {
                        (*mbmi).mode = NEARMV as u8;
                        (*mbmi).mv.as_int = near_mvs[CNT_NEAR as usize].as_int;
                        vp8_clamp_mv2(&raw mut (*mbmi).mv, &raw mut (*pbi).mb);
                    }
                } else {
                    (*mbmi).mode = NEARESTMV as u8;
                    (*mbmi).mv.as_int = near_mvs[CNT_NEAREST as usize].as_int;
                    vp8_clamp_mv2(&raw mut (*mbmi).mv, &raw mut (*pbi).mb);
                }
            } else {
                (*mbmi).mode = ZEROMV as u8;
                (*mbmi).mv.as_int = 0 as u32;
            }
        } else {
            (*mbmi).mv.as_int = 0 as u32;
            (*mbmi).mode =
                read_ymode(bc, &raw mut (*pbi).common.fc.ymode_prob as *mut Vp8Prob) as u8;
            if (*mbmi).mode as i32 == B_PRED as i32 {
                let mut j: i32 = 0 as i32;
                (*mbmi).is_4x4 = 1 as u8;
                loop {
                    (*mi).bmi[j as usize].as_mode =
                        read_bmode(bc, &raw mut (*pbi).common.fc.bmode_prob as *mut Vp8Prob);
                    j += 1;
                    if !(j < 16 as i32) {
                        break;
                    }
                }
            }
            (*mbmi).uv_mode =
                read_uv_mode(bc, &raw mut (*pbi).common.fc.uv_mode_prob as *mut Vp8Prob) as u8;
        };
    }
}
unsafe fn read_mb_features(
    mut r: *mut Vp8Reader,
    mut mi: *mut MbModeInfo,
    mut x: *mut MACROBLOCKD,
) {
    unsafe {
        if (*x).segmentation_enabled as i32 != 0 && (*x).update_mb_segmentation_map as i32 != 0 {
            if vp8dx_decode_bool(
                r as *mut BoolDecoder,
                (*x).mb_segment_tree_probs[0 as usize] as i32,
            ) != 0
            {
                (*mi).segment_id = (2 as i32
                    + vp8dx_decode_bool(
                        r as *mut BoolDecoder,
                        (*x).mb_segment_tree_probs[2 as usize] as i32,
                    )) as u8;
            } else {
                (*mi).segment_id = vp8dx_decode_bool(
                    r as *mut BoolDecoder,
                    (*x).mb_segment_tree_probs[1 as usize] as i32,
                ) as u8;
            }
        }
    }
}
unsafe fn decode_mb_mode_mvs(mut pbi: *mut Vp8dComp, mut mi: *mut ModeInfo) {
    unsafe {
        if (*pbi).mb.update_mb_segmentation_map != 0 {
            read_mb_features(
                (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut Vp8Reader,
                &raw mut (*mi).mbmi,
                &raw mut (*pbi).mb,
            );
        } else if (*pbi).common.frame_type as u32 == KEY_FRAME as u32 {
            (*mi).mbmi.segment_id = 0 as u8;
        }
        if (*pbi).common.mb_no_coeff_skip != 0 {
            (*mi).mbmi.mb_skip_coeff = vp8dx_decode_bool(
                (&raw mut (*pbi).mbc as *mut Vp8Reader).offset(8 as isize) as *mut BoolDecoder,
                (*pbi).prob_skip_false as i32,
            ) as u8;
        } else {
            (*mi).mbmi.mb_skip_coeff = 0 as u8;
        }
        (*mi).mbmi.is_4x4 = 0 as u8;
        if (*pbi).common.frame_type as u32 == KEY_FRAME as u32 {
            read_kf_modes(pbi, mi);
        } else {
            read_mb_modes_mv(pbi, mi, &raw mut (*mi).mbmi);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decode_mode_mvs(mut pbi: *mut Vp8dComp) {
    unsafe {
        let mut mi: *mut ModeInfo = (*pbi).common.mi;
        let mut mb_row: i32 = -(1 as i32);
        let mut mb_to_right_edge_start: i32 = 0;
        mb_mode_mv_init(pbi);
        (*pbi).mb.mb_to_top_edge = 0 as i32;
        (*pbi).mb.mb_to_bottom_edge = (((*pbi).common.mb_rows - 1 as i32) * 16 as i32) << 3 as i32;
        mb_to_right_edge_start = (((*pbi).common.mb_cols - 1 as i32) * 16 as i32) << 3 as i32;
        loop {
            mb_row += 1;
            if !(mb_row < (*pbi).common.mb_rows) {
                break;
            }
            let mut mb_col: i32 = -(1 as i32);
            (*pbi).mb.mb_to_left_edge = 0 as i32;
            (*pbi).mb.mb_to_right_edge = mb_to_right_edge_start;
            loop {
                mb_col += 1;
                if !(mb_col < (*pbi).common.mb_cols) {
                    break;
                }
                decode_mb_mode_mvs(pbi, mi);
                (*pbi).mb.mb_to_left_edge -= (16 as i32) << 3 as i32;
                (*pbi).mb.mb_to_right_edge -= (16 as i32) << 3 as i32;
                mi = mi.offset(1);
            }
            (*pbi).mb.mb_to_top_edge -= (16 as i32) << 3 as i32;
            (*pbi).mb.mb_to_bottom_edge -= (16 as i32) << 3 as i32;
            mi = mi.offset(1);
        }
    }
}
