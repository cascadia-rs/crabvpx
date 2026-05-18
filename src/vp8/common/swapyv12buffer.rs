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
pub type DarwinSizeT = usize;
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
#[unsafe(no_mangle)]
pub unsafe fn vp8_swap_yv12_buffer(
    mut new_frame: *mut Yv12BufferConfig,
    mut last_frame: *mut Yv12BufferConfig,
) {
    unsafe {
        let mut temp: *mut u8 = ::core::ptr::null_mut::<u8>();
        temp = (*last_frame).buffer_alloc as *mut u8;
        (*last_frame).buffer_alloc = (*new_frame).buffer_alloc;
        (*new_frame).buffer_alloc = temp as *mut u8;
        temp = (*last_frame).y_buffer as *mut u8;
        (*last_frame).y_buffer = (*new_frame).y_buffer;
        (*new_frame).y_buffer = temp as *mut u8;
        temp = (*last_frame).u_buffer as *mut u8;
        (*last_frame).u_buffer = (*new_frame).u_buffer;
        (*new_frame).u_buffer = temp as *mut u8;
        temp = (*last_frame).v_buffer as *mut u8;
        (*last_frame).v_buffer = (*new_frame).v_buffer;
        (*new_frame).v_buffer = temp as *mut u8;
    }
}
