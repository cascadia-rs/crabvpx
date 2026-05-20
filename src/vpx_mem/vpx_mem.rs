use std::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;

pub type size_t = usize;

pub const DEFAULT_ALIGNMENT: usize = 32;

#[repr(C)]
struct AllocHeader {
    base_ptr: NonNull<u8>,
    layout: Layout,
    size: usize,
}

pub struct AlignedBox {
    data_ptr: NonNull<u8>,
    size: usize,
}

impl AlignedBox {
    pub fn new(align: usize, size: usize) -> Option<Self> {
        unsafe {
            let mut align = align;
            if align == 0 {
                align = DEFAULT_ALIGNMENT;
            }
            if !align.is_power_of_two() {
                align = align.next_power_of_two();
            }
            align = align.max(core::mem::align_of::<AllocHeader>());

            let header_size = core::mem::size_of::<AllocHeader>();
            let total_size = size + header_size + align - 1;
            let layout = Layout::from_size_align(total_size, core::mem::align_of::<AllocHeader>()).ok()?;

            let base_ptr = alloc(layout);
            let base_ptr = NonNull::new(base_ptr)?;

            let min_x = base_ptr.as_ptr() as usize + header_size;
            let aligned_x = (min_x + align - 1) & !(align - 1);
            let data_ptr = NonNull::new(aligned_x as *mut u8)?;

            let header_ptr = (data_ptr.as_ptr() as *mut AllocHeader).sub(1);
            core::ptr::write(header_ptr, AllocHeader { base_ptr, layout, size });

            Some(Self { data_ptr, size })
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.data_ptr.as_ptr()
    }

    pub fn into_raw(self) -> *mut u8 {
        let ptr = self.data_ptr.as_ptr();
        core::mem::forget(self);
        ptr
    }

    pub unsafe fn from_raw(ptr: *mut u8) -> Self {
        let header_ptr = (ptr as *mut AllocHeader).sub(1);
        let size = (*header_ptr).size;
        Self {
            data_ptr: NonNull::new(ptr).expect("AlignedBox::from_raw on null pointer"),
            size,
        }
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
            let header_ptr = (self.data_ptr.as_ptr() as *mut AllocHeader).sub(1);
            let header = core::ptr::read(header_ptr);
            dealloc(header.base_ptr.as_ptr(), header.layout);
        }
    }
}


