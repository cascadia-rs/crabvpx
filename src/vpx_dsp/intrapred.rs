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
pub fn vpx_d207_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    left: &[u8],
) {
    for r in 0..bs - 1 {
        dst[r * stride] = ((left[r] as i32 + left[r + 1] as i32 + 1) >> 1) as u8;
    }
    dst[(bs - 1) * stride] = left[bs - 1];

    for r in 0..bs - 2 {
        dst[r * stride + 1] =
            ((left[r] as i32 + 2 * left[r + 1] as i32 + left[r + 2] as i32 + 2) >> 2) as u8;
    }
    dst[(bs - 2) * stride + 1] =
        ((left[bs - 2] as i32 + 3 * left[bs - 1] as i32 + 2) >> 2) as u8;
    dst[(bs - 1) * stride + 1] = left[bs - 1];

    for c in 0..bs - 2 {
        dst[(bs - 1) * stride + 2 + c] = left[bs - 1];
    }

    for r in (0..=bs - 2).rev() {
        for c in 0..bs - 2 {
            dst[r * stride + 2 + c] = dst[(r + 1) * stride + c];
        }
    }
}
pub fn vpx_d63_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    above: &[u8],
) {
    for c in 0..bs {
        dst[c] = ((above[c] as i32 + above[c + 1] as i32 + 1) >> 1) as u8;
        dst[stride + c] =
            ((above[c] as i32 + 2 * above[c + 1] as i32 + above[c + 2] as i32 + 2) >> 2) as u8;
    }

    let mut size = bs - 2;
    let val = above[bs - 1];
    for r in (2..bs).step_by(2) {
        let r_half = r >> 1;
        for i in 0..size {
            dst[r * stride + i] = dst[r_half + i];
        }
        dst[r * stride + size..r * stride + bs].fill(val);

        for i in 0..size {
            dst[(r + 1) * stride + i] = dst[stride + r_half + i];
        }
        dst[(r + 1) * stride + size..(r + 1) * stride + bs].fill(val);

        size -= 1;
    }
}
#[inline]
unsafe extern "C" fn d45_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn d117_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
    *dst.offset(0 as ::core::ffi::c_int as isize) = (*left.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
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
                    * *left.offset((r - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
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
                (-(2 as ::core::ffi::c_int) as ptrdiff_t * stride + c as ptrdiff_t - 1 as ptrdiff_t)
                    as isize,
            );
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}}
#[inline]
unsafe extern "C" fn d135_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut border: [uint8_t; 63] = [0; 63];
    i = 0 as ::core::ffi::c_int;
    while i < bs - 2 as ::core::ffi::c_int {
        border[i as usize] = (*left.offset((bs - 3 as ::core::ffi::c_int - i) as isize)
            as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *left.offset((bs - 2 as ::core::ffi::c_int - i) as isize) as ::core::ffi::c_int
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
        border[(bs + 1 as ::core::ffi::c_int + i) as usize] =
            (*above.offset(i as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *above.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *above.offset((i + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t;
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
}}
#[inline]
unsafe extern "C" fn d153_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
    *dst.offset(0 as ::core::ffi::c_int as isize) = (*left.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
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
                    * *left.offset((r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
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
}}
#[inline]
unsafe extern "C" fn v_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn h_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn tm_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn dc_128_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn dc_left_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn dc_top_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
#[inline]
unsafe extern "C" fn dc_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bs: ::core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
}}
pub fn vpx_he_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let h = above[0] as i32;
    let i = left[0] as i32;
    let j = left[1] as i32;
    let k = left[2] as i32;
    let l = left[3] as i32;

    let val0 = ((h + 2 * i + j + 2) >> 2) as u8;
    let val1 = ((i + 2 * j + k + 2) >> 2) as u8;
    let val2 = ((j + 2 * k + l + 2) >> 2) as u8;
    let val3 = ((k + 2 * l + l + 2) >> 2) as u8;

    dst[0..4].fill(val0);
    dst[stride..stride + 4].fill(val1);
    dst[2 * stride..2 * stride + 4].fill(val2);
    dst[3 * stride..3 * stride + 4].fill(val3);
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_he_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 1);
        let left_slice = core::slice::from_raw_parts(left, 4);
        vpx_he_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}

pub fn vpx_ve_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    let h = above[0] as i32;
    let i = above[1] as i32;
    let j = above[2] as i32;
    let k = above[3] as i32;
    let l = above[4] as i32;
    let m = above[5] as i32;

    dst[0] = ((h + 2 * i + j + 2) >> 2) as u8;
    dst[1] = ((i + 2 * j + k + 2) >> 2) as u8;
    dst[2] = ((j + 2 * k + l + 2) >> 2) as u8;
    dst[3] = ((k + 2 * l + m + 2) >> 2) as u8;

    let r0 = dst[0];
    let r1 = dst[1];
    let r2 = dst[2];
    let r3 = dst[3];

    for row in 1..4 {
        let offset = row * stride;
        dst[offset] = r0;
        dst[offset + 1] = r1;
        dst[offset + 2] = r2;
        dst[offset + 3] = r3;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_ve_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 6);
        vpx_ve_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
pub fn vpx_d207_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    left: &[u8],
) {
    let i_val = left[0] as i32;
    let j_val = left[1] as i32;
    let k_val = left[2] as i32;
    let l_val = left[3] as i32;

    let val_0_0 = ((i_val + j_val + 1) >> 1) as u8;
    let val_0_1 = ((i_val + 2 * j_val + k_val + 2) >> 2) as u8;
    let val_0_2 = ((j_val + k_val + 1) >> 1) as u8;
    let val_0_3 = ((j_val + 2 * k_val + l_val + 2) >> 2) as u8;

    let val_1_0 = val_0_2;
    let val_1_1 = val_0_3;
    let val_1_2 = ((k_val + l_val + 1) >> 1) as u8;
    let val_1_3 = ((k_val + 3 * l_val + 2) >> 2) as u8;

    let val_2_0 = val_1_2;
    let val_2_1 = val_1_3;
    let val_2_2 = l_val as u8;
    let val_2_3 = l_val as u8;

    let val_3_0 = l_val as u8;
    let val_3_1 = l_val as u8;
    let val_3_2 = l_val as u8;
    let val_3_3 = l_val as u8;

    dst[0] = val_0_0;
    dst[1] = val_0_1;
    dst[2] = val_0_2;
    dst[3] = val_0_3;

    dst[stride] = val_1_0;
    dst[stride + 1] = val_1_1;
    dst[stride + 2] = val_1_2;
    dst[stride + 3] = val_1_3;

    dst[2 * stride] = val_2_0;
    dst[2 * stride + 1] = val_2_1;
    dst[2 * stride + 2] = val_2_2;
    dst[2 * stride + 3] = val_2_3;

    dst[3 * stride] = val_3_0;
    dst[3 * stride + 1] = val_3_1;
    dst[3 * stride + 2] = val_3_2;
    dst[3 * stride + 3] = val_3_3;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d207_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 4);
        vpx_d207_predictor_4x4_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d63_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        vpx_d63e_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
pub fn vpx_d63e_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    let A = above[0] as i32;
    let B = above[1] as i32;
    let C = above[2] as i32;
    let D = above[3] as i32;
    let E = above[4] as i32;
    let F = above[5] as i32;
    let G = above[6] as i32;
    let H = above[7] as i32;

    dst[0] = ((A + B + 1) >> 1) as u8;
    let val_2_0 = ((B + C + 1) >> 1) as u8;
    dst[2 * stride] = val_2_0;
    dst[1] = val_2_0;

    let val_2_1 = ((C + D + 1) >> 1) as u8;
    dst[2 * stride + 1] = val_2_1;
    dst[2] = val_2_1;

    let val_2_2 = ((D + E + 1) >> 1) as u8;
    dst[2 * stride + 2] = val_2_2;
    dst[3] = val_2_2;

    dst[2 * stride + 3] = ((E + 2 * F + G + 2) >> 2) as u8;

    dst[stride] = ((A + 2 * B + C + 2) >> 2) as u8;
    let val_3_0 = ((B + 2 * C + D + 2) >> 2) as u8;
    dst[3 * stride] = val_3_0;
    dst[stride + 1] = val_3_0;

    let val_3_1 = ((C + 2 * D + E + 2) >> 2) as u8;
    dst[3 * stride + 1] = val_3_1;
    dst[stride + 2] = val_3_1;

    let val_3_2 = ((D + 2 * E + F + 2) >> 2) as u8;
    dst[3 * stride + 2] = val_3_2;
    dst[stride + 3] = val_3_2;

    dst[3 * stride + 3] = ((F + 2 * G + H + 2) >> 2) as u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d63e_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        vpx_d63e_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
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
        (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
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
}}
pub fn vpx_d45e_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    let A = above[0] as i32;
    let B = above[1] as i32;
    let C = above[2] as i32;
    let D = above[3] as i32;
    let E = above[4] as i32;
    let F = above[5] as i32;
    let G = above[6] as i32;
    let H = above[7] as i32;

    dst[0] = ((A + 2 * B + C + 2) >> 2) as u8;
    let val_0_1 = ((B + 2 * C + D + 2) >> 2) as u8;
    dst[stride] = val_0_1;
    dst[1] = val_0_1;

    let val_0_2 = ((C + 2 * D + E + 2) >> 2) as u8;
    dst[2 * stride] = val_0_2;
    dst[stride + 1] = val_0_2;
    dst[2] = val_0_2;

    let val_0_3 = ((D + 2 * E + F + 2) >> 2) as u8;
    dst[3 * stride] = val_0_3;
    dst[2 * stride + 1] = val_0_3;
    dst[stride + 2] = val_0_3;
    dst[3] = val_0_3;

    let val_1_3 = ((E + 2 * F + G + 2) >> 2) as u8;
    dst[3 * stride + 1] = val_1_3;
    dst[2 * stride + 2] = val_1_3;
    dst[stride + 3] = val_1_3;

    let val_2_3 = ((F + 2 * G + H + 2) >> 2) as u8;
    dst[3 * stride + 2] = val_2_3;
    dst[2 * stride + 3] = val_2_3;

    dst[3 * stride + 3] = ((G + 2 * H + H + 2) >> 2) as u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d45e_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        vpx_d45e_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
pub fn vpx_d117_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let X = above[0] as i32;
    let A = above[1] as i32;
    let B = above[2] as i32;
    let C = above[3] as i32;
    let D = above[4] as i32;

    let I = left[0] as i32;
    let J = left[1] as i32;
    let K = left[2] as i32;

    let val_0 = ((X + A + 1) >> 1) as u8;
    dst[2 * stride + 1] = val_0;
    dst[0] = val_0;

    let val_1 = ((A + B + 1) >> 1) as u8;
    dst[2 * stride + 2] = val_1;
    dst[1] = val_1;

    let val_2 = ((B + C + 1) >> 1) as u8;
    dst[2 * stride + 3] = val_2;
    dst[2] = val_2;

    dst[3] = ((C + D + 1) >> 1) as u8;
    dst[3 * stride] = ((K + 2 * J + I + 2) >> 2) as u8;
    dst[2 * stride] = ((J + 2 * I + X + 2) >> 2) as u8;

    let val_3 = ((I + 2 * X + A + 2) >> 2) as u8;
    dst[3 * stride + 1] = val_3;
    dst[stride] = val_3;

    let val_4 = ((X + 2 * A + B + 2) >> 2) as u8;
    dst[3 * stride + 2] = val_4;
    dst[stride + 1] = val_4;

    let val_5 = ((A + 2 * B + C + 2) >> 2) as u8;
    dst[3 * stride + 3] = val_5;
    dst[stride + 2] = val_5;

    dst[stride + 3] = ((B + 2 * C + D + 2) >> 2) as u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d117_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 5);
        let left_slice = core::slice::from_raw_parts(left, 3);
        vpx_d117_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
pub fn vpx_d135_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let X = above[0] as i32;
    let A = above[1] as i32;
    let B = above[2] as i32;
    let C = above[3] as i32;
    let D = above[4] as i32;

    let I = left[0] as i32;
    let J = left[1] as i32;
    let K = left[2] as i32;
    let L = left[3] as i32;

    dst[3 * stride] = ((J + 2 * K + L + 2) >> 2) as u8;

    let val_1 = ((I + 2 * J + K + 2) >> 2) as u8;
    dst[2 * stride] = val_1;
    dst[3 * stride + 1] = val_1;

    let val_2 = ((X + 2 * I + J + 2) >> 2) as u8;
    dst[stride] = val_2;
    dst[2 * stride + 1] = val_2;
    dst[3 * stride + 2] = val_2;

    let val_3 = ((A + 2 * X + I + 2) >> 2) as u8;
    dst[0] = val_3;
    dst[stride + 1] = val_3;
    dst[2 * stride + 2] = val_3;
    dst[3 * stride + 3] = val_3;

    let val_4 = ((B + 2 * A + X + 2) >> 2) as u8;
    dst[1] = val_4;
    dst[stride + 2] = val_4;
    dst[2 * stride + 3] = val_4;

    let val_5 = ((C + 2 * B + A + 2) >> 2) as u8;
    dst[2] = val_5;
    dst[stride + 3] = val_5;

    dst[3] = ((D + 2 * C + B + 2) >> 2) as u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d135_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 5);
        let left_slice = core::slice::from_raw_parts(left, 4);
        vpx_d135_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
pub fn vpx_d153_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let I = left[0] as i32;
    let J = left[1] as i32;
    let K = left[2] as i32;
    let L = left[3] as i32;
    let X = above[0] as i32;
    let A = above[1] as i32;
    let B = above[2] as i32;
    let C = above[3] as i32;

    let val_0_0 = ((I + X + 1) >> 1) as u8;
    let val_0_1 = ((I + 2 * X + A + 2) >> 2) as u8;
    let val_0_2 = ((X + 2 * A + B + 2) >> 2) as u8;
    let val_0_3 = ((A + 2 * B + C + 2) >> 2) as u8;

    let val_1_0 = ((J + I + 1) >> 1) as u8;
    let val_1_1 = ((J + 2 * I + X + 2) >> 2) as u8;

    let val_2_0 = ((K + J + 1) >> 1) as u8;
    let val_2_1 = ((K + 2 * J + I + 2) >> 2) as u8;

    let val_3_0 = ((L + K + 1) >> 1) as u8;
    let val_3_1 = ((L + 2 * K + J + 2) >> 2) as u8;

    dst[0] = val_0_0;
    dst[1] = val_0_1;
    dst[2] = val_0_2;
    dst[3] = val_0_3;

    dst[stride] = val_1_0;
    dst[stride + 1] = val_1_1;
    dst[stride + 2] = val_0_0;
    dst[stride + 3] = val_0_1;

    dst[2 * stride] = val_2_0;
    dst[2 * stride + 1] = val_2_1;
    dst[2 * stride + 2] = val_1_0;
    dst[2 * stride + 3] = val_1_1;

    dst[3 * stride] = val_3_0;
    dst[3 * stride + 1] = val_3_1;
    dst[3 * stride + 2] = val_2_0;
    dst[3 * stride + 3] = val_2_1;
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_d153_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 4);
        let left_slice = core::slice::from_raw_parts(left, 4);
        vpx_d153_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d207_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_d207_predictor_safe(dst_slice, stride as usize, 8, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d207_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_d207_predictor_safe(dst_slice, stride as usize, 16, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d207_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 32);
        vpx_d207_predictor_safe(dst_slice, stride as usize, 32, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d63_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 32);
        vpx_d63_predictor_safe(dst_slice, stride as usize, 16, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d63_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 64);
        vpx_d63_predictor_safe(dst_slice, stride as usize, 32, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d63_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 16);
        vpx_d63_predictor_safe(dst_slice, stride as usize, 8, above_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d45_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d45_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d45_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d45_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d117_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d117_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d117_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d117_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d135_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d135_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d135_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d135_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d153_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_8x8_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d153_predictor(dst, stride, 8 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_d153_predictor_16x16_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    d153_predictor(dst, stride, 16 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_v_predictor_8x8_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    for r in 0..8 {
        let start = r * stride;
        dst[start..start + 8].copy_from_slice(&above[..8]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_v_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        vpx_v_predictor_8x8_safe(dst_slice, stride as usize, above_slice);
    }
}

pub fn vpx_v_predictor_16x16_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    for r in 0..16 {
        let start = r * stride;
        dst[start..start + 16].copy_from_slice(&above[..16]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_v_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 16);
        vpx_v_predictor_16x16_safe(dst_slice, stride as usize, above_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    v_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_v_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    v_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_h_predictor_16x16_safe(
    dst: &mut [u8],
    stride: usize,
    left: &[u8],
) {
    for r in 0..16 {
        let start = r * stride;
        let val = left[r];
        dst[start..start + 16].fill(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_h_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_h_predictor_16x16_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    h_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_h_predictor_8x8_safe(
    dst: &mut [u8],
    stride: usize,
    left: &[u8],
) {
    for r in 0..8 {
        let start = r * stride;
        let val = left[r];
        dst[start..start + 8].fill(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_h_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_h_predictor_8x8_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_h_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    h_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_tm_predictor_16x16_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
    top_left: u8,
) {
    let ytop_left = top_left as i32;
    for r in 0..16 {
        let dst_idx = r * stride;
        let left_val = left[r] as i32;
        for c in 0..16 {
            let val = left_val + above[c] as i32 - ytop_left;
            dst[dst_idx + c] = val.clamp(0, 255) as u8;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_tm_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 16);
        let left_slice = core::slice::from_raw_parts(left, 16);
        let top_left = *above.offset(-1);
        vpx_tm_predictor_16x16_safe(dst_slice, stride as usize, above_slice, left_slice, top_left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_tm_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    tm_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_tm_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
    top_left: u8,
) {
    let ytop_left = top_left as i32;
    for r in 0..4 {
        let dst_idx = r * stride;
        let left_val = left[r] as i32;
        for c in 0..4 {
            let val = left_val + above[c] as i32 - ytop_left;
            dst[dst_idx + c] = val.clamp(0, 255) as u8;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_tm_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 4);
        let left_slice = core::slice::from_raw_parts(left, 4);
        let top_left = *above.offset(-1);
        vpx_tm_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice, top_left);
    }
}
pub fn vpx_tm_predictor_8x8_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
    top_left: u8,
) {
    let ytop_left = top_left as i32;
    for r in 0..8 {
        let dst_idx = r * stride;
        let left_val = left[r] as i32;
        for c in 0..8 {
            let val = left_val + above[c] as i32 - ytop_left;
            dst[dst_idx + c] = val.clamp(0, 255) as u8;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_tm_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        let left_slice = core::slice::from_raw_parts(left, 8);
        let top_left = *above.offset(-1);
        vpx_tm_predictor_8x8_safe(dst_slice, stride as usize, above_slice, left_slice, top_left);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_128_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_128_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vpx_dc_128_predictor_16x16_safe(dst_slice, stride as usize);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_128_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vpx_dc_128_predictor_8x8_safe(dst_slice, stride as usize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_128_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_128_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_left_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_dc_left_predictor_8x8_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_left_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_left_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_left_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_left_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_dc_left_predictor_16x16_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_top_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_top_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        vpx_dc_top_predictor_8x8_safe(dst_slice, stride as usize, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_top_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 16);
        vpx_dc_top_predictor_16x16_safe(dst_slice, stride as usize, above_slice);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_top_predictor_4x4_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_top_predictor(dst, stride, 4 as ::core::ffi::c_int, above, left);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_dc_predictor_32x32_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
) { unsafe {
    dc_predictor(dst, stride, 32 as ::core::ffi::c_int, above, left);
}}
pub fn vpx_dc_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let mut sum = 0i32;
    for i in 0..4 {
        sum += above[i] as i32;
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 4) / 8) as u8;
    for r in 0..4 {
        let dst_idx = r * stride;
        dst[dst_idx..dst_idx + 4].fill(expected_dc);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 4);
        let left_slice = core::slice::from_raw_parts(left, 4);
        vpx_dc_predictor_4x4_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_predictor_8x8_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 7 * stride as usize + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 8);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_dc_predictor_8x8_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_predictor_16x16_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 15 * stride as usize + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 16);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_dc_predictor_16x16_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}

pub fn vpx_dc_128_predictor_16x16_safe(dst: &mut [u8], stride: usize) {
    for r in 0..16 {
        let start = r * stride;
        dst[start..start + 16].fill(128);
    }
}

pub fn vpx_dc_128_predictor_8x8_safe(dst: &mut [u8], stride: usize) {
    for r in 0..8 {
        let start = r * stride;
        dst[start..start + 8].fill(128);
    }
}

pub fn vpx_dc_top_predictor_16x16_safe(dst: &mut [u8], stride: usize, above: &[u8]) {
    let mut sum = 0i32;
    for i in 0..16 {
        sum += above[i] as i32;
    }
    let expected_dc = ((sum + 8) / 16) as u8;
    for r in 0..16 {
        let start = r * stride;
        dst[start..start + 16].fill(expected_dc);
    }
}

pub fn vpx_dc_top_predictor_8x8_safe(dst: &mut [u8], stride: usize, above: &[u8]) {
    let mut sum = 0i32;
    for i in 0..8 {
        sum += above[i] as i32;
    }
    let expected_dc = ((sum + 4) / 8) as u8;
    for r in 0..8 {
        let start = r * stride;
        dst[start..start + 8].fill(expected_dc);
    }
}

pub fn vpx_dc_left_predictor_16x16_safe(dst: &mut [u8], stride: usize, left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..16 {
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 8) / 16) as u8;
    for r in 0..16 {
        let start = r * stride;
        dst[start..start + 16].fill(expected_dc);
    }
}

pub fn vpx_dc_left_predictor_8x8_safe(dst: &mut [u8], stride: usize, left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..8 {
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 4) / 8) as u8;
    for r in 0..8 {
        let start = r * stride;
        dst[start..start + 8].fill(expected_dc);
    }
}

pub fn vpx_dc_predictor_16x16_safe(dst: &mut [u8], stride: usize, above: &[u8], left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..16 {
        sum += above[i] as i32;
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 16) / 32) as u8;
    for r in 0..16 {
        let start = r * stride;
        dst[start..start + 16].fill(expected_dc);
    }
}

pub fn vpx_dc_predictor_8x8_safe(dst: &mut [u8], stride: usize, above: &[u8], left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..8 {
        sum += above[i] as i32;
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 8) / 16) as u8;
    for r in 0..8 {
        let start = r * stride;
        dst[start..start + 8].fill(expected_dc);
    }
}
