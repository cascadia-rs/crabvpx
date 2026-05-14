pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int16x4x2_t {
    pub val: [int16x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int32x2x2_t {
    pub val: [int32x2_t; 2],
}
static mut cospi8sqrt2minus1: int16_t = 20091 as int16_t;
static mut sinpi8sqrt2: int16_t =
    (35468 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as int16_t;
