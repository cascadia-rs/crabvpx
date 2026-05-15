unsafe extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint64_t = u64;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const ADDRESS_STORAGE_SIZE: usize = ::core::mem::size_of::<size_t>();
pub const DEFAULT_ALIGNMENT: usize =
    (2 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize);
pub const VPX_MAX_ALLOCABLE_MEMORY: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 40 as ::core::ffi::c_int;
unsafe extern "C" fn check_size_argument_overflow(
    mut nmemb: uint64_t,
    mut size: uint64_t,
) -> ::core::ffi::c_int {
    let total_size: uint64_t = nmemb.wrapping_mul(size);
    if nmemb == 0 as uint64_t {
        return 1 as ::core::ffi::c_int;
    }
    if size > (VPX_MAX_ALLOCABLE_MEMORY as uint64_t).wrapping_div(nmemb) {
        return 0 as ::core::ffi::c_int;
    }
    if total_size != total_size as size_t as uint64_t {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_malloc_address_location(mem: *mut ::core::ffi::c_void) -> *mut size_t { unsafe {
    return (mem as *mut size_t).offset(-(1 as ::core::ffi::c_int as isize));
}}
unsafe extern "C" fn get_aligned_malloc_size(mut size: size_t, mut align: size_t) -> uint64_t {
    return (size as uint64_t)
        .wrapping_add(align as uint64_t)
        .wrapping_sub(1 as uint64_t)
        .wrapping_add(ADDRESS_STORAGE_SIZE as uint64_t);
}
unsafe extern "C" fn set_actual_malloc_address(
    mem: *mut ::core::ffi::c_void,
    malloc_addr: *const ::core::ffi::c_void,
) { unsafe {
    let malloc_addr_location: *mut size_t = get_malloc_address_location(mem) as *mut size_t;
    *malloc_addr_location = malloc_addr as size_t;
}}
unsafe extern "C" fn get_actual_malloc_address(
    mem: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void { unsafe {
    let malloc_addr_location: *mut size_t = get_malloc_address_location(mem) as *mut size_t;
    return *malloc_addr_location as *mut ::core::ffi::c_void;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_memalign(
    mut align: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void { unsafe {
    let mut x: *mut ::core::ffi::c_void = NULL;
    let mut addr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let aligned_size: uint64_t = get_aligned_malloc_size(size, align) as uint64_t;
    if check_size_argument_overflow(1 as uint64_t, aligned_size) == 0 {
        return NULL;
    }
    addr = malloc(aligned_size as size_t);
    if !addr.is_null() {
        x = (((addr as *mut ::core::ffi::c_uchar)
            .offset(::core::mem::size_of::<size_t>() as usize as isize) as size_t)
            .wrapping_add(align.wrapping_sub(1 as size_t))
            & !align.wrapping_sub(1 as size_t)) as *mut ::core::ffi::c_void;
        set_actual_malloc_address(x, addr);
    }
    return x;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_malloc(mut size: size_t) -> *mut ::core::ffi::c_void { unsafe {
    return vpx_memalign(DEFAULT_ALIGNMENT, size);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_calloc(mut num: size_t, mut size: size_t) -> *mut ::core::ffi::c_void { unsafe {
    let mut x: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if check_size_argument_overflow(num as uint64_t, size as uint64_t) == 0 {
        return NULL;
    }
    x = vpx_malloc(num.wrapping_mul(size));
    if !x.is_null() {
        memset(x, 0 as ::core::ffi::c_int, num.wrapping_mul(size));
    }
    return x;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_free(mut memblk: *mut ::core::ffi::c_void) { unsafe {
    if !memblk.is_null() {
        let mut addr: *mut ::core::ffi::c_void = get_actual_malloc_address(memblk);
        free(addr);
    }
}}
