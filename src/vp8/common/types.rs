pub type uint32_t = u32;
pub type uint8_t = u8;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct MV {
    pub row: i16,
    pub col: i16,
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

pub type vp8_prob = u8;

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

pub type B_PREDICTION_MODE = u32;
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

pub type MB_PREDICTION_MODE = u32;
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
    pub offset: i32,
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
pub type vpx_color_space = u32;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub type vpx_color_range_t = vpx_color_range;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct yv12_buffer_config {
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
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}

pub type YV12_BUFFER_CONFIG = yv12_buffer_config;

impl yv12_buffer_config {
    fn safe_len(start: usize, desired_len: usize, total_len: usize) -> usize {
        std::cmp::min(desired_len, total_len.saturating_sub(start))
    }

    pub fn full_buffer_safe(&self) -> &[u8] {
        if self.buffer_alloc.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.buffer_alloc, self.buffer_alloc_sz) }
        }
    }

    pub fn full_buffer_mut_safe(&mut self) -> &mut [u8] {
        if self.buffer_alloc.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.buffer_alloc, self.buffer_alloc_sz) }
        }
    }

    pub fn views(&self) -> (&[u8], &[u8], &[u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + border) * uv_stride;

        let y_offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let u_offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let v_offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);

        let full = self.full_buffer_safe();

        let (y_plane, rest) = full.split_at(yplane_size);
        let (u_plane, v_plane) = rest.split_at(uvplane_size);

        let y_len = Self::safe_len(y_offset, y_height * y_stride, y_plane.len());
        let y_active = &y_plane[y_offset..y_offset + y_len];

        let u_start = u_offset.saturating_sub(yplane_size);
        let u_len = Self::safe_len(u_start, uv_height * uv_stride, u_plane.len());
        let u_active = &u_plane[u_start..u_start + u_len];

        let v_start = v_offset.saturating_sub(yplane_size + uvplane_size);
        let v_len = Self::safe_len(v_start, uv_height * uv_stride, v_plane.len());
        let v_active = &v_plane[v_start..v_start + v_len];

        (y_active, u_active, v_active)
    }

    pub fn views_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + border) * uv_stride;

        let y_offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let u_offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let v_offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);

        let full = self.full_buffer_mut_safe();

        let (y_plane, rest) = full.split_at_mut(yplane_size);
        let (u_plane, v_plane) = rest.split_at_mut(uvplane_size);

        let y_len = Self::safe_len(y_offset, y_height * y_stride, y_plane.len());
        let y_active = &mut y_plane[y_offset..y_offset + y_len];

        let u_start = u_offset.saturating_sub(yplane_size);
        let u_len = Self::safe_len(u_start, uv_height * uv_stride, u_plane.len());
        let u_active = &mut u_plane[u_start..u_start + u_len];

        let v_start = v_offset.saturating_sub(yplane_size + uvplane_size);
        let v_len = Self::safe_len(v_start, uv_height * uv_stride, v_plane.len());
        let v_active = &mut v_plane[v_start..v_start + v_len];

        (y_active, u_active, v_active)
    }

    pub fn safe_views_mut<'a>(
        &self,
        full: &'a mut [u8],
    ) -> (&'a mut [u8], &'a mut [u8], &'a mut [u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + border) * uv_stride;

        let y_offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let u_offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let v_offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);

        assert!(full.len() >= self.buffer_alloc_sz);

        let (y_plane, rest) = full.split_at_mut(yplane_size);
        let (u_plane, v_plane) = rest.split_at_mut(uvplane_size);

        let y_len = Self::safe_len(y_offset, y_height * y_stride, y_plane.len());
        let y_active = &mut y_plane[y_offset..y_offset + y_len];

        let u_start = u_offset.saturating_sub(yplane_size);
        let u_len = Self::safe_len(u_start, uv_height * uv_stride, u_plane.len());
        let u_active = &mut u_plane[u_start..u_start + u_len];

        let v_start = v_offset.saturating_sub(yplane_size + uvplane_size);
        let v_len = Self::safe_len(v_start, uv_height * uv_stride, v_plane.len());
        let v_active = &mut v_plane[v_start..v_start + v_len];

        (y_active, u_active, v_active)
    }

    pub fn safe_get_row_view_mut<'a>(
        &self,
        full: &'a mut [u8],
        row: usize,
    ) -> (&'a mut [u8], &'a mut [u8], &'a mut [u8]) {
        let y_stride = self.y_stride as usize;
        let uv_stride = self.uv_stride as usize;

        let (y_active, u_active, v_active) = self.safe_views_mut(full);

        let y_start = row * 16 * y_stride;
        let u_start = row * 8 * uv_stride;

        let y_row = &mut y_active[y_start..y_start + 16 * y_stride];
        let u_row = &mut u_active[u_start..u_start + 8 * uv_stride];
        let v_row = &mut v_active[u_start..u_start + 8 * uv_stride];

        (y_row, u_row, v_row)
    }

    pub fn safe_get_disjoint_row_views_mut<'a>(
        &self,
        full: &'a mut [u8],
        row1: usize,
        row2: usize,
    ) -> (
        (&'a mut [u8], &'a mut [u8], &'a mut [u8]),
        (&'a mut [u8], &'a mut [u8], &'a mut [u8]),
    ) {
        assert!(row1 < row2);
        let y_stride = self.y_stride as usize;
        let uv_stride = self.uv_stride as usize;

        let (y_active, u_active, v_active) = self.safe_views_mut(full);

        let y_len = 16 * y_stride;
        let y_start1 = row1 * 16 * y_stride;
        let y_start2 = row2 * 16 * y_stride;
        let (y_above_part, y_current_part) = y_active.split_at_mut(y_start2);
        let y_row1 = &mut y_above_part[y_start1..y_start1 + y_len];
        let y_row2 = &mut y_current_part[0..y_len];

        let uv_len = 8 * uv_stride;
        let u_start1 = row1 * 8 * uv_stride;
        let u_start2 = row2 * 8 * uv_stride;
        let (u_above_part, u_current_part) = u_active.split_at_mut(u_start2);
        let u_row1 = &mut u_above_part[u_start1..u_start1 + uv_len];
        let u_row2 = &mut u_current_part[0..uv_len];

        let v_start1 = row1 * 8 * uv_stride;
        let v_start2 = row2 * 8 * uv_stride;
        let (v_above_part, v_current_part) = v_active.split_at_mut(v_start2);
        let v_row1 = &mut v_above_part[v_start1..v_start1 + uv_len];
        let v_row2 = &mut v_current_part[0..uv_len];

        ((y_row1, u_row1, v_row1), (y_row2, u_row2, v_row2))
    }

    pub fn safe_views_mut_with_borders<'a>(
        &self,
        full: &'a mut [u8],
    ) -> (&'a mut [u8], &'a mut [u8], &'a mut [u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_border = border / 2;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + 2 * uv_border) * uv_stride;

        assert!(full.len() >= self.buffer_alloc_sz);
        assert!(yplane_size + 2 * uvplane_size <= full.len());

        let (y_slice, rest) = full.split_at_mut(yplane_size);
        let (u_slice, v_slice) = rest.split_at_mut(uvplane_size);

        (
            &mut y_slice[0..yplane_size],
            &mut u_slice[0..uvplane_size],
            &mut v_slice[0..uvplane_size],
        )
    }

    pub fn safe_y_slice_mut<'a>(&self, full: &'a mut [u8]) -> &'a mut [u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        assert!(full.len() >= self.buffer_alloc_sz);
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn safe_u_slice_mut<'a>(&self, full: &'a mut [u8]) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        assert!(full.len() >= self.buffer_alloc_sz);
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn safe_v_slice_mut<'a>(&self, full: &'a mut [u8]) -> &'a mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        assert!(full.len() >= self.buffer_alloc_sz);
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn safe_uv_slices_mut_with_offset<'a>(
        &self,
        full: &'a mut [u8],
        offset: usize,
    ) -> (&'a mut [u8], &'a mut [u8]) {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset_u = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_u = offset_u
            .saturating_sub(offset)
            .saturating_sub(border * stride + border);

        let offset_v = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_v = offset_v
            .saturating_sub(offset)
            .saturating_sub(border * stride + border);

        assert!(full.len() >= self.buffer_alloc_sz);
        let full_len = full.len();

        let split_pos = std::cmp::min(start_v, full_len);
        let (part_u, part_v) = full.split_at_mut(split_pos);

        let len_u = Self::safe_len(start_u, total_size, part_u.len());
        let u_slice = &mut part_u[start_u..start_u + len_u];

        let len_v = Self::safe_len(0, total_size, part_v.len());
        let v_slice = &mut part_v[0..len_v];

        (u_slice, v_slice)
    }

    pub fn get_row_view_mut(&mut self, row: usize) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let y_stride = self.y_stride as usize;
        let uv_stride = self.uv_stride as usize;

        let (y_active, u_active, v_active) = self.views_mut();

        let y_start = row * 16 * y_stride;
        let u_start = row * 8 * uv_stride;

        let y_row = &mut y_active[y_start..y_start + 16 * y_stride];
        let u_row = &mut u_active[u_start..u_start + 8 * uv_stride];
        let v_row = &mut v_active[u_start..u_start + 8 * uv_stride];

        (y_row, u_row, v_row)
    }

    pub fn get_disjoint_row_views_mut(
        &mut self,
        row1: usize,
        row2: usize,
    ) -> (
        (&mut [u8], &mut [u8], &mut [u8]),
        (&mut [u8], &mut [u8], &mut [u8]),
    ) {
        assert!(row1 < row2);
        let y_stride = self.y_stride as usize;
        let uv_stride = self.uv_stride as usize;

        let (y_active, u_active, v_active) = self.views_mut();

        let y_len = 16 * y_stride;
        let y_start1 = row1 * 16 * y_stride;
        let y_start2 = row2 * 16 * y_stride;
        let (y_above_part, y_current_part) = y_active.split_at_mut(y_start2);
        let y_row1 = &mut y_above_part[y_start1..y_start1 + y_len];
        let y_row2 = &mut y_current_part[0..y_len];

        let uv_len = 8 * uv_stride;
        let u_start1 = row1 * 8 * uv_stride;
        let u_start2 = row2 * 8 * uv_stride;
        let (u_above_part, u_current_part) = u_active.split_at_mut(u_start2);
        let u_row1 = &mut u_above_part[u_start1..u_start1 + uv_len];
        let u_row2 = &mut u_current_part[0..uv_len];

        let v_start1 = row1 * 8 * uv_stride;
        let v_start2 = row2 * 8 * uv_stride;
        let (v_above_part, v_current_part) = v_active.split_at_mut(v_start2);
        let v_row1 = &mut v_above_part[v_start1..v_start1 + uv_len];
        let v_row2 = &mut v_current_part[0..uv_len];

        ((y_row1, u_row1, v_row1), (y_row2, u_row2, v_row2))
    }

    pub fn views_mut_with_borders(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_border = border / 2;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + 2 * uv_border) * uv_stride;

        let full = self.full_buffer_mut_safe();

        assert!(yplane_size + 2 * uvplane_size <= full.len());

        let (y_slice, rest) = full.split_at_mut(yplane_size);
        let (u_slice, v_slice) = rest.split_at_mut(uvplane_size);

        (
            &mut y_slice[0..yplane_size],
            &mut u_slice[0..uvplane_size],
            &mut v_slice[0..uvplane_size],
        )
    }

    pub fn views_with_borders(&self) -> (&[u8], &[u8], &[u8]) {
        let border = self.border as usize;
        let y_stride = self.y_stride as usize;
        let y_height = self.y_height as usize;
        let uv_border = border / 2;
        let uv_stride = self.uv_stride as usize;
        let uv_height = self.uv_height as usize;

        let yplane_size = (y_height + 2 * border) * y_stride;
        let uvplane_size = (uv_height + 2 * uv_border) * uv_stride;

        let full = self.full_buffer_safe();

        assert!(yplane_size + 2 * uvplane_size <= full.len());

        let (y_slice, rest) = full.split_at(yplane_size);
        let (u_slice, v_slice) = rest.split_at(uvplane_size);

        (
            &y_slice[0..yplane_size],
            &u_slice[0..uvplane_size],
            &v_slice[0..uvplane_size],
        )
    }

    pub fn y_slice_safe(&self) -> &[u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &full[start..start + len]
    }

    pub fn y_slice_mut_safe(&mut self) -> &mut [u8] {
        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let height = self.y_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_mut_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn u_slice_safe(&self) -> &[u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &full[start..start + len]
    }

    pub fn u_slice_mut_safe(&mut self) -> &mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_mut_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn v_slice_safe(&self) -> &[u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &full[start..start + len]
    }

    pub fn v_slice_mut_safe(&mut self) -> &mut [u8] {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start = offset.saturating_sub(border * stride + border);
        let full = self.full_buffer_mut_safe();
        let len = Self::safe_len(start, total_size, full.len());
        &mut full[start..start + len]
    }

    pub fn uv_slices_mut_with_offset_safe(&mut self, offset: usize) -> (&mut [u8], &mut [u8]) {
        let border = (self.border / 2) as usize;
        let stride = self.uv_stride as usize;
        let height = self.uv_height as usize;
        let total_height = height + 2 * border;
        let total_size = total_height * stride;

        let offset_u = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_u = offset_u
            .saturating_sub(offset)
            .saturating_sub(border * stride + border);

        let offset_v = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_v = offset_v
            .saturating_sub(offset)
            .saturating_sub(border * stride + border);

        let full = self.full_buffer_mut_safe();
        let full_len = full.len();

        let split_pos = std::cmp::min(start_v, full_len);
        let (part_u, part_v) = full.split_at_mut(split_pos);

        let len_u = Self::safe_len(start_u, total_size, part_u.len());
        let u_slice = &mut part_u[start_u..start_u + len_u];

        let len_v = Self::safe_len(0, total_size, part_v.len());
        let v_slice = &mut part_v[0..len_v];

        (u_slice, v_slice)
    }

    /// Project thread-safe, lock-free `UnsafeRowView` instances mapping the exact boundaries
    /// of the Y, U, and V slice configurations (including borders).
    #[inline]
    pub fn get_safe_unsafe_slices(&self) -> (UnsafeRowView, UnsafeRowView, UnsafeRowView) {
        if self.buffer_alloc.is_null() {
            return (
                UnsafeRowView::new(std::ptr::null_mut(), 0),
                UnsafeRowView::new(std::ptr::null_mut(), 0),
                UnsafeRowView::new(std::ptr::null_mut(), 0),
            );
        }

        let full_buffer = self.full_buffer_safe();

        let border = self.border as usize;
        let stride = self.y_stride as usize;
        let offset = (self.y_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_y = offset.saturating_sub(border * stride + border);
        let ptr_y = full_buffer[start_y..].as_ptr() as *mut u8;

        let border_uv = (self.border / 2) as usize;
        let stride_uv = self.uv_stride as usize;
        let offset_u = (self.u_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_u = offset_u.saturating_sub(border_uv * stride_uv + border_uv);
        let ptr_u = full_buffer[start_u..].as_ptr() as *mut u8;

        let offset_v = (self.v_buffer as usize).saturating_sub(self.buffer_alloc as usize);
        let start_v = offset_v.saturating_sub(border_uv * stride_uv + border_uv);
        let ptr_v = full_buffer[start_v..].as_ptr() as *mut u8;

        (
            UnsafeRowView::new(ptr_y, self.buffer_alloc_sz.saturating_sub(start_y)),
            UnsafeRowView::new(ptr_u, self.buffer_alloc_sz.saturating_sub(start_u)),
            UnsafeRowView::new(ptr_v, self.buffer_alloc_sz.saturating_sub(start_v)),
        )
    }
}

pub type vpx_codec_err_t = u32;
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

pub type jmp_buf = [i32; 48];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [::core::ffi::c_char; 80],
}

impl Default for vpx_internal_error_info {
    fn default() -> Self {
        vpx_internal_error_info {
            error_code: 0,
            has_detail: 0,
            detail: [0; 80],
        }
    }
}

impl vpx_internal_error_info {
    /// Record a decode error and return the [`Vp8Bail`] marker.
    ///
    /// Replaces libvpx's `setjmp`/`longjmp`: instead of unwinding, the decode
    /// path propagates `Err(Vp8Bail)` up to the boundary, which reads the
    /// outcome back from `error_code`. This never panics — corrupt input must
    /// be reported as an error, not crash (the decoder is built with
    /// `panic = abort` in embedders such as Chromium).
    #[must_use = "propagate the bail as Err(Vp8Bail); decoding must stop"]
    pub fn trigger(&mut self, error: vpx_codec_err_t, detail: &str) -> Vp8Bail {
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
        Vp8Bail
    }
}

/// Marker for an internal decode bail, carried as `Err(Vp8Bail)` up the decode
/// call chain. The error code and detail live in [`vpx_internal_error_info`];
/// the boundary turns the bail into a `vpx_codec_err_t`.
pub struct Vp8Bail;

pub type FRAME_TYPE = u32;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;

pub type vp8_subpix_fn_t = Option<
    fn(
        src: &[u8],
        src_offset: usize,
        src_stride: i32,
        xoffset: i32,
        yoffset: i32,
        dst: &mut [u8],
        dst_offset: usize,
        dst_stride: i32,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub qcoeff: [i16; 400],
    pub dqcoeff: [i16; 400],
    pub eobs: [::core::ffi::c_char; 25],
    pub dequant_y1: [i16; 16],
    pub dequant_y1_dc: [i16; 16],
    pub dequant_y2: [i16; 16],
    pub dequant_uv: [i16; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: i32,
    pub dst_fb_idx: usize,
    pub pre_fb_idx: usize,
    pub dst_y_stride: i32,
    pub dst_uv_stride: i32,
    pub dst_border: i32,
    pub pre_y_stride: i32,
    pub pre_uv_stride: i32,
    pub pre_border: i32,
    pub mode_info_idx: usize,
    pub mode_info_stride: i32,
    pub frame_type: FRAME_TYPE,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_left_stride: [i32; 2],
    pub above_context_idx: usize,
    pub segmentation_enabled: u8,
    pub update_mb_segmentation_map: u8,
    pub update_mb_segmentation_data: u8,
    pub mb_segment_abs_delta: u8,
    pub mb_segment_tree_probs: [vp8_prob; 3],
    pub segment_feature_data: [[i8; 4]; 2],
    pub mode_ref_lf_delta_enabled: u8,
    pub mode_ref_lf_delta_update: u8,
    pub last_ref_lf_deltas: [i8; 4],
    pub ref_lf_deltas: [i8; 4],
    pub last_mode_lf_deltas: [i8; 4],
    pub mode_lf_deltas: [i8; 4],
    pub mb_to_left_edge: i32,
    pub mb_to_right_edge: i32,
    pub mb_to_top_edge: i32,
    pub mb_to_bottom_edge: i32,
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc_idx: usize,
    pub corrupted: i32,
    pub error_info: vpx_internal_error_info,
}

impl Default for macroblockd {
    fn default() -> Self {
        macroblockd {
            qcoeff: [0; 400],
            dqcoeff: [0; 400],
            eobs: [0; 25],
            dequant_y1: [0; 16],
            dequant_y1_dc: [0; 16],
            dequant_y2: [0; 16],
            dequant_uv: [0; 16],
            block: [BLOCKD::default(); 25],
            fullpixel_mask: 0,
            dst_fb_idx: 0,
            pre_fb_idx: 0,
            dst_y_stride: 0,
            dst_uv_stride: 0,
            dst_border: 0,
            pre_y_stride: 0,
            pre_uv_stride: 0,
            pre_border: 0,
            mode_info_idx: 0,
            mode_info_stride: 0,
            frame_type: 0,
            up_available: 0,
            left_available: 0,
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
    pub fn mode_info<'a>(&self, mi_base: &'a [MODE_INFO]) -> &'a MODE_INFO {
        &mi_base[self.mode_info_idx]
    }
    pub fn mode_info_mut<'a>(&mut self, mi_base: &'a mut [MODE_INFO]) -> &'a mut MODE_INFO {
        &mut mi_base[self.mode_info_idx]
    }
    pub fn contexts_mut<'a>(
        &'a mut self,
        above_base: &'a mut [ENTROPY_CONTEXT_PLANES],
        left: &'a mut ENTROPY_CONTEXT_PLANES,
    ) -> (
        &'a mut ENTROPY_CONTEXT_PLANES,
        &'a mut ENTROPY_CONTEXT_PLANES,
    ) {
        (&mut above_base[self.above_context_idx], left)
    }
    pub fn decode_tokens_inputs_mut<'a>(
        &'a mut self,
        above_base: &'a mut [ENTROPY_CONTEXT_PLANES],
        left: &'a mut ENTROPY_CONTEXT_PLANES,
    ) -> (
        &'a mut ENTROPY_CONTEXT_PLANES,
        &'a mut ENTROPY_CONTEXT_PLANES,
        &'a mut [i16; 400],
        &'a mut [::core::ffi::c_char; 25],
    ) {
        (
            &mut above_base[self.above_context_idx],
            left,
            &mut self.qcoeff,
            &mut self.eobs,
        )
    }
}

pub type vpx_decrypt_cb =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const u8, *mut u8, i32) -> ()>;

// `decrypt_state` is an opaque token threaded straight through to the
// caller-supplied callback; it is never dereferenced here, so the function
// stays safe to call despite the raw-pointer parameter.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn vpx_decrypt_safe(
    decrypt_cb: vpx_decrypt_cb,
    decrypt_state: *mut ::core::ffi::c_void,
    input: &[u8],
    output: &mut [u8],
) -> bool {
    if let Some(cb) = decrypt_cb {
        let len = std::cmp::min(input.len(), output.len());
        if len > 0 {
            unsafe {
                cb(
                    decrypt_state,
                    input.as_ptr(),
                    output.as_mut_ptr(),
                    len as i32,
                );
            }
            return true;
        }
    }
    false
}

pub type VP8_BD_VALUE = size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_reader {
    pub offset: usize,
    pub value: VP8_BD_VALUE,
    pub count: i32,
    pub range: u32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
}

impl Default for vp8_reader {
    fn default() -> Self {
        Self {
            offset: 0,
            value: 0,
            count: 0,
            range: 0,
            decrypt_cb: None,
            decrypt_state: core::ptr::null_mut(),
        }
    }
}

pub type BOOL_DECODER = vp8_reader;

pub type LOOPFILTERTYPE = u32;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[u8; 1]; 64],
    pub blim: [[u8; 1]; 64],
    pub lim: [[u8; 1]; 64],
    pub hev_thr: [[u8; 1]; 4],
    pub lvl: [[[u8; 4]; 4]; 4],
    pub hev_thr_lut: [[u8; 64]; 2],
    pub mode_lf_lut: [u8; 10],
}

impl Default for loop_filter_info_n {
    fn default() -> Self {
        loop_filter_info_n {
            mblim: [[0; 1]; 64],
            blim: [[0; 1]; 64],
            lim: [[0; 1]; 64],
            hev_thr: [[0; 1]; 4],
            lvl: [[[0; 4]; 4]; 4],
            hev_thr_lut: [[0; 64]; 2],
            mode_lf_lut: [0; 10],
        }
    }
}

pub type TOKEN_PARTITION = u32;
pub const EIGHT_PARTITION: TOKEN_PARTITION = 3;
pub const FOUR_PARTITION: TOKEN_PARTITION = 2;
pub const TWO_PARTITION: TOKEN_PARTITION = 1;
pub const ONE_PARTITION: TOKEN_PARTITION = 0;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct mv_context {
    pub prob: [vp8_prob; 19],
}
pub type MV_CONTEXT = mv_context;

#[derive(Copy, Clone, Default)]
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

pub type CLAMP_TYPE = u32;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;

#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub Y1dequant: [[i16; 2]; 128],
    pub Y2dequant: [[i16; 2]; 128],
    pub UVdequant: [[i16; 2]; 128],
    pub Width: i32,
    pub Height: i32,
    pub horiz_scale: i32,
    pub vert_scale: i32,
    pub clamp_type: CLAMP_TYPE,
    pub frame_to_show_idx: Option<usize>,
    pub yv12_fb: [YV12_BUFFER_CONFIG; 4],
    pub yv12_fb_allocs: [Option<crate::vpx_mem::vpx_mem::AlignedBox>; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: YV12_BUFFER_CONFIG,
    pub temp_scale_frame_alloc: Option<crate::vpx_mem::vpx_mem::AlignedBox>,
    pub last_frame_type: FRAME_TYPE,

    pub frame_type: FRAME_TYPE,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub MBs: i32,
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
    pub mip: Option<Box<[MODE_INFO]>>,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: loop_filter_info_n,
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
    pub above_context: Option<Box<[ENTROPY_CONTEXT_PLANES]>>,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: u32,
    pub version: i32,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: i32,
}
pub type VP8_COMMON = VP8Common;

impl Default for VP8Common {
    fn default() -> Self {
        VP8Common {
            error: vpx_internal_error_info::default(),
            Y1dequant: [[0; 2]; 128],
            Y2dequant: [[0; 2]; 128],
            UVdequant: [[0; 2]; 128],
            Width: 0,
            Height: 0,
            horiz_scale: 0,
            vert_scale: 0,
            clamp_type: 0,
            frame_to_show_idx: None,
            yv12_fb: [YV12_BUFFER_CONFIG::default(); 4],
            yv12_fb_allocs: [None, None, None, None],
            fb_idx_ref_cnt: [0; 4],
            new_fb_idx: 0,
            lst_fb_idx: 0,
            gld_fb_idx: 0,
            alt_fb_idx: 0,
            temp_scale_frame: YV12_BUFFER_CONFIG::default(),
            temp_scale_frame_alloc: None,

            last_frame_type: 0,
            frame_type: 0,
            show_frame: 0,
            frame_flags: 0,
            MBs: 0,
            mb_rows: 0,
            mb_cols: 0,
            mode_info_stride: 0,
            mb_no_coeff_skip: 0,
            no_lpf: 0,
            use_bilinear_mc_filter: 0,
            full_pixel: 0,
            base_qindex: 0,
            y1dc_delta_q: 0,
            y2dc_delta_q: 0,
            y2ac_delta_q: 0,
            uvdc_delta_q: 0,
            uvac_delta_q: 0,
            mip: None,
            filter_type: 0,
            lf_info: loop_filter_info_n::default(),
            filter_level: 0,
            last_sharpness_level: 0,
            sharpness_level: 0,
            refresh_last_frame: 0,
            refresh_golden_frame: 0,
            refresh_alt_ref_frame: 0,
            copy_buffer_to_gf: 0,
            copy_buffer_to_arf: 0,
            refresh_entropy_probs: 0,
            ref_frame_sign_bias: [0; 4],
            above_context: None,
            left_context: ENTROPY_CONTEXT_PLANES::default(),
            lfc: FRAME_CONTEXT::default(),
            fc: FRAME_CONTEXT::default(),
            current_video_frame: 0,
            version: 0,
            multi_token_partition: 0,
            processor_core_count: 0,
        }
    }
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

impl Default for vpx_atomic_int {
    fn default() -> Self {
        vpx_atomic_int {
            value: core::sync::atomic::AtomicI32::new(0),
        }
    }
}

impl Clone for vpx_atomic_int {
    fn clone(&self) -> Self {
        vpx_atomic_int {
            value: core::sync::atomic::AtomicI32::new(
                self.value.load(core::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct FRAGMENT_DATA {
    pub enabled: i32,
    pub count: u32,
    pub ptrs: [*const u8; 9],
    pub sizes: [u32; 9],
}

impl FRAGMENT_DATA {
    pub fn get_slice(&self, idx: usize) -> Option<&[u8]> {
        if idx >= 9 || idx >= self.count as usize {
            return None;
        }
        let ptr = self.ptrs[idx];
        let size = self.sizes[idx] as usize;
        if ptr.is_null() {
            return Some(&[]);
        }
        unsafe { Some(core::slice::from_raw_parts(ptr, size)) }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct VP8D_CONFIG {
    pub Width: i32,
    pub Height: i32,
    pub Version: i32,
    pub postprocess: i32,
    pub max_threads: i32,
    pub error_concealment: i32,
}

/// `UnsafeRowView` is a zero-overhead, thread-safe wrapper around a raw pointer and a length.
///
/// ### Safety & Concurrency Contract
/// Inside multithreaded VP8 decoding, different worker threads concurrently process disjoint
/// rows of the frame buffer or read/write disjoint rows of synchronization status.
/// Since standard safe Rust wrappers like `Mutex` or `RwLock` introduce substantial runtime
/// overhead and lock contention, `UnsafeRowView` uses raw pointers internally to bypass
/// the compiler's aliasing restrictions.
///
/// Safety is mathematically guaranteed at the hardware level by atomic column spinlocks
/// (`vp8_atomic_spin_wait`), which coordinate thread execution to ensure Thread A never
/// writes to column C of row R while Thread B reads from column C of row R.
///
/// Caller MUST ensure that:
/// 1. Thread synchronization prevents concurrent overlapping mutable borrows or mutable/immutable
///    borrow conflicts on the underlying memory segment.
/// 2. Slices produced by `as_slice` or `as_slice_mut` do not outlive the underlying allocation.
#[derive(Copy, Clone)]
pub struct UnsafeRowView {
    ptr: *mut u8,
    len: usize,
}

impl Default for UnsafeRowView {
    #[inline]
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
        }
    }
}

unsafe impl Send for UnsafeRowView {}
unsafe impl Sync for UnsafeRowView {}

impl UnsafeRowView {
    /// Creates a new `UnsafeRowView` from a raw pointer and a length.
    #[inline]
    pub fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Access the underlying raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    /// Access the length of the underlying buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if the underlying buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Project a safe, immutable slice reference for a subsegment of the row.
    ///
    /// # Safety
    /// The caller must ensure via atomic synchronization that no other thread is mutably
    /// accessing this subsegment concurrently.
    #[inline]
    pub unsafe fn as_slice<'a>(&self, offset: usize, len: usize) -> &'a [u8] {
        assert!(
            offset + len <= self.len,
            "UnsafeRowView::as_slice out of bounds: offset={}, len={}, total={}",
            offset,
            len,
            self.len
        );
        // SAFETY: This unsafe block is there by design to project disjoint lock-free slices for hardware-synchronized multithreaded performance. DO NOT REMOVE.
        unsafe { std::slice::from_raw_parts(self.ptr.add(offset), len) }
    }

    /// Project a safe, mutable slice reference for a subsegment of the row.
    ///
    /// # Safety
    /// The caller must ensure via atomic synchronization that no other thread is mutably
    /// or immutably accessing this subsegment concurrently.
    #[inline]
    pub unsafe fn as_slice_mut<'a>(&self, offset: usize, len: usize) -> &'a mut [u8] {
        assert!(
            offset + len <= self.len,
            "UnsafeRowView::as_slice_mut out of bounds: offset={}, len={}, total={}",
            offset,
            len,
            self.len
        );
        // SAFETY: This unsafe block is there by design to project disjoint lock-free slices for hardware-synchronized multithreaded performance. DO NOT REMOVE.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.add(offset), len) }
    }
}

/// `SendPtr` is a thread-safe wrapper around a raw pointer.
pub struct SendPtr<T>(pub *const T);
unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}

impl<T> Copy for SendPtr<T> {}
impl<T> Clone for SendPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Default)]
#[repr(C)]
pub struct VP8D_MT_SYNC {
    pub sync_range: i32,
    pub mt_current_mb_col: Option<Box<[vpx_atomic_int]>>,
    pub mt_yabove_row: Option<Box<[UnsafeRowView]>>,
    pub mt_uabove_row: Option<Box<[UnsafeRowView]>>,
    pub mt_vabove_row: Option<Box<[UnsafeRowView]>>,
    pub mt_yleft_col: Option<Box<[UnsafeRowView]>>,
    pub mt_uleft_col: Option<Box<[UnsafeRowView]>>,
    pub mt_vleft_col: Option<Box<[UnsafeRowView]>>,
    pub mt_yabove_row_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_uabove_row_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_vabove_row_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_yleft_col_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_uleft_col_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub mt_vleft_col_allocs: Option<Box<[Option<crate::vpx_mem::vpx_mem::AlignedBox>]>>,
    pub h_decoding_thread: Option<Box<[Option<std::thread::JoinHandle<()>>]>>,
    pub h_event_start_decoding: Option<Box<[std::sync::Arc<crate::thread_shim::Semaphore>]>>,
    pub h_event_end_decoding: Option<std::sync::Arc<crate::thread_shim::Semaphore>>,
}

#[derive(Default)]
#[repr(C)]
pub struct VP8D_COMP {
    pub mb: MACROBLOCKD,
    pub common: VP8_COMMON,
    pub mbc: [vp8_reader; 9],
    pub oxcf: VP8D_CONFIG,
    pub fragments: FRAGMENT_DATA,
    pub b_multithreaded_rd: vpx_atomic_int,
    pub max_threads: i32,
    pub current_mb_col_main: i32,
    pub decoding_thread_count: u32,
    pub allocated_decoding_thread_count: i32,
    pub mt_baseline_filter_level: [i32; 4],
    pub mt_sync: VP8D_MT_SYNC,
    pub mb_row_di: Option<Box<[std::sync::Arc<std::sync::Mutex<MB_ROW_DEC>>]>>,
    pub ready_for_new_data: i32,
    pub prob_intra: vp8_prob,
    pub prob_last: vp8_prob,
    pub prob_gf: vp8_prob,
    pub prob_skip_false: vp8_prob,
    pub ec_enabled: i32,
    pub ec_active: i32,
    pub decoded_key_frame: i32,
    pub independent_partitions: i32,
    pub frame_corrupt_residual: i32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub restart_threads: i32,
}

impl VP8D_COMP {
    /// Safely splits the root structure into disjoint mutable components
    pub fn split_mut(&mut self) -> (&mut MACROBLOCKD, &mut VP8_COMMON, &mut [vp8_reader; 9]) {
        (&mut self.mb, &mut self.common, &mut self.mbc)
    }
    pub fn split_mt_mut(&mut self) -> (&mut VP8_COMMON, &mut [vp8_reader; 9], &mut VP8D_MT_SYNC) {
        (&mut self.common, &mut self.mbc, &mut self.mt_sync)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_buffers {
    pub pbi: [*mut VP8D_COMP; 32],
}

impl VP8Common {
    pub fn mip_slice(&self) -> &[MODE_INFO] {
        self.mip.as_deref().unwrap_or(&[])
    }
    pub fn mip_slice_mut(&mut self) -> &mut [MODE_INFO] {
        self.mip.as_deref_mut().unwrap_or(&mut [])
    }
    pub fn above_context_slice(&self) -> &[ENTROPY_CONTEXT_PLANES] {
        self.above_context.as_deref().unwrap_or(&[])
    }
    pub fn above_context_slice_mut(&mut self) -> &mut [ENTROPY_CONTEXT_PLANES] {
        self.above_context.as_deref_mut().unwrap_or(&mut [])
    }
    pub fn get_ref_and_dst_fb(
        &mut self,
        ref_idx: usize,
        dst_idx: usize,
    ) -> (&YV12_BUFFER_CONFIG, &mut YV12_BUFFER_CONFIG) {
        assert!(ref_idx != dst_idx);
        if ref_idx < dst_idx {
            let (left, right) = self.yv12_fb.split_at_mut(dst_idx);
            (&left[ref_idx], &mut right[0])
        } else {
            let (left, right) = self.yv12_fb.split_at_mut(ref_idx);
            (&right[0], &mut left[dst_idx])
        }
    }
}
