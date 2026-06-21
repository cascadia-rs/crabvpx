use crate::vp8::common::modecont::vp8_mode_contexts;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;
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

pub type C2RustUnnamed_0 = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed_0 = 4;
pub const ALTREF_FRAME: C2RustUnnamed_0 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_0 = 2;
pub const LAST_FRAME: C2RustUnnamed_0 = 1;
pub const INTRA_FRAME: C2RustUnnamed_0 = 0;

pub const CNT_NEAR: C2RustUnnamed_1 = 2;
pub const CNT_NEAREST: C2RustUnnamed_1 = 1;
pub const CNT_INTRA: C2RustUnnamed_1 = 0;
pub const CNT_SPLITMV: C2RustUnnamed_1 = 3;
pub type C2RustUnnamed_1 = u32;
#[inline]
fn mv_bias(
    neighbor_sign_bias: i32,
    current_ref_frame: i32,
    mvp: &mut int_mv,
    ref_frame_sign_bias: &[i32],
) {
    if current_ref_frame != ref_frame_sign_bias[neighbor_sign_bias as usize] {
        let mv = mvp.as_mv_mut();
        mv.row = -mv.row;
        mv.col = -mv.col;
    }
}
pub const LEFT_TOP_MARGIN: i32 = 16_i32 << 3_i32;
pub const RIGHT_BOTTOM_MARGIN: i32 = 16_i32 << 3_i32;
#[inline]
pub fn vp8_clamp_mv2(mv: &mut MV, xd: &MACROBLOCKD) {
    if (mv.col as i32) < xd.mb_to_left_edge - LEFT_TOP_MARGIN {
        mv.col = (xd.mb_to_left_edge - LEFT_TOP_MARGIN) as i16;
    } else if mv.col as i32 > xd.mb_to_right_edge + RIGHT_BOTTOM_MARGIN {
        mv.col = (xd.mb_to_right_edge + RIGHT_BOTTOM_MARGIN) as i16;
    }
    if (mv.row as i32) < xd.mb_to_top_edge - LEFT_TOP_MARGIN {
        mv.row = (xd.mb_to_top_edge - LEFT_TOP_MARGIN) as i16;
    } else if mv.row as i32 > xd.mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN {
        mv.row = (xd.mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN) as i16;
    }
}
pub static vp8_mbsplit_offset: [[u8; 16]; 4] = [
    [
        0_i32 as u8,
        8_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
    ],
    [
        0_i32 as u8,
        2_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
    ],
    [
        0_i32 as u8,
        2_i32 as u8,
        8_i32 as u8,
        10_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
        0_i32 as u8,
    ],
    [
        0_i32 as u8,
        1_i32 as u8,
        2_i32 as u8,
        3_i32 as u8,
        4_i32 as u8,
        5_i32 as u8,
        6_i32 as u8,
        7_i32 as u8,
        8_i32 as u8,
        9_i32 as u8,
        10_i32 as u8,
        11_i32 as u8,
        12_i32 as u8,
        13_i32 as u8,
        14_i32 as u8,
        15_i32 as u8,
    ],
];
pub fn vp8_find_near_mvs_safe(
    _xd: &MACROBLOCKD,
    above: &MODE_INFO,
    left: &MODE_INFO,
    aboveleft: &MODE_INFO,
    nearest: &mut int_mv,
    nearby: &mut int_mv,
    best_mv: &mut int_mv,
    near_mv_ref_cnts: &mut [i32; 4],
    refframe: i32,
    ref_frame_sign_bias: &[i32; 4],
) {
    let mut near_mvs: [int_mv; 4] = [int_mv::default(); 4];
    near_mv_ref_cnts.fill(0);

    let mut nmv_idx = 0;
    let mut cntx_idx = 0;

    if above.mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
        if above.mbmi.mv.as_int() != 0 {
            nmv_idx += 1;
            near_mvs[nmv_idx] = above.mbmi.mv;
            mv_bias(
                ref_frame_sign_bias[above.mbmi.ref_frame as usize],
                refframe,
                &mut near_mvs[nmv_idx],
                ref_frame_sign_bias,
            );
            cntx_idx += 1;
        }
        near_mv_ref_cnts[cntx_idx] += 2_i32;
    }

    if left.mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
        if left.mbmi.mv.as_int() != 0 {
            let mut this_mv = left.mbmi.mv;
            mv_bias(
                ref_frame_sign_bias[left.mbmi.ref_frame as usize],
                refframe,
                &mut this_mv,
                ref_frame_sign_bias,
            );
            if this_mv.as_int() != near_mvs[nmv_idx].as_int() {
                nmv_idx += 1;
                near_mvs[nmv_idx] = this_mv;
                cntx_idx += 1;
            }
            near_mv_ref_cnts[cntx_idx] += 2_i32;
        } else {
            near_mv_ref_cnts[CNT_INTRA as usize] += 2_i32;
        }
    }

    if aboveleft.mbmi.ref_frame as i32 != INTRA_FRAME as i32 {
        if aboveleft.mbmi.mv.as_int() != 0 {
            let mut this_mv_0 = aboveleft.mbmi.mv;
            mv_bias(
                ref_frame_sign_bias[aboveleft.mbmi.ref_frame as usize],
                refframe,
                &mut this_mv_0,
                ref_frame_sign_bias,
            );
            if this_mv_0.as_int() != near_mvs[nmv_idx].as_int() {
                nmv_idx += 1;
                near_mvs[nmv_idx] = this_mv_0;
                cntx_idx += 1;
            }
            near_mv_ref_cnts[cntx_idx] += 1_i32;
        } else {
            near_mv_ref_cnts[CNT_INTRA as usize] += 1_i32;
        }
    }

    if near_mv_ref_cnts[CNT_SPLITMV as usize] != 0
        && near_mvs[nmv_idx].as_int() == near_mvs[CNT_NEAREST as usize].as_int()
    {
        near_mv_ref_cnts[CNT_NEAREST as usize] += 1_i32;
    }

    near_mv_ref_cnts[CNT_SPLITMV as usize] = ((above.mbmi.mode as i32 == SPLITMV as i32) as i32
        + (left.mbmi.mode as i32 == SPLITMV as i32) as i32)
        * 2_i32
        + (aboveleft.mbmi.mode as i32 == SPLITMV as i32) as i32;

    if near_mv_ref_cnts[CNT_NEAR as usize] > near_mv_ref_cnts[CNT_NEAREST as usize] {
        near_mv_ref_cnts.swap(CNT_NEAREST as usize, CNT_NEAR as usize);

        let tmp_mv = near_mvs[CNT_NEAREST as usize].as_int();
        near_mvs[CNT_NEAREST as usize].set_as_int(near_mvs[CNT_NEAR as usize].as_int());
        near_mvs[CNT_NEAR as usize].set_as_int(tmp_mv);
    }

    if near_mv_ref_cnts[CNT_NEAREST as usize] >= near_mv_ref_cnts[CNT_INTRA as usize] {
        near_mvs[CNT_INTRA as usize] = near_mvs[CNT_NEAREST as usize];
    }

    best_mv.set_as_int(near_mvs[0].as_int());
    nearest.set_as_int(near_mvs[CNT_NEAREST as usize].as_int());
    nearby.set_as_int(near_mvs[CNT_NEAR as usize].as_int());
}

pub fn vp8_mv_ref_probs_safe(p: &mut [vp8_prob; 4], near_mv_ref_ct: &[i32; 4]) {
    p[0] = vp8_mode_contexts[near_mv_ref_ct[0] as usize][0] as vp8_prob;
    p[1] = vp8_mode_contexts[near_mv_ref_ct[1] as usize][1] as vp8_prob;
    p[2] = vp8_mode_contexts[near_mv_ref_ct[2] as usize][2] as vp8_prob;
    p[3] = vp8_mode_contexts[near_mv_ref_ct[3] as usize][3] as vp8_prob;
}
