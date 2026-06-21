pub use crate::vp8::common::types::*;
use crate::vpx_mem::vpx_mem::AlignedBox;

pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;

pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub fn vp8_yv12_de_alloc_frame_buffer_safe(
    ybf: &mut YV12_BUFFER_CONFIG,
    alloc: &mut Option<AlignedBox>,
) {
    *alloc = None;
    *ybf = YV12_BUFFER_CONFIG::default();
}

pub fn vp8_yv12_realloc_frame_buffer_safe(
    ybf: &mut YV12_BUFFER_CONFIG,
    width: i32,
    height: i32,
    border: i32,
    alloc: &mut Option<AlignedBox>,
) -> Result<(), i32> {
    let aligned_width = (width + 15) & !15;
    let aligned_height = (height + 15) & !15;
    let y_stride = (aligned_width + 2 * border + 31) & !31;
    let yplane_size = (aligned_height + 2 * border) * y_stride;
    let uv_width = aligned_width >> 1;
    let uv_height = aligned_height >> 1;
    let uv_stride = y_stride >> 1;
    let uvplane_size = (uv_height + border) * uv_stride;
    let frame_size = (yplane_size + 2 * uvplane_size) as usize;

    if alloc.is_none() {
        let mut aligned_box = match AlignedBox::new(32, frame_size) {
            Some(b) => b,
            None => {
                ybf.buffer_alloc_sz = 0;
                return Err(-1);
            }
        };
        ybf.buffer_alloc = aligned_box.as_mut_ptr();
        ybf.buffer_alloc_sz = frame_size;
        *alloc = Some(aligned_box);
    }

    if ybf.buffer_alloc_sz < frame_size {
        return Err(-1);
    }
    if border & 0x1f != 0 {
        return Err(-3);
    }

    ybf.y_crop_width = width;
    ybf.y_crop_height = height;
    ybf.y_width = aligned_width;
    ybf.y_height = aligned_height;
    ybf.y_stride = y_stride;
    ybf.uv_crop_width = (width + 1) / 2;
    ybf.uv_crop_height = (height + 1) / 2;
    ybf.uv_width = uv_width;
    ybf.uv_height = uv_height;
    ybf.uv_stride = uv_stride;
    ybf.alpha_width = 0;
    ybf.alpha_height = 0;
    ybf.alpha_stride = 0;
    ybf.border = border;
    ybf.frame_size = frame_size;

    // Derive the plane pointers from `buffer_alloc` with pointer arithmetic
    // rather than usize round-trips, so they inherit its provenance (no
    // exposed/int-to-ptr provenance for Miri to lose track of). All offsets
    // stay within the `frame_size` allocation.
    let base = ybf.buffer_alloc;
    unsafe {
        ybf.y_buffer = base.add((border * y_stride + border) as usize);
        ybf.u_buffer =
            base.add(yplane_size as usize + ((border / 2) * uv_stride + border / 2) as usize);
        ybf.v_buffer = base.add(
            yplane_size as usize
                + uvplane_size as usize
                + ((border / 2) * uv_stride + border / 2) as usize,
        );
    }
    ybf.alpha_buffer = core::ptr::null_mut();
    ybf.corrupted = 0;

    Ok(())
}

pub fn vp8_yv12_alloc_frame_buffer_safe(
    ybf: &mut YV12_BUFFER_CONFIG,
    width: i32,
    height: i32,
    border: i32,
    alloc: &mut Option<AlignedBox>,
) -> Result<(), i32> {
    vp8_yv12_de_alloc_frame_buffer_safe(ybf, alloc);
    vp8_yv12_realloc_frame_buffer_safe(ybf, width, height, border, alloc)
}
