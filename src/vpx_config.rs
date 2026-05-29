#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_build_config() -> *const i8 {
    b"--target=generic-gnu --disable-examples --disable-tools --disable-docs --disable-unit-tests --disable-vp9 --disable-vp8-encoder\0"
        as *const u8 as *const i8
}
