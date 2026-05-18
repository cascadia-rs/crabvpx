use std::ffi::c_void;
use std::arch::aarch64::*;
unsafe extern "Rust" {
    fn memcpy(
        __dst: *mut c_void,
        __src: *const c_void,
        __n: size_t,
    ) -> *mut c_void;
}
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x8x2_t {
    pub val: [uint8x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x16x2_t {
    pub val: [uint8x16_t; 2],
}
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
#[inline]
unsafe fn load_replicate_u8_4x1(mut buf: *const uint8_t) -> uint8x8_t {
    let mut a: uint32_t = 0;
    memcpy(
        &raw mut a as *mut c_void,
        buf as *const c_void,
        4 as size_t,
    );
    return vreinterpret_u8_u32(vdup_n_u32(a));
}
#[inline]
unsafe fn load_unaligned_u8_4x1(mut buf: *const uint8_t) -> uint8x8_t {
    let mut a: uint32_t = 0;
    memcpy(
        &raw mut a as *mut c_void,
        buf as *const c_void,
        4 as size_t,
    );
    let mut a_u32 = vdup_n_u32(0);
    a_u32 = vset_lane_u32(a, a_u32, 0);
    return vreinterpret_u8_u32(a_u32);
}
#[inline]
unsafe fn horizontal_add_uint8x4(a: uint8x8_t) -> uint16_t {
    return vaddlv_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint8x8(a: uint8x8_t) -> uint16_t {
    return vaddlv_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint8x16(a: uint8x16_t) -> uint16_t {
    return vaddlvq_u8(a);
}
#[inline]
unsafe fn horizontal_add_uint16x4(a: uint16x4_t) -> uint16_t {
    return vaddv_u16(a);
}
#[inline]
unsafe fn horizontal_add_uint16x8(a: uint16x8_t) -> uint32_t {
    return vaddlvq_u16(a);
}
#[inline]
unsafe fn dc_sum_4(mut ref_0: *const uint8_t) -> uint16_t {
    return horizontal_add_uint8x4(load_unaligned_u8_4x1(ref_0));
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_4x4_neon(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    let d: uint32_t = *(above as *const uint32_t);
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *(dst as *mut uint32_t) = d;
        i += 1;
        dst = dst.offset(stride as isize);
    }
}
#[inline]
unsafe fn convert_u8_to_s16(mut v: uint8x8_t) -> int16x8_t {
    return vreinterpretq_s16_u16(vmovl_u8(v));
}
