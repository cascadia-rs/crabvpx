use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x16x2_t {
    pub val: [uint8x16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint16x8x2_t {
    pub val: [uint16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint32x4x2_t {
    pub val: [uint32x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x8x4_t {
    pub val: [uint8x8_t; 4],
}
pub type loopfilter_y_neon = unsafe fn(
    *mut ::core::ffi::c_uchar,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
) -> ();
pub type loopfilter_uv_neon = unsafe fn(
    *mut ::core::ffi::c_uchar,
    ::core::ffi::c_int,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    ::core::ffi::c_uchar,
    *mut ::core::ffi::c_uchar,
) -> ();
