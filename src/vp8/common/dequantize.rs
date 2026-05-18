use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_short_idct4x4llm_c(
        input: *mut i16,
        pred_ptr: *mut u8,
        pred_stride: i32,
        dst_ptr: *mut u8,
        dst_stride: i32,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
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
    pub row: i16,
    pub col: i16,
}
pub type uint32_t = u32;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type BLOCKD = blockd;
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequantize_b_c(mut d: *mut BLOCKD, mut dqc: *mut i16) {
    unsafe {
        let mut i: i32 = 0;
        let mut dq: *mut i16 = (*d).dqcoeff;
        let mut Q: *mut i16 = (*d).qcoeff;
        i = 0 as i32;
        while i < 16 as i32 {
            *dq.offset(i as isize) =
                (*Q.offset(i as isize) as i32 * *dqc.offset(i as isize) as i32) as i16;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequant_idct_add_c(
    mut input: *mut i16,
    mut dq: *mut i16,
    mut dest: *mut u8,
    mut stride: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 16 as i32 {
            *input.offset(i as isize) =
                (*dq.offset(i as isize) as i32 * *input.offset(i as isize) as i32) as i16;
            i += 1;
        }
        vp8_short_idct4x4llm_c(input, dest, stride, dest, stride);
        core::ptr::write_bytes(
            input as *mut c_void as *mut u8,
            0 as i32 as u8,
            32 as size_t,
        );
    }
}
