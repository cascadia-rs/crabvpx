unsafe extern "C" {
    fn vpx_d117_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d135_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d153_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d207_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d45e_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d63e_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_he_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_ve_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type uint8_t = u8;
pub use crate::vp8::common::types::*;
pub type intra_pred_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
static pred: [intra_pred_fn; 10] = [
    Some(vpx_dc_predictor_4x4_neon),
    Some(vpx_tm_predictor_4x4_neon),
    Some(vpx_ve_predictor_4x4_c),
    Some(vpx_he_predictor_4x4_c),
    Some(vpx_d45e_predictor_4x4_c),
    Some(vpx_d135_predictor_4x4_neon),
    Some(vpx_d117_predictor_4x4_neon),
    Some(vpx_d63e_predictor_4x4_c),
    Some(vpx_d153_predictor_4x4_neon),
    Some(vpx_d207_predictor_4x4_neon),
];
#[unsafe(no_mangle)]
pub extern "C" fn vp8_intra4x4_predict(
    above: *mut ::core::ffi::c_uchar,
    yleft: *mut ::core::ffi::c_uchar,
    left_stride: ::core::ffi::c_int,
    b_mode: B_PREDICTION_MODE,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
    top_left: ::core::ffi::c_uchar,
) {
    if above.is_null() || yleft.is_null() || dst.is_null() {
        return;
    }

    let mut left = [0u8; 8];
    let mut above_buffer = [0u8; 12];
    above_buffer[3] = top_left;

    // SAFETY: The pointers are checked for null, and caller guarantees valid memory of appropriate sizes.
    unsafe {
        let above_slice = core::slice::from_raw_parts(above, 8);
        above_buffer[4..12].copy_from_slice(above_slice);

        let yleft_len = (3 * left_stride + 1) as usize;
        let yleft_slice = core::slice::from_raw_parts(yleft, yleft_len);
        left[0] = yleft_slice[0];
        left[1] = yleft_slice[left_stride as usize];
        left[2] = yleft_slice[2 * left_stride as usize];
        left[3] = yleft_slice[3 * left_stride as usize];

        let pred_fn = pred[b_mode as usize].expect("non-null function pointer");
        pred_fn(
            dst as *mut uint8_t,
            dst_stride as ptrdiff_t,
            above_buffer[4..].as_ptr(),
            left.as_ptr(),
        );
    }
}

pub fn vp8_intra4x4_predict_safe(
    y_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: usize,
    b_mode: B_PREDICTION_MODE,
) {
    let mut Aboveb: [u8; 12] = [0; 12];
    let above_idx = dst_offset - dst_stride;
    let top_left = y_slice[above_idx - 1];

    Aboveb[4..12].copy_from_slice(&y_slice[above_idx..above_idx + 8]);
    Aboveb[3] = top_left;

    let mut Left: [u8; 8] = [0; 8];
    let yleft_idx = dst_offset - 1;
    Left[0] = y_slice[yleft_idx];
    Left[1] = y_slice[yleft_idx + dst_stride];
    Left[2] = y_slice[yleft_idx + 2 * dst_stride];
    Left[3] = y_slice[yleft_idx + 3 * dst_stride];

    match b_mode {
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
        _ => {
            let dst_ptr = y_slice[dst_offset..].as_mut_ptr();
            unsafe {
                pred[b_mode as usize].expect("non-null function pointer")(
                    dst_ptr,
                    dst_stride as ptrdiff_t,
                    Aboveb[4..].as_ptr(),
                    Left.as_ptr(),
                );
            }
        }
    }
}
