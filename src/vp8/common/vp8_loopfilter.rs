pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type uint8_t = u8;
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = u32;
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
    pub mblim: *const u8,
    pub blim: *const u8,
    pub lim: *const u8,
    pub hev_thr: *const u8,
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
pub type C2RustUnnamed = u32;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub type C2RustUnnamed_0 = u32;
pub const MB_LVL_MAX: C2RustUnnamed_0 = 2;
pub const MB_LVL_ALT_Q: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = u32;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const MAX_LOOP_FILTER: i32 = 63_i32;
pub const PARTIAL_FRAME_FRACTION: i32 = 8_i32;
pub const SIMD_WIDTH: i32 = 1_i32;
pub const MAX_MB_SEGMENTS: i32 = 4_i32;
pub const SEGMENT_ABSDATA: i32 = 1_i32;
fn lf_init_lut(lfi: &mut loop_filter_info_n) {
    let mut filt_lvl: i32 = 0;
    filt_lvl = 0_i32;
    while filt_lvl <= MAX_LOOP_FILTER {
        if filt_lvl >= 40_i32 {
            lfi.hev_thr_lut[KEY_FRAME as i32 as usize][filt_lvl as usize] = 2_u8;
            lfi.hev_thr_lut[INTER_FRAME as i32 as usize][filt_lvl as usize] = 3_u8;
        } else if filt_lvl >= 20_i32 {
            lfi.hev_thr_lut[KEY_FRAME as i32 as usize][filt_lvl as usize] = 1_u8;
            lfi.hev_thr_lut[INTER_FRAME as i32 as usize][filt_lvl as usize] = 2_u8;
        } else if filt_lvl >= 15_i32 {
            lfi.hev_thr_lut[KEY_FRAME as i32 as usize][filt_lvl as usize] = 1_u8;
            lfi.hev_thr_lut[INTER_FRAME as i32 as usize][filt_lvl as usize] = 1_u8;
        } else {
            lfi.hev_thr_lut[KEY_FRAME as i32 as usize][filt_lvl as usize] = 0_u8;
            lfi.hev_thr_lut[INTER_FRAME as i32 as usize][filt_lvl as usize] = 0_u8;
        }
        filt_lvl += 1;
    }
    lfi.mode_lf_lut[DC_PRED as i32 as usize] = 1_u8;
    lfi.mode_lf_lut[V_PRED as i32 as usize] = 1_u8;
    lfi.mode_lf_lut[H_PRED as i32 as usize] = 1_u8;
    lfi.mode_lf_lut[TM_PRED as i32 as usize] = 1_u8;
    lfi.mode_lf_lut[B_PRED as i32 as usize] = 0_u8;
    lfi.mode_lf_lut[ZEROMV as i32 as usize] = 1_u8;
    lfi.mode_lf_lut[NEARESTMV as i32 as usize] = 2_u8;
    lfi.mode_lf_lut[NEARMV as i32 as usize] = 2_u8;
    lfi.mode_lf_lut[NEWMV as i32 as usize] = 2_u8;
    lfi.mode_lf_lut[SPLITMV as i32 as usize] = 3_u8;
}
pub fn vp8_loop_filter_update_sharpness(lfi: &mut loop_filter_info_n, sharpness_lvl: i32) {
    let mut i: i32 = 0;
    i = 0_i32;
    while i <= MAX_LOOP_FILTER {
        let mut filt_lvl: i32 = i;
        let mut block_inside_limit: i32 = 0_i32;
        block_inside_limit = filt_lvl >> (sharpness_lvl > 0_i32) as i32;
        block_inside_limit >>= (sharpness_lvl > 4_i32) as i32;
        if sharpness_lvl > 0_i32 && block_inside_limit > 9_i32 - sharpness_lvl {
            block_inside_limit = 9_i32 - sharpness_lvl;
        }
        if block_inside_limit < 1_i32 {
            block_inside_limit = 1_i32;
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
pub fn vp8_loop_filter_frame_init(cm: &mut VP8_COMMON, mbd: &MACROBLOCKD, default_filt_lvl: i32) {
    let mut seg: i32 = 0;
    let mut ref_0: i32 = 0;
    let mut mode: i32 = 0;
    let lfi: &mut loop_filter_info_n = &mut cm.lf_info;
    if cm.last_sharpness_level != cm.sharpness_level {
        vp8_loop_filter_update_sharpness(lfi, cm.sharpness_level);
        cm.last_sharpness_level = cm.sharpness_level;
    }
    seg = 0_i32;
    while seg < MAX_MB_SEGMENTS {
        let mut lvl_seg: i32 = default_filt_lvl;
        let mut lvl_ref: i32 = 0;
        let mut lvl_mode: i32 = 0;
        if mbd.segmentation_enabled != 0 {
            if mbd.mb_segment_abs_delta as i32 == SEGMENT_ABSDATA {
                lvl_seg =
                    mbd.segment_feature_data[MB_LVL_ALT_LF as i32 as usize][seg as usize] as i32;
            } else {
                lvl_seg +=
                    mbd.segment_feature_data[MB_LVL_ALT_LF as i32 as usize][seg as usize] as i32;
            }
            lvl_seg = if lvl_seg > 0_i32 {
                if lvl_seg > 63_i32 { 63_i32 } else { lvl_seg }
            } else {
                0_i32
            };
        }
        if mbd.mode_ref_lf_delta_enabled == 0 {
            lfi.lvl[seg as usize] = [[lvl_seg as u8; 4]; 4];
        } else {
            ref_0 = INTRA_FRAME as i32;
            lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as i32;
            mode = 0_i32;
            lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as i32;
            lvl_mode = if lvl_mode > 0_i32 {
                if lvl_mode > 63_i32 { 63_i32 } else { lvl_mode }
            } else {
                0_i32
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
            mode = 1_i32;
            lvl_mode = if lvl_ref > 0_i32 {
                if lvl_ref > 63_i32 { 63_i32 } else { lvl_ref }
            } else {
                0_i32
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
            ref_0 = 1_i32;
            while ref_0 < MAX_REF_FRAMES as i32 {
                lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as i32;
                mode = 1_i32;
                while mode < 4_i32 {
                    lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as i32;
                    lvl_mode = if lvl_mode > 0_i32 {
                        if lvl_mode > 63_i32 { 63_i32 } else { lvl_mode }
                    } else {
                        0_i32
                    };
                    lfi.lvl[seg as usize][ref_0 as usize][mode as usize] = lvl_mode as u8;
                    mode += 1;
                }
                ref_0 += 1;
            }
        }
        seg += 1;
    }
}
pub fn vp8_loop_filter_row_normal_safe(
    mb_cols: i32,
    lf_info: &loop_filter_info_n,
    frame_type: FRAME_TYPE,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: i32,
    post_ystride: i32,
    post_uvstride: i32,
    y_slice: &mut [u8],
    y_offset: usize,
    mut u_slice: Option<&mut [u8]>,
    u_offset: usize,
    mut v_slice: Option<&mut [u8]>,
    v_offset: usize,
) {
    let mut mb_col: i32 = 0;
    let mut filter_level: i32 = 0;
    let lfi_n = lf_info;
    mb_col = 0_i32;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;
    let mut cur_u_offset = u_offset;
    let mut cur_v_offset = v_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let skip_lf: i32 = (mi.mbmi.mode as i32 != B_PRED as i32
            && mi.mbmi.mode as i32 != SPLITMV as i32
            && mi.mbmi.mb_skip_coeff as i32 != 0) as i32;
        let mode_index: i32 = lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as i32;
        let seg: i32 = mi.mbmi.segment_id as i32;
        let ref_frame: i32 = mi.mbmi.ref_frame as i32;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;

        if filter_level != 0 {
            let hev_index: i32 =
                lfi_n.hev_thr_lut[frame_type as usize][filter_level as usize] as i32;

            let mblim_slice = &lfi_n.mblim[filter_level as usize];
            let blim_slice = &lfi_n.blim[filter_level as usize];
            let lim_slice = &lfi_n.lim[filter_level as usize];
            let thresh_slice = &lfi_n.hev_thr[hev_index as usize];

            let y_stride_usize = post_ystride as usize;
            let uv_stride_usize = post_uvstride as usize;

            if mb_col > 0_i32 {
                crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                if let (Some(u), Some(v)) = (u_slice.as_deref_mut(), v_slice.as_deref_mut()) {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_uv_safe(
                        u,
                        cur_u_offset,
                        v,
                        cur_v_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
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

                if let (Some(u), Some(v)) = (u_slice.as_deref_mut(), v_slice.as_deref_mut()) {
                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_uv_safe(
                        u,
                        cur_u_offset + 4,
                        v,
                        cur_v_offset + 4,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                    );
                }
            }

            if mb_row > 0_i32 {
                crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(
                    y_slice,
                    cur_y_offset,
                    y_stride_usize,
                    mblim_slice,
                    lim_slice,
                    thresh_slice,
                    2,
                );
                if let (Some(u), Some(v)) = (u_slice.as_deref_mut(), v_slice.as_deref_mut()) {
                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_uv_safe(
                        u,
                        cur_u_offset,
                        v,
                        cur_v_offset,
                        uv_stride_usize,
                        mblim_slice,
                        lim_slice,
                        thresh_slice,
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

                if let (Some(u), Some(v)) = (u_slice.as_deref_mut(), v_slice.as_deref_mut()) {
                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_uv_safe(
                        u,
                        cur_u_offset + 4 * uv_stride_usize,
                        v,
                        cur_v_offset + 4 * uv_stride_usize,
                        uv_stride_usize,
                        blim_slice,
                        lim_slice,
                        thresh_slice,
                    );
                }
            }
        }
        cur_y_offset = (cur_y_offset as isize + 16_isize) as usize;
        cur_u_offset = (cur_u_offset as isize + 8_isize) as usize;
        cur_v_offset = (cur_v_offset as isize + 8_isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
pub fn vp8_loop_filter_row_simple_safe(
    mb_cols: i32,
    lf_info: &loop_filter_info_n,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: i32,
    post_ystride: i32,
    y_slice: &mut [u8],
    y_offset: usize,
) {
    let mut mb_col: i32 = 0;
    let mut filter_level: i32 = 0;
    let lfi_n = lf_info;
    mb_col = 0_i32;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let mut skip_lf: i32 = (mi.mbmi.mode as i32 != B_PRED as i32
            && mi.mbmi.mode as i32 != SPLITMV as i32
            && mi.mbmi.mb_skip_coeff as i32 != 0) as i32;
        let mode_index: i32 = lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as i32;
        let seg: i32 = mi.mbmi.segment_id as i32;
        let ref_frame: i32 = mi.mbmi.ref_frame as i32;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize] as i32;
        if filter_level != 0 {
            let mblim_val = lfi_n.mblim[filter_level as usize][0];
            let blim_val = lfi_n.blim[filter_level as usize][0];
            let y_stride_usize = post_ystride as usize;

            if mb_col > 0_i32 {
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
            if mb_row > 0_i32 {
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
        cur_y_offset = (cur_y_offset as isize + 16_isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
pub fn vp8_loop_filter_frame_safe(cm: &mut VP8_COMMON, mbd: &MACROBLOCKD, frame_type: FRAME_TYPE) {
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

    if cm.filter_type == NORMAL_LOOPFILTER as i32 as u32 {
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
    default_filt_lvl: i32,
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

    if cm.filter_type == NORMAL_LOOPFILTER as i32 as u32 {
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
    default_filt_lvl: i32,
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

    if cm.filter_type == NORMAL_LOOPFILTER as i32 as u32 {
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
