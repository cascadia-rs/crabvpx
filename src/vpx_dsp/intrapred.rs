use std::ffi::c_void;
unsafe extern "Rust" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn memset(__b: *mut c_void, __c: i32, __len: size_t) -> *mut c_void;
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
#[inline]
unsafe fn clip_pixel(mut val: i32) -> uint8_t {
    (if val > 255 as i32 {
        255 as i32
    } else if val < 0 as i32 {
        0 as i32
    } else {
        val
    }) as uint8_t
}
#[inline]
unsafe fn d207_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    _above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        r = 0 as i32;
        while r < bs - 1 as i32 {
            *dst.offset((r as ptrdiff_t * stride) as isize) = ((*left.offset(r as isize) as i32
                + *left.offset((r + 1 as i32) as isize) as i32
                + 1 as i32)
                >> 1 as i32)
                as uint8_t;
            r += 1;
        }
        *dst.offset((bs - 1 as i32) as ptrdiff_t * stride) = *left.offset((bs - 1 as i32) as isize);
        dst = dst.offset(1);
        r = 0 as i32;
        while r < bs - 2 as i32 {
            *dst.offset((r as ptrdiff_t * stride) as isize) = ((*left.offset(r as isize) as i32
                + 2 as i32 * *left.offset((r + 1 as i32) as isize) as i32
                + *left.offset((r + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32)
                as uint8_t;
            r += 1;
        }
        *dst.offset((bs - 2 as i32) as ptrdiff_t * stride) =
            ((*left.offset((bs - 2 as i32) as isize) as i32
                + 2 as i32 * *left.offset((bs - 1 as i32) as isize) as i32
                + *left.offset((bs - 1 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
        *dst.offset((bs - 1 as i32) as ptrdiff_t * stride) = *left.offset((bs - 1 as i32) as isize);
        dst = dst.offset(1);
        c = 0 as i32;
        while c < bs - 2 as i32 {
            *dst.offset(((bs - 1 as i32) as ptrdiff_t * stride + c as ptrdiff_t) as isize) =
                *left.offset((bs - 1 as i32) as isize);
            c += 1;
        }
        r = bs - 2 as i32;
        while r >= 0 as i32 {
            c = 0 as i32;
            while c < bs - 2 as i32 {
                *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = *dst.offset(
                    ((r + 1 as i32) as ptrdiff_t * stride + c as ptrdiff_t - 2 as ptrdiff_t)
                        as isize,
                );
                c += 1;
            }
            r -= 1;
        }
    }
}
#[inline]
unsafe fn d63_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut size: i32 = 0;
        c = 0 as i32;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset(c as isize) as i32
                + *above.offset((c + 1 as i32) as isize) as i32
                + 1 as i32)
                >> 1 as i32) as uint8_t;
            *dst.offset((stride + c as ptrdiff_t) as isize) = ((*above.offset(c as isize) as i32
                + 2 as i32 * *above.offset((c + 1 as i32) as isize) as i32
                + *above.offset((c + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32)
                as uint8_t;
            c += 1;
        }
        r = 2 as i32;
        size = bs - 2 as i32;
        while r < bs {
            memcpy(
                dst.offset(((r + 0 as i32) as ptrdiff_t * stride) as isize) as *mut c_void,
                dst.offset((r >> 1 as i32) as isize) as *const c_void,
                size as size_t,
            );
            memset(
                dst.offset(((r + 0 as i32) as ptrdiff_t * stride) as isize)
                    .offset(size as isize) as *mut c_void,
                *above.offset((bs - 1 as i32) as isize) as i32,
                (bs - size) as size_t,
            );
            memcpy(
                dst.offset(((r + 1 as i32) as ptrdiff_t * stride) as isize) as *mut c_void,
                dst.offset(stride).offset((r >> 1 as i32) as isize) as *const c_void,
                size as size_t,
            );
            memset(
                dst.offset(((r + 1 as i32) as ptrdiff_t * stride) as isize)
                    .offset(size as isize) as *mut c_void,
                *above.offset((bs - 1 as i32) as isize) as i32,
                (bs - size) as size_t,
            );
            r += 2 as i32;
            size -= 1;
        }
    }
}
#[inline]
unsafe fn d45_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let above_right: uint8_t = *above.offset((bs - 1 as i32) as isize);
        let dst_row0: *const uint8_t = dst;
        let mut x: i32 = 0;
        let mut size: i32 = 0;
        x = 0 as i32;
        while x < bs - 1 as i32 {
            *dst.offset(x as isize) = ((*above.offset(x as isize) as i32
                + 2 as i32 * *above.offset((x + 1 as i32) as isize) as i32
                + *above.offset((x + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
            x += 1;
        }
        *dst.offset((bs - 1 as i32) as isize) = above_right;
        dst = dst.offset(stride);
        x = 1 as i32;
        size = bs - 2 as i32;
        while x < bs {
            memcpy(
                dst as *mut c_void,
                dst_row0.offset(x as isize) as *const c_void,
                size as size_t,
            );
            memset(
                dst.offset(size as isize) as *mut c_void,
                above_right as i32,
                (x + 1 as i32) as size_t,
            );
            dst = dst.offset(stride);
            x += 1;
            size -= 1;
        }
    }
}
#[inline]
unsafe fn d117_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        c = 0 as i32;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset((c - 1 as i32) as isize) as i32
                + *above.offset(c as isize) as i32
                + 1 as i32)
                >> 1 as i32) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as isize) = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        c = 1 as i32;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset((c - 2 as i32) as isize) as i32
                + 2 as i32 * *above.offset((c - 1 as i32) as isize) as i32
                + *above.offset(c as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as isize) = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        r = 3 as i32;
        while r < bs {
            *dst.offset(((r - 2 as i32) as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 3 as i32) as isize) as i32
                    + 2 as i32 * *left.offset((r - 2 as i32) as isize) as i32
                    + *left.offset((r - 1 as i32) as isize) as i32
                    + 2 as i32)
                    >> 2 as i32) as uint8_t;
            r += 1;
        }
        r = 2 as i32;
        while r < bs {
            c = 1 as i32;
            while c < bs {
                *dst.offset(c as isize) = *dst.offset(
                    (-(2 as i32) as ptrdiff_t * stride + c as ptrdiff_t - 1 as ptrdiff_t) as isize,
                );
                c += 1;
            }
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn d135_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut border: [uint8_t; 63] = [0; 63];
        i = 0 as i32;
        while i < bs - 2 as i32 {
            border[i as usize] = ((*left.offset((bs - 3 as i32 - i) as isize) as i32
                + 2 as i32 * *left.offset((bs - 2 as i32 - i) as isize) as i32
                + *left.offset((bs - 1 as i32 - i) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
            i += 1;
        }
        border[(bs - 2 as i32) as usize] = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        border[(bs - 1 as i32) as usize] = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        border[(bs - 0 as i32) as usize] = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *above.offset(0 as isize) as i32
            + *above.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        i = 0 as i32;
        while i < bs - 2 as i32 {
            border[(bs + 1 as i32 + i) as usize] = ((*above.offset(i as isize) as i32
                + 2 as i32 * *above.offset((i + 1 as i32) as isize) as i32
                + *above.offset((i + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
            i += 1;
        }
        i = 0 as i32;
        while i < bs {
            memcpy(
                dst.offset((i as ptrdiff_t * stride) as isize) as *mut c_void,
                (&raw mut border as *mut uint8_t)
                    .offset(bs as isize)
                    .offset(-(1 as isize))
                    .offset(-(i as isize)) as *const c_void,
                bs as size_t,
            );
            i += 1;
        }
    }
}
#[inline]
unsafe fn d153_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        *dst.offset(0 as isize) = ((*above.offset(-(1 as i32) as isize) as i32
            + *left.offset(0 as isize) as i32
            + 1 as i32)
            >> 1 as i32) as uint8_t;
        r = 1 as i32;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 1 as i32) as isize) as i32
                    + *left.offset(r as isize) as i32
                    + 1 as i32)
                    >> 1 as i32) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        *dst.offset(0 as isize) = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        *dst.offset(stride) = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as uint8_t;
        r = 2 as i32;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 2 as i32) as isize) as i32
                    + 2 as i32 * *left.offset((r - 1 as i32) as isize) as i32
                    + *left.offset(r as isize) as i32
                    + 2 as i32)
                    >> 2 as i32) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        c = 0 as i32;
        while c < bs - 2 as i32 {
            *dst.offset(c as isize) = ((*above.offset((c - 1 as i32) as isize) as i32
                + 2 as i32 * *above.offset(c as isize) as i32
                + *above.offset((c + 1 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        r = 1 as i32;
        while r < bs {
            c = 0 as i32;
            while c < bs - 2 as i32 {
                *dst.offset(c as isize) =
                    *dst.offset((-stride + c as ptrdiff_t - 2 as ptrdiff_t) as isize);
                c += 1;
            }
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            memcpy(dst as *mut c_void, above as *const c_void, bs as size_t);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    _above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            memset(
                dst as *mut c_void,
                *left.offset(r as isize) as i32,
                bs as size_t,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn tm_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut ytop_left: i32 = *above.offset(-(1 as i32) as isize) as i32;
        r = 0 as i32;
        while r < bs {
            c = 0 as i32;
            while c < bs {
                *dst.offset(c as isize) = clip_pixel(
                    *left.offset(r as isize) as i32 + *above.offset(c as isize) as i32 - ytop_left,
                );
                c += 1;
            }
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_128_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    _above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            memset(dst as *mut c_void, 128 as i32, bs as size_t);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_left_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    _above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut expected_dc: i32 = 0;
        let mut sum: i32 = 0 as i32;
        i = 0 as i32;
        while i < bs {
            sum += *left.offset(i as isize) as i32;
            i += 1;
        }
        expected_dc = (sum + (bs >> 1 as i32)) / bs;
        r = 0 as i32;
        while r < bs {
            memset(dst as *mut c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_top_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut expected_dc: i32 = 0;
        let mut sum: i32 = 0 as i32;
        i = 0 as i32;
        while i < bs {
            sum += *above.offset(i as isize) as i32;
            i += 1;
        }
        expected_dc = (sum + (bs >> 1 as i32)) / bs;
        r = 0 as i32;
        while r < bs {
            memset(dst as *mut c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: i32,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut expected_dc: i32 = 0;
        let mut sum: i32 = 0 as i32;
        let count: i32 = 2 as i32 * bs;
        i = 0 as i32;
        while i < bs {
            sum += *above.offset(i as isize) as i32;
            sum += *left.offset(i as isize) as i32;
            i += 1;
        }
        expected_dc = (sum + (count >> 1 as i32)) / count;
        r = 0 as i32;
        while r < bs {
            memset(dst as *mut c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_he_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let H: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        memset(
            dst.offset(stride * 0 as ptrdiff_t) as *mut c_void,
            (H + 2 as i32 * I + J + 2 as i32) >> 2 as i32,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 1 as ptrdiff_t) as *mut c_void,
            (I + 2 as i32 * J + K + 2 as i32) >> 2 as i32,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 2 as ptrdiff_t) as *mut c_void,
            (J + 2 as i32 * K + L + 2 as i32) >> 2 as i32,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 3 as ptrdiff_t) as *mut c_void,
            (K + 2 as i32 * L + L + 2 as i32) >> 2 as i32,
            4 as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_ve_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let H: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let I: i32 = *above.offset(0 as isize) as i32;
        let J: i32 = *above.offset(1 as isize) as i32;
        let K: i32 = *above.offset(2 as isize) as i32;
        let L: i32 = *above.offset(3 as isize) as i32;
        let M: i32 = *above.offset(4 as isize) as i32;
        *dst.offset(0 as isize) = ((H + 2 as i32 * I + J + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as isize) = ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(2 as isize) = ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as isize) = ((K + 2 as i32 * L + M + 2 as i32) >> 2 as i32) as uint8_t;
        memcpy(
            dst.offset(stride * 1 as ptrdiff_t) as *mut c_void,
            dst as *const c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset(stride * 2 as ptrdiff_t) as *mut c_void,
            dst as *const c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset(stride * 3 as ptrdiff_t) as *mut c_void,
            dst as *const c_void,
            4 as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    _above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((I + J + 1 as i32) >> 1 as i32) as uint8_t;
        let fresh21 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh21 = ((J + K + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh21;
        let fresh22 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh22 = ((K + L + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh22;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh23 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh23 = ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh23;
        let fresh24 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh24 = ((K + 2 as i32 * L + L + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh24;
        let fresh25 = &mut *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh25 = L as uint8_t;
        let fresh26 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh26 = *fresh25;
        let fresh27 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh27 = *fresh26;
        let fresh28 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh28 = *fresh27;
        let fresh29 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh29 = *fresh28;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh29;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let E: i32 = *above.offset(4 as isize) as i32;
        let F: i32 = *above.offset(5 as isize) as i32;
        let G: i32 = *above.offset(6 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + B + 1 as i32) >> 1 as i32) as uint8_t;
        let fresh48 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh48 = ((B + C + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh48;
        let fresh49 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh49 = ((C + D + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh49;
        let fresh50 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh50 = ((D + E + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh50;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((E + F + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh51 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh51 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh51;
        let fresh52 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh52 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh52;
        let fresh53 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh53 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh53;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let E: i32 = *above.offset(4 as isize) as i32;
        let F: i32 = *above.offset(5 as isize) as i32;
        let G: i32 = *above.offset(6 as isize) as i32;
        let H: i32 = *above.offset(7 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + B + 1 as i32) >> 1 as i32) as uint8_t;
        let fresh54 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh54 = ((B + C + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh54;
        let fresh55 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh55 = ((C + D + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh55;
        let fresh56 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh56 = ((D + E + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh56;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh57 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh57 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh57;
        let fresh58 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh58 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh58;
        let fresh59 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh59 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh59;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let E: i32 = *above.offset(4 as isize) as i32;
        let F: i32 = *above.offset(5 as isize) as i32;
        let G: i32 = *above.offset(6 as isize) as i32;
        let H: i32 = *above.offset(7 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh30 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh30 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh30;
        let fresh31 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh31 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh32 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh32 = *fresh31;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh32;
        let fresh33 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh33 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh34 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh34 = *fresh33;
        let fresh35 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh35 = *fresh34;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh35;
        let fresh36 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh36 = ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh37 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh37 = *fresh36;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh37;
        let fresh38 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh38 = ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh38;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) = H as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
) {
    unsafe {
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let E: i32 = *above.offset(4 as isize) as i32;
        let F: i32 = *above.offset(5 as isize) as i32;
        let G: i32 = *above.offset(6 as isize) as i32;
        let H: i32 = *above.offset(7 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh39 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh39 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh39;
        let fresh40 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh40 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh41 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh41 = *fresh40;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh41;
        let fresh42 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh42 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh43 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh43 = *fresh42;
        let fresh44 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh44 = *fresh43;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh44;
        let fresh45 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh45 = ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh46 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh46 = *fresh45;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh46;
        let fresh47 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh47 = ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh47;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((G + 2 as i32 * H + H + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let X: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let fresh0 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh0 = ((X + A + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh0;
        let fresh1 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh1 = ((A + B + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh1;
        let fresh2 = &mut *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh2 = ((B + C + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh2;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((C + D + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((K + 2 as i32 * J + I + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((J + 2 as i32 * I + X + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh3 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh3 = ((I + 2 as i32 * X + A + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh3;
        let fresh4 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh4 = ((X + 2 as i32 * A + B + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh4;
        let fresh5 = &mut *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh5 = ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh5;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        let X: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh6 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh6 = ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh6;
        let fresh7 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh7 = ((X + 2 as i32 * I + J + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh8 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh8 = *fresh7;
        *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh8;
        let fresh9 = &mut *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh9 = ((A + 2 as i32 * X + I + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh10 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh10 = *fresh9;
        let fresh11 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh11 = *fresh10;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh11;
        let fresh12 = &mut *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh12 = ((B + 2 as i32 * A + X + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh13 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh13 = *fresh12;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh13;
        let fresh14 = &mut *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh14 = ((C + 2 as i32 * B + A + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh14;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((D + 2 as i32 * C + B + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        let X: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let fresh15 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh15 = ((I + X + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh15;
        let fresh16 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh16 = ((J + I + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh16;
        let fresh17 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh17 = ((K + J + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh17;
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((L + K + 1 as i32) >> 1 as i32) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((X + 2 as i32 * A + B + 2 as i32) >> 2 as i32) as uint8_t;
        let fresh18 = &mut *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh18 = ((I + 2 as i32 * X + A + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh18;
        let fresh19 = &mut *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh19 = ((J + 2 as i32 * I + X + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh19;
        let fresh20 = &mut *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh20 = ((K + 2 as i32 * J + I + 2 as i32) >> 2 as i32) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh20;
        *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((L + 2 as i32 * K + J + 2 as i32) >> 2 as i32) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 16 as i32, above, left);
    }
}
