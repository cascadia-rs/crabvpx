// FFI imports removed
use crate::vp8::common::entropymode::{vp8_default_bmode_probs, vp8_init_mbmode_probs};
use crate::vp8::common::generic::systemdependent::vp8_machine_specific_config;
use crate::vpx_scale::generic::yv12config::{
    vp8_yv12_alloc_frame_buffer_safe, vp8_yv12_de_alloc_frame_buffer_safe,
};

pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

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
    pub has_detail: ::core::ffi::c_int,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: ::core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [::core::ffi::c_int; 48];
pub type vpx_codec_err_t = ::core::ffi::c_uint;
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

pub type FRAME_TYPE = ::core::ffi::c_uint;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;

pub type uint8_t = u8;
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const VP8BORDERINPIXELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NUM_YV12_BUFFERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub fn vp8_de_alloc_frame_buffers(oci: &mut VP8_COMMON) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_YV12_BUFFERS {
        let (fb, alloc) = (
            &mut oci.yv12_fb[i as usize],
            &mut oci.yv12_fb_allocs[i as usize],
        );
        vp8_yv12_de_alloc_frame_buffer_safe(fb, alloc);
        oci.fb_idx_ref_cnt[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    let (temp_fb, temp_alloc) = (&mut oci.temp_scale_frame, &mut oci.temp_scale_frame_alloc);
    vp8_yv12_de_alloc_frame_buffer_safe(temp_fb, temp_alloc);

    oci.above_context = None;
    oci.mip = None;
    oci.frame_to_show_idx = None;
}

pub fn vp8_alloc_frame_buffers(
    oci: &mut VP8_COMMON,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    vp8_de_alloc_frame_buffers(oci);
    if width & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        width += 16 as ::core::ffi::c_int - (width & 0xf as ::core::ffi::c_int);
    }
    if height & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        height += 16 as ::core::ffi::c_int - (height & 0xf as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
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
        oci.new_fb_idx = 0 as ::core::ffi::c_int;
        oci.lst_fb_idx = 1 as ::core::ffi::c_int;
        oci.gld_fb_idx = 2 as ::core::ffi::c_int;
        oci.alt_fb_idx = 3 as ::core::ffi::c_int;
        oci.fb_idx_ref_cnt[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        oci.fb_idx_ref_cnt[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        oci.fb_idx_ref_cnt[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        oci.fb_idx_ref_cnt[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        let (temp_fb, temp_alloc) = (&mut oci.temp_scale_frame, &mut oci.temp_scale_frame_alloc);
        if vp8_yv12_alloc_frame_buffer_safe(
            temp_fb,
            width,
            16 as ::core::ffi::c_int,
            VP8BORDERINPIXELS,
            temp_alloc,
        )
        .is_ok()
        {
            oci.mb_rows = height >> 4 as ::core::ffi::c_int;
            oci.mb_cols = width >> 4 as ::core::ffi::c_int;
            oci.MBs = oci.mb_rows * oci.mb_cols;
            oci.mode_info_stride = oci.mb_cols + 1 as ::core::ffi::c_int;
            let mip_count = ((oci.mb_cols + 1) * (oci.mb_rows + 1)) as usize;
            if let Some(mip_box) = try_alloc_boxed_slice(MODE_INFO::default(), mip_count) {
                oci.mip = Some(mip_box);
                let above_context_count = oci.mb_cols as usize;
                if let Some(above_context_box) =
                    try_alloc_boxed_slice(ENTROPY_CONTEXT_PLANES::default(), above_context_count)
                {
                    oci.above_context = Some(above_context_box);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    vp8_de_alloc_frame_buffers(oci);
    1 as ::core::ffi::c_int
}
pub fn vp8_setup_version(cm: &mut VP8_COMMON) {
    match cm.version {
        0 => {
            cm.no_lpf = 0 as ::core::ffi::c_int;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
            cm.full_pixel = 0 as ::core::ffi::c_int;
        }
        1 => {
            cm.no_lpf = 0 as ::core::ffi::c_int;
            cm.filter_type = SIMPLE_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
            cm.full_pixel = 0 as ::core::ffi::c_int;
        }
        2 => {
            cm.no_lpf = 1 as ::core::ffi::c_int;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
            cm.full_pixel = 0 as ::core::ffi::c_int;
        }
        3 => {
            cm.no_lpf = 1 as ::core::ffi::c_int;
            cm.filter_type = SIMPLE_LOOPFILTER;
            cm.use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
            cm.full_pixel = 1 as ::core::ffi::c_int;
        }
        _ => {
            cm.no_lpf = 0 as ::core::ffi::c_int;
            cm.filter_type = NORMAL_LOOPFILTER;
            cm.use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
            cm.full_pixel = 0 as ::core::ffi::c_int;
        }
    };
}
pub fn vp8_create_common(oci: &mut VP8_COMMON) {
    vp8_machine_specific_config(oci);
    vp8_init_mbmode_probs(oci);
    vp8_default_bmode_probs(&mut oci.fc.bmode_prob);
    oci.mb_no_coeff_skip = 1 as ::core::ffi::c_int;
    oci.no_lpf = 0 as ::core::ffi::c_int;
    oci.filter_type = NORMAL_LOOPFILTER;
    oci.use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
    oci.full_pixel = 0 as ::core::ffi::c_int;
    oci.multi_token_partition = ONE_PARTITION;
    oci.clamp_type = RECON_CLAMP_REQUIRED;
    oci.ref_frame_sign_bias = [0 as ::core::ffi::c_int; 4];
    oci.copy_buffer_to_gf = 0 as ::core::ffi::c_int;
    oci.copy_buffer_to_arf = 0 as ::core::ffi::c_int;
    oci.frame_to_show_idx = None;
}
pub fn vp8_remove_common(oci: &mut VP8_COMMON) {
    vp8_de_alloc_frame_buffers(oci);
}
