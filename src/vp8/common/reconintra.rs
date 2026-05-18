use std::ffi::c_void;
unsafe extern "Rust" {
    fn vpx_dc_128_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_128_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_16x16_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_8x8_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn pthread_once(_: *mut pthread_once_t, _: Option<unsafe fn() -> ()>) -> i32;
    fn vp8_init_intra4x4_predictors_internal();
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: i64,
    pub __opaque: [i8; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut i16,
    pub dqcoeff: *mut i16,
    pub predictor: *mut u8,
    pub dequant: *mut i16,
    pub offset: i32,
    pub eob: *mut i8,
    pub bmi: b_mode_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
pub type B_PREDICTION_MODE = u32;
pub const B_MODE_COUNT: B_PREDICTION_MODE = 14;
pub const NEW4X4: B_PREDICTION_MODE = 13;
pub const ZERO4X4: B_PREDICTION_MODE = 12;
pub const ABOVE4X4: B_PREDICTION_MODE = 11;
pub const LEFT4X4: B_PREDICTION_MODE = 10;
pub const B_HU_PRED: B_PREDICTION_MODE = 9;
pub const B_HD_PRED: B_PREDICTION_MODE = 8;
pub const B_VL_PRED: B_PREDICTION_MODE = 7;
pub const B_VR_PRED: B_PREDICTION_MODE = 6;
pub const B_RD_PRED: B_PREDICTION_MODE = 5;
pub const B_LD_PRED: B_PREDICTION_MODE = 4;
pub const B_HE_PRED: B_PREDICTION_MODE = 3;
pub const B_VE_PRED: B_PREDICTION_MODE = 2;
pub const B_TM_PRED: B_PREDICTION_MODE = 1;
pub const B_DC_PRED: B_PREDICTION_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
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
    pub pre: YV12_BUFFER_CONFIG,
    pub dst: YV12_BUFFER_CONFIG,
    pub mode_info_context: *mut MODE_INFO,
    pub mode_info_stride: i32,
    pub frame_type: FRAME_TYPE,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_above: [*mut u8; 3],
    pub recon_left: [*mut u8; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: *mut ENTROPY_CONTEXT_PLANES,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [vp8_prob; 3],
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
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc: *mut c_void,
    pub corrupted: i32,
    pub error_info: vpx_internal_error_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [i32; 48];
pub type vpx_codec_err_t = u32;
pub const VPX_CODEC_LIST_END: vpx_codec_err_t = 9;
pub const VPX_CODEC_INVALID_PARAM: vpx_codec_err_t = 8;
pub const VPX_CODEC_CORRUPT_FRAME: vpx_codec_err_t = 7;
pub const VPX_CODEC_UNSUP_FEATURE: vpx_codec_err_t = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: vpx_codec_err_t = 5;
pub const VPX_CODEC_INCAPABLE: vpx_codec_err_t = 4;
pub const VPX_CODEC_ABI_MISMATCH: vpx_codec_err_t = 3;
pub const VPX_CODEC_MEM_ERROR: vpx_codec_err_t = 2;
pub const VPX_CODEC_ERROR: vpx_codec_err_t = 1;
pub const VPX_CODEC_OK: vpx_codec_err_t = 0;
pub type vp8_subpix_fn_t = Option<unsafe fn(*mut u8, i32, i32, i32, *mut u8, i32) -> ()>;
pub type vp8_prob = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type ENTROPY_CONTEXT = i8;
pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
pub type MODE_INFO = modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub mode: uint8_t,
    pub uv_mode: uint8_t,
    pub ref_frame: uint8_t,
    pub is_4x4: uint8_t,
    pub mv: int_mv,
    pub partitioning: uint8_t,
    pub mb_skip_coeff: uint8_t,
    pub need_to_clamp_mvs: uint8_t,
    pub segment_id: uint8_t,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
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
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type BLOCKD = blockd;
pub type pthread_once_t = *mut c_void;
pub type MB_PREDICTION_MODE = u32;
pub const MB_MODE_COUNT: MB_PREDICTION_MODE = 10;
pub const SPLITMV: MB_PREDICTION_MODE = 9;
pub const NEWMV: MB_PREDICTION_MODE = 8;
pub const ZEROMV: MB_PREDICTION_MODE = 7;
pub const NEARMV: MB_PREDICTION_MODE = 6;
pub const NEARESTMV: MB_PREDICTION_MODE = 5;
pub const B_PRED: MB_PREDICTION_MODE = 4;
pub const TM_PRED: MB_PREDICTION_MODE = 3;
pub const H_PRED: MB_PREDICTION_MODE = 2;
pub const V_PRED: MB_PREDICTION_MODE = 1;
pub const DC_PRED: MB_PREDICTION_MODE = 0;
pub type MACROBLOCKD = macroblockd;
pub type intra_pred_fn =
    Option<unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
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
static mut pred: [[intra_pred_fn; 2]; 4] = [[None; 2]; 4];
static mut dc_pred: [[[intra_pred_fn; 2]; 2]; 2] = [[[None; 2]; 2]; 2];
unsafe fn vp8_init_intra_predictors_internal() {
    unsafe {
        pred[V_PRED as usize][SIZE_16 as usize] = Some(
            vpx_v_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[H_PRED as usize][SIZE_16 as usize] = Some(
            vpx_h_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[TM_PRED as usize][SIZE_16 as usize] = Some(
            vpx_tm_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[0 as usize][0 as usize][SIZE_16 as usize] = Some(
            vpx_dc_128_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[0 as usize][1 as usize][SIZE_16 as usize] = Some(
            vpx_dc_top_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[1 as usize][0 as usize][SIZE_16 as usize] = Some(
            vpx_dc_left_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[1 as usize][1 as usize][SIZE_16 as usize] = Some(
            vpx_dc_predictor_16x16_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[V_PRED as usize][SIZE_8 as usize] = Some(
            vpx_v_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[H_PRED as usize][SIZE_8 as usize] = Some(
            vpx_h_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        pred[TM_PRED as usize][SIZE_8 as usize] = Some(
            vpx_tm_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[0 as usize][0 as usize][SIZE_8 as usize] = Some(
            vpx_dc_128_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[0 as usize][1 as usize][SIZE_8 as usize] = Some(
            vpx_dc_top_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[1 as usize][0 as usize][SIZE_8 as usize] = Some(
            vpx_dc_left_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
        dc_pred[1 as usize][1 as usize][SIZE_8 as usize] = Some(
            vpx_dc_predictor_8x8_c
                as unsafe fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
        ) as intra_pred_fn;
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
        let mut mode: MB_PREDICTION_MODE =
            (*(*x).mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
        let mut yleft_col: [uint8_t; 16] = [0; 16];
        let mut i: i32 = 0;
        let mut fn_0: intra_pred_fn = None;
        i = 0 as i32;
        while i < 16 as i32 {
            yleft_col[i as usize] = *yleft.offset((i * left_stride) as isize) as uint8_t;
            i += 1;
        }
        if mode as u32 == DC_PRED as u32 {
            fn_0 =
                dc_pred[(*x).left_available as usize][(*x).up_available as usize][SIZE_16 as usize];
        } else {
            fn_0 = pred[mode as usize][SIZE_16 as usize];
        }
        fn_0.expect("non-null function pointer")(
            ypred_ptr as *mut uint8_t,
            y_stride as ptrdiff_t,
            yabove_row,
            &raw mut yleft_col as *mut uint8_t,
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
        let mut uvmode: MB_PREDICTION_MODE =
            (*(*x).mode_info_context).mbmi.uv_mode as MB_PREDICTION_MODE;
        let mut uleft_col: [u8; 8] = [0; 8];
        let mut vleft_col: [u8; 8] = [0; 8];
        let mut i: i32 = 0;
        let mut fn_0: intra_pred_fn = None;
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
            upred_ptr as *mut uint8_t,
            pred_stride as ptrdiff_t,
            uabove_row,
            &raw mut uleft_col as *mut u8,
        );
        fn_0.expect("non-null function pointer")(
            vpred_ptr as *mut uint8_t,
            pred_stride as ptrdiff_t,
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
