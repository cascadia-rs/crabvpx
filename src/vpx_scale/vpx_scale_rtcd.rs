use std::sync::Once;

use crate::vpx_ports::aarch64_cpudetect::arm_cpu_caps;

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    let _flags = arm_cpu_caps();
}

pub fn vpx_scale_rtcd() {
    INIT.call_once(|| {
        setup_rtcd_internal();
    });
}
