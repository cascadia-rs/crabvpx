use std::ffi::c_void;
#[inline]
fn clip_pixel(mut val: i32) -> u8 {
    (if val > 255 as i32 {
        255 as i32
    } else if val < 0 as i32 {
        0 as i32
    } else {
        val
    }) as u8
}
#[inline]
unsafe fn d207_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    _above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        r = 0 as i32;
        while r < bs - 1 as i32 {
            *dst.offset((r as isize * stride) as isize) = ((*left.offset(r as isize) as i32
                + *left.offset((r + 1 as i32) as isize) as i32
                + 1 as i32)
                >> 1 as i32) as u8;
            r += 1;
        }
        *dst.offset((bs - 1 as i32) as isize * stride) = *left.offset((bs - 1 as i32) as isize);
        dst = dst.offset(1);
        r = 0 as i32;
        while r < bs - 2 as i32 {
            *dst.offset((r as isize * stride) as isize) = ((*left.offset(r as isize) as i32
                + 2 as i32 * *left.offset((r + 1 as i32) as isize) as i32
                + *left.offset((r + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            r += 1;
        }
        *dst.offset((bs - 2 as i32) as isize * stride) = ((*left.offset((bs - 2 as i32) as isize)
            as i32
            + 2 as i32 * *left.offset((bs - 1 as i32) as isize) as i32
            + *left.offset((bs - 1 as i32) as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        *dst.offset((bs - 1 as i32) as isize * stride) = *left.offset((bs - 1 as i32) as isize);
        dst = dst.offset(1);
        c = 0 as i32;
        while c < bs - 2 as i32 {
            *dst.offset(((bs - 1 as i32) as isize * stride + c as isize) as isize) =
                *left.offset((bs - 1 as i32) as isize);
            c += 1;
        }
        r = bs - 2 as i32;
        while r >= 0 as i32 {
            c = 0 as i32;
            while c < bs - 2 as i32 {
                *dst.offset((r as isize * stride + c as isize) as isize) = *dst
                    .offset(((r + 1 as i32) as isize * stride + c as isize - 2 as isize) as isize);
                c += 1;
            }
            r -= 1;
        }
    }
}
#[inline]
unsafe fn d63_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    _left: *const u8,
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
                >> 1 as i32) as u8;
            *dst.offset((stride + c as isize) as isize) = ((*above.offset(c as isize) as i32
                + 2 as i32 * *above.offset((c + 1 as i32) as isize) as i32
                + *above.offset((c + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            c += 1;
        }
        r = 2 as i32;
        size = bs - 2 as i32;
        while r < bs {
            core::ptr::copy_nonoverlapping(
                (dst.offset((r >> 1 as i32) as isize) as *const c_void) as *const u8,
                (dst.offset(((r + 0 as i32) as isize * stride) as isize) as *mut c_void) as *mut u8,
                (size as usize) as usize,
            );
            core::ptr::write_bytes(
                dst.offset(((r + 0 as i32) as isize * stride) as isize)
                    .offset(size as isize) as *mut c_void as *mut u8,
                *above.offset((bs - 1 as i32) as isize) as u8,
                (bs - size) as usize,
            );
            core::ptr::copy_nonoverlapping(
                (dst.offset(stride).offset((r >> 1 as i32) as isize) as *const c_void) as *const u8,
                (dst.offset(((r + 1 as i32) as isize * stride) as isize) as *mut c_void) as *mut u8,
                (size as usize) as usize,
            );
            core::ptr::write_bytes(
                dst.offset(((r + 1 as i32) as isize * stride) as isize)
                    .offset(size as isize) as *mut c_void as *mut u8,
                *above.offset((bs - 1 as i32) as isize) as u8,
                (bs - size) as usize,
            );
            r += 2 as i32;
            size -= 1;
        }
    }
}
#[inline]
unsafe fn d45_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    _left: *const u8,
) {
    unsafe {
        let above_right: u8 = *above.offset((bs - 1 as i32) as isize);
        let dst_row0: *const u8 = dst;
        let mut x: i32 = 0;
        let mut size: i32 = 0;
        x = 0 as i32;
        while x < bs - 1 as i32 {
            *dst.offset(x as isize) = ((*above.offset(x as isize) as i32
                + 2 as i32 * *above.offset((x + 1 as i32) as isize) as i32
                + *above.offset((x + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            x += 1;
        }
        *dst.offset((bs - 1 as i32) as isize) = above_right;
        dst = dst.offset(stride);
        x = 1 as i32;
        size = bs - 2 as i32;
        while x < bs {
            core::ptr::copy_nonoverlapping(
                (dst_row0.offset(x as isize) as *const c_void) as *const u8,
                (dst as *mut c_void) as *mut u8,
                (size as usize) as usize,
            );
            core::ptr::write_bytes(
                dst.offset(size as isize) as *mut c_void as *mut u8,
                above_right as u8,
                (x + 1 as i32) as usize,
            );
            dst = dst.offset(stride);
            x += 1;
            size -= 1;
        }
    }
}
#[inline]
unsafe fn d117_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        c = 0 as i32;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset((c - 1 as i32) as isize) as i32
                + *above.offset(c as isize) as i32
                + 1 as i32)
                >> 1 as i32) as u8;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as isize) = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        c = 1 as i32;
        while c < bs {
            *dst.offset(c as isize) = ((*above.offset((c - 2 as i32) as isize) as i32
                + 2 as i32 * *above.offset((c - 1 as i32) as isize) as i32
                + *above.offset(c as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            c += 1;
        }
        dst = dst.offset(stride);
        *dst.offset(0 as isize) = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        r = 3 as i32;
        while r < bs {
            *dst.offset(((r - 2 as i32) as isize * stride) as isize) =
                ((*left.offset((r - 3 as i32) as isize) as i32
                    + 2 as i32 * *left.offset((r - 2 as i32) as isize) as i32
                    + *left.offset((r - 1 as i32) as isize) as i32
                    + 2 as i32)
                    >> 2 as i32) as u8;
            r += 1;
        }
        r = 2 as i32;
        while r < bs {
            c = 1 as i32;
            while c < bs {
                *dst.offset(c as isize) =
                    *dst.offset((-(2 as i32) as isize * stride + c as isize - 1 as isize) as isize);
                c += 1;
            }
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn d135_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut border: [u8; 63] = [0; 63];
        i = 0 as i32;
        while i < bs - 2 as i32 {
            border[i as usize] = ((*left.offset((bs - 3 as i32 - i) as isize) as i32
                + 2 as i32 * *left.offset((bs - 2 as i32 - i) as isize) as i32
                + *left.offset((bs - 1 as i32 - i) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            i += 1;
        }
        border[(bs - 2 as i32) as usize] = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        border[(bs - 1 as i32) as usize] = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        border[(bs - 0 as i32) as usize] = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *above.offset(0 as isize) as i32
            + *above.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        i = 0 as i32;
        while i < bs - 2 as i32 {
            border[(bs + 1 as i32 + i) as usize] = ((*above.offset(i as isize) as i32
                + 2 as i32 * *above.offset((i + 1 as i32) as isize) as i32
                + *above.offset((i + 2 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            i += 1;
        }
        i = 0 as i32;
        while i < bs {
            core::ptr::copy_nonoverlapping(
                ((&raw mut border as *mut u8)
                    .offset(bs as isize)
                    .offset(-(1 as isize))
                    .offset(-(i as isize)) as *const c_void) as *const u8,
                (dst.offset((i as isize * stride) as isize) as *mut c_void) as *mut u8,
                (bs as usize) as usize,
            );
            i += 1;
        }
    }
}
#[inline]
unsafe fn d153_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        *dst.offset(0 as isize) = ((*above.offset(-(1 as i32) as isize) as i32
            + *left.offset(0 as isize) as i32
            + 1 as i32)
            >> 1 as i32) as u8;
        r = 1 as i32;
        while r < bs {
            *dst.offset((r as isize * stride) as isize) = ((*left.offset((r - 1 as i32) as isize)
                as i32
                + *left.offset(r as isize) as i32
                + 1 as i32)
                >> 1 as i32) as u8;
            r += 1;
        }
        dst = dst.offset(1);
        *dst.offset(0 as isize) = ((*left.offset(0 as isize) as i32
            + 2 as i32 * *above.offset(-(1 as i32) as isize) as i32
            + *above.offset(0 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        *dst.offset(stride) = ((*above.offset(-(1 as i32) as isize) as i32
            + 2 as i32 * *left.offset(0 as isize) as i32
            + *left.offset(1 as isize) as i32
            + 2 as i32)
            >> 2 as i32) as u8;
        r = 2 as i32;
        while r < bs {
            *dst.offset((r as isize * stride) as isize) = ((*left.offset((r - 2 as i32) as isize)
                as i32
                + 2 as i32 * *left.offset((r - 1 as i32) as isize) as i32
                + *left.offset(r as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            r += 1;
        }
        dst = dst.offset(1);
        c = 0 as i32;
        while c < bs - 2 as i32 {
            *dst.offset(c as isize) = ((*above.offset((c - 1 as i32) as isize) as i32
                + 2 as i32 * *above.offset(c as isize) as i32
                + *above.offset((c + 1 as i32) as isize) as i32
                + 2 as i32)
                >> 2 as i32) as u8;
            c += 1;
        }
        dst = dst.offset(stride);
        r = 1 as i32;
        while r < bs {
            c = 0 as i32;
            while c < bs - 2 as i32 {
                *dst.offset(c as isize) = *dst.offset((-stride + c as isize - 2 as isize) as isize);
                c += 1;
            }
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn v_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    _left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            core::ptr::copy_nonoverlapping(
                above as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                bs as usize,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn h_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    _above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            core::ptr::write_bytes(
                dst as *mut c_void as *mut u8,
                *left.offset(r as isize) as u8,
                bs as usize,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn tm_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    mut left: *const u8,
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
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    _above: *const u8,
    _left: *const u8,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < bs {
            core::ptr::write_bytes(dst as *mut c_void as *mut u8, 128 as u8, bs as usize);
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_left_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    _above: *const u8,
    mut left: *const u8,
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
            core::ptr::write_bytes(
                dst as *mut c_void as *mut u8,
                expected_dc as u8,
                bs as usize,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_top_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    _left: *const u8,
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
            core::ptr::write_bytes(
                dst as *mut c_void as *mut u8,
                expected_dc as u8,
                bs as usize,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[inline]
unsafe fn dc_predictor(
    mut dst: *mut u8,
    mut stride: isize,
    mut bs: i32,
    mut above: *const u8,
    mut left: *const u8,
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
            core::ptr::write_bytes(
                dst as *mut c_void as *mut u8,
                expected_dc as u8,
                bs as usize,
            );
            dst = dst.offset(stride);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_he_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let H: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        core::ptr::write_bytes(
            (dst.offset(stride * 0 as isize) as *mut c_void) as *mut u8,
            ((H + 2 as i32 * I + J + 2 as i32) >> 2 as i32) as u8,
            (4 as usize) as usize,
        );
        core::ptr::write_bytes(
            (dst.offset(stride * 1 as isize) as *mut c_void) as *mut u8,
            ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as u8,
            (4 as usize) as usize,
        );
        core::ptr::write_bytes(
            (dst.offset(stride * 2 as isize) as *mut c_void) as *mut u8,
            ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as u8,
            (4 as usize) as usize,
        );
        core::ptr::write_bytes(
            (dst.offset(stride * 3 as isize) as *mut c_void) as *mut u8,
            ((K + 2 as i32 * L + L + 2 as i32) >> 2 as i32) as u8,
            (4 as usize) as usize,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_ve_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    _left: *const u8,
) {
    unsafe {
        let H: i32 = *above.offset(-(1 as i32) as isize) as i32;
        let I: i32 = *above.offset(0 as isize) as i32;
        let J: i32 = *above.offset(1 as isize) as i32;
        let K: i32 = *above.offset(2 as isize) as i32;
        let L: i32 = *above.offset(3 as isize) as i32;
        let M: i32 = *above.offset(4 as isize) as i32;
        *dst.offset(0 as isize) = ((H + 2 as i32 * I + J + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize) = ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(2 as isize) = ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize) = ((K + 2 as i32 * L + M + 2 as i32) >> 2 as i32) as u8;
        core::ptr::copy_nonoverlapping(
            (dst as *const c_void) as *const u8,
            (dst.offset(stride * 1 as isize) as *mut c_void) as *mut u8,
            (4 as usize) as usize,
        );
        core::ptr::copy_nonoverlapping(
            (dst as *const c_void) as *const u8,
            (dst.offset(stride * 2 as isize) as *mut c_void) as *mut u8,
            (4 as usize) as usize,
        );
        core::ptr::copy_nonoverlapping(
            (dst as *const c_void) as *const u8,
            (dst.offset(stride * 3 as isize) as *mut c_void) as *mut u8,
            (4 as usize) as usize,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    _above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        let I: i32 = *left.offset(0 as isize) as i32;
        let J: i32 = *left.offset(1 as isize) as i32;
        let K: i32 = *left.offset(2 as isize) as i32;
        let L: i32 = *left.offset(3 as isize) as i32;
        *dst.offset(0 as isize + 0 as isize * stride) = ((I + J + 1 as i32) >> 1 as i32) as u8;
        let fresh21 = &mut *dst.offset(0 as isize + 1 as isize * stride);
        *fresh21 = ((J + K + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh21;
        let fresh22 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh22 = ((K + L + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(2 as isize + 1 as isize * stride) = *fresh22;
        *dst.offset(1 as isize + 0 as isize * stride) =
            ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as u8;
        let fresh23 = &mut *dst.offset(1 as isize + 1 as isize * stride);
        *fresh23 = ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 0 as isize * stride) = *fresh23;
        let fresh24 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh24 = ((K + 2 as i32 * L + L + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh24;
        let fresh25 = &mut *dst.offset(3 as isize + 3 as isize * stride);
        *fresh25 = L as u8;
        let fresh26 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh26 = *fresh25;
        let fresh27 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh27 = *fresh26;
        let fresh28 = &mut *dst.offset(0 as isize + 3 as isize * stride);
        *fresh28 = *fresh27;
        let fresh29 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh29 = *fresh28;
        *dst.offset(3 as isize + 2 as isize * stride) = *fresh29;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    _left: *const u8,
) {
    unsafe {
        let A: i32 = *above.offset(0 as isize) as i32;
        let B: i32 = *above.offset(1 as isize) as i32;
        let C: i32 = *above.offset(2 as isize) as i32;
        let D: i32 = *above.offset(3 as isize) as i32;
        let E: i32 = *above.offset(4 as isize) as i32;
        let F: i32 = *above.offset(5 as isize) as i32;
        let G: i32 = *above.offset(6 as isize) as i32;
        *dst.offset(0 as isize + 0 as isize * stride) = ((A + B + 1 as i32) >> 1 as i32) as u8;
        let fresh48 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh48 = ((B + C + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh48;
        let fresh49 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh49 = ((C + D + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh49;
        let fresh50 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh50 = ((D + E + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(3 as isize + 0 as isize * stride) = *fresh50;
        *dst.offset(3 as isize + 2 as isize * stride) = ((E + F + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 1 as isize * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        let fresh51 = &mut *dst.offset(0 as isize + 3 as isize * stride);
        *fresh51 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 1 as isize * stride) = *fresh51;
        let fresh52 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh52 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(2 as isize + 1 as isize * stride) = *fresh52;
        let fresh53 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh53 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh53;
        *dst.offset(3 as isize + 3 as isize * stride) =
            ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63e_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    _left: *const u8,
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
        *dst.offset(0 as isize + 0 as isize * stride) = ((A + B + 1 as i32) >> 1 as i32) as u8;
        let fresh54 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh54 = ((B + C + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh54;
        let fresh55 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh55 = ((C + D + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh55;
        let fresh56 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh56 = ((D + E + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(3 as isize + 0 as isize * stride) = *fresh56;
        *dst.offset(3 as isize + 2 as isize * stride) =
            ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(0 as isize + 1 as isize * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        let fresh57 = &mut *dst.offset(0 as isize + 3 as isize * stride);
        *fresh57 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 1 as isize * stride) = *fresh57;
        let fresh58 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh58 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(2 as isize + 1 as isize * stride) = *fresh58;
        let fresh59 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh59 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh59;
        *dst.offset(3 as isize + 3 as isize * stride) =
            ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    _left: *const u8,
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
        *dst.offset(0 as isize + 0 as isize * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        let fresh30 = &mut *dst.offset(0 as isize + 1 as isize * stride);
        *fresh30 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh30;
        let fresh31 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh31 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as u8;
        let fresh32 = &mut *dst.offset(1 as isize + 1 as isize * stride);
        *fresh32 = *fresh31;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh32;
        let fresh33 = &mut *dst.offset(0 as isize + 3 as isize * stride);
        *fresh33 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as u8;
        let fresh34 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh34 = *fresh33;
        let fresh35 = &mut *dst.offset(2 as isize + 1 as isize * stride);
        *fresh35 = *fresh34;
        *dst.offset(3 as isize + 0 as isize * stride) = *fresh35;
        let fresh36 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh36 = ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as u8;
        let fresh37 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh37 = *fresh36;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh37;
        let fresh38 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh38 = ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 2 as isize * stride) = *fresh38;
        *dst.offset(3 as isize + 3 as isize * stride) = H as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45e_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    _left: *const u8,
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
        *dst.offset(0 as isize + 0 as isize * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        let fresh39 = &mut *dst.offset(0 as isize + 1 as isize * stride);
        *fresh39 = ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh39;
        let fresh40 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh40 = ((C + 2 as i32 * D + E + 2 as i32) >> 2 as i32) as u8;
        let fresh41 = &mut *dst.offset(1 as isize + 1 as isize * stride);
        *fresh41 = *fresh40;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh41;
        let fresh42 = &mut *dst.offset(0 as isize + 3 as isize * stride);
        *fresh42 = ((D + 2 as i32 * E + F + 2 as i32) >> 2 as i32) as u8;
        let fresh43 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh43 = *fresh42;
        let fresh44 = &mut *dst.offset(2 as isize + 1 as isize * stride);
        *fresh44 = *fresh43;
        *dst.offset(3 as isize + 0 as isize * stride) = *fresh44;
        let fresh45 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh45 = ((E + 2 as i32 * F + G + 2 as i32) >> 2 as i32) as u8;
        let fresh46 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh46 = *fresh45;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh46;
        let fresh47 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh47 = ((F + 2 as i32 * G + H + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 2 as isize * stride) = *fresh47;
        *dst.offset(3 as isize + 3 as isize * stride) =
            ((G + 2 as i32 * H + H + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
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
        let fresh0 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh0 = ((X + A + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 0 as isize * stride) = *fresh0;
        let fresh1 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh1 = ((A + B + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh1;
        let fresh2 = &mut *dst.offset(3 as isize + 2 as isize * stride);
        *fresh2 = ((B + C + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(2 as isize + 0 as isize * stride) = *fresh2;
        *dst.offset(3 as isize + 0 as isize * stride) = ((C + D + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 3 as isize * stride) =
            ((K + 2 as i32 * J + I + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(0 as isize + 2 as isize * stride) =
            ((J + 2 as i32 * I + X + 2 as i32) >> 2 as i32) as u8;
        let fresh3 = &mut *dst.offset(1 as isize + 3 as isize * stride);
        *fresh3 = ((I + 2 as i32 * X + A + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(0 as isize + 1 as isize * stride) = *fresh3;
        let fresh4 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh4 = ((X + 2 as i32 * A + B + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 1 as isize * stride) = *fresh4;
        let fresh5 = &mut *dst.offset(3 as isize + 3 as isize * stride);
        *fresh5 = ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(2 as isize + 1 as isize * stride) = *fresh5;
        *dst.offset(3 as isize + 1 as isize * stride) =
            ((B + 2 as i32 * C + D + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
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
        *dst.offset(0 as isize + 3 as isize * stride) =
            ((J + 2 as i32 * K + L + 2 as i32) >> 2 as i32) as u8;
        let fresh6 = &mut *dst.offset(0 as isize + 2 as isize * stride);
        *fresh6 = ((I + 2 as i32 * J + K + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 3 as isize * stride) = *fresh6;
        let fresh7 = &mut *dst.offset(0 as isize + 1 as isize * stride);
        *fresh7 = ((X + 2 as i32 * I + J + 2 as i32) >> 2 as i32) as u8;
        let fresh8 = &mut *dst.offset(1 as isize + 2 as isize * stride);
        *fresh8 = *fresh7;
        *dst.offset(2 as isize + 3 as isize * stride) = *fresh8;
        let fresh9 = &mut *dst.offset(0 as isize + 0 as isize * stride);
        *fresh9 = ((A + 2 as i32 * X + I + 2 as i32) >> 2 as i32) as u8;
        let fresh10 = &mut *dst.offset(1 as isize + 1 as isize * stride);
        *fresh10 = *fresh9;
        let fresh11 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh11 = *fresh10;
        *dst.offset(3 as isize + 3 as isize * stride) = *fresh11;
        let fresh12 = &mut *dst.offset(1 as isize + 0 as isize * stride);
        *fresh12 = ((B + 2 as i32 * A + X + 2 as i32) >> 2 as i32) as u8;
        let fresh13 = &mut *dst.offset(2 as isize + 1 as isize * stride);
        *fresh13 = *fresh12;
        *dst.offset(3 as isize + 2 as isize * stride) = *fresh13;
        let fresh14 = &mut *dst.offset(2 as isize + 0 as isize * stride);
        *fresh14 = ((C + 2 as i32 * B + A + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(3 as isize + 1 as isize * stride) = *fresh14;
        *dst.offset(3 as isize + 0 as isize * stride) =
            ((D + 2 as i32 * C + B + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
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
        let fresh15 = &mut *dst.offset(2 as isize + 1 as isize * stride);
        *fresh15 = ((I + X + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 0 as isize * stride) = *fresh15;
        let fresh16 = &mut *dst.offset(2 as isize + 2 as isize * stride);
        *fresh16 = ((J + I + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 1 as isize * stride) = *fresh16;
        let fresh17 = &mut *dst.offset(2 as isize + 3 as isize * stride);
        *fresh17 = ((K + J + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(0 as isize + 2 as isize * stride) = *fresh17;
        *dst.offset(0 as isize + 3 as isize * stride) = ((L + K + 1 as i32) >> 1 as i32) as u8;
        *dst.offset(3 as isize + 0 as isize * stride) =
            ((A + 2 as i32 * B + C + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(2 as isize + 0 as isize * stride) =
            ((X + 2 as i32 * A + B + 2 as i32) >> 2 as i32) as u8;
        let fresh18 = &mut *dst.offset(3 as isize + 1 as isize * stride);
        *fresh18 = ((I + 2 as i32 * X + A + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 0 as isize * stride) = *fresh18;
        let fresh19 = &mut *dst.offset(3 as isize + 2 as isize * stride);
        *fresh19 = ((J + 2 as i32 * I + X + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 1 as isize * stride) = *fresh19;
        let fresh20 = &mut *dst.offset(3 as isize + 3 as isize * stride);
        *fresh20 = ((K + 2 as i32 * J + I + 2 as i32) >> 2 as i32) as u8;
        *dst.offset(1 as isize + 2 as isize * stride) = *fresh20;
        *dst.offset(1 as isize + 3 as isize * stride) =
            ((L + 2 as i32 * K + J + 2 as i32) >> 2 as i32) as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d207_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d207_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d207_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d207_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d63_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d63_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d63_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d63_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d45_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d45_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d45_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d45_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d117_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d117_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d117_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d117_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d135_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d135_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d135_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d135_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d153_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d153_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_d153_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        d153_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        v_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        v_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        v_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_v_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        v_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        h_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        h_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        h_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_h_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        h_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        tm_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        tm_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        tm_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tm_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        tm_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_128_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_128_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_128_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_128_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_128_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_left_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_left_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_left_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_left_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_left_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_top_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_top_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_top_predictor(dst, stride, 16 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_top_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_top_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_32x32_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_predictor(dst, stride, 32 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_4x4_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_predictor(dst, stride, 4 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_8x8_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_predictor(dst, stride, 8 as i32, above, left);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dc_predictor_16x16_c(
    mut dst: *mut u8,
    mut stride: isize,
    mut above: *const u8,
    mut left: *const u8,
) {
    unsafe {
        dc_predictor(dst, stride, 16 as i32, above, left);
    }
}
