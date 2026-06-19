pub type size_t = usize;

pub const DEFAULT_ALIGNMENT: usize = 32;

pub struct AlignedBox {
    vec: Vec<u8>,
    offset: usize,
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

        // Allocate extra space for alignment
        let mut vec = Vec::new();
        if vec.try_reserve(size + align).is_err() {
            return None;
        }
        vec.resize(size + align, 0u8);

        let raw_ptr = vec.as_ptr() as usize;
        let aligned_ptr = (raw_ptr + align - 1) & !(align - 1);
        let offset = aligned_ptr - raw_ptr;

        Some(Self { vec, offset, size })
    }

    /// Returns the aligned base pointer with **write** provenance.
    ///
    /// The pointer is derived from `&mut self` (not `&self`), so callers that
    /// cache it and later write through it — e.g. `buffer_alloc` reconstructed
    /// into a `&mut [u8]` via `from_raw_parts_mut` — do not retag from a
    /// read-only borrow, which is Undefined Behavior under Stacked Borrows.
    /// Nothing else accesses the backing `Vec` after this, so the cached
    /// pointer stays valid for the buffer's lifetime.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        if self.size == 0 {
            std::ptr::null_mut()
        } else {
            // SAFETY: `offset + size <= vec.len()` (established in `new`), so
            // the offset stays inside the allocation.
            unsafe { self.vec.as_mut_ptr().add(self.offset) }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.vec[self.offset..self.offset + self.size]
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.vec[self.offset..self.offset + self.size]
    }
}
