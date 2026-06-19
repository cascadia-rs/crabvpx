use std::sync::Once;

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    // Pure-Rust generic build: no runtime SIMD dispatch to initialize.
}

pub fn vp8_rtcd() {
    INIT.call_once(|| {
        setup_rtcd_internal();
    });
}
