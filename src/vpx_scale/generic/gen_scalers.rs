unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_horizontal_line_5_4_scale_c(
    mut source: *const ::core::ffi::c_uchar,
    mut source_width: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    _dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        let mut e: ::core::ffi::c_uint = 0;
        let mut des: *mut ::core::ffi::c_uchar = dest;
        let mut src: *const ::core::ffi::c_uchar = source;
        i = 0 as ::core::ffi::c_uint;
        while i < source_width {
            a = *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b = *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            c = *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            d = *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            e = *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            *des.offset(0 as ::core::ffi::c_int as isize) = a as ::core::ffi::c_uchar;
            *des.offset(1 as ::core::ffi::c_int as isize) =
                (b.wrapping_mul(192 as ::core::ffi::c_uint)
                    .wrapping_add(c.wrapping_mul(64 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset(2 as ::core::ffi::c_int as isize) =
                (c.wrapping_mul(128 as ::core::ffi::c_uint)
                    .wrapping_add(d.wrapping_mul(128 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset(3 as ::core::ffi::c_int as isize) =
                (d.wrapping_mul(64 as ::core::ffi::c_uint)
                    .wrapping_add(e.wrapping_mul(192 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            src = src.offset(5 as ::core::ffi::c_int as isize);
            des = des.offset(4 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(5 as ::core::ffi::c_uint);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_vertical_band_5_4_scale_c(
    mut source: *mut ::core::ffi::c_uchar,
    mut src_pitch: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_pitch: ::core::ffi::c_uint,
    mut dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        let mut e: ::core::ffi::c_uint = 0;
        let mut des: *mut ::core::ffi::c_uchar = dest;
        let mut src: *mut ::core::ffi::c_uchar = source;
        i = 0 as ::core::ffi::c_uint;
        while i < dest_width {
            a = *src.offset((0 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            b = *src.offset((1 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            c = *src.offset((2 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            d = *src.offset((3 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            e = *src.offset((4 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            *des.offset((0 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                a as ::core::ffi::c_uchar;
            *des.offset((1 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                (b.wrapping_mul(192 as ::core::ffi::c_uint)
                    .wrapping_add(c.wrapping_mul(64 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset((2 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                (c.wrapping_mul(128 as ::core::ffi::c_uint)
                    .wrapping_add(d.wrapping_mul(128 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset((3 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                (d.wrapping_mul(64 as ::core::ffi::c_uint)
                    .wrapping_add(e.wrapping_mul(192 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            src = src.offset(1);
            des = des.offset(1);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_horizontal_line_5_3_scale_c(
    mut source: *const ::core::ffi::c_uchar,
    mut source_width: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    _dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        let mut e: ::core::ffi::c_uint = 0;
        let mut des: *mut ::core::ffi::c_uchar = dest;
        let mut src: *const ::core::ffi::c_uchar = source;
        i = 0 as ::core::ffi::c_uint;
        while i < source_width {
            a = *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b = *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            c = *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            d = *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            e = *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            *des.offset(0 as ::core::ffi::c_int as isize) = a as ::core::ffi::c_uchar;
            *des.offset(1 as ::core::ffi::c_int as isize) =
                (b.wrapping_mul(85 as ::core::ffi::c_uint)
                    .wrapping_add(c.wrapping_mul(171 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset(2 as ::core::ffi::c_int as isize) =
                (d.wrapping_mul(171 as ::core::ffi::c_uint)
                    .wrapping_add(e.wrapping_mul(85 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            src = src.offset(5 as ::core::ffi::c_int as isize);
            des = des.offset(3 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(5 as ::core::ffi::c_uint);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_vertical_band_5_3_scale_c(
    mut source: *mut ::core::ffi::c_uchar,
    mut src_pitch: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_pitch: ::core::ffi::c_uint,
    mut dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        let mut e: ::core::ffi::c_uint = 0;
        let mut des: *mut ::core::ffi::c_uchar = dest;
        let mut src: *mut ::core::ffi::c_uchar = source;
        i = 0 as ::core::ffi::c_uint;
        while i < dest_width {
            a = *src.offset((0 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            b = *src.offset((1 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            c = *src.offset((2 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            d = *src.offset((3 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            e = *src.offset((4 as ::core::ffi::c_uint).wrapping_mul(src_pitch) as isize)
                as ::core::ffi::c_uint;
            *des.offset((0 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                a as ::core::ffi::c_uchar;
            *des.offset((1 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                (b.wrapping_mul(85 as ::core::ffi::c_uint)
                    .wrapping_add(c.wrapping_mul(171 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *des.offset((2 as ::core::ffi::c_uint).wrapping_mul(dest_pitch) as isize) =
                (d.wrapping_mul(171 as ::core::ffi::c_uint)
                    .wrapping_add(e.wrapping_mul(85 as ::core::ffi::c_uint))
                    .wrapping_add(128 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            src = src.offset(1);
            des = des.offset(1);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_horizontal_line_2_1_scale_c(
    mut source: *const ::core::ffi::c_uchar,
    mut source_width: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    _dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut a: ::core::ffi::c_uint = 0;
        let mut des: *mut ::core::ffi::c_uchar = dest;
        let mut src: *const ::core::ffi::c_uchar = source;
        i = 0 as ::core::ffi::c_uint;
        while i < source_width {
            a = *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            *des.offset(0 as ::core::ffi::c_int as isize) = a as ::core::ffi::c_uchar;
            src = src.offset(2 as ::core::ffi::c_int as isize);
            des = des.offset(1 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(2 as ::core::ffi::c_uint);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_vertical_band_2_1_scale_c(
    mut source: *mut ::core::ffi::c_uchar,
    _src_pitch: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    _dest_pitch: ::core::ffi::c_uint,
    mut dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        memcpy(
            dest as *mut ::core::ffi::c_void,
            source as *const ::core::ffi::c_void,
            dest_width as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_vertical_band_2_1_scale_i_c(
    mut source: *mut ::core::ffi::c_uchar,
    mut src_pitch: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    _dest_pitch: ::core::ffi::c_uint,
    mut dest_width: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut temp: ::core::ffi::c_int = 0;
        let mut width: ::core::ffi::c_int = dest_width as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < width {
            temp = 8 as ::core::ffi::c_int;
            temp += *source.offset((i - src_pitch as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                * 3 as ::core::ffi::c_int;
            temp += *source.offset(i as isize) as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
            temp += *source.offset((i as ::core::ffi::c_uint).wrapping_add(src_pitch) as isize)
                as ::core::ffi::c_int
                * 3 as ::core::ffi::c_int;
            temp >>= 4 as ::core::ffi::c_int;
            *dest.offset(i as isize) = temp as ::core::ffi::c_uchar;
            i += 1;
        }
    }
}
