#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(not(feature = "rust"))]
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

    unsafe fn calculate_frame_info(img: *const ffi::vpx_image_t) -> DecodedFrame {
        let mut context = md5::Context::new();
        let img = &*img;

        for plane in 0..3 {
            let data = img.planes[plane];
            let stride = img.stride[plane] as usize;
            let w = if plane == 0 { img.d_w } else { (img.d_w + 1) >> 1 };
            let h = if plane == 0 { img.d_h } else { (img.d_h + 1) >> 1 };

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
        let threads = std::env::var("CRABVPX_THREADS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let cfg = ffi::vpx_codec_dec_cfg_t {
            threads,
            w: 0,
            h: 0,
        };
        let res = unsafe {
            ffi::vpx_codec_dec_init_ver(
                &mut self.ctx,
                ffi::vpx_codec_vp8_dx(),
                &cfg,
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

    fn get_frame(&mut self) -> Result<Option<DecodedFrame>, String> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let mut iter: ffi::vpx_codec_iter_t = std::ptr::null();
        let img = unsafe { ffi::vpx_codec_get_frame(&mut self.ctx, &mut iter) };

        if img.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { Self::calculate_frame_info(img) }))
        }
    }
}

#[cfg(feature = "rust")]
pub struct CrabVpxDecoder {
    inner: crabvpx::api::Vp8Decoder,
    frame_counter: usize,
}

#[cfg(feature = "rust")]
impl CrabVpxDecoder {
    pub fn new() -> Self {
        Self {
            inner: crabvpx::api::Vp8Decoder::new(),
            frame_counter: 0,
        }
    }
}

#[cfg(feature = "rust")]
impl VideoDecoder for CrabVpxDecoder {
    fn init(&mut self) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.init().map_err(|e| e.to_string())
    }

    fn decode_frame(&mut self, payload: &[u8]) -> Result<(), String> {
        use crabvpx::api::Decoder;
        self.inner.decode(payload).map_err(|e| e.to_string())
    }

    fn get_frame(&mut self) -> Result<Option<DecodedFrame>, String> {
        use crabvpx::api::Decoder;
        match self.inner.get_frame().map_err(|e| e.to_string())? {
            Some(img) => {
                if std::env::var("CRABVPX_DUMP").is_ok() && self.frame_counter == 1 {
                    let threads = std::env::var("CRABVPX_THREADS").unwrap_or_else(|_| "1".to_string());
                    let path = format!("/usr/local/google/home/liberato/.gemini/jetski/brain/b80e2e3b-61ce-4410-9009-62a1230d94f6/scratch/dump_threads_{}.yuv", threads);
                    let mut f = std::fs::File::create(&path).map_err(|e| e.to_string())?;
                    use std::io::Write;
                    
                    if let Some(y) = img.plane(crabvpx::api::Plane::Y) {
                        f.write_all(y).map_err(|e| e.to_string())?;
                    }
                    if let Some(u) = img.plane(crabvpx::api::Plane::U) {
                        f.write_all(u).map_err(|e| e.to_string())?;
                    }
                    if let Some(v) = img.plane(crabvpx::api::Plane::V) {
                        f.write_all(v).map_err(|e| e.to_string())?;
                    }
                    println!("DEBUG: Dumped Frame 1 to {}", path);
                }
                self.frame_counter += 1;
                
                Ok(Some(DecodedFrame {
                    md5: img.md5(),
                    width: img.width(),
                    height: img.height(),
                    bit_depth: img.bit_depth(),
                }))
            }
            None => Ok(None),
        }
    }
}
