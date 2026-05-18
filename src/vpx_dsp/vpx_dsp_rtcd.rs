unsafe extern "Rust" {
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe fn() -> ()>,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type pthread_once_t = *mut ::core::ffi::c_void;
unsafe fn setup_rtcd_internal() {}
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
unsafe fn once(mut func: Option<unsafe fn() -> ()>) {
    unsafe {
        static INIT: std::sync::Once = std::sync::Once::new();
        if let Some(f) = func {
            INIT.call_once(|| f());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_dsp_rtcd() {
    unsafe {
        once(Some(setup_rtcd_internal as unsafe fn() -> ()));
    }
}
