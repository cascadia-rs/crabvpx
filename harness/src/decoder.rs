#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(not(feature = "rust"))]
#[allow(unused_imports)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub trait VideoDecoder {
    fn init(&mut self) -> Result<(), String>;
    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String>;
    fn get_frame(&mut self) -> Result<Option<()>, String>;
}

#[cfg(not(feature = "rust"))]
pub struct LibVpxOracleDecoder {
    ctx: ffi::vpx_codec_ctx_t,
    initialized: bool,
}

#[cfg(not(feature = "rust"))]
impl LibVpxOracleDecoder {
    pub fn new() -> Self {
        Self {
            ctx: unsafe { std::mem::zeroed() },
            initialized: false,
        }
    }
}

#[cfg(not(feature = "rust"))]
impl Drop for LibVpxOracleDecoder {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                ffi::vpx_codec_destroy(&mut self.ctx);
            }
        }
    }
}

#[cfg(not(feature = "rust"))]
impl VideoDecoder for LibVpxOracleDecoder {
    fn init(&mut self) -> Result<(), String> {
        let res = unsafe {
            ffi::vpx_codec_dec_init_ver(
                &mut self.ctx,
                ffi::vpx_codec_vp8_dx(),
                std::ptr::null(),
                0,
                ffi::VPX_DECODER_ABI_VERSION as i32,
            )
        };
        if res == ffi::vpx_codec_err_t_VPX_CODEC_OK {
            self.initialized = true;
            Ok(())
        } else {
            Err(format!("vpx_codec_dec_init_ver failed: {}", res))
        }
    }

    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let res = unsafe {
            ffi::vpx_codec_decode(
                &mut self.ctx,
                payload.as_ptr(),
                payload.len() as u32,
                std::ptr::null_mut(),
                0,
            )
        };

        if res == ffi::vpx_codec_err_t_VPX_CODEC_OK {
            Ok(())
        } else {
            Err(format!("vpx_codec_decode failed: {}", res))
        }
    }

    fn get_frame(&mut self) -> Result<Option<()>, String> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let mut iter: ffi::vpx_codec_iter_t = std::ptr::null();
        let img = unsafe { ffi::vpx_codec_get_frame(&mut self.ctx, &mut iter) };

        if img.is_null() {
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

#[cfg(feature = "rust")]
pub struct CrabVpxDecoder {
    inner: crabvpx::api::Vp8Decoder,
}

#[cfg(feature = "rust")]
impl CrabVpxDecoder {
    pub fn new() -> Self {
        Self {
            inner: crabvpx::api::Vp8Decoder::new(),
        }
    }
}

#[cfg(feature = "rust")]
impl VideoDecoder for CrabVpxDecoder {
    fn init(&mut self) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.init()
    }

    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.decode(payload)
    }

    fn get_frame(&mut self) -> Result<Option<()>, String> {
        use crabvpx::api::Decoder;
        self.inner.get_frame()
    }
}
