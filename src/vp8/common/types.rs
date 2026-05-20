pub type uint32_t = u32;
pub type uint8_t = u8;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct int_mv {
    pub as_mv: MV,
}
impl int_mv {
    #[inline]
    pub fn as_int(&self) -> uint32_t {
        let row_u16 = self.as_mv.row as u16;
        let col_u16 = self.as_mv.col as u16;
        ((col_u16 as u32) << 16) | (row_u16 as u32)
    }
    #[inline]
    pub fn set_as_int(&mut self, val: uint32_t) {
        self.as_mv.row = (val & 0xffff) as i16;
        self.as_mv.col = ((val >> 16) & 0xffff) as i16;
    }
    #[inline]
    pub fn as_mv(&self) -> MV {
        self.as_mv
    }
    #[inline]
    pub fn as_mv_mut(&mut self) -> &mut MV {
        &mut self.as_mv
    }
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

impl Default for ENTROPY_CONTEXT_PLANES {
    #[inline]
    fn default() -> Self {
        ENTROPY_CONTEXT_PLANES {
            y1: [0; 4],
            u: [0; 2],
            v: [0; 2],
            y2: 0,
        }
    }
}

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

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct b_mode_info {
    pub mv: int_mv,
}
impl b_mode_info {
    #[inline]
    pub fn mode(&self) -> B_PREDICTION_MODE {
        self.mv.as_int()
    }
    #[inline]
    pub fn set_mode(&mut self, mode: B_PREDICTION_MODE) {
        self.mv.set_as_int(mode);
    }
    #[inline]
    pub fn mv(&self) -> int_mv {
        self.mv
    }
    #[inline]
    pub fn mv_mut(&mut self) -> &mut int_mv {
        &mut self.mv
    }
}

pub type MB_PREDICTION_MODE = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: MB_PREDICTION_MODE = 10;
pub const SPLITMV: MB_PREDICTION_MODE = 9;
pub const NEWMV: MB_PREDICTION_MODE = 8;
pub const ZEROMV: MB_PREDICTION_MODE = 7;
pub const NEARMV: MB_PREDICTION_MODE = 6;
pub const NEARESTMV: MB_PREDICTION_MODE = 5;
pub const B_PRED: MB_PREDICTION_MODE = 4;
pub const TM_PRED: MB_PREDICTION_MODE = 3;
pub const H_PRED: MB_PREDICTION_MODE = 2;
pub const V_PRED: MB_PREDICTION_MODE = 1;
pub const DC_PRED: MB_PREDICTION_MODE = 0;

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

impl Default for MB_MODE_INFO {
    #[inline]
    fn default() -> Self {
        MB_MODE_INFO {
            mode: 0,
            uv_mode: 0,
            ref_frame: 0,
            is_4x4: 0,
            mv: int_mv::default(),
            partitioning: 0,
            mb_skip_coeff: 0,
            need_to_clamp_mvs: 0,
            segment_id: 0,
        }
    }
}

#[derive(Copy, Clone, Default)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}

impl Default for modeinfo {
    #[inline]
    fn default() -> Self {
        modeinfo {
            mbmi: MB_MODE_INFO::default(),
            bmi: [b_mode_info::default(); 16],
        }
    }
}

pub type MODE_INFO = modeinfo;

pub type size_t = usize;
pub type vpx_color_space = ::core::ffi::c_uint;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub type vpx_color_range_t = vpx_color_range;

#[derive(Copy, Clone, Default)]
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

impl yv12_buffer_config {
    pub unsafe fn y_slice(&self) -> &[u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.y_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn y_slice_mut<'a>(&self) -> &'a mut [u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.y_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub unsafe fn u_slice(&self) -> &[u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.u_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn u_slice_mut<'a>(&self) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.u_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub unsafe fn v_slice(&self) -> &[u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.v_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn v_slice_mut<'a>(&self) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let start_ptr = self.v_buffer.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub unsafe fn y_view_mut<'a>(&self) -> &'a mut [u8] {
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        core::slice::from_raw_parts_mut(self.y_buffer, height * stride)
    }

    pub unsafe fn u_view_mut<'a>(&self) -> &'a mut [u8] {
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        core::slice::from_raw_parts_mut(self.u_buffer, height * stride)
    }

    pub unsafe fn v_view_mut<'a>(&self) -> &'a mut [u8] {
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        core::slice::from_raw_parts_mut(self.v_buffer, height * stride)
    }

    pub unsafe fn y_slice_with_offset<'a>(&self, offset: usize) -> &'a [u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.y_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn y_slice_mut_with_offset<'a>(&self, offset: usize) -> &'a mut [u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.y_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub unsafe fn u_slice_with_offset<'a>(&self, offset: usize) -> &'a [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.u_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn u_slice_mut_with_offset<'a>(&self, offset: usize) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.u_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub unsafe fn v_slice_with_offset<'a>(&self, offset: usize) -> &'a [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.v_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts(start_ptr, total_size)
    }

    pub unsafe fn v_slice_mut_with_offset<'a>(&self, offset: usize) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;
        let active_ptr = self.v_buffer.offset(-(offset as isize));
        let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
        core::slice::from_raw_parts_mut(start_ptr, total_size)
    }

    pub fn views_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;
        unsafe {
            (
                core::slice::from_raw_parts_mut(self.y_buffer, y_height * y_stride),
                core::slice::from_raw_parts_mut(self.u_buffer, uv_height * uv_stride),
                core::slice::from_raw_parts_mut(self.v_buffer, uv_height * uv_stride),
            )
        }
    }

    pub fn y_slice_safe(&self) -> &[u8] {
        unsafe {
            let border = self.border as usize;
            let stride = self.y_stride as usize;
            let height = self.y_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.y_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn y_slice_mut_safe(&mut self) -> &mut [u8] {
        unsafe {
            let border = self.border as usize;
            let stride = self.y_stride as usize;
            let height = self.y_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.y_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn u_slice_safe(&self) -> &[u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.u_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn u_slice_mut_safe(&mut self) -> &mut [u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.u_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn v_slice_safe(&self) -> &[u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.v_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn v_slice_mut_safe(&mut self) -> &mut [u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let start_ptr = self.v_buffer.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn y_slice_with_offset_safe(&self, offset: usize) -> &[u8] {
        unsafe {
            let border = self.border as usize;
            let stride = self.y_stride as usize;
            let height = self.y_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.y_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn y_slice_mut_with_offset_safe(&mut self, offset: usize) -> &mut [u8] {
        unsafe {
            let border = self.border as usize;
            let stride = self.y_stride as usize;
            let height = self.y_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.y_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn u_slice_with_offset_safe(&self, offset: usize) -> &[u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.u_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn u_slice_mut_with_offset_safe(&mut self, offset: usize) -> &mut [u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.u_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn v_slice_with_offset_safe(&self, offset: usize) -> &[u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.v_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts(start_ptr, total_size)
        }
    }

    pub fn v_slice_mut_with_offset_safe(&mut self, offset: usize) -> &mut [u8] {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let active_ptr = self.v_buffer.offset(-(offset as isize));
            let start_ptr = active_ptr.offset(-((border * stride + border) as isize));
            core::slice::from_raw_parts_mut(start_ptr, total_size)
        }
    }

    pub fn uv_slices_with_offset_safe(&self, offset: usize) -> (&[u8], &[u8]) {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let u_active_ptr = self.u_buffer.offset(-(offset as isize));
            let u_start_ptr = u_active_ptr.offset(-((border * stride + border) as isize));
            let v_active_ptr = self.v_buffer.offset(-(offset as isize));
            let v_start_ptr = v_active_ptr.offset(-((border * stride + border) as isize));
            (
                core::slice::from_raw_parts(u_start_ptr, total_size),
                core::slice::from_raw_parts(v_start_ptr, total_size),
            )
        }
    }

    pub fn uv_slices_mut_with_offset_safe(&mut self, offset: usize) -> (&mut [u8], &mut [u8]) {
        unsafe {
            let border = (self.border / 2) as usize;
            let stride = self.uv_stride as usize;
            let height = self.uv_height as usize;
            let total_height = height + 2 * border;
            let total_size = total_height * stride;
            let u_active_ptr = self.u_buffer.offset(-(offset as isize));
            let u_start_ptr = u_active_ptr.offset(-((border * stride + border) as isize));
            let v_active_ptr = self.v_buffer.offset(-(offset as isize));
            let v_start_ptr = v_active_ptr.offset(-((border * stride + border) as isize));
            (
                core::slice::from_raw_parts_mut(u_start_ptr, total_size),
                core::slice::from_raw_parts_mut(v_start_ptr, total_size),
            )
        }
    }
}

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

impl Default for vpx_internal_error_info {
    fn default() -> Self {
        vpx_internal_error_info {
            error_code: 0,
            has_detail: 0,
            detail: [0; 80],
            setjmp: 0,
            jmp: [0; 48],
        }
    }
}

impl vpx_internal_error_info {
    pub fn trigger(&mut self, error: vpx_codec_err_t, detail: &str) {
        self.error_code = error;
        self.has_detail = 0;
        if !detail.is_empty() {
            self.has_detail = 1;
            let bytes = detail.as_bytes();
            let len = std::cmp::min(bytes.len(), self.detail.len() - 1);
            for i in 0..len {
                self.detail[i] = bytes[i] as ::core::ffi::c_char;
            }
            self.detail[len] = 0; // Null terminator
        }
        if self.setjmp != 0 {
            unsafe {
                unsafe extern "C" {
                    fn longjmp(_: *mut ::core::ffi::c_int, _: ::core::ffi::c_int) -> !;
                }
                longjmp(
                    &raw mut self.jmp as *mut ::core::ffi::c_int,
                    self.error_code as ::core::ffi::c_int,
                );
            }
        }
    }
}


pub type FRAME_TYPE = ::core::ffi::c_uint;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;

pub type vp8_subpix_fn_t = Option<
    fn(
        src: &[u8],
        src_offset: usize,
        src_stride: ::core::ffi::c_int,
        xoffset: ::core::ffi::c_int,
        yoffset: ::core::ffi::c_int,
        dst: &mut [u8],
        dst_offset: usize,
        dst_stride: ::core::ffi::c_int,
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
    pub mode_info_idx: usize,
    pub mode_info_stride: ::core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub up_available: ::core::ffi::c_int,
    pub left_available: ::core::ffi::c_int,
    pub recon_above: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left_stride: [::core::ffi::c_int; 2],
    pub above_context_idx: usize,
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
    pub current_bc_idx: usize,
    pub corrupted: ::core::ffi::c_int,
    pub error_info: vpx_internal_error_info,
}

impl Default for macroblockd {
    fn default() -> Self {
        macroblockd {
            predictor: [0; 384],
            qcoeff: [0; 400],
            dqcoeff: [0; 400],
            eobs: [0; 25],
            dequant_y1: [0; 16],
            dequant_y1_dc: [0; 16],
            dequant_y2: [0; 16],
            dequant_uv: [0; 16],
            block: [BLOCKD::default(); 25],
            fullpixel_mask: 0,
            pre: YV12_BUFFER_CONFIG::default(),
            dst: YV12_BUFFER_CONFIG::default(),
            mode_info_idx: 0,
            mode_info_stride: 0,
            frame_type: 0,
            up_available: 0,
            left_available: 0,
            recon_above: [core::ptr::null_mut(); 3],
            recon_left: [core::ptr::null_mut(); 3],
            recon_left_stride: [0; 2],
            above_context_idx: 0,
            segmentation_enabled: 0,
            update_mb_segmentation_map: 0,
            update_mb_segmentation_data: 0,
            mb_segment_abs_delta: 0,
            mb_segment_tree_probs: [0; 3],
            segment_feature_data: [[0; 4]; 2],
            mode_ref_lf_delta_enabled: 0,
            mode_ref_lf_delta_update: 0,
            last_ref_lf_deltas: [0; 4],
            ref_lf_deltas: [0; 4],
            last_mode_lf_deltas: [0; 4],
            mode_lf_deltas: [0; 4],
            mb_to_left_edge: 0,
            mb_to_right_edge: 0,
            mb_to_top_edge: 0,
            mb_to_bottom_edge: 0,
            subpixel_predict: None,
            subpixel_predict8x4: None,
            subpixel_predict8x8: None,
            subpixel_predict16x16: None,
            current_bc_idx: 0,
            corrupted: 0,
            error_info: vpx_internal_error_info::default(),
        }
    }
}

pub type MACROBLOCKD = macroblockd;

impl macroblockd {
    pub fn mode_info(&self, mi_base: *const MODE_INFO) -> &MODE_INFO {
        unsafe { &*mi_base.add(self.mode_info_idx) }
    }
    pub fn mode_info_mut(&mut self, mi_base: *mut MODE_INFO) -> &mut MODE_INFO {
        unsafe { &mut *mi_base.add(self.mode_info_idx) }
    }
    pub fn contexts_mut<'a>(
        &'a mut self,
        above_base: *mut ENTROPY_CONTEXT_PLANES,
        left: &'a mut ENTROPY_CONTEXT_PLANES,
    ) -> (&'a mut ENTROPY_CONTEXT_PLANES, &'a mut ENTROPY_CONTEXT_PLANES) {
        unsafe {
            (&mut *above_base.add(self.above_context_idx), left)
        }
    }
    pub fn decode_tokens_inputs_mut<'a>(
        &'a mut self,
        above_base: *mut ENTROPY_CONTEXT_PLANES,
        left: &'a mut ENTROPY_CONTEXT_PLANES,
    ) -> (
        &'a mut ENTROPY_CONTEXT_PLANES,
        &'a mut ENTROPY_CONTEXT_PLANES,
        &'a mut [::core::ffi::c_short; 400],
        &'a mut [::core::ffi::c_char; 25],
    ) {
        unsafe {
            (
                &mut *above_base.add(self.above_context_idx),
                left,
                &mut self.qcoeff,
                &mut self.eobs,
            )
        }
    }
}

pub type vpx_decrypt_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
    ) -> (),
>;

pub type VP8_BD_VALUE = size_t;

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

pub type BOOL_DECODER = vp8_reader;

pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;

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

pub type TOKEN_PARTITION = ::core::ffi::c_uint;
pub const EIGHT_PARTITION: TOKEN_PARTITION = 3;
pub const FOUR_PARTITION: TOKEN_PARTITION = 2;
pub const TWO_PARTITION: TOKEN_PARTITION = 1;
pub const ONE_PARTITION: TOKEN_PARTITION = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv_context {
    pub prob: [vp8_prob; 19],
}
pub type MV_CONTEXT = mv_context;

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
pub type FRAME_CONTEXT = frame_contexts;

pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;

#[derive(Clone)]
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
    pub frame_to_show_idx: Option<usize>,
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
    pub mip: Option<Box<[MODE_INFO]>>,
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
    pub above_context: Option<Box<[ENTROPY_CONTEXT_PLANES]>>,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: ::core::ffi::c_uint,
    pub version: ::core::ffi::c_int,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: ::core::ffi::c_int,
}
pub type VP8_COMMON = VP8Common;

pub type pthread_t = *mut ::core::ffi::c_void;
pub type semaphore_t = *mut ::core::ffi::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODETHREAD_DATA {
    pub ithread: ::core::ffi::c_int,
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone, Default)]
#[repr(C, align(32))]
pub struct MB_ROW_DEC {
    pub mbd: MACROBLOCKD,
}

#[repr(transparent)]
pub struct vpx_atomic_int {
    pub value: core::sync::atomic::AtomicI32,
}

impl Clone for vpx_atomic_int {
    fn clone(&self) -> Self {
        vpx_atomic_int {
            value: core::sync::atomic::AtomicI32::new(self.value.load(core::sync::atomic::Ordering::Relaxed)),
        }
    }
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

#[repr(C)]
pub struct VP8D_COMP {
    pub mb: MACROBLOCKD,
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
    pub mt_current_mb_col: Option<Box<[vpx_atomic_int]>>,
    pub mt_yabove_row: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_uabove_row: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_vabove_row: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_yleft_col: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_uleft_col: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_vleft_col: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mb_row_di: Option<Box<[MB_ROW_DEC]>>,
    pub de_thread_data: Option<Box<[DECODETHREAD_DATA]>>,
    pub h_decoding_thread: Option<Box<[pthread_t]>>,
    pub h_event_start_decoding: Option<Box<[semaphore_t]>>,
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

impl VP8D_COMP {
    /// Safely splits the root structure into disjoint mutable components
    pub fn split_mut(&mut self) -> (&mut MACROBLOCKD, &mut VP8_COMMON, &mut [vp8_reader; 9]) {
        (&mut self.mb, &mut self.common, &mut self.mbc)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_buffers {
    pub pbi: [*mut VP8D_COMP; 32],
}

impl VP8Common {
    pub fn mip_ptr(&self) -> *mut MODE_INFO {
        self.mip.as_ref().map_or(core::ptr::null_mut(), |b| b.as_ptr() as *mut _)
    }
    pub fn mip_mut_ptr(&mut self) -> *mut MODE_INFO {
        self.mip.as_mut().map_or(core::ptr::null_mut(), |b| b.as_mut_ptr())
    }
    pub fn above_context_ptr(&self) -> *mut ENTROPY_CONTEXT_PLANES {
        self.above_context.as_ref().map_or(core::ptr::null_mut(), |b| b.as_ptr() as *mut _)
    }
    pub fn above_context_mut_ptr(&mut self) -> *mut ENTROPY_CONTEXT_PLANES {
        self.above_context.as_mut().map_or(core::ptr::null_mut(), |b| b.as_mut_ptr())
    }
}

