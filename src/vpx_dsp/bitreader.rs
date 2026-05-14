extern "C" {
    static vpx_norm: [uint8_t; 256];
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type vpx_decrypt_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type BD_VALUE = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_reader {
    pub value: BD_VALUE,
    pub range: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_int,
    pub buffer_end: *const uint8_t,
    pub buffer: *const uint8_t,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub clear_buffer: [uint8_t; 9],
}
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vpx_read(
    mut r: *mut vpx_reader,
    mut prob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bit: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut value: BD_VALUE = 0;
    let mut bigsplit: BD_VALUE = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut range: ::core::ffi::c_uint = 0;
    let mut split: ::core::ffi::c_uint = (*r)
        .range
        .wrapping_mul(prob as ::core::ffi::c_uint)
        .wrapping_add((256 as ::core::ffi::c_int - prob) as ::core::ffi::c_uint)
        >> CHAR_BIT;
    if (*r).count < 0 as ::core::ffi::c_int {
        vpx_reader_fill(r);
    }
    value = (*r).value;
    count = (*r).count;
    bigsplit = (split as BD_VALUE) << BD_VALUE_SIZE - CHAR_BIT;
    range = split;
    if value >= bigsplit {
        range = (*r).range.wrapping_sub(split);
        value = value.wrapping_sub(bigsplit);
        bit = 1 as ::core::ffi::c_uint;
    }
    let shift: ::core::ffi::c_uchar = vpx_norm[range as ::core::ffi::c_uchar as usize];
    range <<= shift as ::core::ffi::c_int;
    value <<= shift as ::core::ffi::c_int;
    count -= shift as ::core::ffi::c_int;
    (*r).value = value;
    (*r).count = count;
    (*r).range = range;
    return bit as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn vpx_read_bit(mut r: *mut vpx_reader) -> ::core::ffi::c_int {
    return vpx_read(r, 128 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
#[no_mangle]
pub unsafe extern "C" fn vpx_reader_init(
    mut r: *mut vpx_reader,
    mut buffer: *const uint8_t,
    mut size: size_t,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if size != 0 && buffer.is_null() {
        return 1 as ::core::ffi::c_int;
    } else {
        (*r).buffer_end = buffer.offset(size as isize);
        (*r).buffer = buffer;
        (*r).value = 0 as BD_VALUE;
        (*r).count = -(8 as ::core::ffi::c_int);
        (*r).range = 255 as ::core::ffi::c_uint;
        (*r).decrypt_cb = decrypt_cb;
        (*r).decrypt_state = decrypt_state;
        vpx_reader_fill(r);
        return (vpx_read_bit(r) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn vpx_reader_fill(mut r: *mut vpx_reader) {
    let buffer_end: *const uint8_t = (*r).buffer_end;
    let mut buffer: *const uint8_t = (*r).buffer;
    let mut buffer_start: *const uint8_t = buffer;
    let mut value: BD_VALUE = (*r).value;
    let mut count: ::core::ffi::c_int = (*r).count;
    let bytes_left: size_t = buffer_end.offset_from(buffer) as ::core::ffi::c_long as size_t;
    let bits_left: size_t = bytes_left.wrapping_mul(CHAR_BIT as size_t);
    let mut shift: ::core::ffi::c_int = BD_VALUE_SIZE - CHAR_BIT - (count + CHAR_BIT);
    if (*r).decrypt_cb.is_some() {
        let mut n: size_t = if (::core::mem::size_of::<[uint8_t; 9]>() as usize) < bytes_left {
            ::core::mem::size_of::<[uint8_t; 9]>() as size_t
        } else {
            bytes_left
        };
        (*r).decrypt_cb.expect("non-null function pointer")(
            (*r).decrypt_state,
            buffer as *const ::core::ffi::c_uchar,
            &raw mut (*r).clear_buffer as *mut ::core::ffi::c_uchar,
            n as ::core::ffi::c_int,
        );
        buffer = &raw mut (*r).clear_buffer as *mut uint8_t;
        buffer_start = &raw mut (*r).clear_buffer as *mut uint8_t;
    }
    if bits_left > BD_VALUE_SIZE as size_t {
        let bits: ::core::ffi::c_int =
            (shift as ::core::ffi::c_uint & 0xfffffff8 as ::core::ffi::c_uint)
                .wrapping_add(CHAR_BIT as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut nv: BD_VALUE = 0;
        let mut big_endian_values: BD_VALUE = 0;
        memcpy(
            &raw mut big_endian_values as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            ::core::mem::size_of::<BD_VALUE>() as size_t,
        );
        big_endian_values = BSwap64(big_endian_values as uint64_t) as BD_VALUE;
        nv = big_endian_values >> BD_VALUE_SIZE - bits;
        count += bits;
        buffer = buffer.offset((bits >> 3 as ::core::ffi::c_int) as isize);
        value = (*r).value | nv << (shift & 0x7 as ::core::ffi::c_int);
    } else {
        let bits_over: ::core::ffi::c_int = shift + CHAR_BIT - bits_left as ::core::ffi::c_int;
        let mut loop_end: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if bits_over >= 0 as ::core::ffi::c_int {
            count += LOTS_OF_BITS;
            loop_end = bits_over;
        }
        if bits_over < 0 as ::core::ffi::c_int || bits_left != 0 {
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
        .offset(buffer.offset_from(buffer_start) as ::core::ffi::c_long as isize);
    (*r).value = value;
    (*r).count = count;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_reader_find_end(mut r: *mut vpx_reader) -> *const uint8_t {
    while (*r).count > CHAR_BIT && (*r).count < BD_VALUE_SIZE {
        (*r).count -= CHAR_BIT;
        (*r).buffer = (*r).buffer.offset(-1);
    }
    return (*r).buffer;
}
