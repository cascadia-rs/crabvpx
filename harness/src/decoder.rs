#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[allow(unused_imports)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedFrame {
    pub md5: String,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
}

pub trait VideoDecoder {
    fn init(&mut self) -> Result<(), String>;
    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String>;
    fn get_frame(&mut self) -> Result<Option<DecodedFrame>, String>;
}

pub struct LibVpxOracleDecoder {
    ctx: ffi::vpx_codec_ctx_t,
    initialized: bool,
    vpx: ffi::Vpx,
}

impl LibVpxOracleDecoder {
    pub fn new() -> Self {
        let lib_name = format!("../libvpx/libvpx.{}", std::env::consts::DLL_EXTENSION);
        let vpx = unsafe {
            ffi::Vpx::new(&lib_name)
                .unwrap_or_else(|e| panic!("Failed to load {}: {:?}", lib_name, e))
        };
        Self {
            ctx: unsafe { std::mem::zeroed() },
            initialized: false,
            vpx,
        }
    }

    unsafe fn calculate_frame_info(img: *const ffi::vpx_image_t) -> DecodedFrame {
        unsafe {
            let mut context = md5::Context::new();
            let img = &*img;

            for plane in 0..3 {
                let data = img.planes[plane];
                let stride = img.stride[plane] as usize;
                let w = if plane == 0 {
                    img.d_w
                } else {
                    (img.d_w + 1) >> 1
                };
                let h = if plane == 0 {
                    img.d_h
                } else {
                    (img.d_h + 1) >> 1
                };

                for row in 0..h {
                    let row_ptr = data.add(row as usize * stride);
                    let row_data = std::slice::from_raw_parts(row_ptr, w as usize);
                    context.consume(row_data);
                }
            }

            DecodedFrame {
                md5: format!("{:x}", context.compute()),
                width: img.d_w,
                height: img.d_h,
                bit_depth: img.bit_depth,
            }
        }
    }
}

impl Default for LibVpxOracleDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LibVpxOracleDecoder {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                self.vpx.vpx_codec_destroy(&mut self.ctx);
            }
        }
    }
}

impl VideoDecoder for LibVpxOracleDecoder {
    fn init(&mut self) -> Result<(), String> {
        let res = unsafe {
            self.vpx.vpx_codec_dec_init_ver(
                &mut self.ctx,
                self.vpx.vpx_codec_vp8_dx(),
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
            self.vpx.vpx_codec_decode(
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

    fn get_frame(&mut self) -> Result<Option<DecodedFrame>, String> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let mut iter: ffi::vpx_codec_iter_t = std::ptr::null();
        let img = unsafe { self.vpx.vpx_codec_get_frame(&mut self.ctx, &mut iter) };

        if img.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { Self::calculate_frame_info(img) }))
        }
    }
}

pub struct CrabVpxDecoder {
    inner: crabvpx::api::Vp8Decoder,
}

impl CrabVpxDecoder {
    pub fn new() -> Self {
        Self {
            inner: crabvpx::api::Vp8Decoder::new(),
        }
    }
}

impl Default for CrabVpxDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoDecoder for CrabVpxDecoder {
    fn init(&mut self) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.init()
    }

    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.decode(payload)
    }

    fn get_frame(&mut self) -> Result<Option<DecodedFrame>, String> {
        use crabvpx::api::Decoder;
        match self.inner.get_frame()? {
            Some(f) => Ok(Some(DecodedFrame {
                md5: f.md5,
                width: f.width,
                height: f.height,
                bit_depth: f.bit_depth,
            })),
            None => Ok(None),
        }
    }
}
