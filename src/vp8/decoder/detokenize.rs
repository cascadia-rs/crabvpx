unsafe extern "C" {
    static vp8_norm: [::core::ffi::c_uchar; 256];
    fn vp8dx_bool_decoder_fill(br: *mut BOOL_DECODER);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type int16_t = i16;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;
pub type pthread_t = *mut ::core::ffi::c_void;
pub type mach_port_t = __darwin_mach_port_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_COMP {
    pub mb: MACROBLOCKD,
    pub dec_fb_ref: [*mut YV12_BUFFER_CONFIG; 4],
    pub common: VP8_COMMON,
    pub mbc: [vp8_reader; 9],
    pub oxcf: VP8D_CONFIG,
    pub fragments: FRAGMENT_DATA,
    pub b_multithreaded_rd: vpx_atomic_int,
    pub max_threads: ::core::ffi::c_int,
    pub current_mb_col_main: ::core::ffi::c_int,
    pub decoding_thread_count: ::core::ffi::c_uint,
    pub allocated_decoding_thread_count: ::core::ffi::c_int,
    pub mt_baseline_filter_level: [::core::ffi::c_int; 4],
    pub sync_range: ::core::ffi::c_int,
    pub mt_current_mb_col: *mut vpx_atomic_int,
    pub mt_yabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_uabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_vabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_yleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_uleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_vleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mb_row_di: *mut MB_ROW_DEC,
    pub de_thread_data: *mut DECODETHREAD_DATA,
    pub h_decoding_thread: *mut pthread_t,
    pub h_event_start_decoding: *mut semaphore_t,
    pub h_event_end_decoding: semaphore_t,
    pub ready_for_new_data: ::core::ffi::c_int,
    pub prob_intra: vp8_prob,
    pub prob_last: vp8_prob,
    pub prob_gf: vp8_prob,
    pub prob_skip_false: vp8_prob,
    pub ec_enabled: ::core::ffi::c_int,
    pub ec_active: ::core::ffi::c_int,
    pub decoded_key_frame: ::core::ffi::c_int,
    pub independent_partitions: ::core::ffi::c_int,
    pub frame_corrupt_residual: ::core::ffi::c_int,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub restart_threads: ::core::ffi::c_int,
}
pub type vpx_decrypt_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type semaphore_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODETHREAD_DATA {
    pub ithread: ::core::ffi::c_int,
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_ROW_DEC {
    pub mbd: MACROBLOCKD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_atomic_int {
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAGMENT_DATA {
    pub enabled: ::core::ffi::c_int,
    pub count: ::core::ffi::c_uint,
    pub ptrs: [*const ::core::ffi::c_uchar; 9],
    pub sizes: [::core::ffi::c_uint; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_CONFIG {
    pub Width: ::core::ffi::c_int,
    pub Height: ::core::ffi::c_int,
    pub Version: ::core::ffi::c_int,
    pub postprocess: ::core::ffi::c_int,
    pub max_threads: ::core::ffi::c_int,
    pub error_concealment: ::core::ffi::c_int,
}
pub type BOOL_DECODER = vp8_reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_reader {
    pub user_buffer_end: *const ::core::ffi::c_uchar,
    pub user_buffer: *const ::core::ffi::c_uchar,
    pub value: VP8_BD_VALUE,
    pub count: ::core::ffi::c_int,
    pub range: ::core::ffi::c_uint,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
}
pub type VP8_BD_VALUE = size_t;
pub type VP8_COMMON = VP8Common;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[::core::ffi::c_uchar; 1]; 64],
    pub blim: [[::core::ffi::c_uchar; 1]; 64],
    pub lim: [[::core::ffi::c_uchar; 1]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 1]; 4],
    pub lvl: [[[::core::ffi::c_uchar; 4]; 4]; 4],
    pub hev_thr_lut: [[::core::ffi::c_uchar; 64]; 2],
    pub mode_lf_lut: [::core::ffi::c_uchar; 10],
}
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type ProbaArray = *const [[uint8_t; 11]; 3];
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
#[unsafe(no_mangle)]
pub extern "C" fn vp8_reset_mb_tokens_context(mut x: *mut MACROBLOCKD) {
    unsafe {
        let a_ctx = &mut *(*x).above_context;
        let l_ctx = &mut *(*x).left_context;
        a_ctx.y1 = [0; 4];
        a_ctx.u = [0; 2];
        a_ctx.v = [0; 2];
        l_ctx.y1 = [0; 4];
        l_ctx.u = [0; 2];
        l_ctx.v = [0; 2];
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            a_ctx.y2 = 0;
            l_ctx.y2 = 0;
        }
    }
}
static kBands: [uint8_t; 17] = [
    0, 1, 2, 3, 6, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 0,
];
static kCat3: [uint8_t; 3] = [173, 148, 140];
static kCat4: [uint8_t; 4] = [176, 155, 140, 135];
static kCat5: [uint8_t; 5] = [180, 157, 141, 134, 130];
static kCat6: [uint8_t; 11] = [254, 254, 243, 230, 196, 177, 153, 140, 133, 130, 129];
static kCat3456: [&[uint8_t]; 4] = [
    &kCat3,
    &kCat4,
    &kCat5,
    &kCat6,
];
static kZigzag: [uint8_t; 16] = [
    0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15,
];
fn GetSigned(
    br: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder,
    value_to_sign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if br.read_bool(128 as ::core::ffi::c_int) != 0 {
        -value_to_sign
    } else {
        value_to_sign
    }
}
fn GetCoeffs(
    br: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder,
    prob: &[[[vp8_prob; 11]; 3]; 8],
    ctx: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    out: &mut [int16_t],
) -> ::core::ffi::c_int {
    let mut p: &[vp8_prob; 11] = &prob[n as usize][ctx as usize];
    if br.read_bool(p[0] as ::core::ffi::c_int) == 0 {
        return 0;
    }
    loop {
        n += 1;
        if br.read_bool(p[1] as ::core::ffi::c_int) == 0 {
            p = &prob[kBands[n as usize] as usize][0];
        } else {
            let mut v: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            if br.read_bool(p[2] as ::core::ffi::c_int) == 0 {
                p = &prob[kBands[n as usize] as usize][1];
                v = 1;
            } else {
                if br.read_bool(p[3] as ::core::ffi::c_int) == 0 {
                    if br.read_bool(p[4] as ::core::ffi::c_int) == 0 {
                        v = 2;
                    } else {
                        v = 3 + br.read_bool(p[5] as ::core::ffi::c_int);
                    }
                } else if br.read_bool(p[6] as ::core::ffi::c_int) == 0 {
                    if br.read_bool(p[7] as ::core::ffi::c_int) == 0 {
                        v = 5 + br.read_bool(159);
                    } else {
                        v = 7 + 2 * br.read_bool(165);
                        v += br.read_bool(145);
                    }
                } else {
                    let bit1 = br.read_bool(p[8] as ::core::ffi::c_int);
                    let bit0 = br.read_bool(p[(9 + bit1) as usize] as ::core::ffi::c_int);
                    let cat = 2 * bit1 + bit0;
                    v = 0;
                    let tab = kCat3456[cat as usize];
                    for &prob_val in tab {
                        v += v + br.read_bool(prob_val as ::core::ffi::c_int);
                    }
                    v += 3 + (8 << cat);
                }
                p = &prob[kBands[n as usize] as usize][2];
            }
            j = kZigzag[(n - 1) as usize] as ::core::ffi::c_int;
            out[j as usize] = GetSigned(br, v) as int16_t;
            if n == 16 || br.read_bool(p[0] as ::core::ffi::c_int) == 0 {
                return n;
            }
        }
        if n == 16 {
            return 16;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_decode_mb_tokens(
    mut pbi: *mut VP8D_COMP,
    mut x: *mut MACROBLOCKD,
) -> ::core::ffi::c_int { unsafe {
    let pbi_mut = &mut *pbi;
    let x_ref = &mut *x;
    let bc = &mut pbi_mut.mbc[x_ref.current_bc_idx];
    let len = bc.user_buffer_end.offset_from(bc.user_buffer) as usize;
    let slice = if len == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(bc.user_buffer, len)
    };
    let mut safe_decoder = crate::vp8::decoder::dboolhuff::SafeBoolDecoder {
        buffer: slice,
        offset: 0,
        value: bc.value,
        count: bc.count,
        range: bc.range,
        decrypt_cb: bc.decrypt_cb,
        decrypt_state: bc.decrypt_state,
    };
    let fc_ref = &pbi_mut.common.fc;
    let a_ctx = &mut *x_ref.above_context;
    let l_ctx = &mut *x_ref.left_context;
    let mode_info = &*x_ref.mode_info_context;
    let mut eobtotal: ::core::ffi::c_int = 0;

    let mut skip_dc: ::core::ffi::c_int = 0;
    if mode_info.mbmi.is_4x4 == 0 {
        let coef_probs = &fc_ref.coef_probs[1];
        let ctx = a_ctx.y2 as ::core::ffi::c_int + l_ctx.y2 as ::core::ffi::c_int;
        let nonzeros = GetCoeffs(
            &mut safe_decoder,
            coef_probs,
            ctx,
            0,
            &mut x_ref.qcoeff[24 * 16 .. 25 * 16],
        );
        let nonzero_bool = (nonzeros > 0) as ::core::ffi::c_int as ENTROPY_CONTEXT;
        l_ctx.y2 = nonzero_bool;
        a_ctx.y2 = nonzero_bool;
        x_ref.eobs[24] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros - 16;
        skip_dc = 1;
    } else {
        skip_dc = 0;
    }

    let coef_probs_y1 = if mode_info.mbmi.is_4x4 == 0 {
        &fc_ref.coef_probs[0]
    } else {
        &fc_ref.coef_probs[3]
    };

    for i in 0..16 {
        let col = i & 3;
        let row = i >> 2;
        let ctx = a_ctx.y1[col] as ::core::ffi::c_int + l_ctx.y1[row] as ::core::ffi::c_int;
        let nonzeros = GetCoeffs(
            &mut safe_decoder,
            coef_probs_y1,
            ctx,
            skip_dc,
            &mut x_ref.qcoeff[(i * 16) as usize .. ((i + 1) * 16) as usize],
        );
        let nonzero_bool = (nonzeros > 0) as ::core::ffi::c_int as ENTROPY_CONTEXT;
        l_ctx.y1[row] = nonzero_bool;
        a_ctx.y1[col] = nonzero_bool;
        let eob_val = nonzeros + skip_dc;
        x_ref.eobs[i as usize] = eob_val as ::core::ffi::c_char;
        eobtotal += eob_val;
    }

    let coef_probs_uv = &fc_ref.coef_probs[2];

    for i in 16..20 {
        let col = i & 1;
        let row = (i & 3) >> 1;
        let ctx = a_ctx.u[col] as ::core::ffi::c_int + l_ctx.u[row] as ::core::ffi::c_int;
        let nonzeros = GetCoeffs(
            &mut safe_decoder,
            coef_probs_uv,
            ctx,
            0,
            &mut x_ref.qcoeff[(i * 16) as usize .. ((i + 1) * 16) as usize],
        );
        let nonzero_bool = (nonzeros > 0) as ::core::ffi::c_int as ENTROPY_CONTEXT;
        l_ctx.u[row] = nonzero_bool;
        a_ctx.u[col] = nonzero_bool;
        x_ref.eobs[i as usize] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros;
    }

    for i in 20..24 {
        let col = i & 1;
        let row = (i & 3) >> 1;
        let ctx = a_ctx.v[col] as ::core::ffi::c_int + l_ctx.v[row] as ::core::ffi::c_int;
        let nonzeros = GetCoeffs(
            &mut safe_decoder,
            coef_probs_uv,
            ctx,
            0,
            &mut x_ref.qcoeff[(i * 16) as usize .. ((i + 1) * 16) as usize],
        );
        let nonzero_bool = (nonzeros > 0) as ::core::ffi::c_int as ENTROPY_CONTEXT;
        l_ctx.v[row] = nonzero_bool;
        a_ctx.v[col] = nonzero_bool;
        x_ref.eobs[i as usize] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros;
    }

    bc.user_buffer = bc.user_buffer.add(safe_decoder.offset);
    bc.value = safe_decoder.value;
    bc.count = safe_decoder.count;
    bc.range = safe_decoder.range;
    return eobtotal;
}}
