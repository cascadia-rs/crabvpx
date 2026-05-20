use crate::vp8::decoder::dboolhuff::SafeBoolDecoder;
use crate::vp8::common::entropy::{vp8_coef_update_probs, vp8_mb_feature_data_bits};
use crate::vp8::decoder::detokenize::{vp8_decode_mb_tokens, vp8_reset_mb_tokens_context};
use crate::vp8::decoder::decodemv::vp8_decode_mode_mvs;
use crate::vp8::common::vp8_loopfilter::{vp8_loop_filter_frame_init, vp8_loop_filter_row_normal_safe, vp8_loop_filter_row_simple_safe};
use crate::vp8::common::quant_common::{vp8_ac_yquant, vp8_dc_quant, vp8_dc2quant, vp8_ac2quant, vp8_dc_uv_quant, vp8_ac_uv_quant};
use crate::vpx_scale::generic::yv12extend::vp8_yv12_extend_frame_borders_c;
use crate::vp8::common::extend::vp8_extend_mb_row;
use crate::vp8::common::reconintra::intra_prediction_down_copy;
use crate::vp8::common::idctllm::vp8_short_inv_walsh4x4_1_safe;
use crate::vp8::common::dequantize::vp8_dequantize_b_safe;
use crate::vp8::common::idctllm::vp8_short_inv_walsh4x4_safe;
use crate::vp8::common::idct_blk::{vp8_dequant_idct_add_y_block_safe, vp8_dequant_idct_add_uv_block_safe};
use crate::vp8::common::dequantize::vp8_dequant_idct_add_safe;
use crate::vp8::common::idctllm::vp8_dc_only_idct_add_safe;
use crate::vp8::common::safe_predict::*;
use crate::vp8::decoder::threading::vp8_decoder_remove_threads;
use crate::vp8::common::alloccommon::vp8_setup_version;
use crate::vp8::common::reconintra4x4::vp8_intra4x4_predict_safe;
use crate::vp8::decoder::threading::vp8mt_decode_mb_rows;
use crate::vp8::common::entropymode::vp8_init_mbmode_probs;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_LVL_MAX: C2RustUnnamed = 2;
pub const MB_LVL_ALT_LF: C2RustUnnamed = 1;
pub const MB_LVL_ALT_Q: C2RustUnnamed = 0;
pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
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
fn vp8dx_safe_bool_error(br: &SafeBoolDecoder) -> ::core::ffi::c_int {
    if br.count > VP8_BD_VALUE_SIZE && br.count < VP8_LOTS_OF_BITS {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const MB_FEATURE_TREE_PROBS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_MB_SEGMENTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_REF_LF_DELTAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_MODE_LF_DELTAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEGMENT_DELTADATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEGMENT_ABSDATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENTROPY_NODES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const BLOCK_TYPES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COEF_BANDS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PREV_COEF_CONTEXTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXQ: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const QINDEX_RANGE: ::core::ffi::c_int = MAXQ + 1 as ::core::ffi::c_int;
#[inline]
fn vpx_atomic_load_acquire(
    atomic: &vpx_atomic_int,
) -> ::core::ffi::c_int {
    atomic.value.load(core::sync::atomic::Ordering::Acquire)
}

pub(crate) fn setup_intra_recon_left(
    ybf: &mut YV12_BUFFER_CONFIG,
    mb_row: ::core::ffi::c_int,
) {
    let y_border = ybf.border as usize;
    let y_stride = ybf.y_stride as usize;
    let uv_border = (ybf.border / 2) as usize;
    let uv_stride = ybf.uv_stride as usize;
    let mb_row = mb_row as usize;

    let y_slice = ybf.y_slice_mut_safe();
    let y_base = (y_border + mb_row * 16) * y_stride + y_border - 1;
    for i in 0..16 {
        let idx = y_base + i * y_stride;
        if idx < y_slice.len() {
            y_slice[idx] = 129;
        } else {
            debug_assert!(false, "Y slice overflow in setup_intra_recon_left");
        }
    }

    let u_slice = ybf.u_slice_mut_safe();
    let u_base = (uv_border + mb_row * 8) * uv_stride + uv_border - 1;
    for i in 0..8 {
        let idx = u_base + i * uv_stride;
        if idx < u_slice.len() {
            u_slice[idx] = 129;
        } else {
            debug_assert!(false, "U slice overflow in setup_intra_recon_left");
        }
    }

    let v_slice = ybf.v_slice_mut_safe();
    let v_base = (uv_border + mb_row * 8) * uv_stride + uv_border - 1;
    for i in 0..8 {
        let idx = v_base + i * uv_stride;
        if idx < v_slice.len() {
            v_slice[idx] = 129;
        } else {
            debug_assert!(false, "V slice overflow in setup_intra_recon_left");
        }
    }
}
pub fn vp8cx_init_de_quantizer(pbi: &mut VP8D_COMP) {
    let pc = &mut pbi.common;
    let mut Q: ::core::ffi::c_int = 0;
    while Q < QINDEX_RANGE {
        pc.Y1dequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc_quant(Q, pc.y1dc_delta_q) as ::core::ffi::c_short;
        pc.Y2dequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc2quant(Q, pc.y2dc_delta_q) as ::core::ffi::c_short;
        pc.UVdequant[Q as usize][0 as ::core::ffi::c_int as usize] =
            vp8_dc_uv_quant(Q, pc.uvdc_delta_q) as ::core::ffi::c_short;
        pc.Y1dequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac_yquant(Q) as ::core::ffi::c_short;
        pc.Y2dequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac2quant(Q, pc.y2ac_delta_q) as ::core::ffi::c_short;
        pc.UVdequant[Q as usize][1 as ::core::ffi::c_int as usize] =
            vp8_ac_uv_quant(Q, pc.uvac_delta_q) as ::core::ffi::c_short;
        Q += 1;
    }
}
pub fn vp8_mb_init_dequantizer(
    pc: &VP8_COMMON,
    xd: &mut MACROBLOCKD,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut QIndex: ::core::ffi::c_int = 0;
    let mbmi = &xd.mode_info(pc.mip_ptr()).mbmi;
    if xd.segmentation_enabled != 0 {
        if xd.mb_segment_abs_delta as ::core::ffi::c_int == SEGMENT_ABSDATA {
            QIndex = xd.segment_feature_data[MB_LVL_ALT_Q as ::core::ffi::c_int as usize]
                [mbmi.segment_id as usize] as ::core::ffi::c_int;
        } else {
            QIndex = pc.base_qindex
                + xd.segment_feature_data[MB_LVL_ALT_Q as ::core::ffi::c_int as usize]
                    [mbmi.segment_id as usize] as ::core::ffi::c_int;
        }
        QIndex = if QIndex >= 0 as ::core::ffi::c_int {
            if QIndex <= MAXQ {
                QIndex
            } else {
                MAXQ
            }
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        QIndex = pc.base_qindex;
    }
    xd.dequant_y1_dc[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_short;
    xd.dequant_y1[0 as ::core::ffi::c_int as usize] =
        pc.Y1dequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    xd.dequant_y2[0 as ::core::ffi::c_int as usize] =
        pc.Y2dequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    xd.dequant_uv[0 as ::core::ffi::c_int as usize] =
        pc.UVdequant[QIndex as usize][0 as ::core::ffi::c_int as usize];
    i = 1 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        xd.dequant_y1[i as usize] =
            pc.Y1dequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        xd.dequant_y1_dc[i as usize] = xd.dequant_y1[i as usize];
        xd.dequant_y2[i as usize] =
            pc.Y2dequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        xd.dequant_uv[i as usize] =
            pc.UVdequant[QIndex as usize][1 as ::core::ffi::c_int as usize];
        i += 1;
    }
}
fn decode_macroblock(
    common: &VP8_COMMON,
    safe_decoders: &mut [SafeBoolDecoder],
    xd: &mut MACROBLOCKD,
    mb_idx: ::core::ffi::c_uint,
    above: &mut ENTROPY_CONTEXT_PLANES,
    left: &mut ENTROPY_CONTEXT_PLANES,
) {
    let mut mode: MB_PREDICTION_MODE = DC_PRED;
    let mut i: ::core::ffi::c_int = 0;
    let mip = common.mip_ptr();
    if xd.mode_info(mip).mbmi.mb_skip_coeff != 0 {
        let is_4x4 = xd.mode_info(mip).mbmi.is_4x4 != 0;
        vp8_reset_mb_tokens_context(above, left, is_4x4);
    } else if vp8dx_safe_bool_error(&safe_decoders[xd.current_bc_idx]) == 0 {
        let mut eobtotal: ::core::ffi::c_int = 0;
        let is_4x4 = xd.mode_info(mip).mbmi.is_4x4 != 0;
        let bc_idx = xd.current_bc_idx;
        let qcoeff = &mut xd.qcoeff;
        let eobs = &mut xd.eobs;
        eobtotal = vp8_decode_mb_tokens(
            &mut safe_decoders[bc_idx],
            &common.fc,
            qcoeff,
            eobs,
            above,
            left,
            is_4x4,
        );
        xd.mode_info_mut(common.mip_ptr()).mbmi.mb_skip_coeff =
            (eobtotal == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
    }
    mode = xd.mode_info(mip).mbmi.mode as MB_PREDICTION_MODE;

    if xd.segmentation_enabled != 0 {
        vp8_mb_init_dequantizer(common, xd);
    }
    if xd.mode_info(mip).mbmi.ref_frame as ::core::ffi::c_int
        == INTRA_FRAME as ::core::ffi::c_int
    {
        let uvmode = xd.mode_info(mip).mbmi.uv_mode as MB_PREDICTION_MODE;
        let left_available = xd.left_available;
        let up_available = xd.up_available;
        let left_stride_uv = xd.recon_left_stride[1] as usize;
        let left_stride_y = xd.recon_left_stride[0] as usize;

        let uv_stride = xd.dst.uv_stride as usize;
        let uv_border = (xd.dst.border / 2) as usize;
        let uv_buffer_offset = uv_border * uv_stride + uv_border;
        let dst_stride = xd.dst.y_stride;
        let dst_stride_us = dst_stride as usize;
        let border = xd.dst.border as usize;
        let y_buffer_offset = border * dst_stride_us + border;


        let mut uabove = [0u8; 9];
        let mut vabove = [0u8; 9];
        let mut uleft = [0u8; 8];
        let mut vleft = [0u8; 8];

        {
            let (u_slice, v_slice) = xd.dst.uv_slices_mut_with_offset_safe(0);
            uabove.copy_from_slice(&u_slice[uv_buffer_offset - uv_stride - 1 .. uv_buffer_offset - uv_stride + 8]);
            vabove.copy_from_slice(&v_slice[uv_buffer_offset - uv_stride - 1 .. uv_buffer_offset - uv_stride + 8]);

            for i in 0..8 {
                uleft[i] = u_slice[uv_buffer_offset - 1 + i * left_stride_uv];
                vleft[i] = v_slice[uv_buffer_offset - 1 + i * left_stride_uv];
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
            let dst_y_slice = xd.dst.y_slice_mut_safe();
            
            let mut yabove = [0u8; 17];
            yabove.copy_from_slice(&dst_y_slice[y_buffer_offset - dst_stride_us - 1 .. y_buffer_offset - dst_stride_us + 16]);
            
            let mut yleft = [0u8; 16];
            for i in 0..16 {
                yleft[i] = dst_y_slice[y_buffer_offset - 1 + i * left_stride_y];
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
            if xd.mode_info(mip).mbmi.mb_skip_coeff != 0 {
                xd.eobs.fill(0);
            }
            intra_prediction_down_copy(xd, None);
            
            let b_modes = {
                let mi = xd.mode_info(mip);
                let mut modes = [0 as B_PREDICTION_MODE; 16];
                for idx in 0..16 {
                    modes[idx] = mi.bmi[idx].mode();
                }
                modes
            };

            let dst_y_slice = xd.dst.y_slice_mut_safe();
            
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let b_offset = xd.block[i as usize].offset;
                let b_mode = b_modes[i as usize];
                let dst_offset = y_buffer_offset + b_offset as usize;
                
                let above_idx = dst_offset - dst_stride as usize;
                let yleft_idx = dst_offset - 1;
                
                let mut above_buf = [0u8; 8];
                above_buf.copy_from_slice(&dst_y_slice[above_idx .. above_idx + 8]);
                let top_left_val = dst_y_slice[above_idx - 1];
                
                let mut left_buf = [0u8; 4];
                for r in 0..4 {
                    left_buf[r] = dst_y_slice[yleft_idx + r * dst_stride as usize];
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
        crate::vp8::common::reconinter::vp8_build_inter_predictors_mb(xd, mip);
    }

    if xd.mode_info(mip).mbmi.mb_skip_coeff == 0 {
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
 
            let y_stride = xd.dst.y_stride;
            let (dst_y_view, _, _) = xd.dst.views_mut();
            let q_y: &mut [i16; 256] = (&mut xd.qcoeff[0..256]).try_into().unwrap();
            let dst_len = 15 * y_stride as usize + 16;
            let dst_slice = &mut dst_y_view[..dst_len];
            let eobs_y: &[::core::ffi::c_char; 16] = (&xd.eobs[0..16]).try_into().unwrap();
 
            vp8_dequant_idct_add_y_block_safe(q_y, dq_y, dst_slice, y_stride, eobs_y);
        }
 
        let uv_stride = xd.dst.uv_stride;
        let (_, dst_u_view, dst_v_view) = xd.dst.views_mut();
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
fn get_delta_q(
    bc: &mut SafeBoolDecoder,
    prev: i32,
    q_update: &mut i32,
) -> i32 {
    let mut ret_val = 0;
    if bc.read_bool(vp8_prob_half as i32) != 0 {
        ret_val = bc.read_literal(4);
        if bc.read_bool(vp8_prob_half as i32) != 0 {
            ret_val = -ret_val;
        }
    }
    if ret_val != prev {
        *q_update = 1;
    }
    ret_val
}

fn yv12_extend_frame_top_c(ybf: &mut YV12_BUFFER_CONFIG) {
    let border = ybf.border as usize;

    // Y plane
    let y_stride = ybf.y_stride as usize;
    let y_slice = ybf.y_slice_mut_safe();
    let y_src_start = border * y_stride;
    let y_src_end = y_src_start + y_stride;

    for r in 0..border {
        let dest_start = r * y_stride;
        y_slice.copy_within(y_src_start..y_src_end, dest_start);
    }

    // U plane
    let uv_border = border / 2;
    let uv_stride = ybf.uv_stride as usize;
    let u_slice = ybf.u_slice_mut_safe();
    let u_src_start = uv_border * uv_stride;
    let u_src_end = u_src_start + uv_stride;

    for r in 0..uv_border {
        let dest_start = r * uv_stride;
        u_slice.copy_within(u_src_start..u_src_end, dest_start);
    }

    // V plane
    let v_slice = ybf.v_slice_mut_safe();
    let v_src_start = uv_border * uv_stride;
    let v_src_end = v_src_start + uv_stride;

    for r in 0..uv_border {
        let dest_start = r * uv_stride;
        v_slice.copy_within(v_src_start..v_src_end, dest_start);
    }
}
fn yv12_extend_frame_bottom_c(ybf: &mut YV12_BUFFER_CONFIG) {
    let border = ybf.border as usize;

    // Y plane
    let y_stride = ybf.y_stride as usize;
    let y_height = ybf.y_height as usize;
    let y_slice = ybf.y_slice_mut_safe();
    let y_src_start = (border + y_height - 1) * y_stride;
    let y_src_end = y_src_start + y_stride;

    for r in 0..border {
        let dest_start = (border + y_height + r) * y_stride;
        y_slice.copy_within(y_src_start..y_src_end, dest_start);
    }

    // U plane
    let uv_border = border / 2;
    let uv_stride = ybf.uv_stride as usize;
    let uv_height = ybf.uv_height as usize;
    let u_slice = ybf.u_slice_mut_safe();
    let u_src_start = (uv_border + uv_height - 1) * uv_stride;
    let u_src_end = u_src_start + uv_stride;

    for r in 0..uv_border {
        let dest_start = (uv_border + uv_height + r) * uv_stride;
        u_slice.copy_within(u_src_start..u_src_end, dest_start);
    }

    // V plane
    let v_slice = ybf.v_slice_mut_safe();
    let v_src_start = (uv_border + uv_height - 1) * uv_stride;
    let v_src_end = v_src_start + uv_stride;

    for r in 0..uv_border {
        let dest_start = (uv_border + uv_height + r) * uv_stride;
        v_slice.copy_within(v_src_start..v_src_end, dest_start);
    }
}
fn yv12_extend_frame_left_right(
    ybf: &mut YV12_BUFFER_CONFIG,
    mb_row: i32,
) {
    let border = ybf.border as usize;

    // Y Plane
    let y_stride = ybf.y_stride as usize;
    let y_width = ybf.y_width as usize;
    let y_slice = ybf.y_slice_mut_safe();
    
    let y_plane_height = 16;
    for i in 0..y_plane_height {
        let y_line = mb_row as usize * 16 + i;
        let active_line_start_idx = (border * y_stride + border) + y_line * y_stride;
        
        let first_val = y_slice[active_line_start_idx];
        let last_val = y_slice[active_line_start_idx + y_width - 1];
        
        let left_border_start = active_line_start_idx - border;
        y_slice[left_border_start..active_line_start_idx].fill(first_val);
        
        let right_border_start = active_line_start_idx + y_width;
        let right_border_end = right_border_start + border;
        y_slice[right_border_start..right_border_end].fill(last_val);
    }

    // U Plane
    let uv_border = border / 2;
    let uv_stride = ybf.uv_stride as usize;
    let uv_width = ybf.uv_width as usize;
    let u_slice = ybf.u_slice_mut_safe();
    
    let uv_plane_height = 8;
    for i in 0..uv_plane_height {
        let uv_line = mb_row as usize * 8 + i;
        let active_line_start_idx = (uv_border * uv_stride + uv_border) + uv_line * uv_stride;
        
        let first_val = u_slice[active_line_start_idx];
        let last_val = u_slice[active_line_start_idx + uv_width - 1];
        
        let left_border_start = active_line_start_idx - uv_border;
        u_slice[left_border_start..active_line_start_idx].fill(first_val);
        
        let right_border_start = active_line_start_idx + uv_width;
        let right_border_end = right_border_start + uv_border;
        u_slice[right_border_start..right_border_end].fill(last_val);
    }

    // V Plane
    let v_slice = ybf.v_slice_mut_safe();
    for i in 0..uv_plane_height {
        let uv_line = mb_row as usize * 8 + i;
        let active_line_start_idx = (uv_border * uv_stride + uv_border) + uv_line * uv_stride;
        
        let first_val = v_slice[active_line_start_idx];
        let last_val = v_slice[active_line_start_idx + uv_width - 1];
        
        let left_border_start = active_line_start_idx - uv_border;
        v_slice[left_border_start..active_line_start_idx].fill(first_val);
        
        let right_border_start = active_line_start_idx + uv_width;
        let right_border_end = right_border_start + uv_border;
        v_slice[right_border_start..right_border_end].fill(last_val);
    }
}
fn decode_mb_rows(pbi: &mut VP8D_COMP) {
    let (xd, pc, mbc) = pbi.split_mut();
    let num_active_partitions = (1 as ::core::ffi::c_int) << pc.multi_token_partition as ::core::ffi::c_uint;
    let mut safe_decoders: Vec<SafeBoolDecoder> = mbc[0..num_active_partitions as usize]
        .iter()
        .map(|bc| SafeBoolDecoder::from_bool_decoder(bc))
        .collect();
    let mut ibc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_part: ::core::ffi::c_int = num_active_partitions;
    let mut recon_yoffset: ::core::ffi::c_int = 0;
    let mut recon_uvoffset: ::core::ffi::c_int = 0;
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut mb_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let new_fb_idx = pc.new_fb_idx as usize;
    let mut recon_y_stride: ::core::ffi::c_int = pc.yv12_fb[new_fb_idx].y_stride;
    let mut recon_uv_stride: ::core::ffi::c_int = pc.yv12_fb[new_fb_idx].uv_stride;
    let mut y_offset: usize = 0;
    let mut u_offset: usize = 0;
    let mut v_offset: usize = 0;
    let mut extended_row: i32 = 0;
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
        ref_fb_corrupted[i as usize] = this_fb.corrupted;
        i += 1;
    }
    xd.up_available = 0 as ::core::ffi::c_int;
    if pc.filter_level != 0 {
        let filter_level = pc.filter_level;
        vp8_loop_filter_frame_init(pc, xd, filter_level);
    }
    vp8_setup_intra_recon_top_line(&mut pc.yv12_fb[new_fb_idx]);
    
    let mut above_context_box = pc.above_context.take().unwrap();
    
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < pc.mb_rows {
        if num_part > 1 as ::core::ffi::c_int {
            xd.current_bc_idx = ibc as usize;
            ibc += 1;
            if ibc == num_part {
                ibc = 0 as ::core::ffi::c_int;
            }
        }
        recon_yoffset = mb_row * recon_y_stride * 16 as ::core::ffi::c_int;
        recon_uvoffset = mb_row * recon_uv_stride * 8 as ::core::ffi::c_int;
        
        let mut left_context = ENTROPY_CONTEXT_PLANES::default();
        xd.left_available = 0 as ::core::ffi::c_int;
        xd.mb_to_top_edge = -((mb_row * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
        xd.mb_to_bottom_edge = ((pc.mb_rows - 1 as ::core::ffi::c_int - mb_row)
            * 16 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int;

        xd.recon_left_stride[0 as ::core::ffi::c_int as usize] = xd.dst.y_stride;
        xd.recon_left_stride[1 as ::core::ffi::c_int as usize] = xd.dst.uv_stride;
        setup_intra_recon_left(
            &mut pc.yv12_fb[new_fb_idx],
            mb_row,
        );
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < pc.mb_cols {
            xd.mb_to_left_edge =
                -((mb_col * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
            xd.mb_to_right_edge = ((pc.mb_cols - 1 as ::core::ffi::c_int - mb_col)
                * 16 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int;

            let (y_ptr, u_ptr, v_ptr) = {
                let new_fb = &mut pc.yv12_fb[new_fb_idx];
                let (y_slice, u_slice, v_slice) = new_fb.views_mut();
                (
                    y_slice[recon_yoffset as usize..].as_mut_ptr(),
                    u_slice[recon_uvoffset as usize..].as_mut_ptr(),
                    v_slice[recon_uvoffset as usize..].as_mut_ptr(),
                )
            };
            xd.dst.y_buffer = y_ptr;
            xd.dst.u_buffer = u_ptr;
            xd.dst.v_buffer = v_ptr;

            let ref_frame = xd.mode_info(pc.mip_ptr()).mbmi.ref_frame;
            
            if ref_frame as ::core::ffi::c_int
                >= LAST_FRAME as ::core::ffi::c_int
            {
                let ref_0: MV_REFERENCE_FRAME = ref_frame as MV_REFERENCE_FRAME;
                let fb_idx = match ref_0 {
                    LAST_FRAME => pc.lst_fb_idx as usize,
                    GOLDEN_FRAME => pc.gld_fb_idx as usize,
                    ALTREF_FRAME => pc.alt_fb_idx as usize,
                    _ => panic!("Invalid ref frame"),
                };
                let (pre_y, pre_u, pre_v) = {
                    let ref_fb = &pc.yv12_fb[fb_idx];
                    let border = ref_fb.border as usize;
                    let stride = ref_fb.y_stride as usize;
                    let uv_border = border / 2;
                    let uv_stride = ref_fb.uv_stride as usize;
                    
                    let y_slice = ref_fb.y_slice_safe();
                    let u_slice = ref_fb.u_slice_safe();
                    let v_slice = ref_fb.v_slice_safe();
                    
                    let y_offset = border * stride + border + recon_yoffset as usize;
                    let u_offset = uv_border * uv_stride + uv_border + recon_uvoffset as usize;
                    let v_offset = uv_border * uv_stride + uv_border + recon_uvoffset as usize;
                    
                    (
                        y_slice[y_offset..].as_ptr() as *mut u8,
                        u_slice[u_offset..].as_ptr() as *mut u8,
                        v_slice[v_offset..].as_ptr() as *mut u8,
                    )
                };
                xd.pre.y_buffer = pre_y;
                xd.pre.u_buffer = pre_u;
                xd.pre.v_buffer = pre_v;
            } else {
                xd.pre.y_buffer = ::core::ptr::null_mut::<uint8_t>();
                xd.pre.u_buffer = ::core::ptr::null_mut::<uint8_t>();
                xd.pre.v_buffer = ::core::ptr::null_mut::<uint8_t>();
            }
            xd.corrupted |= ref_fb_corrupted[ref_frame as usize];
            
            let above = &mut above_context_box[mb_col as usize];
            decode_macroblock(pc, &mut safe_decoders, xd, mb_idx as ::core::ffi::c_uint, above, &mut left_context);
            
            mb_idx += 1;
            xd.left_available = 1 as ::core::ffi::c_int;
            xd.corrupted |= vp8dx_safe_bool_error(&safe_decoders[xd.current_bc_idx]);
            
            recon_yoffset += 16 as ::core::ffi::c_int;
            recon_uvoffset += 8 as ::core::ffi::c_int;
            
            xd.mode_info_idx += 1;
            mb_col += 1;
        }
        vp8_extend_mb_row(
            &mut pc.yv12_fb[new_fb_idx],
            mb_row,
        );
        xd.mode_info_idx += 1;
        xd.up_available = 1 as ::core::ffi::c_int;
        if pc.filter_level != 0 {
            if mb_row > 0 as ::core::ffi::c_int {
                let (y_slice, u_slice, v_slice) = pc.yv12_fb[new_fb_idx].views_mut();

                let stride = pc.mode_info_stride as usize;
                let mip_slice = pc.mip.as_ref().unwrap();
                let mode_info_idx = mb_row as usize * stride + 1;

                if pc.filter_type as ::core::ffi::c_uint
                    == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    vp8_loop_filter_row_normal_safe(
                        pc.mb_cols,
                        &pc.lf_info,
                        pc.frame_type,
                        mip_slice,
                        mode_info_idx,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        recon_uv_stride,
                        y_slice,
                        y_offset,
                        Some(&mut *u_slice),
                        u_offset,
                        Some(&mut *v_slice),
                        v_offset,
                    );
                } else {
                    vp8_loop_filter_row_simple_safe(
                        pc.mb_cols,
                        &pc.lf_info,
                        mip_slice,
                        mode_info_idx,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        y_slice,
                        y_offset,
                    );
                }
                if mb_row > 1 as ::core::ffi::c_int {
                    yv12_extend_frame_left_right(&mut pc.yv12_fb[new_fb_idx], extended_row);
                    extended_row += 1;
                }
                y_offset = (y_offset as isize + (recon_y_stride * 16 as ::core::ffi::c_int) as isize) as usize;
                u_offset = (u_offset as isize + (recon_uv_stride * 8 as ::core::ffi::c_int) as isize) as usize;
                v_offset = (v_offset as isize + (recon_uv_stride * 8 as ::core::ffi::c_int) as isize) as usize;
            }
        } else if mb_row > 0 as ::core::ffi::c_int {
            yv12_extend_frame_left_right(&mut pc.yv12_fb[new_fb_idx], extended_row);
            extended_row += 1;
        }
        mb_row += 1;
    }
    if pc.filter_level != 0 {
        let (y_slice, u_slice, v_slice) = pc.yv12_fb[new_fb_idx].views_mut();

        let stride = pc.mode_info_stride as usize;
        let mip_slice = pc.mip.as_ref().unwrap();
        let mode_info_idx = mb_row as usize * stride + 1;

        if pc.filter_type as ::core::ffi::c_uint
            == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            vp8_loop_filter_row_normal_safe(
                pc.mb_cols,
                &pc.lf_info,
                pc.frame_type,
                mip_slice,
                mode_info_idx,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                recon_uv_stride,
                y_slice,
                y_offset,
                Some(&mut *u_slice),
                u_offset,
                Some(&mut *v_slice),
                v_offset,
            );
        } else {
            vp8_loop_filter_row_simple_safe(
                pc.mb_cols,
                &pc.lf_info,
                mip_slice,
                mode_info_idx,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                y_slice,
                y_offset,
            );
        }
        yv12_extend_frame_left_right(&mut pc.yv12_fb[new_fb_idx], extended_row);
        extended_row += 1;
    }
    yv12_extend_frame_left_right(&mut pc.yv12_fb[new_fb_idx], extended_row);
    yv12_extend_frame_top_c(&mut pc.yv12_fb[new_fb_idx]);
    yv12_extend_frame_bottom_c(&mut pc.yv12_fb[new_fb_idx]);
    
    pc.above_context = Some(above_context_box);
    for (bc, safe_bc) in mbc[0..safe_decoders.len()].iter_mut().zip(safe_decoders.iter()) {
        safe_bc.update_bool_decoder(bc);
    }
}
fn read_partition_size(
    pbi: &VP8D_COMP,
    cx_size: &[u8],
) -> ::core::ffi::c_uint {
    let mut temp: [::core::ffi::c_uchar; 3] = [0; 3];
    let mut data_slice = cx_size;
    if let Some(decrypt_cb) = pbi.decrypt_cb {
        unsafe {
            decrypt_cb(
                pbi.decrypt_state,
                cx_size.as_ptr(),
                temp.as_mut_ptr(),
                3 as ::core::ffi::c_int,
            );
        }
        data_slice = &temp;
    }
    (data_slice[0] as ::core::ffi::c_int
        + ((data_slice[1] as ::core::ffi::c_int) << 8)
        + ((data_slice[2] as ::core::ffi::c_int) << 16)) as ::core::ffi::c_uint
}
fn read_available_partition_size(
    pbi: &mut VP8D_COMP,
    token_part_sizes: &[u8],
    fragment: &[u8],
    i: ::core::ffi::c_int,
    num_part: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut partition_size: ::core::ffi::c_uint = 0;
    let bytes_left = fragment.len();
    if i < num_part - 1 {
        let size_offset = (i * 3) as usize;
        if size_offset + 3 <= token_part_sizes.len() {
            partition_size = read_partition_size(pbi, &token_part_sizes[size_offset .. size_offset + 3]);
        } else if pbi.ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            pbi.common.error.trigger(
                VPX_CODEC_CORRUPT_FRAME,
                "Truncated partition size data",
            );
        }
    } else {
        partition_size = bytes_left as ::core::ffi::c_uint;
    }
    if partition_size as usize > bytes_left {
        if pbi.ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            pbi.common.error.trigger(
                VPX_CODEC_CORRUPT_FRAME,
                &format!("Truncated packet or corrupt partition {} length", i + 1),
            );
        }
    }
    partition_size
}
fn setup_token_decoder(
    pbi: &mut VP8D_COMP,
    token_part_sizes: &[u8],
    first_partition_length: usize,
    safe_decoder: &mut SafeBoolDecoder,
) {
    let mut multi_token_partition: TOKEN_PARTITION =
        safe_decoder.read_literal(2) as TOKEN_PARTITION;
    if safe_decoder.count <= VP8_BD_VALUE_SIZE || safe_decoder.count >= VP8_LOTS_OF_BITS {
        pbi.common.multi_token_partition = multi_token_partition;
    }

    let num_token_partitions = (1 as usize) << pbi.common.multi_token_partition as usize;
    
    let mut new_ptrs: [*const u8; 9] = [core::ptr::null(); 9];
    let mut new_sizes: [u32; 9] = [0; 9];
    let mut new_slices: [Option<&[u8]>; 9] = [None; 9];
    let mut new_count = 0;

    let mut fragment_idx = 0;
    
    let fragments = pbi.fragments;
    let orig_count = fragments.count as usize;

    let mut target_partition_idx = 0;

    while fragment_idx < orig_count && target_partition_idx < num_token_partitions + 1 {
        let mut current_remaining = match fragments.get_slice(fragment_idx) {
            Some(slice) => {
                if slice.is_empty() {
                    fragment_idx += 1;
                    continue;
                }
                slice
            }
            None => {
                fragment_idx += 1;
                continue;
            }
        };
        
        if fragment_idx == 0 {
            let ext_first_part_size = first_partition_length + 3 * (num_token_partitions - 1);
            if current_remaining.len() < ext_first_part_size {
                pbi.common.error.trigger(
                    VPX_CODEC_CORRUPT_FRAME,
                    &format!("Corrupted fragment size {}", current_remaining.len()),
                );
            }
            
            let (first_part, remaining) = current_remaining.split_at(ext_first_part_size);
            
            new_ptrs[0] = first_part.as_ptr();
            new_sizes[0] = first_part.len() as u32;
            new_slices[0] = Some(first_part);
            new_count = 1;
            target_partition_idx = 1;
            
            current_remaining = remaining;
        }
        
        while !current_remaining.is_empty() && target_partition_idx < num_token_partitions + 1 {
            let partition_size = read_available_partition_size(
                pbi,
                token_part_sizes,
                current_remaining,
                (target_partition_idx - 1) as i32,
                num_token_partitions as i32,
            ) as usize;
            
            if current_remaining.len() < partition_size {
                pbi.common.error.trigger(
                    VPX_CODEC_CORRUPT_FRAME,
                    &format!("Corrupted fragment size {}", current_remaining.len()),
                );
            }
            
            let (partition, next_remaining) = current_remaining.split_at(partition_size);
            
            new_ptrs[target_partition_idx] = partition.as_ptr();
            new_sizes[target_partition_idx] = partition.len() as u32;
            new_slices[target_partition_idx] = Some(partition);
            new_count = target_partition_idx + 1;
            
            target_partition_idx += 1;
            current_remaining = next_remaining;
        }
        
        fragment_idx += 1;
    }
    
    pbi.fragments.count = new_count as u32;
    for i in 0..9 {
        pbi.fragments.ptrs[i] = new_ptrs[i];
        pbi.fragments.sizes[i] = new_sizes[i];
    }

    let mut partition_idx = 1;
    while partition_idx < new_count {
        if let Some(slice) = new_slices[partition_idx] {
            crate::vp8::decoder::dboolhuff::vp8dx_start_decode_safe(
                &mut pbi.mbc[(partition_idx - 1) as usize],
                slice,
                pbi.decrypt_cb,
                pbi.decrypt_state,
            );
        }
        partition_idx = partition_idx.wrapping_add(1);
    }
    if pbi.decoding_thread_count > num_token_partitions.wrapping_sub(1) as u32 {
        pbi.decoding_thread_count = num_token_partitions.wrapping_sub(1) as u32;
    }
    if pbi.decoding_thread_count as ::core::ffi::c_int
        > pbi.common.mb_rows - 1 as ::core::ffi::c_int
    {
        pbi.decoding_thread_count =
            (pbi.common.mb_rows - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
}
fn init_frame(pbi: &mut VP8D_COMP) {
    if pbi.common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pbi.common.fc.mvc = crate::vp8::common::entropymv::vp8_default_mv_context;
        vp8_init_mbmode_probs(&mut pbi.common);
        crate::vp8::common::entropy::vp8_default_coef_probs(&mut pbi.common.fc.coef_probs);
        pbi.mb.segment_feature_data = [[0; 4]; 2];
        pbi.mb.mb_segment_abs_delta = SEGMENT_DELTADATA as ::core::ffi::c_uchar;
        pbi.mb.ref_lf_deltas = [0; 4];
        pbi.mb.mode_lf_deltas = [0; 4];
        pbi.common.refresh_golden_frame = 1 as ::core::ffi::c_int;
        pbi.common.refresh_alt_ref_frame = 1 as ::core::ffi::c_int;
        pbi.common.copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        pbi.common.copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        pbi.common.ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
        pbi.common.ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
    } else {
        if pbi.common.use_bilinear_mc_filter == 0 {
            pbi.mb.subpixel_predict = Some(safe_vp8_sixtap_predict4x4_neon);
            pbi.mb.subpixel_predict8x4 = Some(safe_vp8_sixtap_predict8x4_neon);
            pbi.mb.subpixel_predict8x8 = Some(safe_vp8_sixtap_predict8x8_neon);
            pbi.mb.subpixel_predict16x16 = Some(safe_vp8_sixtap_predict16x16_neon);
        } else {
            pbi.mb.subpixel_predict = Some(safe_vp8_bilinear_predict4x4_neon);
            pbi.mb.subpixel_predict8x4 = Some(safe_vp8_bilinear_predict8x4_neon);
            pbi.mb.subpixel_predict8x8 = Some(safe_vp8_bilinear_predict8x8_neon);
            pbi.mb.subpixel_predict16x16 = Some(safe_vp8_bilinear_predict16x16_neon);
        }
        if pbi.decoded_key_frame != 0 && pbi.ec_enabled != 0 && pbi.ec_active == 0 {
            pbi.ec_active = 1 as ::core::ffi::c_int;
        }
    }
    pbi.mb.mode_info_idx = (pbi.common.mode_info_stride + 1) as usize;
    pbi.mb.above_context_idx = 0;
    pbi.mb.frame_type = pbi.common.frame_type;
    let mip = pbi.common.mip_ptr();
    pbi.mb.mode_info_mut(mip).mbmi.mode = DC_PRED as ::core::ffi::c_int as uint8_t;
    pbi.mb.mode_info_stride = pbi.common.mode_info_stride;
    pbi.mb.corrupted = 0 as ::core::ffi::c_int;
    pbi.mb.fullpixel_mask = !(0 as ::core::ffi::c_int);
    if pbi.common.full_pixel != 0 {
        pbi.mb.fullpixel_mask = !(7 as ::core::ffi::c_int);
    }
}
pub fn vp8_decode_frame(pbi: &mut VP8D_COMP) -> ::core::ffi::c_int {
    let fragments = pbi.fragments;
    let data_slice = fragments.get_slice(0).unwrap_or(&[]);
    
    let mut data_idx = 0;
    let mut first_partition_length_in_bytes: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;

    let mut corrupt_tokens: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prev_independent_partitions: ::core::ffi::c_int = pbi.independent_partitions;
    
    let new_fb_idx = pbi.common.new_fb_idx as usize;
    pbi.common.yv12_fb[new_fb_idx].corrupted = 0 as ::core::ffi::c_int;
    pbi.mb.corrupted = 0 as ::core::ffi::c_int;
    
    let mut clear_buffer: [::core::ffi::c_uchar; 10] = [0; 10];
    let mut clear_slice = data_slice;
    let mut is_decrypted = false;

    if data_slice.len() < 3 {
        if pbi.ec_active == 0 {
            pbi.common.error.trigger(
                VPX_CODEC_CORRUPT_FRAME,
                "Truncated packet",
            );
        }
        pbi.common.frame_type = INTER_FRAME;
        pbi.common.version = 0 as ::core::ffi::c_int;
        pbi.common.show_frame = 1 as ::core::ffi::c_int;
        first_partition_length_in_bytes = 0 as ::core::ffi::c_int;
    } else {
        if pbi.decrypt_cb.is_some() {
            let n = std::cmp::min(10, data_slice.len());
            unsafe {
                pbi.decrypt_cb.expect("non-null function pointer")(
                    pbi.decrypt_state,
                    data_slice.as_ptr(),
                    clear_buffer.as_mut_ptr(),
                    n as i32,
                );
            }
            clear_slice = &clear_buffer[..n];
            is_decrypted = true;
        }
        
        pbi.common.frame_type = (clear_slice[0] as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as FRAME_TYPE;
        pbi.common.version = (clear_slice[0] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int);
        pbi.common.show_frame = (clear_slice[0] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 1 as ::core::ffi::c_int);
        
        first_partition_length_in_bytes = (
            (clear_slice[0] as ::core::ffi::c_int)
            | (clear_slice[1] as ::core::ffi::c_int) << 8
            | (clear_slice[2] as ::core::ffi::c_int) << 16
        ) >> 5;
        
        data_idx += 3;
        if is_decrypted {
            clear_slice = &clear_slice[3..];
        } else {
            clear_slice = &data_slice[data_idx..];
        }
        
        vp8_setup_version(&mut pbi.common);
        if pbi.common.frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (data_slice.len() - data_idx) >= 7 {
                if clear_slice[0] as ::core::ffi::c_int != 0x9d as ::core::ffi::c_int
                    || clear_slice[1] as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int
                    || clear_slice[2] as ::core::ffi::c_int != 0x2a as ::core::ffi::c_int
                {
                    pbi.common.error.trigger(
                        VPX_CODEC_UNSUP_BITSTREAM,
                        "Invalid frame sync code",
                    );
                }
                pbi.common.Width = ((clear_slice[3] as ::core::ffi::c_int
                    | (clear_slice[4] as ::core::ffi::c_int) << 8)
                    & 0x3fff as ::core::ffi::c_int);
                pbi.common.horiz_scale = (clear_slice[4] as ::core::ffi::c_int >> 6);
                pbi.common.Height = ((clear_slice[5] as ::core::ffi::c_int
                    | (clear_slice[6] as ::core::ffi::c_int) << 8)
                    & 0x3fff as ::core::ffi::c_int);
                pbi.common.vert_scale = (clear_slice[6] as ::core::ffi::c_int >> 6);
                data_idx += 7;
            } else if pbi.ec_active == 0 {
                pbi.common.error.trigger(
                    VPX_CODEC_CORRUPT_FRAME,
                    "Truncated key frame header",
                );
            } else {
                data_idx = data_slice.len();
            }
        } else {
            pbi.mb.pre = pbi.common.yv12_fb[new_fb_idx];
            pbi.mb.dst = pbi.common.yv12_fb[new_fb_idx];
        }
    }
    if pbi.decoded_key_frame == 0
        && pbi.common.frame_type as ::core::ffi::c_uint
            != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if pbi.ec_active == 0
        && (data_slice.len() - data_idx) < first_partition_length_in_bytes as usize
    {
        pbi.common.error.trigger(
            VPX_CODEC_CORRUPT_FRAME,
            "Truncated packet or corrupt partition 0 length",
        );
    }
    
    init_frame(pbi);
    let size = data_slice.len() - data_idx;
    if size != 0 {
        let active_data = &data_slice[data_idx .. data_idx + size];
        crate::vp8::decoder::dboolhuff::vp8dx_start_decode_safe(
            &mut pbi.mbc[8],
            active_data,
            pbi.decrypt_cb,
            pbi.decrypt_state,
        );
    }
    
    let mut safe_decoder = SafeBoolDecoder::from_bool_decoder(&pbi.mbc[8]);

    
    if pbi.common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_decoder.read_bool(vp8_prob_half as i32);
        pbi.common.clamp_type =
            safe_decoder.read_bool(vp8_prob_half as i32) as CLAMP_TYPE;
    }
    pbi.mb.segmentation_enabled =
        safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
    if pbi.mb.segmentation_enabled != 0 {
        pbi.mb.update_mb_segmentation_map =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        pbi.mb.update_mb_segmentation_data =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        if pbi.mb.update_mb_segmentation_data != 0 {
            pbi.mb.mb_segment_abs_delta =
                safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
            pbi.mb.segment_feature_data = [[0; 4]; 2];
            i = 0 as ::core::ffi::c_int;
            while i < MB_LVL_MAX as ::core::ffi::c_int {
                j = 0 as ::core::ffi::c_int;
                while j < MAX_MB_SEGMENTS {
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        pbi.mb.segment_feature_data[i as usize][j as usize] = safe_decoder.read_literal(vp8_mb_feature_data_bits[i as usize]) as ::core::ffi::c_schar;
                        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                            pbi.mb.segment_feature_data[i as usize][j as usize] = -(pbi.mb
                                .segment_feature_data[i as usize][j as usize]
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_schar;
                        }
                    } else {
                        pbi.mb.segment_feature_data[i as usize][j as usize] =
                            0 as ::core::ffi::c_schar;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        if pbi.mb.update_mb_segmentation_map != 0 {
            pbi.mb.mb_segment_tree_probs = [255; 3];
            i = 0 as ::core::ffi::c_int;
            while i < MB_FEATURE_TREE_PROBS {
                if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                    pbi.mb.mb_segment_tree_probs[i as usize] =
                        safe_decoder.read_literal(8) as vp8_prob;
                }
                i += 1;
            }
        }
    } else {
        pbi.mb.update_mb_segmentation_map = 0 as ::core::ffi::c_uchar;
        pbi.mb.update_mb_segmentation_data = 0 as ::core::ffi::c_uchar;
    }
    pbi.common.filter_type =
        safe_decoder.read_bool(vp8_prob_half as i32) as LOOPFILTERTYPE;
    pbi.common.filter_level = safe_decoder.read_literal(6);
    pbi.common.sharpness_level = safe_decoder.read_literal(3);
    pbi.mb.mode_ref_lf_delta_update = 0 as ::core::ffi::c_uchar;
    pbi.mb.mode_ref_lf_delta_enabled =
        safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
    if pbi.mb.mode_ref_lf_delta_enabled != 0 {
        pbi.mb.mode_ref_lf_delta_update =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        if pbi.mb.mode_ref_lf_delta_update != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < MAX_REF_LF_DELTAS {
                if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                    pbi.mb.ref_lf_deltas[i as usize] =
                        safe_decoder.read_literal(6) as ::core::ffi::c_schar;
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        pbi.mb.ref_lf_deltas[i as usize] = (pbi.mb.ref_lf_deltas[i as usize]
                            as ::core::ffi::c_int
                            * -(1 as ::core::ffi::c_int))
                            as ::core::ffi::c_schar;
                    }
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < MAX_MODE_LF_DELTAS {
                if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                    pbi.mb.mode_lf_deltas[i as usize] =
                        safe_decoder.read_literal(6) as ::core::ffi::c_schar;
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        pbi.mb.mode_lf_deltas[i as usize] = (pbi.mb.mode_lf_deltas[i as usize]
                            as ::core::ffi::c_int
                            * -(1 as ::core::ffi::c_int))
                            as ::core::ffi::c_schar;
                    }
                }
                i += 1;
            }
        }
    }
    
    let token_part_sizes_len = data_slice.len() - (data_idx + first_partition_length_in_bytes as usize);
    let token_part_sizes_slice = &data_slice[data_idx + first_partition_length_in_bytes as usize .. ];
    let token_part_sizes_offset = data_idx + first_partition_length_in_bytes as usize;
    
    setup_token_decoder(
        pbi,
        token_part_sizes_slice,
        token_part_sizes_offset,
        &mut safe_decoder,
    );
    pbi.mb.current_bc_idx = 0;

    let mut Q: ::core::ffi::c_int = 0;
    let mut q_update: ::core::ffi::c_int = 0;
    Q = safe_decoder.read_literal(7);
    pbi.common.base_qindex = Q;
    q_update = 0 as ::core::ffi::c_int;
    pbi.common.y1dc_delta_q = get_delta_q(&mut safe_decoder, pbi.common.y1dc_delta_q, &mut q_update);
    pbi.common.y2dc_delta_q = get_delta_q(&mut safe_decoder, pbi.common.y2dc_delta_q, &mut q_update);
    pbi.common.y2ac_delta_q = get_delta_q(&mut safe_decoder, pbi.common.y2ac_delta_q, &mut q_update);
    pbi.common.uvdc_delta_q = get_delta_q(&mut safe_decoder, pbi.common.uvdc_delta_q, &mut q_update);
    pbi.common.uvac_delta_q = get_delta_q(&mut safe_decoder, pbi.common.uvac_delta_q, &mut q_update);
    if q_update != 0 {
        vp8cx_init_de_quantizer(pbi);
    }
    vp8_mb_init_dequantizer(&pbi.common, &mut pbi.mb);
    
    if pbi.common.frame_type as ::core::ffi::c_uint
        != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pbi.common.refresh_golden_frame = safe_decoder.read_bool(vp8_prob_half as i32);
        pbi.common.refresh_alt_ref_frame = safe_decoder.read_bool(vp8_prob_half as i32);
        pbi.common.copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        if pbi.common.refresh_golden_frame == 0 {
            pbi.common.copy_buffer_to_gf = safe_decoder.read_literal(2);
        }
        pbi.common.copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        if pbi.common.refresh_alt_ref_frame == 0 {
            pbi.common.copy_buffer_to_arf = safe_decoder.read_literal(2);
        }
        pbi.common.ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] = safe_decoder.read_bool(vp8_prob_half as i32);
        pbi.common.ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] = safe_decoder.read_bool(vp8_prob_half as i32);
    }
    
    pbi.common.refresh_entropy_probs =
        safe_decoder.read_bool(vp8_prob_half as i32);
    if pbi.common.refresh_entropy_probs == 0 as ::core::ffi::c_int {
        pbi.common.lfc = pbi.common.fc;
    }
    pbi.common.refresh_last_frame = (pbi.common.frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        || safe_decoder.read_bool(vp8_prob_half as i32) != 0)
        as ::core::ffi::c_int;
        
    pbi.independent_partitions = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BLOCK_TYPES {
        j = 0 as ::core::ffi::c_int;
        while j < COEF_BANDS {
            k = 0 as ::core::ffi::c_int;
            while k < PREV_COEF_CONTEXTS {
                l = 0 as ::core::ffi::c_int;
                while l < ENTROPY_NODES {
                    if safe_decoder.read_bool(vp8_coef_update_probs[i as usize][j as usize][k as usize][l as usize] as i32) != 0 {
                        pbi.common.fc.coef_probs[i as usize][j as usize][k as usize][l as usize] = safe_decoder.read_literal(8) as vp8_prob;
                    }
                    if k > 0 as ::core::ffi::c_int
                        && pbi.common.fc.coef_probs[i as usize][j as usize][k as usize][l as usize] as ::core::ffi::c_int
                            != pbi.common.fc.coef_probs[i as usize][j as usize]
                                [(k - 1 as ::core::ffi::c_int) as usize]
                                [l as usize] as ::core::ffi::c_int
                    {
                        pbi.independent_partitions = 0 as ::core::ffi::c_int;
                    }
                    l += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    
    pbi.mb.qcoeff = [0; 400];
    let mut mip = pbi.common.mip.take().unwrap();
    vp8_decode_mode_mvs(pbi, &mut mip, &mut safe_decoder);
    pbi.common.mip = Some(mip);
    
    safe_decoder.update_bool_decoder(&mut pbi.mbc[8]);
    
    if let Some(ref mut above_context) = pbi.common.above_context {
        above_context.fill(ENTROPY_CONTEXT_PLANES::default());
    }
    pbi.frame_corrupt_residual = 0 as ::core::ffi::c_int;
    
    if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0
        && pbi.common.multi_token_partition as ::core::ffi::c_uint
            != ONE_PARTITION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut thread: ::core::ffi::c_uint = 0;
        if vp8mt_decode_mb_rows(pbi) != 0 {
            vp8_decoder_remove_threads(pbi);
            pbi.restart_threads = 1 as ::core::ffi::c_int;
            pbi.common.error.trigger(
                VPX_CODEC_CORRUPT_FRAME,
                "",
            );
        }
        
        vp8_yv12_extend_frame_borders_c(&mut pbi.common.yv12_fb[new_fb_idx]);
        thread = 0 as ::core::ffi::c_uint;
        while thread < pbi.decoding_thread_count {
            corrupt_tokens |= pbi.mb_row_di.as_ref().unwrap()[thread as usize].mbd.corrupted;
            thread = thread.wrapping_add(1);
        }
    } else {
        decode_mb_rows(pbi);
        corrupt_tokens |= pbi.mb.corrupted;
    }
    
    pbi.common.yv12_fb[new_fb_idx].corrupted = vp8dx_bool_error(&pbi.mbc[8]);
    pbi.common.yv12_fb[new_fb_idx].corrupted |= corrupt_tokens;
    
    if pbi.decoded_key_frame == 0 {
        if pbi.common.frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
            && pbi.common.yv12_fb[new_fb_idx].corrupted == 0
        {
            pbi.decoded_key_frame = 1 as ::core::ffi::c_int;
        } else {
            pbi.common.error.trigger(
                VPX_CODEC_CORRUPT_FRAME,
                "A stream must start with a complete key frame",
            );
        }
    }
    
    if pbi.common.refresh_entropy_probs == 0 as ::core::ffi::c_int {
        pbi.common.fc = pbi.common.lfc;
        pbi.independent_partitions = prev_independent_partitions;
    }
    return 0 as ::core::ffi::c_int;
}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
