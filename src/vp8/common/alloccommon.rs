// FFI imports removed
use crate::vp8::common::entropymode::{vp8_default_bmode_probs, vp8_init_mbmode_probs};
use crate::vp8::common::generic::systemdependent::vp8_machine_specific_config;
use crate::vpx_scale::generic::yv12config::{
    vp8_yv12_alloc_frame_buffer_safe, vp8_yv12_de_alloc_frame_buffer_safe,
};

pub use crate::vp8::common::types::*;

/// Allocate a boxed slice of `count` clones of `value` without aborting on OOM.
///
/// Dimensions are bitstream-controlled (up to VP8's spec maximum of 16383x16383),
/// so a hostile header can request a very large `count`. Using `try_reserve`
/// lets the decoder return an error instead of triggering a process abort.
fn try_alloc_boxed_slice<T: Clone>(value: T, count: usize) -> Option<Box<[T]>> {
    let mut v: Vec<T> = Vec::new();
    v.try_reserve_exact(count).ok()?;
    v.resize(count, value);
    Some(v.into_boxed_slice())
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: i32,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [i32; 48];
pub type vpx_codec_err_t = u32;
pub const VPX_CODEC_LIST_END: vpx_codec_err_t = 9;
pub const VPX_CODEC_INVALID_PARAM: vpx_codec_err_t = 8;
pub const VPX_CODEC_CORRUPT_FRAME: vpx_codec_err_t = 7;
pub const VPX_CODEC_UNSUP_FEATURE: vpx_codec_err_t = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: vpx_codec_err_t = 5;
pub const VPX_CODEC_INCAPABLE: vpx_codec_err_t = 4;
pub const VPX_CODEC_ABI_MISMATCH: vpx_codec_err_t = 3;
pub const VPX_CODEC_MEM_ERROR: vpx_codec_err_t = 2;
pub const VPX_CODEC_ERROR: vpx_codec_err_t = 1;
pub const VPX_CODEC_OK: vpx_codec_err_t = 0;

pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;

pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const VP8BORDERINPIXELS: i32 = 32_i32;
pub const NUM_YV12_BUFFERS: i32 = 4_i32;
pub fn vp8_de_alloc_frame_buffers(oci: &mut VP8_COMMON) {
    let mut i: i32 = 0;
    i = 0_i32;
    while i < NUM_YV12_BUFFERS {
        let (fb, alloc) = (
            &mut oci.yv12_fb[i as usize],
            &mut oci.yv12_fb_allocs[i as usize],
        );
        vp8_yv12_de_alloc_frame_buffer_safe(fb, alloc);
        oci.fb_idx_ref_cnt[i as usize] = 0_i32;
        i += 1;
    }
    let (temp_fb, temp_alloc) = (&mut oci.temp_scale_frame, &mut oci.temp_scale_frame_alloc);
    vp8_yv12_de_alloc_frame_buffer_safe(temp_fb, temp_alloc);

    oci.above_context = None;
    oci.mip = None;
    oci.frame_to_show_idx = None;
}

pub fn vp8_alloc_frame_buffers(oci: &mut VP8_COMMON, mut width: i32, mut height: i32) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    vp8_de_alloc_frame_buffers(oci);
    if width & 0xf_i32 != 0_i32 {
        width += 16_i32 - (width & 0xf_i32);
    }
    if height & 0xf_i32 != 0_i32 {
        height += 16_i32 - (height & 0xf_i32);
    }
    i = 0_i32;
    loop {
        if i >= NUM_YV12_BUFFERS {
            current_block = 10886091980245723256;
            break;
        }
        let (fb, alloc) = (
            &mut oci.yv12_fb[i as usize],
            &mut oci.yv12_fb_allocs[i as usize],
        );
        if vp8_yv12_alloc_frame_buffer_safe(fb, width, height, VP8BORDERINPIXELS, alloc).is_err() {
            current_block = 9271424053274658668;
            break;
        }
        i += 1;
    }
    if current_block == 10886091980245723256 {
        oci.new_fb_idx = 0_i32;
        oci.lst_fb_idx = 1_i32;
        oci.gld_fb_idx = 2_i32;
        oci.alt_fb_idx = 3_i32;
        oci.fb_idx_ref_cnt[0_i32 as usize] = 1_i32;
        oci.fb_idx_ref_cnt[1_i32 as usize] = 1_i32;
        oci.fb_idx_ref_cnt[2_i32 as usize] = 1_i32;
        oci.fb_idx_ref_cnt[3_i32 as usize] = 1_i32;
        let (temp_fb, temp_alloc) = (&mut oci.temp_scale_frame, &mut oci.temp_scale_frame_alloc);
        if vp8_yv12_alloc_frame_buffer_safe(temp_fb, width, 16_i32, VP8BORDERINPIXELS, temp_alloc)
            .is_ok()
        {
            oci.mb_rows = height >> 4_i32;
            oci.mb_cols = width >> 4_i32;
            oci.MBs = oci.mb_rows * oci.mb_cols;
            oci.mode_info_stride = oci.mb_cols + 1_i32;
            let mip_count = ((oci.mb_cols + 1) * (oci.mb_rows + 1)) as usize;
            if let Some(mip_box) = try_alloc_boxed_slice(MODE_INFO::default(), mip_count) {
                oci.mip = Some(mip_box);
                let above_context_count = oci.mb_cols as usize;
                if let Some(above_context_box) =
                    try_alloc_boxed_slice(ENTROPY_CONTEXT_PLANES::default(), above_context_count)
                {
                    oci.above_context = Some(above_context_box);
                    return 0_i32;
                }
            }
        }
    }
    vp8_de_alloc_frame_buffers(oci);
    1_i32
}
pub fn vp8_setup_version(cm: &mut VP8_COMMON) {
    match cm.version {
        0 => {
            cm.no_lpf = 0_i32;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 0_i32;
            cm.full_pixel = 0_i32;
        }
        1 => {
            cm.no_lpf = 0_i32;
            cm.filter_type = SIMPLE_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1_i32;
            cm.full_pixel = 0_i32;
        }
        2 => {
            cm.no_lpf = 1_i32;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1_i32;
            cm.full_pixel = 0_i32;
        }
        3 => {
            cm.no_lpf = 1_i32;
            cm.filter_type = SIMPLE_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1_i32;
            cm.full_pixel = 1_i32;
        }
        _ => {
            cm.no_lpf = 0_i32;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 0_i32;
            cm.full_pixel = 0_i32;
        }
    };
}
pub fn vp8_create_common(oci: &mut VP8_COMMON) {
    vp8_machine_specific_config(oci);
    vp8_init_mbmode_probs(oci);
    vp8_default_bmode_probs(&mut oci.fc.bmode_prob);
    oci.mb_no_coeff_skip = 1_i32;
    oci.no_lpf = 0_i32;
    oci.filter_type = NORMAL_LOOPFILTER;
    oci.use_bilinear_mc_filter = 0_i32;
    oci.full_pixel = 0_i32;
    oci.multi_token_partition = ONE_PARTITION;
    oci.clamp_type = RECON_CLAMP_REQUIRED;
    oci.ref_frame_sign_bias = [0_i32; 4];
    oci.copy_buffer_to_gf = 0_i32;
    oci.copy_buffer_to_arf = 0_i32;
    oci.frame_to_show_idx = None;
}
pub fn vp8_remove_common(oci: &mut VP8_COMMON) {
    vp8_de_alloc_frame_buffers(oci);
}
