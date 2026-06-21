//! Sub-pixel filter coefficients — port of `vp8/common/filter.c`.
//!
//! The 6-tap and bilinear interpolation coefficient tables (`vp8_sub_pel_filters`
//! etc.). The interpolation kernels themselves live in [`crate::vp8::common::simd`]
//! / safe_predict; this module holds the shared coefficients.

pub const VP8_FILTER_WEIGHT: i32 = 128_i32;
pub const VP8_FILTER_SHIFT: i32 = 7_i32;

pub static vp8_bilinear_filters: [[i16; 2]; 8] = [
    [128_i32 as i16, 0_i32 as i16],
    [112_i32 as i16, 16_i32 as i16],
    [96_i32 as i16, 32_i32 as i16],
    [80_i32 as i16, 48_i32 as i16],
    [64_i32 as i16, 64_i32 as i16],
    [48_i32 as i16, 80_i32 as i16],
    [32_i32 as i16, 96_i32 as i16],
    [16_i32 as i16, 112_i32 as i16],
];

pub static vp8_sub_pel_filters: [[i16; 6]; 8] = [
    [
        0_i32 as i16,
        0_i32 as i16,
        128_i32 as i16,
        0_i32 as i16,
        0_i32 as i16,
        0_i32 as i16,
    ],
    [
        0_i32 as i16,
        -6_i32 as i16,
        123_i32 as i16,
        12_i32 as i16,
        -1_i32 as i16,
        0_i32 as i16,
    ],
    [
        2_i32 as i16,
        -11_i32 as i16,
        108_i32 as i16,
        36_i32 as i16,
        -8_i32 as i16,
        1_i32 as i16,
    ],
    [
        0_i32 as i16,
        -9_i32 as i16,
        93_i32 as i16,
        50_i32 as i16,
        -6_i32 as i16,
        0_i32 as i16,
    ],
    [
        3_i32 as i16,
        -16_i32 as i16,
        77_i32 as i16,
        77_i32 as i16,
        -16_i32 as i16,
        3_i32 as i16,
    ],
    [
        0_i32 as i16,
        -6_i32 as i16,
        50_i32 as i16,
        93_i32 as i16,
        -9_i32 as i16,
        0_i32 as i16,
    ],
    [
        1_i32 as i16,
        -8_i32 as i16,
        36_i32 as i16,
        108_i32 as i16,
        -11_i32 as i16,
        2_i32 as i16,
    ],
    [
        0_i32 as i16,
        -1_i32 as i16,
        12_i32 as i16,
        123_i32 as i16,
        -6_i32 as i16,
        0_i32 as i16,
    ],
];

fn filter_block2d_first_pass_safe(
    src: &[u8],
    src_stride: usize,
    output_height: usize,
    output_width: usize,
    vp8_filter: &[i16; 6],
    output: &mut [i32],
) {
    let f0 = vp8_filter[0] as i32;
    let f1 = vp8_filter[1] as i32;
    let f2 = vp8_filter[2] as i32;
    let f3 = vp8_filter[3] as i32;
    let f4 = vp8_filter[4] as i32;
    let f5 = vp8_filter[5] as i32;
    let half_weight = VP8_FILTER_WEIGHT >> 1;

    let req_src = (output_height.saturating_sub(1)) * src_stride + output_width + 5;
    let req_out = output_height * output_width;
    if src.len() < req_src || output.len() < req_out {
        return;
    }
    let src = &src[..req_src];
    let output = &mut output[..req_out];

    for i in 0..output_height {
        let src_row = &src[i * src_stride..i * src_stride + output_width + 5];
        let out_row = &mut output[i * output_width..(i + 1) * output_width];
        for (out, window) in out_row.iter_mut().zip(src_row.windows(6)) {
            let mut temp = window[0] as i32 * f0
                + window[1] as i32 * f1
                + window[2] as i32 * f2
                + window[3] as i32 * f3
                + window[4] as i32 * f4
                + window[5] as i32 * f5
                + half_weight;

            temp >>= VP8_FILTER_SHIFT;
            *out = temp.clamp(0, 255);
        }
    }
}

fn filter_block2d_second_pass_safe(
    src: &[i32],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    output_height: usize,
    output_width: usize,
    vp8_filter: &[i16; 6],
) {
    let f0 = vp8_filter[0] as i32;
    let f1 = vp8_filter[1] as i32;
    let f2 = vp8_filter[2] as i32;
    let f3 = vp8_filter[3] as i32;
    let f4 = vp8_filter[4] as i32;
    let f5 = vp8_filter[5] as i32;
    let half_weight = VP8_FILTER_WEIGHT >> 1;

    let req_src = (output_height + 5) * src_stride;
    let req_dst = (output_height.saturating_sub(1)) * dst_pitch + output_width;
    if src.len() < req_src || dst.len() < req_dst {
        return;
    }
    let src = &src[..req_src];
    let dst = &mut dst[..req_dst];

    for i in 0..output_height {
        let r0 = &src[i * src_stride..i * src_stride + output_width];
        let r1 = &src[(i + 1) * src_stride..(i + 1) * src_stride + output_width];
        let r2 = &src[(i + 2) * src_stride..(i + 2) * src_stride + output_width];
        let r3 = &src[(i + 3) * src_stride..(i + 3) * src_stride + output_width];
        let r4 = &src[(i + 4) * src_stride..(i + 4) * src_stride + output_width];
        let r5 = &src[(i + 5) * src_stride..(i + 5) * src_stride + output_width];
        let dst_row = &mut dst[i * dst_pitch..i * dst_pitch + output_width];

        for ((((((dst_pix, &p0), &p1), &p2), &p3), &p4), &p5) in dst_row
            .iter_mut()
            .zip(r0.iter())
            .zip(r1.iter())
            .zip(r2.iter())
            .zip(r3.iter())
            .zip(r4.iter())
            .zip(r5.iter())
        {
            let mut temp = p0 * f0 + p1 * f1 + p2 * f2 + p3 * f3 + p4 * f4 + p5 * f5 + half_weight;

            temp >>= VP8_FILTER_SHIFT;
            *dst_pix = temp.clamp(0, 255) as u8;
        }
    }
}

fn filter_block2d_safe(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    h_filter: &[i16; 6],
    v_filter: &[i16; 6],
) {
    let mut f_data = [0i32; 36]; // 4x9
    filter_block2d_first_pass_safe(src, src_stride, 9, 4, h_filter, &mut f_data);
    filter_block2d_second_pass_safe(&f_data, 4, dst, dst_pitch, 4, 4, v_filter);
}

fn filter_block2d_bil_first_pass_safe(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u16],
    dst_width: usize,
    height: usize,
    width: usize,
    vp8_filter: &[i16; 2],
) {
    let f0 = vp8_filter[0] as i32;
    let f1 = vp8_filter[1] as i32;
    let half_weight = VP8_FILTER_WEIGHT / 2;

    let req_src = (height.saturating_sub(1)) * src_stride + width + 1;
    let req_dst = height * dst_width;
    if src.len() < req_src || dst.len() < req_dst {
        return;
    }
    let src = &src[..req_src];
    let dst = &mut dst[..req_dst];

    for i in 0..height {
        let src_row = &src[i * src_stride..i * src_stride + width + 1];
        let dst_row = &mut dst[i * dst_width..(i + 1) * dst_width];
        for (out, window) in dst_row.iter_mut().zip(src_row.windows(2)) {
            let temp = window[0] as i32 * f0 + window[1] as i32 * f1 + half_weight;
            *out = (temp >> VP8_FILTER_SHIFT) as u16;
        }
    }
}

fn filter_block2d_bil_second_pass_safe(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    height: usize,
    width: usize,
    vp8_filter: &[i16; 2],
) {
    let f0 = vp8_filter[0] as i32;
    let f1 = vp8_filter[1] as i32;
    let half_weight = VP8_FILTER_WEIGHT / 2;

    let req_src = (height + 1) * src_stride;
    let req_dst = (height.saturating_sub(1)) * dst_pitch + width;
    if src.len() < req_src || dst.len() < req_dst {
        return;
    }
    let src = &src[..req_src];
    let dst = &mut dst[..req_dst];

    for i in 0..height {
        let r0 = &src[i * src_stride..i * src_stride + width];
        let r1 = &src[(i + 1) * src_stride..(i + 1) * src_stride + width];
        let dst_row = &mut dst[i * dst_pitch..i * dst_pitch + width];

        for ((dst_pix, &p0), &p1) in dst_row.iter_mut().zip(r0.iter()).zip(r1.iter()) {
            let temp = p0 as i32 * f0 + p1 as i32 * f1 + half_weight;
            *dst_pix = (temp >> VP8_FILTER_SHIFT) as u8;
        }
    }
}

pub(crate) fn filter_block2d_bil_safe(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    h_filter: &[i16; 2],
    v_filter: &[i16; 2],
) {
    let mut f_data = [0u16; 272];
    let f_data_width = width;

    filter_block2d_bil_first_pass_safe(
        src,
        src_stride,
        &mut f_data,
        f_data_width,
        height + 1,
        width,
        h_filter,
    );

    filter_block2d_bil_second_pass_safe(
        &f_data,
        f_data_width,
        dst,
        dst_pitch,
        height,
        width,
        v_filter,
    );
}

pub(crate) fn filter_block2d_sixtap_safe(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    h_filter: &[i16; 6],
    v_filter: &[i16; 6],
) {
    let mut f_data = [0i32; 336];
    let f_data_len = (height + 5) * width;
    let f_data_slice = &mut f_data[..f_data_len];

    filter_block2d_first_pass_safe(src, src_stride, height + 5, width, h_filter, f_data_slice);
    filter_block2d_second_pass_safe(f_data_slice, width, dst, dst_pitch, height, width, v_filter);
}
