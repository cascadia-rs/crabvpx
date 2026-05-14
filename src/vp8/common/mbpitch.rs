pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
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
pub type jmp_buf = [::core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: ::core::ffi::c_int,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: ::core::ffi::c_int,
    pub jmp: jmp_buf,
}
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
pub type vp8_prob = ::core::ffi::c_uchar;
pub type ENTROPY_CONTEXT = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type FRAME_TYPE = ::core::ffi::c_uint;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
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
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}
pub type MODE_INFO = modeinfo;
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
pub type BLOCKD = blockd;
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
pub type MACROBLOCKD = macroblockd;
#[no_mangle]
pub unsafe extern "C" fn vp8_setup_block_dptrs(mut x: *mut MACROBLOCKD) {
    let mut r: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 4 as ::core::ffi::c_int {
            (*x).block[(r * 4 as ::core::ffi::c_int + c) as usize].predictor =
                (&raw mut (*x).predictor as *mut ::core::ffi::c_uchar)
                    .offset((r * 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
                    .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            (*x).block[(16 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = (&raw mut (*x).predictor as *mut ::core::ffi::c_uchar)
                .offset(256 as ::core::ffi::c_int as isize)
                .offset((r * 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 2 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
        while c < 2 as ::core::ffi::c_int {
            (*x).block[(20 as ::core::ffi::c_int + r * 2 as ::core::ffi::c_int + c) as usize]
                .predictor = (&raw mut (*x).predictor as *mut ::core::ffi::c_uchar)
                .offset(320 as ::core::ffi::c_int as isize)
                .offset((r * 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                .offset((c * 4 as ::core::ffi::c_int) as isize);
            c += 1;
        }
        r += 1;
    }
    r = 0 as ::core::ffi::c_int;
    while r < 25 as ::core::ffi::c_int {
        (*x).block[r as usize].qcoeff = (&raw mut (*x).qcoeff as *mut ::core::ffi::c_short)
            .offset((r * 16 as ::core::ffi::c_int) as isize);
        (*x).block[r as usize].dqcoeff = (&raw mut (*x).dqcoeff as *mut ::core::ffi::c_short)
            .offset((r * 16 as ::core::ffi::c_int) as isize);
        (*x).block[r as usize].eob =
            (&raw mut (*x).eobs as *mut ::core::ffi::c_char).offset(r as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_build_block_doffsets(mut x: *mut MACROBLOCKD) {
    let mut block: ::core::ffi::c_int = 0;
    block = 0 as ::core::ffi::c_int;
    while block < 16 as ::core::ffi::c_int {
        (*x).block[block as usize].offset =
            (block >> 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * (*x).dst.y_stride
                + (block & 3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        block += 1;
    }
    block = 16 as ::core::ffi::c_int;
    while block < 20 as ::core::ffi::c_int {
        (*x).block[block as usize].offset = (block - 16 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
            * 4 as ::core::ffi::c_int
            * (*x).dst.uv_stride
            + (block & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int;
        (*x).block[(block + 4 as ::core::ffi::c_int) as usize].offset =
            (*x).block[block as usize].offset;
        block += 1;
    }
}
