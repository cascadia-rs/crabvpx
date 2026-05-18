use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_yv12_alloc_frame_buffer(
        ybf: *mut Yv12BufferConfig,
        width: i32,
        height: i32,
        border: i32,
    ) -> i32;
    fn vp8_yv12_de_alloc_frame_buffer(ybf: *mut Yv12BufferConfig) -> i32;
    fn vpx_calloc(num: SizeT, size: SizeT) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
    fn vp8_init_mbmode_probs(x: *mut Vp8Common);
    fn vp8_default_bmode_probs(dest: *mut Vp8Prob);
    fn vp8_machine_specific_config(_: *mut VP8Common);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: BPredictionMode,
    pub mv: IntMv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union IntMv {
    pub as_int: u32,
    pub as_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
}
pub type BPredictionMode = u32;
pub const B_MODE_COUNT: BPredictionMode = 14;
pub const NEW4X4: BPredictionMode = 13;
pub const ZERO4X4: BPredictionMode = 12;
pub const ABOVE4X4: BPredictionMode = 11;
pub const LEFT4X4: BPredictionMode = 10;
pub const B_HU_PRED: BPredictionMode = 9;
pub const B_HD_PRED: BPredictionMode = 8;
pub const B_VL_PRED: BPredictionMode = 7;
pub const B_VR_PRED: BPredictionMode = 6;
pub const B_RD_PRED: BPredictionMode = 5;
pub const B_LD_PRED: BPredictionMode = 4;
pub const B_HE_PRED: BPredictionMode = 3;
pub const B_VE_PRED: BPredictionMode = 2;
pub const B_TM_PRED: BPredictionMode = 1;
pub const B_DC_PRED: BPredictionMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: VpxCodecErrT,
    pub has_detail: i32,
    pub detail: [i8; 80],
    pub setjmp: i32,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
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
pub type Vp8Prob = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntropyContextPlanes {
    pub y1: [EntropyContext; 4],
    pub u: [EntropyContext; 2],
    pub v: [EntropyContext; 2],
    pub y2: EntropyContext,
}
pub type EntropyContext = i8;
pub type FrameType = u32;
pub const INTER_FRAME: FrameType = 1;
pub const KEY_FRAME: FrameType = 0;
pub type ModeInfo = Modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Modeinfo {
    pub mbmi: MbModeInfo,
    pub bmi: [BModeInfo; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbModeInfo {
    pub mode: u8,
    pub uv_mode: u8,
    pub ref_frame: u8,
    pub is_4x4: u8,
    pub mv: IntMv,
    pub partitioning: u8,
    pub mb_skip_coeff: u8,
    pub need_to_clamp_mvs: u8,
    pub segment_id: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: SizeT,
    pub border: i32,
    pub frame_size: SizeT,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: VpxColorSpaceT,
    pub color_range: VpxColorRangeT,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type VpxColorRangeT = VpxColorRange;
pub type VpxColorRange = u32;
pub const VPX_CR_FULL_RANGE: VpxColorRange = 1;
pub const VPX_CR_STUDIO_RANGE: VpxColorRange = 0;
pub type VpxColorSpaceT = VpxColorSpace;
pub type VpxColorSpace = u32;
pub const VPX_CS_SRGB: VpxColorSpace = 7;
pub const VPX_CS_RESERVED: VpxColorSpace = 6;
pub const VPX_CS_BT_2020: VpxColorSpace = 5;
pub const VPX_CS_SMPTE_240: VpxColorSpace = 4;
pub const VPX_CS_SMPTE_170: VpxColorSpace = 3;
pub const VPX_CS_BT_709: VpxColorSpace = 2;
pub const VPX_CS_BT_601: VpxColorSpace = 1;
pub const VPX_CS_UNKNOWN: VpxColorSpace = 0;
pub type SizeT = DarwinSizeT;
pub type DarwinSizeT = usize;
pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterInfoN {
    pub mblim: [[u8; 16]; 64],
    pub blim: [[u8; 16]; 64],
    pub lim: [[u8; 16]; 64],
    pub hev_thr: [[u8; 16]; 4],
    pub lvl: [[[u8; 4]; 4]; 4],
    pub hev_thr_lut: [[u8; 64]; 2],
    pub mode_lf_lut: [u8; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: VpxInternalErrorInfo,
    pub y1dequant: [[i16; 2]; 128],
    pub y2dequant: [[i16; 2]; 128],
    pub uvdequant: [[i16; 2]; 128],
    pub width: i32,
    pub height: i32,
    pub horiz_scale: i32,
    pub vert_scale: i32,
    pub clamp_type: ClampType,
    pub frame_to_show: *mut Yv12BufferConfig,
    pub yv12_fb: [Yv12BufferConfig; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: Yv12BufferConfig,
    pub last_frame_type: FrameType,
    pub frame_type: FrameType,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub mbs: i32,
    pub mb_rows: i32,
    pub mb_cols: i32,
    pub mode_info_stride: i32,
    pub mb_no_coeff_skip: i32,
    pub no_lpf: i32,
    pub use_bilinear_mc_filter: i32,
    pub full_pixel: i32,
    pub base_qindex: i32,
    pub y1dc_delta_q: i32,
    pub y2dc_delta_q: i32,
    pub y2ac_delta_q: i32,
    pub uvdc_delta_q: i32,
    pub uvac_delta_q: i32,
    pub mip: *mut ModeInfo,
    pub mi: *mut ModeInfo,
    pub show_frame_mi: *mut ModeInfo,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: LoopFilterInfoN,
    pub filter_level: i32,
    pub last_sharpness_level: i32,
    pub sharpness_level: i32,
    pub refresh_last_frame: i32,
    pub refresh_golden_frame: i32,
    pub refresh_alt_ref_frame: i32,
    pub copy_buffer_to_gf: i32,
    pub copy_buffer_to_arf: i32,
    pub refresh_entropy_probs: i32,
    pub ref_frame_sign_bias: [i32; 4],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: EntropyContextPlanes,
    pub lfc: FrameContext,
    pub fc: FrameContext,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: TokenPartition,
    pub processor_core_count: i32,
}
pub type TokenPartition = u32;
pub const EIGHT_PARTITION: TokenPartition = 3;
pub const FOUR_PARTITION: TokenPartition = 2;
pub const TWO_PARTITION: TokenPartition = 1;
pub const ONE_PARTITION: TokenPartition = 0;
pub type FrameContext = FrameContexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameContexts {
    pub bmode_prob: [Vp8Prob; 9],
    pub ymode_prob: [Vp8Prob; 4],
    pub uv_mode_prob: [Vp8Prob; 3],
    pub sub_mv_ref_prob: [Vp8Prob; 3],
    pub coef_probs: [[[[Vp8Prob; 11]; 3]; 8]; 4],
    pub mvc: [MvContext; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [Vp8Prob; 19],
}
pub type ClampType = u32;
pub const RECON_CLAMP_NOTREQUIRED: ClampType = 1;
pub const RECON_CLAMP_REQUIRED: ClampType = 0;
pub type Vp8Common = VP8Common;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VP8BORDERINPIXELS: i32 = 32 as i32;
pub const NUM_YV12_BUFFERS: i32 = 4 as i32;
#[unsafe(no_mangle)]
pub unsafe fn vp8_de_alloc_frame_buffers(mut oci: *mut Vp8Common) {
    unsafe {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < NUM_YV12_BUFFERS {
            vp8_yv12_de_alloc_frame_buffer(
                (&raw mut (*oci).yv12_fb as *mut Yv12BufferConfig).offset(i as isize)
                    as *mut Yv12BufferConfig,
            );
            (*oci).fb_idx_ref_cnt[i as usize] = 0 as i32;
            i += 1;
        }
        vp8_yv12_de_alloc_frame_buffer(&raw mut (*oci).temp_scale_frame);
        vpx_free((*oci).above_context as *mut c_void);
        vpx_free((*oci).mip as *mut c_void);
        (*oci).above_context = ::core::ptr::null_mut::<EntropyContextPlanes>();
        (*oci).mip = ::core::ptr::null_mut::<ModeInfo>();
        (*oci).mi = ::core::ptr::null_mut::<ModeInfo>();
        (*oci).show_frame_mi = ::core::ptr::null_mut::<ModeInfo>();
        (*oci).frame_to_show = ::core::ptr::null_mut::<Yv12BufferConfig>();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_alloc_frame_buffers(
    mut oci: *mut Vp8Common,
    mut width: i32,
    mut height: i32,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut i: i32 = 0;
        vp8_de_alloc_frame_buffers(oci);
        if width & 0xf as i32 != 0 as i32 {
            width += 16 as i32 - (width & 0xf as i32);
        }
        if height & 0xf as i32 != 0 as i32 {
            height += 16 as i32 - (height & 0xf as i32);
        }
        i = 0 as i32;
        loop {
            if !(i < NUM_YV12_BUFFERS) {
                current_block = 10886091980245723256;
                break;
            }
            if vp8_yv12_alloc_frame_buffer(
                (&raw mut (*oci).yv12_fb as *mut Yv12BufferConfig).offset(i as isize)
                    as *mut Yv12BufferConfig,
                width,
                height,
                VP8BORDERINPIXELS,
            ) < 0 as i32
            {
                current_block = 15863795849835083362;
                break;
            }
            i += 1;
        }
        if current_block == 10886091980245723256 {
            (*oci).new_fb_idx = 0 as i32;
            (*oci).lst_fb_idx = 1 as i32;
            (*oci).gld_fb_idx = 2 as i32;
            (*oci).alt_fb_idx = 3 as i32;
            (*oci).fb_idx_ref_cnt[0 as usize] = 1 as i32;
            (*oci).fb_idx_ref_cnt[1 as usize] = 1 as i32;
            (*oci).fb_idx_ref_cnt[2 as usize] = 1 as i32;
            (*oci).fb_idx_ref_cnt[3 as usize] = 1 as i32;
            if !(vp8_yv12_alloc_frame_buffer(
                &raw mut (*oci).temp_scale_frame,
                width,
                16 as i32,
                VP8BORDERINPIXELS,
            ) < 0 as i32)
            {
                (*oci).mb_rows = height >> 4 as i32;
                (*oci).mb_cols = width >> 4 as i32;
                (*oci).mbs = (*oci).mb_rows * (*oci).mb_cols;
                (*oci).mode_info_stride = (*oci).mb_cols + 1 as i32;
                (*oci).mip = vpx_calloc(
                    (((*oci).mb_cols + 1 as i32) * ((*oci).mb_rows + 1 as i32)) as SizeT,
                    ::core::mem::size_of::<ModeInfo>() as SizeT,
                ) as *mut ModeInfo;
                if !(*oci).mip.is_null() {
                    (*oci).mi = (*oci)
                        .mip
                        .offset((*oci).mode_info_stride as isize)
                        .offset(1 as isize);
                    (*oci).above_context = vpx_calloc(
                        (::core::mem::size_of::<EntropyContextPlanes>() as SizeT)
                            .wrapping_mul((*oci).mb_cols as SizeT),
                        1 as SizeT,
                    ) as *mut EntropyContextPlanes;
                    if !(*oci).above_context.is_null() {
                        return 0 as i32;
                    }
                }
            }
        }
        vp8_de_alloc_frame_buffers(oci);
        1 as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_version(mut cm: *mut Vp8Common) {
    unsafe {
        match (*cm).version {
            0 => {
                (*cm).no_lpf = 0 as i32;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 0 as i32;
                (*cm).full_pixel = 0 as i32;
            }
            1 => {
                (*cm).no_lpf = 0 as i32;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as i32;
                (*cm).full_pixel = 0 as i32;
            }
            2 => {
                (*cm).no_lpf = 1 as i32;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as i32;
                (*cm).full_pixel = 0 as i32;
            }
            3 => {
                (*cm).no_lpf = 1 as i32;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 1 as i32;
                (*cm).full_pixel = 1 as i32;
            }
            _ => {
                (*cm).no_lpf = 0 as i32;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = 0 as i32;
                (*cm).full_pixel = 0 as i32;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_create_common(mut oci: *mut Vp8Common) {
    unsafe {
        vp8_machine_specific_config(oci as *mut VP8Common);
        vp8_init_mbmode_probs(oci);
        vp8_default_bmode_probs(&raw mut (*oci).fc.bmode_prob as *mut Vp8Prob);
        (*oci).mb_no_coeff_skip = 1 as i32;
        (*oci).no_lpf = 0 as i32;
        (*oci).filter_type = NORMAL_LOOPFILTER;
        (*oci).use_bilinear_mc_filter = 0 as i32;
        (*oci).full_pixel = 0 as i32;
        (*oci).multi_token_partition = ONE_PARTITION;
        (*oci).clamp_type = RECON_CLAMP_REQUIRED;
        core::ptr::write_bytes(
            &raw mut (*oci).ref_frame_sign_bias as *mut i32 as *mut c_void as *mut u8,
            0 as i32 as u8,
            ::core::mem::size_of::<[i32; 4]>() as SizeT,
        );
        (*oci).copy_buffer_to_gf = 0 as i32;
        (*oci).copy_buffer_to_arf = 0 as i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_remove_common(mut oci: *mut Vp8Common) {
    unsafe {
        vp8_de_alloc_frame_buffers(oci);
    }
}
