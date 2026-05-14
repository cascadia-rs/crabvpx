pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type vpx_decrypt_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type VP8_BD_VALUE = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BOOL_DECODER {
    pub user_buffer_end: *const ::core::ffi::c_uchar,
    pub user_buffer: *const ::core::ffi::c_uchar,
    pub value: VP8_BD_VALUE,
    pub count: ::core::ffi::c_int,
    pub range: ::core::ffi::c_uint,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
}
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn vp8dx_start_decode(
    mut br: *mut BOOL_DECODER,
    mut source: *const ::core::ffi::c_uchar,
    mut source_sz: ::core::ffi::c_uint,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if source_sz != 0 && source.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    (*br).user_buffer_end = if !source.is_null() {
        source.offset(source_sz as isize)
    } else {
        source
    };
    (*br).user_buffer = source;
    (*br).value = 0 as VP8_BD_VALUE;
    (*br).count = -(8 as ::core::ffi::c_int);
    (*br).range = 255 as ::core::ffi::c_uint;
    (*br).decrypt_cb = decrypt_cb;
    (*br).decrypt_state = decrypt_state;
    vp8dx_bool_decoder_fill(br);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vp8dx_bool_decoder_fill(mut br: *mut BOOL_DECODER) {
    let mut bufptr: *const ::core::ffi::c_uchar = (*br).user_buffer;
    let mut value: VP8_BD_VALUE = (*br).value;
    let mut count: ::core::ffi::c_int = (*br).count;
    let mut shift: ::core::ffi::c_int = VP8_BD_VALUE_SIZE - CHAR_BIT - (count + CHAR_BIT);
    let mut bytes_left: size_t =
        (*br).user_buffer_end.offset_from(bufptr) as ::core::ffi::c_long as size_t;
    let mut bits_left: size_t = bytes_left.wrapping_mul(CHAR_BIT as size_t);
    let mut x: ::core::ffi::c_int = shift + CHAR_BIT - bits_left as ::core::ffi::c_int;
    let mut loop_end: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decrypted: [::core::ffi::c_uchar; 9] = [0; 9];
    if (*br).decrypt_cb.is_some() {
        let mut n: size_t =
            if (::core::mem::size_of::<[::core::ffi::c_uchar; 9]>() as usize) < bytes_left {
                ::core::mem::size_of::<[::core::ffi::c_uchar; 9]>() as size_t
            } else {
                bytes_left
            };
        (*br).decrypt_cb.expect("non-null function pointer")(
            (*br).decrypt_state,
            bufptr,
            &raw mut decrypted as *mut ::core::ffi::c_uchar,
            n as ::core::ffi::c_int,
        );
        bufptr = &raw mut decrypted as *mut ::core::ffi::c_uchar;
    }
    if x >= 0 as ::core::ffi::c_int {
        count += VP8_LOTS_OF_BITS;
        loop_end = x;
    }
    if x < 0 as ::core::ffi::c_int || bits_left != 0 {
        while shift >= loop_end {
            count += CHAR_BIT;
            value |= (*bufptr as VP8_BD_VALUE) << shift;
            bufptr = bufptr.offset(1);
            (*br).user_buffer = (*br).user_buffer.offset(1);
            shift -= CHAR_BIT;
        }
    }
    (*br).value = value;
    (*br).count = count;
}
