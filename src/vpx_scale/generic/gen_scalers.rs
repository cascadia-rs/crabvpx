use std::ffi::c_void;
unsafe extern "Rust" {}
pub type SizeT = DarwinSizeT;
pub type DarwinSizeT = usize;
#[unsafe(no_mangle)]
pub unsafe fn vp8_horizontal_line_5_4_scale_c(
    mut source: *const u8,
    mut source_width: u32,
    mut dest: *mut u8,
    _dest_width: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = 0;
        let mut e: u32 = 0;
        let mut des: *mut u8 = dest;
        let mut src: *const u8 = source;
        i = 0 as u32;
        while i < source_width {
            a = *src.offset(0 as isize) as u32;
            b = *src.offset(1 as isize) as u32;
            c = *src.offset(2 as isize) as u32;
            d = *src.offset(3 as isize) as u32;
            e = *src.offset(4 as isize) as u32;
            *des.offset(0 as isize) = a as u8;
            *des.offset(1 as isize) = (b
                .wrapping_mul(192 as u32)
                .wrapping_add(c.wrapping_mul(64 as u32))
                .wrapping_add(128 as u32)
                >> 8 as i32) as u8;
            *des.offset(2 as isize) = (c
                .wrapping_mul(128 as u32)
                .wrapping_add(d.wrapping_mul(128 as u32))
                .wrapping_add(128 as u32)
                >> 8 as i32) as u8;
            *des.offset(3 as isize) = (d
                .wrapping_mul(64 as u32)
                .wrapping_add(e.wrapping_mul(192 as u32))
                .wrapping_add(128 as u32)
                >> 8 as i32) as u8;
            src = src.offset(5 as isize);
            des = des.offset(4 as isize);
            i = i.wrapping_add(5 as u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_vertical_band_5_4_scale_c(
    mut source: *mut u8,
    mut src_pitch: u32,
    mut dest: *mut u8,
    mut dest_pitch: u32,
    mut dest_width: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = 0;
        let mut e: u32 = 0;
        let mut des: *mut u8 = dest;
        let mut src: *mut u8 = source;
        i = 0 as u32;
        while i < dest_width {
            a = *src.offset((0 as u32).wrapping_mul(src_pitch) as isize) as u32;
            b = *src.offset((1 as u32).wrapping_mul(src_pitch) as isize) as u32;
            c = *src.offset((2 as u32).wrapping_mul(src_pitch) as isize) as u32;
            d = *src.offset((3 as u32).wrapping_mul(src_pitch) as isize) as u32;
            e = *src.offset((4 as u32).wrapping_mul(src_pitch) as isize) as u32;
            *des.offset((0 as u32).wrapping_mul(dest_pitch) as isize) = a as u8;
            *des.offset((1 as u32).wrapping_mul(dest_pitch) as isize) =
                (b.wrapping_mul(192 as u32)
                    .wrapping_add(c.wrapping_mul(64 as u32))
                    .wrapping_add(128 as u32)
                    >> 8 as i32) as u8;
            *des.offset((2 as u32).wrapping_mul(dest_pitch) as isize) =
                (c.wrapping_mul(128 as u32)
                    .wrapping_add(d.wrapping_mul(128 as u32))
                    .wrapping_add(128 as u32)
                    >> 8 as i32) as u8;
            *des.offset((3 as u32).wrapping_mul(dest_pitch) as isize) =
                (d.wrapping_mul(64 as u32)
                    .wrapping_add(e.wrapping_mul(192 as u32))
                    .wrapping_add(128 as u32)
                    >> 8 as i32) as u8;
            src = src.offset(1);
            des = des.offset(1);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_horizontal_line_5_3_scale_c(
    mut source: *const u8,
    mut source_width: u32,
    mut dest: *mut u8,
    _dest_width: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = 0;
        let mut e: u32 = 0;
        let mut des: *mut u8 = dest;
        let mut src: *const u8 = source;
        i = 0 as u32;
        while i < source_width {
            a = *src.offset(0 as isize) as u32;
            b = *src.offset(1 as isize) as u32;
            c = *src.offset(2 as isize) as u32;
            d = *src.offset(3 as isize) as u32;
            e = *src.offset(4 as isize) as u32;
            *des.offset(0 as isize) = a as u8;
            *des.offset(1 as isize) = (b
                .wrapping_mul(85 as u32)
                .wrapping_add(c.wrapping_mul(171 as u32))
                .wrapping_add(128 as u32)
                >> 8 as i32) as u8;
            *des.offset(2 as isize) = (d
                .wrapping_mul(171 as u32)
                .wrapping_add(e.wrapping_mul(85 as u32))
                .wrapping_add(128 as u32)
                >> 8 as i32) as u8;
            src = src.offset(5 as isize);
            des = des.offset(3 as isize);
            i = i.wrapping_add(5 as u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_vertical_band_5_3_scale_c(
    mut source: *mut u8,
    mut src_pitch: u32,
    mut dest: *mut u8,
    mut dest_pitch: u32,
    mut dest_width: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = 0;
        let mut e: u32 = 0;
        let mut des: *mut u8 = dest;
        let mut src: *mut u8 = source;
        i = 0 as u32;
        while i < dest_width {
            a = *src.offset((0 as u32).wrapping_mul(src_pitch) as isize) as u32;
            b = *src.offset((1 as u32).wrapping_mul(src_pitch) as isize) as u32;
            c = *src.offset((2 as u32).wrapping_mul(src_pitch) as isize) as u32;
            d = *src.offset((3 as u32).wrapping_mul(src_pitch) as isize) as u32;
            e = *src.offset((4 as u32).wrapping_mul(src_pitch) as isize) as u32;
            *des.offset((0 as u32).wrapping_mul(dest_pitch) as isize) = a as u8;
            *des.offset((1 as u32).wrapping_mul(dest_pitch) as isize) =
                (b.wrapping_mul(85 as u32)
                    .wrapping_add(c.wrapping_mul(171 as u32))
                    .wrapping_add(128 as u32)
                    >> 8 as i32) as u8;
            *des.offset((2 as u32).wrapping_mul(dest_pitch) as isize) =
                (d.wrapping_mul(171 as u32)
                    .wrapping_add(e.wrapping_mul(85 as u32))
                    .wrapping_add(128 as u32)
                    >> 8 as i32) as u8;
            src = src.offset(1);
            des = des.offset(1);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_horizontal_line_2_1_scale_c(
    mut source: *const u8,
    mut source_width: u32,
    mut dest: *mut u8,
    _dest_width: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut a: u32 = 0;
        let mut des: *mut u8 = dest;
        let mut src: *const u8 = source;
        i = 0 as u32;
        while i < source_width {
            a = *src.offset(0 as isize) as u32;
            *des.offset(0 as isize) = a as u8;
            src = src.offset(2 as isize);
            des = des.offset(1 as isize);
            i = i.wrapping_add(2 as u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_vertical_band_2_1_scale_c(
    mut source: *mut u8,
    _src_pitch: u32,
    mut dest: *mut u8,
    _dest_pitch: u32,
    mut dest_width: u32,
) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            source as *const c_void as *const u8,
            dest as *mut c_void as *mut u8,
            dest_width as SizeT,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_vertical_band_2_1_scale_i_c(
    mut source: *mut u8,
    mut src_pitch: u32,
    mut dest: *mut u8,
    _dest_pitch: u32,
    mut dest_width: u32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut temp: i32 = 0;
        let mut width: i32 = dest_width as i32;
        i = 0 as i32;
        while i < width {
            temp = 8 as i32;
            temp += *source.offset((i - src_pitch as i32) as isize) as i32 * 3 as i32;
            temp += *source.offset(i as isize) as i32 * 10 as i32;
            temp += *source.offset((i as u32).wrapping_add(src_pitch) as isize) as i32 * 3 as i32;
            temp >>= 4 as i32;
            *dest.offset(i as isize) = temp as u8;
            i += 1;
        }
    }
}
