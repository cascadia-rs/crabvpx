use std::ffi::c_void;
unsafe extern "Rust" {
    fn vpx_memalign(align: SizeT, size: SizeT) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
}
pub type DarwinSizeT = usize;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorRangeT = VpxColorRange;
pub type SizeT = DarwinSizeT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: SizeT,
    pub border: i32,
    pub frame_size: SizeT,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: VpxColorSpaceT,
    pub color_range: VpxColorRangeT,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_de_alloc_frame_buffer(mut ybf: *mut Yv12BufferConfig) -> i32 {
    unsafe {
        if !ybf.is_null() {
            if (*ybf).buffer_alloc_sz > 0 as SizeT {
                vpx_free((*ybf).buffer_alloc as *mut c_void);
            }
            core::ptr::write_bytes(
                ybf as *mut c_void as *mut u8,
                0 as i32 as u8,
                ::core::mem::size_of::<Yv12BufferConfig>() as SizeT,
            );
        } else {
            return -(1 as i32);
        }
        0 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_realloc_frame_buffer(
    mut ybf: *mut Yv12BufferConfig,
    mut width: i32,
    mut height: i32,
    mut border: i32,
) -> i32 {
    unsafe {
        if !ybf.is_null() {
            let mut aligned_width: i32 = (width + 15 as i32) & !(15 as i32);
            let mut aligned_height: i32 = (height + 15 as i32) & !(15 as i32);
            let mut y_stride: i32 = (aligned_width + 2 as i32 * border + 31 as i32) & !(31 as i32);
            let mut yplane_size: i32 = (aligned_height + 2 as i32 * border) * y_stride;
            let mut uv_width: i32 = aligned_width >> 1 as i32;
            let mut uv_height: i32 = aligned_height >> 1 as i32;
            let mut uv_stride: i32 = y_stride >> 1 as i32;
            let mut uvplane_size: i32 = (uv_height + border) * uv_stride;
            let frame_size: SizeT = (yplane_size + 2 as i32 * uvplane_size) as SizeT;
            if (*ybf).buffer_alloc.is_null() {
                (*ybf).buffer_alloc = vpx_memalign(32 as SizeT, frame_size) as *mut u8;
                if (*ybf).buffer_alloc.is_null() {
                    (*ybf).buffer_alloc_sz = 0 as SizeT;
                    return -(1 as i32);
                }
                (*ybf).buffer_alloc_sz = frame_size;
            }
            if (*ybf).buffer_alloc_sz < frame_size {
                return -(1 as i32);
            }
            if border & 0x1f as i32 != 0 {
                return -(3 as i32);
            }
            (*ybf).y_crop_width = width;
            (*ybf).y_crop_height = height;
            (*ybf).y_width = aligned_width;
            (*ybf).y_height = aligned_height;
            (*ybf).y_stride = y_stride;
            (*ybf).uv_crop_width = (width + 1 as i32) / 2 as i32;
            (*ybf).uv_crop_height = (height + 1 as i32) / 2 as i32;
            (*ybf).uv_width = uv_width;
            (*ybf).uv_height = uv_height;
            (*ybf).uv_stride = uv_stride;
            (*ybf).alpha_width = 0 as i32;
            (*ybf).alpha_height = 0 as i32;
            (*ybf).alpha_stride = 0 as i32;
            (*ybf).border = border;
            (*ybf).frame_size = frame_size;
            (*ybf).y_buffer = (*ybf)
                .buffer_alloc
                .offset((border * y_stride) as isize)
                .offset(border as isize);
            (*ybf).u_buffer = (*ybf)
                .buffer_alloc
                .offset(yplane_size as isize)
                .offset((border / 2 as i32 * uv_stride) as isize)
                .offset((border / 2 as i32) as isize);
            (*ybf).v_buffer = (*ybf)
                .buffer_alloc
                .offset(yplane_size as isize)
                .offset(uvplane_size as isize)
                .offset((border / 2 as i32 * uv_stride) as isize)
                .offset((border / 2 as i32) as isize);
            (*ybf).alpha_buffer = ::core::ptr::null_mut::<u8>();
            (*ybf).corrupted = 0 as i32;
            return 0 as i32;
        }
        -(2 as i32)
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_yv12_alloc_frame_buffer(
    mut ybf: *mut Yv12BufferConfig,
    mut width: i32,
    mut height: i32,
    mut border: i32,
) -> i32 {
    unsafe {
        if !ybf.is_null() {
            vp8_yv12_de_alloc_frame_buffer(ybf);
            return vp8_yv12_realloc_frame_buffer(ybf, width, height, border);
        }
        -(2 as i32)
    }
}
