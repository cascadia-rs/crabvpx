use std::ffi::c_void;
unsafe extern "Rust" {
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
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
pub type Vp8Prob = u8;
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
pub type VpxColorRangeT = VpxColorRange;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type SizeT = DarwinSizeT;
pub type DarwinSizeT = usize;
pub type BLOCKD = Blockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterInfo {
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
}
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
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
pub type ClampType = u32;
pub const RECON_CLAMP_NOTREQUIRED: ClampType = 1;
pub const RECON_CLAMP_REQUIRED: ClampType = 0;
pub type Vp8Common = VP8Common;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const ZEROMV: C2RustUnnamed = 7;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub type MACROBLOCKD = Macroblockd;
pub const MAX_REF_FRAMES: C2RustUnnamed_1 = 4;
pub const INTRA_FRAME: C2RustUnnamed_1 = 0;
pub const MB_LVL_ALT_LF: C2RustUnnamed_0 = 1;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_0 = u32;
pub const MB_LVL_MAX: C2RustUnnamed_0 = 2;
pub const MB_LVL_ALT_Q: C2RustUnnamed_0 = 0;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_1 = u32;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const MAX_LOOP_FILTER: i32 = 63 as i32;
pub const PARTIAL_FRAME_FRACTION: i32 = 8 as i32;
pub const SIMD_WIDTH: i32 = 16 as i32;
pub const MAX_MB_SEGMENTS: i32 = 4 as i32;
pub const SEGMENT_ABSDATA: i32 = 1 as i32;
unsafe fn lf_init_lut(mut lfi: *mut LoopFilterInfoN) {
    unsafe {
        let mut filt_lvl: i32 = 0;
        filt_lvl = 0 as i32;
        while filt_lvl <= MAX_LOOP_FILTER {
            if filt_lvl >= 40 as i32 {
                (*lfi).hev_thr_lut[KEY_FRAME as usize][filt_lvl as usize] = 2 as u8;
                (*lfi).hev_thr_lut[INTER_FRAME as usize][filt_lvl as usize] = 3 as u8;
            } else if filt_lvl >= 20 as i32 {
                (*lfi).hev_thr_lut[KEY_FRAME as usize][filt_lvl as usize] = 1 as u8;
                (*lfi).hev_thr_lut[INTER_FRAME as usize][filt_lvl as usize] = 2 as u8;
            } else if filt_lvl >= 15 as i32 {
                (*lfi).hev_thr_lut[KEY_FRAME as usize][filt_lvl as usize] = 1 as u8;
                (*lfi).hev_thr_lut[INTER_FRAME as usize][filt_lvl as usize] = 1 as u8;
            } else {
                (*lfi).hev_thr_lut[KEY_FRAME as usize][filt_lvl as usize] = 0 as u8;
                (*lfi).hev_thr_lut[INTER_FRAME as usize][filt_lvl as usize] = 0 as u8;
            }
            filt_lvl += 1;
        }
        (*lfi).mode_lf_lut[DC_PRED as usize] = 1 as u8;
        (*lfi).mode_lf_lut[V_PRED as usize] = 1 as u8;
        (*lfi).mode_lf_lut[H_PRED as usize] = 1 as u8;
        (*lfi).mode_lf_lut[TM_PRED as usize] = 1 as u8;
        (*lfi).mode_lf_lut[B_PRED as usize] = 0 as u8;
        (*lfi).mode_lf_lut[ZEROMV as usize] = 1 as u8;
        (*lfi).mode_lf_lut[NEARESTMV as usize] = 2 as u8;
        (*lfi).mode_lf_lut[NEARMV as usize] = 2 as u8;
        (*lfi).mode_lf_lut[NEWMV as usize] = 2 as u8;
        (*lfi).mode_lf_lut[SPLITMV as usize] = 3 as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_update_sharpness(
    mut lfi: *mut LoopFilterInfoN,
    mut sharpness_lvl: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i <= MAX_LOOP_FILTER {
            let mut filt_lvl: i32 = i;
            let mut block_inside_limit: i32 = 0 as i32;
            block_inside_limit = filt_lvl >> (sharpness_lvl > 0 as i32) as i32;
            block_inside_limit >>= (sharpness_lvl > 4 as i32) as i32;
            if sharpness_lvl > 0 as i32 && block_inside_limit > 9 as i32 - sharpness_lvl {
                block_inside_limit = 9 as i32 - sharpness_lvl;
            }
            if block_inside_limit < 1 as i32 {
                block_inside_limit = 1 as i32;
            }
            core::ptr::write_bytes(
                &raw mut *(&raw mut (*lfi).lim as *mut [u8; 16]).offset(i as isize) as *mut u8
                    as *mut c_void as *mut u8,
                block_inside_limit as u8,
                SIMD_WIDTH as SizeT,
            );
            core::ptr::write_bytes(
                &raw mut *(&raw mut (*lfi).blim as *mut [u8; 16]).offset(i as isize) as *mut u8
                    as *mut c_void as *mut u8,
                (2 as i32 * filt_lvl + block_inside_limit) as u8,
                SIMD_WIDTH as SizeT,
            );
            core::ptr::write_bytes(
                &raw mut *(&raw mut (*lfi).mblim as *mut [u8; 16]).offset(i as isize) as *mut u8
                    as *mut c_void as *mut u8,
                (2 as i32 * (filt_lvl + 2 as i32) + block_inside_limit) as u8,
                SIMD_WIDTH as SizeT,
            );
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_init(mut cm: *mut Vp8Common) {
    unsafe {
        let mut lfi: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        let mut i: i32 = 0;
        vp8_loop_filter_update_sharpness(lfi, (*cm).sharpness_level);
        (*cm).last_sharpness_level = (*cm).sharpness_level;
        lf_init_lut(lfi);
        i = 0 as i32;
        while i < 4 as i32 {
            core::ptr::write_bytes(
                &raw mut *(&raw mut (*lfi).hev_thr as *mut [u8; 16]).offset(i as isize) as *mut u8
                    as *mut c_void as *mut u8,
                i as u8,
                SIMD_WIDTH as SizeT,
            );
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_frame_init(
    mut cm: *mut Vp8Common,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: i32,
) {
    unsafe {
        let mut seg: i32 = 0;
        let mut ref_0: i32 = 0;
        let mut mode: i32 = 0;
        let mut lfi: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        if (*cm).last_sharpness_level != (*cm).sharpness_level {
            vp8_loop_filter_update_sharpness(lfi, (*cm).sharpness_level);
            (*cm).last_sharpness_level = (*cm).sharpness_level;
        }
        seg = 0 as i32;
        while seg < MAX_MB_SEGMENTS {
            let mut lvl_seg: i32 = default_filt_lvl;
            let mut lvl_ref: i32 = 0;
            let mut lvl_mode: i32 = 0;
            if (*mbd).segmentation_enabled != 0 {
                if (*mbd).mb_segment_abs_delta as i32 == SEGMENT_ABSDATA {
                    lvl_seg =
                        (*mbd).segment_feature_data[MB_LVL_ALT_LF as usize][seg as usize] as i32;
                } else {
                    lvl_seg +=
                        (*mbd).segment_feature_data[MB_LVL_ALT_LF as usize][seg as usize] as i32;
                }
                lvl_seg = if lvl_seg > 0 as i32 {
                    if lvl_seg > 63 as i32 {
                        63 as i32
                    } else {
                        lvl_seg
                    }
                } else {
                    0 as i32
                };
            }
            if (*mbd).mode_ref_lf_delta_enabled == 0 {
                core::ptr::write_bytes(
                    &raw mut *(&raw mut *(&raw mut (*lfi).lvl as *mut [[u8; 4]; 4])
                        .offset(seg as isize) as *mut [u8; 4])
                        .offset(0 as isize) as *mut u8 as *mut c_void
                        as *mut u8,
                    lvl_seg as u8,
                    (4 as i32 * 4 as i32) as SizeT,
                );
            } else {
                ref_0 = INTRA_FRAME as i32;
                lvl_ref = lvl_seg + (*mbd).ref_lf_deltas[ref_0 as usize] as i32;
                mode = 0 as i32;
                lvl_mode = lvl_ref + (*mbd).mode_lf_deltas[mode as usize] as i32;
                lvl_mode = if lvl_mode > 0 as i32 {
                    if lvl_mode > 63 as i32 {
                        63 as i32
                    } else {
                        lvl_mode
                    }
                } else {
                    0 as i32
                };
                (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
                mode = 1 as i32;
                lvl_mode = if lvl_ref > 0 as i32 {
                    if lvl_ref > 63 as i32 {
                        63 as i32
                    } else {
                        lvl_ref
                    }
                } else {
                    0 as i32
                };
                (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
                ref_0 = 1 as i32;
                while ref_0 < MAX_REF_FRAMES as i32 {
                    lvl_ref = lvl_seg + (*mbd).ref_lf_deltas[ref_0 as usize] as i32;
                    mode = 1 as i32;
                    while mode < 4 as i32 {
                        lvl_mode = lvl_ref + (*mbd).mode_lf_deltas[mode as usize] as i32;
                        lvl_mode = if lvl_mode > 0 as i32 {
                            if lvl_mode > 63 as i32 {
                                63 as i32
                            } else {
                                lvl_mode
                            }
                        } else {
                            0 as i32
                        };
                        (*lfi).lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
                        mode += 1;
                    }
                    ref_0 += 1;
                }
            }
            seg += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_row_normal(
    mut cm: *mut Vp8Common,
    mut mode_info_context: *mut ModeInfo,
    mut mb_row: i32,
    mut post_ystride: i32,
    mut post_uvstride: i32,
    mut y_ptr: *mut u8,
    mut u_ptr: *mut u8,
    mut v_ptr: *mut u8,
) {
    unsafe {
        let mut mb_col: i32 = 0;
        let mut filter_level: i32 = 0;
        let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        let mut lfi: LoopFilterInfo = LoopFilterInfo {
            mblim: ::core::ptr::null::<u8>(),
            blim: ::core::ptr::null::<u8>(),
            lim: ::core::ptr::null::<u8>(),
            hev_thr: ::core::ptr::null::<u8>(),
        };
        let mut frame_type: FrameType = (*cm).frame_type;
        mb_col = 0 as i32;
        while mb_col < (*cm).mb_cols {
            let mut skip_lf: i32 = ((*mode_info_context).mbmi.mode as i32 != B_PRED as i32
                && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                as i32;
            let mode_index: i32 =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
            let seg: i32 = (*mode_info_context).mbmi.segment_id as i32;
            let ref_frame: i32 = (*mode_info_context).mbmi.ref_frame as i32;
            filter_level =
                (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
            if filter_level != 0 {
                let hev_index: i32 =
                    (*lfi_n).hev_thr_lut[frame_type as usize][filter_level as usize] as i32;
                lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                    .offset(filter_level as isize) as *mut u8;
                lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                    .offset(filter_level as isize) as *mut u8;
                lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [u8; 16])
                    .offset(filter_level as isize) as *mut u8;
                lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr as *mut [u8; 16])
                    .offset(hev_index as isize) as *mut u8;
                if mb_col > 0 as i32 {
                    vp8_loop_filter_mbv_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bv_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if mb_row > 0 as i32 {
                    vp8_loop_filter_mbh_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bh_c(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
            }
            y_ptr = y_ptr.offset(16 as isize);
            u_ptr = u_ptr.offset(8 as isize);
            v_ptr = v_ptr.offset(8 as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_col += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_row_simple(
    mut cm: *mut Vp8Common,
    mut mode_info_context: *mut ModeInfo,
    mut mb_row: i32,
    mut post_ystride: i32,
    mut y_ptr: *mut u8,
) {
    unsafe {
        let mut mb_col: i32 = 0;
        let mut filter_level: i32 = 0;
        let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        mb_col = 0 as i32;
        while mb_col < (*cm).mb_cols {
            let mut skip_lf: i32 = ((*mode_info_context).mbmi.mode as i32 != B_PRED as i32
                && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                as i32;
            let mode_index: i32 =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
            let seg: i32 = (*mode_info_context).mbmi.segment_id as i32;
            let ref_frame: i32 = (*mode_info_context).mbmi.ref_frame as i32;
            filter_level =
                (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
            if filter_level != 0 {
                if mb_col > 0 as i32 {
                    vp8_loop_filter_simple_vertical_edge_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                            .offset(filter_level as isize) as *mut u8,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bvs_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                            .offset(filter_level as isize) as *mut u8,
                    );
                }
                if mb_row > 0 as i32 {
                    vp8_loop_filter_simple_horizontal_edge_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                            .offset(filter_level as isize) as *mut u8,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bhs_c(
                        y_ptr,
                        post_ystride,
                        &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                            .offset(filter_level as isize) as *mut u8,
                    );
                }
            }
            y_ptr = y_ptr.offset(16 as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_col += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_frame(
    mut cm: *mut Vp8Common,
    mut mbd: *mut MACROBLOCKD,
    mut frame_type: i32,
) {
    unsafe {
        let mut post: *mut Yv12BufferConfig = (*cm).frame_to_show;
        let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        let mut lfi: LoopFilterInfo = LoopFilterInfo {
            mblim: ::core::ptr::null::<u8>(),
            blim: ::core::ptr::null::<u8>(),
            lim: ::core::ptr::null::<u8>(),
            hev_thr: ::core::ptr::null::<u8>(),
        };
        let mut mb_row: i32 = 0;
        let mut mb_col: i32 = 0;
        let mut mb_rows: i32 = (*cm).mb_rows;
        let mut mb_cols: i32 = (*cm).mb_cols;
        let mut filter_level: i32 = 0;
        let mut y_ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut u_ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut v_ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut mode_info_context: *const ModeInfo = (*cm).mi;
        let mut post_y_stride: i32 = (*post).y_stride;
        let mut post_uv_stride: i32 = (*post).uv_stride;
        vp8_loop_filter_frame_init(cm, mbd, (*cm).filter_level);
        y_ptr = (*post).y_buffer as *mut u8;
        u_ptr = (*post).u_buffer as *mut u8;
        v_ptr = (*post).v_buffer as *mut u8;
        if (*cm).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
            mb_row = 0 as i32;
            while mb_row < mb_rows {
                mb_col = 0 as i32;
                while mb_col < mb_cols {
                    let mut skip_lf: i32 = ((*mode_info_context).mbmi.mode as i32 != B_PRED as i32
                        && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                        && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                        as i32;
                    let mode_index: i32 =
                        (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
                    let seg: i32 = (*mode_info_context).mbmi.segment_id as i32;
                    let ref_frame: i32 = (*mode_info_context).mbmi.ref_frame as i32;
                    filter_level =
                        (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
                    if filter_level != 0 {
                        let hev_index: i32 =
                            (*lfi_n).hev_thr_lut[frame_type as usize][filter_level as usize] as i32;
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
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if mb_row > 0 as i32 {
                            vp8_loop_filter_mbh_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                u_ptr,
                                v_ptr,
                                post_y_stride,
                                post_uv_stride,
                                &raw mut lfi,
                            );
                        }
                    }
                    y_ptr = y_ptr.offset(16 as isize);
                    u_ptr = u_ptr.offset(8 as isize);
                    v_ptr = v_ptr.offset(8 as isize);
                    mode_info_context = mode_info_context.offset(1);
                    mb_col += 1;
                }
                y_ptr = y_ptr.offset((post_y_stride * 16 as i32 - (*post).y_width) as isize);
                u_ptr = u_ptr.offset((post_uv_stride * 8 as i32 - (*post).uv_width) as isize);
                v_ptr = v_ptr.offset((post_uv_stride * 8 as i32 - (*post).uv_width) as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_row += 1;
            }
        } else {
            mb_row = 0 as i32;
            while mb_row < mb_rows {
                mb_col = 0 as i32;
                while mb_col < mb_cols {
                    let mut skip_lf_0: i32 = ((*mode_info_context).mbmi.mode as i32
                        != B_PRED as i32
                        && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                        && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                        as i32;
                    let mode_index_0: i32 =
                        (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
                    let seg_0: i32 = (*mode_info_context).mbmi.segment_id as i32;
                    let ref_frame_0: i32 = (*mode_info_context).mbmi.ref_frame as i32;
                    filter_level = (*lfi_n).lvl[seg_0 as usize][ref_frame_0 as usize]
                        [mode_index_0 as usize] as i32;
                    if filter_level != 0 {
                        let mut mblim: *const u8 = &raw mut *(&raw mut (*lfi_n).mblim
                            as *mut [u8; 16])
                            .offset(filter_level as isize)
                            as *mut u8;
                        let mut blim: *const u8 = &raw mut *(&raw mut (*lfi_n).blim
                            as *mut [u8; 16])
                            .offset(filter_level as isize)
                            as *mut u8;
                        if mb_col > 0 as i32 {
                            vp8_loop_filter_simple_vertical_edge_c(y_ptr, post_y_stride, mblim);
                        }
                        if skip_lf_0 == 0 {
                            vp8_loop_filter_bvs_c(y_ptr, post_y_stride, blim);
                        }
                        if mb_row > 0 as i32 {
                            vp8_loop_filter_simple_horizontal_edge_c(y_ptr, post_y_stride, mblim);
                        }
                        if skip_lf_0 == 0 {
                            vp8_loop_filter_bhs_c(y_ptr, post_y_stride, blim);
                        }
                    }
                    y_ptr = y_ptr.offset(16 as isize);
                    u_ptr = u_ptr.offset(8 as isize);
                    v_ptr = v_ptr.offset(8 as isize);
                    mode_info_context = mode_info_context.offset(1);
                    mb_col += 1;
                }
                y_ptr = y_ptr.offset((post_y_stride * 16 as i32 - (*post).y_width) as isize);
                u_ptr = u_ptr.offset((post_uv_stride * 8 as i32 - (*post).uv_width) as isize);
                v_ptr = v_ptr.offset((post_uv_stride * 8 as i32 - (*post).uv_width) as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_row += 1;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_frame_yonly(
    mut cm: *mut Vp8Common,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: i32,
) {
    unsafe {
        let mut post: *mut Yv12BufferConfig = (*cm).frame_to_show;
        let mut y_ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut mb_row: i32 = 0;
        let mut mb_col: i32 = 0;
        let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        let mut lfi: LoopFilterInfo = LoopFilterInfo {
            mblim: ::core::ptr::null::<u8>(),
            blim: ::core::ptr::null::<u8>(),
            lim: ::core::ptr::null::<u8>(),
            hev_thr: ::core::ptr::null::<u8>(),
        };
        let mut filter_level: i32 = 0;
        let mut frame_type: FrameType = (*cm).frame_type;
        let mut mode_info_context: *const ModeInfo = (*cm).mi;
        vp8_loop_filter_frame_init(cm, mbd, default_filt_lvl);
        y_ptr = (*post).y_buffer as *mut u8;
        mb_row = 0 as i32;
        while mb_row < (*cm).mb_rows {
            mb_col = 0 as i32;
            while mb_col < (*cm).mb_cols {
                let mut skip_lf: i32 = ((*mode_info_context).mbmi.mode as i32 != B_PRED as i32
                    && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                    && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                    as i32;
                let mode_index: i32 =
                    (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
                let seg: i32 = (*mode_info_context).mbmi.segment_id as i32;
                let ref_frame: i32 = (*mode_info_context).mbmi.ref_frame as i32;
                filter_level =
                    (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
                if filter_level != 0 {
                    if (*cm).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
                        let hev_index: i32 =
                            (*lfi_n).hev_thr_lut[frame_type as usize][filter_level as usize] as i32;
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
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                        if mb_row > 0 as i32 {
                            vp8_loop_filter_mbh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                    } else {
                        if mb_col > 0 as i32 {
                            vp8_loop_filter_simple_vertical_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bvs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                        if mb_row > 0 as i32 {
                            vp8_loop_filter_simple_horizontal_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bhs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                    }
                }
                y_ptr = y_ptr.offset(16 as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_col += 1;
            }
            y_ptr = y_ptr.offset(((*post).y_stride * 16 as i32 - (*post).y_width) as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_row += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_loop_filter_partial_frame(
    mut cm: *mut Vp8Common,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: i32,
) {
    unsafe {
        let mut post: *mut Yv12BufferConfig = (*cm).frame_to_show;
        let mut y_ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut mb_row: i32 = 0;
        let mut mb_col: i32 = 0;
        let mut mb_cols: i32 = (*post).y_width >> 4 as i32;
        let mut mb_rows: i32 = (*post).y_height >> 4 as i32;
        let mut linestocopy: i32 = 0;
        let mut lfi_n: *mut LoopFilterInfoN = &raw mut (*cm).lf_info;
        let mut lfi: LoopFilterInfo = LoopFilterInfo {
            mblim: ::core::ptr::null::<u8>(),
            blim: ::core::ptr::null::<u8>(),
            lim: ::core::ptr::null::<u8>(),
            hev_thr: ::core::ptr::null::<u8>(),
        };
        let mut filter_level: i32 = 0;
        let mut frame_type: FrameType = (*cm).frame_type;
        let mut mode_info_context: *const ModeInfo = ::core::ptr::null::<ModeInfo>();
        vp8_loop_filter_frame_init(cm, mbd, default_filt_lvl);
        linestocopy = mb_rows / PARTIAL_FRAME_FRACTION;
        linestocopy = if linestocopy != 0 {
            linestocopy << 4 as i32
        } else {
            16 as i32
        };
        y_ptr = (*post)
            .y_buffer
            .offset((((*post).y_height >> 5 as i32) * 16 as i32 * (*post).y_stride) as isize)
            as *mut u8;
        mode_info_context = (*cm)
            .mi
            .offset((((*post).y_height >> 5 as i32) * (mb_cols + 1 as i32)) as isize);
        mb_row = 0 as i32;
        while mb_row < linestocopy >> 4 as i32 {
            mb_col = 0 as i32;
            while mb_col < mb_cols {
                let mut skip_lf: i32 = ((*mode_info_context).mbmi.mode as i32 != B_PRED as i32
                    && (*mode_info_context).mbmi.mode as i32 != SPLITMV as i32
                    && (*mode_info_context).mbmi.mb_skip_coeff as i32 != 0)
                    as i32;
                let mode_index: i32 =
                    (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as i32;
                let seg: i32 = (*mode_info_context).mbmi.segment_id as i32;
                let ref_frame: i32 = (*mode_info_context).mbmi.ref_frame as i32;
                filter_level =
                    (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
                if filter_level != 0 {
                    if (*cm).filter_type as u32 == NORMAL_LOOPFILTER as u32 {
                        let hev_index: i32 =
                            (*lfi_n).hev_thr_lut[frame_type as usize][filter_level as usize] as i32;
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
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_c(
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                        vp8_loop_filter_mbh_c(
                            y_ptr,
                            ::core::ptr::null_mut::<u8>(),
                            ::core::ptr::null_mut::<u8>(),
                            (*post).y_stride,
                            0 as i32,
                            &raw mut lfi,
                        );
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_c(
                                y_ptr,
                                ::core::ptr::null_mut::<u8>(),
                                ::core::ptr::null_mut::<u8>(),
                                (*post).y_stride,
                                0 as i32,
                                &raw mut lfi,
                            );
                        }
                    } else {
                        if mb_col > 0 as i32 {
                            vp8_loop_filter_simple_vertical_edge_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bvs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                        vp8_loop_filter_simple_horizontal_edge_c(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [u8; 16])
                                .offset(filter_level as isize)
                                as *mut u8,
                        );
                        if skip_lf == 0 {
                            vp8_loop_filter_bhs_c(
                                y_ptr,
                                (*post).y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [u8; 16])
                                    .offset(filter_level as isize)
                                    as *mut u8,
                            );
                        }
                    }
                }
                y_ptr = y_ptr.offset(16 as isize);
                mode_info_context = mode_info_context.offset(1 as isize);
                mb_col += 1;
            }
            y_ptr = y_ptr.offset(((*post).y_stride * 16 as i32 - (*post).y_width) as isize);
            mode_info_context = mode_info_context.offset(1 as isize);
            mb_row += 1;
        }
    }
}
