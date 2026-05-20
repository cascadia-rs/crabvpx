use crate::vp8::decoder::detokenize::{vp8_decode_mb_tokens, vp8_reset_mb_tokens_context};
use crate::vp8::common::vp8_loopfilter::vp8_loop_filter_frame_init;
use crate::vp8::decoder::decodeframe::vp8_mb_init_dequantizer;
use crate::vp8::common::mbpitch::vp8_setup_block_dptrs;
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
}

unsafe extern "C" {
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vpx_memalign(align: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
}
static mach_task_self_: mach_port_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
pub type __darwin_natural_t = ::core::ffi::c_uint;
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
pub struct _opaque_pthread_attr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub type kern_return_t = ::core::ffi::c_int;
pub type task_t = mach_port_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const THREAD_EXIT_SUCCESS: *mut ::core::ffi::c_void = NULL;
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
    mbrd: &mut [MB_ROW_DEC],
) {
    for m in mbrd.iter_mut() {
        let mbd = &mut m.mbd;
        mbd.subpixel_predict = xd.subpixel_predict;
        mbd.subpixel_predict8x4 = xd.subpixel_predict8x4;
        mbd.subpixel_predict8x8 = xd.subpixel_predict8x8;
        mbd.subpixel_predict16x16 = xd.subpixel_predict16x16;
        mbd.frame_type = pc.frame_type;
        mbd.pre = xd.pre;
        mbd.dst = xd.dst;
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
) {
    let mut mode: MB_PREDICTION_MODE = DC_PRED;
    let mut i: ::core::ffi::c_int = 0;
    let mip = common.mip_ptr();
    if xd.mode_info(mip).mbmi.mb_skip_coeff != 0 {
        let is_4x4 = xd.mode_info(mip).mbmi.is_4x4 != 0;
        let (above, left) = xd.contexts_mut(common.above_context_ptr(), left_context);
        vp8_reset_mb_tokens_context(above, left, is_4x4);
    } else if vp8dx_safe_bool_error(safe_decoder) == 0 {
        let mut eobtotal: ::core::ffi::c_int = 0;
        let is_4x4 = xd.mode_info(mip).mbmi.is_4x4 != 0;
        let (above, left, qcoeff, eobs) = xd.decode_tokens_inputs_mut(common.above_context_ptr(), left_context);
        eobtotal = vp8_decode_mb_tokens(
            safe_decoder,
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
            let dst_y_slice = xd.dst.y_slice_mut_safe();
            
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
            if xd.mode_info(mip).mbmi.mb_skip_coeff != 0 {
                xd.eobs.fill(0);
            }
            intra_prediction_down_copy(xd, above_y);
            
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
fn mt_decode_mb_rows(
    common: &mut VP8_COMMON,
    mbc: &mut [vp8_reader; 9],
    mt_sync: &mut VP8D_MT_SYNC,
    xd: &mut MACROBLOCKD,
    start_mb_row: ::core::ffi::c_int,
    decoding_thread_count: ::core::ffi::c_uint,
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
    
    let mip = pc.mip_ptr();
    mb_row = start_mb_row;
    while mb_row < pc.mb_rows {
        let mut recon_yoffset: ::core::ffi::c_int = 0;
        let mut recon_uvoffset: ::core::ffi::c_int = 0;
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut filter_level: ::core::ffi::c_int = 0;
        
        let lfi_n = &pc.lf_info;
        
        last_mb_row = mb_row;
        xd.current_bc_idx = (mb_row % num_part) as usize;
        let bc_idx = xd.current_bc_idx;
        
        let mut safe_decoder = crate::vp8::decoder::dboolhuff::SafeBoolDecoder::from_bool_decoder(&mbc[bc_idx]);
        
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
        
        if pc.filter_level != 0 {
            xd.recon_left_stride[0] = 1;
            xd.recon_left_stride[1] = 1;
        } else {
            xd.recon_left_stride[0] = xd.dst.y_stride;
            xd.recon_left_stride[1] = xd.dst.uv_stride;
            
            let yv12_fb_new_ref = &mut pc.yv12_fb[new_fb_idx];
            crate::vp8::decoder::decodeframe::setup_intra_recon_left(yv12_fb_new_ref, mb_row);
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
            
            xd.dst.y_buffer = &slice_y[y_offset] as *const u8 as *mut u8;
            xd.dst.u_buffer = &slice_u[uv_offset] as *const u8 as *mut u8;
            xd.dst.v_buffer = &slice_v[uv_offset] as *const u8 as *mut u8;
            
            let cur_ref_frame = xd.mode_info(mip).mbmi.ref_frame;
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
                let this_fb = &pc.yv12_fb[fb_idx];
                let slice_y = this_fb.y_slice_safe();
                let slice_u = this_fb.u_slice_safe();
                let slice_v = this_fb.v_slice_safe();
                
                let border_y = this_fb.border as usize;
                let stride_y = this_fb.y_stride as usize;
                let active_start_y = border_y * stride_y + border_y;
                
                let border_uv = (this_fb.border / 2) as usize;
                let stride_uv = this_fb.uv_stride as usize;
                let active_start_uv = border_uv * stride_uv + border_uv;
                
                let y_offset = active_start_y + recon_yoffset as usize;
                let uv_offset = active_start_uv + recon_uvoffset as usize;
                
                xd.pre.y_buffer = &slice_y[y_offset] as *const u8 as *mut u8;
                xd.pre.u_buffer = &slice_u[uv_offset] as *const u8 as *mut u8;
                xd.pre.v_buffer = &slice_v[uv_offset] as *const u8 as *mut u8;
            } else {
                xd.pre.y_buffer = ::core::ptr::null_mut();
                xd.pre.u_buffer = ::core::ptr::null_mut();
                xd.pre.v_buffer = ::core::ptr::null_mut();
            }
            
            let (above_y, above_u, above_v, left_y, left_u, left_v) = if pc.filter_level != 0 {
                let mb_row_us = mb_row as usize;
                let mb_col_us = mb_col as usize;
                
                let ay = &mt_sync.mt_yabove_row.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()
                    [31 + mb_col_us * 16 .. 31 + mb_col_us * 16 + 24];
                let au = &mt_sync.mt_uabove_row.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()
                    [15 + mb_col_us * 8 .. 15 + mb_col_us * 8 + 9];
                let av = &mt_sync.mt_vabove_row.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()
                    [15 + mb_col_us * 8 .. 15 + mb_col_us * 8 + 9];
                    
                let ly = &mt_sync.mt_yleft_col.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()[0..16];
                let lu = &mt_sync.mt_uleft_col.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()[0..8];
                let lv = &mt_sync.mt_vleft_col.as_ref().unwrap()[mb_row_us].as_ref().unwrap().as_slice()[0..8];
                
                (Some(ay), Some(au), Some(av), Some(ly), Some(lu), Some(lv))
            } else {
                (None, None, None, None, None, None)
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
            );
            
            xd.left_available = 1;
            xd.corrupted |= vp8dx_safe_bool_error(&safe_decoder);
            
            if pc.filter_level != 0 {
                let cur_mbmi = &xd.mode_info(mip).mbmi;
                let skip_lf = (cur_mbmi.mode as ::core::ffi::c_int != B_PRED as ::core::ffi::c_int
                    && cur_mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int
                    && cur_mbmi.mb_skip_coeff as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
                let mode_index = lfi_n.mode_lf_lut[cur_mbmi.mode as usize] as ::core::ffi::c_int;
                let seg = cur_mbmi.segment_id as usize;
                let ref_frame = cur_mbmi.ref_frame as usize;
                filter_level = lfi_n.lvl[seg][ref_frame][mode_index as usize] as ::core::ffi::c_int;
                
                if mb_row != pc.mb_rows - 1 {
                    let border = xd.dst.border as usize;
                    let stride = xd.dst.y_stride as usize;
                    let src_idx = (border + 15) * stride + border;
                    let src_slice = &xd.dst.y_slice_safe()[src_idx..src_idx + 16];
                    
                    let dst_ab = mt_sync.mt_yabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                    let border_uv = (xd.dst.border / 2) as usize;
                    let stride_uv = xd.dst.uv_stride as usize;
                    let src_idx_u = (border_uv + 7) * stride_uv + border_uv;
                    let src_slice_u = &xd.dst.u_slice_safe()[src_idx_u..src_idx_u + 8];
                    let src_slice_v = &xd.dst.v_slice_safe()[src_idx_u..src_idx_u + 8];
                    let dst_ab_u = mt_sync.mt_uabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                    let dst_ab_v = mt_sync.mt_vabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                    
                    let offset = 32 + (mb_col * 16) as usize;
                    dst_ab.as_slice_mut()[offset..offset + 16].copy_from_slice(src_slice);
                    
                    let offset_uv = 16 + (mb_col * 8) as usize;
                    dst_ab_u.as_slice_mut()[offset_uv..offset_uv + 8].copy_from_slice(src_slice_u);
                    dst_ab_v.as_slice_mut()[offset_uv..offset_uv + 8].copy_from_slice(src_slice_v);
                }
                
                if mb_col != pc.mb_cols - 1 {
                    let next_mbmi = &pc.mip.as_ref().unwrap()[xd.mode_info_idx + 1].mbmi;
                    if next_mbmi.ref_frame as ::core::ffi::c_int == INTRA_FRAME as ::core::ffi::c_int {
                        let border = xd.dst.border as usize;
                        let stride = xd.dst.y_stride as usize;
                        let y_slice = xd.dst.y_slice_safe();
                        let dst_ab = mt_sync.mt_yleft_col.as_mut().unwrap()[mb_row as usize].as_mut().unwrap();
                        let border_uv = (xd.dst.border / 2) as usize;
                        let stride_uv = xd.dst.uv_stride as usize;
                        let u_slice = xd.dst.u_slice_safe();
                        let v_slice = xd.dst.v_slice_safe();
                        let dst_ab_u = mt_sync.mt_uleft_col.as_mut().unwrap()[mb_row as usize].as_mut().unwrap();
                        let dst_ab_v = mt_sync.mt_vleft_col.as_mut().unwrap()[mb_row as usize].as_mut().unwrap();
                        
                        let dst_slice = dst_ab.as_slice_mut();
                        for i in 0..16 {
                            let src_idx = border * stride + border + i * stride + 15;
                            dst_slice[i] = y_slice[src_idx];
                        }
                        
                        let dst_slice_u = dst_ab_u.as_slice_mut();
                        let dst_slice_v = dst_ab_v.as_slice_mut();
                        for i in 0..8 {
                            let src_idx = border_uv * stride_uv + border_uv + i * stride_uv + 7;
                            dst_slice_u[i] = u_slice[src_idx];
                            dst_slice_v[i] = v_slice[src_idx];
                        }
                    }
                }
                
                if filter_level != 0 {
                    if pc.filter_type as ::core::ffi::c_uint == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint {
                        #[cfg(target_arch = "aarch64")]
                        {
                            unsafe {
                                let mut lfi: loop_filter_info = loop_filter_info {
                                    mblim: ::core::ptr::null(),
                                    blim: ::core::ptr::null(),
                                    lim: ::core::ptr::null(),
                                    hev_thr: ::core::ptr::null(),
                                };
                                let frame_type = pc.frame_type;
                                let hev_index = lfi_n.hev_thr_lut[frame_type as usize][filter_level as usize] as usize;
                                lfi.mblim = lfi_n.mblim[filter_level as usize].as_ptr();
                                lfi.blim = lfi_n.blim[filter_level as usize].as_ptr();
                                lfi.lim = lfi_n.lim[filter_level as usize].as_ptr();
                                lfi.hev_thr = lfi_n.hev_thr[hev_index].as_ptr();
                                
                                if mb_col > 0 {
                                    vp8_loop_filter_mbv_neon(xd.dst.y_buffer, xd.dst.u_buffer, xd.dst.v_buffer, recon_y_stride, recon_uv_stride, &mut lfi);
                                }
                                if skip_lf == 0 {
                                    vp8_loop_filter_bv_neon(xd.dst.y_buffer, xd.dst.u_buffer, xd.dst.v_buffer, recon_y_stride, recon_uv_stride, &mut lfi);
                                }
                                if mb_row > 0 {
                                    vp8_loop_filter_mbh_neon(xd.dst.y_buffer, xd.dst.u_buffer, xd.dst.v_buffer, recon_y_stride, recon_uv_stride, &mut lfi);
                                }
                                if skip_lf == 0 {
                                    vp8_loop_filter_bh_neon(xd.dst.y_buffer, xd.dst.u_buffer, xd.dst.v_buffer, recon_y_stride, recon_uv_stride, &mut lfi);
                                }
                            }
                        }
                        #[cfg(not(target_arch = "aarch64"))]
                        {
                            let y_border = xd.dst.border as usize;
                            let y_stride = xd.dst.y_stride as usize;
                            let y_active_start = y_border * y_stride + y_border;
                            
                            let uv_border = (xd.dst.border / 2) as usize;
                            let uv_stride = xd.dst.uv_stride as usize;
                            let uv_active_start = uv_border * uv_stride + uv_border;
                            
                            let frame_type = pc.frame_type;
                            let hev_index = lfi_n.hev_thr_lut[frame_type as usize][filter_level as usize] as usize;
                            
                            let blimit_m_slice = &lfi_n.mblim[filter_level as usize];
                            let blimit_b_slice = &lfi_n.blim[filter_level as usize];
                            let limit_slice = &lfi_n.lim[filter_level as usize];
                            let thresh_slice = &lfi_n.hev_thr[hev_index];
                            
                            if mb_col > 0 {
                                {
                                    let y_slice = xd.dst.y_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(y_slice, y_active_start, y_stride, blimit_m_slice, limit_slice, thresh_slice, 2);
                                }
                                if !xd.dst.u_buffer.is_null() {
                                    let u_slice = xd.dst.u_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(u_slice, uv_active_start, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                }
                                if !xd.dst.v_buffer.is_null() {
                                    let v_slice = xd.dst.v_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_vertical_edge_safe(v_slice, uv_active_start, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                }
                            }
                            if skip_lf == 0 {
                                {
                                    let y_slice = xd.dst.y_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(y_slice, y_active_start + 4, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(y_slice, y_active_start + 8, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(y_slice, y_active_start + 12, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                }
                                if !xd.dst.u_buffer.is_null() {
                                    let u_slice = xd.dst.u_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(u_slice, uv_active_start + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                }
                                if !xd.dst.v_buffer.is_null() {
                                    let v_slice = xd.dst.v_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_vertical_edge_safe(v_slice, uv_active_start + 4, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                }
                            }
                            if mb_row > 0 {
                                {
                                    let y_slice = xd.dst.y_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(y_slice, y_active_start, y_stride, blimit_m_slice, limit_slice, thresh_slice, 2);
                                }
                                if !xd.dst.u_buffer.is_null() {
                                    let u_slice = xd.dst.u_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(u_slice, uv_active_start, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                }
                                if !xd.dst.v_buffer.is_null() {
                                    let v_slice = xd.dst.v_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::mbloop_filter_horizontal_edge_safe(v_slice, uv_active_start, uv_stride, blimit_m_slice, limit_slice, thresh_slice, 1);
                                }
                            }
                            if skip_lf == 0 {
                                {
                                    let y_slice = xd.dst.y_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(y_slice, y_active_start + 4 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(y_slice, y_active_start + 8 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(y_slice, y_active_start + 12 * y_stride, y_stride, blimit_b_slice, limit_slice, thresh_slice, 2);
                                }
                                if !xd.dst.u_buffer.is_null() {
                                    let u_slice = xd.dst.u_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(u_slice, uv_active_start + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                }
                                if !xd.dst.v_buffer.is_null() {
                                    let v_slice = xd.dst.v_slice_mut_safe();
                                    crate::vp8::common::loopfilter_filters::loop_filter_horizontal_edge_safe(v_slice, uv_active_start + 4 * uv_stride, uv_stride, blimit_b_slice, limit_slice, thresh_slice, 1);
                                }
                            }
                        }
                    } else {
                        #[cfg(target_arch = "aarch64")]
                        {
                            unsafe {
                                if mb_col > 0 {
                                    vp8_loop_filter_mbvs_neon(xd.dst.y_buffer, recon_y_stride, lfi_n.mblim[filter_level as usize].as_ptr());
                                }
                                if skip_lf == 0 {
                                    vp8_loop_filter_bvs_neon(xd.dst.y_buffer, recon_y_stride, lfi_n.blim[filter_level as usize].as_ptr());
                                }
                                if mb_row > 0 {
                                    vp8_loop_filter_mbhs_neon(xd.dst.y_buffer, recon_y_stride, lfi_n.mblim[filter_level as usize].as_ptr());
                                }
                                if skip_lf == 0 {
                                    vp8_loop_filter_bhs_neon(xd.dst.y_buffer, recon_y_stride, lfi_n.blim[filter_level as usize].as_ptr());
                                }
                            }
                        }
                        #[cfg(not(target_arch = "aarch64"))]
                        {
                            let y_border = xd.dst.border as usize;
                            let y_stride = xd.dst.y_stride as usize;
                            let y_active_start = y_border * y_stride + y_border;
                            
                            if mb_col > 0 {
                                let blimit_val = lfi_n.mblim[filter_level as usize][0];
                                let y_slice = xd.dst.y_slice_mut_safe();
                                crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_vertical_edge_safe(y_slice, y_active_start, y_stride, blimit_val);
                            }
                            if skip_lf == 0 {
                                let blimit_val = lfi_n.blim[filter_level as usize][0];
                                let y_slice = xd.dst.y_slice_mut_safe();
                                crate::vp8::common::loopfilter_filters::vp8_loop_filter_bvs_safe(y_slice, y_active_start, y_stride, blimit_val);
                            }
                            if mb_row > 0 {
                                let blimit_val = lfi_n.mblim[filter_level as usize][0];
                                let y_slice = xd.dst.y_slice_mut_safe();
                                crate::vp8::common::loopfilter_filters::vp8_loop_filter_simple_horizontal_edge_safe(y_slice, y_active_start, y_stride, blimit_val);
                            }
                            if skip_lf == 0 {
                                let blimit_val = lfi_n.blim[filter_level as usize][0];
                                let y_slice = xd.dst.y_slice_mut_safe();
                                crate::vp8::common::loopfilter_filters::vp8_loop_filter_bhs_safe(y_slice, y_active_start, y_stride, blimit_val);
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
        
        safe_decoder.update_bool_decoder(&mut mbc[bc_idx]);
        
        if pc.filter_level != 0 {
            if mb_row != pc.mb_rows - 1 {
                let mut lasty = pc.yv12_fb[lst_fb_idx].y_width + VP8BORDERINPIXELS;
                let mut lastuv = (pc.yv12_fb[lst_fb_idx].y_width >> 1) + (VP8BORDERINPIXELS >> 1);
                
                let dst_ab = mt_sync.mt_yabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                let dst_ab_u = mt_sync.mt_uabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                let dst_ab_v = mt_sync.mt_vabove_row.as_mut().unwrap()[(mb_row + 1) as usize].as_mut().unwrap();
                
                let dst_slice = dst_ab.as_slice_mut();
                let val = dst_slice[lasty as usize - 1];
                dst_slice[lasty as usize..lasty as usize + 4].fill(val);
                
                let dst_slice_u = dst_ab_u.as_slice_mut();
                let val_u = dst_slice_u[lastuv as usize - 1];
                dst_slice_u[lastuv as usize..lastuv as usize + 4].fill(val_u);
                
                let dst_slice_v = dst_ab_v.as_slice_mut();
                let val_v = dst_slice_v[lastuv as usize - 1];
                dst_slice_v[lastuv as usize..lastuv as usize + 4].fill(val_v);
            }
        } else {
            let yv12_fb_new_ref = &mut pc.yv12_fb[new_fb_idx];
            vp8_extend_mb_row(yv12_fb_new_ref, mb_row);
        }
        
        vpx_atomic_store_release(current_mb_col, mb_col + nsync);
        xd.mode_info_idx += 1;
        xd.up_available = 1;
        xd.mode_info_idx += (xd.mode_info_stride as usize) * (decoding_thread_count as usize);
        mb_row = (mb_row as u32).wrapping_add(decoding_thread_count.wrapping_add(1)) as i32;
    }
    
    if last_mb_row + decoding_thread_count as i32 + 1 >= pc.mb_rows {
        if let Some(ref end_sem) = mt_sync.h_event_end_decoding {
            end_sem.signal();
        }
    }
}
unsafe extern "C" fn thread_decoding_proc(
    p_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let (ithread, pbi, mbrd) = unsafe {
        let td = &*(p_data as *const DECODETHREAD_DATA);
        (
            td.ithread,
            &mut *(td.ptr1 as *mut VP8D_COMP),
            &mut *(td.ptr2 as *mut MB_ROW_DEC),
        )
    };
    unsafe {
        while vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0 {
            let start_decoding_sem = &pbi.mt_sync.h_event_start_decoding.as_ref().unwrap()[ithread as usize];
            start_decoding_sem.wait();
            if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) == 0 {
                break;
            }
            let xd = &mut mbrd.mbd;
            if setjmp(&raw mut xd.error_info.jmp as *mut ::core::ffi::c_int) != 0 {
                xd.error_info.setjmp = 0;
                pbi.mt_sync.h_event_end_decoding.as_ref().unwrap().signal();
            } else {
                xd.error_info.setjmp = 1;
                let decoding_thread_count = pbi.decoding_thread_count;
                let (common, mbc, mt_sync) = pbi.split_mt_mut();
                mt_decode_mb_rows(common, mbc, mt_sync, xd, ithread + 1, decoding_thread_count);
                xd.error_info.setjmp = 0;
            }
        }
    }
    THREAD_EXIT_SUCCESS
}
#[unsafe(no_mangle)]
pub fn vp8_decoder_create_threads(pbi: &mut VP8D_COMP) {
    let pbi_raw_ptr = pbi as *mut VP8D_COMP as *mut ::core::ffi::c_void;
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
        
        pbi.mt_sync.h_decoding_thread = Some(vec![core::ptr::null_mut::<core::ffi::c_void>(); count].into_boxed_slice());
        
        let mb_row_di_vec = vec![MB_ROW_DEC::default(); count];
        pbi.mb_row_di = Some(mb_row_di_vec.into_boxed_slice());
        
        pbi.de_thread_data = Some(vec![DECODETHREAD_DATA { ithread: 0, ptr1: core::ptr::null_mut(), ptr2: core::ptr::null_mut() }; count].into_boxed_slice());
        
        unsafe {
            let h_event_start_decoding = pbi.mt_sync.h_event_start_decoding.as_mut().unwrap();
            let h_decoding_thread = pbi.mt_sync.h_decoding_thread.as_mut().unwrap();
            let de_thread_data = pbi.de_thread_data.as_mut().unwrap();
            let mb_row_di = pbi.mb_row_di.as_mut().unwrap();
            
            ithread = 0 as ::core::ffi::c_uint;
            while ithread < pbi.decoding_thread_count {
                vp8_setup_block_dptrs(&mut mb_row_di[ithread as usize].mbd);
                
                de_thread_data[ithread as usize].ithread = ithread as ::core::ffi::c_int;
                de_thread_data[ithread as usize].ptr1 = pbi_raw_ptr;
                de_thread_data[ithread as usize].ptr2 = &mut mb_row_di[ithread as usize] as *mut MB_ROW_DEC as *mut ::core::ffi::c_void;
                
                if crate::thread_shim::vp8_pthread_create(
                    &mut h_decoding_thread[ithread as usize] as *mut pthread_t,
                    ::core::ptr::null::<pthread_attr_t>() as *const ::core::ffi::c_void,
                    Some(
                        thread_decoding_proc
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            )
                                -> *mut ::core::ffi::c_void,
                    ),
                    &mut de_thread_data[ithread as usize] as *mut DECODETHREAD_DATA
                        as *mut ::core::ffi::c_void,
                ) != 0
                {
                    break;
                } else {
                    ithread = ithread.wrapping_add(1);
                }
            }
            pbi.allocated_decoding_thread_count = ithread as ::core::ffi::c_int;
            if pbi.allocated_decoding_thread_count
                != pbi.decoding_thread_count as ::core::ffi::c_int
            {
                if pbi.allocated_decoding_thread_count == 0 as ::core::ffi::c_int {
                    pbi.mt_sync.h_event_end_decoding = None;
                }
                pbi.common.error.trigger(
                    VPX_CODEC_MEM_ERROR,
                    "Failed to create threads",
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
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
}
#[unsafe(no_mangle)]
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
        let mut yabove = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (width + ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_yabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            yabove.push(ab);
        }
        pbi.mt_sync.mt_yabove_row = Some(yabove.into_boxed_slice());
        
        // mt_uabove_row
        let mut uabove = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_uabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            uabove.push(ab);
        }
        pbi.mt_sync.mt_uabove_row = Some(uabove.into_boxed_slice());
        
        // mt_vabove_row
        let mut vabove = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(16, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_vabove_row[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            vabove.push(ab);
        }
        pbi.mt_sync.mt_vabove_row = Some(vabove.into_boxed_slice());
        
        // mt_yleft_col
        let mut yleft = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 16usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_yleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            yleft.push(ab);
        }
        pbi.mt_sync.mt_yleft_col = Some(yleft.into_boxed_slice());
        
        // mt_uleft_col
        let mut uleft = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 8usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_uleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            uleft.push(ab);
        }
        pbi.mt_sync.mt_uleft_col = Some(uleft.into_boxed_slice());
        
        // mt_vleft_col
        let mut vleft = Vec::with_capacity(mb_rows_usize);
        for _ in 0..mb_rows_usize {
            let size = 8usize;
            let mut ab = crate::vpx_mem::vpx_mem::AlignedBox::new(32, size);
            if ab.is_none() {
                pbi.common.error.trigger(VPX_CODEC_MEM_ERROR, "Failed to allocate pbi->mt_vleft_col[i]");
                return;
            }
            if let Some(ref mut b) = ab {
                b.as_slice_mut().fill(0);
            }
            vleft.push(ab);
        }
        pbi.mt_sync.mt_vleft_col = Some(vleft.into_boxed_slice());
    }
}
pub fn vp8_decoder_remove_threads(pbi: &mut VP8D_COMP) {
    if vpx_atomic_load_acquire(&pbi.b_multithreaded_rd) != 0 {
        vpx_atomic_store_release(&pbi.b_multithreaded_rd, 0);
        
        let h_event_start_decoding = pbi.mt_sync.h_event_start_decoding.as_ref().unwrap();
        let h_decoding_thread = pbi.mt_sync.h_decoding_thread.as_ref().unwrap();
        
        let mut i: i32 = 0;
        unsafe {
            while i < pbi.allocated_decoding_thread_count {
                h_event_start_decoding[i as usize].signal();
                crate::thread_shim::vp8_pthread_join(
                    h_decoding_thread[i as usize] as pthread_t,
                    ::core::ptr::null_mut(),
                );
                i += 1;
            }
            pbi.mt_sync.h_decoding_thread = None;
            pbi.mt_sync.h_event_start_decoding = None;
            pbi.mt_sync.h_event_end_decoding = None;
            pbi.mb_row_di = None;
            pbi.de_thread_data = None;
        }
        let mb_rows = pbi.common.mb_rows;
        vp8mt_de_alloc_temp_buffers(pbi, mb_rows);
    }
}
#[unsafe(no_mangle)]
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
        let yabove_ab = pbi.mt_sync.mt_yabove_row.as_mut().unwrap()[0].as_mut().unwrap();
        let len = (y_width + 5) as usize;
        let uabove_ab = pbi.mt_sync.mt_uabove_row.as_mut().unwrap()[0].as_mut().unwrap();
        let len_uv = ((y_width >> 1) + 5) as usize;
        let vabove_ab = pbi.mt_sync.mt_vabove_row.as_mut().unwrap()[0].as_mut().unwrap();
        
        yabove_ab.as_slice_mut()[31..31+len].fill(127);
        uabove_ab.as_slice_mut()[15..15+len_uv].fill(127);
        vabove_ab.as_slice_mut()[15..15+len_uv].fill(127);
        
        j = 1;
        while j < pc_ref.mb_rows {
            let yabove_ab = pbi.mt_sync.mt_yabove_row.as_mut().unwrap()[j as usize].as_mut().unwrap();
            let uabove_ab = pbi.mt_sync.mt_uabove_row.as_mut().unwrap()[j as usize].as_mut().unwrap();
            let vabove_ab = pbi.mt_sync.mt_vabove_row.as_mut().unwrap()[j as usize].as_mut().unwrap();
            yabove_ab.as_slice_mut()[31] = 129;
            uabove_ab.as_slice_mut()[15] = 129;
            vabove_ab.as_slice_mut()[15] = 129;
            j += 1;
        }
        
        j = 0;
        while j < pc_ref.mb_rows {
            let yleft_ab = pbi.mt_sync.mt_yleft_col.as_mut().unwrap()[j as usize].as_mut().unwrap();
            let uleft_ab = pbi.mt_sync.mt_uleft_col.as_mut().unwrap()[j as usize].as_mut().unwrap();
            let vleft_ab = pbi.mt_sync.mt_vleft_col.as_mut().unwrap()[j as usize].as_mut().unwrap();
            yleft_ab.as_slice_mut()[..16].fill(129);
            uleft_ab.as_slice_mut()[..8].fill(129);
            vleft_ab.as_slice_mut()[..8].fill(129);
            j += 1;
        }
        filter_branch = true;
    }
    
    if filter_branch {
        vp8_loop_filter_frame_init(pc_ref, &pbi.mb, filter_level);
    } else {
        let yv12_fb_new_ref = &mut pc_ref.yv12_fb[new_fb_idx];
        vp8_setup_intra_recon_top_line(yv12_fb_new_ref);
    }
    
    let mb_row_di = pbi.mb_row_di.as_mut().unwrap();
    
    setup_decoding_thread_data(
        &pbi.common,
        pbi.mt_sync.mt_current_mb_col.as_deref(),
        &pbi.mb,
        mb_row_di,
    );

    let h_event_start_decoding = pbi.mt_sync.h_event_start_decoding.as_ref().unwrap();

    unsafe {
        i = 0;
        while i < pbi.decoding_thread_count {
            h_event_start_decoding[i as usize].signal();
            i = i.wrapping_add(1);
        }
        
        let xd = &mut pbi.mb;
        let setjmp_res = setjmp(&raw mut xd.error_info.jmp as *mut ::core::ffi::c_int);
        if setjmp_res != 0 {
            xd.error_info.setjmp = 0;
            xd.corrupted = 1;
            i = 0;
            while i < pbi.decoding_thread_count {
                pbi.mt_sync.h_event_end_decoding.as_ref().unwrap().wait();
                i = i.wrapping_add(1);
            }
            return -1;
        }
        
        xd.error_info.setjmp = 1;
        
        let decoding_thread_count = pbi.decoding_thread_count;
        let common = &mut pbi.common;
        let mbc = &mut pbi.mbc;
        let mt_sync = &mut pbi.mt_sync;
        
        mt_decode_mb_rows(common, mbc, mt_sync, xd, 0, decoding_thread_count);
        
        xd.error_info.setjmp = 0;
        
        i = 0;
        while i < pbi.decoding_thread_count.wrapping_add(1) {
            pbi.mt_sync.h_event_end_decoding.as_ref().unwrap().wait();
            i = i.wrapping_add(1);
        }
    }
    
    0}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
