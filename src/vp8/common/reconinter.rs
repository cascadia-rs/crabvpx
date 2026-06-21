//! Inter prediction (motion compensation) — port of `vp8/common/reconinter.c`.
//!
//! Builds the predicted MB from reference frames: full-pel block copies and
//! sub-pel interpolation (the sixtap/bilinear kernels in safe_predict / simd).

pub use crate::vp8::common::types::*;

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
pub const CHAR_BIT: i32 = 8_i32;
/// `vp8_copy_mem16x16_c` — vp8/common/reconinter.c:23. 16x16 full-pel block copy.
pub fn vp8_copy_mem16x16_safe(src: &[u8], src_stride: i32, dst: &mut [u8], dst_stride: i32) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..16 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 16].copy_from_slice(&src[src_idx..src_idx + 16]);
    }
}

/// `vp8_copy_mem8x8_c` — vp8/common/reconinter.c:35. 8x8 full-pel block copy.
pub fn vp8_copy_mem8x8_safe(src: &[u8], src_stride: i32, dst: &mut [u8], dst_stride: i32) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..8 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 8].copy_from_slice(&src[src_idx..src_idx + 8]);
    }
}

/// `vp8_copy_mem8x4_c` — vp8/common/reconinter.c:47. 8x4 full-pel block copy.
pub fn vp8_copy_mem8x4_safe(src: &[u8], src_stride: i32, dst: &mut [u8], dst_stride: i32) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..4 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 8].copy_from_slice(&src[src_idx..src_idx + 8]);
    }
}

fn call_subpixel_predict(
    sppf: vp8_subpix_fn_t,
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    xoffset: i32,
    yoffset: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
) {
    if let Some(f) = sppf {
        f(
            src, src_offset, src_stride, xoffset, yoffset, dst, dst_offset, dst_stride,
        );
    }
}

fn build_inter_predictors4b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: i32,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;

    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;

    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;

    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;

    if row & 7 != 0 || col & 7 != 0 {
        call_subpixel_predict(
            sppf,
            pre_slice,
            pre_idx,
            pre_stride,
            col & 7,
            row & 7,
            dst_slice,
            dst_idx,
            dst_stride,
        );
    } else {
        let pre_len = 7 * pre_stride_u + 8;
        let dst_len = 7 * dst_stride_u + 8;

        let sub_pre = &pre_slice[pre_idx..pre_idx + pre_len];
        let sub_dst = &mut dst_slice[dst_idx..dst_idx + dst_len];

        vp8_copy_mem8x8_safe(sub_pre, pre_stride, sub_dst, dst_stride);
    }
}

fn build_inter_predictors2b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: i32,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;

    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;

    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;

    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;

    if row & 7 != 0 || col & 7 != 0 {
        call_subpixel_predict(
            sppf,
            pre_slice,
            pre_idx,
            pre_stride,
            col & 7,
            row & 7,
            dst_slice,
            dst_idx,
            dst_stride,
        );
    } else {
        let pre_len = 3 * pre_stride_u + 8;
        let dst_len = 3 * dst_stride_u + 8;

        let sub_pre = &pre_slice[pre_idx..pre_idx + pre_len];
        let sub_dst = &mut dst_slice[dst_idx..dst_idx + dst_len];

        vp8_copy_mem8x4_safe(sub_pre, pre_stride, sub_dst, dst_stride);
    }
}

fn build_inter_predictors_b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: i32,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;

    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;

    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;

    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;

    if row & 7 != 0 || col & 7 != 0 {
        call_subpixel_predict(
            sppf,
            pre_slice,
            pre_idx,
            pre_stride,
            col & 7,
            row & 7,
            dst_slice,
            dst_idx,
            dst_stride,
        );
    } else {
        for r in 0..4 {
            let s_idx = pre_idx + r * pre_stride_u;
            let d_idx = dst_idx + r * dst_stride_u;
            dst_slice[d_idx..d_idx + 4].copy_from_slice(&pre_slice[s_idx..s_idx + 4]);
        }
    }
}
fn clamp_mv_to_umv_border(
    mv: &mut MV,
    mb_to_left_edge: i32,
    mb_to_right_edge: i32,
    mb_to_top_edge: i32,
    mb_to_bottom_edge: i32,
) {
    if (mv.col as i32) < mb_to_left_edge - (19_i32 << 3_i32) {
        mv.col = (mb_to_left_edge - (16_i32 << 3_i32)) as i16;
    } else if mv.col as i32 > mb_to_right_edge + (18_i32 << 3_i32) {
        mv.col = (mb_to_right_edge + (16_i32 << 3_i32)) as i16;
    }
    if (mv.row as i32) < mb_to_top_edge - (19_i32 << 3_i32) {
        mv.row = (mb_to_top_edge - (16_i32 << 3_i32)) as i16;
    } else if mv.row as i32 > mb_to_bottom_edge + (18_i32 << 3_i32) {
        mv.row = (mb_to_bottom_edge + (16_i32 << 3_i32)) as i16;
    }
}
fn clamp_uvmv_to_umv_border(
    mv: &mut MV,
    mb_to_left_edge: i32,
    mb_to_right_edge: i32,
    mb_to_top_edge: i32,
    mb_to_bottom_edge: i32,
) {
    mv.col = (if (2_i32 * mv.col as i32) < mb_to_left_edge - (19_i32 << 3_i32) {
        (mb_to_left_edge - (16_i32 << 3_i32)) >> 1_i32
    } else {
        mv.col as i32
    }) as i16;
    mv.col = (if 2_i32 * mv.col as i32 > mb_to_right_edge + (18_i32 << 3_i32) {
        (mb_to_right_edge + (16_i32 << 3_i32)) >> 1_i32
    } else {
        mv.col as i32
    }) as i16;
    mv.row = (if (2_i32 * mv.row as i32) < mb_to_top_edge - (19_i32 << 3_i32) {
        (mb_to_top_edge - (16_i32 << 3_i32)) >> 1_i32
    } else {
        mv.row as i32
    }) as i16;
    mv.row = (if 2_i32 * mv.row as i32 > mb_to_bottom_edge + (18_i32 << 3_i32) {
        (mb_to_bottom_edge + (16_i32 << 3_i32)) >> 1_i32
    } else {
        mv.row as i32
    }) as i16;
}
/// `vp8_build_inter16x16_predictors_mb` — vp8/common/reconinter.c:297. Builds the
/// 16x16 luma + 8x8 chroma inter predictors for a whole-MB motion vector.
pub fn vp8_build_inter16x16_predictors_mb(
    x: &mut MACROBLOCKD,
    mi: &MODE_INFO,
    mut dst_y_slice: &mut [u8],
    mut dst_u_slice: &mut [u8],
    mut dst_v_slice: &mut [u8],
    pre_y_slice: &[u8],
    pre_u_slice: &[u8],
    pre_v_slice: &[u8],
) {
    let dst_y_stride = x.dst_y_stride;
    let dst_uv_stride = x.dst_uv_stride;
    let pre_y_stride = x.pre_y_stride;
    let pre_uv_stride = x.pre_uv_stride;

    let border = x.dst_border as usize;
    let uv_border = border / 2;

    let mbmi = mi.mbmi;
    let mut _16x16mv = mbmi.mv;
    let need_to_clamp_mvs = mbmi.need_to_clamp_mvs;

    let mb_to_left_edge = x.mb_to_left_edge;
    let mb_to_right_edge = x.mb_to_right_edge;
    let mb_to_top_edge = x.mb_to_top_edge;
    let mb_to_bottom_edge = x.mb_to_bottom_edge;

    let subpixel_predict16x16 = x.subpixel_predict16x16;
    let subpixel_predict8x8 = x.subpixel_predict8x8;
    let fullpixel_mask = x.fullpixel_mask;

    if need_to_clamp_mvs != 0 {
        clamp_mv_to_umv_border(
            _16x16mv.as_mv_mut(),
            mb_to_left_edge,
            mb_to_right_edge,
            mb_to_top_edge,
            mb_to_bottom_edge,
        );
    }

    let mb_row = (-x.mb_to_top_edge / 128) as usize;
    let mb_col = (-x.mb_to_left_edge / 128) as usize;

    let recon_yoffset = mb_row * 16 * dst_y_stride as usize + mb_col * 16;
    let recon_uvoffset = mb_row * 8 * dst_uv_stride as usize + mb_col * 8;

    let pre_recon_yoffset = mb_row * 16 * pre_y_stride as usize + mb_col * 16;
    let pre_recon_uvoffset = mb_row * 8 * pre_uv_stride as usize + mb_col * 8;

    let dst_y_active_offset = border * dst_y_stride as usize + border + recon_yoffset;
    let pre_y_active_offset = border * pre_y_stride as usize + border + pre_recon_yoffset;

    let mv_row_offset = (_16x16mv.as_mv().row as i32 >> 3) * pre_y_stride;
    let mv_col_offset = _16x16mv.as_mv().col as i32 >> 3;
    let pre_y_offset = (pre_y_active_offset as i32 + mv_row_offset + mv_col_offset) as usize;

    if _16x16mv.as_int() & 0x70007_u32 != 0 {
        call_subpixel_predict(
            subpixel_predict16x16,
            pre_y_slice,
            pre_y_offset,
            pre_y_stride,
            _16x16mv.as_mv().col as i32 & 7,
            _16x16mv.as_mv().row as i32 & 7,
            dst_y_slice,
            dst_y_active_offset,
            dst_y_stride,
        );
    } else {
        let pre_len = 15 * pre_y_stride as usize + 16;
        let dst_len = 15 * dst_y_stride as usize + 16;
        let sub_pre = &pre_y_slice[pre_y_offset..pre_y_offset + pre_len];
        let sub_dst = &mut dst_y_slice[dst_y_active_offset..dst_y_active_offset + dst_len];
        vp8_copy_mem16x16_safe(sub_pre, pre_y_stride, sub_dst, dst_y_stride);
    }

    _16x16mv.as_mv_mut().row = (_16x16mv.as_mv().row as i32
        + (1_i32
            | _16x16mv.as_mv().row as i32
                >> (::core::mem::size_of::<i32>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1_usize))) as i16;
    _16x16mv.as_mv_mut().col = (_16x16mv.as_mv().col as i32
        + (1_i32
            | _16x16mv.as_mv().col as i32
                >> (::core::mem::size_of::<i32>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1_usize))) as i16;
    _16x16mv.as_mv_mut().row = (_16x16mv.as_mv().row as i32 / 2_i32) as i16;
    _16x16mv.as_mv_mut().col = (_16x16mv.as_mv().col as i32 / 2_i32) as i16;
    _16x16mv.as_mv_mut().row = (_16x16mv.as_mv().row as i32 & fullpixel_mask) as i16;
    _16x16mv.as_mv_mut().col = (_16x16mv.as_mv().col as i32 & fullpixel_mask) as i16;

    if (2_i32 * _16x16mv.as_mv().col as i32) < mb_to_left_edge - (19_i32 << 3_i32)
        || 2_i32 * _16x16mv.as_mv().col as i32 > mb_to_right_edge + (18_i32 << 3_i32)
        || (2_i32 * _16x16mv.as_mv().row as i32) < mb_to_top_edge - (19_i32 << 3_i32)
        || 2_i32 * _16x16mv.as_mv().row as i32 > mb_to_bottom_edge + (18_i32 << 3_i32)
    {
        return;
    }

    let dst_uv_active_offset = uv_border * dst_uv_stride as usize + uv_border + recon_uvoffset;
    let pre_uv_active_offset = uv_border * pre_uv_stride as usize + uv_border + pre_recon_uvoffset;

    let uv_row_offset = (_16x16mv.as_mv().row as i32 >> 3) * pre_uv_stride;
    let uv_col_offset = _16x16mv.as_mv().col as i32 >> 3;
    let pre_uv_offset = (pre_uv_active_offset as i32 + uv_row_offset + uv_col_offset) as usize;

    if _16x16mv.as_int() & 0x70007_u32 != 0 {
        call_subpixel_predict(
            subpixel_predict8x8,
            pre_u_slice,
            pre_uv_offset,
            pre_uv_stride,
            _16x16mv.as_mv().col as i32 & 7,
            _16x16mv.as_mv().row as i32 & 7,
            dst_u_slice,
            dst_uv_active_offset,
            dst_uv_stride,
        );
        call_subpixel_predict(
            subpixel_predict8x8,
            pre_v_slice,
            pre_uv_offset,
            pre_uv_stride,
            _16x16mv.as_mv().col as i32 & 7,
            _16x16mv.as_mv().row as i32 & 7,
            dst_v_slice,
            dst_uv_active_offset,
            dst_uv_stride,
        );
    } else {
        let pre_len = 7 * pre_uv_stride as usize + 8;
        let dst_len = 7 * dst_uv_stride as usize + 8;

        let sub_pre_u = &pre_u_slice[pre_uv_offset..pre_uv_offset + pre_len];
        let sub_dst_u = &mut dst_u_slice[dst_uv_active_offset..dst_uv_active_offset + dst_len];
        vp8_copy_mem8x8_safe(sub_pre_u, pre_uv_stride, sub_dst_u, dst_uv_stride);

        let sub_pre_v = &pre_v_slice[pre_uv_offset..pre_uv_offset + pre_len];
        let sub_dst_v = &mut dst_v_slice[dst_uv_active_offset..dst_uv_active_offset + dst_len];
        vp8_copy_mem8x8_safe(sub_pre_v, pre_uv_stride, sub_dst_v, dst_uv_stride);
    }
}
fn build_inter4x4_predictors_mb(
    x: &mut MACROBLOCKD,
    mi: &MODE_INFO,
    mut dst_y_slice: &mut [u8],
    mut dst_u_slice: &mut [u8],
    mut dst_v_slice: &mut [u8],
    pre_y_slice: &[u8],
    pre_u_slice: &[u8],
    pre_v_slice: &[u8],
) {
    let partitioning = mi.mbmi.partitioning;
    let need_to_clamp_mvs = mi.mbmi.need_to_clamp_mvs;

    let dst_y_stride = x.dst_y_stride;
    let dst_uv_stride = x.dst_uv_stride;
    let pre_y_stride = x.pre_y_stride;
    let pre_uv_stride = x.pre_uv_stride;

    let border = x.dst_border as usize;
    let uv_border = border / 2;

    // Reconstruct global offsets
    let mb_row = (-x.mb_to_top_edge / 128) as usize;
    let mb_col = (-x.mb_to_left_edge / 128) as usize;

    let recon_yoffset = mb_row * 16 * dst_y_stride as usize + mb_col * 16;
    let recon_uvoffset = mb_row * 8 * dst_uv_stride as usize + mb_col * 8;

    let pre_recon_yoffset = mb_row * 16 * pre_y_stride as usize + mb_col * 16;
    let pre_recon_uvoffset = mb_row * 8 * pre_uv_stride as usize + mb_col * 8;

    {
        let bmi = mi.bmi;
        if (partitioning as i32) < 3_i32 {
            x.block[0].bmi = bmi[0];
            x.block[2].bmi = bmi[2];
            x.block[8].bmi = bmi[8];
            x.block[10].bmi = bmi[10];

            if need_to_clamp_mvs != 0 {
                let mb_to_left_edge = x.mb_to_left_edge;
                let mb_to_right_edge = x.mb_to_right_edge;
                let mb_to_top_edge = x.mb_to_top_edge;
                let mb_to_bottom_edge = x.mb_to_bottom_edge;

                clamp_mv_to_umv_border(
                    x.block[0].bmi.mv_mut().as_mv_mut(),
                    mb_to_left_edge,
                    mb_to_right_edge,
                    mb_to_top_edge,
                    mb_to_bottom_edge,
                );
                clamp_mv_to_umv_border(
                    x.block[2].bmi.mv_mut().as_mv_mut(),
                    mb_to_left_edge,
                    mb_to_right_edge,
                    mb_to_top_edge,
                    mb_to_bottom_edge,
                );
                clamp_mv_to_umv_border(
                    x.block[8].bmi.mv_mut().as_mv_mut(),
                    mb_to_left_edge,
                    mb_to_right_edge,
                    mb_to_top_edge,
                    mb_to_bottom_edge,
                );
                clamp_mv_to_umv_border(
                    x.block[10].bmi.mv_mut().as_mv_mut(),
                    mb_to_left_edge,
                    mb_to_right_edge,
                    mb_to_top_edge,
                    mb_to_bottom_edge,
                );
            }
        } else {
            for i in (0..16).step_by(2) {
                x.block[i].bmi = bmi[i];
                x.block[i + 1].bmi = bmi[i + 1];
                if need_to_clamp_mvs != 0 {
                    let mb_to_left_edge = x.mb_to_left_edge;
                    let mb_to_right_edge = x.mb_to_right_edge;
                    let mb_to_top_edge = x.mb_to_top_edge;
                    let mb_to_bottom_edge = x.mb_to_bottom_edge;

                    clamp_mv_to_umv_border(
                        x.block[i].bmi.mv_mut().as_mv_mut(),
                        mb_to_left_edge,
                        mb_to_right_edge,
                        mb_to_top_edge,
                        mb_to_bottom_edge,
                    );
                    clamp_mv_to_umv_border(
                        x.block[i + 1].bmi.mv_mut().as_mv_mut(),
                        mb_to_left_edge,
                        mb_to_right_edge,
                        mb_to_top_edge,
                        mb_to_bottom_edge,
                    );
                }
            }
        }
    }

    let subpixel_predict8x8 = x.subpixel_predict8x8;
    let subpixel_predict8x4 = x.subpixel_predict8x4;
    let subpixel_predict = x.subpixel_predict;

    let dst_y_active_offset = border * dst_y_stride as usize + border + recon_yoffset;
    let pre_y_active_offset = border * pre_y_stride as usize + border + pre_recon_yoffset;

    if (partitioning as i32) < 3_i32 {
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[0],
            dst_y_slice,
            dst_y_active_offset + x.block[0].offset as usize,
            dst_y_stride,
            pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[2],
            dst_y_slice,
            dst_y_active_offset + x.block[2].offset as usize,
            dst_y_stride,
            pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[8],
            dst_y_slice,
            dst_y_active_offset + x.block[8].offset as usize,
            dst_y_stride,
            pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[10],
            dst_y_slice,
            dst_y_active_offset + x.block[10].offset as usize,
            dst_y_stride,
            pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
    } else {
        for i in (0..16).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i + 1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    dst_y_slice,
                    dst_y_active_offset + d0.offset as usize,
                    dst_y_stride,
                    pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    dst_y_slice,
                    dst_y_active_offset + d0.offset as usize,
                    dst_y_stride,
                    pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    dst_y_slice,
                    dst_y_active_offset + d1.offset as usize,
                    dst_y_stride,
                    pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
            }
        }
    }

    let dst_uv_active_offset = uv_border * dst_uv_stride as usize + uv_border + recon_uvoffset;
    let pre_uv_active_offset = uv_border * pre_uv_stride as usize + uv_border + pre_recon_uvoffset;

    {
        for i in (16..20).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i + 1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    dst_u_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    dst_u_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    dst_u_slice,
                    dst_uv_active_offset + d1.offset as usize,
                    dst_uv_stride,
                    pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            }
        }
    }
    {
        for i in (20..24).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i + 1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    dst_v_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    dst_v_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    dst_v_slice,
                    dst_uv_active_offset + d1.offset as usize,
                    dst_uv_stride,
                    pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            }
        }
    }
}
fn build_4x4uvmvs(x: &mut MACROBLOCKD, mi: &MODE_INFO) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0_i32;
    while i < 2_i32 {
        j = 0_i32;
        while j < 2_i32 {
            let mut yoffset: i32 = i * 8_i32 + j * 2_i32;
            let mut uoffset: i32 = 16_i32 + i * 2_i32 + j;
            let mut voffset: i32 = 20_i32 + i * 2_i32 + j;
            let mut temp: i32 = 0;
            temp = mi.bmi[yoffset as usize].mv().as_mv().row as i32
                + mi.bmi[(yoffset + 1_i32) as usize].mv().as_mv().row as i32
                + mi.bmi[(yoffset + 4_i32) as usize].mv().as_mv().row as i32
                + mi.bmi[(yoffset + 5_i32) as usize].mv().as_mv().row as i32;
            temp += 4_i32
                + (temp
                    >> (::core::mem::size_of::<i32>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize))
                    * 8_i32;
            x.block[uoffset as usize].bmi.mv_mut().as_mv_mut().row =
                ((temp / 8_i32) & x.fullpixel_mask) as i16;
            temp = mi.bmi[yoffset as usize].mv().as_mv().col as i32
                + mi.bmi[(yoffset + 1_i32) as usize].mv().as_mv().col as i32
                + mi.bmi[(yoffset + 4_i32) as usize].mv().as_mv().col as i32
                + mi.bmi[(yoffset + 5_i32) as usize].mv().as_mv().col as i32;
            temp += 4_i32
                + (temp
                    >> (::core::mem::size_of::<i32>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize))
                    * 8_i32;
            x.block[uoffset as usize].bmi.mv_mut().as_mv_mut().col =
                ((temp / 8_i32) & x.fullpixel_mask) as i16;
            if mi.mbmi.need_to_clamp_mvs != 0 {
                clamp_uvmv_to_umv_border(
                    x.block[uoffset as usize].bmi.mv_mut().as_mv_mut(),
                    x.mb_to_left_edge,
                    x.mb_to_right_edge,
                    x.mb_to_top_edge,
                    x.mb_to_bottom_edge,
                );
            }
            let u_mv = x.block[uoffset as usize].bmi.mv();
            *x.block[voffset as usize].bmi.mv_mut() = u_mv;
            j += 1;
        }
        i += 1;
    }
}
/// `vp8_build_inter_predictors_mb` — vp8/common/reconinter.c:494. Builds inter
/// predictors for a split-MV macroblock (per-subblock motion).
pub fn vp8_build_inter_predictors_mb(
    xd: &mut MACROBLOCKD,
    mi: &MODE_INFO,
    dst_y: &mut [u8],
    dst_u: &mut [u8],
    dst_v: &mut [u8],
    pre_y: &[u8],
    pre_u: &[u8],
    pre_v: &[u8],
) {
    if mi.mbmi.mode as i32 != SPLITMV as i32 {
        vp8_build_inter16x16_predictors_mb(xd, mi, dst_y, dst_u, dst_v, pre_y, pre_u, pre_v);
    } else {
        build_4x4uvmvs(xd, mi);
        build_inter4x4_predictors_mb(xd, mi, dst_y, dst_u, dst_v, pre_y, pre_u, pre_v);
    };
}
