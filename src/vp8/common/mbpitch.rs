use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
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
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
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
pub union BModeInfo {
    pub as_mode: u32,
    pub mv: IntMv,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
pub type ModeInfo = Modeinfo;
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
pub type BLOCKD = Blockd;
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
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
pub type MACROBLOCKD = Macroblockd;
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_block_dptrs(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        r = 0 as i32;
        while r < 4 as i32 {
            c = 0 as i32;
            while c < 4 as i32 {
                (*x).block[(r * 4 as i32 + c) as usize].predictor = (&raw mut (*x).predictor
                    as *mut u8)
                    .offset((r * 4 as i32 * 16 as i32) as isize)
                    .offset((c * 4 as i32) as isize);
                c += 1;
            }
            r += 1;
        }
        r = 0 as i32;
        while r < 2 as i32 {
            c = 0 as i32;
            while c < 2 as i32 {
                (*x).block[(16 as i32 + r * 2 as i32 + c) as usize].predictor =
                    (&raw mut (*x).predictor as *mut u8)
                        .offset(256 as isize)
                        .offset((r * 4 as i32 * 8 as i32) as isize)
                        .offset((c * 4 as i32) as isize);
                c += 1;
            }
            r += 1;
        }
        r = 0 as i32;
        while r < 2 as i32 {
            c = 0 as i32;
            while c < 2 as i32 {
                (*x).block[(20 as i32 + r * 2 as i32 + c) as usize].predictor =
                    (&raw mut (*x).predictor as *mut u8)
                        .offset(320 as isize)
                        .offset((r * 4 as i32 * 8 as i32) as isize)
                        .offset((c * 4 as i32) as isize);
                c += 1;
            }
            r += 1;
        }
        r = 0 as i32;
        while r < 25 as i32 {
            (*x).block[r as usize].qcoeff =
                (&raw mut (*x).qcoeff as *mut i16).offset((r * 16 as i32) as isize);
            (*x).block[r as usize].dqcoeff =
                (&raw mut (*x).dqcoeff as *mut i16).offset((r * 16 as i32) as isize);
            (*x).block[r as usize].eob = (&raw mut (*x).eobs as *mut i8).offset(r as isize);
            r += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_block_doffsets(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut block: i32 = 0;
        block = 0 as i32;
        while block < 16 as i32 {
            (*x).block[block as usize].offset =
                (block >> 2 as i32) * 4 as i32 * (*x).dst.y_stride + (block & 3 as i32) * 4 as i32;
            block += 1;
        }
        block = 16 as i32;
        while block < 20 as i32 {
            (*x).block[block as usize].offset =
                ((block - 16 as i32) >> 1 as i32) * 4 as i32 * (*x).dst.uv_stride
                    + (block & 1 as i32) * 4 as i32;
            (*x).block[(block + 4 as i32) as usize].offset = (*x).block[block as usize].offset;
            block += 1;
        }
    }
}
