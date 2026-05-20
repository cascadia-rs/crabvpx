use std::sync::Once;

use crate::vpx_ports::aarch64_cpudetect::arm_cpu_caps;

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    let mut _flags: ::core::ffi::c_int = arm_cpu_caps();
}

pub fn vpx_dsp_rtcd() {
    INIT.call_once(setup_rtcd_internal);
}
