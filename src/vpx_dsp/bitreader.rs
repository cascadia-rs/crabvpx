use std::ffi::c_void;
unsafe extern "Rust" {
    static vpx_norm: [u8; 256];
}
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
pub type VpxDecryptCb = Option<unsafe fn(*mut c_void, *const u8, *mut u8, i32) -> ()>;
pub type BdValue = SizeT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxReader {
    pub value: BdValue,
    pub range: u32,
    pub count: i32,
    pub buffer_end: *const u8,
    pub buffer: *const u8,
    pub decrypt_cb: VpxDecryptCb,
    pub decrypt_state: *mut c_void,
    pub clear_buffer: [u8; 9],
}
pub const CHAR_BIT: i32 = 8 as i32;
pub const BD_VALUE_SIZE: i32 = ::core::mem::size_of::<BdValue>() as i32 * CHAR_BIT;
pub const LOTS_OF_BITS: i32 = 0x40000000 as i32;
#[inline]
unsafe fn vpx_read(mut r: *mut VpxReader, mut prob: i32) -> i32 {
    unsafe {
        let mut bit: u32 = 0 as u32;
        let mut value: BdValue = 0;
        let mut bigsplit: BdValue = 0;
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
        bigsplit = (split as BdValue) << (BD_VALUE_SIZE - CHAR_BIT);
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
unsafe fn vpx_read_bit(mut r: *mut VpxReader) -> i32 {
    unsafe { vpx_read(r, 128 as i32) }
}
#[inline]
unsafe fn bswap64(mut x: u64) -> u64 {
    x.swap_bytes()
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_reader_init(
    mut r: *mut VpxReader,
    mut buffer: *const u8,
    mut size: SizeT,
    mut decrypt_cb: VpxDecryptCb,
    mut decrypt_state: *mut c_void,
) -> i32 {
    unsafe {
        if size != 0 && buffer.is_null() {
            1 as i32
        } else {
            (*r).buffer_end = buffer.add(size);
            (*r).buffer = buffer;
            (*r).value = 0 as BdValue;
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
pub unsafe fn vpx_reader_fill(mut r: *mut VpxReader) {
    unsafe {
        let buffer_end: *const u8 = (*r).buffer_end;
        let mut buffer: *const u8 = (*r).buffer;
        let mut buffer_start: *const u8 = buffer;
        let mut value: BdValue = (*r).value;
        let mut count: i32 = (*r).count;
        let bytes_left: SizeT = buffer_end.offset_from(buffer) as SizeT;
        let bits_left: SizeT = bytes_left.wrapping_mul(CHAR_BIT as SizeT);
        let mut shift: i32 = BD_VALUE_SIZE - CHAR_BIT - (count + CHAR_BIT);
        if (*r).decrypt_cb.is_some() {
            let mut n: SizeT = if (::core::mem::size_of::<[u8; 9]>() as usize) < bytes_left {
                ::core::mem::size_of::<[u8; 9]>() as SizeT
            } else {
                bytes_left
            };
            (*r).decrypt_cb.expect("non-null function pointer")(
                (*r).decrypt_state,
                buffer as *const u8,
                &raw mut (*r).clear_buffer as *mut u8,
                n as i32,
            );
            buffer = &raw mut (*r).clear_buffer as *mut u8;
            buffer_start = &raw mut (*r).clear_buffer as *mut u8;
        }
        if bits_left > BD_VALUE_SIZE as SizeT {
            let bits: i32 = (shift as u32 & 0xfffffff8 as u32).wrapping_add(CHAR_BIT as u32) as i32;
            let mut nv: BdValue = 0;
            let mut big_endian_values: BdValue = 0;
            core::ptr::copy_nonoverlapping(
                buffer as *const c_void as *const u8,
                &raw mut big_endian_values as *mut c_void as *mut u8,
                ::core::mem::size_of::<BdValue>() as SizeT,
            );
            big_endian_values = bswap64(big_endian_values as u64) as BdValue;
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
                    value |= (*fresh0 as BdValue) << shift;
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
pub unsafe fn vpx_reader_find_end(mut r: *mut VpxReader) -> *const u8 {
    unsafe {
        while (*r).count > CHAR_BIT && (*r).count < BD_VALUE_SIZE {
            (*r).count -= CHAR_BIT;
            (*r).buffer = (*r).buffer.offset(-1);
        }
        (*r).buffer
    }
}
