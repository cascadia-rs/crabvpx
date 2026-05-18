use crate::vp8::decoder::dboolhuff::SafeBoolDecoder;
use crate::vp8::decoder::detokenize::{vp8_decode_mb_tokens, vp8_reset_mb_tokens_context};
use crate::vp8::common::vp8_loopfilter::vp8_loop_filter_frame_init;
use crate::vp8::common::quant_common::{vp8_ac_yquant, vp8_dc_quant, vp8_dc2quant, vp8_ac2quant, vp8_dc_uv_quant, vp8_ac_uv_quant};

unsafe extern "C" {
    fn vp8dx_decode_bool(br: *mut BOOL_DECODER, probability: ::core::ffi::c_int) -> ::core::ffi::c_int;

    fn vp8_bilinear_predict16x16_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict4x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict8x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_bilinear_predict8x8_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_dc_only_idct_add_neon(
        input_dc: ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_neon(
        input: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dest: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_uv_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst_u: *mut ::core::ffi::c_uchar,
        dst_v: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequant_idct_add_y_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequantize_b_neon(_: *mut blockd, DQC: *mut ::core::ffi::c_short);
    fn vp8_short_inv_walsh4x4_neon(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn vp8_short_inv_walsh4x4_1_c(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn vp8_sixtap_predict16x16_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict4x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict8x4_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_sixtap_predict8x8_neon(
        src_ptr: *mut ::core::ffi::c_uchar,
        src_pixels_per_line: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_pitch: ::core::ffi::c_int,
    );
    fn vp8_yv12_extend_frame_borders_c(ybf: *mut yv12_buffer_config);
    static vp8_norm: [::core::ffi::c_uchar; 256];
    fn vp8dx_start_decode(
        br: *mut BOOL_DECODER,
        source: *const ::core::ffi::c_uchar,
        source_sz: ::core::ffi::c_uint,
        decrypt_cb: vpx_decrypt_cb,
        decrypt_state: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn vp8dx_bool_decoder_fill(br: *mut BOOL_DECODER);
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn vp8_loop_filter_row_normal(
        cm: *mut VP8Common,
        mode_info_context: *mut modeinfo,
        mb_row: ::core::ffi::c_int,
        post_ystride: ::core::ffi::c_int,
        post_uvstride: ::core::ffi::c_int,
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_row_simple(
        cm: *mut VP8Common,
        mode_info_context: *mut modeinfo,
        mb_row: ::core::ffi::c_int,
        post_ystride: ::core::ffi::c_int,
        y_ptr: *mut ::core::ffi::c_uchar,
    );
    static vp8_default_mv_context: [MV_CONTEXT; 2];
    static vp8_coef_update_probs: [[[[vp8_prob; 11]; 3]; 8]; 4];
    static vp8_mb_feature_data_bits: [::core::ffi::c_int; 2];
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
    fn vp8_intra4x4_predict(
        above: *mut ::core::ffi::c_uchar,
        yleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        b_mode: B_PREDICTION_MODE,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
        top_left: ::core::ffi::c_uchar,
    );
    fn vp8_setup_version(cm: *mut VP8_COMMON);
    fn vp8_init_mbmode_probs(x: *mut VP8_COMMON);
    fn vp8_setup_intra_recon_top_line(ybf: *mut YV12_BUFFER_CONFIG);
    fn vp8_decode_mode_mvs(_: *mut VP8D_COMP);
    fn vp8_extend_mb_row(
        ybf: *mut YV12_BUFFER_CONFIG,
        YPtr: *mut ::core::ffi::c_uchar,
        UPtr: *mut ::core::ffi::c_uchar,
        VPtr: *mut ::core::ffi::c_uchar,
    );
    fn vp8mt_decode_mb_rows(pbi: *mut VP8D_COMP, xd: *mut MACROBLOCKD) -> ::core::ffi::c_int;
    fn vp8_decoder_remove_threads(pbi: *mut VP8D_COMP);
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
unsafe extern "C" fn vpx_atomic_load_acquire(
    mut atomic: *const vpx_atomic_int,
) -> ::core::ffi::c_int { unsafe {
    return (*(&raw const (*atomic).value as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::Acquire);
}}
#[inline]
unsafe extern "C" fn intra_prediction_down_copy(
    mut xd: *mut MACROBLOCKD,
    mut above_right_src: *mut ::core::ffi::c_uchar,
) { unsafe {
    let mut dst_stride: ::core::ffi::c_int = (*xd).dst.y_stride;
    let mut above_right_dst: *mut ::core::ffi::c_uchar = (*xd)
        .dst
        .y_buffer
        .offset(-(dst_stride as isize))
        .offset(16 as ::core::ffi::c_int as isize);
    let mut src_ptr: *mut ::core::ffi::c_uint = above_right_src as *mut ::core::ffi::c_uint;
    let mut dst_ptr0: *mut ::core::ffi::c_uint = above_right_dst
        .offset((4 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr1: *mut ::core::ffi::c_uint = above_right_dst
        .offset((8 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr2: *mut ::core::ffi::c_uint = above_right_dst
        .offset((12 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    *dst_ptr0 = *src_ptr;
    *dst_ptr1 = *src_ptr;
    *dst_ptr2 = *src_ptr;
}}
#[inline]
unsafe extern "C" fn setup_intra_recon_left(
    mut y_buffer: *mut ::core::ffi::c_uchar,
    mut u_buffer: *mut ::core::ffi::c_uchar,
    mut v_buffer: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *y_buffer.offset((y_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *u_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *v_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
}}
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
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut QIndex: ::core::ffi::c_int = 0;
    let mbmi = &mut (*xd.mode_info_context).mbmi;
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
}}
fn decode_macroblock(
    pbi: &mut VP8D_COMP,
    xd: &mut MACROBLOCKD,
    mb_idx: ::core::ffi::c_uint,
) { unsafe {
    let mut mode: MB_PREDICTION_MODE = DC_PRED;
    let mut i: ::core::ffi::c_int = 0;
    if (*xd.mode_info_context).mbmi.mb_skip_coeff != 0 {
        vp8_reset_mb_tokens_context(xd);
    } else if vp8dx_bool_error(&pbi.mbc[xd.current_bc_idx]) == 0 {
        let mut eobtotal: ::core::ffi::c_int = 0;
        eobtotal = vp8_decode_mb_tokens(pbi, xd);
        (*xd.mode_info_context).mbmi.mb_skip_coeff =
            (eobtotal == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
    }
    mode = (*xd.mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
    if xd.segmentation_enabled != 0 {
        vp8_mb_init_dequantizer(&pbi.common, xd);
    }
    if (*xd.mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
        == INTRA_FRAME as ::core::ffi::c_int
    {
        crate::vp8::common::reconintra::vp8_build_intra_predictors_mbuv_s(
            xd,
            xd.recon_above[1 as ::core::ffi::c_int as usize],
            xd.recon_above[2 as ::core::ffi::c_int as usize],
            xd.recon_left[1 as ::core::ffi::c_int as usize],
            xd.recon_left[2 as ::core::ffi::c_int as usize],
            xd.recon_left_stride[1 as ::core::ffi::c_int as usize],
            xd.dst.u_buffer as *mut ::core::ffi::c_uchar,
            xd.dst.v_buffer as *mut ::core::ffi::c_uchar,
            xd.dst.uv_stride,
        );
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            crate::vp8::common::reconintra::vp8_build_intra_predictors_mby_s(
                xd,
                xd.recon_above[0 as ::core::ffi::c_int as usize],
                xd.recon_left[0 as ::core::ffi::c_int as usize],
                xd.recon_left_stride[0 as ::core::ffi::c_int as usize],
                xd.dst.y_buffer as *mut ::core::ffi::c_uchar,
                xd.dst.y_stride,
            );
        } else {
            let mut DQC: *mut ::core::ffi::c_short =
                &raw mut xd.dequant_y1 as *mut ::core::ffi::c_short;
            let mut dst_stride: ::core::ffi::c_int = xd.dst.y_stride;
            if (*xd.mode_info_context).mbmi.mb_skip_coeff != 0 {
                memset(
                    &raw mut xd.eobs as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    25 as size_t,
                );
            }
            intra_prediction_down_copy(
                xd as *mut _,
                xd.recon_above[0 as ::core::ffi::c_int as usize]
                    .offset(16 as ::core::ffi::c_int as isize),
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut b: *mut BLOCKD =
                    (&raw mut xd.block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                let mut dst: *mut ::core::ffi::c_uchar =
                    xd.dst.y_buffer.offset((*b).offset as isize);
                let mut b_mode: B_PREDICTION_MODE =
                    (*xd.mode_info_context).bmi[i as usize].as_mode;
                let mut Above: *mut ::core::ffi::c_uchar = dst.offset(-(dst_stride as isize));
                let mut yleft: *mut ::core::ffi::c_uchar =
                    dst.offset(-(1 as ::core::ffi::c_int as isize));
                let mut left_stride: ::core::ffi::c_int = dst_stride;
                let mut top_left: ::core::ffi::c_uchar =
                    *Above.offset(-(1 as ::core::ffi::c_int) as isize);
                vp8_intra4x4_predict(Above, yleft, left_stride, b_mode, dst, dst_stride, top_left);
                if xd.eobs[i as usize] != 0 {
                    if xd.eobs[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        vp8_dequant_idct_add_neon((*b).qcoeff, DQC, dst, dst_stride);
                    } else {
                        vp8_dc_only_idct_add_neon(
                            (*(*b).qcoeff.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                * *DQC.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)
                                as ::core::ffi::c_short,
                            dst,
                            dst_stride,
                            dst,
                            dst_stride,
                        );
                        memset(
                            (*b).qcoeff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_short>() as size_t,
                            ),
                        );
                    }
                }
                i += 1;
            }
        }
    } else {
        crate::vp8::common::reconinter::vp8_build_inter_predictors_mb(xd);
    }
    if (*xd.mode_info_context).mbmi.mb_skip_coeff == 0 {
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            let mut DQC_0: *mut ::core::ffi::c_short =
                &raw mut xd.dequant_y1 as *mut ::core::ffi::c_short;
            if mode as ::core::ffi::c_uint != SPLITMV as ::core::ffi::c_int as ::core::ffi::c_uint {
                let mut b_0: *mut BLOCKD = (&raw mut xd.block as *mut BLOCKD)
                    .offset(24 as ::core::ffi::c_int as isize)
                    as *mut BLOCKD;
                if xd.eobs[24 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                {
                    vp8_dequantize_b_neon(
                        b_0 as *mut blockd,
                        &raw mut xd.dequant_y2 as *mut ::core::ffi::c_short,
                    );
                    vp8_short_inv_walsh4x4_neon(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut xd.qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (16 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                } else {
                    *(*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize) = (*(*b_0)
                        .qcoeff
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        * xd.dequant_y2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    vp8_short_inv_walsh4x4_1_c(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut xd.qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                DQC_0 = &raw mut xd.dequant_y1_dc as *mut ::core::ffi::c_short;
            }
            vp8_dequant_idct_add_y_block_neon(
                &raw mut xd.qcoeff as *mut ::core::ffi::c_short,
                DQC_0,
                xd.dst.y_buffer as *mut ::core::ffi::c_uchar,
                xd.dst.y_stride,
                &raw mut xd.eobs as *mut ::core::ffi::c_char,
            );
        }
        vp8_dequant_idct_add_uv_block_neon(
            (&raw mut xd.qcoeff as *mut ::core::ffi::c_short)
                .offset((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            &raw mut xd.dequant_uv as *mut ::core::ffi::c_short,
            xd.dst.u_buffer as *mut ::core::ffi::c_uchar,
            xd.dst.v_buffer as *mut ::core::ffi::c_uchar,
            xd.dst.uv_stride,
            (&raw mut xd.eobs as *mut ::core::ffi::c_char)
                .offset(16 as ::core::ffi::c_int as isize),
        );
    }
}}
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

unsafe extern "C" fn yv12_extend_frame_top_c(mut ybf: *mut YV12_BUFFER_CONFIG) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    src_ptr1 = (*ybf).y_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = (*ybf).u_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = (*ybf).v_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    dest_ptr1 =
        src_ptr1.offset(-(Border.wrapping_mul(plane_stride as ::core::ffi::c_uint) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr1 as *mut ::core::ffi::c_void,
            src_ptr1 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        i += 1;
    }
}}
unsafe extern "C" fn yv12_extend_frame_bottom_c(mut ybf: *mut YV12_BUFFER_CONFIG) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut src_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    let mut plane_height: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    plane_height = (*ybf).y_height;
    src_ptr1 = (*ybf).y_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    plane_height = (*ybf).uv_height;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = (*ybf).u_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = (*ybf).v_buffer.offset(-(Border as isize)) as *mut ::core::ffi::c_uchar;
    src_ptr2 = src_ptr1
        .offset((plane_height * plane_stride) as isize)
        .offset(-(plane_stride as isize));
    dest_ptr2 = src_ptr2.offset(plane_stride as isize);
    i = 0 as ::core::ffi::c_int;
    while i < Border as ::core::ffi::c_int {
        memcpy(
            dest_ptr2 as *mut ::core::ffi::c_void,
            src_ptr2 as *const ::core::ffi::c_void,
            plane_stride as size_t,
        );
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
}}
unsafe extern "C" fn yv12_extend_frame_left_right_c(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut y_src: *mut ::core::ffi::c_uchar,
    mut u_src: *mut ::core::ffi::c_uchar,
    mut v_src: *mut ::core::ffi::c_uchar,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut src_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut src_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dest_ptr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Border: ::core::ffi::c_uint = 0;
    let mut plane_stride: ::core::ffi::c_int = 0;
    let mut plane_height: ::core::ffi::c_int = 0;
    let mut plane_width: ::core::ffi::c_int = 0;
    Border = (*ybf).border as ::core::ffi::c_uint;
    plane_stride = (*ybf).y_stride;
    plane_height = 16 as ::core::ffi::c_int;
    plane_width = (*ybf).y_width;
    src_ptr1 = y_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    plane_stride = (*ybf).uv_stride;
    plane_height = 8 as ::core::ffi::c_int;
    plane_width = (*ybf).uv_width;
    Border = Border.wrapping_div(2 as ::core::ffi::c_uint);
    src_ptr1 = u_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
    src_ptr1 = v_src;
    src_ptr2 = src_ptr1
        .offset(plane_width as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    dest_ptr1 = src_ptr1.offset(-(Border as isize));
    dest_ptr2 = src_ptr2.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < plane_height {
        memset(
            dest_ptr1 as *mut ::core::ffi::c_void,
            *src_ptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        memset(
            dest_ptr2 as *mut ::core::ffi::c_void,
            *src_ptr2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            Border as size_t,
        );
        src_ptr1 = src_ptr1.offset(plane_stride as isize);
        src_ptr2 = src_ptr2.offset(plane_stride as isize);
        dest_ptr1 = dest_ptr1.offset(plane_stride as isize);
        dest_ptr2 = dest_ptr2.offset(plane_stride as isize);
        i += 1;
    }
}}
unsafe extern "C" fn decode_mb_rows(mut pbi: *mut VP8D_COMP) { unsafe {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    let mut lf_mic: *mut MODE_INFO = (*xd).mode_info_context;
    let mut ibc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_part: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << (*pc).multi_token_partition as ::core::ffi::c_uint;
    let mut recon_yoffset: ::core::ffi::c_int = 0;
    let mut recon_uvoffset: ::core::ffi::c_int = 0;
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    let mut mb_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    let mut recon_y_stride: ::core::ffi::c_int = (*yv12_fb_new).y_stride;
    let mut recon_uv_stride: ::core::ffi::c_int = (*yv12_fb_new).uv_stride;
    let mut ref_buffer: [[*mut ::core::ffi::c_uchar; 3]; 4] =
        [[::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3]; 4];
    let mut dst_buffer: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut lf_dst: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut eb_dst: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut i: ::core::ffi::c_int = 0;
    let mut ref_fb_corrupted: [::core::ffi::c_int; 4] = [0; 4];
    ref_fb_corrupted[INTRA_FRAME as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < MAX_REF_FRAMES as ::core::ffi::c_int {
        let mut this_fb: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[i as usize];
        ref_buffer[i as usize][0 as ::core::ffi::c_int as usize] =
            (*this_fb).y_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][1 as ::core::ffi::c_int as usize] =
            (*this_fb).u_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][2 as ::core::ffi::c_int as usize] =
            (*this_fb).v_buffer as *mut ::core::ffi::c_uchar;
        ref_fb_corrupted[i as usize] = (*this_fb).corrupted;
        i += 1;
    }
    dst_buffer[0 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).y_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[0 as ::core::ffi::c_int as usize] = dst_buffer[0 as ::core::ffi::c_int as usize];
    eb_dst[0 as ::core::ffi::c_int as usize] = lf_dst[0 as ::core::ffi::c_int as usize];
    dst_buffer[1 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).u_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[1 as ::core::ffi::c_int as usize] = dst_buffer[1 as ::core::ffi::c_int as usize];
    eb_dst[1 as ::core::ffi::c_int as usize] = lf_dst[1 as ::core::ffi::c_int as usize];
    dst_buffer[2 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).v_buffer as *mut ::core::ffi::c_uchar;
    lf_dst[2 as ::core::ffi::c_int as usize] = dst_buffer[2 as ::core::ffi::c_int as usize];
    eb_dst[2 as ::core::ffi::c_int as usize] = lf_dst[2 as ::core::ffi::c_int as usize];
    (*xd).up_available = 0 as ::core::ffi::c_int;
    if (*pc).filter_level != 0 {
        let filter_level = (*pc).filter_level;
        vp8_loop_filter_frame_init(&mut *pc, &*xd, filter_level);
    }
    vp8_setup_intra_recon_top_line(yv12_fb_new);
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < (*pc).mb_rows {
        if num_part > 1 as ::core::ffi::c_int {
            (*xd).current_bc_idx = ibc as usize;
            ibc += 1;
            if ibc == num_part {
                ibc = 0 as ::core::ffi::c_int;
            }
        }
        recon_yoffset = mb_row * recon_y_stride * 16 as ::core::ffi::c_int;
        recon_uvoffset = mb_row * recon_uv_stride * 8 as ::core::ffi::c_int;
        (*xd).above_context = (*pc).above_context;
        memset(
            (*xd).left_context as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t,
        );
        (*xd).left_available = 0 as ::core::ffi::c_int;
        (*xd).mb_to_top_edge = -((mb_row * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
        (*xd).mb_to_bottom_edge = (((*pc).mb_rows - 1 as ::core::ffi::c_int - mb_row)
            * 16 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int;
        (*xd).recon_above[0 as ::core::ffi::c_int as usize] =
            dst_buffer[0 as ::core::ffi::c_int as usize].offset(recon_yoffset as isize);
        (*xd).recon_above[1 as ::core::ffi::c_int as usize] =
            dst_buffer[1 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
        (*xd).recon_above[2 as ::core::ffi::c_int as usize] =
            dst_buffer[2 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
        (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [0 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [1 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [2 as ::core::ffi::c_int as usize]
            .offset(-(1 as ::core::ffi::c_int as isize));
        (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [0 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.y_stride as isize));
        (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [1 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.uv_stride as isize));
        (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
            [2 as ::core::ffi::c_int as usize]
            .offset(-((*xd).dst.uv_stride as isize));
        (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize] = (*xd).dst.y_stride;
        (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize] = (*xd).dst.uv_stride;
        setup_intra_recon_left(
            (*xd).recon_left[0 as ::core::ffi::c_int as usize],
            (*xd).recon_left[1 as ::core::ffi::c_int as usize],
            (*xd).recon_left[2 as ::core::ffi::c_int as usize],
            (*xd).dst.y_stride,
            (*xd).dst.uv_stride,
        );
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*pc).mb_cols {
            (*xd).mb_to_left_edge =
                -((mb_col * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
            (*xd).mb_to_right_edge = (((*pc).mb_cols - 1 as ::core::ffi::c_int - mb_col)
                * 16 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int;
            (*xd).dst.y_buffer = dst_buffer[0 as ::core::ffi::c_int as usize]
                .offset(recon_yoffset as isize) as *mut uint8_t;
            (*xd).dst.u_buffer = dst_buffer[1 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            (*xd).dst.v_buffer = dst_buffer[2 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            if (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
                >= LAST_FRAME as ::core::ffi::c_int
            {
                let ref_0: MV_REFERENCE_FRAME =
                    (*(*xd).mode_info_context).mbmi.ref_frame as MV_REFERENCE_FRAME;
                (*xd).pre.y_buffer = ref_buffer[ref_0 as usize][0 as ::core::ffi::c_int as usize]
                    .offset(recon_yoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.u_buffer = ref_buffer[ref_0 as usize][1 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.v_buffer = ref_buffer[ref_0 as usize][2 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
            } else {
                (*xd).pre.y_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.u_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.v_buffer = ::core::ptr::null_mut::<uint8_t>();
            }
            (*xd).corrupted |= ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
            decode_macroblock(&mut *pbi, &mut *xd, mb_idx as ::core::ffi::c_uint);
            mb_idx += 1;
            (*xd).left_available = 1 as ::core::ffi::c_int;
            (*xd).corrupted |= vp8dx_bool_error(&(*pbi).mbc[(*xd).current_bc_idx]);
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [0 as ::core::ffi::c_int as usize]
                .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [1 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [2 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [0 as ::core::ffi::c_int as usize]
                .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [1 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_left
                [2 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            recon_yoffset += 16 as ::core::ffi::c_int;
            recon_uvoffset += 8 as ::core::ffi::c_int;
            (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
            (*xd).above_context = (*xd).above_context.offset(1);
            mb_col += 1;
        }
        vp8_extend_mb_row(
            yv12_fb_new,
            (*xd).dst.y_buffer.offset(16 as ::core::ffi::c_int as isize),
            (*xd).dst.u_buffer.offset(8 as ::core::ffi::c_int as isize),
            (*xd).dst.v_buffer.offset(8 as ::core::ffi::c_int as isize),
        );
        (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
        (*xd).up_available = 1 as ::core::ffi::c_int;
        if (*pc).filter_level != 0 {
            if mb_row > 0 as ::core::ffi::c_int {
                if (*pc).filter_type as ::core::ffi::c_uint
                    == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    vp8_loop_filter_row_normal(
                        pc as *mut VP8Common,
                        lf_mic as *mut modeinfo,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        recon_uv_stride,
                        lf_dst[0 as ::core::ffi::c_int as usize],
                        lf_dst[1 as ::core::ffi::c_int as usize],
                        lf_dst[2 as ::core::ffi::c_int as usize],
                    );
                } else {
                    vp8_loop_filter_row_simple(
                        pc as *mut VP8Common,
                        lf_mic as *mut modeinfo,
                        mb_row - 1 as ::core::ffi::c_int,
                        recon_y_stride,
                        lf_dst[0 as ::core::ffi::c_int as usize],
                    );
                }
                if mb_row > 1 as ::core::ffi::c_int {
                    yv12_extend_frame_left_right_c(
                        yv12_fb_new,
                        eb_dst[0 as ::core::ffi::c_int as usize],
                        eb_dst[1 as ::core::ffi::c_int as usize],
                        eb_dst[2 as ::core::ffi::c_int as usize],
                    );
                    eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst
                        [0 as ::core::ffi::c_int as usize]
                        .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
                    eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst
                        [1 as ::core::ffi::c_int as usize]
                        .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                    eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst
                        [2 as ::core::ffi::c_int as usize]
                        .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                }
                lf_dst[0 as ::core::ffi::c_int as usize] = lf_dst[0 as ::core::ffi::c_int as usize]
                    .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
                lf_dst[1 as ::core::ffi::c_int as usize] = lf_dst[1 as ::core::ffi::c_int as usize]
                    .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                lf_dst[2 as ::core::ffi::c_int as usize] = lf_dst[2 as ::core::ffi::c_int as usize]
                    .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
                lf_mic = lf_mic.offset((*pc).mb_cols as isize);
                lf_mic = lf_mic.offset(1);
            }
        } else if mb_row > 0 as ::core::ffi::c_int {
            yv12_extend_frame_left_right_c(
                yv12_fb_new,
                eb_dst[0 as ::core::ffi::c_int as usize],
                eb_dst[1 as ::core::ffi::c_int as usize],
                eb_dst[2 as ::core::ffi::c_int as usize],
            );
            eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst[0 as ::core::ffi::c_int as usize]
                .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
            eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst[1 as ::core::ffi::c_int as usize]
                .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
            eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst[2 as ::core::ffi::c_int as usize]
                .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
        }
        mb_row += 1;
    }
    if (*pc).filter_level != 0 {
        if (*pc).filter_type as ::core::ffi::c_uint
            == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            vp8_loop_filter_row_normal(
                pc as *mut VP8Common,
                lf_mic as *mut modeinfo,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                recon_uv_stride,
                lf_dst[0 as ::core::ffi::c_int as usize],
                lf_dst[1 as ::core::ffi::c_int as usize],
                lf_dst[2 as ::core::ffi::c_int as usize],
            );
        } else {
            vp8_loop_filter_row_simple(
                pc as *mut VP8Common,
                lf_mic as *mut modeinfo,
                mb_row - 1 as ::core::ffi::c_int,
                recon_y_stride,
                lf_dst[0 as ::core::ffi::c_int as usize],
            );
        }
        yv12_extend_frame_left_right_c(
            yv12_fb_new,
            eb_dst[0 as ::core::ffi::c_int as usize],
            eb_dst[1 as ::core::ffi::c_int as usize],
            eb_dst[2 as ::core::ffi::c_int as usize],
        );
        eb_dst[0 as ::core::ffi::c_int as usize] = eb_dst[0 as ::core::ffi::c_int as usize]
            .offset((recon_y_stride * 16 as ::core::ffi::c_int) as isize);
        eb_dst[1 as ::core::ffi::c_int as usize] = eb_dst[1 as ::core::ffi::c_int as usize]
            .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
        eb_dst[2 as ::core::ffi::c_int as usize] = eb_dst[2 as ::core::ffi::c_int as usize]
            .offset((recon_uv_stride * 8 as ::core::ffi::c_int) as isize);
    }
    yv12_extend_frame_left_right_c(
        yv12_fb_new,
        eb_dst[0 as ::core::ffi::c_int as usize],
        eb_dst[1 as ::core::ffi::c_int as usize],
        eb_dst[2 as ::core::ffi::c_int as usize],
    );
    yv12_extend_frame_top_c(yv12_fb_new);
    yv12_extend_frame_bottom_c(yv12_fb_new);
}}
fn read_partition_size(
    pbi: *mut VP8D_COMP,
    cx_size: &[u8],
) -> ::core::ffi::c_uint { unsafe {
    let mut temp: [::core::ffi::c_uchar; 3] = [0; 3];
    let mut data_slice = cx_size;
    if (*pbi).decrypt_cb.is_some() {
        (*pbi).decrypt_cb.expect("non-null function pointer")(
            (*pbi).decrypt_state,
            cx_size.as_ptr(),
            &raw mut temp as *mut ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int,
        );
        data_slice = &temp;
    }
    return (data_slice[0] as ::core::ffi::c_int
        + ((data_slice[1] as ::core::ffi::c_int) << 8)
        + ((data_slice[2] as ::core::ffi::c_int) << 16)) as ::core::ffi::c_uint;
}}
fn read_available_partition_size(
    pbi: *mut VP8D_COMP,
    token_part_sizes: &[u8],
    fragment: &[u8],
    i: ::core::ffi::c_int,
    num_part: ::core::ffi::c_int,
) -> ::core::ffi::c_uint { unsafe {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut partition_size: ::core::ffi::c_uint = 0;
    let bytes_left = fragment.len();
    if i < num_part - 1 {
        let size_offset = (i * 3) as usize;
        if size_offset + 3 <= token_part_sizes.len() {
            partition_size = read_partition_size(pbi, &token_part_sizes[size_offset .. size_offset + 3]);
        } else if (*pbi).ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated partition size data\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        partition_size = bytes_left as ::core::ffi::c_uint;
    }
    if partition_size as usize > bytes_left {
        if (*pbi).ec_active != 0 {
            partition_size = bytes_left as ::core::ffi::c_uint;
        } else {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet or corrupt partition %d length\0" as *const u8
                    as *const ::core::ffi::c_char,
                i + 1,
            );
        }
    }
    return partition_size;
}}
fn setup_token_decoder(
    mut pbi: *mut VP8D_COMP,
    token_part_sizes: &[u8],
    safe_decoder: &mut SafeBoolDecoder,
) { unsafe {
    let mut partition_idx: ::core::ffi::c_uint = 0;
    let mut fragment_idx: ::core::ffi::c_uint = 0;
    let mut num_token_partitions: ::core::ffi::c_uint = 0;
    let mut multi_token_partition: TOKEN_PARTITION =
        safe_decoder.read_literal(2) as TOKEN_PARTITION;
    if safe_decoder.count <= VP8_BD_VALUE_SIZE || safe_decoder.count >= VP8_LOTS_OF_BITS {
        (*pbi).common.multi_token_partition = multi_token_partition;
    }
    num_token_partitions = ((1 as ::core::ffi::c_int)
        << (*pbi).common.multi_token_partition as ::core::ffi::c_uint)
        as ::core::ffi::c_uint;
    fragment_idx = 0 as ::core::ffi::c_uint;
    while fragment_idx < (*pbi).fragments.count {
        let mut fragment_size: ::core::ffi::c_uint = (*pbi).fragments.sizes[fragment_idx as usize];
        if fragment_idx == 0 as ::core::ffi::c_uint {
            let mut ext_first_part_size: ptrdiff_t = token_part_sizes
                .as_ptr()
                .offset_from((*pbi).fragments.ptrs[0 as ::core::ffi::c_int as usize])
                as ptrdiff_t
                + (3 as ::core::ffi::c_uint)
                    .wrapping_mul(num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint))
                    as ptrdiff_t;
            if fragment_size < ext_first_part_size as ::core::ffi::c_uint {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupted fragment size %d\0" as *const u8 as *const ::core::ffi::c_char,
                    fragment_size,
                );
            }
            fragment_size = fragment_size.wrapping_sub(ext_first_part_size as ::core::ffi::c_uint);
            if fragment_size > 0 as ::core::ffi::c_uint {
                (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] =
                    ext_first_part_size as ::core::ffi::c_uint;
                fragment_idx = fragment_idx.wrapping_add(1);
                (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                    [0 as ::core::ffi::c_int as usize]
                    .offset((*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] as isize);
            }
        }
        while fragment_size > 0 as ::core::ffi::c_uint {
            let fragment_slice = core::slice::from_raw_parts((*pbi).fragments.ptrs[fragment_idx as usize], fragment_size as usize);
            let partition_size: ptrdiff_t = read_available_partition_size(
                pbi,
                token_part_sizes,
                fragment_slice,
                fragment_idx.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                num_token_partitions as ::core::ffi::c_int,
            ) as ptrdiff_t;
            (*pbi).fragments.sizes[fragment_idx as usize] = partition_size as ::core::ffi::c_uint;
            if fragment_size < partition_size as ::core::ffi::c_uint {
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupted fragment size %d\0" as *const u8 as *const ::core::ffi::c_char,
                    fragment_size,
                );
            }
            fragment_size = fragment_size.wrapping_sub(partition_size as ::core::ffi::c_uint);
            if fragment_size > 0 as ::core::ffi::c_uint {
                fragment_idx = fragment_idx.wrapping_add(1);
                (*pbi).fragments.ptrs[fragment_idx as usize] = (*pbi).fragments.ptrs
                    [fragment_idx.wrapping_sub(1 as ::core::ffi::c_uint) as usize]
                    .offset(partition_size as isize);
            }
        }
        fragment_idx = fragment_idx.wrapping_add(1);
    }
    (*pbi).fragments.count = num_token_partitions.wrapping_add(1 as ::core::ffi::c_uint);
    partition_idx = 1 as ::core::ffi::c_uint;
    while partition_idx < (*pbi).fragments.count {
        if vp8dx_start_decode(
            &raw mut (*pbi).mbc[(partition_idx - 1) as usize] as *mut BOOL_DECODER,
            (*pbi).fragments.ptrs[partition_idx as usize],
            (*pbi).fragments.sizes[partition_idx as usize],
            (*pbi).decrypt_cb,
            (*pbi).decrypt_state,
        ) != 0
        {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate bool decoder %d\0" as *const u8 as *const ::core::ffi::c_char,
                partition_idx,
            );
        }
        partition_idx = partition_idx.wrapping_add(1);
    }
    if (*pbi).decoding_thread_count > num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint) {
        (*pbi).decoding_thread_count = num_token_partitions.wrapping_sub(1 as ::core::ffi::c_uint);
    }
    if (*pbi).decoding_thread_count as ::core::ffi::c_int
        > (*pbi).common.mb_rows - 1 as ::core::ffi::c_int
    {
        (*pbi).decoding_thread_count =
            ((*pbi).common.mb_rows - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
}}
unsafe extern "C" fn init_frame(mut pbi: *mut VP8D_COMP) { unsafe {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    if (*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memcpy(
            &raw mut (*pc).fc.mvc as *mut MV_CONTEXT as *mut ::core::ffi::c_void,
            &raw const vp8_default_mv_context as *const MV_CONTEXT as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[MV_CONTEXT; 2]>() as size_t,
        );
        vp8_init_mbmode_probs(pc);
        crate::vp8::common::entropy::vp8_default_coef_probs(&mut (*pc).fc.coef_probs);
        memset(
            &raw mut (*xd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[::core::ffi::c_schar; 4]; 2]>() as size_t,
        );
        (*xd).mb_segment_abs_delta = SEGMENT_DELTADATA as ::core::ffi::c_uchar;
        memset(
            &raw mut (*xd).ref_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        memset(
            &raw mut (*xd).mode_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        (*pc).refresh_golden_frame = 1 as ::core::ffi::c_int;
        (*pc).refresh_alt_ref_frame = 1 as ::core::ffi::c_int;
        (*pc).copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        (*pc).copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        (*pc).ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
        (*pc).ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] =
            0 as ::core::ffi::c_int;
    } else {
        if (*pc).use_bilinear_mc_filter == 0 {
            (*xd).subpixel_predict = Some(
                vp8_sixtap_predict4x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x4 = Some(
                vp8_sixtap_predict8x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x8 = Some(
                vp8_sixtap_predict8x8_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict16x16 = Some(
                vp8_sixtap_predict16x16_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
        } else {
            (*xd).subpixel_predict = Some(
                vp8_bilinear_predict4x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x4 = Some(
                vp8_bilinear_predict8x4_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict8x8 = Some(
                vp8_bilinear_predict8x8_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
            (*xd).subpixel_predict16x16 = Some(
                vp8_bilinear_predict16x16_neon
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as vp8_subpix_fn_t;
        }
        if (*pbi).decoded_key_frame != 0 && (*pbi).ec_enabled != 0 && (*pbi).ec_active == 0 {
            (*pbi).ec_active = 1 as ::core::ffi::c_int;
        }
    }
    (*xd).left_context = &raw mut (*pc).left_context;
    (*xd).mode_info_context = (*pc).mi;
    (*xd).frame_type = (*pc).frame_type;
    (*(*xd).mode_info_context).mbmi.mode = DC_PRED as ::core::ffi::c_int as uint8_t;
    (*xd).mode_info_stride = (*pc).mode_info_stride;
    (*xd).corrupted = 0 as ::core::ffi::c_int;
    (*xd).fullpixel_mask = !(0 as ::core::ffi::c_int);
    if (*pc).full_pixel != 0 {
        (*xd).fullpixel_mask = !(7 as ::core::ffi::c_int);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_decode_frame(mut pbi: *mut VP8D_COMP) -> ::core::ffi::c_int { unsafe {
    let bc = &mut (*pbi).mbc[8];
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).mb;
    let mut data: *const ::core::ffi::c_uchar =
        (*pbi).fragments.ptrs[0 as ::core::ffi::c_int as usize];
    let data_sz: ::core::ffi::c_uint = (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize];
    let mut data_end: *const ::core::ffi::c_uchar = data.offset(data_sz as isize);
    let mut first_partition_length_in_bytes: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mb_feature_data_bits: *const ::core::ffi::c_int =
        &raw const vp8_mb_feature_data_bits as *const ::core::ffi::c_int;
    let mut corrupt_tokens: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prev_independent_partitions: ::core::ffi::c_int = (*pbi).independent_partitions;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    (*xd).corrupted = 0 as ::core::ffi::c_int;
    (*yv12_fb_new).corrupted = 0 as ::core::ffi::c_int;
    if (data_end.offset_from(data) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
        if (*pbi).ec_active == 0 {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Truncated packet\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*pc).frame_type = INTER_FRAME;
        (*pc).version = 0 as ::core::ffi::c_int;
        (*pc).show_frame = 1 as ::core::ffi::c_int;
        first_partition_length_in_bytes = 0 as ::core::ffi::c_int;
    } else {
        let mut clear_buffer: [::core::ffi::c_uchar; 10] = [0; 10];
        let mut clear: *const ::core::ffi::c_uchar = data;
        if (*pbi).decrypt_cb.is_some() {
            let mut n: ::core::ffi::c_int =
                (if (::core::mem::size_of::<[::core::ffi::c_uchar; 10]>() as usize)
                    < data_sz as usize
                {
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 10]>() as usize
                } else {
                    data_sz as usize
                }) as ::core::ffi::c_int;
            (*pbi).decrypt_cb.expect("non-null function pointer")(
                (*pbi).decrypt_state,
                data,
                &raw mut clear_buffer as *mut ::core::ffi::c_uchar,
                n,
            );
            clear = &raw mut clear_buffer as *mut ::core::ffi::c_uchar;
        }
        (*pc).frame_type = (*clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int) as FRAME_TYPE;
        (*pc).version = *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int;
        (*pc).show_frame = *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int;
        first_partition_length_in_bytes = (*clear.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            | (*clear.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
            | (*clear.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int)
            >> 5 as ::core::ffi::c_int;
        if (*pbi).ec_active == 0 && first_partition_length_in_bytes == 0 as ::core::ffi::c_int {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_CORRUPT_FRAME,
                b"Corrupt partition 0 length\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        data = data.offset(3 as ::core::ffi::c_int as isize);
        clear = clear.offset(3 as ::core::ffi::c_int as isize);
        vp8_setup_version(pc);
        if (*pc).frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if data_end.offset_from(data) as ::core::ffi::c_long >= 7 as ::core::ffi::c_long {
                if *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0x9d as ::core::ffi::c_int
                    || *clear.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 0x1 as ::core::ffi::c_int
                    || *clear.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 0x2a as ::core::ffi::c_int
                {
                    vpx_internal_error(
                        &raw mut (*pc).error,
                        VPX_CODEC_UNSUP_BITSTREAM,
                        b"Invalid frame sync code\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*pc).Width = (*clear.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    | (*clear.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    & 0x3fff as ::core::ffi::c_int;
                (*pc).horiz_scale = *clear.offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int;
                (*pc).Height = (*clear.offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    | (*clear.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    & 0x3fff as ::core::ffi::c_int;
                (*pc).vert_scale = *clear.offset(6 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int;
                data = data.offset(7 as ::core::ffi::c_int as isize);
            } else if (*pbi).ec_active == 0 {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Truncated key frame header\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                data = data_end;
            }
        } else {
            (*xd).pre = *yv12_fb_new;
            (*xd).dst = *yv12_fb_new;
        }
    }
    if (*pbi).decoded_key_frame == 0
        && (*pc).frame_type as ::core::ffi::c_uint
            != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*pbi).ec_active == 0
        && (data_end.offset_from(data) as ::core::ffi::c_long)
            < first_partition_length_in_bytes as ::core::ffi::c_long
    {
        vpx_internal_error(
            &raw mut (*pc).error,
            VPX_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt partition 0 length\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    init_frame(pbi);
    if vp8dx_start_decode(
        &raw mut *bc,
        data,
        data_end.offset_from(data) as ::core::ffi::c_long as ::core::ffi::c_uint,
        (*pbi).decrypt_cb,
        (*pbi).decrypt_state,
    ) != 0
    {
        vpx_internal_error(
            &raw mut (*pc).error,
            VPX_CODEC_MEM_ERROR,
            b"Failed to allocate bool decoder 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let len = bc.user_buffer_end.offset_from(bc.user_buffer) as usize;
    let slice = if len == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(bc.user_buffer, len)
    };
    let mut safe_decoder = SafeBoolDecoder {
        buffer: slice,
        offset: 0,
        value: bc.value,
        count: bc.count,
        range: bc.range,
        decrypt_cb: bc.decrypt_cb,
        decrypt_state: bc.decrypt_state,
    };

    if (*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_decoder.read_bool(vp8_prob_half as i32);
        (*pc).clamp_type =
            safe_decoder.read_bool(vp8_prob_half as i32) as CLAMP_TYPE;
    }
    (*xd).segmentation_enabled =
        safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
    if (*xd).segmentation_enabled != 0 {
        (*xd).update_mb_segmentation_map =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        (*xd).update_mb_segmentation_data =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        if (*xd).update_mb_segmentation_data != 0 {
            (*xd).mb_segment_abs_delta =
                safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
            memset(
                &raw mut (*xd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[[::core::ffi::c_schar; 4]; 2]>() as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while i < MB_LVL_MAX as ::core::ffi::c_int {
                j = 0 as ::core::ffi::c_int;
                while j < MAX_MB_SEGMENTS {
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        (*xd).segment_feature_data[i as usize][j as usize] = safe_decoder.read_literal(*mb_feature_data_bits.offset(i as isize)) as ::core::ffi::c_schar;
                        if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                            (*xd).segment_feature_data[i as usize][j as usize] = -((*xd)
                                .segment_feature_data[i as usize][j as usize]
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_schar;
                        }
                    } else {
                        (*xd).segment_feature_data[i as usize][j as usize] =
                            0 as ::core::ffi::c_schar;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        if (*xd).update_mb_segmentation_map != 0 {
            memset(
                &raw mut (*xd).mb_segment_tree_probs as *mut vp8_prob as *mut ::core::ffi::c_void,
                255 as ::core::ffi::c_int,
                ::core::mem::size_of::<[vp8_prob; 3]>() as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while i < MB_FEATURE_TREE_PROBS {
                if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                    (*xd).mb_segment_tree_probs[i as usize] =
                        safe_decoder.read_literal(8) as vp8_prob;
                }
                i += 1;
            }
        }
    } else {
        (*xd).update_mb_segmentation_map = 0 as ::core::ffi::c_uchar;
        (*xd).update_mb_segmentation_data = 0 as ::core::ffi::c_uchar;
    }
    (*pc).filter_type =
        safe_decoder.read_bool(vp8_prob_half as i32) as LOOPFILTERTYPE;
    (*pc).filter_level = safe_decoder.read_literal(6);
    (*pc).sharpness_level = safe_decoder.read_literal(3);
    (*xd).mode_ref_lf_delta_update = 0 as ::core::ffi::c_uchar;
    (*xd).mode_ref_lf_delta_enabled =
        safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
    if (*xd).mode_ref_lf_delta_enabled != 0 {
        (*xd).mode_ref_lf_delta_update =
            safe_decoder.read_bool(vp8_prob_half as i32) as ::core::ffi::c_uchar;
        if (*xd).mode_ref_lf_delta_update != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < MAX_REF_LF_DELTAS {
                if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                    (*xd).ref_lf_deltas[i as usize] =
                        safe_decoder.read_literal(6) as ::core::ffi::c_schar;
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        (*xd).ref_lf_deltas[i as usize] = ((*xd).ref_lf_deltas[i as usize]
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
                    (*xd).mode_lf_deltas[i as usize] =
                        safe_decoder.read_literal(6) as ::core::ffi::c_schar;
                    if safe_decoder.read_bool(vp8_prob_half as i32) != 0 {
                        (*xd).mode_lf_deltas[i as usize] = ((*xd).mode_lf_deltas[i as usize]
                            as ::core::ffi::c_int
                            * -(1 as ::core::ffi::c_int))
                            as ::core::ffi::c_schar;
                    }
                }
                i += 1;
            }
        }
    }
    let token_part_sizes_len = data_end.offset_from(data.offset(first_partition_length_in_bytes as isize)) as usize;
    let token_part_sizes_slice = core::slice::from_raw_parts(data.offset(first_partition_length_in_bytes as isize), token_part_sizes_len);
    setup_token_decoder(
        pbi,
        token_part_sizes_slice,
        &mut safe_decoder,
    );
    (*xd).current_bc_idx = 0;

    let mut Q: ::core::ffi::c_int = 0;

    let mut q_update: ::core::ffi::c_int = 0;
    Q = safe_decoder.read_literal(7);
    (*pc).base_qindex = Q;
    q_update = 0 as ::core::ffi::c_int;
    (*pc).y1dc_delta_q = get_delta_q(&mut safe_decoder, (*pc).y1dc_delta_q, &mut q_update);
    (*pc).y2dc_delta_q = get_delta_q(&mut safe_decoder, (*pc).y2dc_delta_q, &mut q_update);
    (*pc).y2ac_delta_q = get_delta_q(&mut safe_decoder, (*pc).y2ac_delta_q, &mut q_update);
    (*pc).uvdc_delta_q = get_delta_q(&mut safe_decoder, (*pc).uvdc_delta_q, &mut q_update);
    (*pc).uvac_delta_q = get_delta_q(&mut safe_decoder, (*pc).uvac_delta_q, &mut q_update);
    if q_update != 0 {
        vp8cx_init_de_quantizer(&mut *pbi);
    }
    vp8_mb_init_dequantizer(&(*pbi).common, &mut (*pbi).mb);
    if (*pc).frame_type as ::core::ffi::c_uint
        != KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*pc).refresh_golden_frame =
            safe_decoder.read_bool(vp8_prob_half as i32);
        (*pc).refresh_alt_ref_frame =
            safe_decoder.read_bool(vp8_prob_half as i32);
        (*pc).copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        if (*pc).refresh_golden_frame == 0 {
            (*pc).copy_buffer_to_gf =
                safe_decoder.read_literal(2);
        }
        (*pc).copy_buffer_to_arf = 0 as ::core::ffi::c_int;
        if (*pc).refresh_alt_ref_frame == 0 {
            (*pc).copy_buffer_to_arf =
                safe_decoder.read_literal(2);
        }
        (*pc).ref_frame_sign_bias[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
            safe_decoder.read_bool(vp8_prob_half as i32);
        (*pc).ref_frame_sign_bias[ALTREF_FRAME as ::core::ffi::c_int as usize] =
            safe_decoder.read_bool(vp8_prob_half as i32);
    }
    (*pc).refresh_entropy_probs =
        safe_decoder.read_bool(vp8_prob_half as i32);
    if (*pc).refresh_entropy_probs == 0 as ::core::ffi::c_int {
        (*pc).lfc = (*pc).fc;
    }
    (*pc).refresh_last_frame = ((*pc).frame_type as ::core::ffi::c_uint
        == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
        || safe_decoder.read_bool(vp8_prob_half as i32) != 0)
        as ::core::ffi::c_int;
    (*pbi).independent_partitions = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BLOCK_TYPES {
        j = 0 as ::core::ffi::c_int;
        while j < COEF_BANDS {
            k = 0 as ::core::ffi::c_int;
            while k < PREV_COEF_CONTEXTS {
                l = 0 as ::core::ffi::c_int;
                while l < ENTROPY_NODES {
                    let p: *mut vp8_prob =
                        (&raw mut *(&raw mut *(&raw mut *(&raw mut (*pc).fc.coef_probs
                            as *mut [[[vp8_prob; 11]; 3]; 8])
                            .offset(i as isize)
                            as *mut [[vp8_prob; 11]; 3])
                            .offset(j as isize)
                            as *mut [vp8_prob; 11])
                            .offset(k as isize) as *mut vp8_prob)
                            .offset(l as isize);
                    if safe_decoder.read_bool(vp8_coef_update_probs[i as usize][j as usize][k as usize][l as usize] as i32) != 0 {
                        *p = safe_decoder.read_literal(8) as vp8_prob;
                    }
                    if k > 0 as ::core::ffi::c_int
                        && *p as ::core::ffi::c_int
                            != (*pc).fc.coef_probs[i as usize][j as usize]
                                [(k - 1 as ::core::ffi::c_int) as usize]
                                [l as usize] as ::core::ffi::c_int
                    {
                        (*pbi).independent_partitions = 0 as ::core::ffi::c_int;
                    }
                    l += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    bc.user_buffer = bc.user_buffer.add(safe_decoder.offset);
    bc.value = safe_decoder.value;
    bc.count = safe_decoder.count;
    bc.range = safe_decoder.range;

    memset(
        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_short; 400]>() as size_t,
    );
    vp8_decode_mode_mvs(pbi);
    memset(
        (*pc).above_context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t)
            .wrapping_mul((*pc).mb_cols as size_t),
    );
    (*pbi).frame_corrupt_residual = 0 as ::core::ffi::c_int;
    if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0
        && (*pc).multi_token_partition as ::core::ffi::c_uint
            != ONE_PARTITION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut thread: ::core::ffi::c_uint = 0;
        if vp8mt_decode_mb_rows(pbi, xd) != 0 {
            vp8_decoder_remove_threads(pbi);
            (*pbi).restart_threads = 1 as ::core::ffi::c_int;
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_CORRUPT_FRAME,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        vp8_yv12_extend_frame_borders_c(yv12_fb_new as *mut yv12_buffer_config);
        thread = 0 as ::core::ffi::c_uint;
        while thread < (*pbi).decoding_thread_count {
            corrupt_tokens |= (*(*pbi).mb_row_di.offset(thread as isize)).mbd.corrupted;
            thread = thread.wrapping_add(1);
        }
    } else {
        decode_mb_rows(pbi);
        corrupt_tokens |= (*xd).corrupted;
    }
    (*yv12_fb_new).corrupted = vp8dx_bool_error(bc);
    (*yv12_fb_new).corrupted |= corrupt_tokens;
    if (*pbi).decoded_key_frame == 0 {
        if (*pc).frame_type as ::core::ffi::c_uint
            == KEY_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*yv12_fb_new).corrupted == 0
        {
            (*pbi).decoded_key_frame = 1 as ::core::ffi::c_int;
        } else {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_CORRUPT_FRAME,
                b"A stream must start with a complete key frame\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if (*pc).refresh_entropy_probs == 0 as ::core::ffi::c_int {
        (*pc).fc = (*pc).lfc;
        (*pbi).independent_partitions = prev_independent_partitions;
    }
    return 0 as ::core::ffi::c_int;
}}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
