use std::ffi::c_void;
unsafe extern "Rust" {
    pub type VpxCodecAlgPriv;
    fn vpx_codec_destroy(ctx: *mut VpxCodecCtxT) -> VpxCodecErrT;
}
pub type BuiltinVaList = *mut i8;
pub type DarwinSizeT = usize;
pub type SizeT = DarwinSizeT;
pub type VpxImgFmt = u32;
pub const VPX_IMG_FMT_I44016: VpxImgFmt = 2311;
pub const VPX_IMG_FMT_I44416: VpxImgFmt = 2310;
pub const VPX_IMG_FMT_I42216: VpxImgFmt = 2309;
pub const VPX_IMG_FMT_I42016: VpxImgFmt = 2306;
pub const VPX_IMG_FMT_NV12: VpxImgFmt = 265;
pub const VPX_IMG_FMT_I440: VpxImgFmt = 263;
pub const VPX_IMG_FMT_I444: VpxImgFmt = 262;
pub const VPX_IMG_FMT_I422: VpxImgFmt = 261;
pub const VPX_IMG_FMT_I420: VpxImgFmt = 258;
pub const VPX_IMG_FMT_YV12: VpxImgFmt = 769;
pub const VPX_IMG_FMT_NONE: VpxImgFmt = 0;
pub type VpxImgFmtT = VpxImgFmt;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorRangeT = VpxColorRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxImage {
    pub fmt: VpxImgFmtT,
    pub cs: VpxColorSpaceT,
    pub range: VpxColorRangeT,
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
    pub self_allocd: i32,
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
pub type VpxCodecCapsT = i64;
pub type VpxCodecFlagsT = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecIface {
    pub name: *const i8,
    pub abi_version: i32,
    pub caps: VpxCodecCapsT,
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
    Option<unsafe fn(*const VpxCodecEncCfgT, *mut *mut c_void) -> VpxCodecErrT>;
pub type VpxCodecEncCfgT = VpxCodecEncCfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecEncCfg {
    pub g_usage: u32,
    pub g_threads: u32,
    pub g_profile: u32,
    pub g_w: u32,
    pub g_h: u32,
    pub g_bit_depth: VpxBitDepthT,
    pub g_input_bit_depth: u32,
    pub g_timebase: VpxRational,
    pub g_error_resilient: VpxCodecErFlagsT,
    pub g_pass: VpxEncPass,
    pub g_lag_in_frames: u32,
    pub rc_dropframe_thresh: u32,
    pub rc_resize_allowed: u32,
    pub rc_scaled_width: u32,
    pub rc_scaled_height: u32,
    pub rc_resize_up_thresh: u32,
    pub rc_resize_down_thresh: u32,
    pub rc_end_usage: VpxRcMode,
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
    pub kf_mode: VpxKfMode,
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
pub type VpxKfMode = u32;
pub const VPX_KF_DISABLED: VpxKfMode = 0;
pub const VPX_KF_AUTO: VpxKfMode = 1;
pub const VPX_KF_FIXED: VpxKfMode = 0;
pub type VpxFixedBufT = VpxFixedBuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxFixedBuf {
    pub buf: *mut c_void,
    pub sz: SizeT,
}
pub type VpxRcMode = u32;
pub const VPX_Q: VpxRcMode = 3;
pub const VPX_CQ: VpxRcMode = 2;
pub const VPX_CBR: VpxRcMode = 1;
pub const VPX_VBR: VpxRcMode = 0;
pub type VpxEncPass = u32;
pub const VPX_RC_LAST_PASS: VpxEncPass = 2;
pub const VPX_RC_FIRST_PASS: VpxEncPass = 1;
pub const VPX_RC_ONE_PASS: VpxEncPass = 0;
pub type VpxCodecErFlagsT = u32;
pub type VpxBitDepthT = VpxBitDepth;
pub type VpxBitDepth = u32;
pub const VPX_BITS_12: VpxBitDepth = 12;
pub const VPX_BITS_10: VpxBitDepth = 10;
pub const VPX_BITS_8: VpxBitDepth = 8;
pub type VpxCodecGetPreviewFrameFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxImageT>;
pub type VpxCodecAlgPrivT = VpxCodecAlgPriv;
pub type VpxCodecGetGlobalHeadersFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT) -> *mut VpxFixedBufT>;
pub type VpxCodecEncConfigSetFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const VpxCodecEncCfgT) -> VpxCodecErrT>;
pub type VpxCodecGetCxDataFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *const VpxCodecCxPktT>;
pub type VpxCodecIterT = *const c_void;
pub type VpxCodecCxPktT = VpxCodecCxPkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCxPkt {
    pub kind: VpxCodecCxPktKind,
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
    pub sz: SizeT,
    pub pts: VpxCodecPtsT,
    pub duration: u64,
    pub flags: VpxCodecFrameFlagsT,
    pub partition_id: i32,
    pub width: [u32; 5],
    pub height: [u32; 5],
    pub spatial_layer_encoded: [u8; 5],
}
pub type VpxCodecFrameFlagsT = u32;
pub type VpxCodecPtsT = i64;
pub type VpxCodecCxPktKind = u32;
pub const VPX_CODEC_CUSTOM_PKT: VpxCodecCxPktKind = 256;
pub const VPX_CODEC_PSNR_PKT: VpxCodecCxPktKind = 3;
pub const VPX_CODEC_FPMB_STATS_PKT: VpxCodecCxPktKind = 2;
pub const VPX_CODEC_STATS_PKT: VpxCodecCxPktKind = 1;
pub const VPX_CODEC_CX_FRAME_PKT: VpxCodecCxPktKind = 0;
pub type VpxCodecEncodeFnT = Option<
    unsafe fn(
        *mut VpxCodecAlgPrivT,
        *const VpxImageT,
        VpxCodecPtsT,
        u64,
        VpxEncFrameFlagsT,
        VpxEncDeadlineT,
    ) -> VpxCodecErrT,
>;
pub type VpxEncDeadlineT = u64;
pub type VpxEncFrameFlagsT = i64;
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
    ) -> VpxCodecErrT,
>;
pub type VpxReleaseFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecFrameBufferT = VpxCodecFrameBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecFrameBuffer {
    pub data: *mut u8,
    pub size: SizeT,
    pub priv_0: *mut c_void,
}
pub type VpxGetFrameBufferCbFnT =
    Option<unsafe fn(*mut c_void, SizeT, *mut VpxCodecFrameBufferT) -> i32>;
pub type VpxCodecGetFrameFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecIterT) -> *mut VpxImageT>;
pub type VpxCodecDecodeFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *const u8, u32, *mut c_void) -> VpxCodecErrT>;
pub type VpxCodecGetSiFnT =
    Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut VpxCodecStreamInfoT) -> VpxCodecErrT>;
pub type VpxCodecStreamInfoT = VpxCodecStreamInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecStreamInfo {
    pub sz: u32,
    pub w: u32,
    pub h: u32,
    pub is_kf: u32,
}
pub type VpxCodecPeekSiFnT =
    Option<unsafe fn(*const u8, u32, *mut VpxCodecStreamInfoT) -> VpxCodecErrT>;
pub type VpxCodecCtrlFnMapT = VpxCodecCtrlFnMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecCtrlFnMap {
    pub ctrl_id: i32,
    pub fn_0: VpxCodecControlFnT,
}
pub type VpxCodecControlFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT, *mut c_void) -> VpxCodecErrT>;
pub type VaList = BuiltinVaList;
pub type VpxCodecDestroyFnT = Option<unsafe fn(*mut VpxCodecAlgPrivT) -> VpxCodecErrT>;
pub type VpxCodecInitFnT =
    Option<unsafe fn(*mut VpxCodecCtxT, *mut VpxCodecPrivEncMrCfgT) -> VpxCodecErrT>;
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
    pub err: VpxCodecErrT,
    pub err_detail: *const i8,
    pub init_flags: VpxCodecFlagsT,
    pub config: C2RustUnnamed_4,
    pub priv_0: *mut VpxCodecPrivT,
}
pub type VpxCodecPrivT = VpxCodecPriv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPriv {
    pub err_detail: *const i8,
    pub init_flags: VpxCodecFlagsT,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxCodecPktList {
    pub cnt: u32,
    pub max: u32,
    pub pkts: [VpxCodecCxPkt; 1],
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const UINT32_MAX: u32 = 4294967295 as u32;
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VPX_IMAGE_ABI_VERSION: i32 = 5 as i32;
pub const VPX_CODEC_ABI_VERSION: i32 = 4 as i32 + VPX_IMAGE_ABI_VERSION;
pub const VPX_CODEC_CAP_ENCODER: i32 = 0x2 as i32;
pub const VPX_TPL_ABI_VERSION: i32 = 5 as i32;
pub const VPX_EXT_RATECTRL_ABI_VERSION: i32 = 7 as i32 + VPX_TPL_ABI_VERSION;
pub const VPX_ENCODER_ABI_VERSION: i32 =
    18 as i32 + VPX_CODEC_ABI_VERSION + VPX_EXT_RATECTRL_ABI_VERSION;
pub const VPX_CODEC_CAP_PSNR: i32 = 0x10000 as i32;
pub const VPX_CODEC_CAP_OUTPUT_PARTITION: i32 = 0x20000 as i32;
pub const VPX_CODEC_USE_PSNR: i32 = 0x10000 as i32;
pub const VPX_CODEC_USE_OUTPUT_PARTITION: i32 = 0x20000 as i32;
pub const VPX_CODEC_INTERNAL_ABI_VERSION: i32 = 5 as i32;
unsafe fn get_alg_priv(mut ctx: *mut VpxCodecCtxT) -> *mut VpxCodecAlgPrivT {
    unsafe { (*ctx).priv_0 as *mut VpxCodecAlgPrivT }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_enc_init_ver(
    mut ctx: *mut VpxCodecCtxT,
    mut iface: *const VpxCodecIfaceT,
    mut cfg: *const VpxCodecEncCfgT,
    mut flags: VpxCodecFlagsT,
    mut ver: i32,
) -> VpxCodecErrT {
    unsafe {
        let mut res: VpxCodecErrT = VPX_CODEC_OK;
        if ver != VPX_ENCODER_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if ctx.is_null() || iface.is_null() || cfg.is_null() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*iface).abi_version != VPX_CODEC_INTERNAL_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_PSNR as VpxCodecFlagsT != 0
            && (*iface).caps & VPX_CODEC_CAP_PSNR as VpxCodecCapsT == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_OUTPUT_PARTITION as VpxCodecFlagsT != 0
            && (*iface).caps & VPX_CODEC_CAP_OUTPUT_PARTITION as VpxCodecCapsT == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else {
            (*ctx).iface = iface;
            (*ctx).name = (*iface).name;
            (*ctx).priv_0 = ::core::ptr::null_mut::<VpxCodecPrivT>();
            (*ctx).init_flags = flags;
            (*ctx).config.enc = cfg as *const VpxCodecEncCfg;
            res = (*(*ctx).iface).init.expect("non-null function pointer")(
                ctx,
                ::core::ptr::null_mut::<VpxCodecPrivEncMrCfgT>(),
            );
            if res as u64 != 0 {
                (*ctx).err_detail = if !(*ctx).priv_0.is_null() {
                    (*(*ctx).priv_0).err_detail
                } else {
                    ::core::ptr::null::<i8>()
                };
                vpx_codec_destroy(ctx);
            }
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as VpxCodecErrT
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_enc_init_multi_ver(
    mut ctx: *mut VpxCodecCtxT,
    mut iface: *const VpxCodecIfaceT,
    mut cfg: *const VpxCodecEncCfgT,
    mut num_enc: i32,
    mut flags: VpxCodecFlagsT,
    mut dsf: *const VpxRationalT,
    mut ver: i32,
) -> VpxCodecErrT {
    unsafe {
        let mut res: VpxCodecErrT = VPX_CODEC_OK;
        if ver != VPX_ENCODER_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if ctx.is_null()
            || iface.is_null()
            || cfg.is_null()
            || (num_enc > 16 as i32 || num_enc < 1 as i32)
            || dsf.is_null()
        {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*iface).abi_version != VPX_CODEC_INTERNAL_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_PSNR as VpxCodecFlagsT != 0
            && (*iface).caps & VPX_CODEC_CAP_PSNR as VpxCodecCapsT == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_OUTPUT_PARTITION as VpxCodecFlagsT != 0
            && (*iface).caps & VPX_CODEC_CAP_OUTPUT_PARTITION as VpxCodecCapsT == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else {
            let mut i: i32 = 0;
            let mut mem_loc: *mut c_void = ::core::ptr::null_mut::<c_void>();
            if (*iface).enc.mr_get_mem_loc.is_none() {
                return VPX_CODEC_INCAPABLE;
            }
            res = (*iface)
                .enc
                .mr_get_mem_loc
                .expect("non-null function pointer")(cfg, &raw mut mem_loc);
            if res as u64 == 0 {
                i = 0 as i32;
                while i < num_enc {
                    let mut mr_cfg: VpxCodecPrivEncMrCfgT = VpxCodecPrivEncMrCfg {
                        mr_total_resolutions: 0,
                        mr_encoder_id: 0,
                        mr_down_sampling_factor: VpxRational { num: 0, den: 0 },
                        mr_low_res_mode_info: ::core::ptr::null_mut::<c_void>(),
                    };
                    if (*dsf).num < 1 as i32
                        || (*dsf).num > 4096 as i32
                        || (*dsf).den < 1 as i32
                        || (*dsf).den > (*dsf).num
                    {
                        res = VPX_CODEC_INVALID_PARAM;
                    } else {
                        mr_cfg.mr_low_res_mode_info = mem_loc;
                        mr_cfg.mr_total_resolutions = num_enc as u32;
                        mr_cfg.mr_encoder_id = (num_enc - 1 as i32 - i) as u32;
                        mr_cfg.mr_down_sampling_factor = *dsf as VpxRational;
                        (*ctx).iface = iface;
                        (*ctx).name = (*iface).name;
                        (*ctx).priv_0 = ::core::ptr::null_mut::<VpxCodecPrivT>();
                        (*ctx).init_flags = flags;
                        (*ctx).config.enc = cfg as *const VpxCodecEncCfg;
                        res = (*(*ctx).iface).init.expect("non-null function pointer")(
                            ctx,
                            &raw mut mr_cfg,
                        );
                    }
                    if res as u64 != 0 {
                        let mut error_detail: *const i8 = if !(*ctx).priv_0.is_null() {
                            (*(*ctx).priv_0).err_detail
                        } else {
                            ::core::ptr::null::<i8>()
                        };
                        (*ctx).err_detail = error_detail;
                        vpx_codec_destroy(ctx);
                        while i != 0 {
                            ctx = ctx.offset(-1);
                            (*ctx).err_detail = error_detail;
                            vpx_codec_destroy(ctx);
                            i -= 1;
                        }
                        return (if !ctx.is_null() {
                            (*ctx).err = res;
                            (*ctx).err as u32
                        } else {
                            res as u32
                        }) as VpxCodecErrT;
                    }
                    ctx = ctx.offset(1);
                    cfg = cfg.offset(1);
                    dsf = dsf.offset(1);
                    i += 1;
                }
                ctx = ctx.offset(-1);
            }
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as VpxCodecErrT
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_enc_config_default(
    mut iface: *const VpxCodecIfaceT,
    mut cfg: *mut VpxCodecEncCfgT,
    mut usage: u32,
) -> VpxCodecErrT {
    unsafe {
        let mut res: VpxCodecErrT = VPX_CODEC_OK;
        if iface.is_null() || cfg.is_null() || usage != 0 as u32 {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            *cfg = (*(*iface).enc.cfg_maps).cfg;
            res = VPX_CODEC_OK;
        }
        res
    }
}
unsafe fn floating_point_init() {}
unsafe fn floating_point_restore() {}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_encode(
    mut ctx: *mut VpxCodecCtxT,
    mut img: *const VpxImageT,
    mut pts: VpxCodecPtsT,
    mut duration: u64,
    mut flags: VpxEncFrameFlagsT,
    mut deadline: VpxEncDeadlineT,
) -> VpxCodecErrT {
    unsafe {
        let mut res: VpxCodecErrT = VPX_CODEC_OK;
        if ctx.is_null() || !img.is_null() && duration == 0 {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else if duration > UINT32_MAX as u64 || deadline > UINT32_MAX as VpxEncDeadlineT {
            res = VPX_CODEC_INVALID_PARAM;
        } else {
            let mut num_enc: u32 = (*(*ctx).priv_0).enc.total_encoders;
            floating_point_init();
            if num_enc == 1 as u32 {
                res = (*(*ctx).iface)
                    .enc
                    .encode
                    .expect("non-null function pointer")(
                    get_alg_priv(ctx),
                    img,
                    pts,
                    duration,
                    flags,
                    deadline,
                );
            } else {
                let mut i: i32 = 0;
                ctx = ctx.offset(num_enc.wrapping_sub(1 as u32) as isize);
                if !img.is_null() {
                    img = img.offset(num_enc.wrapping_sub(1 as u32) as isize);
                }
                i = num_enc.wrapping_sub(1 as u32) as i32;
                while i >= 0 as i32 {
                    res = (*(*ctx).iface)
                        .enc
                        .encode
                        .expect("non-null function pointer")(
                        get_alg_priv(ctx),
                        img,
                        pts,
                        duration,
                        flags,
                        deadline,
                    );
                    if res as u64 != 0 {
                        break;
                    }
                    ctx = ctx.offset(-1);
                    if !img.is_null() {
                        img = img.offset(-1);
                    }
                    i -= 1;
                }
                ctx = ctx.offset(1);
            }
            floating_point_restore();
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as VpxCodecErrT
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_get_cx_data(
    mut ctx: *mut VpxCodecCtxT,
    mut iter: *mut VpxCodecIterT,
) -> *const VpxCodecCxPktT {
    unsafe {
        let mut pkt: *const VpxCodecCxPktT = ::core::ptr::null::<VpxCodecCxPktT>();
        if !ctx.is_null() {
            if iter.is_null() {
                (*ctx).err = VPX_CODEC_INVALID_PARAM;
            } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
                (*ctx).err = VPX_CODEC_ERROR;
            } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
                (*ctx).err = VPX_CODEC_INCAPABLE;
            } else {
                pkt = (*(*ctx).iface)
                    .enc
                    .get_cx_data
                    .expect("non-null function pointer")(
                    get_alg_priv(ctx), iter
                );
            }
        }
        if !pkt.is_null() && (*pkt).kind as u32 == VPX_CODEC_CX_FRAME_PKT as u32 {
            let priv_0: *mut VpxCodecPrivT = (*ctx).priv_0;
            let dst_buf: *mut i8 = (*priv_0).enc.cx_data_dst_buf.buf as *mut i8;
            if !dst_buf.is_null()
                && (*pkt).data.raw.buf != dst_buf as *mut c_void
                && (*pkt)
                    .data
                    .raw
                    .sz
                    .wrapping_add((*priv_0).enc.cx_data_pad_before as SizeT)
                    .wrapping_add((*priv_0).enc.cx_data_pad_after as SizeT)
                    <= (*priv_0).enc.cx_data_dst_buf.sz
            {
                let mut modified_pkt: *mut VpxCodecCxPktT = &raw mut (*priv_0).enc.cx_data_pkt;
                core::ptr::copy_nonoverlapping(
                    (*pkt).data.raw.buf as *const u8,
                    dst_buf.offset((*priv_0).enc.cx_data_pad_before as isize) as *mut c_void
                        as *mut u8,
                    (*pkt).data.raw.sz,
                );
                *modified_pkt = *pkt;
                (*modified_pkt).data.raw.buf = dst_buf as *mut c_void;
                (*modified_pkt).data.raw.sz = (*modified_pkt).data.raw.sz.wrapping_add(
                    (*priv_0)
                        .enc
                        .cx_data_pad_before
                        .wrapping_add((*priv_0).enc.cx_data_pad_after) as SizeT,
                );
                pkt = modified_pkt;
            }
            if dst_buf == (*pkt).data.raw.buf as *mut i8 {
                (*priv_0).enc.cx_data_dst_buf.buf = dst_buf.add((*pkt).data.raw.sz) as *mut c_void;
                (*priv_0).enc.cx_data_dst_buf.sz = (*priv_0)
                    .enc
                    .cx_data_dst_buf
                    .sz
                    .wrapping_sub((*pkt).data.raw.sz);
            }
        }
        pkt
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_set_cx_data_buf(
    mut ctx: *mut VpxCodecCtxT,
    mut buf: *const VpxFixedBufT,
    mut pad_before: u32,
    mut pad_after: u32,
) -> VpxCodecErrT {
    unsafe {
        if ctx.is_null() || (*ctx).priv_0.is_null() {
            return VPX_CODEC_INVALID_PARAM;
        }
        if !buf.is_null() {
            (*(*ctx).priv_0).enc.cx_data_dst_buf = *buf;
            (*(*ctx).priv_0).enc.cx_data_pad_before = pad_before;
            (*(*ctx).priv_0).enc.cx_data_pad_after = pad_after;
        } else {
            (*(*ctx).priv_0).enc.cx_data_dst_buf.buf = NULL;
            (*(*ctx).priv_0).enc.cx_data_dst_buf.sz = 0 as SizeT;
            (*(*ctx).priv_0).enc.cx_data_pad_before = 0 as u32;
            (*(*ctx).priv_0).enc.cx_data_pad_after = 0 as u32;
        }
        VPX_CODEC_OK
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_get_preview_frame(mut ctx: *mut VpxCodecCtxT) -> *const VpxImageT {
    unsafe {
        let mut img: *mut VpxImageT = ::core::ptr::null_mut::<VpxImageT>();
        if !ctx.is_null() {
            if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
                (*ctx).err = VPX_CODEC_ERROR;
            } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
                (*ctx).err = VPX_CODEC_INCAPABLE;
            } else if (*(*ctx).iface).enc.get_preview.is_none() {
                (*ctx).err = VPX_CODEC_INCAPABLE;
            } else {
                img = (*(*ctx).iface)
                    .enc
                    .get_preview
                    .expect("non-null function pointer")(get_alg_priv(ctx));
            }
        }
        img
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_get_global_headers(mut ctx: *mut VpxCodecCtxT) -> *mut VpxFixedBufT {
    unsafe {
        let mut buf: *mut VpxFixedBufT = ::core::ptr::null_mut::<VpxFixedBufT>();
        if !ctx.is_null() {
            if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
                (*ctx).err = VPX_CODEC_ERROR;
            } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
                (*ctx).err = VPX_CODEC_INCAPABLE;
            } else if (*(*ctx).iface).enc.get_glob_hdrs.is_none() {
                (*ctx).err = VPX_CODEC_INCAPABLE;
            } else {
                buf = (*(*ctx).iface)
                    .enc
                    .get_glob_hdrs
                    .expect("non-null function pointer")(get_alg_priv(ctx));
            }
        }
        buf
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_enc_config_set(
    mut ctx: *mut VpxCodecCtxT,
    mut cfg: *const VpxCodecEncCfgT,
) -> VpxCodecErrT {
    unsafe {
        let mut res: VpxCodecErrT = VPX_CODEC_OK;
        if ctx.is_null() || (*ctx).iface.is_null() || (*ctx).priv_0.is_null() || cfg.is_null() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_ENCODER as VpxCodecCapsT == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            res = (*(*ctx).iface)
                .enc
                .cfg_set
                .expect("non-null function pointer")(get_alg_priv(ctx), cfg);
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as VpxCodecErrT
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_pkt_list_add(
    mut list: *mut VpxCodecPktList,
    mut pkt: *const VpxCodecCxPkt,
) -> i32 {
    unsafe {
        if (*list).cnt < (*list).max {
            let fresh0 = (*list).cnt;
            (*list).cnt = (*list).cnt.wrapping_add(1);
            *(&raw mut (*list).pkts as *mut VpxCodecCxPkt).offset(fresh0 as isize) = *pkt;
            return 0 as i32;
        }
        1 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_pkt_list_get(
    mut list: *mut VpxCodecPktList,
    mut iter: *mut VpxCodecIterT,
) -> *const VpxCodecCxPktT {
    unsafe {
        let mut pkt: *const VpxCodecCxPktT = ::core::ptr::null::<VpxCodecCxPktT>();
        if (*iter).is_null() {
            *iter = &raw mut (*list).pkts as *mut VpxCodecCxPkt as VpxCodecIterT;
        }
        pkt = *iter as *const VpxCodecCxPktT;
        if (pkt.offset_from(&raw mut (*list).pkts as *mut VpxCodecCxPkt) as SizeT)
            < (*list).cnt as SizeT
        {
            *iter = pkt.offset(1 as isize) as VpxCodecIterT;
        } else {
            pkt = ::core::ptr::null::<VpxCodecCxPktT>();
        }
        pkt
    }
}
