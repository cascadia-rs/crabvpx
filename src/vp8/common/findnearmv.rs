unsafe extern "C" {
    static vp8_mode_contexts: [[::core::ffi::c_int; 4]; 6];
}
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
unsafe extern "C" fn mv_bias(
    mut refmb_ref_frame_sign_bias: ::core::ffi::c_int,
    mut refframe: ::core::ffi::c_int,
    mut mvp: *mut int_mv,
    mut ref_frame_sign_bias: *const ::core::ffi::c_int,
) { unsafe {
    if refmb_ref_frame_sign_bias != *ref_frame_sign_bias.offset(refframe as isize) {
        (*mvp).as_mv.row = ((*mvp).as_mv.row as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
        (*mvp).as_mv.col = ((*mvp).as_mv.col as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    }
}}
pub const LEFT_TOP_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const RIGHT_BOTTOM_MARGIN: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vp8_clamp_mv2(mut mv: *mut int_mv, mut xd: *const MACROBLOCKD) { unsafe {
    if ((*mv).as_mv.col as ::core::ffi::c_int) < (*xd).mb_to_left_edge - LEFT_TOP_MARGIN {
        (*mv).as_mv.col = ((*xd).mb_to_left_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if (*mv).as_mv.col as ::core::ffi::c_int > (*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN {
        (*mv).as_mv.col = ((*xd).mb_to_right_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
    if ((*mv).as_mv.row as ::core::ffi::c_int) < (*xd).mb_to_top_edge - LEFT_TOP_MARGIN {
        (*mv).as_mv.row = ((*xd).mb_to_top_edge - LEFT_TOP_MARGIN) as ::core::ffi::c_short;
    } else if (*mv).as_mv.row as ::core::ffi::c_int > (*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN
    {
        (*mv).as_mv.row = ((*xd).mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN) as ::core::ffi::c_short;
    }
}}
#[unsafe(no_mangle)]
pub static mut vp8_mbsplit_offset: [[::core::ffi::c_uchar; 16]; 4] = [
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
) { unsafe {
    let mut above: *const MODE_INFO = here.offset(-((*xd).mode_info_stride as isize));
    let mut left: *const MODE_INFO = here.offset(-(1 as ::core::ffi::c_int as isize));
    let mut aboveleft: *const MODE_INFO = above.offset(-(1 as ::core::ffi::c_int as isize));
    let mut near_mvs: [int_mv; 4] = [int_mv { as_int: 0 }; 4];
    let mut mv: *mut int_mv = &raw mut near_mvs as *mut int_mv;
    let mut cntx: *mut ::core::ffi::c_int = near_mv_ref_cnts as *mut ::core::ffi::c_int;
    let ref mut fresh0 = (*mv.offset(2 as ::core::ffi::c_int as isize)).as_int;
    *fresh0 = 0 as uint32_t;
    let ref mut fresh1 = (*mv.offset(1 as ::core::ffi::c_int as isize)).as_int;
    *fresh1 = *fresh0;
    (*mv.offset(0 as ::core::ffi::c_int as isize)).as_int = *fresh1;
    let ref mut fresh2 = *near_mv_ref_cnts.offset(3 as ::core::ffi::c_int as isize);
    *fresh2 = 0 as ::core::ffi::c_int;
    let ref mut fresh3 = *near_mv_ref_cnts.offset(2 as ::core::ffi::c_int as isize);
    *fresh3 = *fresh2;
    let ref mut fresh4 = *near_mv_ref_cnts.offset(1 as ::core::ffi::c_int as isize);
    *fresh4 = *fresh3;
    *near_mv_ref_cnts.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
    if (*above).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
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
        *cntx += 2 as ::core::ffi::c_int;
    }
    if (*left).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
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
            *cntx += 2 as ::core::ffi::c_int;
        } else {
            *near_mv_ref_cnts.offset(CNT_INTRA as ::core::ffi::c_int as isize) +=
                2 as ::core::ffi::c_int;
        }
    }
    if (*aboveleft).mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
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
            *cntx += 1 as ::core::ffi::c_int;
        } else {
            *near_mv_ref_cnts.offset(CNT_INTRA as ::core::ffi::c_int as isize) +=
                1 as ::core::ffi::c_int;
        }
    }
    if *near_mv_ref_cnts.offset(CNT_SPLITMV as ::core::ffi::c_int as isize) != 0 {
        if (*mv).as_int == near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int {
            *near_mv_ref_cnts.offset(CNT_NEAREST as ::core::ffi::c_int as isize) +=
                1 as ::core::ffi::c_int;
        }
    }
    *near_mv_ref_cnts.offset(CNT_SPLITMV as ::core::ffi::c_int as isize) =
        (((*above).mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int)
            as ::core::ffi::c_int
            + ((*left).mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int)
                as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int
            + ((*aboveleft).mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int)
                as ::core::ffi::c_int;
    if *near_mv_ref_cnts.offset(CNT_NEAR as ::core::ffi::c_int as isize)
        > *near_mv_ref_cnts.offset(CNT_NEAREST as ::core::ffi::c_int as isize)
    {
        let mut tmp: ::core::ffi::c_int = 0;
        tmp = *near_mv_ref_cnts.offset(CNT_NEAREST as ::core::ffi::c_int as isize);
        *near_mv_ref_cnts.offset(CNT_NEAREST as ::core::ffi::c_int as isize) =
            *near_mv_ref_cnts.offset(CNT_NEAR as ::core::ffi::c_int as isize);
        *near_mv_ref_cnts.offset(CNT_NEAR as ::core::ffi::c_int as isize) = tmp;
        tmp = near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int as ::core::ffi::c_int;
        near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int =
            near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int;
        near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int = tmp as uint32_t;
    }
    if *near_mv_ref_cnts.offset(CNT_NEAREST as ::core::ffi::c_int as isize)
        >= *near_mv_ref_cnts.offset(CNT_INTRA as ::core::ffi::c_int as isize)
    {
        near_mvs[CNT_INTRA as ::core::ffi::c_int as usize] =
            near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize];
    }
    (*best_mv).as_int = near_mvs[0 as ::core::ffi::c_int as usize].as_int;
    (*nearest).as_int = near_mvs[CNT_NEAREST as ::core::ffi::c_int as usize].as_int;
    (*nearby).as_int = near_mvs[CNT_NEAR as ::core::ffi::c_int as usize].as_int;
}}
unsafe extern "C" fn invert_and_clamp_mvs(
    mut inv: *mut int_mv,
    mut src: *mut int_mv,
    mut xd: *mut MACROBLOCKD,
) { unsafe {
    (*inv).as_mv.row = ((*src).as_mv.row as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
        as ::core::ffi::c_short;
    (*inv).as_mv.col = ((*src).as_mv.col as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
        as ::core::ffi::c_short;
    vp8_clamp_mv2(inv, xd);
    vp8_clamp_mv2(src, xd);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_find_near_mvs_bias(
    mut xd: *mut MACROBLOCKD,
    mut here: *const MODE_INFO,
    mut mode_mv_sb: *mut [int_mv; 10],
    mut best_mv_sb: *mut int_mv,
    mut cnt: *mut ::core::ffi::c_int,
    mut refframe: ::core::ffi::c_int,
    mut ref_frame_sign_bias: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut sign_bias: ::core::ffi::c_int = *ref_frame_sign_bias.offset(refframe as isize);
    vp8_find_near_mvs(
        xd,
        here,
        (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
            .offset(NEARESTMV as ::core::ffi::c_int as isize) as *mut int_mv,
        (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
            .offset(NEARMV as ::core::ffi::c_int as isize) as *mut int_mv,
        best_mv_sb.offset(sign_bias as isize) as *mut int_mv,
        cnt,
        refframe,
        ref_frame_sign_bias,
    );
    invert_and_clamp_mvs(
        (&raw mut *mode_mv_sb.offset((sign_bias == 0) as ::core::ffi::c_int as isize)
            as *mut int_mv)
            .offset(NEARESTMV as ::core::ffi::c_int as isize) as *mut int_mv,
        (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
            .offset(NEARESTMV as ::core::ffi::c_int as isize) as *mut int_mv,
        xd,
    );
    invert_and_clamp_mvs(
        (&raw mut *mode_mv_sb.offset((sign_bias == 0) as ::core::ffi::c_int as isize)
            as *mut int_mv)
            .offset(NEARMV as ::core::ffi::c_int as isize) as *mut int_mv,
        (&raw mut *mode_mv_sb.offset(sign_bias as isize) as *mut int_mv)
            .offset(NEARMV as ::core::ffi::c_int as isize) as *mut int_mv,
        xd,
    );
    invert_and_clamp_mvs(
        best_mv_sb.offset((sign_bias == 0) as ::core::ffi::c_int as isize) as *mut int_mv,
        best_mv_sb.offset(sign_bias as isize) as *mut int_mv,
        xd,
    );
    return sign_bias;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_mv_ref_probs(
    mut p: *mut vp8_prob,
    mut near_mv_ref_ct: *const ::core::ffi::c_int,
) -> *mut vp8_prob { unsafe {
    *p.offset(0 as ::core::ffi::c_int as isize) =
        vp8_mode_contexts[*near_mv_ref_ct.offset(0 as ::core::ffi::c_int as isize) as usize]
            [0 as ::core::ffi::c_int as usize] as vp8_prob;
    *p.offset(1 as ::core::ffi::c_int as isize) =
        vp8_mode_contexts[*near_mv_ref_ct.offset(1 as ::core::ffi::c_int as isize) as usize]
            [1 as ::core::ffi::c_int as usize] as vp8_prob;
    *p.offset(2 as ::core::ffi::c_int as isize) =
        vp8_mode_contexts[*near_mv_ref_ct.offset(2 as ::core::ffi::c_int as isize) as usize]
            [2 as ::core::ffi::c_int as usize] as vp8_prob;
    *p.offset(3 as ::core::ffi::c_int as isize) =
        vp8_mode_contexts[*near_mv_ref_ct.offset(3 as ::core::ffi::c_int as isize) as usize]
            [3 as ::core::ffi::c_int as usize] as vp8_prob;
    return p as *mut vp8_prob;
}}
