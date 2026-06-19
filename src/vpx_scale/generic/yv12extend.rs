pub use crate::vp8::common::types::*;
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
pub type uint8_t = u8;
fn extend_plane(
    plane: &mut [u8],
    stride: usize,
    width: usize,
    height: usize,
    extend_top: usize,
    extend_left: usize,
    extend_bottom: usize,
    extend_right: usize,
) {
    let linesize = extend_left + width + extend_right;

    // 1. Extend left/right for active rows.
    for r in 0..height {
        let row_idx = extend_top + r;
        let row_start = row_idx * stride;

        let src_left_val = plane[row_start + extend_left];
        let src_right_val = plane[row_start + extend_left + width - 1];

        // Fill left border
        for i in 0..extend_left {
            plane[row_start + i] = src_left_val;
        }

        // Fill right border
        let right_start = row_start + extend_left + width;
        for i in 0..extend_right {
            plane[right_start + i] = src_right_val;
        }
    }

    // 2. Extend top rows.
    let src_top_start = extend_top * stride;
    for r in 0..extend_top {
        let dst_start = r * stride;
        plane.copy_within(src_top_start..src_top_start + linesize, dst_start);
    }

    // 3. Extend bottom rows.
    let src_bot_start = (extend_top + height - 1) * stride;
    for r in 0..extend_bottom {
        let dst_start = (extend_top + height + r) * stride;
        plane.copy_within(src_bot_start..src_bot_start + linesize, dst_start);
    }
}

pub fn vp8_yv12_extend_frame_borders_c(ybf: &mut YV12_BUFFER_CONFIG) {
    let uv_border = ybf.border / 2;

    let y_extend_bottom = (ybf.border + ybf.y_height - ybf.y_crop_height) as usize;
    let y_extend_right = (ybf.border + ybf.y_width - ybf.y_crop_width) as usize;

    let uv_extend_bottom = (uv_border + ybf.uv_height - ybf.uv_crop_height) as usize;
    let uv_extend_right = (uv_border + ybf.uv_width - ybf.uv_crop_width) as usize;

    let y_stride = ybf.y_stride as usize;
    let y_crop_width = ybf.y_crop_width as usize;
    let y_crop_height = ybf.y_crop_height as usize;
    let y_border = ybf.border as usize;

    let y_plane = ybf.y_slice_mut_safe();
    extend_plane(
        y_plane,
        y_stride,
        y_crop_width,
        y_crop_height,
        y_border,
        y_border,
        y_extend_bottom,
        y_extend_right,
    );

    let uv_stride = ybf.uv_stride as usize;
    let uv_crop_width = ybf.uv_crop_width as usize;
    let uv_crop_height = ybf.uv_crop_height as usize;

    let u_plane = ybf.u_slice_mut_safe();
    extend_plane(
        u_plane,
        uv_stride,
        uv_crop_width,
        uv_crop_height,
        uv_border as usize,
        uv_border as usize,
        uv_extend_bottom,
        uv_extend_right,
    );

    let v_plane = ybf.v_slice_mut_safe();
    extend_plane(
        v_plane,
        uv_stride,
        uv_crop_width,
        uv_crop_height,
        uv_border as usize,
        uv_border as usize,
        uv_extend_bottom,
        uv_extend_right,
    );
}
pub fn vp8_yv12_copy_frame_c(src_ybc: &YV12_BUFFER_CONFIG, dst_ybc: &mut YV12_BUFFER_CONFIG) {
    let src_border = src_ybc.border as usize;
    let dst_border = dst_ybc.border as usize;
    let src_y_stride = src_ybc.y_stride as usize;
    let dst_y_stride = dst_ybc.y_stride as usize;
    let y_height = src_ybc.y_height as usize;
    let y_width = src_ybc.y_width as usize;

    let src_y = src_ybc.y_slice_safe();
    let dst_y = dst_ybc.y_slice_mut_safe();
    for r in 0..y_height {
        let src_start = (src_border + r) * src_y_stride + src_border;
        let dst_start = (dst_border + r) * dst_y_stride + dst_border;
        dst_y[dst_start..dst_start + y_width]
            .copy_from_slice(&src_y[src_start..src_start + y_width]);
    }

    let src_uv_border = src_border / 2;
    let dst_uv_border = dst_border / 2;
    let src_uv_stride = src_ybc.uv_stride as usize;
    let dst_uv_stride = dst_ybc.uv_stride as usize;
    let uv_height = src_ybc.uv_height as usize;
    let uv_width = src_ybc.uv_width as usize;

    let src_u = src_ybc.u_slice_safe();
    let dst_u = dst_ybc.u_slice_mut_safe();
    for r in 0..uv_height {
        let src_start = (src_uv_border + r) * src_uv_stride + src_uv_border;
        let dst_start = (dst_uv_border + r) * dst_uv_stride + dst_uv_border;
        dst_u[dst_start..dst_start + uv_width]
            .copy_from_slice(&src_u[src_start..src_start + uv_width]);
    }

    let src_v = src_ybc.v_slice_safe();
    let dst_v = dst_ybc.v_slice_mut_safe();
    for r in 0..uv_height {
        let src_start = (src_uv_border + r) * src_uv_stride + src_uv_border;
        let dst_start = (dst_uv_border + r) * dst_uv_stride + dst_uv_border;
        dst_v[dst_start..dst_start + uv_width]
            .copy_from_slice(&src_v[src_start..src_start + uv_width]);
    }

    vp8_yv12_extend_frame_borders_c(dst_ybc);
}
pub fn vpx_yv12_copy_y_c(src_ybc: &YV12_BUFFER_CONFIG, dst_ybc: &mut YV12_BUFFER_CONFIG) {
    let src_border = src_ybc.border as usize;
    let dst_border = dst_ybc.border as usize;
    let src_y_stride = src_ybc.y_stride as usize;
    let dst_y_stride = dst_ybc.y_stride as usize;
    let y_height = src_ybc.y_height as usize;
    let y_width = src_ybc.y_width as usize;

    let src_y = src_ybc.y_slice_safe();
    let dst_y = dst_ybc.y_slice_mut_safe();
    for r in 0..y_height {
        let src_start = (src_border + r) * src_y_stride + src_border;
        let dst_start = (dst_border + r) * dst_y_stride + dst_border;
        dst_y[dst_start..dst_start + y_width]
            .copy_from_slice(&src_y[src_start..src_start + y_width]);
    }
}
