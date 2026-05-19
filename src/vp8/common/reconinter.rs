use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
    pub bmi: BModeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: u32,
    pub mv: IntMv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
pub const B_MODE_COUNT: u32 = 14;
pub const NEW4X4: u32 = 13;
pub const ZERO4X4: u32 = 12;
pub const ABOVE4X4: u32 = 11;
pub const LEFT4X4: u32 = 10;
pub const B_HU_PRED: u32 = 9;
pub const B_HD_PRED: u32 = 8;
pub const B_VL_PRED: u32 = 7;
pub const B_VR_PRED: u32 = 6;
pub const B_RD_PRED: u32 = 5;
pub const B_LD_PRED: u32 = 4;
pub const B_HE_PRED: u32 = 3;
pub const B_VE_PRED: u32 = 2;
pub const B_TM_PRED: u32 = 1;
pub const B_DC_PRED: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Macroblockd {
    pub predictor: [u8; 384],
    pub qcoeff: [i16; 400],
    pub dqcoeff: [i16; 400],
    pub eobs: [i8; 25],
    pub dequant_y1: [i16; 16],
    pub dequant_y1_dc: [i16; 16],
    pub dequant_y2: [i16; 16],
    pub dequant_uv: [i16; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: i32,
    pub pre: Yv12BufferConfig,
    pub dst: Yv12BufferConfig,
    pub mode_info_context: *mut ModeInfo,
    pub mode_info_stride: i32,
    pub frame_type: u32,
    pub up_available: bool,
    pub left_available: bool,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: *mut EntropyContextPlanes,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [u8; 3],
    pub segment_feature_data: [[i8; 4]; 2],
    pub mode_ref_lf_delta_enabled: u8,
    pub mode_ref_lf_delta_update: u8,
    pub last_ref_lf_deltas: [i8; 4],
    pub ref_lf_deltas: [i8; 4],
    pub last_mode_lf_deltas: [i8; 4],
    pub mode_lf_deltas: [i8; 4],
    pub mb_to_left_edge: i32,
    pub mb_to_right_edge: i32,
    pub mb_to_top_edge: i32,
    pub mb_to_bottom_edge: i32,
    pub subpixel_predict: Vp8SubpixFnT,
    pub subpixel_predict8x4: Vp8SubpixFnT,
    pub subpixel_predict8x8: Vp8SubpixFnT,
    pub subpixel_predict16x16: Vp8SubpixFnT,
    pub current_bc: *mut c_void,
    pub corrupted: i32,
    pub error_info: VpxInternalErrorInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
pub const VPX_CODEC_LIST_END: u32 = 9;
pub const VPX_CODEC_INVALID_PARAM: u32 = 8;
pub const VPX_CODEC_CORRUPT_FRAME: u32 = 7;
pub const VPX_CODEC_UNSUP_FEATURE: u32 = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: u32 = 5;
pub const VPX_CODEC_INCAPABLE: u32 = 4;
pub const VPX_CODEC_ABI_MISMATCH: u32 = 3;
pub const VPX_CODEC_MEM_ERROR: u32 = 2;
pub const VPX_CODEC_ERROR: u32 = 1;
pub const VPX_CODEC_OK: u32 = 0;
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [i8; 4],
    pub u: [i8; 2],
    pub v: [i8; 2],
    pub y2: i8,
}
pub const INTER_FRAME: u32 = 1;
pub const KEY_FRAME: u32 = 0;
pub type ModeInfo = Modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbModeInfo {
    pub mode: u8,
    pub uv_mode: u8,
    pub ref_frame: u8,
    pub is_4x4: u8,
    pub mv: IntMv,
    pub partitioning: u8,
    pub mb_skip_coeff: u8,
    pub need_to_clamp_mvs: u8,
    pub segment_id: u8,
}

pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub type BLOCKD = Blockd;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const ZEROMV: C2RustUnnamed = 7;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub type MACROBLOCKD = Macroblockd;
pub const CHAR_BIT: i32 = 8 as i32;
#[unsafe(no_mangle)]
pub unsafe fn vp8_copy_mem16x16_c(
    mut src: *mut u8,
    mut src_stride: i32,
    mut dst: *mut u8,
    mut dst_stride: i32,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < 16 as i32 {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                16 as usize,
            );
            src = src.offset(src_stride as isize);
            dst = dst.offset(dst_stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_copy_mem8x8_c(
    mut src: *mut u8,
    mut src_stride: i32,
    mut dst: *mut u8,
    mut dst_stride: i32,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < 8 as i32 {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                8 as usize,
            );
            src = src.offset(src_stride as isize);
            dst = dst.offset(dst_stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_copy_mem8x4_c(
    mut src: *mut u8,
    mut src_stride: i32,
    mut dst: *mut u8,
    mut dst_stride: i32,
) {
    unsafe {
        let mut r: i32 = 0;
        r = 0 as i32;
        while r < 4 as i32 {
            core::ptr::copy_nonoverlapping(
                src as *const c_void as *const u8,
                dst as *mut c_void as *mut u8,
                8 as usize,
            );
            src = src.offset(src_stride as isize);
            dst = dst.offset(dst_stride as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter_predictors_b(
    mut d: *mut BLOCKD,
    mut pitch: i32,
    mut base_pre: *mut u8,
    mut pre_stride: i32,
    mut sppf: Vp8SubpixFnT,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut pred_ptr: *mut u8 = (*d).predictor;
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        ptr = base_pre
            .offset((*d).offset as isize)
            .offset((((*d).bmi.mv.as_mv.row as i32 >> 3 as i32) * pre_stride) as isize)
            .offset(((*d).bmi.mv.as_mv.col as i32 >> 3 as i32) as isize);
        if (*d).bmi.mv.as_mv.row as i32 & 7 as i32 != 0
            || (*d).bmi.mv.as_mv.col as i32 & 7 as i32 != 0
        {
            sppf.expect("non-null function pointer")(
                ptr,
                pre_stride,
                (*d).bmi.mv.as_mv.col as i32 & 7 as i32,
                (*d).bmi.mv.as_mv.row as i32 & 7 as i32,
                pred_ptr,
                pitch,
            );
        } else {
            r = 0 as i32;
            while r < 4 as i32 {
                *pred_ptr.offset(0 as isize) = *ptr.offset(0 as isize);
                *pred_ptr.offset(1 as isize) = *ptr.offset(1 as isize);
                *pred_ptr.offset(2 as isize) = *ptr.offset(2 as isize);
                *pred_ptr.offset(3 as isize) = *ptr.offset(3 as isize);
                pred_ptr = pred_ptr.offset(pitch as isize);
                ptr = ptr.offset(pre_stride as isize);
                r += 1;
            }
        };
    }
}
unsafe fn build_inter_predictors4b(
    mut x: *mut MACROBLOCKD,
    mut d: *mut BLOCKD,
    mut dst: *mut u8,
    mut dst_stride: i32,
    mut base_pre: *mut u8,
    mut pre_stride: i32,
) {
    unsafe {
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        ptr = base_pre
            .offset((*d).offset as isize)
            .offset((((*d).bmi.mv.as_mv.row as i32 >> 3 as i32) * pre_stride) as isize)
            .offset(((*d).bmi.mv.as_mv.col as i32 >> 3 as i32) as isize);
        if (*d).bmi.mv.as_mv.row as i32 & 7 as i32 != 0
            || (*d).bmi.mv.as_mv.col as i32 & 7 as i32 != 0
        {
            (*x).subpixel_predict8x8.expect("non-null function pointer")(
                ptr,
                pre_stride,
                (*d).bmi.mv.as_mv.col as i32 & 7 as i32,
                (*d).bmi.mv.as_mv.row as i32 & 7 as i32,
                dst,
                dst_stride,
            );
        } else {
            vp8_copy_mem8x8_c(ptr, pre_stride, dst, dst_stride);
        };
    }
}
unsafe fn build_inter_predictors2b(
    mut x: *mut MACROBLOCKD,
    mut d: *mut BLOCKD,
    mut dst: *mut u8,
    mut dst_stride: i32,
    mut base_pre: *mut u8,
    mut pre_stride: i32,
) {
    unsafe {
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        ptr = base_pre
            .offset((*d).offset as isize)
            .offset((((*d).bmi.mv.as_mv.row as i32 >> 3 as i32) * pre_stride) as isize)
            .offset(((*d).bmi.mv.as_mv.col as i32 >> 3 as i32) as isize);
        if (*d).bmi.mv.as_mv.row as i32 & 7 as i32 != 0
            || (*d).bmi.mv.as_mv.col as i32 & 7 as i32 != 0
        {
            (*x).subpixel_predict8x4.expect("non-null function pointer")(
                ptr,
                pre_stride,
                (*d).bmi.mv.as_mv.col as i32 & 7 as i32,
                (*d).bmi.mv.as_mv.row as i32 & 7 as i32,
                dst,
                dst_stride,
            );
        } else {
            vp8_copy_mem8x4_c(ptr, pre_stride, dst, dst_stride);
        };
    }
}
unsafe fn build_inter_predictors_b(
    mut d: *mut BLOCKD,
    mut dst: *mut u8,
    mut dst_stride: i32,
    mut base_pre: *mut u8,
    mut pre_stride: i32,
    mut sppf: Vp8SubpixFnT,
) {
    unsafe {
        let mut r: i32 = 0;
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        ptr = base_pre
            .offset((*d).offset as isize)
            .offset((((*d).bmi.mv.as_mv.row as i32 >> 3 as i32) * pre_stride) as isize)
            .offset(((*d).bmi.mv.as_mv.col as i32 >> 3 as i32) as isize);
        if (*d).bmi.mv.as_mv.row as i32 & 7 as i32 != 0
            || (*d).bmi.mv.as_mv.col as i32 & 7 as i32 != 0
        {
            sppf.expect("non-null function pointer")(
                ptr,
                pre_stride,
                (*d).bmi.mv.as_mv.col as i32 & 7 as i32,
                (*d).bmi.mv.as_mv.row as i32 & 7 as i32,
                dst,
                dst_stride,
            );
        } else {
            r = 0 as i32;
            while r < 4 as i32 {
                *dst.offset(0 as isize) = *ptr.offset(0 as isize);
                *dst.offset(1 as isize) = *ptr.offset(1 as isize);
                *dst.offset(2 as isize) = *ptr.offset(2 as isize);
                *dst.offset(3 as isize) = *ptr.offset(3 as isize);
                dst = dst.offset(dst_stride as isize);
                ptr = ptr.offset(pre_stride as isize);
                r += 1;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter16x16_predictors_mbuv(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut uptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut vptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut upred_ptr: *mut u8 =
            (&raw mut (*x).predictor as *mut u8).offset(256 as isize) as *mut u8;
        let mut vpred_ptr: *mut u8 =
            (&raw mut (*x).predictor as *mut u8).offset(320 as isize) as *mut u8;
        let mut mv_row: i32 = (*(*x).mode_info_context).mbmi.mv.as_mv.row as i32;
        let mut mv_col: i32 = (*(*x).mode_info_context).mbmi.mv.as_mv.col as i32;
        let mut offset: i32 = 0;
        let mut pre_stride: i32 = (*x).pre.uv_stride;
        mv_row += 1 as i32
            | mv_row
                >> (::core::mem::size_of::<i32>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1_usize);
        mv_col += 1 as i32
            | mv_col
                >> (::core::mem::size_of::<i32>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1_usize);
        mv_row /= 2 as i32;
        mv_col /= 2 as i32;
        mv_row &= (*x).fullpixel_mask;
        mv_col &= (*x).fullpixel_mask;
        offset = (mv_row >> 3 as i32) * pre_stride + (mv_col >> 3 as i32);
        uptr = (*x).pre.u_buffer.offset(offset as isize) as *mut u8;
        vptr = (*x).pre.v_buffer.offset(offset as isize) as *mut u8;
        if (mv_row | mv_col) & 7 as i32 != 0 {
            (*x).subpixel_predict8x8.expect("non-null function pointer")(
                uptr,
                pre_stride,
                mv_col & 7 as i32,
                mv_row & 7 as i32,
                upred_ptr,
                8 as i32,
            );
            (*x).subpixel_predict8x8.expect("non-null function pointer")(
                vptr,
                pre_stride,
                mv_col & 7 as i32,
                mv_row & 7 as i32,
                vpred_ptr,
                8 as i32,
            );
        } else {
            vp8_copy_mem8x8_c(uptr, pre_stride, upred_ptr, 8 as i32);
            vp8_copy_mem8x8_c(vptr, pre_stride, vpred_ptr, 8 as i32);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter4x4_predictors_mbuv(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut pre_stride: i32 = (*x).pre.uv_stride;
        let mut base_pre: *mut u8 = ::core::ptr::null_mut::<u8>();
        i = 0 as i32;
        while i < 2 as i32 {
            j = 0 as i32;
            while j < 2 as i32 {
                let mut yoffset: i32 = i * 8 as i32 + j * 2 as i32;
                let mut uoffset: i32 = 16 as i32 + i * 2 as i32 + j;
                let mut voffset: i32 = 20 as i32 + i * 2 as i32 + j;
                let mut temp: i32 = 0;
                temp = (*x).block[yoffset as usize].bmi.mv.as_mv.row as i32
                    + (*x).block[(yoffset + 1 as i32) as usize].bmi.mv.as_mv.row as i32
                    + (*x).block[(yoffset + 4 as i32) as usize].bmi.mv.as_mv.row as i32
                    + (*x).block[(yoffset + 5 as i32) as usize].bmi.mv.as_mv.row as i32;
                temp += 4 as i32
                    + (temp
                        >> (::core::mem::size_of::<i32>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize))
                        * 8 as i32;
                (*x).block[uoffset as usize].bmi.mv.as_mv.row =
                    ((temp / 8 as i32) & (*x).fullpixel_mask) as i16;
                temp = (*x).block[yoffset as usize].bmi.mv.as_mv.col as i32
                    + (*x).block[(yoffset + 1 as i32) as usize].bmi.mv.as_mv.col as i32
                    + (*x).block[(yoffset + 4 as i32) as usize].bmi.mv.as_mv.col as i32
                    + (*x).block[(yoffset + 5 as i32) as usize].bmi.mv.as_mv.col as i32;
                temp += 4 as i32
                    + (temp
                        >> (::core::mem::size_of::<i32>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize))
                        * 8 as i32;
                (*x).block[uoffset as usize].bmi.mv.as_mv.col =
                    ((temp / 8 as i32) & (*x).fullpixel_mask) as i16;
                (*x).block[voffset as usize].bmi.mv.as_int =
                    (*x).block[uoffset as usize].bmi.mv.as_int;
                j += 1;
            }
            i += 1;
        }
        base_pre = (*x).pre.u_buffer as *mut u8;
        i = 16 as i32;
        while i < 20 as i32 {
            let mut d0: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
            let mut d1: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset((i + 1 as i32) as isize) as *mut BLOCKD;
            if (*d0).bmi.mv.as_int == (*d1).bmi.mv.as_int {
                build_inter_predictors2b(x, d0, (*d0).predictor, 8 as i32, base_pre, pre_stride);
            } else {
                vp8_build_inter_predictors_b(
                    d0,
                    8 as i32,
                    base_pre,
                    pre_stride,
                    (*x).subpixel_predict,
                );
                vp8_build_inter_predictors_b(
                    d1,
                    8 as i32,
                    base_pre,
                    pre_stride,
                    (*x).subpixel_predict,
                );
            }
            i += 2 as i32;
        }
        base_pre = (*x).pre.v_buffer as *mut u8;
        i = 20 as i32;
        while i < 24 as i32 {
            let mut d0_0: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
            let mut d1_0: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset((i + 1 as i32) as isize) as *mut BLOCKD;
            if (*d0_0).bmi.mv.as_int == (*d1_0).bmi.mv.as_int {
                build_inter_predictors2b(
                    x,
                    d0_0,
                    (*d0_0).predictor,
                    8 as i32,
                    base_pre,
                    pre_stride,
                );
            } else {
                vp8_build_inter_predictors_b(
                    d0_0,
                    8 as i32,
                    base_pre,
                    pre_stride,
                    (*x).subpixel_predict,
                );
                vp8_build_inter_predictors_b(
                    d1_0,
                    8 as i32,
                    base_pre,
                    pre_stride,
                    (*x).subpixel_predict,
                );
            }
            i += 2 as i32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter16x16_predictors_mby(
    mut x: *mut MACROBLOCKD,
    mut dst_y: *mut u8,
    mut dst_ystride: i32,
) {
    unsafe {
        let mut ptr_base: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut mv_row: i32 = (*(*x).mode_info_context).mbmi.mv.as_mv.row as i32;
        let mut mv_col: i32 = (*(*x).mode_info_context).mbmi.mv.as_mv.col as i32;
        let mut pre_stride: i32 = (*x).pre.y_stride;
        ptr_base = (*x).pre.y_buffer as *mut u8;
        ptr = ptr_base
            .offset(((mv_row >> 3 as i32) * pre_stride) as isize)
            .offset((mv_col >> 3 as i32) as isize);
        if (mv_row | mv_col) & 7 as i32 != 0 {
            (*x).subpixel_predict16x16
                .expect("non-null function pointer")(
                ptr,
                pre_stride,
                mv_col & 7 as i32,
                mv_row & 7 as i32,
                dst_y,
                dst_ystride,
            );
        } else {
            vp8_copy_mem16x16_c(ptr, pre_stride, dst_y, dst_ystride);
        };
    }
}
unsafe fn clamp_mv_to_umv_border(mut mv: *mut MV, mut xd: *const MACROBLOCKD) {
    unsafe {
        if ((*mv).col as i32) < (*xd).mb_to_left_edge - ((19 as i32) << 3 as i32) {
            (*mv).col = ((*xd).mb_to_left_edge - ((16 as i32) << 3 as i32)) as i16;
        } else if (*mv).col as i32 > (*xd).mb_to_right_edge + ((18 as i32) << 3 as i32) {
            (*mv).col = ((*xd).mb_to_right_edge + ((16 as i32) << 3 as i32)) as i16;
        }
        if ((*mv).row as i32) < (*xd).mb_to_top_edge - ((19 as i32) << 3 as i32) {
            (*mv).row = ((*xd).mb_to_top_edge - ((16 as i32) << 3 as i32)) as i16;
        } else if (*mv).row as i32 > (*xd).mb_to_bottom_edge + ((18 as i32) << 3 as i32) {
            (*mv).row = ((*xd).mb_to_bottom_edge + ((16 as i32) << 3 as i32)) as i16;
        }
    }
}
unsafe fn clamp_uvmv_to_umv_border(mut mv: *mut MV, mut xd: *const MACROBLOCKD) {
    unsafe {
        (*mv).col =
            (if (2 as i32 * (*mv).col as i32) < (*xd).mb_to_left_edge - ((19 as i32) << 3 as i32) {
                ((*xd).mb_to_left_edge - ((16 as i32) << 3 as i32)) >> 1 as i32
            } else {
                (*mv).col as i32
            }) as i16;
        (*mv).col =
            (if 2 as i32 * (*mv).col as i32 > (*xd).mb_to_right_edge + ((18 as i32) << 3 as i32) {
                ((*xd).mb_to_right_edge + ((16 as i32) << 3 as i32)) >> 1 as i32
            } else {
                (*mv).col as i32
            }) as i16;
        (*mv).row =
            (if (2 as i32 * (*mv).row as i32) < (*xd).mb_to_top_edge - ((19 as i32) << 3 as i32) {
                ((*xd).mb_to_top_edge - ((16 as i32) << 3 as i32)) >> 1 as i32
            } else {
                (*mv).row as i32
            }) as i16;
        (*mv).row =
            (if 2 as i32 * (*mv).row as i32 > (*xd).mb_to_bottom_edge + ((18 as i32) << 3 as i32) {
                ((*xd).mb_to_bottom_edge + ((16 as i32) << 3 as i32)) >> 1 as i32
            } else {
                (*mv).row as i32
            }) as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter16x16_predictors_mb(
    mut x: *mut MACROBLOCKD,
    mut dst_y: *mut u8,
    mut dst_u: *mut u8,
    mut dst_v: *mut u8,
    mut dst_ystride: i32,
    mut dst_uvstride: i32,
) {
    unsafe {
        let mut offset: i32 = 0;
        let mut ptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut uptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut vptr: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut _16x16mv: IntMv = IntMv { as_int: 0 };
        let mut ptr_base: *mut u8 = (*x).pre.y_buffer as *mut u8;
        let mut pre_stride: i32 = (*x).pre.y_stride;
        _16x16mv.as_int = (*(*x).mode_info_context).mbmi.mv.as_int;
        if (*(*x).mode_info_context).mbmi.need_to_clamp_mvs != 0 {
            clamp_mv_to_umv_border(&raw mut _16x16mv.as_mv, x);
        }
        ptr = ptr_base
            .offset(((_16x16mv.as_mv.row as i32 >> 3 as i32) * pre_stride) as isize)
            .offset((_16x16mv.as_mv.col as i32 >> 3 as i32) as isize);
        if _16x16mv.as_int & 0x70007 as u32 != 0 {
            (*x).subpixel_predict16x16
                .expect("non-null function pointer")(
                ptr,
                pre_stride,
                _16x16mv.as_mv.col as i32 & 7 as i32,
                _16x16mv.as_mv.row as i32 & 7 as i32,
                dst_y,
                dst_ystride,
            );
        } else {
            vp8_copy_mem16x16_c(ptr, pre_stride, dst_y, dst_ystride);
        }
        _16x16mv.as_mv.row = (_16x16mv.as_mv.row as i32
            + (1 as i32
                | _16x16mv.as_mv.row as i32
                    >> (::core::mem::size_of::<i32>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize))) as i16;
        _16x16mv.as_mv.col = (_16x16mv.as_mv.col as i32
            + (1 as i32
                | _16x16mv.as_mv.col as i32
                    >> (::core::mem::size_of::<i32>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize))) as i16;
        _16x16mv.as_mv.row = (_16x16mv.as_mv.row as i32 / 2 as i32) as i16;
        _16x16mv.as_mv.col = (_16x16mv.as_mv.col as i32 / 2 as i32) as i16;
        _16x16mv.as_mv.row = (_16x16mv.as_mv.row as i32 & (*x).fullpixel_mask) as i16;
        _16x16mv.as_mv.col = (_16x16mv.as_mv.col as i32 & (*x).fullpixel_mask) as i16;
        if (2 as i32 * _16x16mv.as_mv.col as i32) < (*x).mb_to_left_edge - ((19 as i32) << 3 as i32)
            || 2 as i32 * _16x16mv.as_mv.col as i32
                > (*x).mb_to_right_edge + ((18 as i32) << 3 as i32)
            || (2 as i32 * _16x16mv.as_mv.row as i32)
                < (*x).mb_to_top_edge - ((19 as i32) << 3 as i32)
            || 2 as i32 * _16x16mv.as_mv.row as i32
                > (*x).mb_to_bottom_edge + ((18 as i32) << 3 as i32)
        {
            return;
        }
        pre_stride >>= 1 as i32;
        offset = (_16x16mv.as_mv.row as i32 >> 3 as i32) * pre_stride
            + (_16x16mv.as_mv.col as i32 >> 3 as i32);
        uptr = (*x).pre.u_buffer.offset(offset as isize) as *mut u8;
        vptr = (*x).pre.v_buffer.offset(offset as isize) as *mut u8;
        if _16x16mv.as_int & 0x70007 as u32 != 0 {
            (*x).subpixel_predict8x8.expect("non-null function pointer")(
                uptr,
                pre_stride,
                _16x16mv.as_mv.col as i32 & 7 as i32,
                _16x16mv.as_mv.row as i32 & 7 as i32,
                dst_u,
                dst_uvstride,
            );
            (*x).subpixel_predict8x8.expect("non-null function pointer")(
                vptr,
                pre_stride,
                _16x16mv.as_mv.col as i32 & 7 as i32,
                _16x16mv.as_mv.row as i32 & 7 as i32,
                dst_v,
                dst_uvstride,
            );
        } else {
            vp8_copy_mem8x8_c(uptr, pre_stride, dst_u, dst_uvstride);
            vp8_copy_mem8x8_c(vptr, pre_stride, dst_v, dst_uvstride);
        };
    }
}
unsafe fn build_inter4x4_predictors_mb(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut i: i32 = 0;
        let mut base_dst: *mut u8 = (*x).dst.y_buffer as *mut u8;
        let mut base_pre: *mut u8 = (*x).pre.y_buffer as *mut u8;
        if ((*(*x).mode_info_context).mbmi.partitioning as i32) < 3 as i32 {
            let mut b: *mut BLOCKD = ::core::ptr::null_mut::<BLOCKD>();
            let mut dst_stride: i32 = (*x).dst.y_stride;
            (*x).block[0 as usize].bmi = (*(*x).mode_info_context).bmi[0 as usize];
            (*x).block[2 as usize].bmi = (*(*x).mode_info_context).bmi[2 as usize];
            (*x).block[8 as usize].bmi = (*(*x).mode_info_context).bmi[8 as usize];
            (*x).block[10 as usize].bmi = (*(*x).mode_info_context).bmi[10 as usize];
            if (*(*x).mode_info_context).mbmi.need_to_clamp_mvs != 0 {
                clamp_mv_to_umv_border(
                    &raw mut (*(&raw mut (*x).block as *mut BLOCKD).offset(0 as isize))
                        .bmi
                        .mv
                        .as_mv,
                    x,
                );
                clamp_mv_to_umv_border(
                    &raw mut (*(&raw mut (*x).block as *mut BLOCKD).offset(2 as isize))
                        .bmi
                        .mv
                        .as_mv,
                    x,
                );
                clamp_mv_to_umv_border(
                    &raw mut (*(&raw mut (*x).block as *mut BLOCKD).offset(8 as isize))
                        .bmi
                        .mv
                        .as_mv,
                    x,
                );
                clamp_mv_to_umv_border(
                    &raw mut (*(&raw mut (*x).block as *mut BLOCKD).offset(10 as isize))
                        .bmi
                        .mv
                        .as_mv,
                    x,
                );
            }
            b = (&raw mut (*x).block as *mut BLOCKD).offset(0 as isize) as *mut BLOCKD;
            build_inter_predictors4b(
                x,
                b,
                base_dst.offset((*b).offset as isize),
                dst_stride,
                base_pre,
                dst_stride,
            );
            b = (&raw mut (*x).block as *mut BLOCKD).offset(2 as isize) as *mut BLOCKD;
            build_inter_predictors4b(
                x,
                b,
                base_dst.offset((*b).offset as isize),
                dst_stride,
                base_pre,
                dst_stride,
            );
            b = (&raw mut (*x).block as *mut BLOCKD).offset(8 as isize) as *mut BLOCKD;
            build_inter_predictors4b(
                x,
                b,
                base_dst.offset((*b).offset as isize),
                dst_stride,
                base_pre,
                dst_stride,
            );
            b = (&raw mut (*x).block as *mut BLOCKD).offset(10 as isize) as *mut BLOCKD;
            build_inter_predictors4b(
                x,
                b,
                base_dst.offset((*b).offset as isize),
                dst_stride,
                base_pre,
                dst_stride,
            );
        } else {
            i = 0 as i32;
            while i < 16 as i32 {
                let mut d0: *mut BLOCKD =
                    (&raw mut (*x).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                let mut d1: *mut BLOCKD = (&raw mut (*x).block as *mut BLOCKD)
                    .offset((i + 1 as i32) as isize)
                    as *mut BLOCKD;
                let mut dst_stride_0: i32 = (*x).dst.y_stride;
                (*x).block[(i + 0 as i32) as usize].bmi =
                    (*(*x).mode_info_context).bmi[(i + 0 as i32) as usize];
                (*x).block[(i + 1 as i32) as usize].bmi =
                    (*(*x).mode_info_context).bmi[(i + 1 as i32) as usize];
                if (*(*x).mode_info_context).mbmi.need_to_clamp_mvs != 0 {
                    clamp_mv_to_umv_border(
                        &raw mut (*(&raw mut (*x).block as *mut BLOCKD)
                            .offset((i + 0 as i32) as isize))
                        .bmi
                        .mv
                        .as_mv,
                        x,
                    );
                    clamp_mv_to_umv_border(
                        &raw mut (*(&raw mut (*x).block as *mut BLOCKD)
                            .offset((i + 1 as i32) as isize))
                        .bmi
                        .mv
                        .as_mv,
                        x,
                    );
                }
                if (*d0).bmi.mv.as_int == (*d1).bmi.mv.as_int {
                    build_inter_predictors2b(
                        x,
                        d0,
                        base_dst.offset((*d0).offset as isize),
                        dst_stride_0,
                        base_pre,
                        dst_stride_0,
                    );
                } else {
                    build_inter_predictors_b(
                        d0,
                        base_dst.offset((*d0).offset as isize),
                        dst_stride_0,
                        base_pre,
                        dst_stride_0,
                        (*x).subpixel_predict,
                    );
                    build_inter_predictors_b(
                        d1,
                        base_dst.offset((*d1).offset as isize),
                        dst_stride_0,
                        base_pre,
                        dst_stride_0,
                        (*x).subpixel_predict,
                    );
                }
                i += 2 as i32;
            }
        }
        base_dst = (*x).dst.u_buffer as *mut u8;
        base_pre = (*x).pre.u_buffer as *mut u8;
        i = 16 as i32;
        while i < 20 as i32 {
            let mut d0_0: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
            let mut d1_0: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset((i + 1 as i32) as isize) as *mut BLOCKD;
            let mut dst_stride_1: i32 = (*x).dst.uv_stride;
            if (*d0_0).bmi.mv.as_int == (*d1_0).bmi.mv.as_int {
                build_inter_predictors2b(
                    x,
                    d0_0,
                    base_dst.offset((*d0_0).offset as isize),
                    dst_stride_1,
                    base_pre,
                    dst_stride_1,
                );
            } else {
                build_inter_predictors_b(
                    d0_0,
                    base_dst.offset((*d0_0).offset as isize),
                    dst_stride_1,
                    base_pre,
                    dst_stride_1,
                    (*x).subpixel_predict,
                );
                build_inter_predictors_b(
                    d1_0,
                    base_dst.offset((*d1_0).offset as isize),
                    dst_stride_1,
                    base_pre,
                    dst_stride_1,
                    (*x).subpixel_predict,
                );
            }
            i += 2 as i32;
        }
        base_dst = (*x).dst.v_buffer as *mut u8;
        base_pre = (*x).pre.v_buffer as *mut u8;
        i = 20 as i32;
        while i < 24 as i32 {
            let mut d0_1: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
            let mut d1_1: *mut BLOCKD =
                (&raw mut (*x).block as *mut BLOCKD).offset((i + 1 as i32) as isize) as *mut BLOCKD;
            let mut dst_stride_2: i32 = (*x).dst.uv_stride;
            if (*d0_1).bmi.mv.as_int == (*d1_1).bmi.mv.as_int {
                build_inter_predictors2b(
                    x,
                    d0_1,
                    base_dst.offset((*d0_1).offset as isize),
                    dst_stride_2,
                    base_pre,
                    dst_stride_2,
                );
            } else {
                build_inter_predictors_b(
                    d0_1,
                    base_dst.offset((*d0_1).offset as isize),
                    dst_stride_2,
                    base_pre,
                    dst_stride_2,
                    (*x).subpixel_predict,
                );
                build_inter_predictors_b(
                    d1_1,
                    base_dst.offset((*d1_1).offset as isize),
                    dst_stride_2,
                    base_pre,
                    dst_stride_2,
                    (*x).subpixel_predict,
                );
            }
            i += 2 as i32;
        }
    }
}
unsafe fn build_4x4uvmvs(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0 as i32;
        while i < 2 as i32 {
            j = 0 as i32;
            while j < 2 as i32 {
                let mut yoffset: i32 = i * 8 as i32 + j * 2 as i32;
                let mut uoffset: i32 = 16 as i32 + i * 2 as i32 + j;
                let mut voffset: i32 = 20 as i32 + i * 2 as i32 + j;
                let mut temp: i32 = 0;
                temp = (*(*x).mode_info_context).bmi[(yoffset + 0 as i32) as usize]
                    .mv
                    .as_mv
                    .row as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 1 as i32) as usize]
                        .mv
                        .as_mv
                        .row as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 4 as i32) as usize]
                        .mv
                        .as_mv
                        .row as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 5 as i32) as usize]
                        .mv
                        .as_mv
                        .row as i32;
                temp += 4 as i32
                    + (temp
                        >> (::core::mem::size_of::<i32>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize))
                        * 8 as i32;
                (*x).block[uoffset as usize].bmi.mv.as_mv.row =
                    ((temp / 8 as i32) & (*x).fullpixel_mask) as i16;
                temp = (*(*x).mode_info_context).bmi[(yoffset + 0 as i32) as usize]
                    .mv
                    .as_mv
                    .col as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 1 as i32) as usize]
                        .mv
                        .as_mv
                        .col as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 4 as i32) as usize]
                        .mv
                        .as_mv
                        .col as i32
                    + (*(*x).mode_info_context).bmi[(yoffset + 5 as i32) as usize]
                        .mv
                        .as_mv
                        .col as i32;
                temp += 4 as i32
                    + (temp
                        >> (::core::mem::size_of::<i32>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize))
                        * 8 as i32;
                (*x).block[uoffset as usize].bmi.mv.as_mv.col =
                    ((temp / 8 as i32) & (*x).fullpixel_mask) as i16;
                if (*(*x).mode_info_context).mbmi.need_to_clamp_mvs != 0 {
                    clamp_uvmv_to_umv_border(
                        &raw mut (*(&raw mut (*x).block as *mut BLOCKD).offset(uoffset as isize))
                            .bmi
                            .mv
                            .as_mv,
                        x,
                    );
                }
                (*x).block[voffset as usize].bmi.mv.as_int =
                    (*x).block[uoffset as usize].bmi.mv.as_int;
                j += 1;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_inter_predictors_mb(mut xd: *mut MACROBLOCKD) {
    unsafe {
        if (*(*xd).mode_info_context).mbmi.mode as i32 != SPLITMV as i32 {
            vp8_build_inter16x16_predictors_mb(
                xd,
                (*xd).dst.y_buffer as *mut u8,
                (*xd).dst.u_buffer as *mut u8,
                (*xd).dst.v_buffer as *mut u8,
                (*xd).dst.y_stride,
                (*xd).dst.uv_stride,
            );
        } else {
            build_4x4uvmvs(xd);
            build_inter4x4_predictors_mb(xd);
        };
    }
}
