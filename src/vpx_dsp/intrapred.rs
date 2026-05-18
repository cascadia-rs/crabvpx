unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
#[inline]
unsafe extern "C" fn clip_pixel(mut val: ::core::ffi::c_int) -> uint8_t {
    return (if val > 255 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else if val < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        val
    }) as uint8_t;
}
#[inline]
unsafe extern "C" fn d207_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        r = 0 as ::core::ffi::c_int;
        while r < bs - 1 as ::core::ffi::c_int {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                (*left.offset(r as isize) as ::core::ffi::c_int
                    + *left.offset((r + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        *dst.offset(((bs - 1 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize) =
            *left.offset((bs - 1 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
        r = 0 as ::core::ffi::c_int;
        while r < bs - 2 as ::core::ffi::c_int {
            *dst.offset((r as ptrdiff_t * stride) as isize) = (*left.offset(r as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((r + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset((r + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as uint8_t;
            r += 1;
        }
        *dst.offset(((bs - 2 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize) =
            (*left.offset((bs - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(((bs - 1 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize) =
            *left.offset((bs - 1 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
        c = 0 as ::core::ffi::c_int;
        while c < bs - 2 as ::core::ffi::c_int {
            *dst.offset(
                ((bs - 1 as ::core::ffi::c_int) as ptrdiff_t * stride + c as ptrdiff_t) as isize,
            ) = *left.offset((bs - 1 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r = bs - 2 as ::core::ffi::c_int;
        while r >= 0 as ::core::ffi::c_int {
            c = 0 as ::core::ffi::c_int;
            while c < bs - 2 as ::core::ffi::c_int {
                *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = *dst.offset(
                    ((r + 1 as ::core::ffi::c_int) as ptrdiff_t * stride + c as ptrdiff_t
                        - 2 as ptrdiff_t) as isize,
                );
                c += 1;
            }
            r -= 1;
        }
    }
}
#[inline]
unsafe extern "C" fn d63_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        c = 0 as ::core::ffi::c_int;
        while c < bs {
            *dst.offset(c as isize) = (*above.offset(c as isize) as ::core::ffi::c_int
                + *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset((stride + c as ptrdiff_t) as isize) = (*above.offset(c as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((c + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as uint8_t;
            c += 1;
        }
        r = 2 as ::core::ffi::c_int;
        size = bs - 2 as ::core::ffi::c_int;
        while r < bs {
            memcpy(
                dst.offset(((r + 0 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize)
                    as *mut ::core::ffi::c_void,
                dst.offset((r >> 1 as ::core::ffi::c_int) as isize) as *const ::core::ffi::c_void,
                size as size_t,
            );
            memset(
                dst.offset(((r + 0 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize)
                    .offset(size as isize) as *mut ::core::ffi::c_void,
                *above.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
                (bs - size) as size_t,
            );
            memcpy(
                dst.offset(((r + 1 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize)
                    as *mut ::core::ffi::c_void,
                dst.offset(stride as isize)
                    .offset((r >> 1 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_void,
                size as size_t,
            );
            memset(
                dst.offset(((r + 1 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize)
                    .offset(size as isize) as *mut ::core::ffi::c_void,
                *above.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
                (bs - size) as size_t,
            );
            r += 2 as ::core::ffi::c_int;
            size -= 1;
        }
    }
}
#[inline]
unsafe extern "C" fn d45_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let above_right: uint8_t = *above.offset((bs - 1 as ::core::ffi::c_int) as isize);
        let dst_row0: *const uint8_t = dst;
        let mut x: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < bs - 1 as ::core::ffi::c_int {
            *dst.offset(x as isize) = (*above.offset(x as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((x + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((x + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
            x += 1;
        }
        *dst.offset((bs - 1 as ::core::ffi::c_int) as isize) = above_right;
        dst = dst.offset(stride as isize);
        x = 1 as ::core::ffi::c_int;
        size = bs - 2 as ::core::ffi::c_int;
        while x < bs {
            memcpy(
                dst as *mut ::core::ffi::c_void,
                dst_row0.offset(x as isize) as *const ::core::ffi::c_void,
                size as size_t,
            );
            memset(
                dst.offset(size as isize) as *mut ::core::ffi::c_void,
                above_right as ::core::ffi::c_int,
                (x + 1 as ::core::ffi::c_int) as size_t,
            );
            dst = dst.offset(stride as isize);
            x += 1;
            size -= 1;
        }
    }
}
#[inline]
unsafe extern "C" fn d117_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        c = 0 as ::core::ffi::c_int;
        while c < bs {
            *dst.offset(c as isize) = (*above.offset((c - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *above.offset(c as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            (*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        c = 1 as ::core::ffi::c_int;
        while c < bs {
            *dst.offset(c as isize) = (*above.offset((c - 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((c - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(c as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            (*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        r = 3 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset(((r - 2 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize) =
                (*left.offset((r - 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *left.offset((r - 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                    + *left.offset((r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        r = 2 as ::core::ffi::c_int;
        while r < bs {
            c = 1 as ::core::ffi::c_int;
            while c < bs {
                *dst.offset(c as isize) = *dst.offset(
                    (-(2 as ::core::ffi::c_int) as ptrdiff_t * stride + c as ptrdiff_t
                        - 1 as ptrdiff_t) as isize,
                );
                c += 1;
            }
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn d135_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut border: [uint8_t; 63] = [0; 63];
        i = 0 as ::core::ffi::c_int;
        while i < bs - 2 as ::core::ffi::c_int {
            border[i as usize] = (*left.offset((bs - 3 as ::core::ffi::c_int - i) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((bs - 2 as ::core::ffi::c_int - i) as isize)
                        as ::core::ffi::c_int
                + *left.offset((bs - 1 as ::core::ffi::c_int - i) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
        border[(bs - 2 as ::core::ffi::c_int) as usize] =
            (*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        border[(bs - 1 as ::core::ffi::c_int) as usize] =
            (*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        border[(bs - 0 as ::core::ffi::c_int) as usize] =
            (*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        i = 0 as ::core::ffi::c_int;
        while i < bs - 2 as ::core::ffi::c_int {
            border[(bs + 1 as ::core::ffi::c_int + i) as usize] = (*above.offset(i as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((i + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < bs {
            memcpy(
                dst.offset((i as ptrdiff_t * stride) as isize) as *mut ::core::ffi::c_void,
                (&raw mut border as *mut uint8_t)
                    .offset(bs as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(-(i as isize)) as *const ::core::ffi::c_void,
                bs as size_t,
            );
            i += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn d153_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            (*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
        r = 1 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                (*left.offset((r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *left.offset(r as isize) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            (*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(stride as isize) = (*above.offset(-(1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        r = 2 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                (*left.offset((r - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *left.offset((r - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                    + *left.offset(r as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        c = 0 as ::core::ffi::c_int;
        while c < bs - 2 as ::core::ffi::c_int {
            *dst.offset(c as isize) = (*above.offset((c - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * *above.offset(c as isize) as ::core::ffi::c_int
                + *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r = 1 as ::core::ffi::c_int;
        while r < bs {
            c = 0 as ::core::ffi::c_int;
            while c < bs - 2 as ::core::ffi::c_int {
                *dst.offset(c as isize) =
                    *dst.offset((-stride + c as ptrdiff_t - 2 as ptrdiff_t) as isize);
                c += 1;
            }
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memcpy(
                dst as *mut ::core::ffi::c_void,
                above as *const ::core::ffi::c_void,
                bs as size_t,
            );
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memset(
                dst as *mut ::core::ffi::c_void,
                *left.offset(r as isize) as ::core::ffi::c_int,
                bs as size_t,
            );
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn tm_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut ytop_left: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            c = 0 as ::core::ffi::c_int;
            while c < bs {
                *dst.offset(c as isize) = clip_pixel(
                    *left.offset(r as isize) as ::core::ffi::c_int
                        + *above.offset(c as isize) as ::core::ffi::c_int
                        - ytop_left,
                );
                c += 1;
            }
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_128_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memset(
                dst as *mut ::core::ffi::c_void,
                128 as ::core::ffi::c_int,
                bs as size_t,
            );
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_left_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut expected_dc: ::core::ffi::c_int = 0;
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < bs {
            sum += *left.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        expected_dc = (sum + (bs >> 1 as ::core::ffi::c_int)) / bs;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memset(dst as *mut ::core::ffi::c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_top_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut expected_dc: ::core::ffi::c_int = 0;
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < bs {
            sum += *above.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        expected_dc = (sum + (bs >> 1 as ::core::ffi::c_int)) / bs;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memset(dst as *mut ::core::ffi::c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut expected_dc: ::core::ffi::c_int = 0;
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let count: ::core::ffi::c_int = 2 as ::core::ffi::c_int * bs;
        i = 0 as ::core::ffi::c_int;
        while i < bs {
            sum += *above.offset(i as isize) as ::core::ffi::c_int;
            sum += *left.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        expected_dc = (sum + (count >> 1 as ::core::ffi::c_int)) / count;
        r = 0 as ::core::ffi::c_int;
        while r < bs {
            memset(dst as *mut ::core::ffi::c_void, expected_dc, bs as size_t);
            dst = dst.offset(stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_he_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let H: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        let I: ::core::ffi::c_int =
            *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *left.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let L: ::core::ffi::c_int =
            *left.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        memset(
            dst.offset((stride * 0 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            H + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset((stride * 1 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset((stride * 2 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset((stride * 3 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            K + 2 as ::core::ffi::c_int * L + L + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_ve_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let H: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        let I: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let L: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let M: ::core::ffi::c_int =
            *above.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            (H + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) =
            (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) =
            (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) =
            (K + 2 as ::core::ffi::c_int * L + M + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        memcpy(
            dst.offset((stride * 1 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset((stride * 2 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset((stride * 3 as ptrdiff_t) as isize) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d207_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: ::core::ffi::c_int =
            *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *left.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let L: ::core::ffi::c_int =
            *left.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (I + J + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh21 = *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh21 = (J + K + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh21;
        let ref mut fresh22 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh22 = (K + L + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh22;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh23 = *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh23 = (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh23;
        let ref mut fresh24 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh24 = (K + 2 as ::core::ffi::c_int * L + L + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh24;
        let ref mut fresh25 = *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh25 = L as uint8_t;
        let ref mut fresh26 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh26 = *fresh25;
        let ref mut fresh27 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh27 = *fresh26;
        let ref mut fresh28 = *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh28 = *fresh27;
        let ref mut fresh29 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh29 = *fresh28;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh29;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let E: ::core::ffi::c_int =
            *above.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let F: ::core::ffi::c_int =
            *above.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let G: ::core::ffi::c_int =
            *above.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (A + B + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh48 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh48 = (B + C + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh48;
        let ref mut fresh49 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh49 = (C + D + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh49;
        let ref mut fresh50 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh50 = (D + E + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh50;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) =
            (E + F + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) =
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh51 = *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh51 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh51;
        let ref mut fresh52 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh52 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh52;
        let ref mut fresh53 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh53 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh53;
        *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let E: ::core::ffi::c_int =
            *above.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let F: ::core::ffi::c_int =
            *above.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let G: ::core::ffi::c_int =
            *above.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let H: ::core::ffi::c_int =
            *above.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (A + B + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh54 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh54 = (B + C + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh54;
        let ref mut fresh55 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh55 = (C + D + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh55;
        let ref mut fresh56 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh56 = (D + E + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh56;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) =
            (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) =
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh57 = *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh57 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh57;
        let ref mut fresh58 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh58 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh58;
        let ref mut fresh59 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh59 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh59;
        *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let E: ::core::ffi::c_int =
            *above.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let F: ::core::ffi::c_int =
            *above.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let G: ::core::ffi::c_int =
            *above.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let H: ::core::ffi::c_int =
            *above.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh30 = *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh30 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh30;
        let ref mut fresh31 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh31 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh32 = *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh32 = *fresh31;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh32;
        let ref mut fresh33 = *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh33 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh34 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh34 = *fresh33;
        let ref mut fresh35 = *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh35 = *fresh34;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh35;
        let ref mut fresh36 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh36 = (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh37 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh37 = *fresh36;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh37;
        let ref mut fresh38 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh38 = (F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh38;
        *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = H as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let E: ::core::ffi::c_int =
            *above.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let F: ::core::ffi::c_int =
            *above.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let G: ::core::ffi::c_int =
            *above.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let H: ::core::ffi::c_int =
            *above.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh39 = *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh39 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh39;
        let ref mut fresh40 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh40 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh41 = *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh41 = *fresh40;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh41;
        let ref mut fresh42 = *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh42 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh43 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh43 = *fresh42;
        let ref mut fresh44 = *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh44 = *fresh43;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh44;
        let ref mut fresh45 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh45 = (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh46 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh46 = *fresh45;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh46;
        let ref mut fresh47 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh47 = (F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh47;
        *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (G + 2 as ::core::ffi::c_int * H + H + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: ::core::ffi::c_int =
            *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *left.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let X: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let ref mut fresh0 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh0 = (X + A + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh0;
        let ref mut fresh1 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh1 = (A + B + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh1;
        let ref mut fresh2 = *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh2 = (B + C + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh2;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (C + D + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) =
            (J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh3 = *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh3 = (I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh3;
        let ref mut fresh4 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh4 = (X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh4;
        let ref mut fresh5 = *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh5 = (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh5;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) =
            (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: ::core::ffi::c_int =
            *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *left.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let L: ::core::ffi::c_int =
            *left.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let X: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let D: ::core::ffi::c_int =
            *above.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh6 = *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh6 = (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh6;
        let ref mut fresh7 = *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh7 = (X + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh8 = *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh8 = *fresh7;
        *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh8;
        let ref mut fresh9 = *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize);
        *fresh9 = (A + 2 as ::core::ffi::c_int * X + I + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh10 = *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh10 = *fresh9;
        let ref mut fresh11 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh11 = *fresh10;
        *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh11;
        let ref mut fresh12 = *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize);
        *fresh12 = (B + 2 as ::core::ffi::c_int * A + X + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh13 = *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh13 = *fresh12;
        *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh13;
        let ref mut fresh14 = *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize);
        *fresh14 = (C + 2 as ::core::ffi::c_int * B + A + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh14;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (D + 2 as ::core::ffi::c_int * C + B + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let I: ::core::ffi::c_int =
            *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let J: ::core::ffi::c_int =
            *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let K: ::core::ffi::c_int =
            *left.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let L: ::core::ffi::c_int =
            *left.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let X: ::core::ffi::c_int =
            *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        let A: ::core::ffi::c_int =
            *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let B: ::core::ffi::c_int =
            *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let C: ::core::ffi::c_int =
            *above.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let ref mut fresh15 = *dst.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh15 = (I + X + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh15;
        let ref mut fresh16 = *dst.offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh16 = (J + I + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh16;
        let ref mut fresh17 = *dst.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh17 = (K + J + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh17;
        *dst.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (L + K + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) =
            (X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh18 = *dst.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
        *fresh18 = (I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh18;
        let ref mut fresh19 = *dst.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
        *fresh19 = (J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh19;
        let ref mut fresh20 = *dst.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
        *fresh20 = (K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh20;
        *dst.offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) =
            (L + 2 as ::core::ffi::c_int * K + J + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d207_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d207_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d207_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d207_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d63_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d45_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d117_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d135_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        d153_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        v_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        h_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_tm_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_tm_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_tm_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_tm_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        tm_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_128_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_left_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_top_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        dc_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
    }
}
