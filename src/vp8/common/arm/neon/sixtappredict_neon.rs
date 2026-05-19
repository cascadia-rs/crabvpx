use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint32x2x2_t {
    pub val: [uint32x2_t; 2],
}
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
static vp8_sub_pel_filters: [[int8_t; 8]; 8] = [
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        -(128 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
        123 as ::core::ffi::c_int as int8_t,
        12 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
        108 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
        1 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        -(9 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        -(16 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
        77 as ::core::ffi::c_int as int8_t,
        -(16 as ::core::ffi::c_int) as int8_t,
        3 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
        93 as ::core::ffi::c_int as int8_t,
        -(9 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
        36 as ::core::ffi::c_int as int8_t,
        108 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
        2 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
        12 as ::core::ffi::c_int as int8_t,
        123 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
];
static abs_filters: [[uint8_t; 8]; 8] = [
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        128 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        123 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        2 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        93 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        3 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        77 as ::core::ffi::c_int as uint8_t,
        77 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        93 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        1 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        123 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
];
