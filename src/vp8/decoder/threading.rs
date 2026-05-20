use crate::vp8::decoder::detokenize::{vp8_decode_mb_tokens, vp8_reset_mb_tokens_context};
use crate::vp8::common::vp8_loopfilter::vp8_loop_filter_frame_init;
use crate::vp8::decoder::decodeframe::vp8_mb_init_dequantizer;
use crate::vp8::common::extend::vp8_extend_mb_row;
use crate::vp8::common::reconintra::intra_prediction_down_copy;
use crate::vp8::common::idctllm::vp8_short_inv_walsh4x4_1_safe;
use crate::vp8::common::dequantize::vp8_dequantize_b_safe;
use crate::vp8::common::idctllm::vp8_short_inv_walsh4x4_safe;
use crate::vp8::common::idct_blk::{vp8_dequant_idct_add_y_block_safe, vp8_dequant_idct_add_uv_block_safe};
use crate::vp8::common::dequantize::vp8_dequant_idct_add_safe;
use crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe;
use crate::vp8::common::reconintra4x4::vp8_intra4x4_predict_safe;

#[cfg(target_arch = "aarch64")]
use crate::simd_shim::*;

unsafe extern "C" {
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
}

use crate::vp8::common::setupintrarecon::vp8_setup_intra_recon_top_line;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}

pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;

pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8BORDERINPIXELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
#[inline]
fn vp8dx_bool_error(br: &BOOL_DECODER) -> ::core::ffi::c_int {
    if br.count > VP8_BD_VALUE_SIZE && br.count < VP8_LOTS_OF_BITS {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
fn vp8dx_safe_bool_error(br: &crate::vp8::decoder::dboolhuff::SafeBoolDecoder) -> ::core::ffi::c_int {
    if br.count > VP8_BD_VALUE_SIZE && br.count < VP8_LOTS_OF_BITS {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const SYNC_POLICY_FIFO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
fn vpx_atomic_init(
    atomic: &vpx_atomic_int,
    value: ::core::ffi::c_int,
) {
    atomic.value.store(value, core::sync::atomic::Ordering::SeqCst);
}
#[inline]
fn vpx_atomic_store_release(
    atomic: &vpx_atomic_int,
    value: ::core::ffi::c_int,
) {
    atomic.value.store(value, core::sync::atomic::Ordering::Release);
}
#[inline]
fn vpx_atomic_load_acquire(
    atomic: &vpx_atomic_int,
) -> ::core::ffi::c_int {
    atomic.value.load(core::sync::atomic::Ordering::Acquire)
}
#[inline]
fn vp8_atomic_spin_wait(
    mb_col: ::core::ffi::c_int,
    last_row_current_mb_col: &vpx_atomic_int,
    nsync: ::core::ffi::c_int,
) {
    while mb_col > vpx_atomic_load_acquire(last_row_current_mb_col) - nsync {}
}

#[inline]

fn setup_decoding_thread_data(
    pc: &VP8_COMMON,
    mt_current_mb_col: Option<&[vpx_atomic_int]>,
    xd: &MACROBLOCKD,
    mbrd: &[std::sync::Arc<std::sync::Mutex<MB_ROW_DEC>>],
) {
    for m_arc in mbrd.iter() {
        let mut m = m_arc.lock().unwrap();
        let mbd = &mut m.mbd;
        mbd.subpixel_predict = xd.subpixel_predict;
        mbd.subpixel_predict8x4 = xd.subpixel_predict8x4;
        mbd.subpixel_predict8x8 = xd.subpixel_predict8x8;
        mbd.subpixel_predict16x16 = xd.subpixel_predict16x16;
        mbd.frame_type = pc.frame_type;
        mbd.pre_fb_idx = xd.pre_fb_idx;
        mbd.dst_fb_idx = xd.dst_fb_idx;
        mbd.dst_y_stride = xd.dst_y_stride;
        mbd.dst_uv_stride = xd.dst_uv_stride;
        mbd.dst_border = xd.dst_border;
        mbd.pre_y_stride = xd.pre_y_stride;
        mbd.pre_uv_stride = xd.pre_uv_stride;
        mbd.pre_border = xd.pre_border;
        mbd.segmentation_enabled = xd.segmentation_enabled;
        mbd.mb_segment_abs_delta = xd.mb_segment_abs_delta;
        mbd.segment_feature_data = xd.segment_feature_data;
        mbd.ref_lf_deltas = xd.ref_lf_deltas;
        mbd.mode_lf_deltas = xd.mode_lf_deltas;
        mbd.mode_ref_lf_delta_enabled = xd.mode_ref_lf_delta_enabled;
        mbd.mode_ref_lf_delta_update = xd.mode_ref_lf_delta_update;
        mbd.current_bc_idx = 0;
        mbd.dequant_y1_dc = xd.dequant_y1_dc;
        mbd.dequant_y1 = xd.dequant_y1;
        mbd.dequant_y2 = xd.dequant_y2;
        mbd.dequant_uv = xd.dequant_uv;
        mbd.fullpixel_mask = !0;
        mbd.corrupted = 0;
        if pc.full_pixel != 0 {
            mbd.fullpixel_mask = !7;
        }
    }
    if let Some(mt_current_mb_col) = mt_current_mb_col {
        for i in 0..pc.mb_rows as usize {
            vpx_atomic_store_release(
                &mt_current_mb_col[i],
                -1,
            );
        }
    }
}
fn mt_decode_macroblock(
    common: &VP8_COMMON,
    safe_decoder: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder,
    xd: &mut MACROBLOCKD,
    mb_idx: ::core::ffi::c_uint,
    above_y: Option<&[u8]>,
    above_u: Option<&[u8]>,
    above_v: Option<&[u8]>,
    left_y: Option<&[u8]>,
    left_u: Option<&[u8]>,
    left_v: Option<&[u8]>,
    left_context: &mut ENTROPY_CONTEXT_PLANES,
    dst_views: (UnsafeRowView, UnsafeRowView, UnsafeRowView),
    above_context_raw: *mut ENTROPY_CONTEXT_PLANES,
    mip_raw: *mut MODE_INFO,
) {
    // SAFETY: This entire macroblock prediction and reconstruction block uses disjoint row slice
    // projections and raw context pointer dereferencing. Safety is mathematically guaranteed at
    // the macro-architecture level by atomic column spinlock synchronization. DO NOT REMOVE.
    unsafe {
        let mut mode: MB_PREDICTION_MODE = DC_PRED;
        let mut i: ::core::ffi::c_int = 0;
        let mut mi = *xd.mode_info(common.mip_slice());

        let mb_row = (-xd.mb_to_top_edge / 128) as usize;
        let mb_col = (-xd.mb_to_left_edge / 128) as usize;
        let recon_yoffset = mb_row * 16 * xd.dst_y_stride as usize + mb_col * 16;
        let recon_uvoffset = mb_row * 8 * xd.dst_uv_stride as usize + mb_col * 8;

        if mi.mbmi.mb_skip_coeff != 0 {
            let is_4x4 = mi.mbmi.is_4x4 != 0;
            let above_context_slice = std::slice::from_raw_parts_mut(above_context_raw, common.mb_cols as usize);
            let (above, left) = xd.contexts_mut(above_context_slice, left_context);
            vp8_reset_mb_tokens_context(above, left, is_4x4);
        } else if vp8dx_safe_bool_error(safe_decoder) == 0 {
            let mut eobtotal: ::core::ffi::c_int = 0;
            let is_4x4 = mi.mbmi.is_4x4 != 0;
            let above_context_slice = std::slice::from_raw_parts_mut(above_context_raw, common.mb_cols as usize);
            let (above, left, qcoeff, eobs) = xd.decode_tokens_inputs_mut(above_context_slice, left_context);
            eobtotal = vp8_decode_mb_tokens(
                safe_decoder,
                &common.fc,
                qcoeff,
                eobs,
                above,
                left,
                is_4x4,
            );
            let skip_coeff = (eobtotal == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
            let mip_slice = std::slice::from_raw_parts_mut(mip_raw, common.mip_slice().len());
            mip_slice[xd.mode_info_idx].mbmi.mb_skip_coeff = skip_coeff;
            mi.mbmi.mb_skip_coeff = skip_coeff;
        }
        mode = mi.mbmi.mode as MB_PREDICTION_MODE;

        if xd.segmentation_enabled != 0 {
            vp8_mb_init_dequantizer(common, xd);
        }
        if mi.mbmi.ref_frame as ::core::ffi::c_int
            == INTRA_FRAME as ::core::ffi::c_int
        {
            let uvmode = mi.mbmi.uv_mode as MB_PREDICTION_MODE;
            let left_available = xd.left_available;
            let up_available = xd.up_available;
            let left_stride_uv = xd.recon_left_stride[1] as usize;
            let left_stride_y = xd.recon_left_stride[0] as usize;

            let uv_stride = xd.dst_uv_stride as usize;
            let uv_border = (xd.dst_border / 2) as usize;
            let uv_buffer_offset = uv_border * uv_stride + uv_border + recon_uvoffset;
            let dst_stride = xd.dst_y_stride;
            let dst_stride_us = dst_stride as usize;
            let border = xd.dst_border as usize;
            let y_buffer_offset = border * dst_stride_us + border + recon_yoffset;

            let mut uabove = [0u8; 9];
            let mut vabove = [0u8; 9];
            let mut uleft = [0u8; 8];
            let mut vleft = [0u8; 8];

            {
                let u_slice = dst_views.1.as_slice_mut(0, dst_views.1.len());
                let v_slice = dst_views.2.as_slice_mut(0, dst_views.2.len());
                
                if let (Some(au), Some(av)) = (above_u, above_v) {
                    uabove.copy_from_slice(au);
                    vabove.copy_from_slice(av);
                } else {
                    uabove.copy_from_slice(&u_slice[uv_buffer_offset - uv_stride - 1 .. uv_buffer_offset - uv_stride + 8]);
                    vabove.copy_from_slice(&v_slice[uv_buffer_offset - uv_stride - 1 .. uv_buffer_offset - uv_stride + 8]);
                }

                if let (Some(lu), Some(lv)) = (left_u, left_v) {
                    uleft.copy_from_slice(lu);
                    vleft.copy_from_slice(lv);
                } else {
                    for i in 0..8 {
                        uleft[i] = u_slice[uv_buffer_offset - 1 + i * left_stride_uv];
                        vleft[i] = v_slice[uv_buffer_offset - 1 + i * left_stride_uv];
                    }
                }

                let upred = &mut u_slice[uv_buffer_offset .. uv_buffer_offset + 7 * uv_stride + 8];
                let vpred = &mut v_slice[uv_buffer_offset .. uv_buffer_offset + 7 * uv_stride + 8];

                crate::vp8::common::reconintra::vp8_build_intra_predictors_mbuv_safe(
                    uvmode,
                    left_available,
                    up_available,
                    &uabove,
                    &vabove,
                    &uleft,
                    &vleft,
                    upred,
                    vpred,
                    uv_stride,
                );
            }

            if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
                let dst_y_slice = dst_views.0.as_slice_mut(0, dst_views.0.len());
                
                let mut yabove = [0u8; 17];
                if let Some(ay) = above_y {
                    yabove.copy_from_slice(&ay[0..17]);
                } else {
                    yabove.copy_from_slice(&dst_y_slice[y_buffer_offset - dst_stride_us - 1 .. y_buffer_offset - dst_stride_us + 16]);
                }
                
                let mut yleft = [0u8; 16];
                if let Some(ly) = left_y {
                    yleft.copy_from_slice(ly);
                } else {
                    for i in 0..16 {
                        yleft[i] = dst_y_slice[y_buffer_offset - 1 + i * left_stride_y];
                    }
                }
                

                let ypred = &mut dst_y_slice[y_buffer_offset .. y_buffer_offset + 15 * dst_stride_us + 16];

                crate::vp8::common::reconintra::vp8_build_intra_predictors_mby_safe(
                    mode,
                    left_available,
                    up_available,
                    &yabove,
                    &yleft,
                    ypred,
                    dst_stride_us,
                );
            } else {
                if mi.mbmi.mb_skip_coeff != 0 {
                    xd.eobs.fill(0);
                }
                let dst_y_slice = dst_views.0.as_slice_mut(0, dst_views.0.len());
                let dst_y_slice_mb = &mut dst_y_slice[recon_yoffset..];
                intra_prediction_down_copy(dst_stride_us, border, dst_y_slice_mb, above_y);
                
                let b_modes = {
                    let mut modes = [0 as B_PREDICTION_MODE; 16];
                    for idx in 0..16 {
                        modes[idx] = mi.bmi[idx].mode();
                    }
                    modes
                };

                let dst_y_slice = dst_views.0.as_slice_mut(0, dst_views.0.len());
                
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    let b_offset = xd.block[i as usize].offset;
                    let b_mode = b_modes[i as usize];
                    let dst_offset = y_buffer_offset + b_offset as usize;
                    
                    let above_idx = dst_offset - dst_stride as usize;
                    let yleft_idx = dst_offset - 1;
                    
                    let mut above_buf = [0u8; 8];
                    if i < 4 && above_y.is_some() {
                        let ay = above_y.unwrap();
                        let start = (i as usize % 4) * 4;
                        above_buf.copy_from_slice(&ay[start + 1 .. start + 9]);
                    } else {
                        above_buf.copy_from_slice(&dst_y_slice[above_idx .. above_idx + 8]);
                    }
                    
                    let top_left_val = if i < 4 {
                        if let Some(ay) = above_y {
                            let start = (i as usize % 4) * 4;
                            ay[start]
                        } else {
                            dst_y_slice[above_idx - 1]
                        }
                    } else if i % 4 == 0 {
                        if let Some(ly) = left_y {
                            let start = (i as usize / 4) * 4;
                            ly[start - 1]
                        } else {
                            dst_y_slice[above_idx - 1]
                        }
                    } else {
                        dst_y_slice[above_idx - 1]
                    };
                    
                    let mut left_buf = [0u8; 4];
                    if i % 4 == 0 && left_y.is_some() {
                        let ly = left_y.unwrap();
                        let start = (i as usize / 4) * 4;
                        left_buf.copy_from_slice(&ly[start .. start + 4]);
                    } else {
                        for r in 0..4 {
                            left_buf[r] = dst_y_slice[yleft_idx + r * dst_stride as usize];
                        }
                    }
                    
                    vp8_intra4x4_predict_safe(
                        dst_y_slice,
                        dst_offset,
                        dst_stride as usize,
                        b_mode,
                        &above_buf,
                        &left_buf,
                        top_left_val,
                    );
                    if xd.eobs[i as usize] != 0 {
                        let block_idx = i as usize;
                        let q_offset = block_idx * 16;
                        let q_sub: &mut [i16; 16] = (&mut xd.qcoeff[q_offset..q_offset + 16]).try_into().unwrap();
                        let dq_ref = &xd.dequant_y1;
                        
                        let dst_slice_offset = y_buffer_offset + b_offset as usize;
                        let dst_sub_len = 3 * dst_stride as usize + 4;
                        let dst_sub_slice = &mut dst_y_slice[dst_slice_offset..dst_slice_offset + dst_sub_len];
     
                        if xd.eobs[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            vp8_dequant_idct_add_safe(q_sub, dq_ref, dst_sub_slice, dst_stride);
                        } else {
                            let input_dc = q_sub[0] * dq_ref[0];
                            
                            let mut pred = [0u8; 16];
                            for r in 0..4 {
                                for c in 0..4 {
                                    pred[r * 4 + c] = dst_sub_slice[r * dst_stride as usize + c];
                                }
                            }
                            
                            vp8_dc_only_idct_add_safe(
                                input_dc,
                                &pred,
                                4,
                                dst_sub_slice,
                                dst_stride,
                            );
                            
                            q_sub[0] = 0;
                            q_sub[1] = 0;
                        }
                    }
                    i += 1;
                }
            }
        } else {
            let pre_fb = &common.yv12_fb[xd.pre_fb_idx];
            let (pre_y, pre_u, pre_v) = pre_fb.views_with_borders();
            let dst_y = dst_views.0.as_slice_mut(0, dst_views.0.len());
            let dst_u = dst_views.1.as_slice_mut(0, dst_views.1.len());
            let dst_v = dst_views.2.as_slice_mut(0, dst_views.2.len());
            crate::vp8::common::reconinter::vp8_build_inter_predictors_mb(
                xd,
                &mi,
                dst_y,
                dst_u,
                dst_v,
                pre_y,
                pre_u,
                pre_v,
            );
        }

        if mi.mbmi.mb_skip_coeff == 0 {
            if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
                let dq_y: &[i16; 16] = if mode as ::core::ffi::c_uint != SPLITMV as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if xd.eobs[24 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        > 1 as ::core::ffi::c_int
                    {
                        let qcoeff_slice = &xd.qcoeff[24 * 16 .. 24 * 16 + 16];
                        let dqcoeff_slice = &mut xd.dqcoeff[24 * 16 .. 24 * 16 + 16];
                        vp8_dequantize_b_safe(qcoeff_slice, dqcoeff_slice, &xd.dequant_y2);

                        let walsh_input: &[i16; 16] = (&xd.dqcoeff[24 * 16 .. 24 * 16 + 16]).try_into().unwrap();
                        vp8_short_inv_walsh4x4_safe(walsh_input, &mut xd.qcoeff);

                        xd.qcoeff[24 * 16 .. 24 * 16 + 16].fill(0);
                    } else {
                        xd.dqcoeff[24 * 16] = (xd.qcoeff[24 * 16] as i32
                            * xd.dequant_y2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                            as ::core::ffi::c_short;
                        let dqcoeff_slice = &xd.dqcoeff[24 * 16 .. 24 * 16 + 16];
                        vp8_short_inv_walsh4x4_1_safe(
                            dqcoeff_slice,
                            &mut xd.qcoeff,
                        );
                        xd.qcoeff[24 * 16 .. 24 * 16 + 2].fill(0);
                    }
                    &xd.dequant_y1_dc
                } else {
                    &xd.dequant_y1
                };

                let y_stride = xd.dst_y_stride;
                let dst_y_view = dst_views.0.as_slice_mut(recon_yoffset, dst_views.0.len() - recon_yoffset);
                let q_y: &mut [i16; 256] = (&mut xd.qcoeff[0..256]).try_into().unwrap();
                let dst_len = 15 * y_stride as usize + 16;
                let dst_slice = &mut dst_y_view[..dst_len];
                let eobs_y: &[::core::ffi::c_char; 16] = (&xd.eobs[0..16]).try_into().unwrap();

                vp8_dequant_idct_add_y_block_safe(q_y, dq_y, dst_slice, y_stride, eobs_y);
            }

            let uv_stride = xd.dst_uv_stride;
            let dst_u_view = dst_views.1.as_slice_mut(recon_uvoffset, dst_views.1.len() - recon_uvoffset);
            let dst_v_view = dst_views.2.as_slice_mut(recon_uvoffset, dst_views.2.len() - recon_uvoffset);
            let q_uv: &mut [i16; 128] = (&mut xd.qcoeff[256..384]).try_into().unwrap();
            let dst_u_len = 7 * uv_stride as usize + 8;
            let dst_u_slice = &mut dst_u_view[..dst_u_len];
            let dst_v_slice = &mut dst_v_view[..dst_u_len];
            let eobs_uv: &[::core::ffi::c_char; 8] = (&xd.eobs[16..24]).try_into().unwrap();

            vp8_dequant_idct_add_uv_block_safe(
                q_uv,
                &xd.dequant_uv,
                dst_u_slice,
                dst_v_slice,
                uv_stride,
                eobs_uv,
            );
        }
    }
}
fn mt_decode_mb_rows(
    common: &VP8_COMMON,
    mbc_raw: *mut vp8_reader,
    mt_sync: &VP8D_MT_SYNC,
    xd: &mut MACROBLOCKD,
    start_mb_row: ::core::ffi::c_int,
    decoding_thread_count: ::core::ffi::c_uint,
    fragments: FRAGMENT_DATA,
) {
    let mut mb_row: ::core::ffi::c_int = 0;
    let pc = common;
    let nsync: ::core::ffi::c_int = mt_sync.sync_range;
    let first_row_no_sync_above: vpx_atomic_int = vpx_atomic_int {
        value: core::sync::atomic::AtomicI32::new(pc.mb_cols + nsync),
    };
    let mut num_part: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << pc.multi_token_partition as ::core::ffi::c_uint;
    let mut last_mb_row: ::core::ffi::c_int = start_mb_row;
    
    let new_fb_idx = pc.new_fb_idx as usize;
    let lst_fb_idx = pc.lst_fb_idx as usize;
    
    let recon_y_stride = pc.yv12_fb[new_fb_idx].y_stride;
    let recon_uv_stride = pc.yv12_fb[new_fb_idx].uv_stride;
    
    let mut ref_buffer: [[*mut ::core::ffi::c_uchar; 3]; 4] =
        [[::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3]; 4];
    let mut dst_buffer: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut i: ::core::ffi::c_int = 0;
    let mut ref_fb_corrupted: [::core::ffi::c_int; 4] = [0; 4];
    ref_fb_corrupted[INTRA_FRAME as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    
    i = 1 as ::core::ffi::c_int;
    while i < MAX_REF_FRAMES as ::core::ffi::c_int {
        let fb_idx = match i {
            1 => pc.lst_fb_idx as usize,
            2 => pc.gld_fb_idx as usize,
            3 => pc.alt_fb_idx as usize,
            _ => panic!("Invalid ref frame index"),
        };
        let this_fb = &pc.yv12_fb[fb_idx];
        ref_buffer[i as usize][0 as ::core::ffi::c_int as usize] = this_fb.y_buffer;
        ref_buffer[i as usize][1 as ::core::ffi::c_int as usize] = this_fb.u_buffer;
        ref_buffer[i as usize][2 as ::core::ffi::c_int as usize] = this_fb.v_buffer;
        ref_fb_corrupted[i as usize] = this_fb.corrupted;
        i += 1;
    }
    
    let yv12_fb_new = &pc.yv12_fb[new_fb_idx];
    dst_buffer[0 as ::core::ffi::c_int as usize] = yv12_fb_new.y_buffer;
    dst_buffer[1 as ::core::ffi::c_int as usize] = yv12_fb_new.u_buffer;
    dst_buffer[2 as ::core::ffi::c_int as usize] = yv12_fb_new.v_buffer;
    
    xd.up_available = (start_mb_row != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    xd.mode_info_idx = (pc.mode_info_stride * (start_mb_row + 1) + 1) as usize;
    xd.mode_info_stride = pc.mode_info_stride;
    
    mb_row = start_mb_row;
    while mb_row < pc.mb_rows {
        // SAFETY: This entire row decoding loop uses disjoint row slices, shared pointer reads,
        // and raw context indexing. Concurrency is mathematically synchronized via atomic column
        // spinlocks at the macro-architecture level. DO NOT REMOVE this safety boundary.
        unsafe {
            let mut recon_yoffset: ::core::ffi::c_int = 0;
            let mut recon_uvoffset: ::core::ffi::c_int = 0;
            let mut mb_col: ::core::ffi::c_int = 0;
            let mut filter_level: ::core::ffi::c_int = 0;
            
            last_mb_row = mb_row;
            xd.current_bc_idx = (mb_row % num_part) as usize;
            let bc_idx = xd.current_bc_idx;
            let slice = fragments.get_slice(bc_idx + 1).unwrap_or(&[]);
            
            let mut safe_decoder = crate::vp8::decoder::dboolhuff::SafeBoolDecoder::from_bool_decoder(
                &*mbc_raw.add(bc_idx),
                slice,
            );
        
        let mt_current_mb_col = mt_sync.mt_current_mb_col.as_ref().unwrap();
        let last_row_current_mb_col: &vpx_atomic_int = if mb_row > 0 {
            &mt_current_mb_col[(mb_row - 1) as usize]
        } else {
            &first_row_no_sync_above
        };
        let current_mb_col: &vpx_atomic_int = &mt_current_mb_col[mb_row as usize];
        
        recon_yoffset = mb_row * recon_y_stride * 16 as ::core::ffi::c_int;
        recon_uvoffset = mb_row * recon_uv_stride * 8 as ::core::ffi::c_int;
        xd.above_context_idx = 0;
        let mut left_context = ENTROPY_CONTEXT_PLANES::default();
        xd.left_available = 0 as ::core::ffi::c_int;
        xd.mb_to_top_edge = -((mb_row * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
        xd.mb_to_bottom_edge = ((pc.mb_rows - 1 - mb_row) * 16) << 3;
        
        let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
        let dst_views = dst_fb.get_safe_unsafe_slices();

        if pc.filter_level != 0 {
            xd.recon_left_stride[0] = 1;
            xd.recon_left_stride[1] = 1;
        } else {
            xd.recon_left_stride[0] = xd.dst_y_stride;
            xd.recon_left_stride[1] = xd.dst_uv_stride;
            
            let y_border = dst_fb.border as usize;
            let y_stride = dst_fb.y_stride as usize;
            let uv_border = (dst_fb.border / 2) as usize;
            let uv_stride = dst_fb.uv_stride as usize;
            let mb_row_usize = mb_row as usize;

            let dst_y_slice = dst_views.0.as_slice_mut(0, dst_views.0.len());
            let y_base = (y_border + mb_row_usize * 16) * y_stride + y_border - 1;
            for i in 0..16 {
                dst_y_slice[y_base + i * y_stride] = 129;
            }

            let dst_u_slice = dst_views.1.as_slice_mut(0, dst_views.1.len());
            let u_base = (uv_border + mb_row_usize * 8) * uv_stride + uv_border - 1;
            for i in 0..8 {
                dst_u_slice[u_base + i * uv_stride] = 129;
            }

            let dst_v_slice = dst_views.2.as_slice_mut(0, dst_views.2.len());
            let v_base = (uv_border + mb_row_usize * 8) * uv_stride + uv_border - 1;
            for i in 0..8 {
                dst_v_slice[v_base + i * uv_stride] = 129;
            }
        }
        
        mb_col = 0;
        while mb_col < pc.mb_cols {
            if (mb_col - 1) % nsync == 0 {
                vpx_atomic_store_release(current_mb_col, mb_col - 1);
            }
            if mb_row != 0 && mb_col & nsync - 1 == 0 {
                vp8_atomic_spin_wait(mb_col, last_row_current_mb_col, nsync);
            }
            xd.mb_to_left_edge = -((mb_col * 16) << 3);
            xd.mb_to_right_edge = ((pc.mb_cols - 1 - mb_col) * 16) << 3;
            
            let yv12_fb_new = &pc.yv12_fb[new_fb_idx];
            let slice_y = yv12_fb_new.y_slice_safe();
            let slice_u = yv12_fb_new.u_slice_safe();
            let slice_v = yv12_fb_new.v_slice_safe();
            
            let border_y = yv12_fb_new.border as usize;
            let stride_y = yv12_fb_new.y_stride as usize;
            let active_start_y = border_y * stride_y + border_y;
            
            let border_uv = (yv12_fb_new.border / 2) as usize;
            let stride_uv = yv12_fb_new.uv_stride as usize;
            let active_start_uv = border_uv * stride_uv + border_uv;
            
            let y_offset = active_start_y + recon_yoffset as usize;
            let uv_offset = active_start_uv + recon_uvoffset as usize;
            
            let dst_y_ptr = &slice_y[y_offset] as *const u8 as *mut u8;
            let dst_u_ptr = &slice_u[uv_offset] as *const u8 as *mut u8;
            let dst_v_ptr = &slice_v[uv_offset] as *const u8 as *mut u8;
            
            let cur_ref_frame = xd.mode_info(pc.mip_slice()).mbmi.ref_frame;
            xd.corrupted |= ref_fb_corrupted[cur_ref_frame as usize];
            if xd.corrupted != 0 {
                let mt_current_mb_col = mt_sync.mt_current_mb_col.as_ref().unwrap();
                while mb_row < pc.mb_rows {
                    let cur_col = &mt_current_mb_col[mb_row as usize];
                    vpx_atomic_store_release(cur_col, pc.mb_cols + nsync);
                    mb_row = (mb_row as u32).wrapping_add(decoding_thread_count.wrapping_add(1)) as i32;
                }
                xd.error_info.trigger(
                    VPX_CODEC_CORRUPT_FRAME,
                    "Corrupted reference frame",
                );
            }
            
            if cur_ref_frame as ::core::ffi::c_int >= LAST_FRAME as ::core::ffi::c_int {
                let ref_0 = cur_ref_frame as MV_REFERENCE_FRAME;
                let fb_idx = match ref_0 {
                    1 => pc.lst_fb_idx as usize,
                    2 => pc.gld_fb_idx as usize,
                    3 => pc.alt_fb_idx as usize,
                    _ => panic!("Invalid ref frame index"),
                };
                xd.pre_fb_idx = fb_idx;
                let this_fb = &pc.yv12_fb[fb_idx];
                xd.pre_y_stride = this_fb.y_stride;
                xd.pre_uv_stride = this_fb.uv_stride;
                xd.pre_border = this_fb.border;
            } else {
                xd.pre_fb_idx = new_fb_idx;
            }
            
            let (above_y, above_u, above_v, left_y, left_u, left_v) = if pc.filter_level != 0 {
                let mb_row_us = mb_row as usize;
                let mb_col_us = mb_col as usize;
                
                let yabove_view = mt_sync.mt_yabove_row.as_ref().unwrap()[mb_row_us];
                let offset_y = 31 + mb_col_us * 16;
                let ay = yabove_view.as_slice(offset_y, 24);
                
                let uabove_view = mt_sync.mt_uabove_row.as_ref().unwrap()[mb_row_us];
                let offset_u = 15 + mb_col_us * 8;
                let au = uabove_view.as_slice(offset_u, 9);
                
                let vabove_view = mt_sync.mt_vabove_row.as_ref().unwrap()[mb_row_us];
                let av = vabove_view.as_slice(offset_u, 9);
                    
                let yleft_view = mt_sync.mt_yleft_col.as_ref().unwrap()[mb_row_us];
                let ly = yleft_view.as_slice(0, 16);
                
                let uleft_view = mt_sync.mt_uleft_col.as_ref().unwrap()[mb_row_us];
                let lu = uleft_view.as_slice(0, 8);
                
                let vleft_view = mt_sync.mt_vleft_col.as_ref().unwrap()[mb_row_us];
                let lv = vleft_view.as_slice(0, 8);
                
                (Some(ay), Some(au), Some(av), Some(ly), Some(lu), Some(lv))
            } else {
                (None, None, None, None, None, None)
            };
            
            let above_context_slice_mut_raw = match pc.above_context.as_ref() {
                Some(ac) => ac.as_ptr() as *mut ENTROPY_CONTEXT_PLANES,
                None => std::ptr::null_mut(),
            };
            let mip_slice_mut_raw = match pc.mip.as_ref() {
                Some(m) => m.as_ptr() as *mut MODE_INFO,
                None => std::ptr::null_mut(),
            };

            let mb_idx = (mb_row * pc.mb_cols + mb_col) as u32;
            mt_decode_macroblock(
                pc,
                &mut safe_decoder,
                xd,
                mb_idx,
                above_y,
                above_u,
                above_v,
                left_y,
                left_u,
                left_v,
                &mut left_context,
                dst_views,
                above_context_slice_mut_raw,
                mip_slice_mut_raw,
            );
            
            xd.left_available = 1;
            xd.corrupted |= vp8dx_safe_bool_error(&safe_decoder);
            
            if pc.filter_level != 0 {
                let lfi_n = &pc.lf_info;
                let cur_mbmi = &xd.mode_info(pc.mip_slice()).mbmi;
                let skip_lf = (cur_mbmi.mode as ::core::ffi::c_int != B_PRED as ::core::ffi::c_int
                    && cur_mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int
                    && cur_mbmi.mb_skip_coeff as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
                let mode_index = lfi_n.mode_lf_lut[cur_mbmi.mode as usize] as ::core::ffi::c_int;
                let seg = cur_mbmi.segment_id as usize;
                let ref_frame = cur_mbmi.ref_frame as usize;
                filter_level = lfi_n.lvl[seg][ref_frame][mode_index as usize] as ::core::ffi::c_int;
                
                if mb_row != pc.mb_rows - 1 {
                    let border = xd.dst_border as usize;
                    let stride = xd.dst_y_stride as usize;
                    let src_idx = (border + 15) * stride + border + recon_yoffset as usize;
                    let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                    let src_slice = &dst_fb.y_slice_safe()[src_idx..src_idx + 16];
                    
                    let dst_ab = mt_sync.mt_yabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                    let border_uv = (xd.dst_border / 2) as usize;
                    let stride_uv = xd.dst_uv_stride as usize;
                    let src_idx_u = (border_uv + 7) * stride_uv + border_uv + recon_uvoffset as usize;
                    let src_slice_u = &dst_fb.u_slice_safe()[src_idx_u..src_idx_u + 8];
                    let src_slice_v = &dst_fb.v_slice_safe()[src_idx_u..src_idx_u + 8];
                    let dst_ab_u = mt_sync.mt_uabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                    let dst_ab_v = mt_sync.mt_vabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                    
                    let offset = 32 + (mb_col * 16) as usize;
                    dst_ab.as_slice_mut(offset, 16).copy_from_slice(src_slice);
                    
                    let offset_uv = 16 + (mb_col * 8) as usize;
                    dst_ab_u.as_slice_mut(offset_uv, 8).copy_from_slice(src_slice_u);
                    dst_ab_v.as_slice_mut(offset_uv, 8).copy_from_slice(src_slice_v);
                }
                
                if mb_col != pc.mb_cols - 1 {
                    let next_mbmi = &pc.mip.as_ref().unwrap()[xd.mode_info_idx + 1].mbmi;
                    if next_mbmi.ref_frame as ::core::ffi::c_int == INTRA_FRAME as ::core::ffi::c_int {
                        let border = xd.dst_border as usize;
                        let stride = xd.dst_y_stride as usize;
                        let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                        let y_slice = dst_fb.y_slice_safe();
                        let dst_ab = mt_sync.mt_yleft_col.as_ref().unwrap()[mb_row as usize];
                        let border_uv = (xd.dst_border / 2) as usize;
                        let stride_uv = xd.dst_uv_stride as usize;
                        let u_slice = dst_fb.u_slice_safe();
                        let v_slice = dst_fb.v_slice_safe();
                        let dst_ab_u = mt_sync.mt_uleft_col.as_ref().unwrap()[mb_row as usize];
                        let dst_ab_v = mt_sync.mt_vleft_col.as_ref().unwrap()[mb_row as usize];
                        
                        let dst_slice = dst_ab.as_slice_mut(0, 16);
                        for i in 0..16 {
                            let src_idx = border * stride + border + i * stride + 15 + recon_yoffset as usize;
                            dst_slice[i] = y_slice[src_idx];
                        }
                        
                        let dst_slice_u = dst_ab_u.as_slice_mut(0, 8);
                        let dst_slice_v = dst_ab_v.as_slice_mut(0, 8);
                        for i in 0..8 {
                            let src_idx = border_uv * stride_uv + border_uv + i * stride_uv + 7 + recon_uvoffset as usize;
                            dst_slice_u[i] = u_slice[src_idx];
                            dst_slice_v[i] = v_slice[src_idx];
                        }
                    }
                }
                
                if filter_level != 0 {
                    if pc.filter_type as ::core::ffi::c_uint == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint {
                        #[cfg(target_arch = "aarch64")]
                        {
                            let y_stride = xd.dst_y_stride as ::core::ffi::c_int;
                            let uv_stride = xd.dst_uv_stride as ::core::ffi::c_int;
                            
                            let frame_type = pc.frame_type;
                            let hev_index = lfi_n.hev_thr_lut[frame_type as usize][filter_level as usize] as usize;
                            
                            let mut lfi: loop_filter_info = loop_filter_info {
                                mblim: lfi_n.mblim[filter_level as usize].as_ptr(),
                                blim: lfi_n.blim[filter_level as usize].as_ptr(),
                                lim: lfi_n.lim[filter_level as usize].as_ptr(),
                                hev_thr: lfi_n.hev_thr[hev_index].as_ptr(),
                            };
                            
                            let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                            let col_offset_y = (mb_col * 16) as usize;
                            let col_offset_uv = (mb_col * 8) as usize;
                            
                            let (row_above, row_current) = if mb_row > 0 {
                                let y_stride_us = y_stride as usize;
                                let uv_stride_us = uv_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let uv_len = 8 * uv_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;
                                let u_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;
                                let v_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;
                                let y_start_curr = (mb_row as usize) * 16 * y_stride_us;
                                let u_start_curr = (mb_row as usize) * 8 * uv_stride_us;
                                let v_start_curr = (mb_row as usize) * 8 * uv_stride_us;
                                (
                                    (
                                        dst_views.0.as_slice_mut(y_start_above, y_len),
                                        dst_views.1.as_slice_mut(u_start_above, uv_len),
                                        dst_views.2.as_slice_mut(v_start_above, uv_len),
                                    ),
                                    (
                                        dst_views.0.as_slice_mut(y_start_curr, y_len),
                                        dst_views.1.as_slice_mut(u_start_curr, uv_len),
                                        dst_views.2.as_slice_mut(v_start_curr, uv_len),
                                    )
                                )
                            } else {
                                let y_stride_us = y_stride as usize;
                                let uv_stride_us = uv_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let uv_len = 8 * uv_stride_us;
                                (
                                    (&mut [] as &mut [u8], &mut [] as &mut [u8], &mut [] as &mut [u8]),
                                    (
                                        dst_views.0.as_slice_mut(0, y_len),
                                        dst_views.1.as_slice_mut(0, uv_len),
                                        dst_views.2.as_slice_mut(0, uv_len),
                                    )
                                )
                            };

                            if mb_row > 0 {
                                if mb_col > 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_mbv_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bv_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                                let y_stride_us = y_stride as usize;
                                let uv_stride_us = uv_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let uv_len = 8 * uv_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;
                                let u_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;
                                let v_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;

                                crate::simd_shim::safe_vp8_loop_filter_mbh_neon(
                                    dst_views.0.as_slice_mut(y_start_above, 2 * y_len),
                                    y_len + col_offset_y,
                                    dst_views.1.as_slice_mut(u_start_above, 2 * uv_len),
                                    uv_len + col_offset_uv,
                                    dst_views.2.as_slice_mut(v_start_above, 2 * uv_len),
                                    uv_len + col_offset_uv,
                                    y_stride, uv_stride,
                                    &mut lfi
                                );
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bh_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                            } else {
                                if mb_col > 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_mbv_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bv_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bh_neon(
                                        row_current.0, col_offset_y,
                                        row_current.1, col_offset_uv,
                                        row_current.2, col_offset_uv,
                                        y_stride, uv_stride,
                                        &mut lfi
                                    );
                                }
                            }
                        }
                        #[cfg(not(target_arch = "aarch64"))]
                        {
                            let y_stride = xd.dst_y_stride as usize;
                            let uv_stride = xd.dst_uv_stride as usize;
                            
                            let frame_type = pc.frame_type;
                            let hev_index = lfi_n.hev_thr_lut[frame_type as usize][filter_level as usize] as usize;
                            
                            let blimit_m_slice = &lfi_n.mblim[filter_level as usize];
                            let blimit_b_slice = &lfi_n.blim[filter_level as usize];
                            let limit_slice = &lfi_n.lim[filter_level as usize];
                            let thresh_slice = &lfi_n.hev_thr[hev_index];
                            
                            let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                            let has_u = !dst_fb.u_buffer.is_null();
                            let has_v = !dst_fb.v_buffer.is_null();
                            
                            let col_offset_y = (mb_col * 16) as usize;
                            let col_offset_uv = (mb_col * 8) as usize;
                            
                            let (row_above, row_current) = if mb_row > 0 {
                                let y_stride_us = y_stride as usize;
                                let uv_stride_us = uv_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let uv_len = 8 * uv_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;
                                let u_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;
                                let v_start_above = (mb_row as usize - 1) * 8 * uv_stride_us;
                                let y_start_curr = (mb_row as usize) * 16 * y_stride_us;
                                let u_start_curr = (mb_row as usize) * 8 * uv_stride_us;
                                let v_start_curr = (mb_row as usize) * 8 * uv_stride_us;
                                (
                                    (
                                        dst_views.0.as_slice_mut(y_start_above, y_len),
                                        dst_views.1.as_slice_mut(u_start_above, uv_len),
                                        dst_views.2.as_slice_mut(v_start_above, uv_len),
                                    ),
                                    (
                                        dst_views.0.as_slice_mut(y_start_curr, y_len),
                                        dst_views.1.as_slice_mut(u_start_curr, uv_len),
                                        dst_views.2.as_slice_mut(v_start_curr, uv_len),
                                    )
                                )
                            } else {
                                let y_stride_us = y_stride as usize;
                                let uv_stride_us = uv_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let uv_len = 8 * uv_stride_us;
                                (
                                    (&mut [] as &mut [u8], &mut [] as &mut [u8], &mut [] as &mut [u8]),
                                    (
                                        dst_views.0.as_slice_mut(0, y_len),
                                        dst_views.1.as_slice_mut(0, uv_len),
                                        dst_views.2.as_slice_mut(0, uv_len),
                                    )
                                )
                            };

                            if mb_row > 0 {
                                if mb_col > 0 {
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.0, col_offset_y, y_stride, blimit_m_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.1, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.2, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                                if skip_lf == 0 {
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 4, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 8, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 12, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.1, col_offset_uv + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.2, col_offset_uv + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                                
                                crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_split_safe(
                                    row_above.0, row_current.0, col_offset_y, y_stride, blimit_m_slice, limit_slice, thresh_slice, 2
                                );
                                if has_u {
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_split_safe(
                                        row_above.1, row_current.1, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1
                                    );
                                }
                                if has_v {
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_split_safe(
                                        row_above.2, row_current.2, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1
                                    );
                                }
                                
                                if skip_lf == 0 {
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 4 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 8 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 12 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.1, col_offset_uv + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.2, col_offset_uv + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                            } else {
                                if mb_col > 0 {
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.0, col_offset_y, y_stride, blimit_m_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.1, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(row_current.2, col_offset_uv, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                                if skip_lf == 0 {
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 4, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 8, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.0, col_offset_y + 12, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.1, col_offset_uv + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(row_current.2, col_offset_uv + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                                
                                if skip_lf == 0 {
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 4 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 8 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.0, col_offset_y + 12 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    if has_u {
                                        crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.1, col_offset_uv + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                    if has_v {
                                        crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(row_current.2, col_offset_uv + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                    }
                                }
                            }
                        }
                    } else {
                        #[cfg(target_arch = "aarch64")]
                        {
                            let y_stride = xd.dst_y_stride as ::core::ffi::c_int;
                            let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                            let col_offset_y = (mb_col * 16) as usize;
                            
                            let blimit_m = &lfi_n.mblim[filter_level as usize];
                            let blimit_b = &lfi_n.blim[filter_level as usize];
                            
                            let (row_above, row_current) = if mb_row > 0 {
                                let y_stride_us = y_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;
                                let y_start_curr = (mb_row as usize) * 16 * y_stride_us;
                                (
                                    (
                                        dst_views.0.as_slice_mut(y_start_above, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    ),
                                    (
                                        dst_views.0.as_slice_mut(y_start_curr, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    )
                                )
                            } else {
                                let y_stride_us = y_stride as usize;
                                let y_len = 16 * y_stride_us;
                                (
                                    (&mut [] as &mut [u8], &mut [] as &mut [u8], &mut [] as &mut [u8]),
                                    (
                                        dst_views.0.as_slice_mut(0, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    )
                                )
                            };

                            if mb_row > 0 {
                                if mb_col > 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_mbvs_neon(row_current.0, col_offset_y, y_stride, blimit_m);
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bvs_neon(row_current.0, col_offset_y, y_stride, blimit_b);
                                }
                                let y_stride_us = y_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;

                                crate::simd_shim::safe_vp8_loop_filter_mbhs_neon(
                                    dst_views.0.as_slice_mut(y_start_above, 2 * y_len),
                                    y_len + col_offset_y,
                                    y_stride,
                                    blimit_m
                                );
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bhs_neon(row_current.0, col_offset_y, y_stride, blimit_b);
                                }
                            } else {
                                if mb_col > 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_mbvs_neon(row_current.0, col_offset_y, y_stride, blimit_m);
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bvs_neon(row_current.0, col_offset_y, y_stride, blimit_b);
                                }
                                if skip_lf == 0 {
                                    crate::simd_shim::safe_vp8_loop_filter_bhs_neon(row_current.0, col_offset_y, y_stride, blimit_b);
                                }
                            }
                        }
                        #[cfg(not(target_arch = "aarch64"))]
                        {
                            let y_stride = xd.dst_y_stride as usize;
                            let dst_fb = &pc.yv12_fb[xd.dst_fb_idx];
                            
                            let col_offset_y = (mb_col * 16) as usize;
                            
                            let (row_above, row_current) = if mb_row > 0 {
                                let y_stride_us = y_stride as usize;
                                let y_len = 16 * y_stride_us;
                                let y_start_above = (mb_row as usize - 1) * 16 * y_stride_us;
                                let y_start_curr = (mb_row as usize) * 16 * y_stride_us;
                                (
                                    (
                                        dst_views.0.as_slice_mut(y_start_above, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    ),
                                    (
                                        dst_views.0.as_slice_mut(y_start_curr, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    )
                                )
                            } else {
                                let y_stride_us = y_stride as usize;
                                let y_len = 16 * y_stride_us;
                                (
                                    (&mut [] as &mut [u8], &mut [] as &mut [u8], &mut [] as &mut [u8]),
                                    (
                                        dst_views.0.as_slice_mut(0, y_len),
                                        &mut [] as &mut [u8],
                                        &mut [] as &mut [u8]
                                    )
                                )
                            };

                            if mb_row > 0 {
                                if mb_col > 0 {
                                    let blimit_val = lfi_n.mblim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_vertical_edge_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                                if skip_lf == 0 {
                                    let blimit_val = lfi_n.blim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_bvs_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                                
                                {
                                    let blimit_val = lfi_n.mblim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_horizontal_edge_split_safe(
                                        row_above.0, row_current.0, col_offset_y, y_stride, blimit_val
                                    );
                                }
                                
                                if skip_lf == 0 {
                                    let blimit_val = lfi_n.blim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_bhs_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                            } else {
                                if mb_col > 0 {
                                    let blimit_val = lfi_n.mblim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_vertical_edge_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                                if skip_lf == 0 {
                                    let blimit_val = lfi_n.blim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_bvs_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                                
                                if skip_lf == 0 {
                                    let blimit_val = lfi_n.blim[filter_level as usize][0];
                                    crate::vp8::common::loopfilter_filters::vp8_loop_filter_bhs_safe(row_current.0, col_offset_y, y_stride, blimit_val);
                                }
                            }
                        }
                    }
                }
            }
            
            recon_yoffset += 16;
            recon_uvoffset += 8;
            xd.mode_info_idx += 1;
            xd.above_context_idx += 1;
            mb_col += 1;
        }
        
        safe_decoder.update_bool_decoder(&mut *mbc_raw.add(bc_idx));
        
        if pc.filter_level != 0 {
            if mb_row != pc.mb_rows - 1 {
                let mut lasty = pc.yv12_fb[lst_fb_idx].y_width + VP8BORDERINPIXELS;
                let mut lastuv = (pc.yv12_fb[lst_fb_idx].y_width >> 1) + (VP8BORDERINPIXELS >> 1);
                
                let dst_ab = mt_sync.mt_yabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                let dst_ab_u = mt_sync.mt_uabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                let dst_ab_v = mt_sync.mt_vabove_row.as_ref().unwrap()[(mb_row + 1) as usize];
                
                let dst_slice = dst_ab.as_slice_mut(0, dst_ab.len());
                let val = dst_slice[lasty as usize - 1];
                dst_slice[lasty as usize..lasty as usize + 4].fill(val);
                
                let dst_slice_u = dst_ab_u.as_slice_mut(0, dst_ab_u.len());
                let val_u = dst_slice_u[lastuv as usize - 1];
                dst_slice_u[lastuv as usize..lastuv as usize + 4].fill(val_u);
                
                let dst_slice_v = dst_ab_v.as_slice_mut(0, dst_ab_v.len());
                let val_v = dst_slice_v[lastuv as usize - 1];
                dst_slice_v[lastuv as usize..lastuv as usize + 4].fill(val_v);
            }
        } else {
            let y_stride = dst_fb.y_stride as usize;
            let uv_stride = dst_fb.uv_stride as usize;
            let y_width = dst_fb.y_width as usize;
            let uv_width = dst_fb.uv_width as usize;
            let border = dst_fb.border as usize;
            let mb_row_usize = mb_row as usize;

            let dst_y_slice = dst_views.0.as_slice_mut(0, dst_views.0.len());
            for r in 14..16 {
                let row_idx = border + mb_row_usize * 16 + r;
                let row_start = row_idx * y_stride;
                let src_val = dst_y_slice[row_start + border + y_width - 1];
                let dst_start = row_start + border + y_width;
                for i in 0..4 {
                    dst_y_slice[dst_start + i] = src_val;
                }
            }

            let uv_border = border / 2;
            let dst_u_slice = dst_views.1.as_slice_mut(0, dst_views.1.len());
            let dst_v_slice = dst_views.2.as_slice_mut(0, dst_views.2.len());
            for r in 6..8 {
                let row_idx = uv_border + mb_row_usize * 8 + r;
                let row_start = row_idx * uv_stride;
                
                let src_val_u = dst_u_slice[row_start + uv_border + uv_width - 1];
                let dst_start_u = row_start + uv_border + uv_width;
                
                let src_val_v = dst_v_slice[row_start + uv_border + uv_width - 1];
                let dst_start_v = row_start + uv_border + uv_width;
                
                for i in 0..4 {
                    dst_u_slice[dst_start_u + i] = src_val_u;
                    dst_v_slice[dst_start_v + i] = src_val_v;
                }
            }
        }
        
        vpx_atomic_store_release(current_mb_col, mb_col + nsync);
        xd.mode_info_idx += 1;
        xd.up_available = 1;
        xd.mode_info_idx += (xd.mode_info_stride as usize) * (decoding_thread_count as usize);
        mb_row = (mb_row as u32).wrapping_add(decoding_thread_count.wrapping_add(1)) as i32;
        }
    }
    
    if last_mb_row + decoding_thread_count as i32 + 1 >= pc.mb_rows {
        if let Some(ref end_sem) = mt_sync.h_event_end_decoding {
            end_sem.signal();
        }
    }
}
fn thread_decoding_proc(
    ithread: i32,
    pbi_raw: SendPtr<VP8D_COMP>,
    mbrd: std::sync::Arc<std::sync::Mutex<MB_ROW_DEC>>,
) {
    let pbi = unsafe { &*pbi_raw.0 };
    
    while vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0 {
        let start_decoding_sem = &pbi.mt_sync.h_event_start_decoding.as_ref().unwrap()[ithread as usize];
        start_decoding_sem.wait();
        if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) == 0 {
            break;
        }
        let mut mbrd_guard = mbrd.lock().unwrap();
        let xd = &mut mbrd_guard.mbd;
        let setjmp_val = unsafe { setjmp(&raw mut xd.error_info.jmp as *mut ::core::ffi::c_int) };
        if setjmp_val != 0 {
            xd.error_info.setjmp = 0;
            pbi.mt_sync.h_event_end_decoding.as_ref().unwrap().signal();
        } else {
            xd.error_info.setjmp = 1;
            let decoding_thread_count = pbi.decoding_thread_count;
            let fragments = pbi.fragments;
            let common = &pbi.common;
            let mbc_raw = pbi.mbc.as_ptr() as *mut vp8_reader;
            let mt_sync = &pbi.mt_sync;
            mt_decode_mb_rows(common, mbc_raw, mt_sync, xd, ithread + 1, decoding_thread_count, fragments);
            xd.error_info.setjmp = 0;
        }
    }
}
pub fn vp8_decoder_create_threads(pbi: &mut VP8D_COMP) -> Result<(), &'static str> {
    let pbi_raw = SendPtr(pbi as *const VP8D_COMP);
    let mut core_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ithread: ::core::ffi::c_uint = 0;
    vpx_atomic_init(&pbi.b_multithreaded_rd, 0 as ::core::ffi::c_int);
    pbi.allocated_decoding_thread_count = 0 as ::core::ffi::c_int;
    core_count = if pbi.max_threads > 8 as ::core::ffi::c_int {
        8 as ::core::ffi::c_int
    } else {
        pbi.max_threads
    };
    if core_count > pbi.common.processor_core_count {
        core_count = pbi.common.processor_core_count;
    }
    if core_count > 1 as ::core::ffi::c_int {
        vpx_atomic_init(&pbi.b_multithreaded_rd, 1 as ::core::ffi::c_int);
        pbi.decoding_thread_count =
            (core_count - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        let count = pbi.decoding_thread_count as usize;
        
        let mut start_semaphores = Vec::with_capacity(count);
        for _ in 0..count {
            start_semaphores.push(std::sync::Arc::new(crate::thread_shim::Semaphore::new(0)));
        }
        pbi.mt_sync.h_event_start_decoding = Some(start_semaphores.into_boxed_slice());
        
        pbi.mt_sync.h_event_end_decoding = Some(std::sync::Arc::new(crate::thread_shim::Semaphore::new(0)));
        
        let mut threads = Vec::new();
        threads.resize_with(count, || None);
        pbi.mt_sync.h_decoding_thread = Some(threads.into_boxed_slice());
        
        let mut mb_row_di_vec = Vec::with_capacity(count);
        for _ in 0..count {
            mb_row_di_vec.push(std::sync::Arc::new(std::sync::Mutex::new(MB_ROW_DEC::default())));
        }
        pbi.mb_row_di = Some(mb_row_di_vec.into_boxed_slice());
        
        let h_decoding_thread = pbi.mt_sync.h_decoding_thread.as_mut().unwrap();
        let mb_row_di = pbi.mb_row_di.as_mut().unwrap();
        
        ithread = 0 as ::core::ffi::c_uint;
        while ithread < pbi.decoding_thread_count {
            let mbrd_arc = std::sync::Arc::clone(&mb_row_di[ithread as usize]);
            let ithread_i32 = ithread as i32;
            
            let builder = std::thread::Builder::new();
            match builder.spawn(move || {
                thread_decoding_proc(ithread_i32, pbi_raw, mbrd_arc);
            }) {
                Ok(handle) => {
                    h_decoding_thread[ithread as usize] = Some(handle);
                    ithread = ithread.wrapping_add(1);
                }
                Err(_) => {
                    break;
                }
            }
        }
        pbi.allocated_decoding_thread_count = ithread as ::core::ffi::c_int;
        if pbi.allocated_decoding_thread_count
            != pbi.decoding_thread_count as ::core::ffi::c_int
        {
            if pbi.allocated_decoding_thread_count == 0 as ::core::ffi::c_int {
                pbi.mt_sync.h_event_end_decoding = None;
            }
            return Err("Failed to create threads");
        }
    }
    Ok(())
}
pub fn vp8mt_de_alloc_temp_buffers(
    pbi: &mut VP8D_COMP,
    _mb_rows: ::core::ffi::c_int,
) {
    pbi.mt_sync.mt_current_mb_col = None;
    pbi.mt_sync.mt_yabove_row = None;
    pbi.mt_sync.mt_uabove_row = None;
    pbi.mt_sync.mt_vabove_row = None;
    pbi.mt_sync.mt_yleft_col = None;
    pbi.mt_sync.mt_uleft_col = None;
    pbi.mt_sync.mt_vleft_col = None;
    pbi.mt_sync.mt_yabove_row_allocs = None;
    pbi.mt_sync.mt_uabove_row_allocs = None;
    pbi.mt_sync.mt_vabove_row_allocs = None;
    pbi.mt_sync.mt_yleft_col_allocs = None;
    pbi.mt_sync.mt_uleft_col_allocs = None;
    pbi.mt_sync.mt_vleft_col_allocs = None;
}
pub fn vp8mt_alloc_temp_buffers(
    pbi: &mut VP8D_COMP,
    mut width: ::core::ffi::c_int,
    prev_mb_rows: ::core::ffi::c_int,
) {
    let mut uv_width: ::core::ffi::c_int = 0;
    if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0 {
        vp8mt_de_alloc_temp_buffers(pbi, prev_mb_rows);
        if width & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            width += 16 as ::core::ffi::c_int - (width & 0xf as ::core::ffi::c_int);
        }
        if width < 640 as ::core::ffi::c_int {
            pbi.mt_sync.sync_range = 1 as ::core::ffi::c_int;
        } else if width <= 1280 as ::core::ffi::c_int {
            pbi.mt_sync.sync_range = 8 as ::core::ffi::c_int;
        } else if width <= 2560 as ::core::ffi::c_int {
            pbi.mt_sync.sync_range = 16 as ::core::ffi::c_int;
        } else {
            pbi.mt_sync.sync_range = 32 as ::core::ffi::c_int;
        }
        uv_width = width >> 1 as ::core::ffi::c_int;
        let mb_rows_usize = pbi.common.mb_rows as usize;
        
        let mut current_mb_col_vec = vec![vpx_atomic_int { value: core::sync::atomic::AtomicI32::new(0) }; mb_rows_usize];
        for i in 0..mb_rows_usize {
            vpx_atomic_init(&current_mb_col_vec[i], 0);
        }
        pbi.mt_sync.mt_current_mb_col = Some(current_mb_col_vec.into_boxed_slice());
        
        // mt_yabove_row
        let mut yabove_allocs = Vec::with_capacity(mb_rows_usize);
        let mut yabove_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (width + ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_yabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                yabove_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            yabove_allocs.push(ab);
        }
        pbi.mt_sync.mt_yabove_row = Some(yabove_views.into_boxed_slice());
        pbi.mt_sync.mt_yabove_row_allocs = Some(yabove_allocs.into_boxed_slice());
        
        // mt_uabove_row
        let mut uabove_allocs = Vec::with_capacity(mb_rows_usize);
        let mut uabove_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_uabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                uabove_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            uabove_allocs.push(ab);
        }
        pbi.mt_sync.mt_uabove_row = Some(uabove_views.into_boxed_slice());
        pbi.mt_sync.mt_uabove_row_allocs = Some(uabove_allocs.into_boxed_slice());
        
        // mt_vabove_row
        let mut vabove_allocs = Vec::with_capacity(mb_rows_usize);
        let mut vabove_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_vabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                vabove_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            vabove_allocs.push(ab);
        }
        pbi.mt_sync.mt_vabove_row = Some(vabove_views.into_boxed_slice());
        pbi.mt_sync.mt_vabove_row_allocs = Some(vabove_allocs.into_boxed_slice());
        
        // mt_yleft_col
        let mut yleft_allocs = Vec::with_capacity(mb_rows_usize);
        let mut yleft_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 16usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_yleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                yleft_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            yleft_allocs.push(ab);
        }
        pbi.mt_sync.mt_yleft_col = Some(yleft_views.into_boxed_slice());
        pbi.mt_sync.mt_yleft_col_allocs = Some(yleft_allocs.into_boxed_slice());
        
        // mt_uleft_col
        let mut uleft_allocs = Vec::with_capacity(mb_rows_usize);
        let mut uleft_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 8usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_uleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                uleft_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            uleft_allocs.push(ab);
        }
        pbi.mt_sync.mt_uleft_col = Some(uleft_views.into_boxed_slice());
        pbi.mt_sync.mt_uleft_col_allocs = Some(uleft_allocs.into_boxed_slice());
        
        // mt_vleft_col
        let mut vleft_allocs = Vec::with_capacity(mb_rows_usize);
        let mut vleft_views = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 8usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_vleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
                vleft_views.push(UnsafeRowView::new(b.as_ptr(), size));
            }
            vleft_allocs.push(ab);
        }
        pbi.mt_sync.mt_vleft_col = Some(vleft_views.into_boxed_slice());
        pbi.mt_sync.mt_vleft_col_allocs = Some(vleft_allocs.into_boxed_slice());
    }
}
pub fn vp8_decoder_remove_threads(pbi: &mut VP8D_COMP) {
    if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0 {
        vpx_atomic_store_release(&pbi.b_multithreaded_rd, 0);
        
        let h_event_start_decoding = pbi.mt_sync.h_event_start_decoding.as_ref().unwrap();
        let h_decoding_thread = pbi.mt_sync.h_decoding_thread.as_mut().unwrap();
        
        let mut i: i32 = 0;
        while i < pbi.allocated_decoding_thread_count {
            h_event_start_decoding[i as usize].signal();
            if let Some(handle) = h_decoding_thread[i as usize].take() {
                let _ = handle.join();
            }
            i += 1;
        }
        pbi.mt_sync.h_decoding_thread = None;
        pbi.mt_sync.h_event_start_decoding = None;
        pbi.mt_sync.h_event_end_decoding = None;
        pbi.mb_row_di = None;
        
        let mb_rows = pbi.common.mb_rows;
        vp8mt_de_alloc_temp_buffers(pbi, mb_rows);
    }
}
pub fn vp8mt_decode_mb_rows(
    pbi: &mut VP8D_COMP,
) -> ::core::ffi::c_int {
    let num_part = (1 as ::core::ffi::c_int) << pbi.common.multi_token_partition as ::core::ffi::c_uint;
    println!(
        "DEBUG: RUNNING MULTITHREADED DECODER! partitions {} threads {}",
        num_part,
        pbi.decoding_thread_count + 1
    );
    let pc_ref = &mut pbi.common;
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_int = 0;
    let filter_level: ::core::ffi::c_int = pc_ref.filter_level;
    
    let new_fb_idx = pc_ref.new_fb_idx as usize;
    let y_width = pc_ref.yv12_fb[new_fb_idx].y_width;
    let mut filter_branch = false;
    
    if filter_level != 0 {
        let mt_sync = &mut pbi.mt_sync;
        let yabove_allocs = mt_sync.mt_yabove_row_allocs.as_mut().unwrap();
        let uabove_allocs = mt_sync.mt_uabove_row_allocs.as_mut().unwrap();
        let vabove_allocs = mt_sync.mt_vabove_row_allocs.as_mut().unwrap();
        let yleft_allocs = mt_sync.mt_yleft_col_allocs.as_mut().unwrap();
        let uleft_allocs = mt_sync.mt_uleft_col_allocs.as_mut().unwrap();
        let vleft_allocs = mt_sync.mt_vleft_col_allocs.as_mut().unwrap();

        let len = (y_width + 5) as usize;
        let len_uv = ((y_width >> 1) + 5) as usize;

        yabove_allocs[0].as_mut().unwrap().as_slice_mut()[31 .. 31 + len].fill(127);
        uabove_allocs[0].as_mut().unwrap().as_slice_mut()[15 .. 15 + len_uv].fill(127);
        vabove_allocs[0].as_mut().unwrap().as_slice_mut()[15 .. 15 + len_uv].fill(127);

        j = 1;
        while j < pc_ref.mb_rows {
            yabove_allocs[j as usize].as_mut().unwrap().as_slice_mut()[31] = 129;
            uabove_allocs[j as usize].as_mut().unwrap().as_slice_mut()[15] = 129;
            vabove_allocs[j as usize].as_mut().unwrap().as_slice_mut()[15] = 129;
            j += 1;
        }

        j = 0;
        while j < pc_ref.mb_rows {
            yleft_allocs[j as usize].as_mut().unwrap().as_slice_mut()[0 .. 16].fill(129);
            uleft_allocs[j as usize].as_mut().unwrap().as_slice_mut()[0 .. 8].fill(129);
            vleft_allocs[j as usize].as_mut().unwrap().as_slice_mut()[0 .. 8].fill(129);
            j += 1;
        }
        filter_branch = true;
    }
    
    if filter_branch {
        vp8_loop_filter_frame_init(pc_ref, &pbi.mb, filter_level);
    } else {
        let full_slice = pc_ref.yv12_fb_allocs[new_fb_idx].as_mut().unwrap().as_slice_mut();
        vp8_setup_intra_recon_top_line(&pc_ref.yv12_fb[new_fb_idx], full_slice);
    }
    
    let mb_row_di = pbi.mb_row_di.as_mut().unwrap();
    
    setup_decoding_thread_data(
        &pbi.common,
        pbi.mt_sync.mt_current_mb_col.as_deref(),
        &pbi.mb,
        mb_row_di,
    );

    let h_event_start_decoding = pbi.mt_sync.h_event_start_decoding.as_ref().unwrap();

    i = 0;
    while i < pbi.decoding_thread_count {
        h_event_start_decoding[i as usize].signal();
        i = i.wrapping_add(1);
    }
    
    let xd = &mut pbi.mb;
    let mt_sync = &mut pbi.mt_sync;
    
    let setjmp_res = unsafe { setjmp(&raw mut xd.error_info.jmp as *mut ::core::ffi::c_int) };
    if setjmp_res != 0 {
        xd.error_info.setjmp = 0;
        xd.corrupted = 1;
        i = 0;
        while i < pbi.decoding_thread_count {
            mt_sync.h_event_end_decoding.as_ref().unwrap().wait();
            i = i.wrapping_add(1);
        }
        return -1;
    }
    
    xd.error_info.setjmp = 1;
    
    let decoding_thread_count = pbi.decoding_thread_count;
    let fragments = pbi.fragments;
    let common = &pbi.common;
    let mbc_raw = pbi.mbc.as_mut_ptr();
    
    mt_decode_mb_rows(common, mbc_raw, mt_sync, xd, 0, decoding_thread_count, fragments);
    
    xd.error_info.setjmp = 0;
    
    i = 0;
    while i < pbi.decoding_thread_count.wrapping_add(1) {
        mt_sync.h_event_end_decoding.as_ref().unwrap().wait();
        i = i.wrapping_add(1);
    }
    
    0}

