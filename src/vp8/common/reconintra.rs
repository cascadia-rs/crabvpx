use std::ffi::c_void;
unsafe extern "Rust" {
    fn vpx_dc_128_predictor_16x16_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_dc_128_predictor_8x8_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_dc_left_predictor_16x16_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_dc_left_predictor_8x8_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_dc_predictor_16x16_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_dc_predictor_8x8_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_dc_top_predictor_16x16_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_dc_top_predictor_8x8_c(
        dst: *mut u8,
        stride: PtrdiffT,
        above: *const u8,
        left: *const u8,
    );
    fn vpx_h_predictor_16x16_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_h_predictor_8x8_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_tm_predictor_16x16_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_tm_predictor_8x8_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_v_predictor_16x16_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn vpx_v_predictor_8x8_c(dst: *mut u8, stride: PtrdiffT, above: *const u8, left: *const u8);
    fn pthread_once(_: *mut PthreadOnceT, _: Option<unsafe fn() -> ()>) -> i32;
    fn vp8_init_intra4x4_predictors_internal();
}
pub type DarwinPtrdiffT = isize;
pub type DarwinSizeT = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpaquePthreadOnceT {
    pub __sig: i64,
    pub __opaque: [i8; 8],
}
pub type DarwinPthreadOnceT = OpaquePthreadOnceT;
pub type PtrdiffT = DarwinPtrdiffT;
pub type SizeT = DarwinSizeT;
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
    pub as_mode: BPredictionMode,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
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
pub type Vp8SubpixFnT = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
pub type Vp8Prob = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type EntropyContext = i8;
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
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
pub type VpxColorRangeT = VpxColorRange;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type BLOCKD = Blockd;
pub type PthreadOnceT = *mut c_void;
pub type MbPredictionMode = u32;
pub const MB_MODE_COUNT: MbPredictionMode = 10;
pub const SPLITMV: MbPredictionMode = 9;
pub const NEWMV: MbPredictionMode = 8;
pub const ZEROMV: MbPredictionMode = 7;
pub const NEARMV: MbPredictionMode = 6;
pub const NEARESTMV: MbPredictionMode = 5;
pub const B_PRED: MbPredictionMode = 4;
pub const TM_PRED: MbPredictionMode = 3;
pub const H_PRED: MbPredictionMode = 2;
pub const V_PRED: MbPredictionMode = 1;
pub const DC_PRED: MbPredictionMode = 0;
pub type MACROBLOCKD = Macroblockd;
pub type IntraPredFn = Option<unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
#[allow(non_camel_case_types)]
pub type C2RustUnnamed = u32;
pub const NUM_SIZES: C2RustUnnamed = 2;
pub const _PTHREAD_ONCE_SIG_init: i32 = 0x30b1bcba as i32;
unsafe fn once(mut func: Option<unsafe fn() -> ()>) {
    unsafe {
        static INIT: std::sync::Once = std::sync::Once::new();
        if let Some(f) = func {
            INIT.call_once(|| f());
        }
    }
}
static mut pred: [[IntraPredFn; 2]; 4] = [[None; 2]; 4];
static mut dc_pred: [[[IntraPredFn; 2]; 2]; 2] = [[[None; 2]; 2]; 2];
unsafe fn vp8_init_intra_predictors_internal() {
    unsafe {
        pred[V_PRED as usize][SIZE_16 as usize] = Some(
            vpx_v_predictor_16x16_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[H_PRED as usize][SIZE_16 as usize] = Some(
            vpx_h_predictor_16x16_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[TM_PRED as usize][SIZE_16 as usize] = Some(
            vpx_tm_predictor_16x16_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[0 as usize][0 as usize][SIZE_16 as usize] = Some(
            vpx_dc_128_predictor_16x16_c
                as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[0 as usize][1 as usize][SIZE_16 as usize] = Some(
            vpx_dc_top_predictor_16x16_c
                as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[1 as usize][0 as usize][SIZE_16 as usize] = Some(
            vpx_dc_left_predictor_16x16_c
                as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[1 as usize][1 as usize][SIZE_16 as usize] = Some(
            vpx_dc_predictor_16x16_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        pred[V_PRED as usize][SIZE_8 as usize] =
            Some(vpx_v_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> ())
                as IntraPredFn;
        pred[H_PRED as usize][SIZE_8 as usize] =
            Some(vpx_h_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> ())
                as IntraPredFn;
        pred[TM_PRED as usize][SIZE_8 as usize] = Some(
            vpx_tm_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[0 as usize][0 as usize][SIZE_8 as usize] = Some(
            vpx_dc_128_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[0 as usize][1 as usize][SIZE_8 as usize] = Some(
            vpx_dc_top_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[1 as usize][0 as usize][SIZE_8 as usize] = Some(
            vpx_dc_left_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        dc_pred[1 as usize][1 as usize][SIZE_8 as usize] = Some(
            vpx_dc_predictor_8x8_c as unsafe fn(*mut u8, PtrdiffT, *const u8, *const u8) -> (),
        ) as IntraPredFn;
        vp8_init_intra4x4_predictors_internal();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_intra_predictors_mby_s(
    mut x: *mut MACROBLOCKD,
    mut yabove_row: *mut u8,
    mut yleft: *mut u8,
    mut left_stride: i32,
    mut ypred_ptr: *mut u8,
    mut y_stride: i32,
) {
    unsafe {
        let mut mode: MbPredictionMode = (*(*x).mode_info_context).mbmi.mode as MbPredictionMode;
        let mut yleft_col: [u8; 16] = [0; 16];
        let mut i: i32 = 0;
        let mut fn_0: IntraPredFn = None;
        i = 0 as i32;
        while i < 16 as i32 {
            yleft_col[i as usize] = *yleft.offset((i * left_stride) as isize) as u8;
            i += 1;
        }
        if mode as u32 == DC_PRED as u32 {
            fn_0 =
                dc_pred[(*x).left_available as usize][(*x).up_available as usize][SIZE_16 as usize];
        } else {
            fn_0 = pred[mode as usize][SIZE_16 as usize];
        }
        fn_0.expect("non-null function pointer")(
            ypred_ptr as *mut u8,
            y_stride as PtrdiffT,
            yabove_row,
            &raw mut yleft_col as *mut u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_build_intra_predictors_mbuv_s(
    mut x: *mut MACROBLOCKD,
    mut uabove_row: *mut u8,
    mut vabove_row: *mut u8,
    mut uleft: *mut u8,
    mut vleft: *mut u8,
    mut left_stride: i32,
    mut upred_ptr: *mut u8,
    mut vpred_ptr: *mut u8,
    mut pred_stride: i32,
) {
    unsafe {
        let mut uvmode: MbPredictionMode =
            (*(*x).mode_info_context).mbmi.uv_mode as MbPredictionMode;
        let mut uleft_col: [u8; 8] = [0; 8];
        let mut vleft_col: [u8; 8] = [0; 8];
        let mut i: i32 = 0;
        let mut fn_0: IntraPredFn = None;
        i = 0 as i32;
        while i < 8 as i32 {
            uleft_col[i as usize] = *uleft.offset((i * left_stride) as isize);
            vleft_col[i as usize] = *vleft.offset((i * left_stride) as isize);
            i += 1;
        }
        if uvmode as u32 == DC_PRED as u32 {
            fn_0 =
                dc_pred[(*x).left_available as usize][(*x).up_available as usize][SIZE_8 as usize];
        } else {
            fn_0 = pred[uvmode as usize][SIZE_8 as usize];
        }
        fn_0.expect("non-null function pointer")(
            upred_ptr as *mut u8,
            pred_stride as PtrdiffT,
            uabove_row,
            &raw mut uleft_col as *mut u8,
        );
        fn_0.expect("non-null function pointer")(
            vpred_ptr as *mut u8,
            pred_stride as PtrdiffT,
            vabove_row,
            &raw mut vleft_col as *mut u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_init_intra_predictors() {
    unsafe {
        once(Some(
            vp8_init_intra_predictors_internal as unsafe fn() -> (),
        ));
    }
}
