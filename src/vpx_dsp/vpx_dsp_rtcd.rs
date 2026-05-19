use std::sync::Once;

unsafe extern "C" {
    fn arm_cpu_caps() -> ::core::ffi::c_int;
}

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    unsafe {
        let mut _flags: ::core::ffi::c_int = arm_cpu_caps();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_dsp_rtcd() {
    INIT.call_once(setup_rtcd_internal);
}
