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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem16x16_c(
    mut src: *mut ::core::ffi::c_uchar,
    mut src_stride: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) { unsafe {
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 16 as ::core::ffi::c_int {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            16 as size_t,
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        r += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x8_c(
    mut src: *mut ::core::ffi::c_uchar,
    mut src_stride: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) { unsafe {
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 8 as ::core::ffi::c_int {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            8 as size_t,
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        r += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_mem8x4_c(
    mut src: *mut ::core::ffi::c_uchar,
    mut src_stride: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
) { unsafe {
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < 4 as ::core::ffi::c_int {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            8 as size_t,
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        r += 1;
    }
}}

unsafe fn build_inter_predictors4b(
    x: &MACROBLOCKD,
    d: &BLOCKD,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
    base_pre: *mut ::core::ffi::c_uchar,
    pre_stride: ::core::ffi::c_int,
) { unsafe {
    let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = base_pre
        .offset(d.offset as isize)
        .offset(
            (((d.bmi.mv.as_mv.row as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) * pre_stride)
                as isize),
        )
        .offset((d.bmi.mv.as_mv.col as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize);
    if d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
        || d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
    {
        x.subpixel_predict8x8.expect("non-null function pointer")(
            ptr,
            pre_stride,
            d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst,
            dst_stride,
        );
    } else {
        vp8_copy_mem8x8_neon(ptr, pre_stride, dst, dst_stride);
    };
}}
unsafe fn build_inter_predictors2b(
    x: &MACROBLOCKD,
    d: &BLOCKD,
    dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
    base_pre: *mut ::core::ffi::c_uchar,
    pre_stride: ::core::ffi::c_int,
) { unsafe {
    let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = base_pre
        .offset(d.offset as isize)
        .offset(
            (((d.bmi.mv.as_mv.row as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) * pre_stride)
                as isize),
        )
        .offset((d.bmi.mv.as_mv.col as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize);
    if d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
        || d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
    {
        x.subpixel_predict8x4.expect("non-null function pointer")(
            ptr,
            pre_stride,
            d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst,
            dst_stride,
        );
    } else {
        vp8_copy_mem8x4_neon(ptr, pre_stride, dst, dst_stride);
    };
}}
unsafe fn build_inter_predictors_b(
    d: &BLOCKD,
    mut dst: *mut ::core::ffi::c_uchar,
    dst_stride: ::core::ffi::c_int,
    base_pre: *mut ::core::ffi::c_uchar,
    pre_stride: ::core::ffi::c_int,
    sppf: vp8_subpix_fn_t,
) { unsafe {
    let mut r: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = base_pre
        .offset(d.offset as isize)
        .offset(
            (((d.bmi.mv.as_mv.row as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) * pre_stride)
                as isize),
        )
        .offset((d.bmi.mv.as_mv.col as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize);
    if d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
        || d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int != 0
    {
        sppf.expect("non-null function pointer")(
            ptr,
            pre_stride,
            d.bmi.mv.as_mv.col as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            d.bmi.mv.as_mv.row as ::core::ffi::c_int & 7 as ::core::ffi::c_int,
            dst,
            dst_stride,
        );
    } else {
        r = 0 as ::core::ffi::c_int;
        while r < 4 as ::core::ffi::c_int {
            *dst.offset(0 as ::core::ffi::c_int as isize) =
                *ptr.offset(0 as ::core::ffi::c_int as isize);
            *dst.offset(1 as ::core::ffi::c_int as isize) =
                *ptr.offset(1 as ::core::ffi::c_int as isize);
            *dst.offset(2 as ::core::ffi::c_int as isize) =
                *ptr.offset(2 as ::core::ffi::c_int as isize);
            *dst.offset(3 as ::core::ffi::c_int as isize) =
                *ptr.offset(3 as ::core::ffi::c_int as isize);
            dst = dst.offset(dst_stride as isize);
            ptr = ptr.offset(pre_stride as isize);
            r += 1;
        }
    };
}}
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
fn build_inter4x4_predictors_mb(x: &mut MACROBLOCKD) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut base_dst: *mut ::core::ffi::c_uchar = x.dst.y_buffer as *mut ::core::ffi::c_uchar;
    let mut base_pre: *mut ::core::ffi::c_uchar = x.pre.y_buffer as *mut ::core::ffi::c_uchar;
    if (x.mode_info().mbmi.partitioning as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
    {
        let mut dst_stride: ::core::ffi::c_int = x.dst.y_stride;
        x.block[0 as ::core::ffi::c_int as usize].bmi =
            x.mode_info().bmi[0 as ::core::ffi::c_int as usize];
        x.block[2 as ::core::ffi::c_int as usize].bmi =
            x.mode_info().bmi[2 as ::core::ffi::c_int as usize];
        x.block[8 as ::core::ffi::c_int as usize].bmi =
            x.mode_info().bmi[8 as ::core::ffi::c_int as usize];
        x.block[10 as ::core::ffi::c_int as usize].bmi =
            x.mode_info().bmi[10 as ::core::ffi::c_int as usize];
        if x.mode_info().mbmi.need_to_clamp_mvs != 0 {
            clamp_mv_to_umv_border(
                &mut x.block[0].bmi.mv.as_mv,
                x.mb_to_left_edge,
                x.mb_to_right_edge,
                x.mb_to_top_edge,
                x.mb_to_bottom_edge,
            );
            clamp_mv_to_umv_border(
                &mut x.block[2].bmi.mv.as_mv,
                x.mb_to_left_edge,
                x.mb_to_right_edge,
                x.mb_to_top_edge,
                x.mb_to_bottom_edge,
            );
            clamp_mv_to_umv_border(
                &mut x.block[8].bmi.mv.as_mv,
                x.mb_to_left_edge,
                x.mb_to_right_edge,
                x.mb_to_top_edge,
                x.mb_to_bottom_edge,
            );
            clamp_mv_to_umv_border(
                &mut x.block[10].bmi.mv.as_mv,
                x.mb_to_left_edge,
                x.mb_to_right_edge,
                x.mb_to_top_edge,
                x.mb_to_bottom_edge,
            );
        }
        let mut b = &x.block[0];
        build_inter_predictors4b(
            x,
            b,
            base_dst.offset(b.offset as isize),
            dst_stride,
            base_pre,
            dst_stride,
        );
        b = &x.block[2];
        build_inter_predictors4b(
            x,
            b,
            base_dst.offset(b.offset as isize),
            dst_stride,
            base_pre,
            dst_stride,
        );
        b = &x.block[8];
        build_inter_predictors4b(
            x,
            b,
            base_dst.offset(b.offset as isize),
            dst_stride,
            base_pre,
            dst_stride,
        );
        b = &x.block[10];
        build_inter_predictors4b(
            x,
            b,
            base_dst.offset(b.offset as isize),
            dst_stride,
            base_pre,
            dst_stride,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            let mut dst_stride_0: ::core::ffi::c_int = x.dst.y_stride;
            x.block[(i + 0 as ::core::ffi::c_int) as usize].bmi =
                x.mode_info().bmi[(i + 0 as ::core::ffi::c_int) as usize];
            x.block[(i + 1 as ::core::ffi::c_int) as usize].bmi =
                x.mode_info().bmi[(i + 1 as ::core::ffi::c_int) as usize];
            if x.mode_info().mbmi.need_to_clamp_mvs != 0 {
                clamp_mv_to_umv_border(
                    &mut x.block[(i + 0) as usize].bmi.mv.as_mv,
                    x.mb_to_left_edge,
                    x.mb_to_right_edge,
                    x.mb_to_top_edge,
                    x.mb_to_bottom_edge,
                );
                clamp_mv_to_umv_border(
                    &mut x.block[(i + 1) as usize].bmi.mv.as_mv,
                    x.mb_to_left_edge,
                    x.mb_to_right_edge,
                    x.mb_to_top_edge,
                    x.mb_to_bottom_edge,
                );
            }
            let d0 = &x.block[i as usize];
            let d1 = &x.block[(i + 1) as usize];
            if d0.bmi.mv.as_int == d1.bmi.mv.as_int {
                build_inter_predictors2b(
                    x,
                    d0,
                    base_dst.offset(d0.offset as isize),
                    dst_stride_0,
                    base_pre,
                    dst_stride_0,
                );
            } else {
                build_inter_predictors_b(
                    d0,
                    base_dst.offset(d0.offset as isize),
                    dst_stride_0,
                    base_pre,
                    dst_stride_0,
                    x.subpixel_predict,
                );
                build_inter_predictors_b(
                    d1,
                    base_dst.offset(d1.offset as isize),
                    dst_stride_0,
                    base_pre,
                    dst_stride_0,
                    x.subpixel_predict,
                );
            }
            i += 2 as ::core::ffi::c_int;
        }
    }
    base_dst = x.dst.u_buffer as *mut ::core::ffi::c_uchar;
    base_pre = x.pre.u_buffer as *mut ::core::ffi::c_uchar;
    i = 16 as ::core::ffi::c_int;
    while i < 20 as ::core::ffi::c_int {
        let d0_0 = &x.block[i as usize];
        let d1_0 = &x.block[(i + 1) as usize];
        let mut dst_stride_1: ::core::ffi::c_int = x.dst.uv_stride;
        if d0_0.bmi.mv.as_int == d1_0.bmi.mv.as_int {
            build_inter_predictors2b(
                x,
                d0_0,
                base_dst.offset(d0_0.offset as isize),
                dst_stride_1,
                base_pre,
                dst_stride_1,
            );
        } else {
            build_inter_predictors_b(
                d0_0,
                base_dst.offset(d0_0.offset as isize),
                dst_stride_1,
                base_pre,
                dst_stride_1,
                x.subpixel_predict,
            );
            build_inter_predictors_b(
                d1_0,
                base_dst.offset(d1_0.offset as isize),
                dst_stride_1,
                base_pre,
                dst_stride_1,
                x.subpixel_predict,
            );
        }
        i += 2 as ::core::ffi::c_int;
    }
    base_dst = x.dst.v_buffer as *mut ::core::ffi::c_uchar;
    base_pre = x.pre.v_buffer as *mut ::core::ffi::c_uchar;
    i = 20 as ::core::ffi::c_int;
    while i < 24 as ::core::ffi::c_int {
        let d0_1 = &x.block[i as usize];
        let d1_1 = &x.block[(i + 1) as usize];
        let mut dst_stride_2: ::core::ffi::c_int = x.dst.uv_stride;
        if d0_1.bmi.mv.as_int == d1_1.bmi.mv.as_int {
            build_inter_predictors2b(
                x,
                d0_1,
                base_dst.offset(d0_1.offset as isize),
                dst_stride_2,
                base_pre,
                dst_stride_2,
            );
        } else {
            build_inter_predictors_b(
                d0_1,
                base_dst.offset(d0_1.offset as isize),
                dst_stride_2,
                base_pre,
                dst_stride_2,
                x.subpixel_predict,
            );
            build_inter_predictors_b(
                d1_1,
                base_dst.offset(d1_1.offset as isize),
                dst_stride_2,
                base_pre,
                dst_stride_2,
                x.subpixel_predict,
            );
        }
        i += 2 as ::core::ffi::c_int;
    }
}}
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
