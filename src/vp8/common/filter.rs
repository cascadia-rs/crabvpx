pub const VP8_FILTER_WEIGHT: i32 = 128 as i32;
pub const VP8_FILTER_SHIFT: i32 = 7 as i32;
#[unsafe(no_mangle)]
pub static mut vp8_bilinear_filters: [[i16; 2]; 8] = [
    [128 as i16, 0 as i16],
    [112 as i16, 16 as i16],
    [96 as i16, 32 as i16],
    [80 as i16, 48 as i16],
    [64 as i16, 64 as i16],
    [48 as i16, 80 as i16],
    [32 as i16, 96 as i16],
    [16 as i16, 112 as i16],
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_pel_filters: [[i16; 6]; 8] = [
    [0 as i16, 0 as i16, 128 as i16, 0 as i16, 0 as i16, 0 as i16],
    [
        0 as i16,
        -(6 as i32) as i16,
        123 as i16,
        12 as i16,
        -(1 as i32) as i16,
        0 as i16,
    ],
    [
        2 as i16,
        -(11 as i32) as i16,
        108 as i16,
        36 as i16,
        -(8 as i32) as i16,
        1 as i16,
    ],
    [
        0 as i16,
        -(9 as i32) as i16,
        93 as i16,
        50 as i16,
        -(6 as i32) as i16,
        0 as i16,
    ],
    [
        3 as i16,
        -(16 as i32) as i16,
        77 as i16,
        77 as i16,
        -(16 as i32) as i16,
        3 as i16,
    ],
    [
        0 as i16,
        -(6 as i32) as i16,
        50 as i16,
        93 as i16,
        -(9 as i32) as i16,
        0 as i16,
    ],
    [
        1 as i16,
        -(8 as i32) as i16,
        36 as i16,
        108 as i16,
        -(11 as i32) as i16,
        2 as i16,
    ],
    [
        0 as i16,
        -(1 as i32) as i16,
        12 as i16,
        123 as i16,
        -(6 as i32) as i16,
        0 as i16,
    ],
];
unsafe fn filter_block2d_first_pass(
    mut src_ptr: *mut u8,
    mut output_ptr: *mut i32,
    mut src_pixels_per_line: u32,
    mut pixel_step: u32,
    mut output_height: u32,
    mut output_width: u32,
    mut vp8_filter: *const i16,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut Temp: i32 = 0;
        i = 0 as u32;
        while i < output_height {
            j = 0 as u32;
            while j < output_width {
                Temp = *src_ptr.offset((-(2 as i32) * pixel_step as i32) as isize) as i32
                    * *vp8_filter.offset(0 as isize) as i32
                    + *src_ptr.offset((-(1 as i32) * pixel_step as i32) as isize) as i32
                        * *vp8_filter.offset(1 as isize) as i32
                    + *src_ptr.offset(0 as isize) as i32 * *vp8_filter.offset(2 as isize) as i32
                    + *src_ptr.offset(pixel_step as isize) as i32
                        * *vp8_filter.offset(3 as isize) as i32
                    + *src_ptr.offset((2 as u32).wrapping_mul(pixel_step) as isize) as i32
                        * *vp8_filter.offset(4 as isize) as i32
                    + *src_ptr.offset((3 as u32).wrapping_mul(pixel_step) as isize) as i32
                        * *vp8_filter.offset(5 as isize) as i32
                    + (VP8_FILTER_WEIGHT >> 1 as i32);
                Temp >>= VP8_FILTER_SHIFT;
                if Temp < 0 as i32 {
                    Temp = 0 as i32;
                } else if Temp > 255 as i32 {
                    Temp = 255 as i32;
                }
                *output_ptr.offset(j as isize) = Temp;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
            output_ptr = output_ptr.offset(output_width as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_second_pass(
    mut src_ptr: *mut i32,
    mut output_ptr: *mut u8,
    mut output_pitch: i32,
    mut src_pixels_per_line: u32,
    mut pixel_step: u32,
    mut output_height: u32,
    mut output_width: u32,
    mut vp8_filter: *const i16,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut Temp: i32 = 0;
        i = 0 as u32;
        while i < output_height {
            j = 0 as u32;
            while j < output_width {
                Temp = *src_ptr.offset((-(2 as i32) * pixel_step as i32) as isize)
                    * *vp8_filter.offset(0 as isize) as i32
                    + *src_ptr.offset((-(1 as i32) * pixel_step as i32) as isize)
                        * *vp8_filter.offset(1 as isize) as i32
                    + *src_ptr.offset(0 as isize) * *vp8_filter.offset(2 as isize) as i32
                    + *src_ptr.offset(pixel_step as isize) * *vp8_filter.offset(3 as isize) as i32
                    + *src_ptr.offset((2 as u32).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(4 as isize) as i32
                    + *src_ptr.offset((3 as u32).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(5 as isize) as i32
                    + (VP8_FILTER_WEIGHT >> 1 as i32);
                Temp >>= VP8_FILTER_SHIFT;
                if Temp < 0 as i32 {
                    Temp = 0 as i32;
                } else if Temp > 255 as i32 {
                    Temp = 255 as i32;
                }
                *output_ptr.offset(j as isize) = Temp as u8;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
            output_ptr = output_ptr.offset(output_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d(
    mut src_ptr: *mut u8,
    mut output_ptr: *mut u8,
    mut src_pixels_per_line: u32,
    mut output_pitch: i32,
    mut HFilter: *const i16,
    mut VFilter: *const i16,
) {
    unsafe {
        let mut FData: [i32; 36] = [0; 36];
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as u32).wrapping_mul(src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line,
            1 as u32,
            9 as u32,
            4 as u32,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(8 as isize),
            output_ptr,
            output_pitch,
            4 as u32,
            4 as u32,
            4 as u32,
            4 as u32,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict4x4_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(yoffset as isize) as *const i16;
        filter_block2d(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as u32,
            dst_pitch,
            HFilter,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict8x8_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut FData: [i32; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as u32,
            1 as u32,
            13 as u32,
            8 as u32,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(16 as isize),
            dst_ptr,
            dst_pitch,
            8 as u32,
            8 as u32,
            8 as u32,
            8 as u32,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict8x4_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut FData: [i32; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as u32,
            1 as u32,
            9 as u32,
            8 as u32,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(16 as isize),
            dst_ptr,
            dst_pitch,
            8 as u32,
            8 as u32,
            4 as u32,
            8 as u32,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict16x16_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut FData: [i32; 504] = [0; 504];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [i16; 6])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as u32,
            1 as u32,
            21 as u32,
            16 as u32,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(32 as isize),
            dst_ptr,
            dst_pitch,
            16 as u32,
            16 as u32,
            16 as u32,
            16 as u32,
            VFilter,
        );
    }
}
unsafe fn filter_block2d_bil_first_pass(
    mut src_ptr: *mut u8,
    mut dst_ptr: *mut u16,
    mut src_stride: u32,
    mut height: u32,
    mut width: u32,
    mut vp8_filter: *const i16,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        i = 0 as u32;
        while i < height {
            j = 0 as u32;
            while j < width {
                *dst_ptr.offset(j as isize) = ((*src_ptr.offset(0 as isize) as i32
                    * *vp8_filter.offset(0 as isize) as i32
                    + *src_ptr.offset(1 as isize) as i32 * *vp8_filter.offset(1 as isize) as i32
                    + VP8_FILTER_WEIGHT / 2 as i32)
                    >> VP8_FILTER_SHIFT) as u16;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_stride.wrapping_sub(width) as isize);
            dst_ptr = dst_ptr.offset(width as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_bil_second_pass(
    mut src_ptr: *mut u16,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
    mut height: u32,
    mut width: u32,
    mut vp8_filter: *const i16,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut Temp: i32 = 0;
        i = 0 as u32;
        while i < height {
            j = 0 as u32;
            while j < width {
                Temp = *src_ptr.offset(0 as isize) as i32 * *vp8_filter.offset(0 as isize) as i32
                    + *src_ptr.offset(width as isize) as i32
                        * *vp8_filter.offset(1 as isize) as i32
                    + VP8_FILTER_WEIGHT / 2 as i32;
                *dst_ptr.offset(j as isize) = (Temp >> VP8_FILTER_SHIFT) as u8;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            dst_ptr = dst_ptr.offset(dst_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_bil(
    mut src_ptr: *mut u8,
    mut dst_ptr: *mut u8,
    mut src_pitch: u32,
    mut dst_pitch: u32,
    mut HFilter: *const i16,
    mut VFilter: *const i16,
    mut Width: i32,
    mut Height: i32,
) {
    unsafe {
        let mut FData: [u16; 272] = [0; 272];
        filter_block2d_bil_first_pass(
            src_ptr,
            &raw mut FData as *mut u16,
            src_pitch,
            (Height + 1 as i32) as u32,
            Width as u32,
            HFilter,
        );
        filter_block2d_bil_second_pass(
            &raw mut FData as *mut u16,
            dst_ptr,
            dst_pitch as i32,
            Height as u32,
            Width as u32,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict4x4_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as u32,
            dst_pitch as u32,
            HFilter,
            VFilter,
            4 as i32,
            4 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict8x8_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as u32,
            dst_pitch as u32,
            HFilter,
            VFilter,
            8 as i32,
            8 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict8x4_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as u32,
            dst_pitch as u32,
            HFilter,
            VFilter,
            8 as i32,
            4 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict16x16_c(
    mut src_ptr: *mut u8,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut u8,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const i16 = ::core::ptr::null::<i16>();
        let mut VFilter: *const i16 = ::core::ptr::null::<i16>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(xoffset as isize) as *const i16;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [i16; 2])
            .offset(yoffset as isize) as *const i16;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as u32,
            dst_pitch as u32,
            HFilter,
            VFilter,
            16 as i32,
            16 as i32,
        );
    }
}
