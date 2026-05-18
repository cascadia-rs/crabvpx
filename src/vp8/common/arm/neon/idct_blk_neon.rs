use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int16x8x2_t {
    pub val: [int16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int32x4x2_t {
    pub val: [int32x4_t; 2],
}
static mut cospi8sqrt2minus1: int16_t = 20091 as int16_t;
static mut sinpi8sqrt2: int16_t = 17734 as int16_t;
#[no_mangle]
pub unsafe fn vp8_dequant_idct_add_y_block_neon(
    mut q: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dst: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
    mut eobs: *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if *(eobs as *mut ::core::ffi::c_short).offset(0 as ::core::ffi::c_int as isize) != 0 {
            if *(eobs as *mut ::core::ffi::c_short).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0xfefe as ::core::ffi::c_int
                != 0
            {
                idct_dequant_full_2x_neon(q as *mut int16_t, dq as *mut int16_t, dst, stride);
            } else {
                idct_dequant_0_2x_neon(
                    q as *mut int16_t,
                    *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                    dst,
                    stride,
                );
            }
        }
        if *(eobs as *mut ::core::ffi::c_short).offset(1 as ::core::ffi::c_int as isize) != 0 {
            if *(eobs as *mut ::core::ffi::c_short).offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0xfefe as ::core::ffi::c_int
                != 0
            {
                idct_dequant_full_2x_neon(
                    q.offset(32 as ::core::ffi::c_int as isize),
                    dq as *mut int16_t,
                    dst.offset(8 as ::core::ffi::c_int as isize),
                    stride,
                );
            } else {
                idct_dequant_0_2x_neon(
                    q.offset(32 as ::core::ffi::c_int as isize),
                    *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                    dst.offset(8 as ::core::ffi::c_int as isize),
                    stride,
                );
            }
        }
        q = q.offset(64 as ::core::ffi::c_int as isize);
        dst = dst.offset((4 as ::core::ffi::c_int * stride) as isize);
        eobs = eobs.offset(4 as ::core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe fn vp8_dequant_idct_add_uv_block_neon(
    mut q: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dst_u: *mut ::core::ffi::c_uchar,
    mut dst_v: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
    mut eobs: *mut ::core::ffi::c_char,
) {
    if *(eobs as *mut ::core::ffi::c_short).offset(0 as ::core::ffi::c_int as isize) != 0 {
        if *(eobs as *mut ::core::ffi::c_short).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0xfefe as ::core::ffi::c_int
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut int16_t, dq as *mut int16_t, dst_u, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut int16_t,
                *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                dst_u,
                stride,
            );
        }
    }
    q = q.offset(32 as ::core::ffi::c_int as isize);
    dst_u = dst_u.offset((4 as ::core::ffi::c_int * stride) as isize);
    if *(eobs as *mut ::core::ffi::c_short).offset(1 as ::core::ffi::c_int as isize) != 0 {
        if *(eobs as *mut ::core::ffi::c_short).offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0xfefe as ::core::ffi::c_int
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut int16_t, dq as *mut int16_t, dst_u, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut int16_t,
                *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                dst_u,
                stride,
            );
        }
    }
    q = q.offset(32 as ::core::ffi::c_int as isize);
    if *(eobs as *mut ::core::ffi::c_short).offset(2 as ::core::ffi::c_int as isize) != 0 {
        if *(eobs as *mut ::core::ffi::c_short).offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0xfefe as ::core::ffi::c_int
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut int16_t, dq as *mut int16_t, dst_v, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut int16_t,
                *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                dst_v,
                stride,
            );
        }
    }
    q = q.offset(32 as ::core::ffi::c_int as isize);
    dst_v = dst_v.offset((4 as ::core::ffi::c_int * stride) as isize);
    if *(eobs as *mut ::core::ffi::c_short).offset(3 as ::core::ffi::c_int as isize) != 0 {
        if *(eobs as *mut ::core::ffi::c_short).offset(3 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0xfefe as ::core::ffi::c_int
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut int16_t, dq as *mut int16_t, dst_v, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut int16_t,
                *dq.offset(0 as ::core::ffi::c_int as isize) as int16_t,
                dst_v,
                stride,
            );
        }
    }
}
