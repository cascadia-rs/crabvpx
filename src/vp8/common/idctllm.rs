static cospi8sqrt2minus1: i32 = 20091 as i32;
static sinpi8sqrt2: i32 = 35468 as i32;
#[unsafe(no_mangle)]
pub unsafe fn vp8_short_idct4x4llm_c(
    mut input: *mut i16,
    mut pred_ptr: *mut u8,
    mut pred_stride: i32,
    mut dst_ptr: *mut u8,
    mut dst_stride: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut a1: i32 = 0;
        let mut b1: i32 = 0;
        let mut c1: i32 = 0;
        let mut d1: i32 = 0;
        let mut output: [i16; 16] = [0; 16];
        let mut ip: *mut i16 = input;
        let mut op: *mut i16 = &raw mut output as *mut i16;
        let mut temp1: i32 = 0;
        let mut temp2: i32 = 0;
        let mut shortpitch: i32 = 4 as i32;
        i = 0 as i32;
        while i < 4 as i32 {
            a1 = *ip.offset(0 as isize) as i32 + *ip.offset(8 as isize) as i32;
            b1 = *ip.offset(0 as isize) as i32 - *ip.offset(8 as isize) as i32;
            temp1 = (*ip.offset(4 as isize) as i32 * sinpi8sqrt2) >> 16 as i32;
            temp2 = *ip.offset(12 as isize) as i32
                + ((*ip.offset(12 as isize) as i32 * cospi8sqrt2minus1) >> 16 as i32);
            c1 = temp1 - temp2;
            temp1 = *ip.offset(4 as isize) as i32
                + ((*ip.offset(4 as isize) as i32 * cospi8sqrt2minus1) >> 16 as i32);
            temp2 = (*ip.offset(12 as isize) as i32 * sinpi8sqrt2) >> 16 as i32;
            d1 = temp1 + temp2;
            *op.offset((shortpitch * 0 as i32) as isize) = (a1 + d1) as i16;
            *op.offset((shortpitch * 3 as i32) as isize) = (a1 - d1) as i16;
            *op.offset((shortpitch * 1 as i32) as isize) = (b1 + c1) as i16;
            *op.offset((shortpitch * 2 as i32) as isize) = (b1 - c1) as i16;
            ip = ip.offset(1);
            op = op.offset(1);
            i += 1;
        }
        ip = &raw mut output as *mut i16;
        op = &raw mut output as *mut i16;
        i = 0 as i32;
        while i < 4 as i32 {
            a1 = *ip.offset(0 as isize) as i32 + *ip.offset(2 as isize) as i32;
            b1 = *ip.offset(0 as isize) as i32 - *ip.offset(2 as isize) as i32;
            temp1 = (*ip.offset(1 as isize) as i32 * sinpi8sqrt2) >> 16 as i32;
            temp2 = *ip.offset(3 as isize) as i32
                + ((*ip.offset(3 as isize) as i32 * cospi8sqrt2minus1) >> 16 as i32);
            c1 = temp1 - temp2;
            temp1 = *ip.offset(1 as isize) as i32
                + ((*ip.offset(1 as isize) as i32 * cospi8sqrt2minus1) >> 16 as i32);
            temp2 = (*ip.offset(3 as isize) as i32 * sinpi8sqrt2) >> 16 as i32;
            d1 = temp1 + temp2;
            *op.offset(0 as isize) = ((a1 + d1 + 4 as i32) >> 3 as i32) as i16;
            *op.offset(3 as isize) = ((a1 - d1 + 4 as i32) >> 3 as i32) as i16;
            *op.offset(1 as isize) = ((b1 + c1 + 4 as i32) >> 3 as i32) as i16;
            *op.offset(2 as isize) = ((b1 - c1 + 4 as i32) >> 3 as i32) as i16;
            ip = ip.offset(shortpitch as isize);
            op = op.offset(shortpitch as isize);
            i += 1;
        }
        ip = &raw mut output as *mut i16;
        r = 0 as i32;
        while r < 4 as i32 {
            c = 0 as i32;
            while c < 4 as i32 {
                let mut a: i32 =
                    *ip.offset(c as isize) as i32 + *pred_ptr.offset(c as isize) as i32;
                if a < 0 as i32 {
                    a = 0 as i32;
                }
                if a > 255 as i32 {
                    a = 255 as i32;
                }
                *dst_ptr.offset(c as isize) = a as u8;
                c += 1;
            }
            ip = ip.offset(4 as isize);
            dst_ptr = dst_ptr.offset(dst_stride as isize);
            pred_ptr = pred_ptr.offset(pred_stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dc_only_idct_add_c(
    mut input_dc: i16,
    mut pred_ptr: *mut u8,
    mut pred_stride: i32,
    mut dst_ptr: *mut u8,
    mut dst_stride: i32,
) {
    unsafe {
        let mut a1: i32 = (input_dc as i32 + 4 as i32) >> 3 as i32;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        r = 0 as i32;
        while r < 4 as i32 {
            c = 0 as i32;
            while c < 4 as i32 {
                let mut a: i32 = a1 + *pred_ptr.offset(c as isize) as i32;
                if a < 0 as i32 {
                    a = 0 as i32;
                }
                if a > 255 as i32 {
                    a = 255 as i32;
                }
                *dst_ptr.offset(c as isize) = a as u8;
                c += 1;
            }
            dst_ptr = dst_ptr.offset(dst_stride as isize);
            pred_ptr = pred_ptr.offset(pred_stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_short_inv_walsh4x4_c(mut input: *mut i16, mut mb_dqcoeff: *mut i16) {
    unsafe {
        let mut output: [i16; 16] = [0; 16];
        let mut i: i32 = 0;
        let mut a1: i32 = 0;
        let mut b1: i32 = 0;
        let mut c1: i32 = 0;
        let mut d1: i32 = 0;
        let mut a2: i32 = 0;
        let mut b2: i32 = 0;
        let mut c2: i32 = 0;
        let mut d2: i32 = 0;
        let mut ip: *mut i16 = input;
        let mut op: *mut i16 = &raw mut output as *mut i16;
        i = 0 as i32;
        while i < 4 as i32 {
            a1 = *ip.offset(0 as isize) as i32 + *ip.offset(12 as isize) as i32;
            b1 = *ip.offset(4 as isize) as i32 + *ip.offset(8 as isize) as i32;
            c1 = *ip.offset(4 as isize) as i32 - *ip.offset(8 as isize) as i32;
            d1 = *ip.offset(0 as isize) as i32 - *ip.offset(12 as isize) as i32;
            *op.offset(0 as isize) = (a1 + b1) as i16;
            *op.offset(4 as isize) = (c1 + d1) as i16;
            *op.offset(8 as isize) = (a1 - b1) as i16;
            *op.offset(12 as isize) = (d1 - c1) as i16;
            ip = ip.offset(1);
            op = op.offset(1);
            i += 1;
        }
        ip = &raw mut output as *mut i16;
        op = &raw mut output as *mut i16;
        i = 0 as i32;
        while i < 4 as i32 {
            a1 = *ip.offset(0 as isize) as i32 + *ip.offset(3 as isize) as i32;
            b1 = *ip.offset(1 as isize) as i32 + *ip.offset(2 as isize) as i32;
            c1 = *ip.offset(1 as isize) as i32 - *ip.offset(2 as isize) as i32;
            d1 = *ip.offset(0 as isize) as i32 - *ip.offset(3 as isize) as i32;
            a2 = a1 + b1;
            b2 = c1 + d1;
            c2 = a1 - b1;
            d2 = d1 - c1;
            *op.offset(0 as isize) = ((a2 + 3 as i32) >> 3 as i32) as i16;
            *op.offset(1 as isize) = ((b2 + 3 as i32) >> 3 as i32) as i16;
            *op.offset(2 as isize) = ((c2 + 3 as i32) >> 3 as i32) as i16;
            *op.offset(3 as isize) = ((d2 + 3 as i32) >> 3 as i32) as i16;
            ip = ip.offset(4 as isize);
            op = op.offset(4 as isize);
            i += 1;
        }
        i = 0 as i32;
        while i < 16 as i32 {
            *mb_dqcoeff.offset((i * 16 as i32) as isize) = output[i as usize];
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_short_inv_walsh4x4_1_c(mut input: *mut i16, mut mb_dqcoeff: *mut i16) {
    unsafe {
        let mut i: i32 = 0;
        let mut a1: i32 = 0;
        a1 = (*input.offset(0 as isize) as i32 + 3 as i32) >> 3 as i32;
        i = 0 as i32;
        while i < 16 as i32 {
            *mb_dqcoeff.offset((i * 16 as i32) as isize) = a1 as i16;
            i += 1;
        }
    }
}
