use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x16x2T {
    pub val: [uint8x16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint16x8x2T {
    pub val: [uint16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint32x4x2T {
    pub val: [uint32x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x8x4T {
    pub val: [uint8x8_t; 4],
}
pub type LoopfilterYNeon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
) -> ();
pub type LoopfilterUvNeon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
    *mut u8,
) -> ();
