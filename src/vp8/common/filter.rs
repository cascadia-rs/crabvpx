pub const VP8_FILTER_WEIGHT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const VP8_FILTER_SHIFT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

#[unsafe(no_mangle)]
pub static vp8_bilinear_filters: [[::core::ffi::c_short; 2]; 8] = [
    [
        128 as ::core::ffi::c_int as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        112 as ::core::ffi::c_int as ::core::ffi::c_short,
        16 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        96 as ::core::ffi::c_int as ::core::ffi::c_short,
        32 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        80 as ::core::ffi::c_int as ::core::ffi::c_short,
        48 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        64 as ::core::ffi::c_int as ::core::ffi::c_short,
        64 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        48 as ::core::ffi::c_int as ::core::ffi::c_short,
        80 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        32 as ::core::ffi::c_int as ::core::ffi::c_short,
        96 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        16 as ::core::ffi::c_int as ::core::ffi::c_short,
        112 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
];

#[unsafe(no_mangle)]
pub static vp8_sub_pel_filters: [[::core::ffi::c_short; 6]; 8] = [
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        128 as ::core::ffi::c_int as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(6 as ::core::ffi::c_int) as ::core::ffi::c_short,
        123 as ::core::ffi::c_int as ::core::ffi::c_short,
        12 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        2 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(11 as ::core::ffi::c_int) as ::core::ffi::c_short,
        108 as ::core::ffi::c_int as ::core::ffi::c_short,
        36 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(8 as ::core::ffi::c_int) as ::core::ffi::c_short,
        1 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(9 as ::core::ffi::c_int) as ::core::ffi::c_short,
        93 as ::core::ffi::c_int as ::core::ffi::c_short,
        50 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(6 as ::core::ffi::c_int) as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        3 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(16 as ::core::ffi::c_int) as ::core::ffi::c_short,
        77 as ::core::ffi::c_int as ::core::ffi::c_short,
        77 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(16 as ::core::ffi::c_int) as ::core::ffi::c_short,
        3 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(6 as ::core::ffi::c_int) as ::core::ffi::c_short,
        50 as ::core::ffi::c_int as ::core::ffi::c_short,
        93 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(9 as ::core::ffi::c_int) as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        1 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(8 as ::core::ffi::c_int) as ::core::ffi::c_short,
        36 as ::core::ffi::c_int as ::core::ffi::c_short,
        108 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(11 as ::core::ffi::c_int) as ::core::ffi::c_short,
        2 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
    [
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_short,
        12 as ::core::ffi::c_int as ::core::ffi::c_short,
        123 as ::core::ffi::c_int as ::core::ffi::c_short,
        -(6 as ::core::ffi::c_int) as ::core::ffi::c_short,
        0 as ::core::ffi::c_int as ::core::ffi::c_short,
    ],
];

// Safe Helper Functions

unsafe fn get_src_slice<'a>(
    src_ptr: *const u8,
    stride: usize,
    output_height: usize,
    output_width: usize,
) -> &'a [u8] {
    let start_ptr = src_ptr.offset(-((2 * stride + 2) as isize));
    let len = (output_height - 1) * stride + output_width + 5;
    core::slice::from_raw_parts(start_ptr, len)
}

unsafe fn get_bil_src_slice<'a>(
    src_ptr: *const u8,
    stride: usize,
    width: usize,
    height: usize,
) -> &'a [u8] {
    let len = height * stride + width + 1;
    core::slice::from_raw_parts(src_ptr, len)
}

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
        let r0 = &src[(i + 0) * src_stride..(i + 0) * src_stride + output_width];
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
            let mut temp = p0 * f0
                + p1 * f1
                + p2 * f2
                + p3 * f3
                + p4 * f4
                + p5 * f5
                + half_weight;

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
    filter_block2d_first_pass_safe(
        src,
        src_stride,
        9,
        4,
        h_filter,
        &mut f_data,
    );
    filter_block2d_second_pass_safe(
        &f_data,
        4,
        dst,
        dst_pitch,
        4,
        4,
        v_filter,
    );
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

fn filter_block2d_bil_safe(
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

// FFI Wrappers

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict4x4_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_src_slice(src_ptr, stride, 9, 4);

        let dst_len = 3 * dst_stride + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &vp8_sub_pel_filters[yoffset as usize];

        filter_block2d_safe(src_slice, stride, dst_slice, dst_stride, h_filter, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x8_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_src_slice(src_ptr, stride, 13, 8);

        let dst_len = 7 * dst_stride + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &vp8_sub_pel_filters[yoffset as usize];

        let mut f_data = [0i32; 104];

        filter_block2d_first_pass_safe(src_slice, stride, 13, 8, h_filter, &mut f_data);
        filter_block2d_second_pass_safe(&f_data, 8, dst_slice, dst_stride, 8, 8, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x4_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_src_slice(src_ptr, stride, 9, 8);

        let dst_len = 3 * dst_stride + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &vp8_sub_pel_filters[yoffset as usize];

        let mut f_data = [0i32; 72];

        filter_block2d_first_pass_safe(src_slice, stride, 9, 8, h_filter, &mut f_data);
        filter_block2d_second_pass_safe(&f_data, 8, dst_slice, dst_stride, 4, 8, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict16x16_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_src_slice(src_ptr, stride, 21, 16);

        let dst_len = 15 * dst_stride + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_sub_pel_filters[xoffset as usize];
        let v_filter = &vp8_sub_pel_filters[yoffset as usize];

        let mut f_data = [0i32; 336];

        filter_block2d_first_pass_safe(src_slice, stride, 21, 16, h_filter, &mut f_data);
        filter_block2d_second_pass_safe(&f_data, 16, dst_slice, dst_stride, 16, 16, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict4x4_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_bil_src_slice(src_ptr, stride, 4, 4);

        let dst_len = 3 * dst_stride + 4;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_bilinear_filters[xoffset as usize];
        let v_filter = &vp8_bilinear_filters[yoffset as usize];

        filter_block2d_bil_safe(src_slice, stride, dst_slice, dst_stride, 4, 4, h_filter, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x8_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_bil_src_slice(src_ptr, stride, 8, 8);

        let dst_len = 7 * dst_stride + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_bilinear_filters[xoffset as usize];
        let v_filter = &vp8_bilinear_filters[yoffset as usize];

        filter_block2d_bil_safe(src_slice, stride, dst_slice, dst_stride, 8, 8, h_filter, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x4_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_bil_src_slice(src_ptr, stride, 8, 4);

        let dst_len = 3 * dst_stride + 8;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_bilinear_filters[xoffset as usize];
        let v_filter = &vp8_bilinear_filters[yoffset as usize];

        filter_block2d_bil_safe(src_slice, stride, dst_slice, dst_stride, 8, 4, h_filter, v_filter);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict16x16_c(
    src_ptr: *mut ::core::ffi::c_uchar,
    src_pixels_per_line: ::core::ffi::c_int,
    xoffset: ::core::ffi::c_int,
    yoffset: ::core::ffi::c_int,
    dst_ptr: *mut ::core::ffi::c_uchar,
    dst_pitch: ::core::ffi::c_int,
) {
    if src_ptr.is_null() || dst_ptr.is_null() {
        return;
    }
    let stride = src_pixels_per_line as usize;
    let dst_stride = dst_pitch as usize;

    unsafe {
        let src_slice = get_bil_src_slice(src_ptr, stride, 16, 16);

        let dst_len = 15 * dst_stride + 16;
        let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, dst_len);

        let h_filter = &vp8_bilinear_filters[xoffset as usize];
        let v_filter = &vp8_bilinear_filters[yoffset as usize];

        filter_block2d_bil_safe(src_slice, stride, dst_slice, dst_stride, 16, 16, h_filter, v_filter);
    }
}
