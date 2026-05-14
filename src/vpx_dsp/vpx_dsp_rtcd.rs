extern "C" {
    fn arm_cpu_caps() -> ::core::ffi::c_int;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type pthread_once_t = __darwin_pthread_once_t;
unsafe extern "C" fn setup_rtcd_internal() {
    let mut flags: ::core::ffi::c_int = arm_cpu_caps();
}
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) {
    static mut lock: pthread_once_t = _opaque_pthread_once_t {
        __sig: _PTHREAD_ONCE_SIG_init as ::core::ffi::c_long,
        __opaque: [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };
    pthread_once(&raw mut lock, func as Option<unsafe extern "C" fn() -> ()>);
}
#[no_mangle]
pub unsafe extern "C" fn vpx_dsp_rtcd() {
    once(Some(setup_rtcd_internal as unsafe extern "C" fn() -> ()));
}
