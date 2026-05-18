use std::ffi::c_void;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type vpx_rb_error_handler = Option<unsafe fn(*mut c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: size_t,
    pub error_handler_data: *mut c_void,
    pub error_handler: vpx_rb_error_handler,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_bytes_read(mut rb: *mut vpx_read_bit_buffer) -> size_t {
    unsafe { (*rb).bit_offset.wrapping_add(7 as size_t) >> 3 as i32 }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_read_bit(mut rb: *mut vpx_read_bit_buffer) -> i32 {
    unsafe {
        let off: size_t = (*rb).bit_offset;
        let p: size_t = off >> 3 as i32;
        let q: i32 = 7 as i32 - (off & 0x7 as size_t) as i32;
        if (*rb).bit_buffer.add(p) < (*rb).bit_buffer_end {
            let bit: i32 = *(*rb).bit_buffer.add(p) as i32 >> q & 1 as i32;
            (*rb).bit_offset = off.wrapping_add(1 as size_t);
            bit
        } else {
            if (*rb).error_handler.is_some() {
                (*rb).error_handler.expect("non-null function pointer")((*rb).error_handler_data);
            }
            0 as i32
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_read_literal(mut rb: *mut vpx_read_bit_buffer, mut bits: i32) -> i32 {
    unsafe {
        let mut value: i32 = 0 as i32;
        let mut bit: i32 = 0;
        bit = bits - 1 as i32;
        while bit >= 0 as i32 {
            value |= vpx_rb_read_bit(rb) << bit;
            bit -= 1;
        }
        value
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_read_signed_literal(mut rb: *mut vpx_read_bit_buffer, mut bits: i32) -> i32 {
    unsafe {
        let value: i32 = vpx_rb_read_literal(rb, bits) as i32;
        if vpx_rb_read_bit(rb) != 0 {
            -value
        } else {
            value
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_read_inv_signed_literal(
    mut rb: *mut vpx_read_bit_buffer,
    mut bits: i32,
) -> i32 {
    unsafe { vpx_rb_read_signed_literal(rb, bits) }
}
