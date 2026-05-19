
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;

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
pub fn vpx_d45_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    above: &[u8],
) {
    let above_right = above[bs - 1];

    for x in 0..bs - 1 {
        dst[x] = ((above[x] as i32
            + 2 * above[x + 1] as i32
            + above[x + 2] as i32
            + 2) >> 2) as u8;
    }
    dst[bs - 1] = above_right;

    let mut size = bs - 2;
    for r in 1..bs {
        for i in 0..size {
            dst[r * stride + i] = dst[r + i];
        }
        dst[r * stride + size..r * stride + bs].fill(above_right);
        size -= 1;
    }
}
pub fn vpx_d117_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    above: &[u8],
    left: &[u8],
) {
    for c in 0..bs {
        dst[c] = ((above[c] as i32 + above[c + 1] as i32 + 1) >> 1) as u8;
    }

    dst[stride] = ((left[0] as i32 + 2 * above[0] as i32 + above[1] as i32 + 2) >> 2) as u8;

    for c in 1..bs {
        dst[stride + c] =
            ((above[c - 1] as i32 + 2 * above[c] as i32 + above[c + 1] as i32 + 2) >> 2) as u8;
    }

    dst[2 * stride] = ((above[0] as i32 + 2 * left[0] as i32 + left[1] as i32 + 2) >> 2) as u8;

    for r in 3..bs {
        dst[r * stride] =
            ((left[r - 3] as i32 + 2 * left[r - 2] as i32 + left[r - 1] as i32 + 2) >> 2) as u8;
    }

    for r in 2..bs {
        for c in 1..bs {
            dst[r * stride + c] = dst[(r - 2) * stride + c - 1];
        }
    }
}


pub fn vpx_d153_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    above: &[u8],
    left: &[u8],
) {
    let mut r0 = [0u8; 32];
    let mut c0 = [0u8; 32];
    let mut c1 = [0u8; 32];

    let bs = bs.min(32);

    // Setup boundaries
    r0[0] = ((left[0] as i32 + above[0] as i32 + 1) >> 1) as u8;
    r0[1] = ((left[0] as i32 + 2 * above[0] as i32 + above[1] as i32 + 2) >> 2) as u8;
    for c in 2..bs {
        r0[c] = ((above[c - 2] as i32 + 2 * above[c - 1] as i32 + above[c] as i32 + 2) >> 2) as u8;
    }

    c0[0] = r0[0];
    for r in 1..bs {
        c0[r] = ((left[r - 1] as i32 + left[r] as i32 + 1) >> 1) as u8;
    }

    c1[0] = r0[1];
    c1[1] = ((above[0] as i32 + 2 * left[0] as i32 + left[1] as i32 + 2) >> 2) as u8;
    for r in 2..bs {
        c1[r] = ((left[r - 2] as i32 + 2 * left[r - 1] as i32 + left[r] as i32 + 2) >> 2) as u8;
    }

    // Fill dst
    for r in 0..bs {
        let start = r * stride;
        dst[start] = c0[r];
        dst[start + 1] = c1[r];
        for c in 2..bs {
            if c >= 2 * r + 2 {
                dst[start + c] = r0[c - 2 * r];
            } else {
                if c % 2 == 0 {
                    dst[start + c] = c0[r - c / 2];
                } else {
                    dst[start + c] = c1[r - (c - 1) / 2];
                }
            }
        }
    }
}


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
pub extern "C" fn vpx_d45_predictor_4x4_c(
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
        vpx_d45_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
pub fn vpx_d45_predictor_4x4_safe(
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

    dst[3 * stride + 3] = H as u8;
}
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


pub fn vpx_d135_predictor_safe(
    dst: &mut [u8],
    stride: usize,
    bs: usize,
    above: &[u8],
    left: &[u8],
) {
    let mut border: [u8; 63] = [0; 63];

    for i in 0..(bs - 2) {
        border[i] = ((left[bs - 3 - i] as i32
            + 2 * left[bs - 2 - i] as i32
            + left[bs - 1 - i] as i32
            + 2)
            >> 2) as u8;
    }

    border[bs - 2] = ((above[0] as i32
        + 2 * left[0] as i32
        + left[1] as i32
        + 2)
        >> 2) as u8;

    border[bs - 1] = ((left[0] as i32
        + 2 * above[0] as i32
        + above[1] as i32
        + 2)
        >> 2) as u8;

    border[bs] = ((above[0] as i32
        + 2 * above[1] as i32
        + above[2] as i32
        + 2)
        >> 2) as u8;

    for i in 0..(bs - 2) {
        border[bs + 1 + i] = ((above[i + 1] as i32
            + 2 * above[i + 2] as i32
            + above[i + 3] as i32
            + 2)
            >> 2) as u8;
    }

    for i in 0..bs {
        let src_start = bs - 1 - i;
        let dst_start = i * stride;
        dst[dst_start..dst_start + bs].copy_from_slice(&border[src_start..src_start + bs]);
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
pub extern "C" fn vpx_d45_predictor_8x8_c(
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
        vpx_d45_predictor_safe(dst_slice, stride as usize, 8, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d45_predictor_32x32_c(
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
        vpx_d45_predictor_safe(dst_slice, stride as usize, 32, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d45_predictor_16x16_c(
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
        vpx_d45_predictor_safe(dst_slice, stride as usize, 16, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d117_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 33);
        let left_slice = core::slice::from_raw_parts(left, 32);
        vpx_d117_predictor_safe(dst_slice, stride as usize, 32, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d117_predictor_8x8_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 9);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_d117_predictor_safe(dst_slice, stride as usize, 8, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d117_predictor_16x16_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 17);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_d117_predictor_safe(dst_slice, stride as usize, 16, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d135_predictor_16x16_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 17);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_d135_predictor_safe(dst_slice, stride as usize, 16, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d135_predictor_8x8_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 9);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_d135_predictor_safe(dst_slice, stride as usize, 8, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d135_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 33);
        let left_slice = core::slice::from_raw_parts(left, 32);
        vpx_d135_predictor_safe(dst_slice, stride as usize, 32, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d153_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 32);
        let left_slice = core::slice::from_raw_parts(left, 32);
        vpx_d153_predictor_safe(dst_slice, stride as usize, 32, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d153_predictor_8x8_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 8);
        let left_slice = core::slice::from_raw_parts(left, 8);
        vpx_d153_predictor_safe(dst_slice, stride as usize, 8, above_slice, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_d153_predictor_16x16_c(
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
        let above_slice = core::slice::from_raw_parts(above.offset(-1), 16);
        let left_slice = core::slice::from_raw_parts(left, 16);
        vpx_d153_predictor_safe(dst_slice, stride as usize, 16, above_slice, left_slice);
    }
}
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
pub fn vpx_v_predictor_32x32_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    for r in 0..32 {
        let start = r * stride;
        dst[start..start + 32].copy_from_slice(&above[..32]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_v_predictor_32x32_c(
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
        let above_slice = core::slice::from_raw_parts(above, 32);
        vpx_v_predictor_32x32_safe(dst_slice, stride as usize, above_slice);
    }
}

pub fn vpx_v_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
) {
    for r in 0..4 {
        let start = r * stride;
        dst[start..start + 4].copy_from_slice(&above[..4]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_v_predictor_4x4_c(
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
        let above_slice = core::slice::from_raw_parts(above, 4);
        vpx_v_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
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
pub fn vpx_h_predictor_32x32_safe(
    dst: &mut [u8],
    stride: usize,
    left: &[u8],
) {
    for r in 0..32 {
        let start = r * stride;
        let val = left[r];
        dst[start..start + 32].fill(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_h_predictor_32x32_c(
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
        vpx_h_predictor_32x32_safe(dst_slice, stride as usize, left_slice);
    }
}
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
pub fn vpx_h_predictor_4x4_safe(
    dst: &mut [u8],
    stride: usize,
    left: &[u8],
) {
    for r in 0..4 {
        let start = r * stride;
        let val = left[r];
        dst[start..start + 4].fill(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_h_predictor_4x4_c(
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
        vpx_h_predictor_4x4_safe(dst_slice, stride as usize, left_slice);
    }
}
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
pub fn vpx_tm_predictor_32x32_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
    top_left: u8,
) {
    let ytop_left = top_left as i32;
    for r in 0..32 {
        let dst_idx = r * stride;
        let left_val = left[r] as i32;
        for c in 0..32 {
            let val = left_val + above[c] as i32 - ytop_left;
            dst[dst_idx + c] = val.clamp(0, 255) as u8;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_tm_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 32);
        let left_slice = core::slice::from_raw_parts(left, 32);
        let top_left = *above.offset(-1);
        vpx_tm_predictor_32x32_safe(dst_slice, stride as usize, above_slice, left_slice, top_left);
    }
}
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
pub extern "C" fn vpx_dc_128_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vpx_dc_128_predictor_32x32_safe(dst_slice, stride as usize);
    }
}
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
pub extern "C" fn vpx_dc_128_predictor_4x4_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    _above: *const uint8_t,
    _left: *const uint8_t,
) {
    if dst.is_null() {
        return;
    }
    unsafe {
        let dst_len = 3 * stride as usize + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vpx_dc_128_predictor_4x4_safe(dst_slice, stride as usize);
    }
}
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
pub extern "C" fn vpx_dc_left_predictor_4x4_c(
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
        vpx_dc_left_predictor_4x4_safe(dst_slice, stride as usize, left_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_left_predictor_32x32_c(
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
        vpx_dc_left_predictor_32x32_safe(dst_slice, stride as usize, left_slice);
    }
}
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
pub extern "C" fn vpx_dc_top_predictor_32x32_c(
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
        let above_slice = core::slice::from_raw_parts(above, 32);
        vpx_dc_top_predictor_32x32_safe(dst_slice, stride as usize, above_slice);
    }
}
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
pub extern "C" fn vpx_dc_top_predictor_4x4_c(
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
        let above_slice = core::slice::from_raw_parts(above, 4);
        vpx_dc_top_predictor_4x4_safe(dst_slice, stride as usize, above_slice);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vpx_dc_predictor_32x32_c(
    dst: *mut uint8_t,
    stride: ptrdiff_t,
    above: *const uint8_t,
    left: *const uint8_t,
) {
    if dst.is_null() || above.is_null() || left.is_null() {
        return;
    }
    unsafe {
        let dst_len = 31 * stride as usize + 32;
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        let above_slice = core::slice::from_raw_parts(above, 32);
        let left_slice = core::slice::from_raw_parts(left, 32);
        vpx_dc_predictor_32x32_safe(dst_slice, stride as usize, above_slice, left_slice);
    }
}
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

pub fn vpx_dc_128_predictor_4x4_safe(dst: &mut [u8], stride: usize) {
    for r in 0..4 {
        let start = r * stride;
        dst[start..start + 4].fill(128);
    }
}

pub fn vpx_dc_left_predictor_4x4_safe(dst: &mut [u8], stride: usize, left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..4 {
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 2) / 4) as u8;
    for r in 0..4 {
        let start = r * stride;
        dst[start..start + 4].fill(expected_dc);
    }
}

pub fn vpx_dc_top_predictor_4x4_safe(dst: &mut [u8], stride: usize, above: &[u8]) {
    let mut sum = 0i32;
    for i in 0..4 {
        sum += above[i] as i32;
    }
    let expected_dc = ((sum + 2) / 4) as u8;
    for r in 0..4 {
        let start = r * stride;
        dst[start..start + 4].fill(expected_dc);
    }
}

pub fn vpx_dc_128_predictor_32x32_safe(dst: &mut [u8], stride: usize) {
    for r in 0..32 {
        let start = r * stride;
        dst[start..start + 32].fill(128);
    }
}

pub fn vpx_dc_left_predictor_32x32_safe(dst: &mut [u8], stride: usize, left: &[u8]) {
    let mut sum = 0i32;
    for i in 0..32 {
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 16) / 32) as u8;
    for r in 0..32 {
        let start = r * stride;
        dst[start..start + 32].fill(expected_dc);
    }
}

pub fn vpx_dc_top_predictor_32x32_safe(dst: &mut [u8], stride: usize, above: &[u8]) {
    let mut sum = 0i32;
    for i in 0..32 {
        sum += above[i] as i32;
    }
    let expected_dc = ((sum + 16) / 32) as u8;
    for r in 0..32 {
        let start = r * stride;
        dst[start..start + 32].fill(expected_dc);
    }
}

pub fn vpx_dc_predictor_32x32_safe(
    dst: &mut [u8],
    stride: usize,
    above: &[u8],
    left: &[u8],
) {
    let mut sum = 0i32;
    for i in 0..32 {
        sum += above[i] as i32;
        sum += left[i] as i32;
    }
    let expected_dc = ((sum + 32) / 64) as u8;
    for r in 0..32 {
        let start = r * stride;
        dst[start..start + 32].fill(expected_dc);
    }
}
