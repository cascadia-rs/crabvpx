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
        core::mem::swap(
            &mut (*last_frame).buffer_alloc,
            &mut (*new_frame).buffer_alloc,
        );
        core::mem::swap(
            &mut (*last_frame).buffer_alloc_base,
            &mut (*new_frame).buffer_alloc_base,
        );
        core::mem::swap(
            &mut (*last_frame).buffer_alloc_cap,
            &mut (*new_frame).buffer_alloc_cap,
        );
        core::mem::swap(&mut (*last_frame).y_buffer, &mut (*new_frame).y_buffer);
        core::mem::swap(&mut (*last_frame).u_buffer, &mut (*new_frame).u_buffer);
        core::mem::swap(&mut (*last_frame).v_buffer, &mut (*new_frame).v_buffer);
    }
}
