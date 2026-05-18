use std::ffi::c_void;
unsafe extern "Rust" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn vpx_d117_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d135_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d153_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d207_predictor_4x4_c(
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
    fn vpx_dc_predictor_4x4_c(
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
    fn vpx_tm_predictor_4x4_c(
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
pub type B_PREDICTION_MODE = u32;
pub const B_MODE_COUNT: B_PREDICTION_MODE = 14;
pub const NEW4X4: B_PREDICTION_MODE = 13;
pub const ZERO4X4: B_PREDICTION_MODE = 12;
pub const ABOVE4X4: B_PREDICTION_MODE = 11;
pub const LEFT4X4: B_PREDICTION_MODE = 10;
pub const B_HU_PRED: B_PREDICTION_MODE = 9;
pub const B_HD_PRED: B_PREDICTION_MODE = 8;
pub const B_VL_PRED: B_PREDICTION_MODE = 7;
pub const B_VR_PRED: B_PREDICTION_MODE = 6;
pub const B_RD_PRED: B_PREDICTION_MODE = 5;
pub const B_LD_PRED: B_PREDICTION_MODE = 4;
pub const B_HE_PRED: B_PREDICTION_MODE = 3;
pub const B_VE_PRED: B_PREDICTION_MODE = 2;
pub const B_TM_PRED: B_PREDICTION_MODE = 1;
pub const B_DC_PRED: B_PREDICTION_MODE = 0;
pub type intra_pred_fn =
    Option<unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
static mut pred: [intra_pred_fn; 10] = [None; 10];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_intra4x4_predictors_internal() {
    unsafe {
        pred[B_DC_PRED as usize] = Some(
            vpx_dc_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_TM_PRED as usize] = Some(
            vpx_tm_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_VE_PRED as usize] = Some(
            vpx_ve_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_HE_PRED as usize] = Some(
            vpx_he_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_LD_PRED as usize] = Some(
            vpx_d45e_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_RD_PRED as usize] = Some(
            vpx_d135_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_VR_PRED as usize] = Some(
            vpx_d117_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_VL_PRED as usize] = Some(
            vpx_d63e_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_HD_PRED as usize] = Some(
            vpx_d153_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[B_HU_PRED as usize] = Some(
            vpx_d207_predictor_4x4_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_intra4x4_predict(
    mut above: *mut u8,
    mut yleft: *mut u8,
    mut left_stride: i32,
    mut b_mode: B_PREDICTION_MODE,
    mut dst: *mut u8,
    mut dst_stride: i32,
    mut top_left: u8,
) {
    unsafe {
        let mut Aboveb: [u8; 12] = [0; 12];
        let mut Above: *mut u8 = (&raw mut Aboveb as *mut u8).offset(4 as isize);
        let mut Left: [u8; 4] = [0; 4];
        Left[0 as usize] = *yleft.offset(0 as isize);
        Left[1 as usize] = *yleft.offset(left_stride as isize);
        Left[2 as usize] = *yleft.offset((2 as i32 * left_stride) as isize);
        Left[3 as usize] = *yleft.offset((3 as i32 * left_stride) as isize);
        memcpy(Above as *mut c_void, above as *const c_void, 8 as size_t);
        *Above.offset(-(1 as i32) as isize) = top_left;
        pred[b_mode as usize].expect("non-null function pointer")(
            dst as *mut uint8_t,
            dst_stride as ptrdiff_t,
            Above,
            &raw mut Left as *mut u8,
        );
    }
}
