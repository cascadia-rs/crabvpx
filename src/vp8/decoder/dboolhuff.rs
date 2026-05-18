pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub use crate::vp8::common::types::*;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_start_decode(
    mut br: *mut BOOL_DECODER,
    mut source: *const ::core::ffi::c_uchar,
    mut source_sz: ::core::ffi::c_uint,
    mut decrypt_cb: vpx_decrypt_cb,
    mut decrypt_state: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { unsafe {
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_bool_decoder_fill(mut br: *mut BOOL_DECODER) { unsafe {
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
}}

pub struct SafeBoolDecoder<'a> {
    pub buffer: &'a [u8],
    pub offset: usize,
    pub value: VP8_BD_VALUE,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut core::ffi::c_void,
}

impl<'a> SafeBoolDecoder<'a> {
    pub fn new(source: &'a [u8], decrypt_cb: vpx_decrypt_cb, decrypt_state: *mut core::ffi::c_void) -> Self {
        let mut decoder = Self {
            buffer: source,
            offset: 0,
            value: 0,
            count: -8,
            range: 255,
            decrypt_cb,
            decrypt_state,
        };
        decoder.fill();
        decoder
    }

    pub fn from_bool_decoder(bc: &BOOL_DECODER) -> Self {
        let slice = unsafe {
            let len = bc.user_buffer_end.offset_from(bc.user_buffer) as usize;
            if len == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(bc.user_buffer, len)
            }
        };
        Self {
            buffer: slice,
            offset: 0,
            value: bc.value,
            count: bc.count,
            range: bc.range,
            decrypt_cb: bc.decrypt_cb,
            decrypt_state: bc.decrypt_state,
        }
    }

    pub fn update_bool_decoder(&self, bc: &mut BOOL_DECODER) {
        bc.user_buffer = unsafe { bc.user_buffer.add(self.offset) };
        bc.value = self.value;
        bc.count = self.count;
        bc.range = self.range;
    }


    pub fn fill(&mut self) {
        let mut shift = VP8_BD_VALUE_SIZE - CHAR_BIT - (self.count + CHAR_BIT);
        let bytes_left = self.buffer.len() - self.offset;
        let bits_left = bytes_left * (CHAR_BIT as usize);
        let x = shift + CHAR_BIT - (bits_left as i32);
        let mut loop_end = 0;

        let mut current_slice = &self.buffer[self.offset..];
        let mut decrypted = [0u8; 9];
        if let Some(cb) = self.decrypt_cb {
            let n = core::cmp::min(9, bytes_left);
            unsafe {
                cb(self.decrypt_state, current_slice.as_ptr(), decrypted.as_mut_ptr(), n as i32);
            }
            current_slice = &decrypted[..n];
        }

        if x >= 0 {
            self.count += VP8_LOTS_OF_BITS;
            loop_end = x;
        }

        if x < 0 || bits_left != 0 {
            let mut local_offset = 0;
            while shift >= loop_end {
                self.count += CHAR_BIT;
                if local_offset < current_slice.len() {
                    self.value |= (current_slice[local_offset] as VP8_BD_VALUE) << shift;
                    local_offset += 1;
                    self.offset += 1;
                }
                shift -= CHAR_BIT;
            }
        }
    }

    pub fn read_bool(&mut self, probability: i32) -> i32 {
        let split = 1 + (((self.range - 1) * (probability as u32)) >> 8);
        if self.count < 0 {
            self.fill();
        }
        let mut value = self.value;
        let mut count = self.count;
        let bigsplit = (split as VP8_BD_VALUE) << (VP8_BD_VALUE_SIZE - 8);
        let mut range = split;
        let mut bit = 0;
        if value >= bigsplit {
            range = self.range - split;
            value = value - bigsplit;
            bit = 1;
        }
        let shift = crate::vp8::common::entropy::vp8_norm[range as usize];
        range <<= shift;
        value <<= shift;
        count -= shift as i32;
        self.value = value;
        self.count = count;
        self.range = range;
        bit
    }

    pub fn read_literal(&mut self, bits: i32) -> i32 {
        let mut z = 0;
        for bit in (0..bits).rev() {
            z |= self.read_bool(0x80) << bit;
        }
        z
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_decode_bool(
    br: *mut BOOL_DECODER,
    probability: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let len = (*br).user_buffer_end.offset_from((*br).user_buffer) as usize;
        let slice = if len == 0 {
            &[]
        } else {
            core::slice::from_raw_parts((*br).user_buffer, len)
        };
        let mut safe_decoder = SafeBoolDecoder {
            buffer: slice,
            offset: 0,
            value: (*br).value,
            count: (*br).count,
            range: (*br).range,
            decrypt_cb: (*br).decrypt_cb,
            decrypt_state: (*br).decrypt_state,
        };
        let bit = safe_decoder.read_bool(probability);
        (*br).user_buffer = (*br).user_buffer.add(safe_decoder.offset);
        (*br).value = safe_decoder.value;
        (*br).count = safe_decoder.count;
        (*br).range = safe_decoder.range;
        bit
    }
}


