use std::ffi::c_void;
unsafe extern "Rust" {
    static vp8_mode_contexts: [[i32; 4]; 6];
}
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
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
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
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
pub type Vp8Prob = u8;
pub type EntropyContext = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const ZEROMV: C2RustUnnamed = 7;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
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
pub union BModeInfo {
    pub as_mode: BPredictionMode,
    pub mv: IntMv,
}
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_0 = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed_0 = 4;
pub const ALTREF_FRAME: C2RustUnnamed_0 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_0 = 2;
pub const LAST_FRAME: C2RustUnnamed_0 = 1;
pub const INTRA_FRAME: C2RustUnnamed_0 = 0;
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
pub type MACROBLOCKD = Macroblockd;
pub const CNT_NEAR: C2RustUnnamed_1 = 2;
pub const CNT_NEAREST: C2RustUnnamed_1 = 1;
pub const CNT_INTRA: C2RustUnnamed_1 = 0;
pub const CNT_SPLITMV: C2RustUnnamed_1 = 3;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed_1 = u32;
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
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_offset: [[u8; 16]; 4] = [
    [
        0 as u8, 8 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
        0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    ],
    [
        0 as u8, 2 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
        0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    ],
    [
        0 as u8, 2 as u8, 8 as u8, 10 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
        0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    ],
    [
        0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8,
        10 as u8, 11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8,
    ],
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_find_near_mvs(
    mut xd: *mut MACROBLOCKD,
    mut here: *const ModeInfo,
    mut nearest: *mut IntMv,
    mut nearby: *mut IntMv,
    mut best_mv: *mut IntMv,
    mut near_mv_ref_cnts: *mut i32,
    mut refframe: i32,
    mut ref_frame_sign_bias: *mut i32,
) {
    unsafe {
        let mut above: *const ModeInfo = here.offset(-((*xd).mode_info_stride as isize));
        let mut left: *const ModeInfo = here.offset(-(1 as isize));
        let mut aboveleft: *const ModeInfo = above.offset(-(1 as isize));
        let mut near_mvs: [IntMv; 4] = [IntMv { as_int: 0 }; 4];
        let mut mv: *mut IntMv = &raw mut near_mvs as *mut IntMv;
        let mut cntx: *mut i32 = near_mv_ref_cnts as *mut i32;
        let fresh0 = &mut (*mv.offset(2 as isize)).as_int;
        *fresh0 = 0 as u32;
        let fresh1 = &mut (*mv.offset(1 as isize)).as_int;
        *fresh1 = *fresh0;
        (*mv.offset(0 as isize)).as_int = *fresh1;
        let fresh2 = &mut *near_mv_ref_cnts.offset(3 as isize);
        *fresh2 = 0 as i32;
        let fresh3 = &mut *near_mv_ref_cnts.offset(2 as isize);
        *fresh3 = *fresh2;
        let fresh4 = &mut *near_mv_ref_cnts.offset(1 as isize);
        *fresh4 = *fresh3;
        *near_mv_ref_cnts.offset(0 as isize) = *fresh4;
        if (*above).mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
            if (*above).mbmi.mv.as_int != 0 {
                mv = mv.offset(1);
                (*mv).as_int = (*above).mbmi.mv.as_int;
                mv_bias(
                    *ref_frame_sign_bias.offset((*above).mbmi.ref_frame as isize),
                    refframe,
                    mv,
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
                    refframe,
                    &raw mut this_mv,
                    ref_frame_sign_bias,
                );
                if this_mv.as_int != (*mv).as_int {
                    mv = mv.offset(1);
                    (*mv).as_int = this_mv.as_int;
                    cntx = cntx.offset(1);
                }
                *cntx += 2 as i32;
            } else {
                *near_mv_ref_cnts.offset(CNT_INTRA as isize) += 2 as i32;
            }
        }
        if (*aboveleft).mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
            if (*aboveleft).mbmi.mv.as_int != 0 {
                let mut this_mv_0: IntMv = IntMv { as_int: 0 };
                this_mv_0.as_int = (*aboveleft).mbmi.mv.as_int;
                mv_bias(
                    *ref_frame_sign_bias.offset((*aboveleft).mbmi.ref_frame as isize),
                    refframe,
                    &raw mut this_mv_0,
                    ref_frame_sign_bias,
                );
                if this_mv_0.as_int != (*mv).as_int {
                    mv = mv.offset(1);
                    (*mv).as_int = this_mv_0.as_int;
                    cntx = cntx.offset(1);
                }
                *cntx += 1 as i32;
            } else {
                *near_mv_ref_cnts.offset(CNT_INTRA as isize) += 1 as i32;
            }
        }
        if *near_mv_ref_cnts.offset(CNT_SPLITMV as isize) != 0
            && (*mv).as_int == near_mvs[CNT_NEAREST as usize].as_int
        {
            *near_mv_ref_cnts.offset(CNT_NEAREST as isize) += 1 as i32;
        }
        *near_mv_ref_cnts.offset(CNT_SPLITMV as isize) = (((*above).mbmi.mode as i32
            == SPLITMV as i32) as i32
            + ((*left).mbmi.mode as i32 == SPLITMV as i32) as i32)
            * 2 as i32
            + ((*aboveleft).mbmi.mode as i32 == SPLITMV as i32) as i32;
        if *near_mv_ref_cnts.offset(CNT_NEAR as isize)
            > *near_mv_ref_cnts.offset(CNT_NEAREST as isize)
        {
            let mut tmp: i32 = 0;
            tmp = *near_mv_ref_cnts.offset(CNT_NEAREST as isize);
            *near_mv_ref_cnts.offset(CNT_NEAREST as isize) =
                *near_mv_ref_cnts.offset(CNT_NEAR as isize);
            *near_mv_ref_cnts.offset(CNT_NEAR as isize) = tmp;
            tmp = near_mvs[CNT_NEAREST as usize].as_int as i32;
            near_mvs[CNT_NEAREST as usize].as_int = near_mvs[CNT_NEAR as usize].as_int;
            near_mvs[CNT_NEAR as usize].as_int = tmp as u32;
        }
        if *near_mv_ref_cnts.offset(CNT_NEAREST as isize)
            >= *near_mv_ref_cnts.offset(CNT_INTRA as isize)
        {
            near_mvs[CNT_INTRA as usize] = near_mvs[CNT_NEAREST as usize];
        }
        (*best_mv).as_int = near_mvs[0 as usize].as_int;
        (*nearest).as_int = near_mvs[CNT_NEAREST as usize].as_int;
        (*nearby).as_int = near_mvs[CNT_NEAR as usize].as_int;
    }
}
unsafe fn invert_and_clamp_mvs(mut inv: *mut IntMv, mut src: *mut IntMv, mut xd: *mut MACROBLOCKD) {
    unsafe {
        (*inv).as_mv.row = ((*src).as_mv.row as i32 * -(1 as i32)) as i16;
        (*inv).as_mv.col = ((*src).as_mv.col as i32 * -(1 as i32)) as i16;
        vp8_clamp_mv2(inv, xd);
        vp8_clamp_mv2(src, xd);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_find_near_mvs_bias(
    mut xd: *mut MACROBLOCKD,
    mut here: *const ModeInfo,
    mut mode_mv_sb: *mut [IntMv; 10],
    mut best_mv_sb: *mut IntMv,
    mut cnt: *mut i32,
    mut refframe: i32,
    mut ref_frame_sign_bias: *mut i32,
) -> i32 {
    unsafe {
        let mut sign_bias: i32 = *ref_frame_sign_bias.offset(refframe as isize);
        vp8_find_near_mvs(
            xd,
            here,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut IntMv)
                .offset(NEARESTMV as isize) as *mut IntMv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut IntMv).offset(NEARMV as isize)
                as *mut IntMv,
            best_mv_sb.offset(sign_bias as isize) as *mut IntMv,
            cnt,
            refframe,
            ref_frame_sign_bias,
        );
        invert_and_clamp_mvs(
            (&raw mut *mode_mv_sb.offset((sign_bias == 0) as isize) as *mut IntMv)
                .offset(NEARESTMV as isize) as *mut IntMv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut IntMv)
                .offset(NEARESTMV as isize) as *mut IntMv,
            xd,
        );
        invert_and_clamp_mvs(
            (&raw mut *mode_mv_sb.offset((sign_bias == 0) as isize) as *mut IntMv)
                .offset(NEARMV as isize) as *mut IntMv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut IntMv).offset(NEARMV as isize)
                as *mut IntMv,
            xd,
        );
        invert_and_clamp_mvs(
            best_mv_sb.offset((sign_bias == 0) as isize) as *mut IntMv,
            best_mv_sb.offset(sign_bias as isize) as *mut IntMv,
            xd,
        );
        sign_bias
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_mv_ref_probs(
    mut p: *mut Vp8Prob,
    mut near_mv_ref_ct: *const i32,
) -> *mut Vp8Prob {
    unsafe {
        *p.offset(0 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(0 as isize) as usize][0 as usize] as Vp8Prob;
        *p.offset(1 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(1 as isize) as usize][1 as usize] as Vp8Prob;
        *p.offset(2 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(2 as isize) as usize][2 as usize] as Vp8Prob;
        *p.offset(3 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(3 as isize) as usize][3 as usize] as Vp8Prob;
        p as *mut Vp8Prob
    }
}
