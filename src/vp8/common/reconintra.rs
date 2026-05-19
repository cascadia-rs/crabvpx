
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
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


pub fn vp8_build_intra_predictors_mby_safe(
    mode: MB_PREDICTION_MODE,
    left_available: i32,
    up_available: i32,
    yabove: &[u8; 17],
    yleft: &[u8; 16],
    ypred_slice: &mut [u8],
    y_stride: usize,
) {
    match mode {
        V_PRED => {
            crate::vpx_dsp::intrapred::vpx_v_predictor_16x16_safe(
                ypred_slice,
                y_stride,
                &yabove[1..],
            );
        }
        H_PRED => {
            crate::vpx_dsp::intrapred::vpx_h_predictor_16x16_safe(
                ypred_slice,
                y_stride,
                yleft,
            );
        }
        TM_PRED => {
            crate::vpx_dsp::intrapred::vpx_tm_predictor_16x16_safe(
                ypred_slice,
                y_stride,
                &yabove[1..17],
                yleft,
                yabove[0],
            );
        }
        _ => {
            debug_assert_eq!(mode, DC_PRED);
            match (left_available != 0, up_available != 0) {
                (false, false) => {
                    crate::vpx_dsp::intrapred::vpx_dc_128_predictor_16x16_safe(ypred_slice, y_stride);
                }
                (false, true) => {
                    crate::vpx_dsp::intrapred::vpx_dc_top_predictor_16x16_safe(ypred_slice, y_stride, &yabove[1..]);
                }
                (true, false) => {
                    crate::vpx_dsp::intrapred::vpx_dc_left_predictor_16x16_safe(ypred_slice, y_stride, yleft);
                }
                (true, true) => {
                    crate::vpx_dsp::intrapred::vpx_dc_predictor_16x16_safe(ypred_slice, y_stride, &yabove[1..], yleft);
                }
            }
        }
    }
}
pub fn vp8_build_intra_predictors_mbuv_safe(
    uvmode: MB_PREDICTION_MODE,
    left_available: i32,
    up_available: i32,
    uabove: &[u8; 9],
    vabove: &[u8; 9],
    uleft: &[u8; 8],
    vleft: &[u8; 8],
    upred_slice: &mut [u8],
    vpred_slice: &mut [u8],
    uv_stride: usize,
) {
    match uvmode {
        V_PRED => {
            crate::vpx_dsp::intrapred::vpx_v_predictor_8x8_safe(
                upred_slice,
                uv_stride,
                &uabove[1..],
            );
            crate::vpx_dsp::intrapred::vpx_v_predictor_8x8_safe(
                vpred_slice,
                uv_stride,
                &vabove[1..],
            );
        }
        H_PRED => {
            crate::vpx_dsp::intrapred::vpx_h_predictor_8x8_safe(
                upred_slice,
                uv_stride,
                uleft,
            );
            crate::vpx_dsp::intrapred::vpx_h_predictor_8x8_safe(
                vpred_slice,
                uv_stride,
                vleft,
            );
        }
        TM_PRED => {
            crate::vpx_dsp::intrapred::vpx_tm_predictor_8x8_safe(
                upred_slice,
                uv_stride,
                &uabove[1..9],
                uleft,
                uabove[0],
            );
            crate::vpx_dsp::intrapred::vpx_tm_predictor_8x8_safe(
                vpred_slice,
                uv_stride,
                &vabove[1..9],
                vleft,
                vabove[0],
            );
        }
        _ => {
            debug_assert_eq!(uvmode, DC_PRED);
            match (left_available != 0, up_available != 0) {
                (false, false) => {
                    crate::vpx_dsp::intrapred::vpx_dc_128_predictor_8x8_safe(upred_slice, uv_stride);
                    crate::vpx_dsp::intrapred::vpx_dc_128_predictor_8x8_safe(vpred_slice, uv_stride);
                }
                (false, true) => {
                    crate::vpx_dsp::intrapred::vpx_dc_top_predictor_8x8_safe(upred_slice, uv_stride, &uabove[1..]);
                    crate::vpx_dsp::intrapred::vpx_dc_top_predictor_8x8_safe(vpred_slice, uv_stride, &vabove[1..]);
                }
                (true, false) => {
                    crate::vpx_dsp::intrapred::vpx_dc_left_predictor_8x8_safe(upred_slice, uv_stride, uleft);
                    crate::vpx_dsp::intrapred::vpx_dc_left_predictor_8x8_safe(vpred_slice, uv_stride, vleft);
                }
                (true, true) => {
                    crate::vpx_dsp::intrapred::vpx_dc_predictor_8x8_safe(upred_slice, uv_stride, &uabove[1..], uleft);
                    crate::vpx_dsp::intrapred::vpx_dc_predictor_8x8_safe(vpred_slice, uv_stride, &vabove[1..], vleft);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_init_intra_predictors() {}

pub fn intra_prediction_down_copy(
    xd: &mut MACROBLOCKD,
) {
    let dst_stride = xd.dst.y_stride as usize;
    let border = xd.dst.border as usize;

    let y_slice = xd.dst.y_slice_mut_safe();

    let base_idx = (border - 1) * dst_stride + border + 16;

    let mut src_bytes = [0u8; 4];
    src_bytes.copy_from_slice(&y_slice[base_idx..base_idx + 4]);

    let idx0 = base_idx + 4 * dst_stride;
    let idx1 = base_idx + 8 * dst_stride;
    let idx2 = base_idx + 12 * dst_stride;

    y_slice[idx0..idx0 + 4].copy_from_slice(&src_bytes);
    y_slice[idx1..idx1 + 4].copy_from_slice(&src_bytes);
    y_slice[idx2..idx2 + 4].copy_from_slice(&src_bytes);
}

