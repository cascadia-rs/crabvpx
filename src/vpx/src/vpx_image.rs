unsafe extern "C" {
    fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vpx_memalign(align: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type vpx_img_fmt = ::core::ffi::c_uint;
pub const VPX_IMG_FMT_I44016: vpx_img_fmt = 2311;
pub const VPX_IMG_FMT_I44416: vpx_img_fmt = 2310;
pub const VPX_IMG_FMT_I42216: vpx_img_fmt = 2309;
pub const VPX_IMG_FMT_I42016: vpx_img_fmt = 2306;
pub const VPX_IMG_FMT_NV12: vpx_img_fmt = 265;
pub const VPX_IMG_FMT_I440: vpx_img_fmt = 263;
pub const VPX_IMG_FMT_I444: vpx_img_fmt = 262;
pub const VPX_IMG_FMT_I422: vpx_img_fmt = 261;
pub const VPX_IMG_FMT_I420: vpx_img_fmt = 258;
pub const VPX_IMG_FMT_YV12: vpx_img_fmt = 769;
pub const VPX_IMG_FMT_NONE: vpx_img_fmt = 0;
pub type vpx_img_fmt_t = vpx_img_fmt;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_image {
    pub fmt: vpx_img_fmt_t,
    pub cs: vpx_color_space_t,
    pub range: vpx_color_range_t,
    pub w: ::core::ffi::c_uint,
    pub h: ::core::ffi::c_uint,
    pub bit_depth: ::core::ffi::c_uint,
    pub d_w: ::core::ffi::c_uint,
    pub d_h: ::core::ffi::c_uint,
    pub r_w: ::core::ffi::c_uint,
    pub r_h: ::core::ffi::c_uint,
    pub x_chroma_shift: ::core::ffi::c_uint,
    pub y_chroma_shift: ::core::ffi::c_uint,
    pub planes: [*mut ::core::ffi::c_uchar; 4],
    pub stride: [::core::ffi::c_int; 4],
    pub bps: ::core::ffi::c_int,
    pub user_priv: *mut ::core::ffi::c_void,
    pub img_data: *mut ::core::ffi::c_uchar,
    pub img_data_owner: ::core::ffi::c_int,
    pub self_allocd: ::core::ffi::c_int,
    pub fb_priv: *mut ::core::ffi::c_void,
}
pub type vpx_image_t = vpx_image;
pub const UINT_MAX: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const VPX_IMG_FMT_PLANAR: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const VPX_IMG_FMT_UV_FLIP: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const VPX_IMG_FMT_HAS_ALPHA: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const VPX_IMG_FMT_HIGHBITDEPTH: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const VPX_PLANE_PACKED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VPX_PLANE_Y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VPX_PLANE_U: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VPX_PLANE_V: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VPX_PLANE_ALPHA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn is_valid_img_fmt(mut fmt: vpx_img_fmt_t) -> ::core::ffi::c_int {
    match fmt as ::core::ffi::c_uint {
        769 | 258 | 261 | 262 | 263 | 265 | 2306 | 2309 | 2310 | 2311 => 1 as ::core::ffi::c_int,
        _ => 0 as ::core::ffi::c_int,
    }
}
unsafe extern "C" fn img_alloc_helper(
    mut img: *mut vpx_image_t,
    mut fmt: vpx_img_fmt_t,
    mut d_w: ::core::ffi::c_uint,
    mut d_h: ::core::ffi::c_uint,
    mut buf_align: ::core::ffi::c_uint,
    mut stride_align: ::core::ffi::c_uint,
    mut img_data: *mut ::core::ffi::c_uchar,
) -> *mut vpx_image_t {
    unsafe {
        let mut _ret: ::core::ffi::c_int = 0;
        let mut current_block: u64;
        let mut h: ::core::ffi::c_uint = 0;
        let mut w: ::core::ffi::c_uint = 0;
        let mut xcs: ::core::ffi::c_uint = 0;
        let mut ycs: ::core::ffi::c_uint = 0;
        let mut bps: ::core::ffi::c_uint = 0;
        let mut s: uint64_t = 0;
        let mut stride_in_bytes: ::core::ffi::c_int = 0;
        let mut align: ::core::ffi::c_uint = 0;
        if !img.is_null() {
            memset(
                img as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<vpx_image_t>() as size_t,
            );
        }
        if !(is_valid_img_fmt(fmt) == 0)
            && !(d_w > 0x8000000 as ::core::ffi::c_int as ::core::ffi::c_uint
                || d_h > 0x8000000 as ::core::ffi::c_int as ::core::ffi::c_uint
                || buf_align > 65536 as ::core::ffi::c_int as ::core::ffi::c_uint
                || stride_align > 65536 as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if buf_align == 0 {
                buf_align = 1 as ::core::ffi::c_uint;
            }
            if !(buf_align & buf_align.wrapping_sub(1 as ::core::ffi::c_uint) != 0) {
                if stride_align == 0 {
                    stride_align = 1 as ::core::ffi::c_uint;
                }
                if !(stride_align & stride_align.wrapping_sub(1 as ::core::ffi::c_uint) != 0) {
                    match fmt as ::core::ffi::c_uint {
                        258 | 769 | 265 => {
                            bps = 12 as ::core::ffi::c_uint;
                        }
                        261 | 263 => {
                            bps = 16 as ::core::ffi::c_uint;
                        }
                        262 => {
                            bps = 24 as ::core::ffi::c_uint;
                        }
                        2306 => {
                            bps = 24 as ::core::ffi::c_uint;
                        }
                        2309 | 2311 => {
                            bps = 32 as ::core::ffi::c_uint;
                        }
                        2310 => {
                            bps = 48 as ::core::ffi::c_uint;
                        }
                        _ => {
                            bps = 16 as ::core::ffi::c_uint;
                        }
                    }
                    match fmt as ::core::ffi::c_uint {
                        258 | 769 | 261 | 2306 | 2309 => {
                            xcs = 1 as ::core::ffi::c_uint;
                        }
                        _ => {
                            xcs = 0 as ::core::ffi::c_uint;
                        }
                    }
                    match fmt as ::core::ffi::c_uint {
                        258 | 265 | 263 | 769 | 2306 | 2311 => {
                            ycs = 1 as ::core::ffi::c_uint;
                        }
                        _ => {
                            ycs = 0 as ::core::ffi::c_uint;
                        }
                    }
                    if !img_data.is_null() {
                        w = d_w;
                        h = d_h;
                    } else {
                        align = (((1 as ::core::ffi::c_int) << xcs) - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint;
                        w = d_w.wrapping_add(align) & !align;
                        align = (((1 as ::core::ffi::c_int) << ycs) - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint;
                        h = d_h.wrapping_add(align) & !align;
                    }
                    s = if fmt as ::core::ffi::c_uint & VPX_IMG_FMT_PLANAR as ::core::ffi::c_uint
                        != 0
                    {
                        w as uint64_t
                    } else {
                        (bps as uint64_t)
                            .wrapping_mul(w as uint64_t)
                            .wrapping_div(8 as uint64_t)
                    };
                    s = if fmt as ::core::ffi::c_uint
                        & VPX_IMG_FMT_HIGHBITDEPTH as ::core::ffi::c_uint
                        != 0
                    {
                        s.wrapping_mul(2 as uint64_t)
                    } else {
                        s
                    };
                    s = s
                        .wrapping_add(stride_align as uint64_t)
                        .wrapping_sub(1 as uint64_t)
                        & !(stride_align as uint64_t).wrapping_sub(1 as uint64_t);
                    if !(s > INT_MAX as uint64_t) {
                        stride_in_bytes = s as ::core::ffi::c_int;
                        s = if fmt as ::core::ffi::c_uint
                            & VPX_IMG_FMT_HIGHBITDEPTH as ::core::ffi::c_uint
                            != 0
                        {
                            s.wrapping_div(2 as uint64_t)
                        } else {
                            s
                        };
                        if img.is_null() {
                            img = calloc(
                                1 as size_t,
                                ::core::mem::size_of::<vpx_image_t>() as size_t,
                            ) as *mut vpx_image_t;
                            if img.is_null() {
                                current_block = 7960401837942226685;
                            } else {
                                (*img).self_allocd = 1 as ::core::ffi::c_int;
                                current_block = 13678349939556791712;
                            }
                        } else {
                            current_block = 13678349939556791712;
                        }
                        match current_block {
                            7960401837942226685 => {}
                            _ => {
                                (*img).img_data = img_data;
                                if img_data.is_null() {
                                    let mut alloc_size: uint64_t = 0;
                                    alloc_size = if fmt as ::core::ffi::c_uint
                                        & VPX_IMG_FMT_PLANAR as ::core::ffi::c_uint
                                        != 0
                                    {
                                        (h as uint64_t)
                                            .wrapping_mul(s)
                                            .wrapping_mul(bps as uint64_t)
                                            .wrapping_div(8 as uint64_t)
                                    } else {
                                        (h as uint64_t).wrapping_mul(s)
                                    };
                                    if alloc_size != alloc_size as size_t as uint64_t {
                                        current_block = 7960401837942226685;
                                    } else {
                                        (*img).img_data =
                                            vpx_memalign(buf_align as size_t, alloc_size as size_t)
                                                as *mut uint8_t
                                                as *mut ::core::ffi::c_uchar;
                                        (*img).img_data_owner = 1 as ::core::ffi::c_int;
                                        current_block = 17233182392562552756;
                                    }
                                } else {
                                    current_block = 17233182392562552756;
                                }
                                match current_block {
                                    7960401837942226685 => {}
                                    _ => {
                                        if !(*img).img_data.is_null() {
                                            (*img).fmt = fmt;
                                            (*img).bit_depth = (if fmt as ::core::ffi::c_uint
                                                & VPX_IMG_FMT_HIGHBITDEPTH as ::core::ffi::c_uint
                                                != 0
                                            {
                                                16 as ::core::ffi::c_int
                                            } else {
                                                8 as ::core::ffi::c_int
                                            })
                                                as ::core::ffi::c_uint;
                                            (*img).w = w;
                                            (*img).h = h;
                                            (*img).x_chroma_shift = xcs;
                                            (*img).y_chroma_shift = ycs;
                                            (*img).bps = bps as ::core::ffi::c_int;
                                            (*img).stride[VPX_PLANE_ALPHA as usize] =
                                                stride_in_bytes;
                                            (*img).stride[VPX_PLANE_Y as usize] =
                                                (*img).stride[VPX_PLANE_ALPHA as usize];
                                            (*img).stride[VPX_PLANE_V as usize] =
                                                stride_in_bytes >> xcs;
                                            (*img).stride[VPX_PLANE_U as usize] =
                                                (*img).stride[VPX_PLANE_V as usize];
                                            _ret = vpx_img_set_rect(
                                                img,
                                                0 as ::core::ffi::c_uint,
                                                0 as ::core::ffi::c_uint,
                                                d_w,
                                                d_h,
                                            );
                                            return img;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        vpx_img_free(img);
        ::core::ptr::null_mut::<vpx_image_t>()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_img_alloc(
    mut img: *mut vpx_image_t,
    mut fmt: vpx_img_fmt_t,
    mut d_w: ::core::ffi::c_uint,
    mut d_h: ::core::ffi::c_uint,
    mut align: ::core::ffi::c_uint,
) -> *mut vpx_image_t {
    unsafe {
        img_alloc_helper(
            img,
            fmt,
            d_w,
            d_h,
            align,
            align,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        )
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_img_wrap(
    mut img: *mut vpx_image_t,
    mut fmt: vpx_img_fmt_t,
    mut d_w: ::core::ffi::c_uint,
    mut d_h: ::core::ffi::c_uint,
    mut stride_align: ::core::ffi::c_uint,
    mut img_data: *mut ::core::ffi::c_uchar,
) -> *mut vpx_image_t {
    unsafe {
        img_alloc_helper(
            img,
            fmt,
            d_w,
            d_h,
            1 as ::core::ffi::c_uint,
            stride_align,
            img_data,
        )
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_img_set_rect(
    mut img: *mut vpx_image_t,
    mut x: ::core::ffi::c_uint,
    mut y: ::core::ffi::c_uint,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        if x <= UINT_MAX.wrapping_sub(w)
            && x.wrapping_add(w) <= (*img).w
            && y <= UINT_MAX.wrapping_sub(h)
            && y.wrapping_add(h) <= (*img).h
        {
            (*img).d_w = w;
            (*img).d_h = h;
            if (*img).fmt as ::core::ffi::c_uint & VPX_IMG_FMT_PLANAR as ::core::ffi::c_uint == 0 {
                (*img).planes[VPX_PLANE_PACKED as usize] = (*img)
                    .img_data
                    .offset(
                        x.wrapping_mul((*img).bps as ::core::ffi::c_uint)
                            .wrapping_div(8 as ::core::ffi::c_uint)
                            as isize,
                    )
                    .offset(y.wrapping_mul(
                        (*img).stride[VPX_PLANE_PACKED as usize] as ::core::ffi::c_uint,
                    ) as isize);
            } else {
                let bytes_per_sample: ::core::ffi::c_int = if (*img).fmt as ::core::ffi::c_uint
                    & VPX_IMG_FMT_HIGHBITDEPTH as ::core::ffi::c_uint
                    != 0
                {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                let mut data: *mut ::core::ffi::c_uchar = (*img).img_data;
                if (*img).fmt as ::core::ffi::c_uint & VPX_IMG_FMT_HAS_ALPHA as ::core::ffi::c_uint
                    != 0
                {
                    (*img).planes[VPX_PLANE_ALPHA as usize] = data
                        .offset(x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(y.wrapping_mul(
                            (*img).stride[VPX_PLANE_ALPHA as usize] as ::core::ffi::c_uint,
                        ) as isize);
                    data = data.add(
                        ((*img).h as size_t)
                            .wrapping_mul((*img).stride[VPX_PLANE_ALPHA as usize] as size_t),
                    );
                }
                (*img).planes[VPX_PLANE_Y as usize] =
                    data.offset(x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(y.wrapping_mul(
                            (*img).stride[VPX_PLANE_Y as usize] as ::core::ffi::c_uint,
                        ) as isize);
                data = data.add(
                    ((*img).h as size_t)
                        .wrapping_mul((*img).stride[VPX_PLANE_Y as usize] as size_t),
                );
                let mut uv_x: ::core::ffi::c_uint = x >> (*img).x_chroma_shift;
                let mut uv_y: ::core::ffi::c_uint = y >> (*img).y_chroma_shift;
                if (*img).fmt as ::core::ffi::c_uint
                    == VPX_IMG_FMT_NV12 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*img).planes[VPX_PLANE_U as usize] =
                        data.offset(uv_x as isize).offset(uv_y.wrapping_mul(
                            (*img).stride[VPX_PLANE_U as usize] as ::core::ffi::c_uint,
                        ) as isize);
                    (*img).planes[VPX_PLANE_V as usize] = (*img).planes[VPX_PLANE_U as usize]
                        .offset(1 as ::core::ffi::c_int as isize);
                } else if (*img).fmt as ::core::ffi::c_uint
                    & VPX_IMG_FMT_UV_FLIP as ::core::ffi::c_uint
                    == 0
                {
                    (*img).planes[VPX_PLANE_U as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(uv_y.wrapping_mul(
                            (*img).stride[VPX_PLANE_U as usize] as ::core::ffi::c_uint,
                        ) as isize);
                    data = data.add(
                        (((*img).h >> (*img).y_chroma_shift) as size_t)
                            .wrapping_mul((*img).stride[VPX_PLANE_U as usize] as size_t),
                    );
                    (*img).planes[VPX_PLANE_V as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(uv_y.wrapping_mul(
                            (*img).stride[VPX_PLANE_V as usize] as ::core::ffi::c_uint,
                        ) as isize);
                } else {
                    (*img).planes[VPX_PLANE_V as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(uv_y.wrapping_mul(
                            (*img).stride[VPX_PLANE_V as usize] as ::core::ffi::c_uint,
                        ) as isize);
                    data = data.add(
                        (((*img).h >> (*img).y_chroma_shift) as size_t)
                            .wrapping_mul((*img).stride[VPX_PLANE_V as usize] as size_t),
                    );
                    (*img).planes[VPX_PLANE_U as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as ::core::ffi::c_uint) as isize)
                        .offset(uv_y.wrapping_mul(
                            (*img).stride[VPX_PLANE_U as usize] as ::core::ffi::c_uint,
                        ) as isize);
                }
            }
            return 0 as ::core::ffi::c_int;
        }
        -(1 as ::core::ffi::c_int)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_img_flip(mut img: *mut vpx_image_t) {
    unsafe {
        (*img).planes[VPX_PLANE_Y as usize] = (*img).planes[VPX_PLANE_Y as usize].offset(
            ((*img).d_h.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int
                * (*img).stride[VPX_PLANE_Y as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_Y as usize] = -(*img).stride[VPX_PLANE_Y as usize];
        (*img).planes[VPX_PLANE_U as usize] = (*img).planes[VPX_PLANE_U as usize].offset(
            (((*img).d_h >> (*img).y_chroma_shift).wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_int
                * (*img).stride[VPX_PLANE_U as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_U as usize] = -(*img).stride[VPX_PLANE_U as usize];
        (*img).planes[VPX_PLANE_V as usize] = (*img).planes[VPX_PLANE_V as usize].offset(
            (((*img).d_h >> (*img).y_chroma_shift).wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_int
                * (*img).stride[VPX_PLANE_V as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_V as usize] = -(*img).stride[VPX_PLANE_V as usize];
        (*img).planes[VPX_PLANE_ALPHA as usize] = (*img).planes[VPX_PLANE_ALPHA as usize].offset(
            ((*img).d_h.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int
                * (*img).stride[VPX_PLANE_ALPHA as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_ALPHA as usize] = -(*img).stride[VPX_PLANE_ALPHA as usize];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vpx_img_free(mut img: *mut vpx_image_t) {
    unsafe {
        if !img.is_null() {
            if !(*img).img_data.is_null() && (*img).img_data_owner != 0 {
                vpx_free((*img).img_data as *mut ::core::ffi::c_void);
            }
            if (*img).self_allocd != 0 {
                free(img as *mut ::core::ffi::c_void);
            }
        }
    }
}
