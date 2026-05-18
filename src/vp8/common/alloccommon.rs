unsafe extern "C" {
    fn vp8_yv12_alloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        border: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn vp8_yv12_de_alloc_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG) -> ::core::ffi::c_int;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vp8_init_mbmode_probs(x: *mut VP8_COMMON);
    fn vp8_default_bmode_probs(dest: *mut vp8_prob);
    fn vp8_machine_specific_config(_: *mut VP8Common);
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
pub type uint32_t = u32;
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
pub type uint8_t = u8;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[::core::ffi::c_uchar; 16]; 64],
    pub blim: [[::core::ffi::c_uchar; 16]; 64],
    pub lim: [[::core::ffi::c_uchar; 16]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 16]; 4],
    pub lvl: [[[::core::ffi::c_uchar; 4]; 4]; 4],
    pub hev_thr_lut: [[::core::ffi::c_uchar; 64]; 2],
    pub mode_lf_lut: [::core::ffi::c_uchar; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub Y1dequant: [[::core::ffi::c_short; 2]; 128],
    pub Y2dequant: [[::core::ffi::c_short; 2]; 128],
    pub UVdequant: [[::core::ffi::c_short; 2]; 128],
    pub Width: ::core::ffi::c_int,
    pub Height: ::core::ffi::c_int,
    pub horiz_scale: ::core::ffi::c_int,
    pub vert_scale: ::core::ffi::c_int,
    pub clamp_type: CLAMP_TYPE,
    pub frame_to_show: *mut YV12_BUFFER_CONFIG,
    pub yv12_fb: [YV12_BUFFER_CONFIG; 4],
    pub fb_idx_ref_cnt: [::core::ffi::c_int; 4],
    pub new_fb_idx: ::core::ffi::c_int,
    pub lst_fb_idx: ::core::ffi::c_int,
    pub gld_fb_idx: ::core::ffi::c_int,
    pub alt_fb_idx: ::core::ffi::c_int,
    pub temp_scale_frame: YV12_BUFFER_CONFIG,
    pub last_frame_type: FRAME_TYPE,
    pub frame_type: FRAME_TYPE,
    pub show_frame: ::core::ffi::c_int,
    pub frame_flags: ::core::ffi::c_int,
    pub MBs: ::core::ffi::c_int,
    pub mb_rows: ::core::ffi::c_int,
    pub mb_cols: ::core::ffi::c_int,
    pub mode_info_stride: ::core::ffi::c_int,
    pub mb_no_coeff_skip: ::core::ffi::c_int,
    pub no_lpf: ::core::ffi::c_int,
    pub use_bilinear_mc_filter: ::core::ffi::c_int,
    pub full_pixel: ::core::ffi::c_int,
    pub base_qindex: ::core::ffi::c_int,
    pub y1dc_delta_q: ::core::ffi::c_int,
    pub y2dc_delta_q: ::core::ffi::c_int,
    pub y2ac_delta_q: ::core::ffi::c_int,
    pub uvdc_delta_q: ::core::ffi::c_int,
    pub uvac_delta_q: ::core::ffi::c_int,
    pub mip: *mut MODE_INFO,
    pub mi: *mut MODE_INFO,
    pub show_frame_mi: *mut MODE_INFO,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: loop_filter_info_n,
    pub filter_level: ::core::ffi::c_int,
    pub last_sharpness_level: ::core::ffi::c_int,
    pub sharpness_level: ::core::ffi::c_int,
    pub refresh_last_frame: ::core::ffi::c_int,
    pub refresh_golden_frame: ::core::ffi::c_int,
    pub refresh_alt_ref_frame: ::core::ffi::c_int,
    pub copy_buffer_to_gf: ::core::ffi::c_int,
    pub copy_buffer_to_arf: ::core::ffi::c_int,
    pub refresh_entropy_probs: ::core::ffi::c_int,
    pub ref_frame_sign_bias: [::core::ffi::c_int; 4],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: ::core::ffi::c_uint,
    pub version: ::core::ffi::c_int,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: ::core::ffi::c_int,
}
pub type TOKEN_PARTITION = ::core::ffi::c_uint;
pub const EIGHT_PARTITION: TOKEN_PARTITION = 3;
pub const FOUR_PARTITION: TOKEN_PARTITION = 2;
pub const TWO_PARTITION: TOKEN_PARTITION = 1;
pub const ONE_PARTITION: TOKEN_PARTITION = 0;
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub bmode_prob: [vp8_prob; 9],
    pub ymode_prob: [vp8_prob; 4],
    pub uv_mode_prob: [vp8_prob; 3],
    pub sub_mv_ref_prob: [vp8_prob; 3],
    pub coef_probs: [[[[vp8_prob; 11]; 3]; 8]; 4],
    pub mvc: [MV_CONTEXT; 2],
}
pub type MV_CONTEXT = mv_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv_context {
    pub prob: [vp8_prob; 19],
}
pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type VP8_COMMON = VP8Common;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const VP8BORDERINPIXELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NUM_YV12_BUFFERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_de_alloc_frame_buffers(mut oci: *mut VP8_COMMON) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < NUM_YV12_BUFFERS {
            vp8_yv12_de_alloc_frame_buffer(
                (&raw mut (*oci).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(i as isize)
                    as *mut YV12_BUFFER_CONFIG,
            );
            (*oci).fb_idx_ref_cnt[i as usize] = 0 as ::core::ffi::c_int;
            i += 1;
        }
        vp8_yv12_de_alloc_frame_buffer(&raw mut (*oci).temp_scale_frame);
        vpx_free((*oci).above_context as *mut ::core::ffi::c_void);
        vpx_free((*oci).mip as *mut ::core::ffi::c_void);
        (*oci).above_context = ::core::ptr::null_mut::<ENTROPY_CONTEXT_PLANES>();
        (*oci).mip = ::core::ptr::null_mut::<MODE_INFO>();
        (*oci).mi = ::core::ptr::null_mut::<MODE_INFO>();
        (*oci).show_frame_mi = ::core::ptr::null_mut::<MODE_INFO>();
        (*oci).frame_to_show = ::core::ptr::null_mut::<YV12_BUFFER_CONFIG>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_alloc_frame_buffers(
    mut oci: *mut VP8_COMMON,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        vp8_de_alloc_frame_buffers(oci);
        if width & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            width += 16 as ::core::ffi::c_int - (width & 0xf as ::core::ffi::c_int);
        }
        if height & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            height += 16 as ::core::ffi::c_int - (height & 0xf as ::core::ffi::c_int);
        }
        i = 0 as ::core::ffi::c_int;
        loop {
            if !(i < NUM_YV12_BUFFERS) {
                current_block = 10886091980245723256;
                break;
            }
            if vp8_yv12_alloc_frame_buffer(
                (&raw mut (*oci).yv12_fb as *mut YV12_BUFFER_CONFIG).offset(i as isize)
                    as *mut YV12_BUFFER_CONFIG,
                width,
                height,
                VP8BORDERINPIXELS,
            ) < 0 as ::core::ffi::c_int
            {
                current_block = 15863795849835083362;
                break;
            }
            i += 1;
        }
        match current_block {
            10886091980245723256 => {
                (*oci).new_fb_idx = 0 as ::core::ffi::c_int;
                (*oci).lst_fb_idx = 1 as ::core::ffi::c_int;
                (*oci).gld_fb_idx = 2 as ::core::ffi::c_int;
                (*oci).alt_fb_idx = 3 as ::core::ffi::c_int;
                (*oci).fb_idx_ref_cnt[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                (*oci).fb_idx_ref_cnt[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                (*oci).fb_idx_ref_cnt[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                (*oci).fb_idx_ref_cnt[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                if !(vp8_yv12_alloc_frame_buffer(
                    &raw mut (*oci).temp_scale_frame,
                    width,
                    16 as ::core::ffi::c_int,
                    VP8BORDERINPIXELS,
                ) < 0 as ::core::ffi::c_int)
                {
                    (*oci).mb_rows = height >> 4 as ::core::ffi::c_int;
                    (*oci).mb_cols = width >> 4 as ::core::ffi::c_int;
                    (*oci).MBs = (*oci).mb_rows * (*oci).mb_cols;
                    (*oci).mode_info_stride = (*oci).mb_cols + 1 as ::core::ffi::c_int;
                    (*oci).mip = vpx_calloc(
                        (((*oci).mb_cols + 1 as ::core::ffi::c_int)
                            * ((*oci).mb_rows + 1 as ::core::ffi::c_int))
                            as size_t,
                        ::core::mem::size_of::<MODE_INFO>() as size_t,
                    ) as *mut MODE_INFO;
                    if !(*oci).mip.is_null() {
                        (*oci).mi = (*oci)
                            .mip
                            .offset((*oci).mode_info_stride as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                        (*oci).above_context = vpx_calloc(
                            (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t)
                                .wrapping_mul((*oci).mb_cols as size_t),
                            1 as size_t,
                        )
                            as *mut ENTROPY_CONTEXT_PLANES;
                        if !(*oci).above_context.is_null() {
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            _ => {}
        }
        vp8_de_alloc_frame_buffers(oci);
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_setup_version(mut cm: *mut VP8_COMMON) {
    unsafe {
        match (*cm).version {
            0 => {
                (*cm).no_lpf = 0 as ::core::ffi::c_int;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
                (*cm).full_pixel = 0 as ::core::ffi::c_int;
            }
            1 => {
                (*cm).no_lpf = 0 as ::core::ffi::c_int;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
                (*cm).full_pixel = 0 as ::core::ffi::c_int;
            }
            2 => {
                (*cm).no_lpf = 1 as ::core::ffi::c_int;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
                (*cm).full_pixel = 0 as ::core::ffi::c_int;
            }
            3 => {
                (*cm).no_lpf = 1 as ::core::ffi::c_int;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as ::core::ffi::c_int;
                (*cm).full_pixel = 1 as ::core::ffi::c_int;
            }
            _ => {
                (*cm).no_lpf = 0 as ::core::ffi::c_int;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
                (*cm).full_pixel = 0 as ::core::ffi::c_int;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_create_common(mut oci: *mut VP8_COMMON) {
    unsafe {
        vp8_machine_specific_config(oci as *mut VP8Common);
        vp8_init_mbmode_probs(oci);
        vp8_default_bmode_probs(&raw mut (*oci).fc.bmode_prob as *mut vp8_prob);
        (*oci).mb_no_coeff_skip = 1 as ::core::ffi::c_int;
        (*oci).no_lpf = 0 as ::core::ffi::c_int;
        (*oci).filter_type = NORMAL_LOOPFILTER;
        (*oci).use_bilinear_mc_filter = 0 as ::core::ffi::c_int;
        (*oci).full_pixel = 0 as ::core::ffi::c_int;
        (*oci).multi_token_partition = ONE_PARTITION;
        (*oci).clamp_type = RECON_CLAMP_REQUIRED;
        memset(
            &raw mut (*oci).ref_frame_sign_bias as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_int; 4]>() as size_t,
        );
        (*oci).copy_buffer_to_gf = 0 as ::core::ffi::c_int;
        (*oci).copy_buffer_to_arf = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_remove_common(mut oci: *mut VP8_COMMON) {
    unsafe {
        vp8_de_alloc_frame_buffers(oci);
    }
}
