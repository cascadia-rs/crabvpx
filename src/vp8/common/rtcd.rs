unsafe extern "C" {
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
}
pub type pthread_once_t = *mut ::core::ffi::c_void;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
unsafe extern "C" fn setup_rtcd_internal() {}
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) {
    static INIT: std::sync::Once = std::sync::Once::new();
    if let Some(f) = func {
        INIT.call_once(|| f());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_rtcd() { unsafe {
    once(Some(setup_rtcd_internal as unsafe extern "C" fn() -> ()));
}}
