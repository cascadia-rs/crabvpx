use std::ffi::c_void;
unsafe extern "Rust" {
    fn vpx_d117_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_d135_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_d153_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_d207_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_d45e_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_d63e_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_dc_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_he_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_tm_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_ve_predictor_4x4_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
}
pub type DarwinPtrdiffT = isize;
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
pub type PtrdiffT = DarwinPtrdiffT;
pub type BPredictionMode = u32;
pub const B_MODE_COUNT: BPredictionMode = 14;
pub const NEW4X4: BPredictionMode = 13;
pub const ZERO4X4: BPredictionMode = 12;
pub const ABOVE4X4: BPredictionMode = 11;
pub const LEFT4X4: BPredictionMode = 10;
pub const B_HU_PRED: BPredictionMode = 9;
pub const B_HD_PRED: BPredictionMode = 8;
pub const B_VL_PRED: BPredictionMode = 7;
pub const B_VR_PRED: BPredictionMode = 6;
pub const B_RD_PRED: BPredictionMode = 5;
pub const B_LD_PRED: BPredictionMode = 4;
pub const B_HE_PRED: BPredictionMode = 3;
pub const B_VE_PRED: BPredictionMode = 2;
pub const B_TM_PRED: BPredictionMode = 1;
pub const B_DC_PRED: BPredictionMode = 0;
pub type IntraPredFn = Option<unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> ()>;
static mut pred: [IntraPredFn; 10] = [None; 10];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_intra4x4_predictors_internal() {
    unsafe {
        pred[B_DC_PRED as usize] = Some(
            vpx_dc_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_TM_PRED as usize] = Some(
            vpx_tm_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_VE_PRED as usize] = Some(
            vpx_ve_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_HE_PRED as usize] = Some(
            vpx_he_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_LD_PRED as usize] = Some(
            vpx_d45e_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_RD_PRED as usize] = Some(
            vpx_d135_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_VR_PRED as usize] = Some(
            vpx_d117_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_VL_PRED as usize] = Some(
            vpx_d63e_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_HD_PRED as usize] = Some(
            vpx_d153_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[B_HU_PRED as usize] = Some(
            vpx_d207_predictor_4x4_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_intra4x4_predict(
    mut above: *mut u8,
    mut yleft: *mut u8,
    mut left_stride: i32,
    mut b_mode: BPredictionMode,
    mut dst: *mut u8,
    mut dst_stride: i32,
    mut top_left: u8,
) {
    unsafe {
        let mut Aboveb: [u8; 12] = [0; 12];
        let mut Above: *mut u8 = (&raw mut Aboveb as *mut u8).offset(4 as isize);
        let mut left: [u8; 4] = [0; 4];
        left[0 as usize] = *yleft.offset(0 as isize);
        left[1 as usize] = *yleft.offset(left_stride as isize);
        left[2 as usize] = *yleft.offset((2 as i32 * left_stride) as isize);
        left[3 as usize] = *yleft.offset((3 as i32 * left_stride) as isize);
        core::ptr::copy_nonoverlapping(
            above as *const c_void as *const u8,
            Above as *mut c_void as *mut u8,
            8 as SizeT,
        );
        *Above.offset(-(1 as i32) as isize) = top_left;
        pred[b_mode as usize].expect("non-null function pointer")(
            dst as *mut u8,
            dst_stride as PtrdiffT,
            Above,
            &raw mut left as *mut u8,
        );
    }
}
