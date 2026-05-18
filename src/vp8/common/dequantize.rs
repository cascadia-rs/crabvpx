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
pub struct Blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
    pub bmi: BModeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: BPredictionMode,
    pub mv: IntMv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
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
pub type SizeT = DarwinSizeT;
pub type DarwinSizeT = usize;
pub type BLOCKD = Blockd;
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
        core::ptr::write_bytes(input as *mut c_void as *mut u8, 0 as i32 as u8, 32 as SizeT);
    }
}
