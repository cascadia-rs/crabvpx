
pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type uint8_t = u8;
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
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
pub const MAX_REF_FRAMES: C2RustUnnamed_1 = 4;
pub const INTRA_FRAME: C2RustUnnamed_1 = 0;
pub const MB_LVL_ALT_LF: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MB_LVL_MAX: C2RustUnnamed_0 = 2;
pub const MB_LVL_ALT_Q: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const MAX_LOOP_FILTER: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const PARTIAL_FRAME_FRACTION: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIMD_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_MB_SEGMENTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEGMENT_ABSDATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
fn lf_init_lut(lfi: &mut loop_filter_info_n) {
    let mut filt_lvl: ::core::ffi::c_int = 0;
    filt_lvl = 0 as ::core::ffi::c_int;
    while filt_lvl <= MAX_LOOP_FILTER {
        if filt_lvl >= 40 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                2 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                3 as ::core::ffi::c_uchar;
        } else if filt_lvl >= 20 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                2 as ::core::ffi::c_uchar;
        } else if filt_lvl >= 15 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
        } else {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                0 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                0 as ::core::ffi::c_uchar;
        }
        filt_lvl += 1;
    }
    lfi.mode_lf_lut[DC_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[V_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[H_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[TM_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[B_PRED as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[ZEROMV as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEARESTMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEARMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEWMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[SPLITMV as ::core::ffi::c_int as usize] = 3 as ::core::ffi::c_uchar;
}
pub fn vp8_loop_filter_update_sharpness(
    lfi: &mut loop_filter_info_n,
    sharpness_lvl: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= MAX_LOOP_FILTER {
        let mut filt_lvl: ::core::ffi::c_int = i;
        let mut block_inside_limit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        block_inside_limit =
            filt_lvl >> (sharpness_lvl > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        block_inside_limit =
            block_inside_limit >> (sharpness_lvl > 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if sharpness_lvl > 0 as ::core::ffi::c_int {
            if block_inside_limit > 9 as ::core::ffi::c_int - sharpness_lvl {
                block_inside_limit = 9 as ::core::ffi::c_int - sharpness_lvl;
            }
        }
        if block_inside_limit < 1 as ::core::ffi::c_int {
            block_inside_limit = 1 as ::core::ffi::c_int;
        }
        lfi.lim[i as usize] = [block_inside_limit as u8; 1];
        lfi.blim[i as usize] = [(2 * filt_lvl + block_inside_limit) as u8; 1];
        lfi.mblim[i as usize] = [(2 * (filt_lvl + 2) + block_inside_limit) as u8; 1];
        i += 1;
    }
}
pub fn vp8_loop_filter_init(cm: &mut VP8_COMMON) {
    vp8_loop_filter_update_sharpness(&mut cm.lf_info, cm.sharpness_level);
    cm.last_sharpness_level = cm.sharpness_level;
    lf_init_lut(&mut cm.lf_info);
    for i in 0..4 {
        cm.lf_info.hev_thr[i] = [i as u8; 1];
    }
}
pub fn vp8_loop_filter_frame_init(
    cm: &mut VP8_COMMON,
    mbd: &MACROBLOCKD,
    default_filt_lvl: ::core::ffi::c_int,
) {
    let mut seg: ::core::ffi::c_int = 0;
    let mut ref_0: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let lfi: &mut loop_filter_info_n = &mut cm.lf_info;
    if cm.last_sharpness_level != cm.sharpness_level {
        vp8_loop_filter_update_sharpness(lfi, cm.sharpness_level);
        cm.last_sharpness_level = cm.sharpness_level;
    }
    seg = 0 as ::core::ffi::c_int;
    while seg < MAX_MB_SEGMENTS {
        let mut lvl_seg: ::core::ffi::c_int = default_filt_lvl;
        let mut lvl_ref: ::core::ffi::c_int = 0;
        let mut lvl_mode: ::core::ffi::c_int = 0;
        if mbd.segmentation_enabled != 0 {
            if mbd.mb_segment_abs_delta as ::core::ffi::c_int == SEGMENT_ABSDATA {
                lvl_seg = mbd.segment_feature_data[MB_LVL_ALT_LF as ::core::ffi::c_int as usize]
                    [seg as usize] as ::core::ffi::c_int;
            } else {
                lvl_seg += mbd.segment_feature_data[MB_LVL_ALT_LF as ::core::ffi::c_int as usize]
                    [seg as usize] as ::core::ffi::c_int;
            }
            lvl_seg = if lvl_seg > 0 as ::core::ffi::c_int {
                if lvl_seg > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_seg
                }
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if mbd.mode_ref_lf_delta_enabled == 0 {
            lfi.lvl[seg as usize] = [[lvl_seg as u8; 4]; 4];
        } else {
            ref_0 = INTRA_FRAME as ::core::ffi::c_int;
            lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
            mode = 0 as ::core::ffi::c_int;
            lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
            lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                if lvl_mode > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_mode
                }
            } else {
                0 as ::core::ffi::c_int
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                lvl_mode as ::core::ffi::c_uchar;
            mode = 1 as ::core::ffi::c_int;
            lvl_mode = if lvl_ref > 0 as ::core::ffi::c_int {
                if lvl_ref > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_ref
                }
            } else {
                0 as ::core::ffi::c_int
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                lvl_mode as ::core::ffi::c_uchar;
            ref_0 = 1 as ::core::ffi::c_int;
            while ref_0 < MAX_REF_FRAMES as ::core::ffi::c_int {
                lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
                mode = 1 as ::core::ffi::c_int;
                while mode < 4 as ::core::ffi::c_int {
                    lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
                    lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                        if lvl_mode > 63 as ::core::ffi::c_int {
                            63 as ::core::ffi::c_int
                        } else {
                            lvl_mode
                        }
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                        lvl_mode as ::core::ffi::c_uchar;
                    mode += 1;
                }
                ref_0 += 1;
            }
        }
        seg += 1;
    }
}
pub fn vp8_loop_filter_row_normal_safe(
    mb_cols: ::core::ffi::c_int,
    lf_info: &loop_filter_info_n,
    frame_type: FRAME_TYPE,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: ::core::ffi::c_int,
    post_ystride: ::core::ffi::c_int,
    post_uvstride: ::core::ffi::c_int,
    y_slice: &mut [u8],
    y_offset: usize,
    mut u_slice: Option<&mut [u8]>,
    u_offset: usize,
    mut v_slice: Option<&mut [u8]>,
    v_offset: usize,
) {
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut filter_level: ::core::ffi::c_int = 0;
    let lfi_n = lf_info;
    mb_col = 0 as ::core::ffi::c_int;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;
    let mut cur_u_offset = u_offset;
    let mut cur_v_offset = v_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let skip_lf: ::core::ffi::c_int = (mi.mbmi.mode as ::core::ffi::c_int
            != B_PRED as ::core::ffi::c_int
            && mi.mbmi.mode as ::core::ffi::c_int
                != SPLITMV as ::core::ffi::c_int
            && mi.mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        let mode_index: ::core::ffi::c_int =
            lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as ::core::ffi::c_int;
        let seg: ::core::ffi::c_int = mi.mbmi.segment_id as ::core::ffi::c_int;
        let ref_frame: ::core::ffi::c_int =
            mi.mbmi.ref_frame as ::core::ffi::c_int;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize]
            as ::core::ffi::c_int;

        if filter_level != 0 {
            let hev_index: ::core::ffi::c_int = lfi_n.hev_thr_lut[frame_type as usize]
                [filter_level as usize]
                as ::core::ffi::c_int;
            
            let mblim_slice = &lfi_n.mblim[filter_level as usize];
            let blim_slice = &lfi_n.blim[filter_level as usize];
            let lim_slice = &lfi_n.lim[filter_level as usize];
            let thresh_slice = &lfi_n.hev_thr[hev_index as usize];

            let y_stride_usize = post_ystride as usize;
            let uv_stride_usize = post_uvstride as usize;

            if mb_col > 0 as ::core::ffi::c_int {
                crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                if let Some(u) = u_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(
                        u,
                        cur_u_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
                if let Some(v) = v_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(
                        v,
                        cur_v_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
            }
            
            if skip_lf == 0 {
                crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(
                    y_slice,
                    cur_y_offset + 4,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(
                    y_slice,
                    cur_y_offset + 8,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(
                    y_slice,
                    cur_y_offset + 12,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );

                if let Some(u) = u_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(
                        u,
                        cur_u_offset + 4,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
                if let Some(v) = v_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(
                        v,
                        cur_v_offset + 4,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
            }
            
            if mb_row > 0 as ::core::ffi::c_int {
                crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                if let Some(u) = u_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(
                        u,
                        cur_u_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
                if let Some(v) = v_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(
                        v,
                        cur_v_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
            }
            
            if skip_lf == 0 {
                crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset + 4 * y_stride_usize,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset + 8 * y_stride_usize,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset + 12 * y_stride_usize,
                    y_stride_usize,
                    blim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );

                if let Some(u) = u_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(
                        u,
                        cur_u_offset + 4 * uv_stride_usize,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
                if let Some(v) = v_slice.as_deref_mut() {
                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(
                        v,
                        cur_v_offset + 4 * uv_stride_usize,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                        1,
                    );
                }
            }
        }
        cur_y_offset = (cur_y_offset as isize + 16 as isize) as usize;
        cur_u_offset = (cur_u_offset as isize + 8 as isize) as usize;
        cur_v_offset = (cur_v_offset as isize + 8 as isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
pub fn vp8_loop_filter_row_simple_safe(
    mb_cols: ::core::ffi::c_int,
    lf_info: &loop_filter_info_n,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: ::core::ffi::c_int,
    post_ystride: ::core::ffi::c_int,
    y_slice: &mut [u8],
    y_offset: usize,
) {
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut filter_level: ::core::ffi::c_int = 0;
    let lfi_n = lf_info;
    mb_col = 0 as ::core::ffi::c_int;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let mut skip_lf: ::core::ffi::c_int = (mi.mbmi.mode as ::core::ffi::c_int
            != B_PRED as ::core::ffi::c_int
            && mi.mbmi.mode as ::core::ffi::c_int
                != SPLITMV as ::core::ffi::c_int
            && mi.mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        let mode_index: ::core::ffi::c_int =
            lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as ::core::ffi::c_int;
        let seg: ::core::ffi::c_int = mi.mbmi.segment_id as ::core::ffi::c_int;
        let ref_frame: ::core::ffi::c_int =
            mi.mbmi.ref_frame as ::core::ffi::c_int;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize]
            as ::core::ffi::c_int;
        if filter_level != 0 {
            let mblim_val = lfi_n.mblim[filter_level as usize][0];
            let blim_val = lfi_n.blim[filter_level as usize][0];
            let y_stride_usize = post_ystride as usize;

            if mb_col > 0 as ::core::ffi::c_int {
                crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_vertical_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_val,
                );
            }
            if skip_lf == 0 {
                crate::vp8::common::loopfilter_filters::vp8_loop_filter_bvs_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    blim_val,
                );
            }
            if mb_row > 0 as ::core::ffi::c_int {
                crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_val,
                );
            }
            if skip_lf == 0 {
                crate::vp8::common::loopfilter_filters::vp8_loop_filter_bhs_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    blim_val,
                );
            }
        }
        cur_y_offset = (cur_y_offset as isize + 16 as isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
pub fn vp8_loop_filter_frame_safe(
    cm: &mut VP8_COMMON,
    mbd: &MACROBLOCKD,
    frame_type: FRAME_TYPE,
) {
    let filter_level = cm.filter_level;
    vp8_loop_filter_frame_init(cm, mbd, filter_level);

    if filter_level == 0 {
        return;
    }

    let post_idx = cm.frame_to_show_idx.expect("frame_to_show_idx is None");
    
    let post_ystride = cm.yv12_fb[post_idx].y_stride;
    let post_uvstride = cm.yv12_fb[post_idx].uv_stride;
    let mb_rows = cm.mb_rows;
    let mb_cols = cm.mb_cols;
    let mode_info_stride = cm.mode_info_stride as usize;
    
    let mip_slice = cm.mip.as_ref().unwrap();
    
    let (y_slice, u_slice, v_slice) = cm.yv12_fb[post_idx].views_mut();

    if cm.filter_type as ::core::ffi::c_uint
        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        for mb_row in 0..mb_rows {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            let u_offset = mb_row as usize * 8 * post_uvstride as usize;
            let v_offset = mb_row as usize * 8 * post_uvstride as usize;
            
            vp8_loop_filter_row_normal_safe(
                mb_cols,
                &cm.lf_info,
                frame_type,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                post_uvstride,
                y_slice,
                y_offset,
                Some(&mut *u_slice),
                u_offset,
                Some(&mut *v_slice),
                v_offset,
            );
        }
    } else {
        for mb_row in 0..mb_rows {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            
            vp8_loop_filter_row_simple_safe(
                mb_cols,
                &cm.lf_info,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                y_slice,
                y_offset,
            );
        }
    }
}

pub fn vp8_loop_filter_frame_yonly_safe(
    cm: &mut VP8_COMMON,
    mbd: &MACROBLOCKD,
    default_filt_lvl: ::core::ffi::c_int,
) {
    let filter_level = default_filt_lvl;
    vp8_loop_filter_frame_init(cm, mbd, filter_level);

    if filter_level == 0 {
        return;
    }

    let post_idx = cm.frame_to_show_idx.expect("frame_to_show_idx is None");
    let post_ystride = cm.yv12_fb[post_idx].y_stride;
    let mb_rows = cm.mb_rows;
    let mb_cols = cm.mb_cols;
    let mode_info_stride = cm.mode_info_stride as usize;
    
    let mip_slice = cm.mip.as_ref().unwrap();
    
    let (y_slice, _, _) = cm.yv12_fb[post_idx].views_mut();
    let frame_type = cm.frame_type;

    if cm.filter_type as ::core::ffi::c_uint
        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        for mb_row in 0..mb_rows {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            
            vp8_loop_filter_row_normal_safe(
                mb_cols,
                &cm.lf_info,
                frame_type,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                0,
                y_slice,
                y_offset,
                None,
                0,
                None,
                0,
            );
        }
    } else {
        for mb_row in 0..mb_rows {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            
            vp8_loop_filter_row_simple_safe(
                mb_cols,
                &cm.lf_info,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                y_slice,
                y_offset,
            );
        }
    }
}

pub fn vp8_loop_filter_partial_frame_safe(
    cm: &mut VP8_COMMON,
    mbd: &MACROBLOCKD,
    default_filt_lvl: ::core::ffi::c_int,
) {
    let filter_level = default_filt_lvl;
    vp8_loop_filter_frame_init(cm, mbd, filter_level);

    if filter_level == 0 {
        return;
    }

    let post_idx = cm.frame_to_show_idx.expect("frame_to_show_idx is None");
    let post_ystride = cm.yv12_fb[post_idx].y_stride;
    let mb_rows = cm.mb_rows;
    let mb_cols = cm.mb_cols;
    let mode_info_stride = cm.mode_info_stride as usize;
    
    let mip_slice = cm.mip.as_ref().unwrap();
    
    let (y_slice, _, _) = cm.yv12_fb[post_idx].views_mut();
    let frame_type = cm.frame_type;

    let mut linestocopy = mb_rows / PARTIAL_FRAME_FRACTION;
    if linestocopy == 0 {
        linestocopy = 1;
    }
    
    let start_mb_row = mb_rows / 2;
    let mut end_mb_row = start_mb_row + linestocopy;
    if end_mb_row > mb_rows {
        end_mb_row = mb_rows;
    }

    if cm.filter_type as ::core::ffi::c_uint
        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        for mb_row in start_mb_row..end_mb_row {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            
            vp8_loop_filter_row_normal_safe(
                mb_cols,
                &cm.lf_info,
                frame_type,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                0,
                y_slice,
                y_offset,
                None,
                0,
                None,
                0,
            );
        }
    } else {
        for mb_row in start_mb_row..end_mb_row {
            let mode_info_idx = mb_row as usize * mode_info_stride + 1;
            let y_offset = mb_row as usize * 16 * post_ystride as usize;
            
            vp8_loop_filter_row_simple_safe(
                mb_cols,
                &cm.lf_info,
                mip_slice,
                mode_info_idx,
                mb_row,
                post_ystride,
                y_slice,
                y_offset,
            );
        }
    }
}

