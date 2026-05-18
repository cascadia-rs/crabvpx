use std::ffi::c_void;
unsafe extern "Rust" {
    static vpx_norm: [uint8_t; 256];
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type vpx_decrypt_cb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type BD_VALUE = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_reader {
    pub value: BD_VALUE,
    pub range: u32,
    pub count: i32,
    pub buffer_end: *const uint8_t,
    pub buffer: *const uint8_t,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
    pub clear_buffer: [uint8_t; 9],
}
pub const CHAR_BIT: i32 = 8 as i32;
pub const BD_VALUE_SIZE: i32 = ::core::mem::size_of::<BD_VALUE>() as i32 * CHAR_BIT;
pub const LOTS_OF_BITS: i32 = 0x40000000 as i32;
#[inline]
unsafe fn vpx_read(mut r: *mut vpx_reader, mut prob: i32) -> i32 {
    unsafe {
        let mut bit: u32 = 0 as u32;
        let mut value: BD_VALUE = 0;
        let mut bigsplit: BD_VALUE = 0;
        let mut count: i32 = 0;
        let mut range: u32 = 0;
        let mut split: u32 = (*r)
            .range
            .wrapping_mul(prob as u32)
            .wrapping_add((256 as i32 - prob) as u32)
            >> CHAR_BIT;
        if (*r).count < 0 as i32 {
            vpx_reader_fill(r);
        }
        value = (*r).value;
        count = (*r).count;
        bigsplit = (split as BD_VALUE) << (BD_VALUE_SIZE - CHAR_BIT);
        range = split;
        if value >= bigsplit {
            range = (*r).range.wrapping_sub(split);
            value = value.wrapping_sub(bigsplit);
            bit = 1 as u32;
        }
        let shift: u8 = vpx_norm[range as usize];
        range <<= shift as i32;
        value <<= shift as i32;
        count -= shift as i32;
        (*r).value = value;
        (*r).count = count;
        (*r).range = range;
        bit as i32
    }
}
#[inline]
unsafe fn vpx_read_bit(mut r: *mut vpx_reader) -> i32 {
    unsafe { vpx_read(r, 128 as i32) }
}
#[inline]
unsafe fn BSwap64(mut x: uint64_t) -> uint64_t {
    x.swap_bytes()
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_reader_init(
    mut r: *mut vpx_reader,
    mut buffer: *const uint8_t,
    mut size: size_t,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut c_void,
) -> i32 {
    unsafe {
        if size != 0 && buffer.is_null() {
            1 as i32
        } else {
            (*r).buffer_end = buffer.add(size);
            (*r).buffer = buffer;
            (*r).value = 0 as BD_VALUE;
            (*r).count = -(8 as i32);
            (*r).range = 255 as u32;
            (*r).decrypt_cb = decrypt_cb;
            (*r).decrypt_state = decrypt_state;
            vpx_reader_fill(r);
            (vpx_read_bit(r) != 0 as i32) as i32
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_reader_fill(mut r: *mut vpx_reader) {
    unsafe {
        let buffer_end: *const uint8_t = (*r).buffer_end;
        let mut buffer: *const uint8_t = (*r).buffer;
        let mut buffer_start: *const uint8_t = buffer;
        let mut value: BD_VALUE = (*r).value;
        let mut count: i32 = (*r).count;
        let bytes_left: size_t = buffer_end.offset_from(buffer) as size_t;
        let bits_left: size_t = bytes_left.wrapping_mul(CHAR_BIT as size_t);
        let mut shift: i32 = BD_VALUE_SIZE - CHAR_BIT - (count + CHAR_BIT);
        if (*r).decrypt_cb.is_some() {
            let mut n: size_t = if (::core::mem::size_of::<[uint8_t; 9]>() as usize) < bytes_left {
                ::core::mem::size_of::<[uint8_t; 9]>() as size_t
            } else {
                bytes_left
            };
            (*r).decrypt_cb.expect("non-null function pointer")(
                (*r).decrypt_state,
                buffer as *const u8,
                &raw mut (*r).clear_buffer as *mut u8,
                n as i32,
            );
            buffer = &raw mut (*r).clear_buffer as *mut uint8_t;
            buffer_start = &raw mut (*r).clear_buffer as *mut uint8_t;
        }
        if bits_left > BD_VALUE_SIZE as size_t {
            let bits: i32 = (shift as u32 & 0xfffffff8 as u32).wrapping_add(CHAR_BIT as u32) as i32;
            let mut nv: BD_VALUE = 0;
            let mut big_endian_values: BD_VALUE = 0;
            memcpy(
                &raw mut big_endian_values as *mut c_void,
                buffer as *const c_void,
                ::core::mem::size_of::<BD_VALUE>() as size_t,
            );
            big_endian_values = BSwap64(big_endian_values as uint64_t) as BD_VALUE;
            nv = big_endian_values >> (BD_VALUE_SIZE - bits);
            count += bits;
            buffer = buffer.offset((bits >> 3 as i32) as isize);
            value = (*r).value | nv << (shift & 0x7 as i32);
        } else {
            let bits_over: i32 = shift + CHAR_BIT - bits_left as i32;
            let mut loop_end: i32 = 0 as i32;
            if bits_over >= 0 as i32 {
                count += LOTS_OF_BITS;
                loop_end = bits_over;
            }
            if bits_over < 0 as i32 || bits_left != 0 {
                while shift >= loop_end {
                    count += CHAR_BIT;
                    let fresh0 = buffer;
                    buffer = buffer.offset(1);
                    value |= (*fresh0 as BD_VALUE) << shift;
                    shift -= CHAR_BIT;
                }
            }
        }
        (*r).buffer = (*r)
            .buffer
            .offset(buffer.offset_from(buffer_start) as isize);
        (*r).value = value;
        (*r).count = count;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_reader_find_end(mut r: *mut vpx_reader) -> *const uint8_t {
    unsafe {
        while (*r).count > CHAR_BIT && (*r).count < BD_VALUE_SIZE {
            (*r).count -= CHAR_BIT;
            (*r).buffer = (*r).buffer.offset(-1);
        }
        (*r).buffer
    }
}
