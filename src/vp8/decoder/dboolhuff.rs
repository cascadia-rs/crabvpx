pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub use crate::vp8::common::types::*;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub fn vp8dx_start_decode_safe(
    br: &mut BOOL_DECODER,
    source: &[u8],
    decrypt_cb: vpx_decrypt_cb,
    decrypt_state: *mut ::core::ffi::c_void,
) {
    br.offset = 0;
    br.value = 0 as VP8_BD_VALUE;
    br.count = -8;
    br.range = 255;
    br.decrypt_cb = decrypt_cb;
    br.decrypt_state = decrypt_state;

    let mut safe_decoder = SafeBoolDecoder::from_bool_decoder(br, source);
    safe_decoder.fill();
    safe_decoder.update_bool_decoder(br);
}

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
    pub fn new(
        source: &'a [u8],
        decrypt_cb: vpx_decrypt_cb,
        decrypt_state: *mut core::ffi::c_void,
    ) -> Self {
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

    pub fn from_bool_decoder(bc: &BOOL_DECODER, partition_slice: &'a [u8]) -> Self {
        Self {
            buffer: partition_slice,
            offset: bc.offset,
            value: bc.value,
            count: bc.count,
            range: bc.range,
            decrypt_cb: bc.decrypt_cb,
            decrypt_state: bc.decrypt_state,
        }
    }

    pub fn update_bool_decoder(&self, bc: &mut BOOL_DECODER) {
        bc.offset = self.offset;
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
        if vpx_decrypt_safe(
            self.decrypt_cb,
            self.decrypt_state,
            current_slice,
            &mut decrypted,
        ) {
            let n = core::cmp::min(9, bytes_left);
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
            value -= bigsplit;
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
