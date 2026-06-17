use crate::vp8::vp8_dx_iface::{Vp8DecoderInstance, YV12_BUFFER_CONFIG};

/// A safe wrapper around the decoded image planes.
pub struct Image<'a> {
    d_w: u32,
    d_h: u32,
    y_plane: &'a [u8],
    u_plane: &'a [u8],
    v_plane: &'a [u8],
    alpha_plane: Option<&'a [u8]>,
    bit_depth: u32,
    strides: [usize; 4],
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
        self.d_w
    }

    /// Get the displayed height of the image.
    pub fn height(&self) -> u32 {
        self.d_h
    }

    /// Safely access a specific plane slice.
    pub fn plane(&self, plane: Plane) -> Option<&'a [u8]> {
        match plane {
            Plane::Y => Some(self.y_plane),
            Plane::U => Some(self.u_plane),
            Plane::V => Some(self.v_plane),
            Plane::Alpha => self.alpha_plane,
        }
    }

    /// Row stride (in bytes) of a plane's backing buffer.
    pub fn stride(&self, plane: Plane) -> usize {
        match plane {
            Plane::Y => self.strides[0],
            Plane::U => self.strides[1],
            Plane::V => self.strides[2],
            Plane::Alpha => self.strides[3],
        }
    }

    /// Get the bit depth of the image.
    pub fn bit_depth(&self) -> u32 {
        self.bit_depth
    }

    /// Compute the MD5 hash of the image planes (matching Oracle behavior).
    pub fn md5(&self) -> String {
        let mut context = md5::Context::new();

        for (plane_idx, plane_type) in [(0, Plane::Y), (1, Plane::U), (2, Plane::V)] {
            if let Some(plane_data) = self.plane(plane_type) {
                let stride = self.strides[plane_idx];
                let w = if plane_idx == 0 { self.d_w as usize } else { ((self.d_w + 1) >> 1) as usize };
                let h = if plane_idx == 0 { self.d_h as usize } else { ((self.d_h + 1) >> 1) as usize };

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
        if let Some((cfg, width, height)) = inst.get_frame() {
            let (y_plane, u_plane, v_plane) = cfg.views();
            let alpha_plane = None;

            Ok(Some(Image {
                d_w: width as u32,
                d_h: height as u32,
                y_plane,
                u_plane,
                v_plane,
                alpha_plane,
                bit_depth: if cfg.bit_depth == 0 { 8 } else { cfg.bit_depth },
                strides: [
                    cfg.y_stride as usize,
                    cfg.uv_stride as usize,
                    cfg.uv_stride as usize,
                    cfg.alpha_stride as usize,
                ],
            }))
        } else {
            Ok(None)
        }
    }
}
