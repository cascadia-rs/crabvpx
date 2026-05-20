use crate::vp8::vp8_dx_iface::vpx_image_t;
use crate::vp8::vp8_dx_iface::Vp8DecoderInstance;

/// A safe wrapper around the decoded image planes.
pub struct Image<'a> {
    img: &'a vpx_image_t,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Plane {
    Y,
    U,
    V,
    Alpha,
}

impl<'a> Image<'a> {
    /// Get the displayed width of the image.
    pub fn width(&self) -> u32 {
        self.img.d_w
    }

    /// Get the displayed height of the image.
    pub fn height(&self) -> u32 {
        self.img.d_h
    }

    /// Safely access a specific plane slice.
    pub fn plane(&self, plane: Plane) -> Option<&'a [u8]> {
        unsafe {
            let idx = match plane {
                Plane::Y => 0,
                Plane::U => 1,
                Plane::V => 2,
                Plane::Alpha => 3,
            };

            let ptr = self.img.planes[idx];
            if ptr.is_null() {
                return None;
            }

            let stride = self.img.stride[idx];
            if stride <= 0 {
                return None;
            }

            let height = if idx > 0 && idx < 3 {
                (self.img.d_h + (1 << self.img.y_chroma_shift) - 1) >> self.img.y_chroma_shift
            } else {
                self.img.d_h
            };

            let len = (height as usize) * (stride as usize);
            Some(core::slice::from_raw_parts(ptr as *const u8, len))
        }
    }

    /// Get the bit depth of the image.
    pub fn bit_depth(&self) -> u32 {
        self.img.bit_depth
    }

    /// Compute the MD5 hash of the image planes (matching Oracle behavior).
    pub fn md5(&self) -> String {
        let mut context = md5::Context::new();

        for (plane_idx, plane_type) in [(0, Plane::Y), (1, Plane::U), (2, Plane::V)] {
            if let Some(plane_data) = self.plane(plane_type) {
                let stride = self.img.stride[plane_idx] as usize;
                let w = if plane_idx == 0 { self.img.d_w as usize } else { ((self.img.d_w + 1) >> 1) as usize };
                let h = if plane_idx == 0 { self.img.d_h as usize } else { ((self.img.d_h + 1) >> 1) as usize };

                for row in 0..h {
                    let start = row * stride;
                    let end = start + w;
                    if end <= plane_data.len() {
                        context.consume(&plane_data[start..end]);
                    }
                }
            }
        }
        format!("{:x}", context.compute())
    }
}

/// A generic Video Decoder trait that can be implemented by different codecs
/// (e.g., VP8, VP9, AV1, H264).
pub trait Decoder {
    /// The decoded frame representation.
    type Frame<'a> where Self: 'a;
    /// The error representation.
    type Error;

    /// Initialize the decoder.
    fn init(&mut self) -> Result<(), Self::Error>;

    /// Decode a compressed frame payload.
    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error>;

    /// Retrieve the next available decoded frame.
    fn get_frame<'a>(&'a mut self) -> Result<Option<Self::Frame<'a>>, Self::Error>;
}

/// A safe wrapper around the unsafe VP8 `Vp8DecoderInstance` decoder lifecycle.
pub struct Vp8Decoder {
    instance: Option<Vp8DecoderInstance>,
}

impl Vp8Decoder {
    pub fn new() -> Self {
        Self { instance: None }
    }
}

impl Default for Vp8Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for Vp8Decoder {
    type Frame<'a> = Image<'a>;
    type Error = String;

    fn init(&mut self) -> Result<(), Self::Error> {
        let threads = std::env::var("CRABVPX_THREADS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        self.instance = Some(Vp8DecoderInstance::new(threads)?);
        Ok(())
    }

    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        let inst = self
            .instance
            .as_mut()
            .ok_or_else(|| "Decoder not initialized".to_string())?;
        inst.decode(payload)
    }

    fn get_frame<'a>(&'a mut self) -> Result<Option<Self::Frame<'a>>, Self::Error> {
        let inst = self
            .instance
            .as_mut()
            .ok_or_else(|| "Decoder not initialized".to_string())?;
        if let Some(img) = inst.get_frame() {
            unsafe {
                if img.is_null() {
                    return Ok(None);
                }
                Ok(Some(Image {
                    img: &*(img as *const vpx_image_t),
                }))
            }
        } else {
            Ok(None)
        }
    }
}

