use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int16x8x2_t {
    pub val: [int16x8_t; 2],
}
pub use crate::vp8::common::types::*;
