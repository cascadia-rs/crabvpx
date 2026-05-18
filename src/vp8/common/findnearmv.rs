use crate::vp8::common::modecont::vp8_mode_contexts;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;

pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: C2RustUnnamed_0 = 4;
pub const ALTREF_FRAME: C2RustUnnamed_0 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_0 = 2;
pub const LAST_FRAME: C2RustUnnamed_0 = 1;
pub const INTRA_FRAME: C2RustUnnamed_0 = 0;

pub const CNT_NEAR: C2RustUnnamed_1 = 2;
pub const CNT_NEAREST: C2RustUnnamed_1 = 1;
pub const CNT_INTRA: C2RustUnnamed_1 = 0;
pub const CNT_SPLITMV: C2RustUnnamed_1 = 3;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
#[inline]
fn mv_bias(
    neighbor_sign_bias: ::core::ffi::c_int,
    current_ref_frame: ::core::ffi::c_int,
    mvp: &mut int_mv,
    ref_frame_sign_bias: &[::core::ffi::c_int],
) {
    if current_ref_frame != ref_frame_sign_bias[neighbor_sign_bias as usize] {
        let mv = mvp.as_mv_mut();
        mv.row = -mv.row;
        mv.col = -mv.col;
    }
}
pub const LEFT_TOP_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const RIGHT_BOTTOM_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline]
pub fn vp8_clamp_mv2(mv: &mut MV, xd: &MACROBLOCKD) {
    if (mv.col as ::core::ffi::c_int) < xd.mb_to_left_edge - LEFT_TOP_MARGIN {
        mv.col = (xd.mb_to_left_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if mv.col as ::core::ffi::c_int > xd.mb_to_right_edge + RIGHT_BOTTOM_MARGIN {
        mv.col = (xd.mb_to_right_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
    if (mv.row as ::core::ffi::c_int) < xd.mb_to_top_edge - LEFT_TOP_MARGIN {
        mv.row = (xd.mb_to_top_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if mv.row as ::core::ffi::c_int > xd.mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN
    {
        mv.row = (xd.mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
}
#[unsafe(no_mangle)]
pub static vp8_mbsplit_offset: [[::core::ffi::c_uchar; 16]; 4] = [
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
];
pub fn vp8_find_near_mvs_safe(
    xd: &MACROBLOCKD,
    above: &MODE_INFO,
    left: &MODE_INFO,
    aboveleft: &MODE_INFO,
    nearest: &mut int_mv,
    nearby: &mut int_mv,
    best_mv: &mut int_mv,
    near_mv_ref_cnts: &mut [::core::ffi::c_int; 4],
    refframe: ::core::ffi::c_int,
    ref_frame_sign_bias: &[::core::ffi::c_int; 4],
) {
    let mut near_mvs: [int_mv; 4] = [int_mv { as_int: 0 }; 4];
    near_mv_ref_cnts.fill(0);

    let mut nmv_idx = 0;
    let mut cntx_idx = 0;

    if above.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
        if above.mbmi.mv.as_int() != 0 {
            nmv_idx += 1;
            near_mvs[nmv_idx].set_as_int(above.mbmi.mv.as_int());
            mv_bias(
                ref_frame_sign_bias[above.mbmi.ref_frame as usize],
                refframe,
                &mut near_mvs[nmv_idx],
                ref_frame_sign_bias,
            );
            cntx_idx += 1;
        }
        near_mv_ref_cnts[cntx_idx] += 2 as ::core::ffi::c_int;
    }

    if left.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
        if left.mbmi.mv.as_int() != 0 {
            let mut this_mv = int_mv { as_int: left.mbmi.mv.as_int() };
            mv_bias(
                ref_frame_sign_bias[left.mbmi.ref_frame as usize],
                refframe,
                &mut this_mv,
                ref_frame_sign_bias,
            );
            if this_mv.as_int() != near_mvs[nmv_idx].as_int() {
                nmv_idx += 1;
                near_mvs[nmv_idx].set_as_int(this_mv.as_int());
                cntx_idx += 1;
            }
            near_mv_ref_cnts[cntx_idx] += 2 as ::core::ffi::c_int;
        } else {
            near_mv_ref_cnts[CNT_INTRA as usize] += 2 as ::core::ffi::c_int;
        }
    }

    if aboveleft.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
        if aboveleft.mbmi.mv.as_int() != 0 {
            let mut this_mv_0 = int_mv { as_int: aboveleft.mbmi.mv.as_int() };
            mv_bias(
                ref_frame_sign_bias[aboveleft.mbmi.ref_frame as usize],
                refframe,
                &mut this_mv_0,
                ref_frame_sign_bias,
            );
            if this_mv_0.as_int() != near_mvs[nmv_idx].as_int() {
                nmv_idx += 1;
                near_mvs[nmv_idx].set_as_int(this_mv_0.as_int());
                cntx_idx += 1;
            }
            near_mv_ref_cnts[cntx_idx] += 1 as ::core::ffi::c_int;
        } else {
            near_mv_ref_cnts[CNT_INTRA as usize] += 1 as ::core::ffi::c_int;
        }
    }

    if near_mv_ref_cnts[CNT_SPLITMV as usize] != 0 {
        if near_mvs[nmv_idx].as_int() == near_mvs[CNT_NEAREST as usize].as_int() {
            near_mv_ref_cnts[CNT_NEAREST as usize] += 1 as ::core::ffi::c_int;
        }
    }

    near_mv_ref_cnts[CNT_SPLITMV as usize] = 
        (((above.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int
            + (left.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int
            + (aboveleft.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int);

    if near_mv_ref_cnts[CNT_NEAR as usize] > near_mv_ref_cnts[CNT_NEAREST as usize] {
        let tmp = near_mv_ref_cnts[CNT_NEAREST as usize];
        near_mv_ref_cnts[CNT_NEAREST as usize] = near_mv_ref_cnts[CNT_NEAR as usize];
        near_mv_ref_cnts[CNT_NEAR as usize] = tmp;

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_find_near_mvs(
    mut xd: *mut MACROBLOCKD,
    mut here: *const MODE_INFO,
    mut nearest: *mut int_mv,
    mut nearby: *mut int_mv,
    mut best_mv: *mut int_mv,
    mut near_mv_ref_cnts: *mut ::core::ffi::c_int,
    mut refframe: ::core::ffi::c_int,
    mut ref_frame_sign_bias: *mut ::core::ffi::c_int,
) {
    unsafe {
        let xd_ref = &*xd;
        let stride = xd_ref.mode_info_stride as isize;
        let above_ref = &*here.offset(-stride);
        let left_ref = &*here.offset(-1);
        let aboveleft_ref = &*here.offset(-stride - 1);

        let nearest_ref = &mut *nearest;
        let nearby_ref = &mut *nearby;
        let best_mv_ref = &mut *best_mv;

        let near_mv_ref_cnts_slice = &mut *(near_mv_ref_cnts as *mut [::core::ffi::c_int; 4]);
        let ref_frame_sign_bias_slice = &*(ref_frame_sign_bias as *const [::core::ffi::c_int; 4]);

        vp8_find_near_mvs_safe(
            xd_ref,
            above_ref,
            left_ref,
            aboveleft_ref,
            nearest_ref,
            nearby_ref,
            best_mv_ref,
            near_mv_ref_cnts_slice,
            refframe,
            ref_frame_sign_bias_slice,
        );
    }
}

fn invert_and_clamp_mvs(
    inv: &mut MV,
    src: &mut MV,
    xd: &MACROBLOCKD,
) {
    inv.row = -src.row;
    inv.col = -src.col;
    vp8_clamp_mv2(inv, xd);
    vp8_clamp_mv2(src, xd);
}

pub fn vp8_find_near_mvs_bias_safe(
    xd: &MACROBLOCKD,
    above: &MODE_INFO,
    left: &MODE_INFO,
    aboveleft: &MODE_INFO,
    mode_mv_sb: &mut [[int_mv; 10]; 2],
    best_mv_sb: &mut [int_mv; 2],
    cnt: &mut [::core::ffi::c_int; 4],
    refframe: ::core::ffi::c_int,
    ref_frame_sign_bias: &[::core::ffi::c_int; 4],
) -> ::core::ffi::c_int {
    let sign_bias = ref_frame_sign_bias[refframe as usize];

    let (mode_first, mode_second) = mode_mv_sb.split_at_mut(1);
    let sb_mv = if sign_bias == 0 { &mut mode_first[0] } else { &mut mode_second[0] };

    let (best_first, best_second) = best_mv_sb.split_at_mut(1);
    let best_sb_mv = if sign_bias == 0 { &mut best_first[0] } else { &mut best_second[0] };

    let (nearest, nearby) = {
        let (first, second) = sb_mv.split_at_mut(6);
        (&mut first[5], &mut second[0])
    };

    vp8_find_near_mvs_safe(
        xd,
        above,
        left,
        aboveleft,
        nearest,
        nearby,
        best_sb_mv,
        cnt,
        refframe,
        ref_frame_sign_bias,
    );

    let (sb_mv_mut, inv_sb_mv_mut) = if sign_bias == 0 {
        (&mut mode_first[0], &mut mode_second[0])
    } else {
        (&mut mode_second[0], &mut mode_first[0])
    };

    let (best_sb_mv_mut, best_inv_sb_mv_mut) = if sign_bias == 0 {
        (&mut best_first[0], &mut best_second[0])
    } else {
        (&mut best_second[0], &mut best_first[0])
    };

    invert_and_clamp_mvs(
        inv_sb_mv_mut[NEARESTMV as usize].as_mv_mut(),
        sb_mv_mut[NEARESTMV as usize].as_mv_mut(),
        xd,
    );
    invert_and_clamp_mvs(
        inv_sb_mv_mut[NEARMV as usize].as_mv_mut(),
        sb_mv_mut[NEARMV as usize].as_mv_mut(),
        xd,
    );
    invert_and_clamp_mvs(
        best_inv_sb_mv_mut.as_mv_mut(),
        best_sb_mv_mut.as_mv_mut(),
        xd,
    );

    sign_bias
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_find_near_mvs_bias(
    mut xd: *mut MACROBLOCKD,
    mut here: *const MODE_INFO,
    mut mode_mv_sb: *mut [int_mv; 10],
    mut best_mv_sb: *mut int_mv,
    mut cnt: *mut ::core::ffi::c_int,
    mut refframe: ::core::ffi::c_int,
    mut ref_frame_sign_bias: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let xd_ref = &*xd;
        let stride = xd_ref.mode_info_stride as isize;
        let above_ref = &*here.offset(-stride);
        let left_ref = &*here.offset(-1);
        let aboveleft_ref = &*here.offset(-stride - 1);

        let mode_mv_sb_ref = &mut *(mode_mv_sb as *mut [[int_mv; 10]; 2]);
        let best_mv_sb_ref = &mut *(best_mv_sb as *mut [int_mv; 2]);
        let cnt_ref = &mut *(cnt as *mut [::core::ffi::c_int; 4]);
        let ref_frame_sign_bias_ref = &*(ref_frame_sign_bias as *const [::core::ffi::c_int; 4]);

        vp8_find_near_mvs_bias_safe(
            xd_ref,
            above_ref,
            left_ref,
            aboveleft_ref,
            mode_mv_sb_ref,
            best_mv_sb_ref,
            cnt_ref,
            refframe,
            ref_frame_sign_bias_ref,
        )
    }
}
pub fn vp8_mv_ref_probs_safe(
    p: &mut [vp8_prob; 4],
    near_mv_ref_ct: &[::core::ffi::c_int; 4],
) {
    p[0] = vp8_mode_contexts[near_mv_ref_ct[0] as usize][0] as vp8_prob;
    p[1] = vp8_mode_contexts[near_mv_ref_ct[1] as usize][1] as vp8_prob;
    p[2] = vp8_mode_contexts[near_mv_ref_ct[2] as usize][2] as vp8_prob;
    p[3] = vp8_mode_contexts[near_mv_ref_ct[3] as usize][3] as vp8_prob;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_mv_ref_probs(
    p: *mut vp8_prob,
    near_mv_ref_ct: *const ::core::ffi::c_int,
) -> *mut vp8_prob {
    unsafe {
        let p_slice = &mut *(p as *mut [vp8_prob; 4]);
        let ct_slice = &*(near_mv_ref_ct as *const [::core::ffi::c_int; 4]);
        vp8_mv_ref_probs_safe(p_slice, ct_slice);
        p
    }
}
