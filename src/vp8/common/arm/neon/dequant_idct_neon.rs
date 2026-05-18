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
static mut cospi8sqrt2minus1: i16 = 20091 as i16;
static mut sinpi8sqrt2: i16 =
    (35468 as i32 >> 1 as i32) as i16;
