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
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
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

pub type intra_pred_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_SIZES: C2RustUnnamed = 2;

static pred: [[intra_pred_fn; 2]; 4] = [
    [None, None],
    [
        Some(vpx_v_predictor_16x16_neon),
        Some(vpx_v_predictor_8x8_neon),
    ],
    [
        Some(vpx_h_predictor_16x16_neon),
        Some(vpx_h_predictor_8x8_neon),
    ],
    [
        Some(vpx_tm_predictor_16x16_neon),
        Some(vpx_tm_predictor_8x8_neon),
    ],
];

static dc_pred: [[[intra_pred_fn; 2]; 2]; 2] = [
    [
        [
            Some(vpx_dc_128_predictor_16x16_neon),
            Some(vpx_dc_128_predictor_8x8_neon),
        ],
        [
            Some(vpx_dc_top_predictor_16x16_neon),
            Some(vpx_dc_top_predictor_8x8_neon),
        ],
    ],
    [
        [
            Some(vpx_dc_left_predictor_16x16_neon),
            Some(vpx_dc_left_predictor_8x8_neon),
        ],
        [
            Some(vpx_dc_predictor_16x16_neon),
            Some(vpx_dc_predictor_8x8_neon),
        ],
    ],
];
pub fn vp8_build_intra_predictors_mby_s(
    x: &MACROBLOCKD,
    yabove_row: *mut ::core::ffi::c_uchar,
    yleft: *mut ::core::ffi::c_uchar,
    left_stride: ::core::ffi::c_int,
    ypred_ptr: *mut ::core::ffi::c_uchar,
    y_stride: ::core::ffi::c_int,
) {
    let mode = x.mode_info().mbmi.mode as MB_PREDICTION_MODE;
    let mut yleft_col: [uint8_t; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = 0;
    while i < 16 {
        yleft_col[i as usize] = unsafe { *yleft.offset((i * left_stride) as isize) } as uint8_t;
        i += 1;
    }
    let fn_0 = if mode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_uint {
        dc_pred[x.left_available as usize][x.up_available as usize]
            [SIZE_16 as usize]
    } else {
        pred[mode as usize][SIZE_16 as usize]
    };
    unsafe {
        fn_0.expect("non-null function pointer")(
            ypred_ptr as *mut uint8_t,
            y_stride as ptrdiff_t,
            yabove_row,
            &raw mut yleft_col as *mut uint8_t,
        );
    }
}
pub fn vp8_build_intra_predictors_mbuv_s(
    x: &MACROBLOCKD,
    uabove_row: *mut ::core::ffi::c_uchar,
    vabove_row: *mut ::core::ffi::c_uchar,
    uleft: *mut ::core::ffi::c_uchar,
    vleft: *mut ::core::ffi::c_uchar,
    left_stride: ::core::ffi::c_int,
    upred_ptr: *mut ::core::ffi::c_uchar,
    vpred_ptr: *mut ::core::ffi::c_uchar,
    pred_stride: ::core::ffi::c_int,
) {
    let uvmode = x.mode_info().mbmi.uv_mode as MB_PREDICTION_MODE;
    let mut uleft_col: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut vleft_col: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut i: ::core::ffi::c_int = 0;
    while i < 8 {
        uleft_col[i as usize] = unsafe { *uleft.offset((i * left_stride) as isize) };
        vleft_col[i as usize] = unsafe { *vleft.offset((i * left_stride) as isize) };
        i += 1;
    }
    let fn_0 = if uvmode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_uint {
        dc_pred[x.left_available as usize][x.up_available as usize]
            [SIZE_8 as usize]
    } else {
        pred[uvmode as usize][SIZE_8 as usize]
    };
    unsafe {
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
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_init_intra_predictors() {}

pub fn intra_prediction_down_copy(
    xd: &mut MACROBLOCKD,
) {
    let dst_stride = xd.dst.y_stride as usize;
    let border = xd.dst.border as usize;

    // Safety: xd.dst must be valid.
    let y_slice = unsafe { xd.dst.y_slice_mut() };

    let base_idx = (border - 1) * dst_stride + border + 16;

    let mut src_bytes = [0u8; 4];
    src_bytes.copy_from_slice(&y_slice[base_idx..base_idx + 4]);

    let idx0 = base_idx + 4 * dst_stride;
    let idx1 = base_idx + 8 * dst_stride;
    let idx2 = base_idx + 12 * dst_stride;

    y_slice[idx0..idx0 + 4].copy_from_slice(&src_bytes);
    y_slice[idx1..idx1 + 4].copy_from_slice(&src_bytes);
    y_slice[idx2..idx2 + 4].copy_from_slice(&src_bytes);
}

