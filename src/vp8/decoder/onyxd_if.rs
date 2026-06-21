use crate::vp8::common::reconintra::vp8_init_intra_predictors;
use crate::vp8::common::vp8_loopfilter::vp8_loop_filter_init;
use crate::vp8::decoder::decodeframe::{vp8_decode_frame, vp8cx_init_de_quantizer};
use crate::vpx_dsp::vpx_dsp_rtcd::vpx_dsp_rtcd;
use std::sync::Once;

// setjmp import removed
pub use crate::vp8::common::alloccommon::{vp8_create_common, vp8_remove_common};
pub use crate::vp8::common::types::*;
use crate::vp8::decoder::threading::{vp8_decoder_create_threads, vp8_decoder_remove_threads};
pub use crate::vpx_scale::generic::yv12extend::vp8_yv12_copy_frame_c;
pub type uint32_t = u32;

pub type uint8_t = u8;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type __darwin_natural_t = u32;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type C2RustUnnamed = u32;
pub const MAX_REF_FRAMES: C2RustUnnamed = 4;
pub const ALTREF_FRAME: C2RustUnnamed = 3;
pub const GOLDEN_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_ppflags_t {
    pub post_proc_flag: i32,
    pub deblocking_level: i32,
    pub noise_level: i32,
    pub display_ref_frame_flag: i32,
    pub display_mb_modes_flag: i32,
    pub display_b_modes_flag: i32,
    pub display_mv_flag: i32,
}
pub type vpx_ref_frame_type = u32;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NUM_YV12_BUFFERS: i32 = 4_i32;

static INIT: Once = Once::new();

fn initialize_dec() {
    vpx_dsp_rtcd();
    vp8_init_intra_predictors();
}
fn remove_decompressor(mut pbi: Box<VP8D_COMP>) {
    vp8_remove_common(&mut pbi.common);
}
fn create_decompressor() -> Option<Box<VP8D_COMP>> {
    let mut pbi = Box::new(VP8D_COMP::default());

    vp8_create_common(&mut pbi.common);
    pbi.common.current_video_frame = 0;
    pbi.ready_for_new_data = 1;
    vp8cx_init_de_quantizer(&mut pbi);
    vp8_loop_filter_init(&mut pbi.common);
    pbi.ec_enabled = 0;
    pbi.ec_active = 0;
    pbi.decoded_key_frame = 0;
    pbi.independent_partitions = 0;
    INIT.call_once(initialize_dec);
    Some(pbi)
}
pub fn vp8dx_get_reference(
    pbi: &mut VP8D_COMP,
    ref_frame_flag: vpx_ref_frame_type,
    sd: &mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    let cm = &mut pbi.common;
    let mut ref_fb_idx: i32 = 0;
    if ref_frame_flag == VP8_LAST_FRAME as i32 as u32 {
        ref_fb_idx = cm.lst_fb_idx;
    } else if ref_frame_flag == VP8_GOLD_FRAME as i32 as u32 {
        ref_fb_idx = cm.gld_fb_idx;
    } else if ref_frame_flag == VP8_ALTR_FRAME as i32 as u32 {
        ref_fb_idx = cm.alt_fb_idx;
    } else {
        let _ = cm.error.trigger(VPX_CODEC_ERROR, "Invalid reference frame");
        return pbi.common.error.error_code;
    }
    if cm.yv12_fb[ref_fb_idx as usize].y_height != sd.y_height
        || cm.yv12_fb[ref_fb_idx as usize].y_width != sd.y_width
        || cm.yv12_fb[ref_fb_idx as usize].uv_height != sd.uv_height
        || cm.yv12_fb[ref_fb_idx as usize].uv_width != sd.uv_width
    {
        let _ = cm
            .error
            .trigger(VPX_CODEC_ERROR, "Incorrect buffer dimensions");
    } else {
        vp8_yv12_copy_frame_c(&cm.yv12_fb[ref_fb_idx as usize], sd);
    }
    pbi.common.error.error_code
}
pub fn vp8dx_set_reference(
    pbi: &mut VP8D_COMP,
    ref_frame_flag: vpx_ref_frame_type,
    sd: &YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    let cm = &mut pbi.common;

    enum TargetFrame {
        Last,
        Gold,
        Alt,
    }
    let target: TargetFrame;

    if ref_frame_flag == VP8_LAST_FRAME as i32 as u32 {
        target = TargetFrame::Last;
    } else if ref_frame_flag == VP8_GOLD_FRAME as i32 as u32 {
        target = TargetFrame::Gold;
    } else if ref_frame_flag == VP8_ALTR_FRAME as i32 as u32 {
        target = TargetFrame::Alt;
    } else {
        let _ = cm.error.trigger(VPX_CODEC_ERROR, "Invalid reference frame");
        return cm.error.error_code;
    }

    let ref_fb_idx = match target {
        TargetFrame::Last => cm.lst_fb_idx,
        TargetFrame::Gold => cm.gld_fb_idx,
        TargetFrame::Alt => cm.alt_fb_idx,
    };

    if cm.yv12_fb[ref_fb_idx as usize].y_height != sd.y_height
        || cm.yv12_fb[ref_fb_idx as usize].y_width != sd.y_width
        || cm.yv12_fb[ref_fb_idx as usize].uv_height != sd.uv_height
        || cm.yv12_fb[ref_fb_idx as usize].uv_width != sd.uv_width
    {
        let _ = cm
            .error
            .trigger(VPX_CODEC_ERROR, "Incorrect buffer dimensions");
    } else {
        let free_fb = get_free_fb(cm);
        let mut temp_idx = ref_fb_idx;

        cm.fb_idx_ref_cnt[free_fb as usize] -= 1;
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut temp_idx, free_fb);

        match target {
            TargetFrame::Last => cm.lst_fb_idx = temp_idx,
            TargetFrame::Gold => cm.gld_fb_idx = temp_idx,
            TargetFrame::Alt => cm.alt_fb_idx = temp_idx,
        }

        vp8_yv12_copy_frame_c(sd, &mut cm.yv12_fb[temp_idx as usize]);
    }
    cm.error.error_code
}
fn get_free_fb(cm: &mut VP8_COMMON) -> i32 {
    let mut i: i32 = 0;
    i = 0_i32;
    while i < NUM_YV12_BUFFERS {
        if cm.fb_idx_ref_cnt[i as usize] == 0_i32 {
            break;
        }
        i += 1;
    }
    cm.fb_idx_ref_cnt[i as usize] = 1_i32;
    i
}
fn ref_cnt_fb(buf: &mut [i32], idx: &mut i32, new_idx: i32) {
    if buf[*idx as usize] > 0_i32 {
        buf[*idx as usize] -= 1;
    }
    *idx = new_idx;
    buf[new_idx as usize] += 1;
}
fn swap_frame_buffers(cm: &mut VP8_COMMON) -> i32 {
    let mut err: i32 = 0_i32;
    if cm.copy_buffer_to_arf != 0 {
        let mut new_fb: i32 = 0_i32;
        if cm.copy_buffer_to_arf == 1_i32 {
            new_fb = cm.lst_fb_idx;
        } else if cm.copy_buffer_to_arf == 2_i32 {
            new_fb = cm.gld_fb_idx;
        } else {
            err = -1_i32;
        }
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut cm.alt_fb_idx, new_fb);
    }
    if cm.copy_buffer_to_gf != 0 {
        let mut new_fb_0: i32 = 0_i32;
        if cm.copy_buffer_to_gf == 1_i32 {
            new_fb_0 = cm.lst_fb_idx;
        } else if cm.copy_buffer_to_gf == 2_i32 {
            new_fb_0 = cm.alt_fb_idx;
        } else {
            err = -1_i32;
        }
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut cm.gld_fb_idx, new_fb_0);
    }
    if cm.refresh_golden_frame != 0 {
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut cm.gld_fb_idx, cm.new_fb_idx);
    }
    if cm.refresh_alt_ref_frame != 0 {
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut cm.alt_fb_idx, cm.new_fb_idx);
    }
    if cm.refresh_last_frame != 0 {
        ref_cnt_fb(&mut cm.fb_idx_ref_cnt, &mut cm.lst_fb_idx, cm.new_fb_idx);
        let lst_fb_idx = cm.lst_fb_idx as usize;
        cm.frame_to_show_idx = Some(lst_fb_idx);
    } else {
        let new_fb_idx = cm.new_fb_idx as usize;
        cm.frame_to_show_idx = Some(new_fb_idx);
    }
    cm.fb_idx_ref_cnt[cm.new_fb_idx as usize] -= 1;
    err
}
fn check_fragments_for_errors(pbi: &mut VP8D_COMP) -> i32 {
    if pbi.ec_active == 0 && pbi.fragments.count <= 1 && pbi.fragments.sizes[0] == 0 {
        let cm = &mut pbi.common;
        if cm.fb_idx_ref_cnt[cm.lst_fb_idx as usize] > 1 {
            let prev_idx = cm.lst_fb_idx as usize;
            cm.fb_idx_ref_cnt[prev_idx] -= 1;
            cm.lst_fb_idx = get_free_fb(cm);
            let new_idx = cm.lst_fb_idx as usize;
            if prev_idx < new_idx {
                let (first, second) = cm.yv12_fb.split_at_mut(new_idx);
                vp8_yv12_copy_frame_c(&first[prev_idx], &mut second[0]);
            } else {
                let (first, second) = cm.yv12_fb.split_at_mut(prev_idx);
                vp8_yv12_copy_frame_c(&second[0], &mut first[new_idx]);
            }
        }
        let lst_fb_idx = cm.lst_fb_idx as usize;
        cm.yv12_fb[lst_fb_idx].corrupted = 1;
        cm.show_frame = 0;
        return 0;
    }
    1
}
pub fn vp8dx_receive_compressed_data_safe(pbi: &mut VP8D_COMP) -> Result<i32, Vp8Bail> {
    let mut retcode: i32;
    pbi.common.error.error_code = VPX_CODEC_OK;

    retcode = check_fragments_for_errors(pbi);
    if retcode <= 0 {
        return Ok(retcode);
    }

    let new_fb_idx = get_free_fb(&mut pbi.common);
    pbi.common.new_fb_idx = new_fb_idx;

    // A bail from the frame decoder (corrupt bitstream / alloc failure)
    // propagates to the decode boundary, which runs the recovery path.
    retcode = vp8_decode_frame(pbi)?;

    if retcode < 0 {
        if pbi.common.fb_idx_ref_cnt[new_fb_idx as usize] > 0 {
            pbi.common.fb_idx_ref_cnt[new_fb_idx as usize] -= 1;
        }
        pbi.common.error.error_code = VPX_CODEC_ERROR;
        if pbi.mb.error_info.error_code != 0 {
            pbi.common.error.error_code = pbi.mb.error_info.error_code;
            pbi.common.error.detail = pbi.mb.error_info.detail;
        }
    } else if swap_frame_buffers(&mut pbi.common) != 0 {
        pbi.common.error.error_code = VPX_CODEC_ERROR;
    } else {
        if pbi.common.show_frame != 0 {
            pbi.common.current_video_frame = pbi.common.current_video_frame.wrapping_add(1);
        }
        pbi.ready_for_new_data = 0;
    }
    Ok(retcode)
}

pub fn vp8dx_get_raw_frame(pbi: &mut VP8D_COMP, sd: &mut YV12_BUFFER_CONFIG) -> i32 {
    let mut ret: i32 = -1_i32;
    if pbi.ready_for_new_data == 1_i32 {
        return ret;
    }
    if pbi.common.show_frame == 0_i32 {
        return ret;
    }
    pbi.ready_for_new_data = 1_i32;
    if let Some(idx) = pbi.common.frame_to_show_idx {
        *sd = pbi.common.yv12_fb[idx];
        sd.y_width = pbi.common.Width;
        sd.y_height = pbi.common.Height;
        sd.uv_height = pbi.common.Height / 2_i32;
        ret = 0_i32;
    } else {
        ret = -1_i32;
    }
    ret
}
pub fn vp8dx_references_buffer(oci: &VP8_COMMON, mip_slice: &[MODE_INFO], ref_frame: i32) -> i32 {
    let stride = oci.mode_info_stride as usize;
    let mut cur_idx = stride + 1;
    let mut mb_row = 0;
    while mb_row < oci.mb_rows {
        let mut mb_col = 0;
        while mb_col < oci.mb_cols {
            if mip_slice[cur_idx].mbmi.ref_frame as i32 == ref_frame {
                return 1_i32;
            }
            mb_col += 1;
            cur_idx += 1;
        }
        cur_idx += 1;
        mb_row += 1;
    }
    0_i32
}
pub fn vp8_create_decoder_instances(fb: &mut frame_buffers, oxcf: &VP8D_CONFIG) -> i32 {
    let mut pbi = match create_decompressor() {
        Some(p) => p,
        None => return VPX_CODEC_ERROR as i32,
    };
    pbi.max_threads = oxcf.max_threads;

    if let Err(detail) = vp8_decoder_create_threads(&mut pbi) {
        pbi.common.error.error_code = VPX_CODEC_MEM_ERROR;
        pbi.common.error.has_detail = 1;
        let bytes = detail.as_bytes();
        let len = std::cmp::min(bytes.len(), pbi.common.error.detail.len() - 1);
        for i in 0..len {
            pbi.common.error.detail[i] = bytes[i] as ::core::ffi::c_char;
        }
        pbi.common.error.detail[len] = 0;

        vp8_decoder_remove_threads(&mut pbi);
        remove_decompressor(pbi);
        fb.pbi = [::core::ptr::null_mut(); 32];
        return VPX_CODEC_ERROR as i32;
    }

    fb.pbi[0] = Box::into_raw(pbi);
    VPX_CODEC_OK as i32
}
pub fn vp8_remove_decoder_instances(fb: &mut frame_buffers) -> i32 {
    let pbi: *mut VP8D_COMP = fb.pbi[0];
    if pbi.is_null() {
        return VPX_CODEC_ERROR as i32;
    }
    // SAFETY: We safely reclaim exclusive ownership of the heap-allocated decoder context
    // from the raw pointer stored in `fb.pbi[0]` (which was previously created via `Box::into_raw`
    // during `vp8_create_decoder_instances`).
    let mut decompressor = unsafe { Box::from_raw(pbi) };
    vp8_decoder_remove_threads(&mut decompressor);
    remove_decompressor(decompressor);
    fb.pbi[0] = ::core::ptr::null_mut::<VP8D_COMP>();
    VPX_CODEC_OK as i32
}
pub fn vp8dx_get_quantizer(pbi: &VP8D_COMP) -> i32 {
    pbi.common.base_qindex
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
