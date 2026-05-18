use std::ffi::c_void;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorRangeT = VpxColorRange;
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
pub type VpxCodecErrT = u32;
pub const VPX_CODEC_LIST_END: VpxCodecErrT = 9;
pub const VPX_CODEC_INVALID_PARAM: VpxCodecErrT = 8;
pub const VPX_CODEC_CORRUPT_FRAME: VpxCodecErrT = 7;
pub const VPX_CODEC_UNSUP_FEATURE: VpxCodecErrT = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: VpxCodecErrT = 5;
pub const VPX_CODEC_INCAPABLE: VpxCodecErrT = 4;
pub const VPX_CODEC_ABI_MISMATCH: VpxCodecErrT = 3;
pub const VPX_CODEC_MEM_ERROR: VpxCodecErrT = 2;
pub const VPX_CODEC_ERROR: VpxCodecErrT = 1;
pub const VPX_CODEC_OK: VpxCodecErrT = 0;
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: SizeT,
    pub border: i32,
    pub frame_size: SizeT,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: VpxColorSpaceT,
    pub color_range: VpxColorRangeT,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
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
pub type Vp8Prob = u8;
pub type EntropyContext = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
pub type BPredictionMode = u32;
pub const B_MODE_COUNT: BPredictionMode = 14;
pub const NEW4X4: BPredictionMode = 13;
pub const ZERO4X4: BPredictionMode = 12;
pub const ABOVE4X4: BPredictionMode = 11;
pub const LEFT4X4: BPredictionMode = 10;
pub const B_HU_PRED: BPredictionMode = 9;
pub const B_HD_PRED: BPredictionMode = 8;
pub const B_VL_PRED: BPredictionMode = 7;
pub const B_VR_PRED: BPredictionMode = 6;
pub const B_RD_PRED: BPredictionMode = 5;
pub const B_LD_PRED: BPredictionMode = 4;
pub const B_HE_PRED: BPredictionMode = 3;
pub const B_VE_PRED: BPredictionMode = 2;
pub const B_TM_PRED: BPredictionMode = 1;
pub const B_DC_PRED: BPredictionMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: BPredictionMode,
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
    pub frame_type: FrameType,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: *mut EntropyContextPlanes,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [Vp8Prob; 3],
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
