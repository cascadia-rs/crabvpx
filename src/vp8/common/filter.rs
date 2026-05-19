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
    for i in 0..output_height {
        let src_row_start = i * src_stride;
        let out_row_start = i * output_width;
        for j in 0..output_width {
            let base = src_row_start + j;
            let val0 = src[base + 0] as i32;
            let val1 = src[base + 1] as i32;
            let val2 = src[base + 2] as i32;
            let val3 = src[base + 3] as i32;
            let val4 = src[base + 4] as i32;
            let val5 = src[base + 5] as i32;

            let mut temp = val0 * vp8_filter[0] as i32
                + val1 * vp8_filter[1] as i32
                + val2 * vp8_filter[2] as i32
                + val3 * vp8_filter[3] as i32
                + val4 * vp8_filter[4] as i32
                + val5 * vp8_filter[5] as i32
                + (VP8_FILTER_WEIGHT >> 1);

            temp >>= VP8_FILTER_SHIFT;
            let clamped = temp.clamp(0, 255);
            output[out_row_start + j] = clamped;
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
    for i in 0..output_height {
        for j in 0..output_width {
            let val0 = src[(i + 0) * src_stride + j];
            let val1 = src[(i + 1) * src_stride + j];
            let val2 = src[(i + 2) * src_stride + j];
            let val3 = src[(i + 3) * src_stride + j];
            let val4 = src[(i + 4) * src_stride + j];
            let val5 = src[(i + 5) * src_stride + j];

            let mut temp = val0 * vp8_filter[0] as i32
                + val1 * vp8_filter[1] as i32
                + val2 * vp8_filter[2] as i32
                + val3 * vp8_filter[3] as i32
                + val4 * vp8_filter[4] as i32
                + val5 * vp8_filter[5] as i32
                + (VP8_FILTER_WEIGHT >> 1);

            temp >>= VP8_FILTER_SHIFT;
            let clamped = temp.clamp(0, 255) as u8;
            dst[i * dst_pitch + j] = clamped;
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
    for i in 0..height {
        let src_row_start = i * src_stride;
        let dst_row_start = i * dst_width;
        for j in 0..width {
            let base = src_row_start + j;
            let val0 = src[base] as i32;
            let val1 = src[base + 1] as i32;

            let temp = val0 * vp8_filter[0] as i32
                + val1 * vp8_filter[1] as i32
                + (VP8_FILTER_WEIGHT / 2);

            dst[dst_row_start + j] = (temp >> VP8_FILTER_SHIFT) as u16;
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
    for i in 0..height {
        for j in 0..width {
            let base = i * src_stride + j;
            let val0 = src[base] as i32;
            let val1 = src[base + src_stride] as i32;

            let temp = val0 * vp8_filter[0] as i32
                + val1 * vp8_filter[1] as i32
                + (VP8_FILTER_WEIGHT / 2);

            dst[i * dst_pitch + j] = (temp >> VP8_FILTER_SHIFT) as u8;
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
