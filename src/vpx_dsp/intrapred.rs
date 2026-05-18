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
    (if val > 255 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else if val < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        val
    }) as uint8_t
}
#[inline]
unsafe extern "C" fn d207_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    _above: *const uint8_t,
    mut left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        r = 0 as ::core::ffi::c_int;
        while r < bs - 1 as ::core::ffi::c_int {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                ((*left.offset(r as isize) as ::core::ffi::c_int
                    + *left.offset((r + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        *dst.offset((bs - 1 as ::core::ffi::c_int) as ptrdiff_t * stride) =
            *left.offset((bs - 1 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
        r = 0 as ::core::ffi::c_int;
        while r < bs - 2 as ::core::ffi::c_int {
            *dst.offset((r as ptrdiff_t * stride) as isize) = ((*left.offset(r as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((r + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset((r + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int)
                as uint8_t;
            r += 1;
        }
        *dst.offset((bs - 2 as ::core::ffi::c_int) as ptrdiff_t * stride) =
            ((*left.offset((bs - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset((bs - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset((bs - 1 as ::core::ffi::c_int) as ptrdiff_t * stride) =
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
    _left: *const uint8_t,
) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        c = 0 as ::core::ffi::c_int;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset(c as isize) as ::core::ffi::c_int
                + *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset((stride + c as ptrdiff_t) as isize) = ((*above.offset(c as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((c + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
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
                dst.offset(stride)
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
    _left: *const uint8_t,
) {
    unsafe {
        let above_right: uint8_t = *above.offset((bs - 1 as ::core::ffi::c_int) as isize);
        let dst_row0: *const uint8_t = dst;
        let mut x: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < bs - 1 as ::core::ffi::c_int {
            *dst.offset(x as isize) = ((*above.offset(x as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((x + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((x + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
            x += 1;
        }
        *dst.offset((bs - 1 as ::core::ffi::c_int) as isize) = above_right;
        dst = dst.offset(stride);
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
            dst = dst.offset(stride);
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
            *dst.offset(c as isize) = ((*above.offset((c - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *above.offset(c as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            ((*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        c = 1 as ::core::ffi::c_int;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset((c - 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((c - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(c as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            ((*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        r = 3 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset(((r - 2 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *left.offset((r - 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                    + *left.offset((r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int)
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
            dst = dst.offset(stride);
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
            border[i as usize] = ((*left.offset((bs - 3 as ::core::ffi::c_int - i) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset((bs - 2 as ::core::ffi::c_int - i) as isize)
                        as ::core::ffi::c_int
                + *left.offset((bs - 1 as ::core::ffi::c_int - i) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
        border[(bs - 2 as ::core::ffi::c_int) as usize] =
            ((*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        border[(bs - 1 as ::core::ffi::c_int) as usize] =
            ((*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        border[(bs - 0 as ::core::ffi::c_int) as usize] =
            ((*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *above.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        i = 0 as ::core::ffi::c_int;
        while i < bs - 2 as ::core::ffi::c_int {
            border[(bs + 1 as ::core::ffi::c_int + i) as usize] = ((*above.offset(i as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((i + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
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
            ((*above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as uint8_t;
        r = 1 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *left.offset(r as isize) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        *dst.offset(0 as ::core::ffi::c_int as isize) =
            ((*left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(stride) = ((*above.offset(-(1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        r = 2 as ::core::ffi::c_int;
        while r < bs {
            *dst.offset((r as ptrdiff_t * stride) as isize) =
                ((*left.offset((r - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *left.offset((r - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                    + *left.offset(r as isize) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int)
                    >> 2 as ::core::ffi::c_int) as uint8_t;
            r += 1;
        }
        dst = dst.offset(1);
        c = 0 as ::core::ffi::c_int;
        while c < bs - 2 as ::core::ffi::c_int {
            *dst.offset(c as isize) = ((*above.offset((c - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * *above.offset(c as isize) as ::core::ffi::c_int
                + *above.offset((c + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride);
        r = 1 as ::core::ffi::c_int;
        while r < bs {
            c = 0 as ::core::ffi::c_int;
            while c < bs - 2 as ::core::ffi::c_int {
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
unsafe extern "C" fn v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    _left: *const uint8_t,
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
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    _above: *const uint8_t,
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
            dst = dst.offset(stride);
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
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_128_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    _above: *const uint8_t,
    _left: *const uint8_t,
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
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn dc_left_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    _above: *const uint8_t,
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
            dst = dst.offset(stride);
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
    _left: *const uint8_t,
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
            dst = dst.offset(stride);
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
            dst = dst.offset(stride);
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
            dst.offset(stride * 0 as ptrdiff_t) as *mut ::core::ffi::c_void,
            (H + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 1 as ptrdiff_t) as *mut ::core::ffi::c_void,
            (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 2 as ptrdiff_t) as *mut ::core::ffi::c_void,
            (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int,
            4 as size_t,
        );
        memset(
            dst.offset(stride * 3 as ptrdiff_t) as *mut ::core::ffi::c_void,
            (K + 2 as ::core::ffi::c_int * L + L + 2 as ::core::ffi::c_int)
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
    _left: *const uint8_t,
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
            ((H + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) =
            ((I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) =
            ((J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) =
            ((K + 2 as ::core::ffi::c_int * L + M + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        memcpy(
            dst.offset(stride * 1 as ptrdiff_t) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset(stride * 2 as ptrdiff_t) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            dst.offset(stride * 3 as ptrdiff_t) as *mut ::core::ffi::c_void,
            dst as *const ::core::ffi::c_void,
            4 as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d207_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    _above: *const uint8_t,
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
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((I + J + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        let fresh21 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh21 = ((J + K + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh21;
        let fresh22 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh22 = ((K + L + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh22;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh23 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh23 = ((J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh23;
        let fresh24 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh24 = ((K + 2 as ::core::ffi::c_int * L + L + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
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
pub unsafe extern "C" fn vpx_d63_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
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
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + B + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        let fresh48 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh48 = ((B + C + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh48;
        let fresh49 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh49 = ((C + D + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh49;
        let fresh50 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh50 = ((D + E + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh50;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((E + F + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh51 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh51 = ((B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh51;
        let fresh52 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh52 = ((C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh52;
        let fresh53 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh53 = ((D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh53;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d63e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
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
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + B + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        let fresh54 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh54 = ((B + C + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh54;
        let fresh55 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh55 = ((C + D + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh55;
        let fresh56 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh56 = ((D + E + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh56;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh57 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh57 = ((B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh57;
        let fresh58 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh58 = ((C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh58;
        let fresh59 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh59 = ((D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh59;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
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
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh30 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh30 = ((B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh30;
        let fresh31 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh31 = ((C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh32 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh32 = *fresh31;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh32;
        let fresh33 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh33 = ((D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh34 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh34 = *fresh33;
        let fresh35 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh35 = *fresh34;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh35;
        let fresh36 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh36 = ((E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh37 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh37 = *fresh36;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh37;
        let fresh38 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh38 = ((F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh38;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) = H as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45e_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    _left: *const uint8_t,
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
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh39 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh39 = ((B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh39;
        let fresh40 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh40 = ((C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh41 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh41 = *fresh40;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh41;
        let fresh42 = &mut *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh42 = ((D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh43 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh43 = *fresh42;
        let fresh44 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh44 = *fresh43;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh44;
        let fresh45 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh45 = ((E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh46 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh46 = *fresh45;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh46;
        let fresh47 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh47 = ((F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh47;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((G + 2 as ::core::ffi::c_int * H + H + 2 as ::core::ffi::c_int)
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
        let fresh0 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh0 = ((X + A + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh0;
        let fresh1 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh1 = ((A + B + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh1;
        let fresh2 = &mut *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh2 = ((B + C + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh2;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((C + D + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride) =
            ((J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh3 = &mut *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh3 = ((I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh3;
        let fresh4 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh4 = ((X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh4;
        let fresh5 = &mut *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh5 = ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh5;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) =
            ((B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int)
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
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh6 = &mut *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh6 = ((I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh6;
        let fresh7 = &mut *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh7 = ((X + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh8 = &mut *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh8 = *fresh7;
        *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh8;
        let fresh9 = &mut *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh9 = ((A + 2 as ::core::ffi::c_int * X + I + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh10 = &mut *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh10 = *fresh9;
        let fresh11 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh11 = *fresh10;
        *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride) = *fresh11;
        let fresh12 = &mut *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh12 = ((B + 2 as ::core::ffi::c_int * A + X + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh13 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh13 = *fresh12;
        *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh13;
        let fresh14 = &mut *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride);
        *fresh14 = ((C + 2 as ::core::ffi::c_int * B + A + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh14;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((D + 2 as ::core::ffi::c_int * C + B + 2 as ::core::ffi::c_int)
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
        let fresh15 = &mut *dst.offset(2 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh15 = ((I + X + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh15;
        let fresh16 = &mut *dst.offset(2 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh16 = ((J + I + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh16;
        let fresh17 = &mut *dst.offset(2 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh17 = ((K + J + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(0 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh17;
        *dst.offset(0 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((L + K + 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(3 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(2 as ptrdiff_t + 0 as ptrdiff_t * stride) =
            ((X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as uint8_t;
        let fresh18 = &mut *dst.offset(3 as ptrdiff_t + 1 as ptrdiff_t * stride);
        *fresh18 = ((I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 0 as ptrdiff_t * stride) = *fresh18;
        let fresh19 = &mut *dst.offset(3 as ptrdiff_t + 2 as ptrdiff_t * stride);
        *fresh19 = ((J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 1 as ptrdiff_t * stride) = *fresh19;
        let fresh20 = &mut *dst.offset(3 as ptrdiff_t + 3 as ptrdiff_t * stride);
        *fresh20 = ((K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as uint8_t;
        *dst.offset(1 as ptrdiff_t + 2 as ptrdiff_t * stride) = *fresh20;
        *dst.offset(1 as ptrdiff_t + 3 as ptrdiff_t * stride) =
            ((L + 2 as ::core::ffi::c_int * K + J + 2 as ::core::ffi::c_int)
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
