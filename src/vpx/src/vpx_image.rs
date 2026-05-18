use std::ffi::c_void;
unsafe extern "Rust" {
    fn calloc(__count: SizeT, __size: SizeT) -> *mut c_void;
    fn free(_: *mut c_void);
    fn vpx_memalign(align: SizeT, size: SizeT) -> *mut c_void;
    fn vpx_free(memblk: *mut c_void);
}
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
pub const UINT_MAX: u32 = 0xffffffff as u32;
pub const INT_MAX: i32 = 2147483647 as i32;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const VPX_IMG_FMT_PLANAR: i32 = 0x100 as i32;
pub const VPX_IMG_FMT_UV_FLIP: i32 = 0x200 as i32;
pub const VPX_IMG_FMT_HAS_ALPHA: i32 = 0x400 as i32;
pub const VPX_IMG_FMT_HIGHBITDEPTH: i32 = 0x800 as i32;
pub const VPX_PLANE_PACKED: i32 = 0 as i32;
pub const VPX_PLANE_Y: i32 = 0 as i32;
pub const VPX_PLANE_U: i32 = 1 as i32;
pub const VPX_PLANE_V: i32 = 2 as i32;
pub const VPX_PLANE_ALPHA: i32 = 3 as i32;
unsafe fn is_valid_img_fmt(mut fmt: VpxImgFmtT) -> i32 {
    match fmt as u32 {
        769 | 258 | 261 | 262 | 263 | 265 | 2306 | 2309 | 2310 | 2311 => 1 as i32,
        _ => 0 as i32,
    }
}
unsafe fn img_alloc_helper(
    mut img: *mut VpxImageT,
    mut fmt: VpxImgFmtT,
    mut d_w: u32,
    mut d_h: u32,
    mut buf_align: u32,
    mut stride_align: u32,
    mut img_data: *mut u8,
) -> *mut VpxImageT {
    unsafe {
        let mut _ret: i32 = 0;
        let mut current_block: u64;
        let mut h: u32 = 0;
        let mut w: u32 = 0;
        let mut xcs: u32 = 0;
        let mut ycs: u32 = 0;
        let mut bps: u32 = 0;
        let mut s: u64 = 0;
        let mut stride_in_bytes: i32 = 0;
        let mut align: u32 = 0;
        if !img.is_null() {
            core::ptr::write_bytes(
                img as *mut c_void as *mut u8,
                0 as i32 as u8,
                ::core::mem::size_of::<VpxImageT>() as SizeT,
            );
        }
        if !(is_valid_img_fmt(fmt) == 0)
            && !(d_w > 0x8000000 as u32
                || d_h > 0x8000000 as u32
                || buf_align > 65536 as u32
                || stride_align > 65536 as u32)
        {
            if buf_align == 0 {
                buf_align = 1 as u32;
            }
            if !(buf_align & buf_align.wrapping_sub(1 as u32) != 0) {
                if stride_align == 0 {
                    stride_align = 1 as u32;
                }
                if !(stride_align & stride_align.wrapping_sub(1 as u32) != 0) {
                    match fmt as u32 {
                        258 | 769 | 265 => {
                            bps = 12 as u32;
                        }
                        261 | 263 => {
                            bps = 16 as u32;
                        }
                        262 => {
                            bps = 24 as u32;
                        }
                        2306 => {
                            bps = 24 as u32;
                        }
                        2309 | 2311 => {
                            bps = 32 as u32;
                        }
                        2310 => {
                            bps = 48 as u32;
                        }
                        _ => {
                            bps = 16 as u32;
                        }
                    }
                    match fmt as u32 {
                        258 | 769 | 261 | 2306 | 2309 => {
                            xcs = 1 as u32;
                        }
                        _ => {
                            xcs = 0 as u32;
                        }
                    }
                    match fmt as u32 {
                        258 | 265 | 263 | 769 | 2306 | 2311 => {
                            ycs = 1 as u32;
                        }
                        _ => {
                            ycs = 0 as u32;
                        }
                    }
                    if !img_data.is_null() {
                        w = d_w;
                        h = d_h;
                    } else {
                        align = (((1 as i32) << xcs) - 1 as i32) as u32;
                        w = d_w.wrapping_add(align) & !align;
                        align = (((1 as i32) << ycs) - 1 as i32) as u32;
                        h = d_h.wrapping_add(align) & !align;
                    }
                    s = if fmt as u32 & VPX_IMG_FMT_PLANAR as u32 != 0 {
                        w as u64
                    } else {
                        (bps as u64).wrapping_mul(w as u64).wrapping_div(8 as u64)
                    };
                    s = if fmt as u32 & VPX_IMG_FMT_HIGHBITDEPTH as u32 != 0 {
                        s.wrapping_mul(2 as u64)
                    } else {
                        s
                    };
                    s = s.wrapping_add(stride_align as u64).wrapping_sub(1 as u64)
                        & !(stride_align as u64).wrapping_sub(1 as u64);
                    if !(s > INT_MAX as u64) {
                        stride_in_bytes = s as i32;
                        s = if fmt as u32 & VPX_IMG_FMT_HIGHBITDEPTH as u32 != 0 {
                            s.wrapping_div(2 as u64)
                        } else {
                            s
                        };
                        if img.is_null() {
                            img = calloc(1 as SizeT, ::core::mem::size_of::<VpxImageT>() as SizeT)
                                as *mut VpxImageT;
                            if img.is_null() {
                                current_block = 7960401837942226685;
                            } else {
                                (*img).self_allocd = 1 as i32;
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
                                    let mut alloc_size: u64 = 0;
                                    alloc_size = if fmt as u32 & VPX_IMG_FMT_PLANAR as u32 != 0 {
                                        (h as u64)
                                            .wrapping_mul(s)
                                            .wrapping_mul(bps as u64)
                                            .wrapping_div(8 as u64)
                                    } else {
                                        (h as u64).wrapping_mul(s)
                                    };
                                    if alloc_size != alloc_size as u64 {
                                        current_block = 7960401837942226685;
                                    } else {
                                        (*img).img_data =
                                            vpx_memalign(buf_align as SizeT, alloc_size as SizeT)
                                                as *mut u8
                                                as *mut u8;
                                        (*img).img_data_owner = 1 as i32;
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
                                            (*img).bit_depth = (if fmt as u32
                                                & VPX_IMG_FMT_HIGHBITDEPTH as u32
                                                != 0
                                            {
                                                16 as i32
                                            } else {
                                                8 as i32
                                            })
                                                as u32;
                                            (*img).w = w;
                                            (*img).h = h;
                                            (*img).x_chroma_shift = xcs;
                                            (*img).y_chroma_shift = ycs;
                                            (*img).bps = bps as i32;
                                            (*img).stride[VPX_PLANE_ALPHA as usize] =
                                                stride_in_bytes;
                                            (*img).stride[VPX_PLANE_Y as usize] =
                                                (*img).stride[VPX_PLANE_ALPHA as usize];
                                            (*img).stride[VPX_PLANE_V as usize] =
                                                stride_in_bytes >> xcs;
                                            (*img).stride[VPX_PLANE_U as usize] =
                                                (*img).stride[VPX_PLANE_V as usize];
                                            _ret =
                                                vpx_img_set_rect(img, 0 as u32, 0 as u32, d_w, d_h);
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
        ::core::ptr::null_mut::<VpxImageT>()
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_img_alloc(
    mut img: *mut VpxImageT,
    mut fmt: VpxImgFmtT,
    mut d_w: u32,
    mut d_h: u32,
    mut align: u32,
) -> *mut VpxImageT {
    unsafe {
        img_alloc_helper(
            img,
            fmt,
            d_w,
            d_h,
            align,
            align,
            ::core::ptr::null_mut::<u8>(),
        )
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_img_wrap(
    mut img: *mut VpxImageT,
    mut fmt: VpxImgFmtT,
    mut d_w: u32,
    mut d_h: u32,
    mut stride_align: u32,
    mut img_data: *mut u8,
) -> *mut VpxImageT {
    unsafe { img_alloc_helper(img, fmt, d_w, d_h, 1 as u32, stride_align, img_data) }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_img_set_rect(
    mut img: *mut VpxImageT,
    mut x: u32,
    mut y: u32,
    mut w: u32,
    mut h: u32,
) -> i32 {
    unsafe {
        if x <= UINT_MAX.wrapping_sub(w)
            && x.wrapping_add(w) <= (*img).w
            && y <= UINT_MAX.wrapping_sub(h)
            && y.wrapping_add(h) <= (*img).h
        {
            (*img).d_w = w;
            (*img).d_h = h;
            if (*img).fmt as u32 & VPX_IMG_FMT_PLANAR as u32 == 0 {
                (*img).planes[VPX_PLANE_PACKED as usize] = (*img)
                    .img_data
                    .offset(x.wrapping_mul((*img).bps as u32).wrapping_div(8 as u32) as isize)
                    .offset(
                        y.wrapping_mul((*img).stride[VPX_PLANE_PACKED as usize] as u32) as isize,
                    );
            } else {
                let bytes_per_sample: i32 =
                    if (*img).fmt as u32 & VPX_IMG_FMT_HIGHBITDEPTH as u32 != 0 {
                        2 as i32
                    } else {
                        1 as i32
                    };
                let mut data: *mut u8 = (*img).img_data;
                if (*img).fmt as u32 & VPX_IMG_FMT_HAS_ALPHA as u32 != 0 {
                    (*img).planes[VPX_PLANE_ALPHA as usize] = data
                        .offset(x.wrapping_mul(bytes_per_sample as u32) as isize)
                        .offset(
                            y.wrapping_mul((*img).stride[VPX_PLANE_ALPHA as usize] as u32) as isize,
                        );
                    data = data.add(
                        ((*img).h as SizeT)
                            .wrapping_mul((*img).stride[VPX_PLANE_ALPHA as usize] as SizeT),
                    );
                }
                (*img).planes[VPX_PLANE_Y as usize] = data
                    .offset(x.wrapping_mul(bytes_per_sample as u32) as isize)
                    .offset(y.wrapping_mul((*img).stride[VPX_PLANE_Y as usize] as u32) as isize);
                data = data.add(
                    ((*img).h as SizeT).wrapping_mul((*img).stride[VPX_PLANE_Y as usize] as SizeT),
                );
                let mut uv_x: u32 = x >> (*img).x_chroma_shift;
                let mut uv_y: u32 = y >> (*img).y_chroma_shift;
                if (*img).fmt as u32 == VPX_IMG_FMT_NV12 as u32 {
                    (*img).planes[VPX_PLANE_U as usize] = data.offset(uv_x as isize).offset(
                        uv_y.wrapping_mul((*img).stride[VPX_PLANE_U as usize] as u32) as isize,
                    );
                    (*img).planes[VPX_PLANE_V as usize] =
                        (*img).planes[VPX_PLANE_U as usize].offset(1 as isize);
                } else if (*img).fmt as u32 & VPX_IMG_FMT_UV_FLIP as u32 == 0 {
                    (*img).planes[VPX_PLANE_U as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as u32) as isize)
                        .offset(
                            uv_y.wrapping_mul((*img).stride[VPX_PLANE_U as usize] as u32) as isize,
                        );
                    data = data.add(
                        (((*img).h >> (*img).y_chroma_shift) as SizeT)
                            .wrapping_mul((*img).stride[VPX_PLANE_U as usize] as SizeT),
                    );
                    (*img).planes[VPX_PLANE_V as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as u32) as isize)
                        .offset(
                            uv_y.wrapping_mul((*img).stride[VPX_PLANE_V as usize] as u32) as isize,
                        );
                } else {
                    (*img).planes[VPX_PLANE_V as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as u32) as isize)
                        .offset(
                            uv_y.wrapping_mul((*img).stride[VPX_PLANE_V as usize] as u32) as isize,
                        );
                    data = data.add(
                        (((*img).h >> (*img).y_chroma_shift) as SizeT)
                            .wrapping_mul((*img).stride[VPX_PLANE_V as usize] as SizeT),
                    );
                    (*img).planes[VPX_PLANE_U as usize] = data
                        .offset(uv_x.wrapping_mul(bytes_per_sample as u32) as isize)
                        .offset(
                            uv_y.wrapping_mul((*img).stride[VPX_PLANE_U as usize] as u32) as isize,
                        );
                }
            }
            return 0 as i32;
        }
        -(1 as i32)
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_img_flip(mut img: *mut VpxImageT) {
    unsafe {
        (*img).planes[VPX_PLANE_Y as usize] = (*img).planes[VPX_PLANE_Y as usize].offset(
            ((*img).d_h.wrapping_sub(1 as u32) as i32 * (*img).stride[VPX_PLANE_Y as usize])
                as isize,
        );
        (*img).stride[VPX_PLANE_Y as usize] = -(*img).stride[VPX_PLANE_Y as usize];
        (*img).planes[VPX_PLANE_U as usize] = (*img).planes[VPX_PLANE_U as usize].offset(
            (((*img).d_h >> (*img).y_chroma_shift).wrapping_sub(1 as u32) as i32
                * (*img).stride[VPX_PLANE_U as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_U as usize] = -(*img).stride[VPX_PLANE_U as usize];
        (*img).planes[VPX_PLANE_V as usize] = (*img).planes[VPX_PLANE_V as usize].offset(
            (((*img).d_h >> (*img).y_chroma_shift).wrapping_sub(1 as u32) as i32
                * (*img).stride[VPX_PLANE_V as usize]) as isize,
        );
        (*img).stride[VPX_PLANE_V as usize] = -(*img).stride[VPX_PLANE_V as usize];
        (*img).planes[VPX_PLANE_ALPHA as usize] = (*img).planes[VPX_PLANE_ALPHA as usize].offset(
            ((*img).d_h.wrapping_sub(1 as u32) as i32 * (*img).stride[VPX_PLANE_ALPHA as usize])
                as isize,
        );
        (*img).stride[VPX_PLANE_ALPHA as usize] = -(*img).stride[VPX_PLANE_ALPHA as usize];
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_img_free(mut img: *mut VpxImageT) {
    unsafe {
        if !img.is_null() {
            if !(*img).img_data.is_null() && (*img).img_data_owner != 0 {
                vpx_free((*img).img_data as *mut c_void);
            }
            if (*img).self_allocd != 0 {
                free(img as *mut c_void);
            }
        }
    }
}
