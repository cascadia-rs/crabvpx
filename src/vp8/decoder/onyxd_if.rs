use crate::vp8::decoder::decodeframe::{vp8cx_init_de_quantizer, vp8_decode_frame};
use crate::vp8::common::vp8_loopfilter::vp8_loop_filter_init;
use std::sync::Once;
use crate::vpx_dsp::vpx_dsp_rtcd::vpx_dsp_rtcd;
use crate::vp8::common::reconintra::vp8_init_intra_predictors;

unsafe extern "C" {
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
}
use crate::vp8::decoder::threading::{vp8_decoder_create_threads, vp8_decoder_remove_threads};
pub use crate::vp8::common::alloccommon::{vp8_create_common, vp8_remove_common};
pub use crate::vp8::common::types::*;
pub use crate::vpx_scale::generic::yv12extend::vp8_yv12_copy_frame_c;
pub type uint32_t = u32;

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
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: C2RustUnnamed = 4;
pub const ALTREF_FRAME: C2RustUnnamed = 3;
pub const GOLDEN_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_ppflags_t {
    pub post_proc_flag: ::core::ffi::c_int,
    pub deblocking_level: ::core::ffi::c_int,
    pub noise_level: ::core::ffi::c_int,
    pub display_ref_frame_flag: ::core::ffi::c_int,
    pub display_mb_modes_flag: ::core::ffi::c_int,
    pub display_b_modes_flag: ::core::ffi::c_int,
    pub display_mv_flag: ::core::ffi::c_int,
}
pub type vpx_ref_frame_type = ::core::ffi::c_uint;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NUM_YV12_BUFFERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

static INIT: Once = Once::new();

fn initialize_dec() {
    vpx_dsp_rtcd();
    vp8_init_intra_predictors();
}
fn remove_decompressor(mut pbi: Box<VP8D_COMP>) {
    vp8_remove_common(&mut pbi.common);
}
fn create_decompressor() -> Option<Box<VP8D_COMP>> {
    let pbi = match Box::try_new(VP8D_COMP::default()) {
        Ok(b) => b,
        Err(_) => return None,
    };
    
    let mut pbi = std::mem::ManuallyDrop::new(pbi);
    let pbi_mut = &mut *pbi;
    
    let setjmp_val = unsafe { setjmp(&raw mut pbi_mut.common.error.jmp as *mut ::core::ffi::c_int) };
    if setjmp_val != 0 {
        let mut pbi_box = std::mem::ManuallyDrop::into_inner(pbi);
        pbi_box.common.error.setjmp = 0;
        return None;
    }
    pbi_mut.common.error.setjmp = 1;
    vp8_create_common(&mut pbi_mut.common);
    pbi_mut.common.current_video_frame = 0;
    pbi_mut.ready_for_new_data = 1;
    vp8cx_init_de_quantizer(pbi_mut);
    vp8_loop_filter_init(&mut pbi_mut.common);
    pbi_mut.common.error.setjmp = 0;
    pbi_mut.ec_enabled = 0;
    pbi_mut.ec_active = 0;
    pbi_mut.decoded_key_frame = 0;
    pbi_mut.independent_partitions = 0;
    INIT.call_once(initialize_dec);
    Some(std::mem::ManuallyDrop::into_inner(pbi))
}
pub fn vp8dx_get_reference(
    pbi: &mut VP8D_COMP,
    ref_frame_flag: vpx_ref_frame_type,
    sd: &mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t {
    let cm = &mut pbi.common;
    let mut ref_fb_idx: ::core::ffi::c_int = 0;
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = cm.lst_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = cm.gld_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = cm.alt_fb_idx;
    } else {
        cm.error.trigger(VPX_CODEC_ERROR, "Invalid reference frame");
        return pbi.common.error.error_code;
    }
    if cm.yv12_fb[ref_fb_idx as usize].y_height != sd.y_height
        || cm.yv12_fb[ref_fb_idx as usize].y_width != sd.y_width
        || cm.yv12_fb[ref_fb_idx as usize].uv_height != sd.uv_height
        || cm.yv12_fb[ref_fb_idx as usize].uv_width != sd.uv_width
    {
        cm.error.trigger(VPX_CODEC_ERROR, "Incorrect buffer dimensions");
    } else {
        vp8_yv12_copy_frame_c(
            &cm.yv12_fb[ref_fb_idx as usize],
            sd,
        );
    }
    return pbi.common.error.error_code;
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
    
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        target = TargetFrame::Last;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        target = TargetFrame::Gold;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        target = TargetFrame::Alt;
    } else {
        cm.error.trigger(VPX_CODEC_ERROR, "Invalid reference frame");
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
        cm.error.trigger(VPX_CODEC_ERROR, "Incorrect buffer dimensions");
    } else {
        let free_fb = get_free_fb(cm);
        let mut temp_idx = ref_fb_idx;
        
        cm.fb_idx_ref_cnt[free_fb as usize] -= 1;
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut temp_idx,
            free_fb,
        );
        
        match target {
            TargetFrame::Last => cm.lst_fb_idx = temp_idx,
            TargetFrame::Gold => cm.gld_fb_idx = temp_idx,
            TargetFrame::Alt => cm.alt_fb_idx = temp_idx,
        }
        
        vp8_yv12_copy_frame_c(
            sd,
            &mut cm.yv12_fb[temp_idx as usize],
        );
    }
    return cm.error.error_code;
}
fn get_free_fb(cm: &mut VP8_COMMON) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_YV12_BUFFERS {
        if cm.fb_idx_ref_cnt[i as usize] == 0 as ::core::ffi::c_int {
            break;
        }
        i += 1;
    }
    cm.fb_idx_ref_cnt[i as usize] = 1 as ::core::ffi::c_int;
    return i;
}
fn ref_cnt_fb(
    buf: &mut [::core::ffi::c_int],
    idx: &mut ::core::ffi::c_int,
    new_idx: ::core::ffi::c_int,
) {
    if buf[*idx as usize] > 0 as ::core::ffi::c_int {
        buf[*idx as usize] -= 1;
    }
    *idx = new_idx;
    buf[new_idx as usize] += 1;
}
fn swap_frame_buffers(cm: &mut VP8_COMMON) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cm.copy_buffer_to_arf != 0 {
        let mut new_fb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if cm.copy_buffer_to_arf == 1 as ::core::ffi::c_int {
            new_fb = cm.lst_fb_idx;
        } else if cm.copy_buffer_to_arf == 2 as ::core::ffi::c_int {
            new_fb = cm.gld_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut cm.alt_fb_idx,
            new_fb,
        );
    }
    if cm.copy_buffer_to_gf != 0 {
        let mut new_fb_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if cm.copy_buffer_to_gf == 1 as ::core::ffi::c_int {
            new_fb_0 = cm.lst_fb_idx;
        } else if cm.copy_buffer_to_gf == 2 as ::core::ffi::c_int {
            new_fb_0 = cm.alt_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut cm.gld_fb_idx,
            new_fb_0,
        );
    }
    if cm.refresh_golden_frame != 0 {
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut cm.gld_fb_idx,
            cm.new_fb_idx,
        );
    }
    if cm.refresh_alt_ref_frame != 0 {
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut cm.alt_fb_idx,
            cm.new_fb_idx,
        );
    }
    if cm.refresh_last_frame != 0 {
        ref_cnt_fb(
            &mut cm.fb_idx_ref_cnt,
            &mut cm.lst_fb_idx,
            cm.new_fb_idx,
        );
        let lst_fb_idx = cm.lst_fb_idx as usize;
        cm.frame_to_show_idx = Some(lst_fb_idx);
    } else {
        let new_fb_idx = cm.new_fb_idx as usize;
        cm.frame_to_show_idx = Some(new_fb_idx);
    }
    cm.fb_idx_ref_cnt[cm.new_fb_idx as usize] -= 1;
    return err;
}
fn check_fragments_for_errors(pbi: &mut VP8D_COMP) -> ::core::ffi::c_int {
    if pbi.ec_active == 0
        && pbi.fragments.count <= 1
        && pbi.fragments.sizes[0] == 0
    {
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
    return 1;
}
pub fn vp8dx_receive_compressed_data_safe(
    pbi: &mut VP8D_COMP,
) -> ::core::ffi::c_int {
    let mut retcode: ::core::ffi::c_int = -1;
    pbi.common.error.error_code = VPX_CODEC_OK;
    
    retcode = check_fragments_for_errors(pbi);
    if retcode <= 0 {
        return retcode;
    }
    
    let new_fb_idx = get_free_fb(&mut pbi.common);
    pbi.common.new_fb_idx = new_fb_idx;
    
    retcode = vp8_decode_frame(pbi);
    
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
    return retcode;
}

pub fn vp8dx_get_raw_frame(
    pbi: &mut VP8D_COMP,
    sd: &mut YV12_BUFFER_CONFIG,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if pbi.ready_for_new_data == 1 as ::core::ffi::c_int {
        return ret;
    }
    if pbi.common.show_frame == 0 as ::core::ffi::c_int {
        return ret;
    }
    pbi.ready_for_new_data = 1 as ::core::ffi::c_int;
    if let Some(idx) = pbi.common.frame_to_show_idx {
        *sd = pbi.common.yv12_fb[idx];
        sd.y_width = pbi.common.Width;
        sd.y_height = pbi.common.Height;
        sd.uv_height = pbi.common.Height / 2 as ::core::ffi::c_int;
        ret = 0 as ::core::ffi::c_int;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
pub fn vp8dx_references_buffer(
    oci: &VP8_COMMON,
    mip_slice: &[MODE_INFO],
    ref_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let stride = oci.mode_info_stride as usize;
    let mut cur_idx = stride + 1;
    let mut mb_row = 0;
    while mb_row < oci.mb_rows {
        let mut mb_col = 0;
        while mb_col < oci.mb_cols {
            if mip_slice[cur_idx].mbmi.ref_frame as ::core::ffi::c_int == ref_frame {
                return 1 as ::core::ffi::c_int;
            }
            mb_col += 1;
            cur_idx += 1;
        }
        cur_idx += 1;
        mb_row += 1;
    }
    return 0 as ::core::ffi::c_int;
}
pub fn vp8_create_decoder_instances(
    fb: &mut frame_buffers,
    oxcf: &VP8D_CONFIG,
) -> ::core::ffi::c_int {
    let pbi = match create_decompressor() {
        Some(p) => p,
        None => return VPX_CODEC_ERROR as ::core::ffi::c_int,
    };
    fb.pbi[0] = Box::into_raw(pbi);
    let pbi_raw = fb.pbi[0];
    unsafe {
        if setjmp(&raw mut (*pbi_raw).common.error.jmp as *mut ::core::ffi::c_int) != 0 {
            (*pbi_raw).common.error.setjmp = 0;
            vp8_remove_decoder_instances(fb);
            fb.pbi = [::core::ptr::null_mut(); 32];
            return VPX_CODEC_ERROR as ::core::ffi::c_int;
        }
        (*pbi_raw).common.error.setjmp = 1;
        (*pbi_raw).max_threads = oxcf.max_threads;
        vp8_decoder_create_threads(&mut *pbi_raw);
        (*pbi_raw).common.error.setjmp = 0;
    }
    return VPX_CODEC_OK as ::core::ffi::c_int;
}
pub fn vp8_remove_decoder_instances(
    fb: &mut frame_buffers,
) -> ::core::ffi::c_int {
    let pbi: *mut VP8D_COMP = fb.pbi[0];
    if pbi.is_null() {
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    unsafe {
        vp8_decoder_remove_threads(&mut *pbi);
        remove_decompressor(Box::from_raw(pbi));
    }
    fb.pbi[0] = ::core::ptr::null_mut::<VP8D_COMP>();
    return VPX_CODEC_OK as ::core::ffi::c_int;
}
pub fn vp8dx_get_quantizer(pbi: &VP8D_COMP) -> ::core::ffi::c_int {
    return pbi.common.base_qindex;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
