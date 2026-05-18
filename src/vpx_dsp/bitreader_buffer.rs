pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type vpx_rb_error_handler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: size_t,
    pub error_handler_data: *mut ::core::ffi::c_void,
    pub error_handler: vpx_rb_error_handler,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_rb_bytes_read(mut rb: *mut vpx_read_bit_buffer) -> size_t {
    unsafe { (*rb).bit_offset.wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_int }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_rb_read_bit(mut rb: *mut vpx_read_bit_buffer) -> ::core::ffi::c_int {
    unsafe {
        let off: size_t = (*rb).bit_offset;
        let p: size_t = off >> 3 as ::core::ffi::c_int;
        let q: ::core::ffi::c_int =
            7 as ::core::ffi::c_int - (off & 0x7 as size_t) as ::core::ffi::c_int;
        if (*rb).bit_buffer.add(p) < (*rb).bit_buffer_end {
            let bit: ::core::ffi::c_int =
                *(*rb).bit_buffer.add(p) as ::core::ffi::c_int >> q & 1 as ::core::ffi::c_int;
            (*rb).bit_offset = off.wrapping_add(1 as size_t);
            bit
        } else {
            if (*rb).error_handler.is_some() {
                (*rb).error_handler.expect("non-null function pointer")((*rb).error_handler_data);
            }
            0 as ::core::ffi::c_int
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_rb_read_literal(
    mut rb: *mut vpx_read_bit_buffer,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bit: ::core::ffi::c_int = 0;
        bit = bits - 1 as ::core::ffi::c_int;
        while bit >= 0 as ::core::ffi::c_int {
            value |= vpx_rb_read_bit(rb) << bit;
            bit -= 1;
        }
        value
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_rb_read_signed_literal(
    mut rb: *mut vpx_read_bit_buffer,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let value: ::core::ffi::c_int = vpx_rb_read_literal(rb, bits) as ::core::ffi::c_int;
        if vpx_rb_read_bit(rb) != 0 {
            -value
        } else {
            value
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_rb_read_inv_signed_literal(
    mut rb: *mut vpx_read_bit_buffer,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { vpx_rb_read_signed_literal(rb, bits) }
}
