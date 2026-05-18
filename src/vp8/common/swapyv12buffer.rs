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
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
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
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[unsafe(no_mangle)]
pub unsafe fn vp8_swap_yv12_buffer(
    mut new_frame: *mut YV12_BUFFER_CONFIG,
    mut last_frame: *mut YV12_BUFFER_CONFIG,
) {
    unsafe {
        let mut temp: *mut u8 = ::core::ptr::null_mut::<u8>();
        temp = (*last_frame).buffer_alloc as *mut u8;
        (*last_frame).buffer_alloc = (*new_frame).buffer_alloc;
        (*new_frame).buffer_alloc = temp as *mut uint8_t;
        temp = (*last_frame).y_buffer as *mut u8;
        (*last_frame).y_buffer = (*new_frame).y_buffer;
        (*new_frame).y_buffer = temp as *mut uint8_t;
        temp = (*last_frame).u_buffer as *mut u8;
        (*last_frame).u_buffer = (*new_frame).u_buffer;
        (*new_frame).u_buffer = temp as *mut uint8_t;
        temp = (*last_frame).v_buffer as *mut u8;
        (*last_frame).v_buffer = (*new_frame).v_buffer;
        (*new_frame).v_buffer = temp as *mut uint8_t;
    }
}
