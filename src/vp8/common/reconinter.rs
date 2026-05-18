unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vp8_copy_mem16x16_neon(
        src: *mut ::core::ffi::c_uchar,
        src_stride: ::core::ffi::c_int,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_copy_mem8x4_neon(
        src: *mut ::core::ffi::c_uchar,
        src_stride: ::core::ffi::c_int,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_copy_mem8x8_neon(
        src: *mut ::core::ffi::c_uchar,
        src_stride: ::core::ffi::c_int,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type uint8_t = u8;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MB_MODE_COUNT: C2RustUnnamed = 10;
pub const SPLITMV: C2RustUnnamed = 9;
pub const NEWMV: C2RustUnnamed = 8;
pub const ZEROMV: C2RustUnnamed = 7;
pub const NEARMV: C2RustUnnamed = 6;
pub const NEARESTMV: C2RustUnnamed = 5;
pub const B_PRED: C2RustUnnamed = 4;
pub const TM_PRED: C2RustUnnamed = 3;
pub const H_PRED: C2RustUnnamed = 2;
pub const V_PRED: C2RustUnnamed = 1;
pub const DC_PRED: C2RustUnnamed = 0;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub fn vp8_copy_mem16x16_safe(
    src: &[u8],
    src_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..16 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 16].copy_from_slice(&src[src_idx..src_idx + 16]);
    }
}

pub fn vp8_copy_mem8x8_safe(
    src: &[u8],
    src_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..8 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 8].copy_from_slice(&src[src_idx..src_idx + 8]);
    }
}

pub fn vp8_copy_mem8x4_safe(
    src: &[u8],
    src_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
) {
    let src_stride = src_stride as usize;
    let dst_stride = dst_stride as usize;
    for r in 0..4 {
        let src_idx = r * src_stride;
        let dst_idx = r * dst_stride;
        dst[dst_idx..dst_idx + 8].copy_from_slice(&src[src_idx..src_idx + 8]);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem16x16_c(
    src: *const ::core::ffi::c_uchar,
    src_stride: ::core::ffi::c_int,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
) {
    let src_len = 15 * src_stride as usize + 16;
    let dst_len = 15 * dst_stride as usize + 16;
    unsafe {
        let src_slice = core::slice::from_raw_parts(src, src_len);
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vp8_copy_mem16x16_safe(src_slice, src_stride, dst_slice, dst_stride);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x8_c(
    src: *const ::core::ffi::c_uchar,
    src_stride: ::core::ffi::c_int,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
) {
    let src_len = 7 * src_stride as usize + 8;
    let dst_len = 7 * dst_stride as usize + 8;
    unsafe {
        let src_slice = core::slice::from_raw_parts(src, src_len);
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vp8_copy_mem8x8_safe(src_slice, src_stride, dst_slice, dst_stride);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x4_c(
    src: *const ::core::ffi::c_uchar,
    src_stride: ::core::ffi::c_int,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
) {
    let src_len = 3 * src_stride as usize + 8;
    let dst_len = 3 * dst_stride as usize + 8;
    unsafe {
        let src_slice = core::slice::from_raw_parts(src, src_len);
        let dst_slice = core::slice::from_raw_parts_mut(dst, dst_len);
        vp8_copy_mem8x4_safe(src_slice, src_stride, dst_slice, dst_stride);
    }
}

fn build_inter_predictors4b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: ::core::ffi::c_int,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: ::core::ffi::c_int,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;
    
    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;
    
    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;
    
    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;
    
    if row & 7 != 0 || col & 7 != 0 {
        assert!(pre_idx < pre_slice.len());
        assert!(dst_idx < dst_slice.len());
        
        let ptr = unsafe { pre_slice.as_ptr().add(pre_idx) as *mut u8 };
        let dst_ptr = unsafe { dst_slice.as_mut_ptr().add(dst_idx) };
        
        unsafe {
            sppf.expect("non-null function pointer")(
                ptr,
                pre_stride,
                col & 7,
                row & 7,
                dst_ptr,
                dst_stride,
            );
        }
    } else {
        let pre_len = 7 * pre_stride_u + 8;
        let dst_len = 7 * dst_stride_u + 8;
        
        let sub_pre = &pre_slice[pre_idx .. pre_idx + pre_len];
        let sub_dst = &mut dst_slice[dst_idx .. dst_idx + dst_len];
        
        vp8_copy_mem8x8_safe(sub_pre, pre_stride, sub_dst, dst_stride);
    }
}

fn build_inter_predictors2b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: ::core::ffi::c_int,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: ::core::ffi::c_int,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;
    
    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;
    
    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;
    
    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;
    
    if row & 7 != 0 || col & 7 != 0 {
        assert!(pre_idx < pre_slice.len());
        assert!(dst_idx < dst_slice.len());
        
        let ptr = unsafe { pre_slice.as_ptr().add(pre_idx) as *mut u8 };
        let dst_ptr = unsafe { dst_slice.as_mut_ptr().add(dst_idx) };
        
        unsafe {
            sppf.expect("non-null function pointer")(
                ptr,
                pre_stride,
                col & 7,
                row & 7,
                dst_ptr,
                dst_stride,
            );
        }
    } else {
        let pre_len = 3 * pre_stride_u + 8;
        let dst_len = 3 * dst_stride_u + 8;
        
        let sub_pre = &pre_slice[pre_idx .. pre_idx + pre_len];
        let sub_dst = &mut dst_slice[dst_idx .. dst_idx + dst_len];
        
        vp8_copy_mem8x4_safe(sub_pre, pre_stride, sub_dst, dst_stride);
    }
}

fn build_inter_predictors_b_safe(
    sppf: vp8_subpix_fn_t,
    d: &BLOCKD,
    dst_slice: &mut [u8],
    dst_offset: usize,
    dst_stride: ::core::ffi::c_int,
    pre_slice: &[u8],
    pre_offset: usize,
    pre_stride: ::core::ffi::c_int,
) {
    let mv = d.bmi.mv().as_mv();
    let row = mv.row as i32;
    let col = mv.col as i32;
    
    let pre_stride_u = pre_stride as usize;
    let dst_stride_u = dst_stride as usize;
    
    let mv_row_offset = (row >> 3) * pre_stride;
    let mv_col_offset = col >> 3;
    
    let pre_idx = (pre_offset as i32 + d.offset + mv_row_offset + mv_col_offset) as usize;
    let dst_idx = dst_offset;
    
    if row & 7 != 0 || col & 7 != 0 {
        assert!(pre_idx < pre_slice.len());
        assert!(dst_idx < dst_slice.len());
        
        let ptr = unsafe { pre_slice.as_ptr().add(pre_idx) as *mut u8 };
        let dst_ptr = unsafe { dst_slice.as_mut_ptr().add(dst_idx) };
        
        unsafe {
            sppf.expect("non-null function pointer")(
                ptr,
                pre_stride,
                col & 7,
                row & 7,
                dst_ptr,
                dst_stride,
            );
        }
    } else {
        for r in 0..4 {
            let s_idx = pre_idx + r * pre_stride_u;
            let d_idx = dst_idx + r * dst_stride_u;
            dst_slice[d_idx .. d_idx + 4].copy_from_slice(&pre_slice[s_idx .. s_idx + 4]);
        }
    }
}
fn clamp_mv_to_umv_border(
    mv: &mut MV,
    mb_to_left_edge: ::core::ffi::c_int,
    mb_to_right_edge: ::core::ffi::c_int,
    mb_to_top_edge: ::core::ffi::c_int,
    mb_to_bottom_edge: ::core::ffi::c_int,
) {
    if (mv.col as ::core::ffi::c_int)
        < mb_to_left_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mv.col = (mb_to_left_edge
            - ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    } else if mv.col as ::core::ffi::c_int
        > mb_to_right_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mv.col = (mb_to_right_edge
            + ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    }
    if (mv.row as ::core::ffi::c_int)
        < mb_to_top_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mv.row = (mb_to_top_edge - ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    } else if mv.row as ::core::ffi::c_int
        > mb_to_bottom_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mv.row = (mb_to_bottom_edge
            + ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
            as ::core::ffi::c_short;
    }
}
fn clamp_uvmv_to_umv_border(
    mv: &mut MV,
    mb_to_left_edge: ::core::ffi::c_int,
    mb_to_right_edge: ::core::ffi::c_int,
    mb_to_top_edge: ::core::ffi::c_int,
    mb_to_bottom_edge: ::core::ffi::c_int,
) {
    mv.col = (if (2 as ::core::ffi::c_int * mv.col as ::core::ffi::c_int)
        < mb_to_left_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mb_to_left_edge - ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int
    } else {
        mv.col as ::core::ffi::c_int
    }) as ::core::ffi::c_short;
    mv.col = (if 2 as ::core::ffi::c_int * mv.col as ::core::ffi::c_int
        > mb_to_right_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mb_to_right_edge + ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int
    } else {
        mv.col as ::core::ffi::c_int
    }) as ::core::ffi::c_short;
    mv.row = (if (2 as ::core::ffi::c_int * mv.row as ::core::ffi::c_int)
        < mb_to_top_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mb_to_top_edge - ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int
    } else {
        mv.row as ::core::ffi::c_int
    }) as ::core::ffi::c_short;
    mv.row = (if 2 as ::core::ffi::c_int * mv.row as ::core::ffi::c_int
        > mb_to_bottom_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        mb_to_bottom_edge + ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int
    } else {
        mv.row as ::core::ffi::c_int
    }) as ::core::ffi::c_short;
}
pub fn vp8_build_inter16x16_predictors_mb(
    x: &MACROBLOCKD,
) { unsafe {
    let dst_y = x.dst.y_buffer;
    let dst_u = x.dst.u_buffer;
    let dst_v = x.dst.v_buffer;
    let dst_ystride = x.dst.y_stride;
    let dst_uvstride = x.dst.uv_stride;
    let mut offset: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut uptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut vptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut _16x16mv: int_mv = int_mv { as_int: 0 };
    let mut ptr_base: *mut ::core::ffi::c_uchar = x.pre.y_buffer as *mut ::core::ffi::c_uchar;
    let mut pre_stride: ::core::ffi::c_int = x.pre.y_stride;
    _16x16mv.as_int = x.mode_info().mbmi.mv.as_int;
    if x.mode_info().mbmi.need_to_clamp_mvs != 0 {
        clamp_mv_to_umv_border(
            &mut _16x16mv.as_mv,
            x.mb_to_left_edge,
            x.mb_to_right_edge,
            x.mb_to_top_edge,
            x.mb_to_bottom_edge,
        );
    }
    ptr = ptr_base
        .offset(
            ((_16x16mv.as_mv.row as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) * pre_stride)
                as isize,
        )
        .offset((_16x16mv.as_mv.col as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize);
    if _16x16mv.as_int & 0x70007 as uint32_t != 0 {
        x.subpixel_predict16x16
            .expect("non-null function pointer")(
            ptr,
            pre_stride,
            _16x16mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            _16x16mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst_y,
            dst_ystride,
        );
    } else {
        vp8_copy_mem16x16_neon(ptr, pre_stride, dst_y, dst_ystride);
    }
    _16x16mv.as_mv.row = (_16x16mv.as_mv.row as ::core::ffi::c_int
        + (1 as ::core::ffi::c_int
            | _16x16mv.as_mv.row as ::core::ffi::c_int
                >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1 as usize))) as ::core::ffi::c_short;
    _16x16mv.as_mv.col = (_16x16mv.as_mv.col as ::core::ffi::c_int
        + (1 as ::core::ffi::c_int
            | _16x16mv.as_mv.col as ::core::ffi::c_int
                >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(1 as usize))) as ::core::ffi::c_short;
    _16x16mv.as_mv.row = (_16x16mv.as_mv.row as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
        as ::core::ffi::c_short;
    _16x16mv.as_mv.col = (_16x16mv.as_mv.col as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
        as ::core::ffi::c_short;
    _16x16mv.as_mv.row =
        (_16x16mv.as_mv.row as ::core::ffi::c_int & x.fullpixel_mask) as ::core::ffi::c_short;
    _16x16mv.as_mv.col =
        (_16x16mv.as_mv.col as ::core::ffi::c_int & x.fullpixel_mask) as ::core::ffi::c_short;
    if (2 as ::core::ffi::c_int * _16x16mv.as_mv.col as ::core::ffi::c_int)
        < x.mb_to_left_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        || 2 as ::core::ffi::c_int * _16x16mv.as_mv.col as ::core::ffi::c_int
            > x.mb_to_right_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        || (2 as ::core::ffi::c_int * _16x16mv.as_mv.row as ::core::ffi::c_int)
            < x.mb_to_top_edge - ((19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        || 2 as ::core::ffi::c_int * _16x16mv.as_mv.row as ::core::ffi::c_int
            > x.mb_to_bottom_edge + ((18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        return;
    }
    pre_stride >>= 1 as ::core::ffi::c_int;
    offset = (_16x16mv.as_mv.row as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) * pre_stride
        + (_16x16mv.as_mv.col as ::core::ffi::c_int >> 3 as ::core::ffi::c_int);
    uptr = x.pre.u_buffer.offset(offset as isize) as *mut ::core::ffi::c_uchar;
    vptr = x.pre.v_buffer.offset(offset as isize) as *mut ::core::ffi::c_uchar;
    if _16x16mv.as_int & 0x70007 as uint32_t != 0 {
        x.subpixel_predict8x8.expect("non-null function pointer")(
            uptr,
            pre_stride,
            _16x16mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            _16x16mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst_u,
            dst_uvstride,
        );
        x.subpixel_predict8x8.expect("non-null function pointer")(
            vptr,
            pre_stride,
            _16x16mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            _16x16mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst_v,
            dst_uvstride,
        );
    } else {
        vp8_copy_mem8x8_neon(uptr, pre_stride, dst_u, dst_uvstride);
        vp8_copy_mem8x8_neon(vptr, pre_stride, dst_v, dst_uvstride);
    };
}}
fn build_inter4x4_predictors_mb(x: &mut MACROBLOCKD) {
    let partitioning = x.mode_info().mbmi.partitioning;
    let need_to_clamp_mvs = x.mode_info().mbmi.need_to_clamp_mvs;
    
    let dst_y_stride = x.dst.y_stride;
    let dst_uv_stride = x.dst.uv_stride;
    let pre_y_stride = x.pre.y_stride;
    let pre_uv_stride = x.pre.uv_stride;
    
    let border = x.dst.border as usize;
    let uv_border = border / 2;
    
    // Reconstruct global offsets
    let mb_row = (-x.mb_to_top_edge / 128) as usize;
    let mb_col = (-x.mb_to_left_edge / 128) as usize;
    
    let recon_yoffset = mb_row * 16 * dst_y_stride as usize + mb_col * 16;
    let recon_uvoffset = mb_row * 8 * dst_uv_stride as usize + mb_col * 8;
    
    let pre_recon_yoffset = mb_row * 16 * pre_y_stride as usize + mb_col * 16;
    let pre_recon_uvoffset = mb_row * 8 * pre_uv_stride as usize + mb_col * 8;
    
    {
        let bmi = x.mode_info().bmi;
        if (partitioning as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
            x.block[0].bmi = bmi[0];
            x.block[2].bmi = bmi[2];
            x.block[8].bmi = bmi[8];
            x.block[10].bmi = bmi[10];
            
            if need_to_clamp_mvs != 0 {
                let mb_to_left_edge = x.mb_to_left_edge;
                let mb_to_right_edge = x.mb_to_right_edge;
                let mb_to_top_edge = x.mb_to_top_edge;
                let mb_to_bottom_edge = x.mb_to_bottom_edge;
                
                clamp_mv_to_umv_border(x.block[0].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
                clamp_mv_to_umv_border(x.block[2].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
                clamp_mv_to_umv_border(x.block[8].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
                clamp_mv_to_umv_border(x.block[10].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
            }
        } else {
            for i in (0..16).step_by(2) {
                x.block[i].bmi = bmi[i];
                x.block[i+1].bmi = bmi[i+1];
                if need_to_clamp_mvs != 0 {
                    let mb_to_left_edge = x.mb_to_left_edge;
                    let mb_to_right_edge = x.mb_to_right_edge;
                    let mb_to_top_edge = x.mb_to_top_edge;
                    let mb_to_bottom_edge = x.mb_to_bottom_edge;
                    
                    clamp_mv_to_umv_border(x.block[i].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
                    clamp_mv_to_umv_border(x.block[i+1].bmi.mv_mut().as_mv_mut(), mb_to_left_edge, mb_to_right_edge, mb_to_top_edge, mb_to_bottom_edge);
                }
            }
        }
    }
    
    let subpixel_predict8x8 = x.subpixel_predict8x8;
    let subpixel_predict8x4 = x.subpixel_predict8x4;
    let subpixel_predict = x.subpixel_predict;
    
    // Reconstruct global Y slices
    let dst_y_global_active = unsafe { x.dst.y_buffer.offset(-(recon_yoffset as isize)) };
    let dst_y_global_start = unsafe { dst_y_global_active.offset(-((border * dst_y_stride as usize + border) as isize)) };
    let dst_y_total_size = (x.dst.y_height as usize + 2 * border) * dst_y_stride as usize;
    let mut dst_y_slice = unsafe { core::slice::from_raw_parts_mut(dst_y_global_start, dst_y_total_size) };
    
    let pre_y_global_active = unsafe { x.pre.y_buffer.offset(-(pre_recon_yoffset as isize)) };
    let pre_y_global_start = unsafe { pre_y_global_active.offset(-((border * pre_y_stride as usize + border) as isize)) };
    let pre_y_total_size = (x.pre.y_height as usize + 2 * border) * pre_y_stride as usize;
    let pre_y_slice = unsafe { core::slice::from_raw_parts(pre_y_global_start, pre_y_total_size) };
    
    let dst_y_active_offset = border * dst_y_stride as usize + border + recon_yoffset;
    let pre_y_active_offset = border * pre_y_stride as usize + border + pre_recon_yoffset;
    
    if (partitioning as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[0],
            &mut dst_y_slice,
            dst_y_active_offset + x.block[0].offset as usize,
            dst_y_stride,
            &pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[2],
            &mut dst_y_slice,
            dst_y_active_offset + x.block[2].offset as usize,
            dst_y_stride,
            &pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[8],
            &mut dst_y_slice,
            dst_y_active_offset + x.block[8].offset as usize,
            dst_y_stride,
            &pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
        build_inter_predictors4b_safe(
            subpixel_predict8x8,
            &x.block[10],
            &mut dst_y_slice,
            dst_y_active_offset + x.block[10].offset as usize,
            dst_y_stride,
            &pre_y_slice,
            pre_y_active_offset,
            pre_y_stride,
        );
    } else {
        for i in (0..16).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i+1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    &mut dst_y_slice,
                    dst_y_active_offset + d0.offset as usize,
                    dst_y_stride,
                    &pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    &mut dst_y_slice,
                    dst_y_active_offset + d0.offset as usize,
                    dst_y_stride,
                    &pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    &mut dst_y_slice,
                    dst_y_active_offset + d1.offset as usize,
                    dst_y_stride,
                    &pre_y_slice,
                    pre_y_active_offset,
                    pre_y_stride,
                );
            }
        }
    }
    
    drop(dst_y_slice);
    
    // Reconstruct global U slices
    let dst_u_global_active = unsafe { x.dst.u_buffer.offset(-(recon_uvoffset as isize)) };
    let dst_u_global_start = unsafe { dst_u_global_active.offset(-((uv_border * dst_uv_stride as usize + uv_border) as isize)) };
    let dst_u_total_size = (x.dst.uv_height as usize + 2 * uv_border) * dst_uv_stride as usize;
    let mut dst_u_slice = unsafe { core::slice::from_raw_parts_mut(dst_u_global_start, dst_u_total_size) };
    
    let pre_u_global_active = unsafe { x.pre.u_buffer.offset(-(pre_recon_uvoffset as isize)) };
    let pre_u_global_start = unsafe { pre_u_global_active.offset(-((uv_border * pre_uv_stride as usize + uv_border) as isize)) };
    let pre_u_total_size = (x.pre.uv_height as usize + 2 * uv_border) * pre_uv_stride as usize;
    let pre_u_slice = unsafe { core::slice::from_raw_parts(pre_u_global_start, pre_u_total_size) };
    
    let dst_uv_active_offset = uv_border * dst_uv_stride as usize + uv_border + recon_uvoffset;
    let pre_uv_active_offset = uv_border * pre_uv_stride as usize + uv_border + pre_recon_uvoffset;
    
    {
        for i in (16..20).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i+1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    &mut dst_u_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    &pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    &mut dst_u_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    &pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    &mut dst_u_slice,
                    dst_uv_active_offset + d1.offset as usize,
                    dst_uv_stride,
                    &pre_u_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            }
        }
    }
    drop(dst_u_slice);
    
    // Reconstruct global V slices
    let dst_v_global_active = unsafe { x.dst.v_buffer.offset(-(recon_uvoffset as isize)) };
    let dst_v_global_start = unsafe { dst_v_global_active.offset(-((uv_border * dst_uv_stride as usize + uv_border) as isize)) };
    let dst_v_total_size = (x.dst.uv_height as usize + 2 * uv_border) * dst_uv_stride as usize;
    let mut dst_v_slice = unsafe { core::slice::from_raw_parts_mut(dst_v_global_start, dst_v_total_size) };
    
    let pre_v_global_active = unsafe { x.pre.v_buffer.offset(-(pre_recon_uvoffset as isize)) };
    let pre_v_global_start = unsafe { pre_v_global_active.offset(-((uv_border * pre_uv_stride as usize + uv_border) as isize)) };
    let pre_v_total_size = (x.pre.uv_height as usize + 2 * uv_border) * pre_uv_stride as usize;
    let pre_v_slice = unsafe { core::slice::from_raw_parts(pre_v_global_start, pre_v_total_size) };
    
    {
        for i in (20..24).step_by(2) {
            let d0 = &x.block[i];
            let d1 = &x.block[i+1];
            if d0.bmi.mv().as_int() == d1.bmi.mv().as_int() {
                build_inter_predictors2b_safe(
                    subpixel_predict8x4,
                    d0,
                    &mut dst_v_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    &pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            } else {
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d0,
                    &mut dst_v_slice,
                    dst_uv_active_offset + d0.offset as usize,
                    dst_uv_stride,
                    &pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
                build_inter_predictors_b_safe(
                    subpixel_predict,
                    d1,
                    &mut dst_v_slice,
                    dst_uv_active_offset + d1.offset as usize,
                    dst_uv_stride,
                    &pre_v_slice,
                    pre_uv_active_offset,
                    pre_uv_stride,
                );
            }
        }
    }
}
fn build_4x4uvmvs(x: &mut MACROBLOCKD) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 2 as ::core::ffi::c_int {
            let mut yoffset: ::core::ffi::c_int =
                i * 8 as ::core::ffi::c_int + j * 2 as ::core::ffi::c_int;
            let mut uoffset: ::core::ffi::c_int =
                16 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int + j;
            let mut voffset: ::core::ffi::c_int =
                20 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int + j;
            let mut temp: ::core::ffi::c_int = 0;
            temp = x.mode_info().bmi[(yoffset + 0 as ::core::ffi::c_int) as usize]
                .mv()
                .as_mv()
                .row as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 1 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .row as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 4 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .row as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 5 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .row as ::core::ffi::c_int;
            temp += 4 as ::core::ffi::c_int
                + (temp
                    >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1 as usize))
                    * 8 as ::core::ffi::c_int;
            x.block[uoffset as usize].bmi.mv_mut().as_mv_mut().row =
                (temp / 8 as ::core::ffi::c_int & x.fullpixel_mask) as ::core::ffi::c_short;
            temp = x.mode_info().bmi[(yoffset + 0 as ::core::ffi::c_int) as usize]
                .mv()
                .as_mv()
                .col as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 1 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .col as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 4 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .col as ::core::ffi::c_int
                + x.mode_info().bmi[(yoffset + 5 as ::core::ffi::c_int) as usize]
                    .mv()
                    .as_mv()
                    .col as ::core::ffi::c_int;
            temp += 4 as ::core::ffi::c_int
                + (temp
                    >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1 as usize))
                    * 8 as ::core::ffi::c_int;
            x.block[uoffset as usize].bmi.mv_mut().as_mv_mut().col =
                (temp / 8 as ::core::ffi::c_int & x.fullpixel_mask) as ::core::ffi::c_short;
            if x.mode_info().mbmi.need_to_clamp_mvs != 0 {
                clamp_uvmv_to_umv_border(
                    x.block[uoffset as usize].bmi.mv_mut().as_mv_mut(),
                    x.mb_to_left_edge,
                    x.mb_to_right_edge,
                    x.mb_to_top_edge,
                    x.mb_to_bottom_edge,
                );
            }
            let u_mv = x.block[uoffset as usize].bmi.mv();
            *x.block[voffset as usize].bmi.mv_mut() = u_mv;
            j += 1;
        }
        i += 1;
    }
}
pub fn vp8_build_inter_predictors_mb(xd: &mut MACROBLOCKD) {
    if xd.mode_info().mbmi.mode as ::core::ffi::c_int != SPLITMV as ::core::ffi::c_int {
        vp8_build_inter16x16_predictors_mb(xd);
    } else {
        build_4x4uvmvs(xd);
        build_inter4x4_predictors_mb(xd);
    };
}
