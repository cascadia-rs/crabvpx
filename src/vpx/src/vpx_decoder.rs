use std::ffi::c_void;
unsafe extern "Rust" {
    pub type VpxCodecAlgPriv;
    fn vpx_codec_destroy(ctx: *mut VpxCodecCtxT) -> u32;
}
pub type BuiltinVaList = *mut i8;
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
pub type VaList = BuiltinVaList;
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
pub type VpxCodecDecCfgT = VpxCodecDecCfg;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VPX_IMAGE_ABI_VERSION: i32 = 5 as i32;
pub const VPX_CODEC_ABI_VERSION: i32 = 4 as i32 + VPX_IMAGE_ABI_VERSION;
pub const VPX_CODEC_CAP_DECODER: i32 = 0x1 as i32;
pub const VPX_DECODER_ABI_VERSION: i32 = 3 as i32 + VPX_CODEC_ABI_VERSION;
pub const VPX_CODEC_CAP_PUT_SLICE: i32 = 0x10000 as i32;
pub const VPX_CODEC_CAP_PUT_FRAME: i32 = 0x20000 as i32;
pub const VPX_CODEC_CAP_POSTPROC: i32 = 0x40000 as i32;
pub const VPX_CODEC_CAP_ERROR_CONCEALMENT: i32 = 0x80000 as i32;
pub const VPX_CODEC_CAP_INPUT_FRAGMENTS: i32 = 0x100000 as i32;
pub const VPX_CODEC_CAP_EXTERNAL_FRAME_BUFFER: i32 = 0x400000 as i32;
pub const VPX_CODEC_USE_POSTPROC: i32 = 0x10000 as i32;
pub const VPX_CODEC_USE_ERROR_CONCEALMENT: i32 = 0x20000 as i32;
pub const VPX_CODEC_USE_INPUT_FRAGMENTS: i32 = 0x40000 as i32;
pub const VPX_CODEC_INTERNAL_ABI_VERSION: i32 = 5 as i32;
unsafe fn get_alg_priv(mut ctx: *mut VpxCodecCtxT) -> *mut VpxCodecAlgPrivT {
    unsafe { (*ctx).priv_0 as *mut VpxCodecAlgPrivT }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_dec_init_ver(
    mut ctx: *mut VpxCodecCtxT,
    mut iface: *const VpxCodecIfaceT,
    mut cfg: *const VpxCodecDecCfgT,
    mut flags: i64,
    mut ver: i32,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ver != VPX_DECODER_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if ctx.is_null() || iface.is_null() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*iface).abi_version != VPX_CODEC_INTERNAL_ABI_VERSION {
            res = VPX_CODEC_ABI_MISMATCH;
        } else if flags & VPX_CODEC_USE_POSTPROC as i64 != 0
            && (*iface).caps & VPX_CODEC_CAP_POSTPROC as i64 == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_ERROR_CONCEALMENT as i64 != 0
            && (*iface).caps & VPX_CODEC_CAP_ERROR_CONCEALMENT as i64 == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else if flags & VPX_CODEC_USE_INPUT_FRAGMENTS as i64 != 0
            && (*iface).caps & VPX_CODEC_CAP_INPUT_FRAGMENTS as i64 == 0
        {
            res = VPX_CODEC_INCAPABLE;
        } else if (*iface).caps & VPX_CODEC_CAP_DECODER as i64 == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            core::ptr::write_bytes(
                ctx as *mut c_void as *mut u8,
                0 as u8,
                ::core::mem::size_of::<VpxCodecCtxT>() as usize,
            );
            (*ctx).iface = iface;
            (*ctx).name = (*iface).name;
            (*ctx).priv_0 = ::core::ptr::null_mut::<VpxCodecPrivT>();
            (*ctx).init_flags = flags;
            (*ctx).config.dec = cfg as *const VpxCodecDecCfg;
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
        }) as u32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_peek_stream_info(
    mut iface: *const VpxCodecIfaceT,
    mut data: *const u8,
    mut data_sz: u32,
    mut si: *mut VpxCodecStreamInfoT,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if iface.is_null()
            || data.is_null()
            || data_sz == 0
            || si.is_null()
            || ((*si).sz as usize) < ::core::mem::size_of::<VpxCodecStreamInfoT>() as usize
        {
            res = VPX_CODEC_INVALID_PARAM;
        } else {
            (*si).w = 0 as u32;
            (*si).h = 0 as u32;
            res = (*iface).dec.peek_si.expect("non-null function pointer")(data, data_sz, si);
        }
        res
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_get_stream_info(
    mut ctx: *mut VpxCodecCtxT,
    mut si: *mut VpxCodecStreamInfoT,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null()
            || si.is_null()
            || ((*si).sz as usize) < ::core::mem::size_of::<VpxCodecStreamInfoT>() as usize
        {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else {
            (*si).w = 0 as u32;
            (*si).h = 0 as u32;
            res = (*(*ctx).iface)
                .dec
                .get_si
                .expect("non-null function pointer")(get_alg_priv(ctx), si);
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
pub unsafe fn vpx_codec_decode(
    mut ctx: *mut VpxCodecCtxT,
    mut data: *const u8,
    mut data_sz: u32,
    mut user_priv: *mut c_void,
    _deadline: i64,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() || data.is_null() && data_sz != 0 || !data.is_null() && data_sz == 0 {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else {
            res = (*(*ctx).iface)
                .dec
                .decode
                .expect("non-null function pointer")(
                get_alg_priv(ctx), data, data_sz, user_priv
            );
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
pub unsafe fn vpx_codec_get_frame(
    mut ctx: *mut VpxCodecCtxT,
    mut iter: *mut VpxCodecIterT,
) -> *mut VpxImageT {
    unsafe {
        let mut img: *mut VpxImageT = ::core::ptr::null_mut::<VpxImageT>();
        if ctx.is_null() || iter.is_null() || (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            img = ::core::ptr::null_mut::<VpxImageT>();
        } else {
            img = (*(*ctx).iface)
                .dec
                .get_frame
                .expect("non-null function pointer")(get_alg_priv(ctx), iter);
        }
        img
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_codec_register_put_frame_cb(
    mut ctx: *mut VpxCodecCtxT,
    mut cb: VpxCodecPutFrameCbFnT,
    mut user_priv: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() || cb.is_none() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_PUT_FRAME as i64 == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            (*(*ctx).priv_0).dec.put_frame_cb.u.put_frame = cb;
            (*(*ctx).priv_0).dec.put_frame_cb.user_priv = user_priv;
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
pub unsafe fn vpx_codec_register_put_slice_cb(
    mut ctx: *mut VpxCodecCtxT,
    mut cb: VpxCodecPutSliceCbFnT,
    mut user_priv: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() || cb.is_none() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_PUT_SLICE as i64 == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            (*(*ctx).priv_0).dec.put_slice_cb.u.put_slice = cb;
            (*(*ctx).priv_0).dec.put_slice_cb.user_priv = user_priv;
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
pub unsafe fn vpx_codec_set_frame_buffer_functions(
    mut ctx: *mut VpxCodecCtxT,
    mut cb_get: VpxGetFrameBufferCbFnT,
    mut cb_release: VpxReleaseFrameBufferCbFnT,
    mut cb_priv: *mut c_void,
) -> u32 {
    unsafe {
        let mut res: u32 = VPX_CODEC_OK;
        if ctx.is_null() || cb_get.is_none() || cb_release.is_none() {
            res = VPX_CODEC_INVALID_PARAM;
        } else if (*ctx).iface.is_null() || (*ctx).priv_0.is_null() {
            res = VPX_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & VPX_CODEC_CAP_EXTERNAL_FRAME_BUFFER as i64 == 0 {
            res = VPX_CODEC_INCAPABLE;
        } else {
            res = (*(*ctx).iface)
                .dec
                .set_fb_fn
                .expect("non-null function pointer")(
                get_alg_priv(ctx),
                cb_get,
                cb_release,
                cb_priv,
            );
        }
        (if !ctx.is_null() {
            (*ctx).err = res;
            (*ctx).err as u32
        } else {
            res as u32
        }) as u32
    }
}
