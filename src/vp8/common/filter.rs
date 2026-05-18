pub const VP8_FILTER_WEIGHT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const VP8_FILTER_SHIFT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut vp8_bilinear_filters: [[::core::ffi::c_short; 2]; 8] = [
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
pub static mut vp8_sub_pel_filters: [[::core::ffi::c_short; 6]; 8] = [
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
unsafe extern "C" fn filter_block2d_first_pass(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut output_ptr: *mut ::core::ffi::c_int,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut pixel_step: ::core::ffi::c_uint,
    mut output_height: ::core::ffi::c_uint,
    mut output_width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < output_height {
            j = 0 as ::core::ffi::c_uint;
            while j < output_width {
                Temp = *src_ptr.offset(
                    (-(2 as ::core::ffi::c_int) * pixel_step as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    * *vp8_filter.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src_ptr.offset(
                        (-(1 as ::core::ffi::c_int) * pixel_step as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                        * *vp8_filter.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * *vp8_filter.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset(pixel_step as isize) as ::core::ffi::c_int
                        * *vp8_filter.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset((2 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        as ::core::ffi::c_int
                        * *vp8_filter.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset((3 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        as ::core::ffi::c_int
                        * *vp8_filter.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + (VP8_FILTER_WEIGHT >> 1 as ::core::ffi::c_int);
                Temp = Temp >> VP8_FILTER_SHIFT;
                if Temp < 0 as ::core::ffi::c_int {
                    Temp = 0 as ::core::ffi::c_int;
                } else if Temp > 255 as ::core::ffi::c_int {
                    Temp = 255 as ::core::ffi::c_int;
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
unsafe extern "C" fn filter_block2d_second_pass(
    mut src_ptr: *mut ::core::ffi::c_int,
    mut output_ptr: *mut ::core::ffi::c_uchar,
    mut output_pitch: ::core::ffi::c_int,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut pixel_step: ::core::ffi::c_uint,
    mut output_height: ::core::ffi::c_uint,
    mut output_width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < output_height {
            j = 0 as ::core::ffi::c_uint;
            while j < output_width {
                Temp = *src_ptr.offset(
                    (-(2 as ::core::ffi::c_int) * pixel_step as ::core::ffi::c_int) as isize,
                ) * *vp8_filter.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + *src_ptr.offset(
                        (-(1 as ::core::ffi::c_int) * pixel_step as ::core::ffi::c_int) as isize,
                    ) * *vp8_filter.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                    + *src_ptr.offset(0 as ::core::ffi::c_int as isize)
                        * *vp8_filter.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset(pixel_step as isize)
                        * *vp8_filter.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset((2 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + *src_ptr.offset((3 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + (VP8_FILTER_WEIGHT >> 1 as ::core::ffi::c_int);
                Temp = Temp >> VP8_FILTER_SHIFT;
                if Temp < 0 as ::core::ffi::c_int {
                    Temp = 0 as ::core::ffi::c_int;
                } else if Temp > 255 as ::core::ffi::c_int {
                    Temp = 255 as ::core::ffi::c_int;
                }
                *output_ptr.offset(j as isize) = Temp as ::core::ffi::c_uchar;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
            output_ptr = output_ptr.offset(output_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn filter_block2d(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut output_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut output_pitch: ::core::ffi::c_int,
    mut HFilter: *const ::core::ffi::c_short,
    mut VFilter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut FData: [::core::ffi::c_int; 36] = [0; 36];
        filter_block2d_first_pass(
            src_ptr
                .offset(-((2 as ::core::ffi::c_uint).wrapping_mul(src_pixels_per_line) as isize)),
            &raw mut FData as *mut ::core::ffi::c_int,
            src_pixels_per_line,
            1 as ::core::ffi::c_uint,
            9 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut ::core::ffi::c_int).offset(8 as ::core::ffi::c_int as isize),
            output_ptr,
            output_pitch,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict4x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch,
            HFilter,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x8_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [::core::ffi::c_int; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as ::core::ffi::c_int * src_pixels_per_line) as isize)),
            &raw mut FData as *mut ::core::ffi::c_int,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            13 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut ::core::ffi::c_int).offset(16 as ::core::ffi::c_int as isize),
            dst_ptr,
            dst_pitch,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict8x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [::core::ffi::c_int; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as ::core::ffi::c_int * src_pixels_per_line) as isize)),
            &raw mut FData as *mut ::core::ffi::c_int,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            9 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut ::core::ffi::c_int).offset(16 as ::core::ffi::c_int as isize),
            dst_ptr,
            dst_pitch,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_sixtap_predict16x16_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [::core::ffi::c_int; 504] = [0; 504];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as ::core::ffi::c_int * src_pixels_per_line) as isize)),
            &raw mut FData as *mut ::core::ffi::c_int,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            21 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut ::core::ffi::c_int).offset(32 as ::core::ffi::c_int as isize),
            dst_ptr,
            dst_pitch,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
unsafe extern "C" fn filter_block2d_bil_first_pass(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut dst_ptr: *mut ::core::ffi::c_ushort,
    mut src_stride: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            j = 0 as ::core::ffi::c_uint;
            while j < width {
                *dst_ptr.offset(j as isize) = (*src_ptr.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    * *vp8_filter.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * *vp8_filter.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + VP8_FILTER_WEIGHT / 2 as ::core::ffi::c_int
                    >> VP8_FILTER_SHIFT)
                    as ::core::ffi::c_ushort;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_stride.wrapping_sub(width) as isize);
            dst_ptr = dst_ptr.offset(width as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn filter_block2d_bil_second_pass(
    mut src_ptr: *mut ::core::ffi::c_ushort,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_uint,
    mut width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            j = 0 as ::core::ffi::c_uint;
            while j < width {
                Temp = *src_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * *vp8_filter.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src_ptr.offset(width as isize) as ::core::ffi::c_int
                        * *vp8_filter.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + VP8_FILTER_WEIGHT / 2 as ::core::ffi::c_int;
                *dst_ptr.offset(j as isize) =
                    (Temp >> VP8_FILTER_SHIFT) as ::core::ffi::c_uint as ::core::ffi::c_uchar;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            dst_ptr = dst_ptr.offset(dst_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn filter_block2d_bil(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut src_pitch: ::core::ffi::c_uint,
    mut dst_pitch: ::core::ffi::c_uint,
    mut HFilter: *const ::core::ffi::c_short,
    mut VFilter: *const ::core::ffi::c_short,
    mut Width: ::core::ffi::c_int,
    mut Height: ::core::ffi::c_int,
) {
    unsafe {
        let mut FData: [::core::ffi::c_ushort; 272] = [0; 272];
        filter_block2d_bil_first_pass(
            src_ptr,
            &raw mut FData as *mut ::core::ffi::c_ushort,
            src_pitch,
            (Height + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            Width as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_bil_second_pass(
            &raw mut FData as *mut ::core::ffi::c_ushort,
            dst_ptr,
            dst_pitch as ::core::ffi::c_int,
            Height as ::core::ffi::c_uint,
            Width as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict4x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x8_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            8 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict8x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            8 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_bilinear_predict16x16_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_int,
    mut xoffset: ::core::ffi::c_int,
    mut yoffset: ::core::ffi::c_int,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: ::core::ffi::c_int,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            16 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
        );
    }
}
