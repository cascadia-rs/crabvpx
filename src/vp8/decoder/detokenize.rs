use crate::vpx_dsp::tables::VPX_NORM as vp8_norm;
use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8dx_bool_decoder_fill(br: *mut BoolDecoder);
}
pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};
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
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
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
pub union BModeInfo {
    pub as_mode: u32,
    pub mv: IntMv,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
pub type ModeInfo = Modeinfo;
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
pub type BLOCKD = Blockd;
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
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
pub type MACROBLOCKD = Macroblockd;
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
pub type ProbaArray = *const [[u8; 11]; 3];
pub const CHAR_BIT: i32 = 8 as i32;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<usize>() as i32 * CHAR_BIT;
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
#[unsafe(no_mangle)]
pub unsafe fn vp8_reset_mb_tokens_context(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut a_ctx: *mut i8 = (*x).above_context as *mut i8;
        let mut l_ctx: *mut i8 = (*x).left_context as *mut i8;
        core::ptr::write_bytes(
            a_ctx as *mut c_void as *mut u8,
            0 as u8,
            (::core::mem::size_of::<EntropyContextPlanes>() as usize).wrapping_sub(1 as usize),
        );
        core::ptr::write_bytes(
            l_ctx as *mut c_void as *mut u8,
            0 as u8,
            (::core::mem::size_of::<EntropyContextPlanes>() as usize).wrapping_sub(1 as usize),
        );
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            let fresh0 = &mut *l_ctx.offset(8 as isize);
            *fresh0 = 0 as i8;
            *a_ctx.offset(8 as isize) = *fresh0;
        }
    }
}
static kBands: [u8; 17] = [
    0 as u8, 1 as u8, 2 as u8, 3 as u8, 6 as u8, 4 as u8, 5 as u8, 6 as u8, 6 as u8, 6 as u8,
    6 as u8, 6 as u8, 6 as u8, 6 as u8, 6 as u8, 7 as u8, 0 as u8,
];
static kCat3: [u8; 4] = [173 as u8, 148 as u8, 140 as u8, 0 as u8];
static kCat4: [u8; 5] = [176 as u8, 155 as u8, 140 as u8, 135 as u8, 0 as u8];
static kCat5: [u8; 6] = [
    180 as u8, 157 as u8, 141 as u8, 134 as u8, 130 as u8, 0 as u8,
];
static kCat6: [u8; 12] = [
    254 as u8, 254 as u8, 243 as u8, 230 as u8, 196 as u8, 177 as u8, 153 as u8, 140 as u8,
    133 as u8, 130 as u8, 129 as u8, 0 as u8,
];
struct SyncPtr(*const u8);
unsafe impl Sync for SyncPtr {}
static kCat3456: [SyncPtr; 4] = {
    [
        SyncPtr(&raw const kCat3 as *const u8),
        SyncPtr(&raw const kCat4 as *const u8),
        SyncPtr(&raw const kCat5 as *const u8),
        SyncPtr(&raw const kCat6 as *const u8),
    ]
};
static kZigzag: [u8; 16] = [
    0 as u8, 1 as u8, 4 as u8, 8 as u8, 5 as u8, 2 as u8, 3 as u8, 6 as u8, 9 as u8, 12 as u8,
    13 as u8, 10 as u8, 7 as u8, 11 as u8, 14 as u8, 15 as u8,
];
unsafe fn get_signed(mut br: *mut BoolDecoder, mut value_to_sign: i32) -> i32 {
    unsafe {
        let mut split: i32 = ((*br).range.wrapping_add(1 as u32) >> 1 as i32) as i32;
        let mut bigsplit: usize = (split as usize) << (VP8_BD_VALUE_SIZE - 8 as i32);
        let mut v: i32 = 0;
        if (*br).count < 0 as i32 {
            vp8dx_bool_decoder_fill(br);
        }
        if (*br).value < bigsplit {
            (*br).range = split as u32;
            v = value_to_sign;
        } else {
            (*br).range = (*br).range.wrapping_sub(split as u32);
            (*br).value = (*br).value.wrapping_sub(bigsplit);
            v = -value_to_sign;
        }
        (*br).range = (*br).range.wrapping_add((*br).range);
        (*br).value = (*br).value.wrapping_add((*br).value);
        (*br).count -= 1;
        v
    }
}
unsafe fn get_coeffs(
    mut br: *mut BoolDecoder,
    mut prob: ProbaArray,
    mut ctx: i32,
    mut n: i32,
    mut out: *mut i16,
) -> i32 {
    unsafe {
        let mut p: *const u8 = &raw const *(&raw const *prob.offset(n as isize) as *const [u8; 11])
            .offset(ctx as isize) as *const u8;
        if vp8dx_decode_bool(br, *p.offset(0 as isize) as i32) == 0 {
            return 0 as i32;
        }
        loop {
            n += 1;
            if vp8dx_decode_bool(br, *p.offset(1 as isize) as i32) == 0 {
                p = &raw const *(&raw const *prob
                    .offset(*(&raw const kBands as *const u8).offset(n as isize) as isize)
                    as *const [u8; 11])
                    .offset(0 as isize) as *const u8;
            } else {
                let mut v: i32 = 0;
                let mut j: i32 = 0;
                if vp8dx_decode_bool(br, *p.offset(2 as isize) as i32) == 0 {
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const u8).offset(n as isize) as isize)
                        as *const [u8; 11])
                        .offset(1 as isize) as *const u8;
                    v = 1 as i32;
                } else {
                    if vp8dx_decode_bool(br, *p.offset(3 as isize) as i32) == 0 {
                        if vp8dx_decode_bool(br, *p.offset(4 as isize) as i32) == 0 {
                            v = 2 as i32;
                        } else {
                            v = 3 as i32 + vp8dx_decode_bool(br, *p.offset(5 as isize) as i32);
                        }
                    } else if vp8dx_decode_bool(br, *p.offset(6 as isize) as i32) == 0 {
                        if vp8dx_decode_bool(br, *p.offset(7 as isize) as i32) == 0 {
                            v = 5 as i32 + vp8dx_decode_bool(br, 159 as i32);
                        } else {
                            v = 7 as i32 + 2 as i32 * vp8dx_decode_bool(br, 165 as i32);
                            v += vp8dx_decode_bool(br, 145 as i32);
                        }
                    } else {
                        let mut tab: *const u8 = ::core::ptr::null::<u8>();
                        let bit1: i32 = vp8dx_decode_bool(br, *p.offset(8 as isize) as i32) as i32;
                        let bit0: i32 =
                            vp8dx_decode_bool(br, *p.offset((9 as i32 + bit1) as isize) as i32)
                                as i32;
                        let cat: i32 = 2 as i32 * bit1 + bit0;
                        v = 0 as i32;
                        tab = kCat3456[cat as usize].0;
                        while *tab != 0 {
                            v += v + vp8dx_decode_bool(br, *tab as i32);
                            tab = tab.offset(1);
                        }
                        v += 3 as i32 + ((8 as i32) << cat);
                    }
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const u8).offset(n as isize) as isize)
                        as *const [u8; 11])
                        .offset(2 as isize) as *const u8;
                }
                j = kZigzag[(n - 1 as i32) as usize] as i32;
                *out.offset(j as isize) = get_signed(br, v) as i16;
                if n == 16 as i32 || vp8dx_decode_bool(br, *p.offset(0 as isize) as i32) == 0 {
                    return n;
                }
            }
            if n == 16 as i32 {
                return 16 as i32;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decode_mb_tokens(mut dx: *mut Vp8dComp, mut x: *mut MACROBLOCKD) -> i32 {
    unsafe {
        let mut bc: *mut BoolDecoder = (*x).current_bc as *mut BoolDecoder;
        let fc: *const FrameContext = &raw mut (*dx).common.fc;
        let mut eobs: *mut i8 = &raw mut (*x).eobs as *mut i8;
        let mut i: i32 = 0;
        let mut nonzeros: i32 = 0;
        let mut eobtotal: i32 = 0 as i32;
        let mut qcoeff_ptr: *mut i16 = ::core::ptr::null_mut::<i16>();
        let mut coef_probs: ProbaArray = ::core::ptr::null::<[[u8; 11]; 3]>();
        let mut a_ctx: *mut i8 = (*x).above_context as *mut i8;
        let mut l_ctx: *mut i8 = (*x).left_context as *mut i8;
        let mut a: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut l: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut skip_dc: i32 = 0 as i32;
        qcoeff_ptr = (&raw mut (*x).qcoeff as *mut i16).offset(0 as isize) as *mut i16;
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            a = a_ctx.offset(8 as isize);
            l = l_ctx.offset(8 as isize);
            coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[u8; 11]; 3]; 8])
                .offset(1 as isize) as *const [[u8; 11]; 3] as ProbaArray;
            nonzeros = get_coeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                0 as i32,
                qcoeff_ptr.offset((24 as i32 * 16 as i32) as isize),
            );
            *l = (nonzeros > 0 as i32) as i8;
            *a = *l;
            *eobs.offset(24 as isize) = nonzeros as i8;
            eobtotal += nonzeros - 16 as i32;
            coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[u8; 11]; 3]; 8])
                .offset(0 as isize) as *const [[u8; 11]; 3] as ProbaArray;
            skip_dc = 1 as i32;
        } else {
            coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[u8; 11]; 3]; 8])
                .offset(3 as isize) as *const [[u8; 11]; 3] as ProbaArray;
            skip_dc = 0 as i32;
        }
        i = 0 as i32;
        while i < 16 as i32 {
            a = a_ctx.offset((i & 3 as i32) as isize);
            l = l_ctx.offset(((i & 0xc as i32) >> 2 as i32) as isize);
            nonzeros = get_coeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                skip_dc,
                qcoeff_ptr as *mut i16,
            );
            *l = (nonzeros > 0 as i32) as i8;
            *a = *l;
            nonzeros += skip_dc;
            *eobs.offset(i as isize) = nonzeros as i8;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as isize);
            i += 1;
        }
        coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[u8; 11]; 3]; 8])
            .offset(2 as isize) as *const [[u8; 11]; 3] as ProbaArray;
        a_ctx = a_ctx.offset(4 as isize);
        l_ctx = l_ctx.offset(4 as isize);
        i = 16 as i32;
        while i < 24 as i32 {
            a = a_ctx
                .offset((((i > 19 as i32) as i32) << 1 as i32) as isize)
                .offset((i & 1 as i32) as isize);
            l = l_ctx
                .offset((((i > 19 as i32) as i32) << 1 as i32) as isize)
                .offset((i & 3 as i32 > 1 as i32) as isize);
            nonzeros = get_coeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                0 as i32,
                qcoeff_ptr as *mut i16,
            );
            *l = (nonzeros > 0 as i32) as i8;
            *a = *l;
            *eobs.offset(i as isize) = nonzeros as i8;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as isize);
            i += 1;
        }
        eobtotal
    }
}
