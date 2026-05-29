use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_yv12_alloc_frame_buffer(
        ybf: *mut Yv12BufferConfig,
        width: i32,
        height: i32,
        border: i32,
    ) -> i32;
    fn vp8_yv12_de_alloc_frame_buffer(ybf: *mut Yv12BufferConfig) -> i32;
    fn vpx_free(memblk: *mut c_void);
    fn vp8_init_mbmode_probs(x: *mut Vp8Common);
    fn vp8_default_bmode_probs(dest: *mut u8);
    fn vp8_machine_specific_config(_: *mut VP8Common);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BModeInfo {
    pub as_mode: u32,
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
pub const B_MODE_COUNT: u32 = 14;
pub const NEW4X4: u32 = 13;
pub const ZERO4X4: u32 = 12;
pub const ABOVE4X4: u32 = 11;
pub const LEFT4X4: u32 = 10;
pub const B_HU_PRED: u32 = 9;
pub const B_HD_PRED: u32 = 8;
pub const B_VL_PRED: u32 = 7;
pub const B_VR_PRED: u32 = 6;
pub const B_RD_PRED: u32 = 5;
pub const B_LD_PRED: u32 = 4;
pub const B_HE_PRED: u32 = 3;
pub const B_VE_PRED: u32 = 2;
pub const B_TM_PRED: u32 = 1;
pub const B_DC_PRED: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpxInternalErrorInfo {
    pub error_code: u32,
    pub has_detail: bool,
    pub detail: [i8; 80],
    pub setjmp: bool,
    pub jmp: JmpBuf,
}
pub type JmpBuf = [i32; 48];
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
pub struct EntropyContextPlanes {
    pub y1: [i8; 4],
    pub u: [i8; 2],
    pub v: [i8; 2],
    pub y2: i8,
}
pub const INTER_FRAME: u32 = 1;
pub const KEY_FRAME: u32 = 0;
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

pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};
pub const SIMPLE_LOOPFILTER: u32 = 1;
pub const NORMAL_LOOPFILTER: u32 = 0;
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
    pub clamp_type: u32,
    pub frame_to_show: *mut Yv12BufferConfig,
    pub yv12_fb: [Yv12BufferConfig; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: Yv12BufferConfig,
    pub last_frame_type: u32,
    pub frame_type: u32,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub mbs: i32,
    pub mb_rows: i32,
    pub mb_cols: i32,
    pub mode_info_stride: i32,
    pub mb_no_coeff_skip: bool,
    pub no_lpf: bool,
    pub use_bilinear_mc_filter: bool,
    pub full_pixel: bool,
    pub base_qindex: i32,
    pub y1dc_delta_q: i32,
    pub y2dc_delta_q: i32,
    pub y2ac_delta_q: i32,
    pub uvdc_delta_q: i32,
    pub uvac_delta_q: i32,
    pub mip: *mut ModeInfo,
    pub mi: *mut ModeInfo,
    pub show_frame_mi: *mut ModeInfo,
    pub filter_type: u32,
    pub lf_info: LoopFilterInfoN,
    pub filter_level: i32,
    pub last_sharpness_level: i32,
    pub sharpness_level: i32,
    pub refresh_last_frame: i32,
    pub refresh_golden_frame: i32,
    pub refresh_alt_ref_frame: i32,
    pub copy_buffer_to_gf: i32,
    pub copy_buffer_to_arf: i32,
    pub refresh_entropy_probs: bool,
    pub ref_frame_sign_bias: [i32; 4],
    pub above_context: *mut EntropyContextPlanes,
    pub left_context: EntropyContextPlanes,
    pub lfc: FrameContext,
    pub fc: FrameContext,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: u32,
    pub processor_core_count: i32,
}
pub const EIGHT_PARTITION: u32 = 3;
pub const FOUR_PARTITION: u32 = 2;
pub const TWO_PARTITION: u32 = 1;
pub const ONE_PARTITION: u32 = 0;
pub type FrameContext = FrameContexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameContexts {
    pub bmode_prob: [u8; 9],
    pub ymode_prob: [u8; 4],
    pub uv_mode_prob: [u8; 3],
    pub sub_mv_ref_prob: [u8; 3],
    pub coef_probs: [[[[u8; 11]; 3]; 8]; 4],
    pub mvc: [MvContext; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [u8; 19],
}
pub const RECON_CLAMP_NOTREQUIRED: u32 = 1;
pub const RECON_CLAMP_REQUIRED: u32 = 0;
pub type Vp8Common = VP8Common;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VP8BORDERINPIXELS: i32 = 32 as i32;
pub const NUM_YV12_BUFFERS: i32 = 4 as i32;
#[unsafe(no_mangle)]
pub unsafe fn vp8_de_alloc_frame_buffers(oci_ptr: *mut Vp8Common) {
    if oci_ptr.is_null() {
        return;
    }
    let oci = unsafe { &mut *oci_ptr };
    let mut i: i32 = 0;
    while i < NUM_YV12_BUFFERS {
        unsafe {
            vp8_yv12_de_alloc_frame_buffer(
                (&raw mut oci.yv12_fb as *mut Yv12BufferConfig).offset(i as isize)
                    as *mut Yv12BufferConfig,
            );
        }
        oci.fb_idx_ref_cnt[i as usize] = 0 as i32;
        i += 1;
    }
    unsafe {
        vp8_yv12_de_alloc_frame_buffer(&raw mut oci.temp_scale_frame);
    }
    if !oci.above_context.is_null() {
        let ac_count = oci.mb_cols as usize;
        unsafe {
            let _ = Vec::from_raw_parts(oci.above_context, 0, ac_count);
        }
    }
    if !oci.mip.is_null() {
        let mip_count = ((oci.mb_cols + 1) * (oci.mb_rows + 1)) as usize;
        unsafe {
            let _ = Vec::from_raw_parts(oci.mip, 0, mip_count);
        }
    }
    oci.above_context = ::core::ptr::null_mut::<EntropyContextPlanes>();
    oci.mip = ::core::ptr::null_mut::<ModeInfo>();
    oci.mi = ::core::ptr::null_mut::<ModeInfo>();
    oci.show_frame_mi = ::core::ptr::null_mut::<ModeInfo>();
    oci.frame_to_show = ::core::ptr::null_mut::<Yv12BufferConfig>();
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_alloc_frame_buffers(
    oci_ptr: *mut Vp8Common,
    mut width: i32,
    mut height: i32,
) -> i32 {
    if oci_ptr.is_null() {
        return 1 as i32;
    }
    let oci = unsafe { &mut *oci_ptr };
    let mut current_block: u64;
    let mut i: i32 = 0;
    unsafe {
        vp8_de_alloc_frame_buffers(oci_ptr);
    }
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
        let alloc_res = unsafe {
            vp8_yv12_alloc_frame_buffer(
                (&raw mut oci.yv12_fb as *mut Yv12BufferConfig).offset(i as isize)
                    as *mut Yv12BufferConfig,
                width,
                height,
                VP8BORDERINPIXELS,
            )
        };
        if alloc_res < 0 as i32 {
            current_block = 15863795849835083362;
            break;
        }
        i += 1;
    }
    if current_block == 10886091980245723256 {
        oci.new_fb_idx = 0 as i32;
        oci.lst_fb_idx = 1 as i32;
        oci.gld_fb_idx = 2 as i32;
        oci.alt_fb_idx = 3 as i32;
        oci.fb_idx_ref_cnt[0 as usize] = 1 as i32;
        oci.fb_idx_ref_cnt[1 as usize] = 1 as i32;
        oci.fb_idx_ref_cnt[2 as usize] = 1 as i32;
        oci.fb_idx_ref_cnt[3 as usize] = 1 as i32;
        let scale_alloc_res = unsafe {
            vp8_yv12_alloc_frame_buffer(
                &raw mut oci.temp_scale_frame,
                width,
                16 as i32,
                VP8BORDERINPIXELS,
            )
        };
        if !(scale_alloc_res < 0 as i32) {
            oci.mb_rows = height >> 4 as i32;
            oci.mb_cols = width >> 4 as i32;
            oci.mbs = oci.mb_rows * oci.mb_cols;
            oci.mode_info_stride = oci.mb_cols + 1 as i32;

            let mip_count = ((oci.mb_cols + 1 as i32) * (oci.mb_rows + 1 as i32)) as usize;
            let mut mip_vec = Vec::<ModeInfo>::with_capacity(mip_count);
            mip_vec.resize(mip_count, unsafe { core::mem::zeroed() });
            oci.mip = mip_vec.as_mut_ptr();
            core::mem::forget(mip_vec);

            if !oci.mip.is_null() {
                unsafe {
                    oci.mi = oci
                        .mip
                        .offset(oci.mode_info_stride as isize)
                        .offset(1 as isize);
                }
                let ac_count = oci.mb_cols as usize;
                let mut ac_vec = Vec::<EntropyContextPlanes>::with_capacity(ac_count);
                ac_vec.resize(ac_count, unsafe { core::mem::zeroed() });
                oci.above_context = ac_vec.as_mut_ptr();
                core::mem::forget(ac_vec);
                if !oci.above_context.is_null() {
                    return 0 as i32;
                }
            }
        }
    }
    unsafe {
        vp8_de_alloc_frame_buffers(oci_ptr);
    }
    1 as i32
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_version(mut cm: *mut Vp8Common) {
    unsafe {
        match (*cm).version {
            0 => {
                (*cm).no_lpf = false;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = false;
                (*cm).full_pixel = false;
            }
            1 => {
                (*cm).no_lpf = false;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = true;
                (*cm).full_pixel = false;
            }
            2 => {
                (*cm).no_lpf = true;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = true;
                (*cm).full_pixel = false;
            }
            3 => {
                (*cm).no_lpf = true;
                (*cm).filter_type = SIMPLE_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = true;
                (*cm).full_pixel = true;
            }
            _ => {
                (*cm).no_lpf = false;
                (*cm).filter_type = NORMAL_LOOPFILTER;
                (*cm).use_bilinear_mc_filter = false;
                (*cm).full_pixel = false;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_create_common(oci_ptr: *mut Vp8Common) {
    if oci_ptr.is_null() {
        return;
    }
    let oci = unsafe { &mut *oci_ptr };
    unsafe {
        vp8_machine_specific_config(oci_ptr as *mut VP8Common);
        vp8_init_mbmode_probs(oci_ptr);
        vp8_default_bmode_probs(&raw mut oci.fc.bmode_prob as *mut u8);
    }
    oci.mb_no_coeff_skip = true;
    oci.no_lpf = false;
    oci.filter_type = NORMAL_LOOPFILTER;
    oci.use_bilinear_mc_filter = false;
    oci.full_pixel = false;
    oci.multi_token_partition = ONE_PARTITION;
    oci.clamp_type = RECON_CLAMP_REQUIRED;
    unsafe {
        core::ptr::write_bytes(
            &raw mut oci.ref_frame_sign_bias as *mut i32 as *mut c_void as *mut u8,
            0 as u8,
            ::core::mem::size_of::<[i32; 4]>() as usize,
        );
    }
    oci.copy_buffer_to_gf = 0 as i32;
    oci.copy_buffer_to_arf = 0 as i32;
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_remove_common(oci_ptr: *mut Vp8Common) {
    unsafe {
        vp8_de_alloc_frame_buffers(oci_ptr);
    }
}
