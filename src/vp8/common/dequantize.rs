unsafe extern "C" {
    fn vp8_short_idct4x4llm_c(
        input: *mut ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut ::core::ffi::c_short,
    pub dqcoeff: *mut ::core::ffi::c_short,
    pub predictor: *mut ::core::ffi::c_uchar,
    pub dequant: *mut ::core::ffi::c_short,
    pub offset: ::core::ffi::c_int,
    pub eob: *mut ::core::ffi::c_char,
    pub bmi: b_mode_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
pub type uint32_t = u32;
pub type B_PREDICTION_MODE = ::core::ffi::c_uint;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type BLOCKD = blockd;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dequantize_b_c(
    mut d: *mut BLOCKD,
    mut DQC: *mut ::core::ffi::c_short,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut DQ: *mut ::core::ffi::c_short = (*d).dqcoeff;
    let mut Q: *mut ::core::ffi::c_short = (*d).qcoeff;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *DQ.offset(i as isize) = (*Q.offset(i as isize) as ::core::ffi::c_int
            * *DQC.offset(i as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        i += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dequant_idct_add_c(
    mut input: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dest: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *input.offset(i as isize) = (*dq.offset(i as isize) as ::core::ffi::c_int
            * *input.offset(i as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        i += 1;
    }
    vp8_short_idct4x4llm_c(input, dest, stride, dest, stride);
    memset(
        input as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        32 as size_t,
    );
}}
