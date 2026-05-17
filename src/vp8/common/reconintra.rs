unsafe extern "C" {
    fn vpx_dc_128_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_128_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_8x8_neon(
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
pub use crate::vp8::common::types::*;

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
pub type pthread_once_t = __darwin_pthread_once_t;

pub type intra_pred_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_SIZES: C2RustUnnamed = 2;
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) { unsafe {
    static mut lock: pthread_once_t = _opaque_pthread_once_t {
        __sig: _PTHREAD_ONCE_SIG_init as ::core::ffi::c_long,
        __opaque: [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };
    pthread_once(&raw mut lock, func as Option<unsafe extern "C" fn() -> ()>);
}}
static mut pred: [[intra_pred_fn; 2]; 4] = [[None; 2]; 4];
static mut dc_pred: [[[intra_pred_fn; 2]; 2]; 2] = [[[None; 2]; 2]; 2];
unsafe extern "C" fn vp8_init_intra_predictors_internal() { unsafe {
    pred[V_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_v_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[H_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_h_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[TM_PRED as ::core::ffi::c_int as usize][SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_tm_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_128_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_top_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_left_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_16 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_predictor_16x16_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_v_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[H_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_h_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    pred[TM_PRED as ::core::ffi::c_int as usize][SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_tm_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    )
        as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_128_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_top_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_left_predictor_8x8_neon
            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        [SIZE_8 as ::core::ffi::c_int as usize] = Some(
        vpx_dc_predictor_8x8_neon
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
