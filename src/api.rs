use crate::vp8::vp8_dx_iface::vpx_codec_vp8_dx;
use crate::vpx::src::vpx_codec::vpx_codec_destroy;
use crate::vpx::src::vpx_decoder::{
    vpx_codec_ctx_t, vpx_codec_dec_init_ver, vpx_codec_decode, vpx_codec_get_frame,
    vpx_codec_iter_t, VPX_CODEC_OK, VPX_DECODER_ABI_VERSION,
};

/// A generic Video Decoder trait that can be implemented by different codecs
/// (e.g., VP8, VP9, AV1, H264).
pub trait Decoder {
    /// The decoded frame representation.
    type Frame;
    /// The error representation.
    type Error;

    /// Initialize the decoder.
    fn init(&mut self) -> Result<(), Self::Error>;

    /// Decode a compressed frame payload.
    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error>;

    /// Retrieve the next available decoded frame.
    fn get_frame(&mut self) -> Result<Option<Self::Frame>, Self::Error>;
}

/// A safe wrapper around the unsafe VP8 `vpx_codec_ctx_t` decoder lifecycle.
pub struct Vp8Decoder {
    ctx: vpx_codec_ctx_t,
    initialized: bool,
}

impl Vp8Decoder {
    pub fn new() -> Self {
        Self {
            ctx: unsafe { core::mem::zeroed() },
            initialized: false,
        }
    }
}

impl Default for Vp8Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Vp8Decoder {
    fn drop(&mut self) {
        if self.initialized {
            // Safely destroy the underlying C context to prevent memory leaks.
            unsafe {
                vpx_codec_destroy(
                    &raw mut self.ctx as *mut _ as *mut crate::vpx::src::vpx_codec::vpx_codec_ctx_t,
                );
            }
        }
    }
}

impl Decoder for Vp8Decoder {
    type Frame = (); // We can expand this to a safe Image struct later.
    type Error = String;

    fn init(&mut self) -> Result<(), Self::Error> {
        let res = unsafe {
            vpx_codec_dec_init_ver(
                &raw mut self.ctx,
                vpx_codec_vp8_dx() as *const _,
                core::ptr::null(),
                0,
                VPX_DECODER_ABI_VERSION,
            )
        };
        if res == VPX_CODEC_OK {
            self.initialized = true;
            Ok(())
        } else {
            Err(format!("vpx_codec_dec_init_ver failed: {}", res))
        }
    }

    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let res = unsafe {
            vpx_codec_decode(
                &raw mut self.ctx,
                payload.as_ptr(),
                payload.len() as u32,
                core::ptr::null_mut(),
                0,
            )
        };

        if res == VPX_CODEC_OK {
            Ok(())
        } else {
            Err(format!("vpx_codec_decode failed: {}", res))
        }
    }

    fn get_frame(&mut self) -> Result<Option<Self::Frame>, Self::Error> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let mut iter: vpx_codec_iter_t = core::ptr::null();
        let img = unsafe { vpx_codec_get_frame(&raw mut self.ctx, &raw mut iter) };

        if img.is_null() {
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}
