extern "C" {
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn sysctlbyname(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_void,
        oldlenp: *mut size_t,
        _: *mut ::core::ffi::c_void,
        newlen: size_t,
    ) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type int64_t = i64;
pub type size_t = __darwin_size_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const HAS_NEON: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const HAS_NEON_DOTPROD: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const HAS_NEON_I8MM: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const HAS_SVE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const HAS_SVE2: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn arm_cpu_env_flags(mut flags: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut env: *const ::core::ffi::c_char =
        getenv(b"VPX_SIMD_CAPS\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null() && *env as ::core::ffi::c_int != 0 {
        *flags = strtol(
            env,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn arm_cpu_env_mask() -> ::core::ffi::c_int {
    let mut env: *const ::core::ffi::c_char =
        getenv(b"VPX_SIMD_CAPS_MASK\0" as *const u8 as *const ::core::ffi::c_char);
    return if !env.is_null() && *env as ::core::ffi::c_int != 0 {
        strtol(
            env,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
    } else {
        !(0 as ::core::ffi::c_int)
    };
}
#[inline]
unsafe extern "C" fn have_feature(mut feature: *const ::core::ffi::c_char) -> int64_t {
    let mut feature_present: int64_t = 0 as int64_t;
    let mut size: size_t = ::core::mem::size_of::<int64_t>() as size_t;
    if sysctlbyname(
        feature,
        &raw mut feature_present as *mut ::core::ffi::c_void,
        &raw mut size,
        NULL,
        0 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return 0 as int64_t;
    }
    return feature_present;
}
unsafe extern "C" fn arm_get_cpu_caps() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    flags |= HAS_NEON;
    if have_feature(b"hw.optional.arm.FEAT_DotProd\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        flags |= HAS_NEON_DOTPROD;
    }
    if have_feature(b"hw.optional.arm.FEAT_I8MM\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        flags |= HAS_NEON_I8MM;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn arm_cpu_caps() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if arm_cpu_env_flags(&raw mut flags) == 0 {
        flags = arm_get_cpu_caps() & arm_cpu_env_mask();
    }
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_NEON_I8MM;
    }
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_SVE;
    }
    if flags & HAS_NEON_I8MM == 0 {
        flags &= !HAS_SVE;
    }
    if flags & HAS_SVE == 0 {
        flags &= !HAS_SVE2;
    }
    return flags;
}
