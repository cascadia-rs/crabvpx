use std::ffi::c_void;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type vpx_decrypt_cb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type VP8_BD_VALUE = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BOOL_DECODER {
    pub user_buffer_end: *const u8,
    pub user_buffer: *const u8,
    pub value: VP8_BD_VALUE,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut c_void,
}
pub const CHAR_BIT: i32 = 8 as i32;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<VP8_BD_VALUE>() as i32 * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: i32 = 0x40000000 as i32;
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_start_decode(
    mut br: *mut BOOL_DECODER,
    mut source: *const u8,
    mut source_sz: u32,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut c_void,
) -> i32 {
    unsafe {
        if source_sz != 0 && source.is_null() {
            return 1 as i32;
        }
        (*br).user_buffer_end = if !source.is_null() {
            source.offset(source_sz as isize)
        } else {
            source
        };
        (*br).user_buffer = source;
        (*br).value = 0 as VP8_BD_VALUE;
        (*br).count = -(8 as i32);
        (*br).range = 255 as u32;
        (*br).decrypt_cb = decrypt_cb;
        (*br).decrypt_state = decrypt_state;
        vp8dx_bool_decoder_fill(br);
        0 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8dx_bool_decoder_fill(mut br: *mut BOOL_DECODER) {
    unsafe {
        let mut bufptr: *const u8 = (*br).user_buffer;
        let mut value: VP8_BD_VALUE = (*br).value;
        let mut count: i32 = (*br).count;
        let mut shift: i32 = VP8_BD_VALUE_SIZE - CHAR_BIT - (count + CHAR_BIT);
        let mut bytes_left: size_t = (*br).user_buffer_end.offset_from(bufptr) as size_t;
        let mut bits_left: size_t = bytes_left.wrapping_mul(CHAR_BIT as size_t);
        let mut x: i32 = shift + CHAR_BIT - bits_left as i32;
        let mut loop_end: i32 = 0 as i32;
        let mut decrypted: [u8; 9] = [0; 9];
        if (*br).decrypt_cb.is_some() {
            let mut n: size_t = if (::core::mem::size_of::<[u8; 9]>() as usize) < bytes_left {
                ::core::mem::size_of::<[u8; 9]>() as size_t
            } else {
                bytes_left
            };
            (*br).decrypt_cb.expect("non-null function pointer")(
                (*br).decrypt_state,
                bufptr,
                &raw mut decrypted as *mut u8,
                n as i32,
            );
            bufptr = &raw mut decrypted as *mut u8;
        }
        if x >= 0 as i32 {
            count += VP8_LOTS_OF_BITS;
            loop_end = x;
        }
        if x < 0 as i32 || bits_left != 0 {
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
}
