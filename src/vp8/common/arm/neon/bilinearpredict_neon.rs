use std::ffi::c_void;
use std::arch::aarch64::*;
extern "Rust" {
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint32x2x2T {
    pub val: [uint32x2_t; 2],
}
#[inline]
fn uint32_to_mem(mut buf: *mut u8, mut a: u32) {
    core::ptr::copy_nonoverlapping(&raw mut a as *const c_void as *const u8, buf as *mut c_void as *mut u8, 4 as usize);
}
static bifilter4_coeff: [[u8; 2]; 8] = [
    [
        128 as u8,
        0 as u8,
    ],
    [
        112 as u8,
        16 as u8,
    ],
    [
        96 as u8,
        32 as u8,
    ],
    [
        80 as u8,
        48 as u8,
    ],
    [
        64 as u8,
        64 as u8,
    ],
    [
        48 as u8,
        80 as u8,
    ],
    [
        32 as u8,
        96 as u8,
    ],
    [
        16 as u8,
        112 as u8,
    ],
];
