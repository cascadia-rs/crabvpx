use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;

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
        temp = (*last_frame).buffer_alloc_base;
        (*last_frame).buffer_alloc_base = (*new_frame).buffer_alloc_base;
        (*new_frame).buffer_alloc_base = temp;
        let temp_cap = (*last_frame).buffer_alloc_cap;
        (*last_frame).buffer_alloc_cap = (*new_frame).buffer_alloc_cap;
        (*new_frame).buffer_alloc_cap = temp_cap;
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
