//! 4x4 intra prediction — port of `vp8/common/reconintra4x4.c`.
//!
//! Per-4x4-subblock intra predictor (the B_PRED mode set) from the local
//! above/left/above-left neighbour pixels.

pub use crate::vp8::common::types::*;

/// `vp8_intra4x4_predict` — vp8/common/reconintra4x4.c:39. Predicts one 4x4
/// subblock for the given B_PRED mode from its neighbour pixels.
pub fn vp8_intra4x4_predict_safe(
    y_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: usize,
    b_mode: B_PREDICTION_MODE,
    above: &[u8; 8],
    left: &[u8; 4],
    top_left: u8,
) {
    let mut Aboveb: [u8; 12] = [0; 12];
    Aboveb[4..12].copy_from_slice(above);
    Aboveb[3] = top_left;

    let mut Left: [u8; 8] = [0; 8];
    Left[0..4].copy_from_slice(left);

    match b_mode {
        B_DC_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_dc_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[4..8],
                &Left[0..4],
            );
        }
        B_VE_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_ve_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[3..9],
            );
        }
        B_HE_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_he_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[3..4],
                &Left[0..4],
            );
        }
        B_HU_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d207_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Left[0..4],
            );
        }
        B_VR_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d117_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[3..8],
                &Left[0..3],
            );
        }
        B_VL_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d63e_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[4..12],
            );
        }
        B_LD_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d45e_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[4..12],
            );
        }
        B_RD_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d135_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[3..8],
                &Left[0..4],
            );
        }
        B_HD_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_d153_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[3..7],
                &Left[0..4],
            );
        }
        B_TM_PRED => {
            let (_, dst_after) = y_slice.split_at_mut(dst_offset);
            crate::vpx_dsp::intrapred::vpx_tm_predictor_4x4_safe(
                dst_after,
                dst_stride,
                &Aboveb[4..8],
                &Left[0..4],
                Aboveb[3],
            );
        }
        _ => unreachable!("Invalid b_mode: {}", b_mode),
    }
}
