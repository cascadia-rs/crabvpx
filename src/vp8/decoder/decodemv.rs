use crate::vp8::common::entropymv::vp8_mv_update_probs;
use crate::vp8::decoder::dboolhuff::SafeBoolDecoder;
use crate::vp8::common::entropymode::{vp8_bmode_tree, vp8_ymode_tree, vp8_kf_ymode_tree, vp8_uv_mode_tree, vp8_small_mvtree, vp8_kf_bmode_prob, vp8_kf_uv_mode_prob, vp8_kf_ymode_prob};


use crate::vp8::common::modecont::vp8_mode_contexts;
use crate::vp8::common::findnearmv::{vp8_mbsplit_offset, vp8_clamp_mv2, LEFT_TOP_MARGIN, RIGHT_BOTTOM_MARGIN};
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type size_t = __darwin_size_t;
pub type mach_port_t = __darwin_mach_port_t;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;
pub type vp8_tree_index = ::core::ffi::c_schar;
pub type C2RustUnnamed = ::core::ffi::c_uint;
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

pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub const CNT_NEAREST: C2RustUnnamed_0 = 1;
pub const CNT_NEAR: C2RustUnnamed_0 = 2;
pub const CNT_SPLITMV: C2RustUnnamed_0 = 3;
pub const CNT_INTRA: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;


#[inline]
fn mv_bias(
    refmb_ref_frame_sign_bias: ::core::ffi::c_int,
    refframe: ::core::ffi::c_int,
    mvp: &mut MV,
    ref_frame_sign_bias: &[::core::ffi::c_int; 4],
) {
    if refmb_ref_frame_sign_bias != ref_frame_sign_bias[refframe as usize] {
        mvp.row = (mvp.row as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
        mvp.col = (mvp.col as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    }
}

#[inline]
fn vp8_check_mv_bounds(
    mv: &MV,
    mb_to_left_edge: ::core::ffi::c_int,
    mb_to_right_edge: ::core::ffi::c_int,
    mb_to_top_edge: ::core::ffi::c_int,
    mb_to_bottom_edge: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut need_to_clamp: ::core::ffi::c_uint = 0;
    need_to_clamp = ((mv.col as ::core::ffi::c_int) < mb_to_left_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= (mv.col as ::core::ffi::c_int > mb_to_right_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= ((mv.row as ::core::ffi::c_int) < mb_to_top_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    need_to_clamp |= (mv.row as ::core::ffi::c_int > mb_to_bottom_edge)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    return need_to_clamp;
}
#[inline]
fn left_block_mode(
    mip_slice: &[MODE_INFO],
    cur_idx: usize,
    b: usize,
) -> B_PREDICTION_MODE {
    if b & 3 == 0 {
        let left_mb = &mip_slice[cur_idx - 1];
        match left_mb.mbmi.mode as ::core::ffi::c_int {
            4 => left_mb.bmi[b + 3].mode(),
            0 => B_DC_PRED,
            1 => B_VE_PRED,
            2 => B_HE_PRED,
            3 => B_TM_PRED,
            _ => B_DC_PRED,
        }
    } else {
        let cur_mb = &mip_slice[cur_idx];
        cur_mb.bmi[b - 1].mode()
    }
}
#[inline]
fn above_block_mode(
    mip_slice: &[MODE_INFO],
    cur_idx: usize,
    mi_stride: usize,
    b: usize,
) -> B_PREDICTION_MODE {
    if b >> 2 == 0 {
        let above_mb = &mip_slice[cur_idx - mi_stride];
        match above_mb.mbmi.mode as ::core::ffi::c_int {
            4 => above_mb.bmi[b + 12].mode(),
            0 => B_DC_PRED,
            1 => B_VE_PRED,
            2 => B_HE_PRED,
            3 => B_TM_PRED,
            _ => B_DC_PRED,
        }
    } else {
        let cur_mb = &mip_slice[cur_idx];
        cur_mb.bmi[b - 4].mode()
    }
}
fn safe_treed_read(
    r: &mut SafeBoolDecoder,
    t: &[vp8_tree_index],
    p: &[vp8_prob],
) -> i32 {
    let mut i: usize = 0;
    loop {
        let prob = p[i >> 1];
        let bit = r.read_bool(prob as i32);
        let next_node = t[i + (bit as usize)];
        if next_node <= 0 {
            return -next_node as i32;
        }
        i = next_node as usize;
    }
}

fn read_bmode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> B_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_bmode_tree, p) as B_PREDICTION_MODE
}

fn read_ymode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_ymode_tree, p) as MB_PREDICTION_MODE
}

fn read_kf_ymode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_kf_ymode_tree, p) as MB_PREDICTION_MODE
}

fn read_uv_mode(
    bc: &mut SafeBoolDecoder,
    p: &[vp8_prob],
) -> MB_PREDICTION_MODE {
    safe_treed_read(bc, &vp8_uv_mode_tree, p) as MB_PREDICTION_MODE
}

fn read_kf_modes(
    mis: usize,
    mip_slice: &mut [MODE_INFO],
    cur_idx: usize,
    safe_decoder: &mut SafeBoolDecoder,
) {
    mip_slice[cur_idx].mbmi.ref_frame = INTRA_FRAME as uint8_t;
    mip_slice[cur_idx].mbmi.mode = read_kf_ymode(safe_decoder, &vp8_kf_ymode_prob) as uint8_t;
    if mip_slice[cur_idx].mbmi.mode as ::core::ffi::c_int == B_PRED as ::core::ffi::c_int {
        mip_slice[cur_idx].mbmi.is_4x4 = 1 as uint8_t;
        for i in 0..16usize {
            let A: B_PREDICTION_MODE = above_block_mode(mip_slice, cur_idx, mis, i) as B_PREDICTION_MODE;
            let L: B_PREDICTION_MODE = left_block_mode(mip_slice, cur_idx, i) as B_PREDICTION_MODE;
            mip_slice[cur_idx].bmi[i].set_mode(read_bmode(
                safe_decoder,
                &vp8_kf_bmode_prob[A as usize][L as usize],
            ));
        }
    }
    mip_slice[cur_idx].mbmi.uv_mode =
        read_uv_mode(safe_decoder, &vp8_kf_uv_mode_prob) as uint8_t;
}
fn read_mvcomponent(
    r: &mut SafeBoolDecoder,
    mvc: &MV_CONTEXT,
) -> i32 {
    let p = &mvc.prob;
    let mut x: i32 = 0;
    if r.read_bool(p[mvpis_short as usize] as i32) != 0 {
        for i in 0..3usize {
            x += r.read_bool(p[MVPbits as usize + i] as i32) << i;
        }
        for i in (4..mvlong_width as usize).rev() {
            x += r.read_bool(p[MVPbits as usize + i] as i32) << i;
        }
        if x & 0xfff0 == 0 || r.read_bool(p[MVPbits as usize + 3] as i32) != 0 {
            x += 8;
        }
    } else {
        x = safe_treed_read(r, &vp8_small_mvtree, &p[MVPshort as usize..]);
    }
    if x != 0 && r.read_bool(p[MVPsign as usize] as i32) != 0 {
        x = -x;
    }
    x
}
fn read_mv(r: &mut SafeBoolDecoder, mv: &mut MV, mvc: &[MV_CONTEXT; 2]) {
    mv.row = (read_mvcomponent(r, &mvc[0]) * 2) as i16;
    mv.col = (read_mvcomponent(r, &mvc[1]) * 2) as i16;
}
fn read_mvcontexts(bc: &mut SafeBoolDecoder, mvc: &mut [MV_CONTEXT; 2]) {
    for i in 0..2 {
        let up_probs = &vp8_mv_update_probs[i].prob;
        let p_probs = &mut mvc[i].prob;
        for j in 0..MVPcount as usize {
            let prob_to_decode = up_probs[j];
            if bc.read_bool(prob_to_decode as i32) != 0 {
                let x = bc.read_literal(7) as vp8_prob;
                p_probs[j] = if x != 0 { x << 1 } else { 1 };
            }
        }
    }
}

static mbsplit_fill_count: [::core::ffi::c_uchar; 4] = [
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mbsplit_fill_offset: [[::core::ffi::c_uchar; 16]; 4] = [
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
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
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
fn mb_mode_mv_init(pbi: &mut VP8D_COMP, safe_decoder: &mut SafeBoolDecoder) {
    let mvc = &mut pbi.common.fc.mvc;

    pbi.common.mb_no_coeff_skip = safe_decoder.read_bool(vp8_prob_half as i32);
    pbi.prob_skip_false = 0 as vp8_prob;
    if pbi.common.mb_no_coeff_skip != 0 {
        pbi.prob_skip_false = safe_decoder.read_literal(8) as vp8_prob;
    }
    if pbi.common.frame_type as ::core::ffi::c_uint
        != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pbi.prob_intra = safe_decoder.read_literal(8) as vp8_prob;
        pbi.prob_last = safe_decoder.read_literal(8) as vp8_prob;
        pbi.prob_gf = safe_decoder.read_literal(8) as vp8_prob;
        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
            for i in 0..4 {
                pbi.common.fc.ymode_prob[i] = safe_decoder.read_literal(8) as vp8_prob;
            }
        }
        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
            for i in 0..3 {
                pbi.common.fc.uv_mode_prob[i] = safe_decoder.read_literal(8) as vp8_prob;
            }
        }
        read_mvcontexts(safe_decoder, mvc);
    }
}
static vp8_sub_mv_ref_prob3: [[vp8_prob; 3]; 8] = [
    [
        147 as ::core::ffi::c_int as vp8_prob,
        136 as ::core::ffi::c_int as vp8_prob,
        18 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        223 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        34 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        106 as ::core::ffi::c_int as vp8_prob,
        145 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        208 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        179 as ::core::ffi::c_int as vp8_prob,
        121 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        223 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        34 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        179 as ::core::ffi::c_int as vp8_prob,
        121 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
    [
        208 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
        1 as ::core::ffi::c_int as vp8_prob,
    ],
];
fn get_sub_mv_ref_prob(left: uint32_t, above: uint32_t) -> &'static [vp8_prob; 3] {
    let lez = (left == 0) as usize;
    let aez = (above == 0) as usize;
    let lea = (left == above) as usize;
    &vp8_sub_mv_ref_prob3[(aez << 2) | (lez << 1) | lea]
}
fn decode_split_mv(
    safe_decoder: &mut SafeBoolDecoder,
    mi: &mut MODE_INFO,
    left_mb: &MODE_INFO,
    above_mb: &MODE_INFO,
    best_mv: int_mv,
    mvc: &[MV_CONTEXT; 2],
    mb_to_left_edge: ::core::ffi::c_int,
    mb_to_right_edge: ::core::ffi::c_int,
    mb_to_top_edge: ::core::ffi::c_int,
    mb_to_bottom_edge: ::core::ffi::c_int,
) {
    let mut s: ::core::ffi::c_int = 3;
    let mut num_p: ::core::ffi::c_int = 16;
    let mut j: ::core::ffi::c_int = 0;
    if safe_decoder.read_bool(110) != 0 {
        s = 2;
        num_p = 4;
        if safe_decoder.read_bool(111) != 0 {
            s = safe_decoder.read_bool(150);
            num_p = 2;
        }
    }
    loop {
        let mut leftmv: int_mv = int_mv::default();
        let mut abovemv: int_mv = int_mv::default();
        let mut blockmv: int_mv = int_mv::default();
        let k = vp8_mbsplit_offset[s as usize][j as usize] as ::core::ffi::c_int;
        if k & 3 == 0 {
            if left_mb.mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int {
                leftmv = left_mb.mbmi.mv;
            } else {
                leftmv = left_mb.bmi[k as usize + 3].mv();
            }
        } else {
            leftmv = mi.bmi[k as usize - 1].mv();
        }
        if k >> 2 == 0 {
            if above_mb.mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int {
                abovemv = above_mb.mbmi.mv;
            } else {
                abovemv = above_mb.bmi[k as usize + 12].mv();
            }
        } else {
            abovemv = mi.bmi[k as usize - 4].mv();
        }
        let prob = get_sub_mv_ref_prob(leftmv.as_int(), abovemv.as_int());
        if safe_decoder.read_bool(prob[0] as i32) != 0 {
            if safe_decoder.read_bool(prob[1] as i32) != 0 {
                blockmv = int_mv::default();
                if safe_decoder.read_bool(prob[2] as i32) != 0 {
                    blockmv.as_mv_mut().row = (read_mvcomponent(safe_decoder, &mvc[0]) * 2) as ::core::ffi::c_short;
                    blockmv.as_mv_mut().row = (blockmv.as_mv().row as ::core::ffi::c_int + best_mv.as_mv().row as ::core::ffi::c_int) as ::core::ffi::c_short;
                    blockmv.as_mv_mut().col = (read_mvcomponent(safe_decoder, &mvc[1]) * 2) as ::core::ffi::c_short;
                    blockmv.as_mv_mut().col = (blockmv.as_mv().col as ::core::ffi::c_int + best_mv.as_mv().col as ::core::ffi::c_int) as ::core::ffi::c_short;
                }
            } else {
                blockmv = abovemv;
            }
        } else {
            blockmv = leftmv;
        }
        mi.mbmi.need_to_clamp_mvs = (mi.mbmi.need_to_clamp_mvs as ::core::ffi::c_uint
            | vp8_check_mv_bounds(
                &blockmv.as_mv(),
                mb_to_left_edge,
                mb_to_right_edge,
                mb_to_top_edge,
                mb_to_bottom_edge,
            )) as uint8_t;
        let fill_count = mbsplit_fill_count[s as usize] as usize;
        let offset_start = (j as usize) * fill_count;
        let fill_offsets = &mbsplit_fill_offset[s as usize][offset_start..offset_start + fill_count];
        for &idx in fill_offsets {
            mi.bmi[idx as usize].mv = blockmv;
        }
        j += 1;
        if !(j < num_p) {
            break;
        }
    }
    mi.mbmi.partitioning = s as uint8_t;
}
fn read_mb_modes_mv(
    pbi: &VP8D_COMP,
    mip_slice: &mut [MODE_INFO],
    cur_idx: usize,
    safe_decoder: &mut SafeBoolDecoder,
) {
    let mis = pbi.mb.mode_info_stride as usize;
    let (prev_mips, cur_and_rest) = mip_slice.split_at_mut(cur_idx);
    let cur_mi = &mut cur_and_rest[0];
    let above_mi = &prev_mips[prev_mips.len() - mis];
    let left_mi = &prev_mips[prev_mips.len() - 1];
    let aboveleft_mi = &prev_mips[prev_mips.len() - mis - 1];
    let ref_frame_sign_bias = &pbi.common.ref_frame_sign_bias;

    cur_mi.mbmi.ref_frame = safe_decoder.read_bool(pbi.prob_intra as i32) as MV_REFERENCE_FRAME as uint8_t;
    if cur_mi.mbmi.ref_frame != 0 {
        let mut cnt: [::core::ffi::c_int; 4] = [0; 4];
        let mut cntx_idx: usize = 0;
        let mut near_mvs: [int_mv; 4] = [int_mv::default(); 4];
        let mut nmv_idx: usize = 0;

        cur_mi.mbmi.need_to_clamp_mvs = 0;
        if safe_decoder.read_bool(pbi.prob_last as i32) != 0 {
            cur_mi.mbmi.ref_frame = (2 + safe_decoder.read_bool(pbi.prob_gf as i32)) as uint8_t;
        }

        if above_mi.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if above_mi.mbmi.mv.as_int() != 0 {
                nmv_idx += 1;
                near_mvs[nmv_idx] = above_mi.mbmi.mv;
                mv_bias(
                    ref_frame_sign_bias[above_mi.mbmi.ref_frame as usize],
                    cur_mi.mbmi.ref_frame as ::core::ffi::c_int,
                    near_mvs[nmv_idx].as_mv_mut(),
                    ref_frame_sign_bias,
                );
                cntx_idx += 1;
            }
            cnt[cntx_idx] += 2;
        }
        if left_mi.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if left_mi.mbmi.mv.as_int() != 0 {
                let mut this_mv: int_mv = int_mv::default();
                this_mv = left_mi.mbmi.mv;
                mv_bias(
                    ref_frame_sign_bias[left_mi.mbmi.ref_frame as usize],
                    cur_mi.mbmi.ref_frame as ::core::ffi::c_int,
                    this_mv.as_mv_mut(),
                    ref_frame_sign_bias,
                );
                if this_mv.as_int() != near_mvs[nmv_idx].as_int() {
                    nmv_idx += 1;
                    near_mvs[nmv_idx] = this_mv;
                    cntx_idx += 1;
                }
                cnt[cntx_idx] += 2;
            } else {
                cnt[CNT_INTRA as usize] += 2;
            }
        }
        if aboveleft_mi.mbmi.ref_frame as ::core::ffi::c_int != INTRA_FRAME as ::core::ffi::c_int {
            if aboveleft_mi.mbmi.mv.as_int() != 0 {
                let mut this_mv_0: int_mv = int_mv::default();
                this_mv_0 = aboveleft_mi.mbmi.mv;
                mv_bias(
                    ref_frame_sign_bias[aboveleft_mi.mbmi.ref_frame as usize],
                    cur_mi.mbmi.ref_frame as ::core::ffi::c_int,
                    this_mv_0.as_mv_mut(),
                    ref_frame_sign_bias,
                );
                if this_mv_0.as_int() != near_mvs[nmv_idx].as_int() {
                    nmv_idx += 1;
                    near_mvs[nmv_idx] = this_mv_0;
                    cntx_idx += 1;
                }
                cnt[cntx_idx] += 1;
            } else {
                cnt[CNT_INTRA as usize] += 1;
            }
        }
        if safe_decoder.read_bool(
            vp8_mode_contexts[cnt[CNT_INTRA as usize] as usize][0] as i32,
        ) != 0 {
            cnt[CNT_NEAREST as usize] += (cnt[CNT_SPLITMV as usize] > 0) as ::core::ffi::c_int
                & (near_mvs[nmv_idx].as_int() == near_mvs[CNT_NEAREST as usize].as_int()) as ::core::ffi::c_int;
            if cnt[CNT_NEAR as usize] > cnt[CNT_NEAREST as usize] {
                let tmp_cnt = cnt[CNT_NEAREST as usize];
                cnt[CNT_NEAREST as usize] = cnt[CNT_NEAR as usize];
                cnt[CNT_NEAR as usize] = tmp_cnt;
                near_mvs.swap(CNT_NEAREST as usize, CNT_NEAR as usize);
            }
            if safe_decoder.read_bool(
                vp8_mode_contexts[cnt[CNT_NEAREST as usize] as usize][1] as i32,
            ) != 0 {
                if safe_decoder.read_bool(
                    vp8_mode_contexts[cnt[CNT_NEAR as usize] as usize][2] as i32,
                ) != 0 {
                    let mvc = &pbi.common.fc.mvc;
                    let mb_to_top_edge = pbi.mb.mb_to_top_edge - LEFT_TOP_MARGIN;
                    let mb_to_bottom_edge = pbi.mb.mb_to_bottom_edge + RIGHT_BOTTOM_MARGIN;
                    let mb_to_right_edge = pbi.mb.mb_to_right_edge + RIGHT_BOTTOM_MARGIN;
                    let mb_to_left_edge = pbi.mb.mb_to_left_edge - LEFT_TOP_MARGIN;

                    let near_index = CNT_INTRA as usize
                        + (cnt[CNT_NEAREST as usize] >= cnt[CNT_INTRA as usize]) as usize;
                    vp8_clamp_mv2(near_mvs[near_index].as_mv_mut(), &pbi.mb);

                    cnt[CNT_SPLITMV as usize] = ((above_mi.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int
                        + (left_mi.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int) * 2
                        + (aboveleft_mi.mbmi.mode as ::core::ffi::c_int == SPLITMV as ::core::ffi::c_int) as ::core::ffi::c_int;

                    if safe_decoder.read_bool(
                        vp8_mode_contexts[cnt[CNT_SPLITMV as usize] as usize][3] as i32,
                    ) != 0 {
                        decode_split_mv(
                            safe_decoder,
                            cur_mi,
                            left_mi,
                            above_mi,
                            near_mvs[near_index],
                            mvc,
                            mb_to_left_edge,
                            mb_to_right_edge,
                            mb_to_top_edge,
                            mb_to_bottom_edge,
                        );
                        cur_mi.mbmi.mv = cur_mi.bmi[15].mv();
                        cur_mi.mbmi.mode = SPLITMV as uint8_t;
                        cur_mi.mbmi.is_4x4 = 1;
                    } else {
                        read_mv(safe_decoder, cur_mi.mbmi.mv.as_mv_mut(), mvc);
                        cur_mi.mbmi.mv.as_mv_mut().row = (cur_mi.mbmi.mv.as_mv().row as ::core::ffi::c_int + near_mvs[near_index].as_mv().row as ::core::ffi::c_int) as ::core::ffi::c_short;
                        cur_mi.mbmi.mv.as_mv_mut().col = (cur_mi.mbmi.mv.as_mv().col as ::core::ffi::c_int + near_mvs[near_index].as_mv().col as ::core::ffi::c_int) as ::core::ffi::c_short;
                        cur_mi.mbmi.need_to_clamp_mvs = vp8_check_mv_bounds(
                            &cur_mi.mbmi.mv.as_mv(),
                            mb_to_left_edge,
                            mb_to_right_edge,
                            mb_to_top_edge,
                            mb_to_bottom_edge,
                        ) as uint8_t;
                        cur_mi.mbmi.mode = NEWMV as uint8_t;
                    }
                } else {
                    cur_mi.mbmi.mode = NEARMV as uint8_t;
                    cur_mi.mbmi.mv = near_mvs[CNT_NEAR as usize];
                    vp8_clamp_mv2(cur_mi.mbmi.mv.as_mv_mut(), &pbi.mb);
                }
            } else {
                cur_mi.mbmi.mode = NEARESTMV as uint8_t;
                cur_mi.mbmi.mv = near_mvs[CNT_NEAREST as usize];
                vp8_clamp_mv2(cur_mi.mbmi.mv.as_mv_mut(), &pbi.mb);
            }
        } else {
            cur_mi.mbmi.mode = ZEROMV as uint8_t;
            cur_mi.mbmi.mv = int_mv::default();
        }
    } else {
        cur_mi.mbmi.mv = int_mv::default();
        cur_mi.mbmi.mode = read_ymode(safe_decoder, &pbi.common.fc.ymode_prob) as uint8_t;
        if cur_mi.mbmi.mode as ::core::ffi::c_int == B_PRED as ::core::ffi::c_int {
            cur_mi.mbmi.is_4x4 = 1;
            for j in 0..16 {
                cur_mi.bmi[j].set_mode(read_bmode(safe_decoder, &pbi.common.fc.bmode_prob));
            }
        }
        cur_mi.mbmi.uv_mode = read_uv_mode(safe_decoder, &pbi.common.fc.uv_mode_prob) as uint8_t;
    }
}
fn read_mb_features(
    safe_decoder: &mut SafeBoolDecoder,
    mi: &mut MB_MODE_INFO,
    x: &MACROBLOCKD,
) {
    if x.segmentation_enabled as ::core::ffi::c_int != 0
        && x.update_mb_segmentation_map as ::core::ffi::c_int != 0
    {
        if safe_decoder.read_bool(x.mb_segment_tree_probs[0] as i32) != 0 {
            mi.segment_id = (2 + safe_decoder.read_bool(x.mb_segment_tree_probs[2] as i32)) as uint8_t;
        } else {
            mi.segment_id = safe_decoder.read_bool(x.mb_segment_tree_probs[1] as i32) as uint8_t;
        }
    }
}
fn decode_mb_mode_mvs(
    pbi: &VP8D_COMP,
    mip_slice: &mut [MODE_INFO],
    cur_idx: usize,
    safe_decoder: &mut SafeBoolDecoder,
) {
    if pbi.mb.update_mb_segmentation_map != 0 {
        read_mb_features(
            safe_decoder,
            &mut mip_slice[cur_idx].mbmi,
            &pbi.mb,
        );
    } else if pbi.common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mip_slice[cur_idx].mbmi.segment_id = 0 as uint8_t;
    }
    if pbi.common.mb_no_coeff_skip != 0 {
        mip_slice[cur_idx].mbmi.mb_skip_coeff = safe_decoder.read_bool(pbi.prob_skip_false as i32) as uint8_t;
    } else {
        mip_slice[cur_idx].mbmi.mb_skip_coeff = 0 as uint8_t;
    }
    mip_slice[cur_idx].mbmi.is_4x4 = 0 as uint8_t;
    if pbi.common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        read_kf_modes(pbi.common.mode_info_stride as usize, mip_slice, cur_idx, safe_decoder);
    } else {
        read_mb_modes_mv(pbi, mip_slice, cur_idx, safe_decoder);
    };
}
pub fn vp8_decode_mode_mvs(
    pbi: &mut VP8D_COMP,
    mip_slice: &mut [MODE_INFO],
    safe_decoder: &mut SafeBoolDecoder,
) {
    let stride = pbi.common.mode_info_stride as usize;
    let mut cur_idx = stride + 1;

    let mut mb_row: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut mb_to_right_edge_start: ::core::ffi::c_int = 0;

    mb_mode_mv_init(pbi, safe_decoder);
    pbi.mb.mb_to_top_edge = 0 as ::core::ffi::c_int;
    pbi.mb.mb_to_bottom_edge = ((pbi.common.mb_rows - 1 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int;
    mb_to_right_edge_start = ((pbi.common.mb_cols - 1 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int;
    loop {
        mb_row += 1;
        if !(mb_row < pbi.common.mb_rows) {
            break;
        }
        let mut mb_col: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        pbi.mb.mb_to_left_edge = 0 as ::core::ffi::c_int;
        pbi.mb.mb_to_right_edge = mb_to_right_edge_start;
        loop {
            mb_col += 1;
            if !(mb_col < pbi.common.mb_cols) {
                break;
            }
            decode_mb_mode_mvs(&*pbi, mip_slice, cur_idx, safe_decoder);
            pbi.mb.mb_to_left_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            pbi.mb.mb_to_right_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            cur_idx += 1;
        }
        pbi.mb.mb_to_top_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
        pbi.mb.mb_to_bottom_edge -= (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
        cur_idx += 1;
    }
}
