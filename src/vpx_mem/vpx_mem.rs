use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;

pub type SizeT = usize;
pub const NULL: *mut c_void = ::core::ptr::null_mut();
pub const DEFAULT_ALIGNMENT: usize = 32;

#[repr(C)]
struct AllocHeader {
    base_ptr: *mut u8,
    layout: Layout,
}

#[unsafe(no_mangle)]
pub unsafe fn vpx_memalign(mut align: SizeT, size: SizeT) -> *mut c_void {
    unsafe {
        if align == 0 {
            align = DEFAULT_ALIGNMENT;
        }
        if !align.is_power_of_two() {
            align = align.next_power_of_two();
        }

        // Ensure align is at least the alignment of AllocHeader so that
        // subtracting the header size from the aligned pointer yields a properly aligned header pointer.
        align = align.max(core::mem::align_of::<AllocHeader>());

        let header_size = core::mem::size_of::<AllocHeader>();

        // Total size must be enough to fit the original size, the header, and alignment padding.
        let total_size = size + header_size + align - 1;
        let layout = match Layout::from_size_align(total_size, core::mem::align_of::<AllocHeader>())
        {
            Ok(l) => l,
            Err(_) => return NULL,
        };

        let base_ptr = alloc(layout);
        if base_ptr.is_null() {
            return NULL;
        }

        let min_x = (base_ptr as usize) + header_size;
        let aligned_x = (min_x + align - 1) & !(align - 1);
        let x = aligned_x as *mut u8;

        let header_ptr = (x as *mut AllocHeader).offset(-1);
        core::ptr::write(header_ptr, AllocHeader { base_ptr, layout });

        x as *mut c_void
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vpx_malloc(size: SizeT) -> *mut c_void {
    unsafe { vpx_memalign(DEFAULT_ALIGNMENT, size) }
}

#[unsafe(no_mangle)]
pub unsafe fn vpx_calloc(num: SizeT, size: SizeT) -> *mut c_void {
    unsafe {
        let total = num.wrapping_mul(size);
        let ptr = vpx_malloc(total);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr as *mut u8, 0, total);
        }
        ptr
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vpx_free(memblk: *mut c_void) {
    unsafe {
        if !memblk.is_null() {
            let x = memblk as *mut u8;
            let header_ptr = (x as *mut AllocHeader).offset(-1);
            let header = core::ptr::read(header_ptr);
            dealloc(header.base_ptr, header.layout);
        }
    }
}
