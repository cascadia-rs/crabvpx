use std::ffi::c_void;
unsafe extern "Rust" {
    fn vpx_d117_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_d135_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_d153_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_d207_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_d45e_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_d63e_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_dc_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_he_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_tm_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
    fn vpx_ve_predictor_4x4_c(dst: *mut u8, stride: isize, above: *const u8, left: *const u8);
}
pub const B_MODE_COUNT: u32 = 14;
pub const NEW4X4: u32 = 13;
pub const ZERO4X4: u32 = 12;
pub const ABOVE4X4: u32 = 11;
pub const LEFT4X4: u32 = 10;
pub const B_HU_PRED: u32 = 9;
pub const B_HD_PRED: u32 = 8;
pub const B_VL_PRED: u32 = 7;
pub const B_VR_PRED: u32 = 6;
pub const B_RD_PRED: u32 = 5;
pub const B_LD_PRED: u32 = 4;
pub const B_HE_PRED: u32 = 3;
pub const B_VE_PRED: u32 = 2;
pub const B_TM_PRED: u32 = 1;
pub const B_DC_PRED: u32 = 0;
pub type IntraPredFn = Option<unsafe fn(*mut u8, isize, *const u8, *const u8) -> ()>;
static pred: [IntraPredFn; 10] = [
    Some(vpx_dc_predictor_4x4_c),
    Some(vpx_tm_predictor_4x4_c),
    Some(vpx_ve_predictor_4x4_c),
    Some(vpx_he_predictor_4x4_c),
    Some(vpx_d45e_predictor_4x4_c),
    Some(vpx_d135_predictor_4x4_c),
    Some(vpx_d117_predictor_4x4_c),
    Some(vpx_d63e_predictor_4x4_c),
    Some(vpx_d153_predictor_4x4_c),
    Some(vpx_d207_predictor_4x4_c),
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_intra4x4_predictors_internal() {}
#[unsafe(no_mangle)]
pub unsafe fn vp8_intra4x4_predict(
    mut above: *mut u8,
    mut yleft: *mut u8,
    mut left_stride: i32,
    mut b_mode: u32,
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
            8 as usize,
        );
        *Above.offset(-(1 as i32) as isize) = top_left;
        pred[b_mode as usize].expect("non-null function pointer")(
            dst as *mut u8,
            dst_stride as isize,
            Above,
            &raw mut left as *mut u8,
        );
    }
}
