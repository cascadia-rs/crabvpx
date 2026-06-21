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
pub fn vp8_setup_intra_recon_top_line(ybf: &YV12_BUFFER_CONFIG, full: &mut [u8]) {
    let y_border = ybf.border as usize;
    let y_stride = ybf.y_stride as usize;
    let y_width = ybf.y_width as usize;

    let uv_border = (ybf.border / 2) as usize;
    let uv_stride = ybf.uv_stride as usize;
    let uv_width = ybf.uv_width as usize;

    if y_border >= 1 {
        let y_slice = ybf.safe_y_slice_mut(full);
        let y_idx = (y_border - 1) * y_stride + (y_border - 1);
        let len = y_width + 5;
        if y_idx + len <= y_slice.len() {
            y_slice[y_idx..y_idx + len].fill(127);
        } else {
            debug_assert!(false, "Y slice overflow in vp8_setup_intra_recon_top_line");
        }
    }

    if uv_border >= 1 {
        let u_slice = ybf.safe_u_slice_mut(full);
        let uv_idx = (uv_border - 1) * uv_stride + (uv_border - 1);
        let len = uv_width + 5;
        if uv_idx + len <= u_slice.len() {
            u_slice[uv_idx..uv_idx + len].fill(127);
        } else {
            debug_assert!(false, "U slice overflow in vp8_setup_intra_recon_top_line");
        }

        let v_slice = ybf.safe_v_slice_mut(full);
        if uv_idx + len <= v_slice.len() {
            v_slice[uv_idx..uv_idx + len].fill(127);
        } else {
            debug_assert!(false, "V slice overflow in vp8_setup_intra_recon_top_line");
        }
    }
}
