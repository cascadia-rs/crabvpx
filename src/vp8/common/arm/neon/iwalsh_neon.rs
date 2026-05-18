use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Int16x4x2T {
    pub val: [int16x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Int32x2x2T {
    pub val: [int32x2_t; 2],
}
