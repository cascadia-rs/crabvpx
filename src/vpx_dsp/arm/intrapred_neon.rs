use std::ffi::c_void;
use std::arch::aarch64::*;
unsafe extern "Rust" {
}
pub type DarwinPtrdiffT = isize;
pub type DarwinSizeT = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x8x2T {
    pub val: [uint8x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint8x16x2T {
    pub val: [uint8x16_t; 2],
}
pub type PtrdiffT = DarwinPtrdiffT;
pub type SizeT = DarwinSizeT;
#[inline]
unsafe fn load_replicate_u8_4x1(mut buf: *const u8) -> uint8x8_t {
    let mut a: u32 = 0;
    core::ptr::copy_nonoverlapping(buf as *const c_void as *const u8, &raw mut a as *mut c_void as *mut u8, 4 as SizeT);
    return vreinterpret_u8_u32(vdup_n_u32(a));
}
#[inline]
unsafe fn load_unaligned_u8_4x1(mut buf: *const u8) -> uint8x8_t {
    let mut a: u32 = 0;
    core::ptr::copy_nonoverlapping(buf as *const c_void as *const u8, &raw mut a as *mut c_void as *mut u8, 4 as SizeT);
    let mut a_u32 = vdup_n_u32(0);
    a_u32 = vset_lane_u32(a, a_u32, 0);
    return vreinterpret_u8_u32(a_u32);
}
#[inline]
unsafe fn horizontal_add_uint8x4(a: uint8x8_t) -> u16 {
    return vaddlv_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint8x8(a: uint8x8_t) -> u16 {
    return vaddlv_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint8x16(a: uint8x16_t) -> u16 {
    return vaddlvq_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint16x4(a: uint16x4_t) -> u16 {
    return vaddv_u16(a);
}
#[inline]
unsafe fn horizontal_add_uint16x8(a: uint16x8_t) -> u32 {
    return vaddlvq_u16(a);
}
#[inline]
unsafe fn dc_sum_4(mut ref_0: *const u8) -> u16 {
    return horizontal_add_uint8x4(load_unaligned_u8_4x1(ref_0));
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_4x4_neon(
    mut dst: *mut u8,
    mut stride: PtrdiffT,
    mut above: *const u8,
    mut left: *const u8,
) {
    let d: u32 = *(above as *const u32);
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *(dst as *mut u32) = d;
        i += 1;
        dst = dst.offset(stride as isize);
    }
}
#[inline]
unsafe fn convert_u8_to_s16(mut v: uint8x8_t) -> int16x8_t {
    return vreinterpretq_s16_u16(vmovl_u8(v));
}
