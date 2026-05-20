use std::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;

pub type size_t = usize;

pub const DEFAULT_ALIGNMENT: usize = 32;

pub struct AlignedBox {
    data_ptr: NonNull<u8>,
    layout: Layout,
    size: usize,
}

impl AlignedBox {
    pub fn new(align: usize, size: usize) -> Option<Self> {
        let mut align = align;
        if align == 0 {
            align = DEFAULT_ALIGNMENT;
        }
        if !align.is_power_of_two() {
            align = align.next_power_of_two();
        }
        
        let layout = Layout::from_size_align(size, align).ok()?;
        let raw_ptr = unsafe { alloc(layout) };
        let data_ptr = NonNull::new(raw_ptr)?;
        
        Some(Self { data_ptr, layout, size })
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.data_ptr.as_ptr()
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data_ptr.as_ptr(), self.size) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data_ptr.as_ptr(), self.size) }
    }
}

impl Drop for AlignedBox {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data_ptr.as_ptr(), self.layout);
        }
    }
}
