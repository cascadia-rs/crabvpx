static mut cfg: *const ::core::ffi::c_char = b"--target=generic-gnu --disable-examples --disable-tools --disable-docs --disable-unit-tests --disable-vp9 --disable-vp8-encoder\0"
    as *const u8 as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_codec_build_config() -> *const ::core::ffi::c_char {
    unsafe { cfg }
}
