unsafe extern "C" {
    fn vp8_loop_filter_bh_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bv_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbh_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbv_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bhs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_bvs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_mbhs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_mbvs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub use crate::vp8::common::types::*;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const ZEROMV: C2RustUnnamed = 7;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub const MAX_REF_FRAMES: C2RustUnnamed_1 = 4;
pub const INTRA_FRAME: C2RustUnnamed_1 = 0;
pub const MB_LVL_ALT_LF: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MB_LVL_MAX: C2RustUnnamed_0 = 2;
pub const MB_LVL_ALT_Q: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 3;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const MAX_LOOP_FILTER: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const PARTIAL_FRAME_FRACTION: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIMD_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_MB_SEGMENTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEGMENT_ABSDATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
fn lf_init_lut(lfi: &mut loop_filter_info_n) {
    let mut filt_lvl: ::core::ffi::c_int = 0;
    filt_lvl = 0 as ::core::ffi::c_int;
    while filt_lvl <= MAX_LOOP_FILTER {
        if filt_lvl >= 40 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                2 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                3 as ::core::ffi::c_uchar;
        } else if filt_lvl >= 20 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                2 as ::core::ffi::c_uchar;
        } else if filt_lvl >= 15 as ::core::ffi::c_int {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                1 as ::core::ffi::c_uchar;
        } else {
            lfi.hev_thr_lut[KEY_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                0 as ::core::ffi::c_uchar;
            lfi.hev_thr_lut[INTER_FRAME as ::core::ffi::c_int as usize][filt_lvl as usize] =
                0 as ::core::ffi::c_uchar;
        }
        filt_lvl += 1;
    }
    lfi.mode_lf_lut[DC_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[V_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[H_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[TM_PRED as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[B_PRED as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[ZEROMV as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEARESTMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEARMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[NEWMV as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_uchar;
    lfi.mode_lf_lut[SPLITMV as ::core::ffi::c_int as usize] = 3 as ::core::ffi::c_uchar;
}
#[unsafe(no_mangle)]
pub fn vp8_loop_filter_update_sharpness(
    lfi: &mut loop_filter_info_n,
    sharpness_lvl: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= MAX_LOOP_FILTER {
        let mut filt_lvl: ::core::ffi::c_int = i;
        let mut block_inside_limit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        block_inside_limit =
            filt_lvl >> (sharpness_lvl > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        block_inside_limit =
            block_inside_limit >> (sharpness_lvl > 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if sharpness_lvl > 0 as ::core::ffi::c_int {
            if block_inside_limit > 9 as ::core::ffi::c_int - sharpness_lvl {
                block_inside_limit = 9 as ::core::ffi::c_int - sharpness_lvl;
            }
        }
        if block_inside_limit < 1 as ::core::ffi::c_int {
            block_inside_limit = 1 as ::core::ffi::c_int;
        }
        lfi.lim[i as usize] = [block_inside_limit as u8; 1];
        lfi.blim[i as usize] = [(2 * filt_lvl + block_inside_limit) as u8; 1];
        lfi.mblim[i as usize] = [(2 * (filt_lvl + 2) + block_inside_limit) as u8; 1];
        i += 1;
    }
}
pub fn vp8_loop_filter_init(cm: &mut VP8_COMMON) {
    vp8_loop_filter_update_sharpness(&mut cm.lf_info, cm.sharpness_level);
    cm.last_sharpness_level = cm.sharpness_level;
    lf_init_lut(&mut cm.lf_info);
    for i in 0..4 {
        cm.lf_info.hev_thr[i] = [i as u8; 1];
    }
}
#[unsafe(no_mangle)]
pub fn vp8_loop_filter_frame_init(
    cm: &mut VP8_COMMON,
    mbd: &MACROBLOCKD,
    default_filt_lvl: ::core::ffi::c_int,
) {
    let mut seg: ::core::ffi::c_int = 0;
    let mut ref_0: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let lfi: &mut loop_filter_info_n = &mut cm.lf_info;
    if cm.last_sharpness_level != cm.sharpness_level {
        vp8_loop_filter_update_sharpness(lfi, cm.sharpness_level);
        cm.last_sharpness_level = cm.sharpness_level;
    }
    seg = 0 as ::core::ffi::c_int;
    while seg < MAX_MB_SEGMENTS {
        let mut lvl_seg: ::core::ffi::c_int = default_filt_lvl;
        let mut lvl_ref: ::core::ffi::c_int = 0;
        let mut lvl_mode: ::core::ffi::c_int = 0;
        if mbd.segmentation_enabled != 0 {
            if mbd.mb_segment_abs_delta as ::core::ffi::c_int == SEGMENT_ABSDATA {
                lvl_seg = mbd.segment_feature_data[MB_LVL_ALT_LF as ::core::ffi::c_int as usize]
                    [seg as usize] as ::core::ffi::c_int;
            } else {
                lvl_seg += mbd.segment_feature_data[MB_LVL_ALT_LF as ::core::ffi::c_int as usize]
                    [seg as usize] as ::core::ffi::c_int;
            }
            lvl_seg = if lvl_seg > 0 as ::core::ffi::c_int {
                if lvl_seg > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_seg
                }
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if mbd.mode_ref_lf_delta_enabled == 0 {
            lfi.lvl[seg as usize] = [[lvl_seg as u8; 4]; 4];
        } else {
            ref_0 = INTRA_FRAME as ::core::ffi::c_int;
            lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
            mode = 0 as ::core::ffi::c_int;
            lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
            lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                if lvl_mode > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_mode
                }
            } else {
                0 as ::core::ffi::c_int
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                lvl_mode as ::core::ffi::c_uchar;
            mode = 1 as ::core::ffi::c_int;
            lvl_mode = if lvl_ref > 0 as ::core::ffi::c_int {
                if lvl_ref > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    lvl_ref
                }
            } else {
                0 as ::core::ffi::c_int
            };
            lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                lvl_mode as ::core::ffi::c_uchar;
            ref_0 = 1 as ::core::ffi::c_int;
            while ref_0 < MAX_REF_FRAMES as ::core::ffi::c_int {
                lvl_ref = lvl_seg + mbd.ref_lf_deltas[ref_0 as usize] as ::core::ffi::c_int;
                mode = 1 as ::core::ffi::c_int;
                while mode < 4 as ::core::ffi::c_int {
                    lvl_mode = lvl_ref + mbd.mode_lf_deltas[mode as usize] as ::core::ffi::c_int;
                    lvl_mode = if lvl_mode > 0 as ::core::ffi::c_int {
                        if lvl_mode > 63 as ::core::ffi::c_int {
                            63 as ::core::ffi::c_int
                        } else {
                            lvl_mode
                        }
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    lfi.lvl[seg as usize][ref_0 as usize][mode as usize] =
                        lvl_mode as ::core::ffi::c_uchar;
                    mode += 1;
                }
                ref_0 += 1;
            }
        }
        seg += 1;
    }
}
pub fn vp8_loop_filter_row_normal_safe(
    mb_cols: ::core::ffi::c_int,
    lf_info: &loop_filter_info_n,
    frame_type: FRAME_TYPE,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: ::core::ffi::c_int,
    post_ystride: ::core::ffi::c_int,
    post_uvstride: ::core::ffi::c_int,
    y_slice: &mut [u8],
    y_offset: usize,
    u_slice: &mut [u8],
    u_offset: usize,
    v_slice: &mut [u8],
    v_offset: usize,
) {
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut filter_level: ::core::ffi::c_int = 0;
    let lfi_n = lf_info;
    let mut lfi: loop_filter_info = loop_filter_info {
        mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
    };
    mb_col = 0 as ::core::ffi::c_int;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;
    let mut cur_u_offset = u_offset;
    let mut cur_v_offset = v_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let mut skip_lf: ::core::ffi::c_int = (mi.mbmi.mode as ::core::ffi::c_int
            != B_PRED as ::core::ffi::c_int
            && mi.mbmi.mode as ::core::ffi::c_int
                != SPLITMV as ::core::ffi::c_int
            && mi.mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        let mode_index: ::core::ffi::c_int =
            lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as ::core::ffi::c_int;
        let seg: ::core::ffi::c_int = mi.mbmi.segment_id as ::core::ffi::c_int;
        let ref_frame: ::core::ffi::c_int =
            mi.mbmi.ref_frame as ::core::ffi::c_int;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize]
            as ::core::ffi::c_int;
        if filter_level != 0 {
            let hev_index: ::core::ffi::c_int = lfi_n.hev_thr_lut[frame_type as usize]
                [filter_level as usize]
                as ::core::ffi::c_int;
            
            lfi.mblim = lfi_n.mblim[filter_level as usize].as_ptr();
            lfi.blim = lfi_n.blim[filter_level as usize].as_ptr();
            lfi.lim = lfi_n.lim[filter_level as usize].as_ptr();
            lfi.hev_thr = lfi_n.hev_thr[hev_index as usize].as_ptr();

            let (y_ptr, u_ptr, v_ptr) = unsafe {
                (
                    y_slice.as_mut_ptr().add(cur_y_offset),
                    u_slice.as_mut_ptr().add(cur_u_offset),
                    v_slice.as_mut_ptr().add(cur_v_offset),
                )
            };

            unsafe {
                if mb_col > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_mbv_neon(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bv_neon(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if mb_row > 0 as ::core::ffi::c_int {
                    vp8_loop_filter_mbh_neon(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
                if skip_lf == 0 {
                    vp8_loop_filter_bh_neon(
                        y_ptr,
                        u_ptr,
                        v_ptr,
                        post_ystride,
                        post_uvstride,
                        &raw mut lfi,
                    );
                }
            }
        }
        cur_y_offset = (cur_y_offset as isize + 16 as isize) as usize;
        cur_u_offset = (cur_u_offset as isize + 8 as isize) as usize;
        cur_v_offset = (cur_v_offset as isize + 8 as isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
pub fn vp8_loop_filter_row_simple_safe(
    mb_cols: ::core::ffi::c_int,
    lf_info: &loop_filter_info_n,
    mode_info_slice: &[MODE_INFO],
    mode_info_idx: usize,
    mb_row: ::core::ffi::c_int,
    post_ystride: ::core::ffi::c_int,
    y_slice: &mut [u8],
    y_offset: usize,
) {
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut filter_level: ::core::ffi::c_int = 0;
    let lfi_n = lf_info;
    mb_col = 0 as ::core::ffi::c_int;
    let mut cur_mi_idx = mode_info_idx;
    let mut cur_y_offset = y_offset;

    while mb_col < mb_cols {
        let mi = &mode_info_slice[cur_mi_idx];
        let mut skip_lf: ::core::ffi::c_int = (mi.mbmi.mode as ::core::ffi::c_int
            != B_PRED as ::core::ffi::c_int
            && mi.mbmi.mode as ::core::ffi::c_int
                != SPLITMV as ::core::ffi::c_int
            && mi.mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        let mode_index: ::core::ffi::c_int =
            lfi_n.mode_lf_lut[mi.mbmi.mode as usize] as ::core::ffi::c_int;
        let seg: ::core::ffi::c_int = mi.mbmi.segment_id as ::core::ffi::c_int;
        let ref_frame: ::core::ffi::c_int =
            mi.mbmi.ref_frame as ::core::ffi::c_int;
        filter_level = lfi_n.lvl[seg as usize][ref_frame as usize][mode_index as usize]
            as ::core::ffi::c_int;
        if filter_level != 0 {
            let y_ptr = unsafe { y_slice.as_mut_ptr().add(cur_y_offset) };
            let mblim_ptr = lfi_n.mblim[filter_level as usize].as_ptr();
            let blim_ptr = lfi_n.blim[filter_level as usize].as_ptr();

            if mb_col > 0 as ::core::ffi::c_int {
                unsafe {
                    vp8_loop_filter_mbvs_neon(
                        y_ptr,
                        post_ystride,
                        mblim_ptr as *mut ::core::ffi::c_uchar,
                    );
                }
            }
            if skip_lf == 0 {
                unsafe {
                    vp8_loop_filter_bvs_neon(
                        y_ptr,
                        post_ystride,
                        blim_ptr as *mut ::core::ffi::c_uchar,
                    );
                }
            }
            if mb_row > 0 as ::core::ffi::c_int {
                unsafe {
                    vp8_loop_filter_mbhs_neon(
                        y_ptr,
                        post_ystride,
                        mblim_ptr as *mut ::core::ffi::c_uchar,
                    );
                }
            }
            if skip_lf == 0 {
                unsafe {
                    vp8_loop_filter_bhs_neon(
                        y_ptr,
                        post_ystride,
                        blim_ptr as *mut ::core::ffi::c_uchar,
                    );
                }
            }
        }
        cur_y_offset = (cur_y_offset as isize + 16 as isize) as usize;
        cur_mi_idx += 1;
        mb_col += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_frame(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut frame_type: ::core::ffi::c_int,
) { unsafe {
    let post_idx = (*cm).frame_to_show_idx.expect("frame_to_show_idx is None");
    let mut post: *mut YV12_BUFFER_CONFIG = &raw mut (*cm).yv12_fb[post_idx];
    let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
    let mut lfi: loop_filter_info = loop_filter_info {
        mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
    };
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut mb_rows: ::core::ffi::c_int = (*cm).mb_rows;
    let mut mb_cols: ::core::ffi::c_int = (*cm).mb_cols;
    let mut filter_level: ::core::ffi::c_int = 0;
    let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut u_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut v_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut mode_info_context: *const MODE_INFO = (*cm).mi;
    let mut post_y_stride: ::core::ffi::c_int = (*post).y_stride;
    let mut post_uv_stride: ::core::ffi::c_int = (*post).uv_stride;
    filter_level = (*cm).filter_level;
    vp8_loop_filter_frame_init(&mut *cm, &*mbd, filter_level);
    y_ptr = (*post).y_buffer as *mut ::core::ffi::c_uchar;
    u_ptr = (*post).u_buffer as *mut ::core::ffi::c_uchar;
    v_ptr = (*post).v_buffer as *mut ::core::ffi::c_uchar;
    if (*cm).filter_type as ::core::ffi::c_uint
        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mb_row = 0 as ::core::ffi::c_int;
        while mb_row < mb_rows {
            mb_col = 0 as ::core::ffi::c_int;
            while mb_col < mb_cols {
                let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                    as ::core::ffi::c_int
                    != B_PRED as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                        != SPLITMV as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                let mode_index: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                    [(*mode_info_context).mbmi.mode as usize]
                    as ::core::ffi::c_int;
                let seg: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                let ref_frame: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                    as ::core::ffi::c_int;
                if filter_level != 0 {
                    let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut[frame_type as usize]
                        [filter_level as usize]
                        as ::core::ffi::c_int;
                    lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(hev_index as isize)
                        as *mut ::core::ffi::c_uchar;
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbv_neon(
                            y_ptr,
                            u_ptr,
                            v_ptr,
                            post_y_stride,
                            post_uv_stride,
                            &raw mut lfi,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bv_neon(
                            y_ptr,
                            u_ptr,
                            v_ptr,
                            post_y_stride,
                            post_uv_stride,
                            &raw mut lfi,
                        );
                    }
                    if mb_row > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbh_neon(
                            y_ptr,
                            u_ptr,
                            v_ptr,
                            post_y_stride,
                            post_uv_stride,
                            &raw mut lfi,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bh_neon(
                            y_ptr,
                            u_ptr,
                            v_ptr,
                            post_y_stride,
                            post_uv_stride,
                            &raw mut lfi,
                        );
                    }
                }
                y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                u_ptr = u_ptr.offset(8 as ::core::ffi::c_int as isize);
                v_ptr = v_ptr.offset(8 as ::core::ffi::c_int as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_col += 1;
            }
            y_ptr =
                y_ptr.offset((post_y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
            u_ptr = u_ptr
                .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
            v_ptr = v_ptr
                .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_row += 1;
        }
    } else {
        mb_row = 0 as ::core::ffi::c_int;
        while mb_row < mb_rows {
            mb_col = 0 as ::core::ffi::c_int;
            while mb_col < mb_cols {
                let mut skip_lf_0: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                    as ::core::ffi::c_int
                    != B_PRED as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                        != SPLITMV as ::core::ffi::c_int
                    && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                let mode_index_0: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                    [(*mode_info_context).mbmi.mode as usize]
                    as ::core::ffi::c_int;
                let seg_0: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                let ref_frame_0: ::core::ffi::c_int =
                    (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                filter_level = (*lfi_n).lvl[seg_0 as usize][ref_frame_0 as usize]
                    [mode_index_0 as usize] as ::core::ffi::c_int;
                if filter_level != 0 {
                    let mut mblim: *const ::core::ffi::c_uchar = &raw mut *(&raw mut (*lfi_n).mblim
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    let mut blim: *const ::core::ffi::c_uchar = &raw mut *(&raw mut (*lfi_n).blim
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbvs_neon(y_ptr, post_y_stride, mblim);
                    }
                    if skip_lf_0 == 0 {
                        vp8_loop_filter_bvs_neon(y_ptr, post_y_stride, blim);
                    }
                    if mb_row > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbhs_neon(y_ptr, post_y_stride, mblim);
                    }
                    if skip_lf_0 == 0 {
                        vp8_loop_filter_bhs_neon(y_ptr, post_y_stride, blim);
                    }
                }
                y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
                u_ptr = u_ptr.offset(8 as ::core::ffi::c_int as isize);
                v_ptr = v_ptr.offset(8 as ::core::ffi::c_int as isize);
                mode_info_context = mode_info_context.offset(1);
                mb_col += 1;
            }
            y_ptr =
                y_ptr.offset((post_y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
            u_ptr = u_ptr
                .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
            v_ptr = v_ptr
                .offset((post_uv_stride * 8 as ::core::ffi::c_int - (*post).uv_width) as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_row += 1;
        }
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_frame_yonly(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: ::core::ffi::c_int,
) { unsafe {
    let post_idx = (*cm).frame_to_show_idx.expect("frame_to_show_idx is None");
    let mut post: *mut YV12_BUFFER_CONFIG = &raw mut (*cm).yv12_fb[post_idx];
    let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
    let mut lfi: loop_filter_info = loop_filter_info {
        mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
    };
    let mut filter_level: ::core::ffi::c_int = 0;
    let mut frame_type: FRAME_TYPE = (*cm).frame_type;
    let mut mode_info_context: *const MODE_INFO = (*cm).mi;
    vp8_loop_filter_frame_init(&mut *cm, &*mbd, default_filt_lvl);
    y_ptr = (*post).y_buffer as *mut ::core::ffi::c_uchar;
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < (*cm).mb_rows {
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*cm).mb_cols {
            let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                as ::core::ffi::c_int
                != B_PRED as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                    != SPLITMV as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            let mode_index: ::core::ffi::c_int =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as ::core::ffi::c_int;
            let seg: ::core::ffi::c_int =
                (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
            let ref_frame: ::core::ffi::c_int =
                (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
            filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                as ::core::ffi::c_int;
            if filter_level != 0 {
                if (*cm).filter_type as ::core::ffi::c_uint
                    == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut[frame_type as usize]
                        [filter_level as usize]
                        as ::core::ffi::c_int;
                    lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(hev_index as isize)
                        as *mut ::core::ffi::c_uchar;
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbv_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bv_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                    if mb_row > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbh_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bh_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                } else {
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbvs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bvs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                    if mb_row > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbhs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bhs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                }
            }
            y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
            mode_info_context = mode_info_context.offset(1);
            mb_col += 1;
        }
        y_ptr =
            y_ptr.offset(((*post).y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
        mode_info_context = mode_info_context.offset(1);
        mb_row += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_partial_frame(
    mut cm: *mut VP8_COMMON,
    mut mbd: *mut MACROBLOCKD,
    mut default_filt_lvl: ::core::ffi::c_int,
) { unsafe {
    let post_idx = (*cm).frame_to_show_idx.expect("frame_to_show_idx is None");
    let mut post: *mut YV12_BUFFER_CONFIG = &raw mut (*cm).yv12_fb[post_idx];
    let mut y_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut mb_cols: ::core::ffi::c_int = (*post).y_width >> 4 as ::core::ffi::c_int;
    let mut mb_rows: ::core::ffi::c_int = (*post).y_height >> 4 as ::core::ffi::c_int;
    let mut linestocopy: ::core::ffi::c_int = 0;
    let mut lfi_n: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
    let mut lfi: loop_filter_info = loop_filter_info {
        mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
        hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
    };
    let mut filter_level: ::core::ffi::c_int = 0;
    let mut frame_type: FRAME_TYPE = (*cm).frame_type;
    let mut mode_info_context: *const MODE_INFO = ::core::ptr::null::<MODE_INFO>();
    vp8_loop_filter_frame_init(&mut *cm, &*mbd, default_filt_lvl);
    linestocopy = mb_rows / PARTIAL_FRAME_FRACTION;
    linestocopy = if linestocopy != 0 {
        linestocopy << 4 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    };
    y_ptr = (*post).y_buffer.offset(
        (((*post).y_height >> 5 as ::core::ffi::c_int)
            * 16 as ::core::ffi::c_int
            * (*post).y_stride) as isize,
    ) as *mut ::core::ffi::c_uchar;
    mode_info_context = (*cm).mi.offset(
        (((*post).y_height >> 5 as ::core::ffi::c_int) * (mb_cols + 1 as ::core::ffi::c_int))
            as isize,
    );
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < linestocopy >> 4 as ::core::ffi::c_int {
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < mb_cols {
            let mut skip_lf: ::core::ffi::c_int = ((*mode_info_context).mbmi.mode
                as ::core::ffi::c_int
                != B_PRED as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mode as ::core::ffi::c_int
                    != SPLITMV as ::core::ffi::c_int
                && (*mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            let mode_index: ::core::ffi::c_int =
                (*lfi_n).mode_lf_lut[(*mode_info_context).mbmi.mode as usize] as ::core::ffi::c_int;
            let seg: ::core::ffi::c_int =
                (*mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
            let ref_frame: ::core::ffi::c_int =
                (*mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
            filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                as ::core::ffi::c_int;
            if filter_level != 0 {
                if (*cm).filter_type as ::core::ffi::c_uint
                    == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut[frame_type as usize]
                        [filter_level as usize]
                        as ::core::ffi::c_int;
                    lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.blim = &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.lim = &raw mut *(&raw mut (*lfi_n).lim as *mut [::core::ffi::c_uchar; 1])
                        .offset(filter_level as isize)
                        as *mut ::core::ffi::c_uchar;
                    lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                        as *mut [::core::ffi::c_uchar; 1])
                        .offset(hev_index as isize)
                        as *mut ::core::ffi::c_uchar;
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbv_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bv_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                    vp8_loop_filter_mbh_neon(
                        y_ptr,
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                        (*post).y_stride,
                        0 as ::core::ffi::c_int,
                        &raw mut lfi,
                    );
                    if skip_lf == 0 {
                        vp8_loop_filter_bh_neon(
                            y_ptr,
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                            (*post).y_stride,
                            0 as ::core::ffi::c_int,
                            &raw mut lfi,
                        );
                    }
                } else {
                    if mb_col > 0 as ::core::ffi::c_int {
                        vp8_loop_filter_mbvs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                    if skip_lf == 0 {
                        vp8_loop_filter_bvs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                    vp8_loop_filter_mbhs_neon(
                        y_ptr,
                        (*post).y_stride,
                        &raw mut *(&raw mut (*lfi_n).mblim as *mut [::core::ffi::c_uchar; 1])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                    if skip_lf == 0 {
                        vp8_loop_filter_bhs_neon(
                            y_ptr,
                            (*post).y_stride,
                            &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                .offset(filter_level as isize)
                                as *mut ::core::ffi::c_uchar,
                        );
                    }
                }
            }
            y_ptr = y_ptr.offset(16 as ::core::ffi::c_int as isize);
            mode_info_context = mode_info_context.offset(1 as ::core::ffi::c_int as isize);
            mb_col += 1;
        }
        y_ptr =
            y_ptr.offset(((*post).y_stride * 16 as ::core::ffi::c_int - (*post).y_width) as isize);
        mode_info_context = mode_info_context.offset(1 as ::core::ffi::c_int as isize);
        mb_row += 1;
    }
}}
