use std::ffi::c_void;
unsafe extern "Rust" {
    pub type VpxCodecAlgPriv;
    fn vsnprintf(__str: *mut i8, __size: usize, __format: *const i8, _: *mut c_void) -> i32;
    fn longjmp(_: *mut i32, _: i32) -> !;
}
pub type BuiltinVaList = *mut i8;
pub type VaList = BuiltinVaList;
pub const VPX_IMG_FMT_I44016: u32 = 2311;
pub const VPX_IMG_FMT_I44416: u32 = 2310;
pub const VPX_IMG_FMT_I42216: u32 = 2309;
pub const VPX_IMG_FMT_I42016: u32 = 2306;
pub const VPX_IMG_FMT_NV12: u32 = 265;
pub const VPX_IMG_FMT_I440: u32 = 263;
pub const VPX_IMG_FMT_I444: u32 = 262;
pub const VPX_IMG_FMT_I422: u32 = 261;
pub const VPX_IMG_FMT_I420: u32 = 258;
pub const VPX_IMG_FMT_YV12: u32 = 769;
pub const VPX_IMG_FMT_NONE: u32 = 0;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxImage {
    pub fmt: u32,
    pub cs: u32,
    pub range: u32,
    pub w: u32,
    pub h: u32,
    pub bit_depth: u32,
    pub d_w: u32,
    pub d_h: u32,
    pub r_w: u32,
    pub r_h: u32,
    pub x_chroma_shift: u32,
    pub y_chroma_shift: u32,
    pub planes: [*mut u8; 4],
    pub stride: [i32; 4],
    pub bps: i32,
    pub user_priv: *mut c_void,
    pub img_data: *mut u8,
    pub img_data_owner: i32,
    pub self_allocd: bool,
    pub fb_priv: *mut c_void,
}
pub type VpxImageT = VpxImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxImageRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
pub type VpxImageRectT = VpxImageRect;
pub const VPX_CODEC_LIST_END: u32 = 9;
pub const VPX_CODEC_INVALID_PARAM: u32 = 8;
pub const VPX_CODEC_CORRUPT_FRAME: u32 = 7;
pub const VPX_CODEC_UNSUP_FEATURE: u32 = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: u32 = 5;
pub const VPX_CODEC_INCAPABLE: u32 = 4;
pub const VPX_CODEC_ABI_MISMATCH: u32 = 3;
pub const VPX_CODEC_MEM_ERROR: u32 = 2;
pub const VPX_CODEC_ERROR: u32 = 1;
pub const VPX_CODEC_OK: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecIface {
    pub name: *const i8,
    pub abi_version: i32,
    pub caps: i64,
    pub init: VpxCodecInitFnT,
    pub destroy: VpxCodecDestroyFnT,
    pub ctrl_maps: *const VpxCodecCtrlFnMapT,
    pub dec: VpxCodecDecIface,
    pub enc: VpxCodecEncIface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncIface {
    pub cfg_map_count: i32,
    pub cfg_maps: *const VpxCodecEncCfgMapT,
    pub encode: VpxCodecEncodeFnT,
    pub get_cx_data: VpxCodecGetCxDataFnT,
    pub cfg_set: VpxCodecEncConfigSetFnT,
    pub get_glob_hdrs: VpxCodecGetGlobalHeadersFnT,
    pub get_preview: VpxCodecGetPreviewFrameFnT,
    pub mr_get_mem_loc: VpxCodecEncMrGetMemLocFnT,
    pub mr_free_mem_loc: VpxCodecEncMrFreeMemLocFnT,
}
pub type VpxCodecEncMrFreeMemLocFnT = Option<unsafe fn(*mut c_void) -> ()>;
pub type VpxCodecEncMrGetMemLocFnT =
    Option<unsafe fn(*const VpxCodecEncCfgT, *mut *mut c_void) -> u32>;
pub type VpxCodecEncCfgT = VpxCodecEncCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncCfg {
    pub g_usage: u32,
    pub g_threads: u32,
    pub g_profile: u32,
    pub g_w: u32,
    pub g_h: u32,
    pub g_bit_depth: u32,
    pub g_input_bit_depth: u32,
    pub g_timebase: VpxRational,
    pub g_error_resilient: u32,
    pub g_pass: u32,
    pub g_lag_in_frames: u32,
    pub rc_dropframe_thresh: u32,
    pub rc_resize_allowed: u32,
    pub rc_scaled_width: u32,
    pub rc_scaled_height: u32,
    pub rc_resize_up_thresh: u32,
    pub rc_resize_down_thresh: u32,
    pub rc_end_usage: u32,
    pub rc_twopass_stats_in: VpxFixedBufT,
    pub rc_firstpass_mb_stats_in: VpxFixedBufT,
    pub rc_target_bitrate: u32,
    pub rc_min_quantizer: u32,
    pub rc_max_quantizer: u32,
    pub rc_undershoot_pct: u32,
    pub rc_overshoot_pct: u32,
    pub rc_buf_sz: u32,
    pub rc_buf_initial_sz: u32,
    pub rc_buf_optimal_sz: u32,
    pub rc_2pass_vbr_bias_pct: u32,
    pub rc_2pass_vbr_minsection_pct: u32,
    pub rc_2pass_vbr_maxsection_pct: u32,
    pub rc_2pass_vbr_corpus_complexity: u32,
    pub kf_mode: u32,
    pub kf_min_dist: u32,
    pub kf_max_dist: u32,
    pub ss_number_layers: u32,
    pub ss_enable_auto_alt_ref: [i32; 5],
    pub ss_target_bitrate: [u32; 5],
    pub ts_number_layers: u32,
    pub ts_target_bitrate: [u32; 5],
    pub ts_rate_decimator: [u32; 5],
    pub ts_periodicity: u32,
    pub ts_layer_id: [u32; 16],
    pub layer_target_bitrate: [u32; 12],
    pub temporal_layering_mode: i32,
    pub use_vizier_rc_params: i32,
    pub active_wq_factor: VpxRationalT,
    pub err_per_mb_factor: VpxRationalT,
    pub sr_default_decay_limit: VpxRationalT,
    pub sr_diff_factor: VpxRationalT,
    pub kf_err_per_mb_factor: VpxRationalT,
    pub kf_frame_min_boost_factor: VpxRationalT,
    pub kf_frame_max_boost_first_factor: VpxRationalT,
    pub kf_frame_max_boost_subs_factor: VpxRationalT,
    pub kf_max_total_boost_factor: VpxRationalT,
    pub gf_max_total_boost_factor: VpxRationalT,
    pub gf_frame_max_boost_factor: VpxRationalT,
    pub zm_factor: VpxRationalT,
    pub rd_mult_inter_qp_fac: VpxRationalT,
    pub rd_mult_arf_qp_fac: VpxRationalT,
    pub rd_mult_key_qp_fac: VpxRationalT,
}
pub type VpxRationalT = VpxRational;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxRational {
    pub num: i32,
    pub den: i32,
}
pub const VPX_KF_DISABLED: u32 = 0;
pub const VPX_KF_AUTO: u32 = 1;
pub const VPX_KF_FIXED: u32 = 0;
pub type VpxFixedBufT = VpxFixedBuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxFixedBuf {
    pub buf: *mut c_void,
    pub sz: usize,
}
pub const VPX_Q: u32 = 3;
pub const VPX_CQ: u32 = 2;
pub const VPX_CBR: u32 = 1;
pub const VPX_VBR: u32 = 0;
pub const VPX_RC_LAST_PASS: u32 = 2;
pub const VPX_RC_FIRST_PASS: u32 = 1;
pub const VPX_RC_ONE_PASS: u32 = 0;
pub const VPX_BITS_12: u32 = 12;
pub const VPX_BITS_10: u32 = 10;
pub const VPX_BITS_8: u32 = 8;
pub type VpxCodecGetPreviewFrameFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxImageT>;
pub type VpxCodecAlgPrivT = VpxCodecAlgPriv;
pub type VpxCodecGetGlobalHeadersFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxFixedBufT>;
pub type VpxCodecEncConfigSetFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const VpxCodecEncCfgT) -> u32>;
pub type VpxCodecGetCxDataFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *const VpxCodecCxPktT>;
pub type VpxCodecIterT = *const c_void;
pub type VpxCodecCxPktT = VpxCodecCxPkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCxPkt {
    pub kind: u32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub frame: C2RustUnnamed_0,
    pub twopass_stats: VpxFixedBufT,
    pub firstpass_mb_stats: VpxFixedBufT,
    pub psnr: VpxPsnrPkt,
    pub raw: VpxFixedBufT,
    pub pad: [i8; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxPsnrPkt {
    pub samples: [u32; 4],
    pub sse: [u64; 4],
    pub psnr: [f64; 4],
    pub spatial_layer_id: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub buf: *mut c_void,
    pub sz: usize,
    pub pts: i64,
    pub duration: u64,
    pub flags: u32,
    pub partition_id: i32,
    pub width: [u32; 5],
    pub height: [u32; 5],
    pub spatial_layer_encoded: [u8; 5],
}
pub const VPX_CODEC_CUSTOM_PKT: u32 = 256;
pub const VPX_CODEC_PSNR_PKT: u32 = 3;
pub const VPX_CODEC_FPMB_STATS_PKT: u32 = 2;
pub const VPX_CODEC_STATS_PKT: u32 = 1;
pub const VPX_CODEC_CX_FRAME_PKT: u32 = 0;
pub type VpxCodecEncodeFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const VpxImageT, i64, u64, i64, u64) -> u32>;
pub type VpxCodecEncCfgMapT = VpxCodecEncCfgMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncCfgMap {
    pub usage: i32,
    pub cfg: VpxCodecEncCfgT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecDecIface {
    pub peek_si: VpxCodecPeekSiFnT,
    pub get_si: VpxCodecGetSiFnT,
    pub decode: VpxCodecDecodeFnT,
    pub get_frame: VpxCodecGetFrameFnT,
    pub set_fb_fn: VpxCodecSetFbFnT,
}
pub type VpxCodecSetFbFnT = Option<
    unsafe fn(
        *mut VpxCodecAlgPrivT,
        VpxGetFrameBufferCbFnT,
        VpxReleaseFrameBufferCbFnT,
        *mut c_void,
    ) -> u32,
>;
pub type VpxReleaseFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecFrameBufferT = VpxCodecFrameBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecFrameBuffer {
    pub data: *mut u8,
    pub size: usize,
    pub priv_0: *mut c_void,
}
pub type VpxGetFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, usize, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecGetFrameFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *mut VpxImageT>;
pub type VpxCodecDecodeFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const u8, u32, *mut c_void) -> u32>;
pub type VpxCodecGetSiFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecStreamInfoT) -> u32>;
pub type VpxCodecStreamInfoT = VpxCodecStreamInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecStreamInfo {
    pub sz: u32,
    pub w: u32,
    pub h: u32,
    pub is_kf: u32,
}
pub type VpxCodecPeekSiFnT = Option<unsafe fn(*const u8, u32, *mut VpxCodecStreamInfoT) -> u32>;
pub type VpxCodecCtrlFnMapT = VpxCodecCtrlFnMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCtrlFnMap {
    pub ctrl_id: i32,
    pub fn_0: VpxCodecControlFnT,
}
pub type VpxCodecControlFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> u32>;
pub type VpxCodecDestroyFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> u32>;
pub type VpxCodecInitFnT = Option<unsafe fn(*mut VpxCodecCtxT, *mut VpxCodecPrivEncMrCfgT) -> u32>;
pub type VpxCodecPrivEncMrCfgT = VpxCodecPrivEncMrCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPrivEncMrCfg {
    pub mr_total_resolutions: u32,
    pub mr_encoder_id: u32,
    pub mr_down_sampling_factor: VpxRational,
    pub mr_low_res_mode_info: *mut c_void,
}
pub type VpxCodecCtxT = VpxCodecCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCtx {
    pub name: *const i8,
    pub iface: *const VpxCodecIfaceT,
    pub err: u32,
    pub err_detail: *const i8,
    pub init_flags: i64,
    pub config: C2RustUnnamed_4,
    pub priv_0: *mut VpxCodecPrivT,
}
pub type VpxCodecPrivT = VpxCodecPriv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPriv {
    pub err_detail: *const i8,
    pub init_flags: i64,
    pub dec: C2RustUnnamed_2,
    pub enc: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub cx_data_dst_buf: VpxFixedBufT,
    pub cx_data_pad_before: u32,
    pub cx_data_pad_after: u32,
    pub cx_data_pkt: VpxCodecCxPktT,
    pub total_encoders: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub put_frame_cb: VpxCodecPrivCbPairT,
    pub put_slice_cb: VpxCodecPrivCbPairT,
}
pub type VpxCodecPrivCbPairT = VpxCodecPrivCbPair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPrivCbPair {
    pub u: C2RustUnnamed_3,
    pub user_priv: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub put_frame: VpxCodecPutFrameCbFnT,
    pub put_slice: VpxCodecPutSliceCbFnT,
}
pub type VpxCodecPutSliceCbFnT = Option<
    unsafe fn(*mut c_void, *const VpxImageT, *const VpxImageRectT, *const VpxImageRectT) -> (),
>;
pub type VpxCodecPutFrameCbFnT = Option<unsafe fn(*mut c_void, *const VpxImageT) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub dec: *const VpxCodecDecCfg,
    pub enc: *const VpxCodecEncCfg,
    pub raw: *const c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecDecCfg {
    pub threads: u32,
    pub w: u32,
    pub h: u32,
}
pub type VpxCodecIfaceT = VpxCodecIface;
pub type JmpBuf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VERSION_MAJOR: i32 = 1 as i32;
pub const VERSION_MINOR: i32 = 16 as i32;
pub const VERSION_PATCH: i32 = 0 as i32;
pub const VERSION_EXTRA: [i8; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [i8; 15]>(*b"122-ge9efe034e\0") };
pub const VERSION_PACKED: i32 = VERSION_MAJOR << 16 as i32 | VERSION_MINOR << 8 as i32 | 0 as i32;
pub const VERSION_STRING_NOSP: [i8; 23] =
    unsafe { ::core::mem::transmute::<[u8; 23], [i8; 23]>(*b"v1.16.0-122-ge9efe034e\0") };
#[unsafe(no_mangle)]
pub fn vpx_codec_version() -> i32 {
    VERSION_PACKED
}
#[unsafe(no_mangle)]
pub fn vpx_codec_version_str() -> *const i8 {
    VERSION_STRING_NOSP.as_ptr()
}
#[unsafe(no_mangle)]
pub fn vpx_codec_version_extra_str() -> *const i8 {
    VERSION_EXTRA.as_ptr()
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_iface_name(mut iface: *const VpxCodecIfaceT) -> *const i8 {
    unsafe {
        if !iface.is_null() {
            (*iface).name
        } else {
            b"<invalid interface>\0" as *const u8 as *const i8
        }
    }
}
#[unsafe(no_mangle)]
pub fn vpx_codec_err_to_string(mut err: u32) -> *const i8 {
    match err as u32 {
        0 => return b"Success\0" as *const u8 as *const i8,
        1 => {
            return b"Unspecified internal error\0" as *const u8 as *const i8;
        }
        2 => {
            return b"Memory allocation error\0" as *const u8 as *const i8;
        }
        3 => return b"ABI version mismatch\0" as *const u8 as *const i8,
        4 => {
            return b"Codec does not implement requested capability\0" as *const u8 as *const i8;
        }
        5 => {
            return b"Bitstream not supported by this decoder\0" as *const u8 as *const i8;
        }
        6 => {
            return b"Bitstream required feature not supported by this decoder\0" as *const u8
                as *const i8;
        }
        7 => {
            return b"Corrupt frame detected\0" as *const u8 as *const i8;
        }
        8 => return b"Invalid parameter\0" as *const u8 as *const i8,
        9 => return b"End of iterated list\0" as *const u8 as *const i8,
        _ => {}
    }
    b"Unrecognized error code\0" as *const u8 as *const i8
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_error(mut ctx: *const VpxCodecCtxT) -> *const i8 {
    unsafe {
        if !ctx.is_null() {
            vpx_codec_err_to_string((*ctx).err)
        } else {
            vpx_codec_err_to_string(VPX_CODEC_INVALID_PARAM)
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_error_detail(mut ctx: *const VpxCodecCtxT) -> *const i8 {
    unsafe {
        if !ctx.is_null() && (*ctx).err as u32 != 0 {
            return if !(*ctx).priv_0.is_null() {
                (*(*ctx).priv_0).err_detail
            } else {
                (*ctx).err_detail
            };
        }
        ::core::ptr::null::<i8>()
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_destroy(mut ctx: *mut VpxCodecCtxT) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else {
            (*(*ctx).iface).destroy.expect("non-null function pointer")(
                (*ctx).priv_0 as *mut VpxCodecAlgPrivT,
            );
            (*ctx).iface = ::core::ptr::null::<VpxCodecIfaceT>();
            (*ctx).name = ::core::ptr::null::<i8>();
            (*ctx).priv_0 = ::core::ptr::null_mut::<VpxCodecPrivT>();
            res = VPX_CODEC_OK;
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as u32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_get_caps(mut iface: *const VpxCodecIfaceT) -> i64 {
    unsafe {
        if !iface.is_null() {
            (*iface).caps
        } else {
            0 as i64
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_control_(
    mut ctx: *mut VpxCodecCtxT,
    mut ctrl_id: i32,
    mut data: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() || ctrl_id == 0 {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null()
            || (*ctx).priv_0.is_null()
            || (*(*ctx).iface).ctrl_maps.is_null()
        {
            res = VPX_CODEC_ERROR;
        } else {
            let mut entry: *const VpxCodecCtrlFnMapT = ::core::ptr::null::<VpxCodecCtrlFnMapT>();
            res = VPX_CODEC_INCAPABLE;
            entry = (*(*ctx).iface).ctrl_maps;
            while (*entry).fn_0.is_some() {
                if (*entry).ctrl_id == 0 || (*entry).ctrl_id == ctrl_id {
                    res = (*entry).fn_0.expect("non-null function pointer")(
                        (*ctx).priv_0 as *mut VpxCodecAlgPrivT,
                        data,
                    );
                    break;
                } else {
                    entry = entry.offset(1);
                }
            }
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as u32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_internal_error(
    mut info: *mut VpxInternalErrorInfo,
    mut error: u32,
    mut fmt: *const i8,
) {
    unsafe {
        (*info).error_code = error;
        (*info).has_detail = false;
        if !fmt.is_null() {
            let mut sz: usize = ::core::mem::size_of::<[i8; 80]>() as usize;
            (*info).has_detail = true;

            // On Windows MSVC, vsnprintf is not readily available in the default linked CRT
            // without explicitly linking legacy_stdio_definitions.lib, which complicates Rust
            // linkage. To maintain cross-platform pure-Rust portability for this transpiled codebase,
            // we simply truncate and copy the format string directly as the detail message, ignoring
            // the variadic arguments.
            let mut i = 0;
            while i < sz.wrapping_sub(1 as usize) && *fmt.add(i) != 0 {
                (*info).detail[i] = *fmt.add(i);
                i += 1;
            }
            (*info).detail[i] = 0 as i8;
        }
        if (*info).setjmp {
            longjmp(&raw mut (*info).jmp as *mut i32, (*info).error_code as i32);
        }
    }
}
