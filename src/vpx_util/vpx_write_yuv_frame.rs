use std::ffi::c_void;
unsafe extern "Rust" {
    pub type Sfilex;
}
pub type Int64T = i64;
pub type DarwinSizeT = usize;
pub type DarwinOffT = Int64T;
pub type SizeT = DarwinSizeT;
pub type FposT = DarwinOffT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sbuf {
    pub _base: *mut u8,
    pub _size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sfile {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _bf: Sbuf,
    pub _lbfsize: i32,
    pub _cookie: *mut c_void,
    pub _close: Option<unsafe fn(*mut c_void) -> i32>,
    pub _read: Option<unsafe fn(*mut c_void, *mut i8, i32) -> i32>,
    pub _seek: Option<unsafe fn(*mut c_void, FposT, i32) -> FposT>,
    pub _write: Option<unsafe fn(*mut c_void, *const i8, i32) -> i32>,
    pub _ub: Sbuf,
    pub _extra: *mut Sfilex,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: Sbuf,
    pub _blksize: i32,
    pub _offset: FposT,
}
pub type FILE = Sfile;
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
#[unsafe(no_mangle)]
pub unsafe fn vpx_write_yuv_frame(_yuv_file: *mut FILE, _s: *mut Yv12BufferConfig) {}
