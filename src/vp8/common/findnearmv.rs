use std::ffi::c_void;
unsafe extern "Rust" {
    static vp8_mode_contexts: [[i32; 4]; 6];
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
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
pub type jmp_buf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: jmp_buf,
}
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type vp8_prob = u8;
pub type ENTROPY_CONTEXT = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
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
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
pub type C2RustUnnamed_0 = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed_0 = 4;
pub const ALTREF_FRAME: C2RustUnnamed_0 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_0 = 2;
pub const LAST_FRAME: C2RustUnnamed_0 = 1;
pub const INTRA_FRAME: C2RustUnnamed_0 = 0;
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
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
    pub bmi: b_mode_info,
}
pub type BLOCKD = blockd;
pub type vp8_subpix_fn_t = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
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
pub type MACROBLOCKD = macroblockd;
pub const CNT_NEAR: C2RustUnnamed_1 = 2;
pub const CNT_NEAREST: C2RustUnnamed_1 = 1;
pub const CNT_INTRA: C2RustUnnamed_1 = 0;
pub const CNT_SPLITMV: C2RustUnnamed_1 = 3;
pub type C2RustUnnamed_1 = u32;
#[inline]
unsafe fn mv_bias(
    mut refmb_ref_frame_sign_bias: i32,
    mut refframe: i32,
    mut mvp: *mut int_mv,
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
unsafe fn vp8_clamp_mv2(mut mv: *mut int_mv, mut xd: *const MACROBLOCKD) {
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
    mut here: *const MODE_INFO,
    mut nearest: *mut int_mv,
    mut nearby: *mut int_mv,
    mut best_mv: *mut int_mv,
    mut near_mv_ref_cnts: *mut i32,
    mut refframe: i32,
    mut ref_frame_sign_bias: *mut i32,
) {
    unsafe {
        let mut above: *const MODE_INFO = here.offset(-((*xd).mode_info_stride as isize));
        let mut left: *const MODE_INFO = here.offset(-(1 as isize));
        let mut aboveleft: *const MODE_INFO = above.offset(-(1 as isize));
        let mut near_mvs: [int_mv; 4] = [int_mv { as_int: 0 }; 4];
        let mut mv: *mut int_mv = &raw mut near_mvs as *mut int_mv;
        let mut cntx: *mut i32 = near_mv_ref_cnts as *mut i32;
        let fresh0 = &mut (*mv.offset(2 as isize)).as_int;
        *fresh0 = 0 as uint32_t;
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
                let mut this_mv: int_mv = int_mv { as_int: 0 };
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
                let mut this_mv_0: int_mv = int_mv { as_int: 0 };
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
            near_mvs[CNT_NEAR as usize].as_int = tmp as uint32_t;
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
unsafe fn invert_and_clamp_mvs(
    mut inv: *mut int_mv,
    mut src: *mut int_mv,
    mut xd: *mut MACROBLOCKD,
) {
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
    mut here: *const MODE_INFO,
    mut mode_mv_sb: *mut [int_mv; 10],
    mut best_mv_sb: *mut int_mv,
    mut cnt: *mut i32,
    mut refframe: i32,
    mut ref_frame_sign_bias: *mut i32,
) -> i32 {
    unsafe {
        let mut sign_bias: i32 = *ref_frame_sign_bias.offset(refframe as isize);
        vp8_find_near_mvs(
            xd,
            here,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
                .offset(NEARESTMV as isize) as *mut int_mv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv).offset(NEARMV as isize)
                as *mut int_mv,
            best_mv_sb.offset(sign_bias as isize) as *mut int_mv,
            cnt,
            refframe,
            ref_frame_sign_bias,
        );
        invert_and_clamp_mvs(
            (&raw mut *mode_mv_sb.offset((sign_bias == 0) as isize) as *mut int_mv)
                .offset(NEARESTMV as isize) as *mut int_mv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
                .offset(NEARESTMV as isize) as *mut int_mv,
            xd,
        );
        invert_and_clamp_mvs(
            (&raw mut *mode_mv_sb.offset((sign_bias == 0) as isize) as *mut int_mv)
                .offset(NEARMV as isize) as *mut int_mv,
            (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv).offset(NEARMV as isize)
                as *mut int_mv,
            xd,
        );
        invert_and_clamp_mvs(
            best_mv_sb.offset((sign_bias == 0) as isize) as *mut int_mv,
            best_mv_sb.offset(sign_bias as isize) as *mut int_mv,
            xd,
        );
        sign_bias
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_mv_ref_probs(
    mut p: *mut vp8_prob,
    mut near_mv_ref_ct: *const i32,
) -> *mut vp8_prob {
    unsafe {
        *p.offset(0 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(0 as isize) as usize][0 as usize] as vp8_prob;
        *p.offset(1 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(1 as isize) as usize][1 as usize] as vp8_prob;
        *p.offset(2 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(2 as isize) as usize][2 as usize] as vp8_prob;
        *p.offset(3 as isize) =
            vp8_mode_contexts[*near_mv_ref_ct.offset(3 as isize) as usize][3 as usize] as vp8_prob;
        p as *mut vp8_prob
    }
}
