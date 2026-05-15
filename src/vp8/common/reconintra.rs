unsafe extern "C" {
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
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn vp8_init_intra4x4_predictors_internal();
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut ::core::ffi::c_short,
    pub dqcoeff: *mut ::core::ffi::c_short,
    pub predictor: *mut ::core::ffi::c_uchar,
    pub dequant: *mut ::core::ffi::c_short,
    pub offset: ::core::ffi::c_int,
    pub eob: *mut ::core::ffi::c_char,
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
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
pub type B_PREDICTION_MODE = ::core::ffi::c_uint;
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
    pub predictor: [::core::ffi::c_uchar; 384],
    pub qcoeff: [::core::ffi::c_short; 400],
    pub dqcoeff: [::core::ffi::c_short; 400],
    pub eobs: [::core::ffi::c_char; 25],
    pub dequant_y1: [::core::ffi::c_short; 16],
    pub dequant_y1_dc: [::core::ffi::c_short; 16],
    pub dequant_y2: [::core::ffi::c_short; 16],
    pub dequant_uv: [::core::ffi::c_short; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: ::core::ffi::c_int,
    pub pre: YV12_BUFFER_CONFIG,
    pub dst: YV12_BUFFER_CONFIG,
    pub mode_info_context: *mut MODE_INFO,
    pub mode_info_stride: ::core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub up_available: ::core::ffi::c_int,
    pub left_available: ::core::ffi::c_int,
    pub recon_above: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left_stride: [::core::ffi::c_int; 2],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: *mut ENTROPY_CONTEXT_PLANES,
    pub segmentation_enabled: ::core::ffi::c_uchar,
    pub update_mb_segmentation_map: ::core::ffi::c_uchar,
    pub update_mb_segmentation_data: ::core::ffi::c_uchar,
    pub mb_segment_abs_delta: ::core::ffi::c_uchar,
    pub mb_segment_tree_probs: [vp8_prob; 3],
    pub segment_feature_data: [[::core::ffi::c_schar; 4]; 2],
    pub mode_ref_lf_delta_enabled: ::core::ffi::c_uchar,
    pub mode_ref_lf_delta_update: ::core::ffi::c_uchar,
    pub last_ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub last_mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mb_to_left_edge: ::core::ffi::c_int,
    pub mb_to_right_edge: ::core::ffi::c_int,
    pub mb_to_top_edge: ::core::ffi::c_int,
    pub mb_to_bottom_edge: ::core::ffi::c_int,
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc: *mut ::core::ffi::c_void,
    pub corrupted: ::core::ffi::c_int,
    pub error_info: vpx_internal_error_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: ::core::ffi::c_int,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: ::core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [::core::ffi::c_int; 48];
pub type vpx_codec_err_t = ::core::ffi::c_uint;
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
pub type vp8_subpix_fn_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type vp8_prob = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type ENTROPY_CONTEXT = ::core::ffi::c_char;
pub type FRAME_TYPE = ::core::ffi::c_uint;
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
    pub y_width: ::core::ffi::c_int,
    pub y_height: ::core::ffi::c_int,
    pub y_crop_width: ::core::ffi::c_int,
    pub y_crop_height: ::core::ffi::c_int,
    pub y_stride: ::core::ffi::c_int,
    pub uv_width: ::core::ffi::c_int,
    pub uv_height: ::core::ffi::c_int,
    pub uv_crop_width: ::core::ffi::c_int,
    pub uv_crop_height: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub alpha_width: ::core::ffi::c_int,
    pub alpha_height: ::core::ffi::c_int,
    pub alpha_stride: ::core::ffi::c_int,
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: ::core::ffi::c_int,
    pub frame_size: size_t,
    pub subsampling_x: ::core::ffi::c_int,
    pub subsampling_y: ::core::ffi::c_int,
    pub bit_depth: ::core::ffi::c_uint,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: ::core::ffi::c_int,
    pub render_height: ::core::ffi::c_int,
    pub corrupted: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
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
pub type BLOCKD = blockd;
pub type pthread_once_t = *mut ::core::ffi::c_void;
pub type MB_PREDICTION_MODE = ::core::ffi::c_uint;
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
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_SIZES: C2RustUnnamed = 2;
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) {
    static INIT: std::sync::Once = std::sync::Once::new();
    if let Some(f) = func {
        INIT.call_once(|| f());
    }
}
static mut pred: [[intra_pred_fn; 2]; 4] = [[None; 2]; 4];
static mut dc_pred: [[[intra_pred_fn; 2]; 2]; 2] = [[[None; 2]; 2]; 2];
unsafe extern "C" fn vp8_init_intra_predictors_internal() { unsafe {
    pred[V_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_v_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[H_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_h_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[TM_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_tm_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_128_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_top_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_left_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_predictor_16x16_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_v_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[H_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_h_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[TM_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_tm_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_128_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_top_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_left_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_predictor_8x8_c
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    vp8_init_intra4x4_predictors_internal();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_build_intra_predictors_mby_s(
    mut x: *mut MACROBLOCKD,
    mut yabove_row: *mut ::core::ffi::c_uchar,
    mut yleft: *mut ::core::ffi::c_uchar,
    mut left_stride: ::core::ffi::c_int,
    mut ypred_ptr: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
) { unsafe {
    let mut mode: MB_PREDICTION_MODE = (*(*x).mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
    let mut yleft_col: [uint8_t; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = 0;
    let mut fn_0: intra_pred_fn = None;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        yleft_col[i as usize] = *yleft.offset((i * left_stride) as isize) as uint8_t;
        i += 1;
    }
    if mode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
        fn_0 = dc_pred[(*x).left_available as usize][(*x).up_available as usize]
            [SIZE_16 as ::core::ffi::c_int as usize];
    } else {
        fn_0 = pred[mode as usize][SIZE_16 as ::core::ffi::c_int as usize];
    }
    fn_0.expect("non-null function pointer")(
        ypred_ptr as *mut uint8_t,
        y_stride as ptrdiff_t,
        yabove_row,
        &raw mut yleft_col as *mut uint8_t,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_build_intra_predictors_mbuv_s(
    mut x: *mut MACROBLOCKD,
    mut uabove_row: *mut ::core::ffi::c_uchar,
    mut vabove_row: *mut ::core::ffi::c_uchar,
    mut uleft: *mut ::core::ffi::c_uchar,
    mut vleft: *mut ::core::ffi::c_uchar,
    mut left_stride: ::core::ffi::c_int,
    mut upred_ptr: *mut ::core::ffi::c_uchar,
    mut vpred_ptr: *mut ::core::ffi::c_uchar,
    mut pred_stride: ::core::ffi::c_int,
) { unsafe {
    let mut uvmode: MB_PREDICTION_MODE =
        (*(*x).mode_info_context).mbmi.uv_mode as MB_PREDICTION_MODE;
    let mut uleft_col: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut vleft_col: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut i: ::core::ffi::c_int = 0;
    let mut fn_0: intra_pred_fn = None;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        uleft_col[i as usize] = *uleft.offset((i * left_stride) as isize);
        vleft_col[i as usize] = *vleft.offset((i * left_stride) as isize);
        i += 1;
    }
    if uvmode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
        fn_0 = dc_pred[(*x).left_available as usize][(*x).up_available as usize]
            [SIZE_8 as ::core::ffi::c_int as usize];
    } else {
        fn_0 = pred[uvmode as usize][SIZE_8 as ::core::ffi::c_int as usize];
    }
    fn_0.expect("non-null function pointer")(
        upred_ptr as *mut uint8_t,
        pred_stride as ptrdiff_t,
        uabove_row,
        &raw mut uleft_col as *mut ::core::ffi::c_uchar,
    );
    fn_0.expect("non-null function pointer")(
        vpred_ptr as *mut uint8_t,
        pred_stride as ptrdiff_t,
        vabove_row,
        &raw mut vleft_col as *mut ::core::ffi::c_uchar,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_init_intra_predictors() { unsafe {
    once(Some(
        vp8_init_intra_predictors_internal as unsafe extern "C" fn() -> (),
    ));
}}
