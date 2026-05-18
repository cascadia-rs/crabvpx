use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Int16x8x2T {
    pub val: [int16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Int32x4x2T {
    pub val: [int32x4_t; 2],
}
static mut cospi8sqrt2minus1: i16 = 20091 as i16;
static mut sinpi8sqrt2: i16 = 17734 as i16;
#[no_mangle]
pub unsafe fn vp8_dequant_idct_add_y_block_neon(
    mut q: *mut i16,
    mut dq: *mut i16,
    mut dst: *mut u8,
    mut stride: i32,
    mut eobs: *mut i8,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        if *(eobs as *mut i16).offset(0 as isize) != 0 {
            if *(eobs as *mut i16).offset(0 as isize)
                as i32
                & 0xfefe as i32
                != 0
            {
                idct_dequant_full_2x_neon(q as *mut i16, dq as *mut i16, dst, stride);
            } else {
                idct_dequant_0_2x_neon(
                    q as *mut i16,
                    *dq.offset(0 as isize) as i16,
                    dst,
                    stride,
                );
            }
        }
        if *(eobs as *mut i16).offset(1 as isize) != 0 {
            if *(eobs as *mut i16).offset(1 as isize)
                as i32
                & 0xfefe as i32
                != 0
            {
                idct_dequant_full_2x_neon(
                    q.offset(32 as isize),
                    dq as *mut i16,
                    dst.offset(8 as isize),
                    stride,
                );
            } else {
                idct_dequant_0_2x_neon(
                    q.offset(32 as isize),
                    *dq.offset(0 as isize) as i16,
                    dst.offset(8 as isize),
                    stride,
                );
            }
        }
        q = q.offset(64 as isize);
        dst = dst.offset((4 as i32 * stride) as isize);
        eobs = eobs.offset(4 as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe fn vp8_dequant_idct_add_uv_block_neon(
    mut q: *mut i16,
    mut dq: *mut i16,
    mut dst_u: *mut u8,
    mut dst_v: *mut u8,
    mut stride: i32,
    mut eobs: *mut i8,
) {
    if *(eobs as *mut i16).offset(0 as isize) != 0 {
        if *(eobs as *mut i16).offset(0 as isize)
            as i32
            & 0xfefe as i32
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut i16, dq as *mut i16, dst_u, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut i16,
                *dq.offset(0 as isize) as i16,
                dst_u,
                stride,
            );
        }
    }
    q = q.offset(32 as isize);
    dst_u = dst_u.offset((4 as i32 * stride) as isize);
    if *(eobs as *mut i16).offset(1 as isize) != 0 {
        if *(eobs as *mut i16).offset(1 as isize)
            as i32
            & 0xfefe as i32
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut i16, dq as *mut i16, dst_u, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut i16,
                *dq.offset(0 as isize) as i16,
                dst_u,
                stride,
            );
        }
    }
    q = q.offset(32 as isize);
    if *(eobs as *mut i16).offset(2 as isize) != 0 {
        if *(eobs as *mut i16).offset(2 as isize)
            as i32
            & 0xfefe as i32
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut i16, dq as *mut i16, dst_v, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut i16,
                *dq.offset(0 as isize) as i16,
                dst_v,
                stride,
            );
        }
    }
    q = q.offset(32 as isize);
    dst_v = dst_v.offset((4 as i32 * stride) as isize);
    if *(eobs as *mut i16).offset(3 as isize) != 0 {
        if *(eobs as *mut i16).offset(3 as isize)
            as i32
            & 0xfefe as i32
            != 0
        {
            idct_dequant_full_2x_neon(q as *mut i16, dq as *mut i16, dst_v, stride);
        } else {
            idct_dequant_0_2x_neon(
                q as *mut i16,
                *dq.offset(0 as isize) as i16,
                dst_v,
                stride,
            );
        }
    }
}
