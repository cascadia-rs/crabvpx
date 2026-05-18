use std::ffi::c_void;
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
pub type VpxRbErrorHandler = Option<unsafe fn(*mut c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxReadBitBuffer {
    pub bit_buffer: *const u8,
    pub bit_buffer_end: *const u8,
    pub bit_offset: SizeT,
    pub error_handler_data: *mut c_void,
    pub error_handler: VpxRbErrorHandler,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_bytes_read(mut rb: *mut VpxReadBitBuffer) -> SizeT {
    unsafe { (*rb).bit_offset.wrapping_add(7 as SizeT) >> 3 as i32 }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_rb_read_bit(mut rb: *mut VpxReadBitBuffer) -> i32 {
    unsafe {
        let off: SizeT = (*rb).bit_offset;
        let p: SizeT = off >> 3 as i32;
        let q: i32 = 7 as i32 - (off & 0x7 as SizeT) as i32;
        if (*rb).bit_buffer.add(p) < (*rb).bit_buffer_end {
            let bit: i32 = *(*rb).bit_buffer.add(p) as i32 >> q & 1 as i32;
            (*rb).bit_offset = off.wrapping_add(1 as SizeT);
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
pub unsafe fn vpx_rb_read_literal(mut rb: *mut VpxReadBitBuffer, mut bits: i32) -> i32 {
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
pub unsafe fn vpx_rb_read_signed_literal(mut rb: *mut VpxReadBitBuffer, mut bits: i32) -> i32 {
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
pub unsafe fn vpx_rb_read_inv_signed_literal(mut rb: *mut VpxReadBitBuffer, mut bits: i32) -> i32 {
    unsafe { vpx_rb_read_signed_literal(rb, bits) }
}
