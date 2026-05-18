use std::ffi::c_void;
use std::arch::aarch64::*;
extern "Rust" {
}
pub type DarwinPtrdiffT = isize;
pub type DarwinSizeT = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uint32x2x2T {
    pub val: [uint32x2_t; 2],
}
pub type SizeT = DarwinSizeT;
pub type PtrdiffT = DarwinPtrdiffT;
#[inline]
unsafe fn uint32_to_mem(mut buf: *mut u8, mut a: u32) {
    core::ptr::copy_nonoverlapping(&raw mut a as *const c_void as *const u8, buf as *mut c_void as *mut u8, 4 as SizeT);
}
static mut vp8_sub_pel_filters: [[i8; 8]; 8] = [
    [
        0 as i8,
        0 as i8,
        -(128 as i32) as i8,
        0 as i8,
        0 as i8,
        0 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        0 as i8,
        -(6 as i32) as i8,
        123 as i8,
        12 as i8,
        -(1 as i32) as i8,
        0 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        2 as i8,
        -(11 as i32) as i8,
        108 as i8,
        36 as i8,
        -(8 as i32) as i8,
        1 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        0 as i8,
        -(9 as i32) as i8,
        93 as i8,
        50 as i8,
        -(6 as i32) as i8,
        0 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        3 as i8,
        -(16 as i32) as i8,
        77 as i8,
        77 as i8,
        -(16 as i32) as i8,
        3 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        0 as i8,
        -(6 as i32) as i8,
        50 as i8,
        93 as i8,
        -(9 as i32) as i8,
        0 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        1 as i8,
        -(8 as i32) as i8,
        36 as i8,
        108 as i8,
        -(11 as i32) as i8,
        2 as i8,
        0 as i8,
        0 as i8,
    ],
    [
        0 as i8,
        -(1 as i32) as i8,
        12 as i8,
        123 as i8,
        -(6 as i32) as i8,
        0 as i8,
        0 as i8,
        0 as i8,
    ],
];
static mut abs_filters: [[u8; 8]; 8] = [
    [
        0 as u8,
        0 as u8,
        128 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        0 as u8,
        6 as u8,
        123 as u8,
        12 as u8,
        1 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        2 as u8,
        11 as u8,
        108 as u8,
        36 as u8,
        8 as u8,
        1 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        0 as u8,
        9 as u8,
        93 as u8,
        50 as u8,
        6 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        3 as u8,
        16 as u8,
        77 as u8,
        77 as u8,
        16 as u8,
        3 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        0 as u8,
        6 as u8,
        50 as u8,
        93 as u8,
        9 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        1 as u8,
        8 as u8,
        36 as u8,
        108 as u8,
        11 as u8,
        2 as u8,
        0 as u8,
        0 as u8,
    ],
    [
        0 as u8,
        1 as u8,
        12 as u8,
        123 as u8,
        6 as u8,
        0 as u8,
        0 as u8,
        0 as u8,
    ],
];
