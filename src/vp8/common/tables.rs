pub const ZERO_TOKEN: i32 = 0;
pub const ONE_TOKEN: i32 = 1;
pub const TWO_TOKEN: i32 = 2;
pub const THREE_TOKEN: i32 = 3;
pub const FOUR_TOKEN: i32 = 4;
pub const DCT_VAL_CATEGORY1: i32 = 5;
pub const DCT_VAL_CATEGORY2: i32 = 6;
pub const DCT_VAL_CATEGORY3: i32 = 7;
pub const DCT_VAL_CATEGORY4: i32 = 8;
pub const DCT_VAL_CATEGORY5: i32 = 9;
pub const DCT_VAL_CATEGORY6: i32 = 10;
pub const DCT_EOB_TOKEN: i32 = 11;

pub const B_DC_PRED: i32 = 0;
pub const B_TM_PRED: i32 = 1;
pub const B_VE_PRED: i32 = 2;
pub const B_HE_PRED: i32 = 3;
pub const B_LD_PRED: i32 = 4;
pub const B_RD_PRED: i32 = 5;
pub const B_VR_PRED: i32 = 6;
pub const B_VL_PRED: i32 = 7;
pub const B_HD_PRED: i32 = 8;
pub const B_HU_PRED: i32 = 9;
pub const LEFT4X4: i32 = 10;
pub const ABOVE4X4: i32 = 11;
pub const ZERO4X4: i32 = 12;
pub const NEW4X4: i32 = 13;

pub const DC_PRED: i32 = 0;
pub const V_PRED: i32 = 1;
pub const H_PRED: i32 = 2;
pub const TM_PRED: i32 = 3;
pub const B_PRED: i32 = 4;

pub const NEARESTMV: i32 = 5;
pub const NEARMV: i32 = 6;
pub const ZEROMV: i32 = 7;
pub const NEWMV: i32 = 8;
pub const SPLITMV: i32 = 9;

pub const MB_MODE_COUNT: i32 = 10;
pub const B_MODE_COUNT: i32 = 14;

#[unsafe(no_mangle)]
pub static VPX_NORM: [u8; 256] = [
    0, 7, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[unsafe(no_mangle)]
pub static vp8_coef_bands: [u8; 16] = [0, 1, 2, 3, 6, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7];

#[unsafe(no_mangle)]
pub static vp8_prev_token_class: [u8; 12] = [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0];

#[unsafe(no_mangle)]
pub static vp8_default_zig_zag1d: [i32; 16] =
    [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

#[unsafe(no_mangle)]
pub static vp8_default_inv_zig_zag: [i16; 16] =
    [1, 2, 6, 7, 3, 5, 8, 13, 4, 9, 12, 14, 10, 11, 15, 16];

#[unsafe(no_mangle)]
pub static vp8_default_zig_zag_mask: [i16; 16] = [
    1, 2, 32, 64, 4, 16, 128, 4096, 8, 256, 2048, 8192, 512, 1024, 16384, -32768,
];

#[unsafe(no_mangle)]
pub static vp8_mb_feature_data_bits: [i32; 2] = [7, 6];

#[unsafe(no_mangle)]
pub static vp8_coef_tree: [i8; 22] = [
    -(DCT_EOB_TOKEN as i8),
    2,
    -(ZERO_TOKEN as i8),
    4,
    -(ONE_TOKEN as i8),
    6,
    8,
    12,
    -(TWO_TOKEN as i8),
    10,
    -(THREE_TOKEN as i8),
    -(FOUR_TOKEN as i8),
    14,
    16,
    -(DCT_VAL_CATEGORY1 as i8),
    -(DCT_VAL_CATEGORY2 as i8),
    18,
    20,
    -(DCT_VAL_CATEGORY3 as i8),
    -(DCT_VAL_CATEGORY4 as i8),
    -(DCT_VAL_CATEGORY5 as i8),
    -(DCT_VAL_CATEGORY6 as i8),
];

#[unsafe(no_mangle)]
pub static vp8_bmode_tree: [i8; 18] = [
    -(B_DC_PRED as i8),
    2,
    -(B_TM_PRED as i8),
    4,
    -(B_VE_PRED as i8),
    6,
    8,
    12,
    -(B_HE_PRED as i8),
    10,
    -(B_RD_PRED as i8),
    -(B_VR_PRED as i8),
    -(B_LD_PRED as i8),
    14,
    -(B_VL_PRED as i8),
    16,
    -(B_HD_PRED as i8),
    -(B_HU_PRED as i8),
];

#[unsafe(no_mangle)]
pub static vp8_ymode_tree: [i8; 8] = [
    -(DC_PRED as i8),
    2,
    4,
    6,
    -(V_PRED as i8),
    -(H_PRED as i8),
    -(TM_PRED as i8),
    -(B_PRED as i8),
];

#[unsafe(no_mangle)]
pub static vp8_kf_ymode_tree: [i8; 8] = [
    -(B_PRED as i8),
    2,
    4,
    6,
    -(DC_PRED as i8),
    -(V_PRED as i8),
    -(H_PRED as i8),
    -(TM_PRED as i8),
];

#[unsafe(no_mangle)]
pub static vp8_uv_mode_tree: [i8; 6] = [
    -(DC_PRED as i8),
    2,
    -(V_PRED as i8),
    4,
    -(H_PRED as i8),
    -(TM_PRED as i8),
];

#[unsafe(no_mangle)]
pub static vp8_mbsplit_tree: [i8; 6] = [-3, 2, -2, 4, -0, -1];

#[unsafe(no_mangle)]
pub static vp8_mv_ref_tree: [i8; 8] = [
    -(ZEROMV as i8),
    2,
    -(NEARESTMV as i8),
    4,
    -(NEARMV as i8),
    6,
    -(NEWMV as i8),
    -(SPLITMV as i8),
];

#[unsafe(no_mangle)]
pub static vp8_sub_mv_ref_tree: [i8; 6] = [
    -(LEFT4X4 as i8),
    2,
    -(ABOVE4X4 as i8),
    4,
    -(ZERO4X4 as i8),
    -(NEW4X4 as i8),
];

#[unsafe(no_mangle)]
pub static vp8_small_mvtree: [i8; 14] = [2, 8, 4, 6, -0, -1, -2, -3, 10, 12, -4, -5, -6, -7];
