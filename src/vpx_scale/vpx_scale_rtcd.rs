use std::ffi::c_void;
unsafe extern "Rust" {
    fn pthread_once(_: *mut pthread_once_t, _: Option<unsafe fn() -> ()>) -> i32;
}
pub type pthread_once_t = *mut c_void;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: i64,
    pub __opaque: [i8; 8],
}
unsafe fn setup_rtcd_internal() {}
pub const _PTHREAD_ONCE_SIG_init: i32 = 0x30b1bcba as i32;
unsafe fn once(mut func: Option<unsafe fn() -> ()>) {
    unsafe {
        static INIT: std::sync::Once = std::sync::Once::new();
        if let Some(f) = func {
            INIT.call_once(|| f());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_scale_rtcd() {
    unsafe {
        once(Some(setup_rtcd_internal as unsafe fn() -> ()));
    }
}
