unsafe extern "Rust" {
    fn vp8_horizontal_line_2_1_scale_c(
        source: *const ::core::ffi::c_uchar,
        source_width: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_horizontal_line_5_3_scale_c(
        source: *const ::core::ffi::c_uchar,
        source_width: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_horizontal_line_5_4_scale_c(
        source: *const ::core::ffi::c_uchar,
        source_width: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_vertical_band_2_1_scale_c(
        source: *mut ::core::ffi::c_uchar,
        src_pitch: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_pitch: ::core::ffi::c_uint,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_vertical_band_2_1_scale_i_c(
        source: *mut ::core::ffi::c_uchar,
        src_pitch: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_pitch: ::core::ffi::c_uint,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_vertical_band_5_3_scale_c(
        source: *mut ::core::ffi::c_uchar,
        src_pitch: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_pitch: ::core::ffi::c_uint,
        dest_width: ::core::ffi::c_uint,
    );
    fn vp8_vertical_band_5_4_scale_c(
        source: *mut ::core::ffi::c_uchar,
        src_pitch: ::core::ffi::c_uint,
        dest: *mut ::core::ffi::c_uchar,
        dest_pitch: ::core::ffi::c_uint,
        dest_width: ::core::ffi::c_uint,
    );
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[derive(Copy, Clone)]
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type uint8_t = u8;
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type Scale1D = Option<unsafe fn(
        *const ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
    ) -> (),
>;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
unsafe fn scale1d_2t1_i(
    mut source: *const ::core::ffi::c_uchar,
    mut source_step: ::core::ffi::c_int,
    _source_scale: ::core::ffi::c_uint,
    _source_length: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_step: ::core::ffi::c_int,
    _dest_scale: ::core::ffi::c_uint,
    mut dest_length: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut temp: ::core::ffi::c_uint = 0;
        let mut source_pitch: ::core::ffi::c_int = source_step;
        source_step *= 2 as ::core::ffi::c_int;
        *dest.offset(0 as ::core::ffi::c_int as isize) =
            *source.offset(0 as ::core::ffi::c_int as isize);
        i = dest_step as ::core::ffi::c_uint;
        j = source_step as ::core::ffi::c_uint;
        while i < dest_length.wrapping_mul(dest_step as ::core::ffi::c_uint) {
            temp = 8 as ::core::ffi::c_uint;
            temp = temp.wrapping_add(
                (3 as ::core::ffi::c_int
                    * *source.offset(j.wrapping_sub(source_pitch as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            temp = temp.wrapping_add(
                (10 as ::core::ffi::c_int * *source.offset(j as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            temp = temp.wrapping_add(
                (3 as ::core::ffi::c_int
                    * *source.offset(j.wrapping_add(source_pitch as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            temp >>= 4 as ::core::ffi::c_int;
            *dest.offset(i as isize) = temp as ::core::ffi::c_char as ::core::ffi::c_uchar;
            i = i.wrapping_add(dest_step as ::core::ffi::c_uint);
            j = j.wrapping_add(source_step as ::core::ffi::c_uint);
        }
    }
}
unsafe fn scale1d_2t1_ps(
    mut source: *const ::core::ffi::c_uchar,
    mut source_step: ::core::ffi::c_int,
    _source_scale: ::core::ffi::c_uint,
    _source_length: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_step: ::core::ffi::c_int,
    _dest_scale: ::core::ffi::c_uint,
    mut dest_length: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        source_step *= 2 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i < dest_length.wrapping_mul(dest_step as ::core::ffi::c_uint) {
            *dest.offset(i as isize) = *source.offset(j as isize);
            i = i.wrapping_add(dest_step as ::core::ffi::c_uint);
            j = j.wrapping_add(source_step as ::core::ffi::c_uint);
        }
    }
}
unsafe fn scale1d_c(
    mut source: *const ::core::ffi::c_uchar,
    mut source_step: ::core::ffi::c_int,
    mut source_scale: ::core::ffi::c_uint,
    _source_length: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_step: ::core::ffi::c_int,
    mut dest_scale: ::core::ffi::c_uint,
    mut dest_length: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut round_value: ::core::ffi::c_uint =
            dest_scale.wrapping_div(2 as ::core::ffi::c_uint);
        let mut left_modifier: ::core::ffi::c_uint = dest_scale;
        let mut right_modifier: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut left_pixel: ::core::ffi::c_uchar = *source;
        let mut right_pixel: ::core::ffi::c_uchar = *source.offset(source_step as isize);
        i = 0 as ::core::ffi::c_uint;
        while i < dest_length.wrapping_mul(dest_step as ::core::ffi::c_uint) {
            *dest.offset(i as isize) = left_modifier
                .wrapping_mul(left_pixel as ::core::ffi::c_uint)
                .wrapping_add(right_modifier.wrapping_mul(right_pixel as ::core::ffi::c_uint))
                .wrapping_add(round_value)
                .wrapping_div(dest_scale)
                as ::core::ffi::c_char
                as ::core::ffi::c_uchar;
            right_modifier = right_modifier.wrapping_add(source_scale);
            while right_modifier > dest_scale {
                right_modifier = right_modifier.wrapping_sub(dest_scale);
                source = source.offset(source_step as isize);
                left_pixel = *source;
                right_pixel = *source.offset(source_step as isize);
            }
            left_modifier = dest_scale.wrapping_sub(right_modifier);
            i = i.wrapping_add(dest_step as ::core::ffi::c_uint);
        }
    }
}
unsafe fn Scale2D(
    mut source: *mut ::core::ffi::c_uchar,
    mut source_pitch: ::core::ffi::c_int,
    mut source_width: ::core::ffi::c_uint,
    mut source_height: ::core::ffi::c_uint,
    mut dest: *mut ::core::ffi::c_uchar,
    mut dest_pitch: ::core::ffi::c_int,
    mut dest_width: ::core::ffi::c_uint,
    mut dest_height: ::core::ffi::c_uint,
    mut temp_area: *mut ::core::ffi::c_uchar,
    mut temp_area_height: ::core::ffi::c_uchar,
    mut hscale: ::core::ffi::c_uint,
    mut hratio: ::core::ffi::c_uint,
    mut vscale: ::core::ffi::c_uint,
    mut vratio: ::core::ffi::c_uint,
    mut interlaced: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut bands: ::core::ffi::c_int = 0;
        let mut dest_band_height: ::core::ffi::c_int = 0;
        let mut source_band_height: ::core::ffi::c_int = 0;
        let mut Scale1Dv: Scale1D = Some(
            scale1d_c
                as unsafe fn(
                    *const ::core::ffi::c_uchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                    *mut ::core::ffi::c_uchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> (),
        );
        let mut Scale1Dh: Scale1D = Some(
            scale1d_c
                as unsafe fn(
                    *const ::core::ffi::c_uchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                    *mut ::core::ffi::c_uchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> (),
        );
        let mut horiz_line_scale: Option<unsafe fn(
                *const ::core::ffi::c_uchar,
                ::core::ffi::c_uint,
                *mut ::core::ffi::c_uchar,
                ::core::ffi::c_uint,
            ) -> (),
        > = None;
        let mut vert_band_scale: Option<unsafe fn(
                *mut ::core::ffi::c_uchar,
                ::core::ffi::c_uint,
                *mut ::core::ffi::c_uchar,
                ::core::ffi::c_uint,
                ::core::ffi::c_uint,
            ) -> (),
        > = None;
        let mut ratio_scalable: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut interpolation: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut source_base: *mut ::core::ffi::c_uchar =
            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut line_src: *mut ::core::ffi::c_uchar =
            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        source_base = source;
        if source_pitch < 0 as ::core::ffi::c_int {
            let mut offset: ::core::ffi::c_int = 0;
            offset = source_height.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            offset *= source_pitch;
            source_base = source_base.offset(offset as isize);
        }
        match hratio
            .wrapping_mul(10 as ::core::ffi::c_uint)
            .wrapping_div(hscale)
        {
            8 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_5_4_scale_c
                        as unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                )
                    as Option<unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                    >;
            }
            6 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_5_3_scale_c
                        as unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                )
                    as Option<unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                    >;
            }
            5 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_2_1_scale_c
                        as unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                )
                    as Option<unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                        ) -> (),
                    >;
            }
            _ => {
                ratio_scalable = 0 as ::core::ffi::c_int;
            }
        }
        match vratio
            .wrapping_mul(10 as ::core::ffi::c_uint)
            .wrapping_div(vscale)
        {
            8 => {
                vert_band_scale = Some(
                    vp8_vertical_band_5_4_scale_c
                        as unsafe fn(
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                )
                    as Option<unsafe fn(
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                    >;
                source_band_height = 5 as ::core::ffi::c_int;
                dest_band_height = 4 as ::core::ffi::c_int;
            }
            6 => {
                vert_band_scale = Some(
                    vp8_vertical_band_5_3_scale_c
                        as unsafe fn(
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                )
                    as Option<unsafe fn(
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                    >;
                source_band_height = 5 as ::core::ffi::c_int;
                dest_band_height = 3 as ::core::ffi::c_int;
            }
            5 => {
                if interlaced != 0 {
                    vert_band_scale = Some(
                        vp8_vertical_band_2_1_scale_c
                            as unsafe fn(
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                ::core::ffi::c_uint,
                            ) -> (),
                    )
                        as Option<unsafe fn(
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                ::core::ffi::c_uint,
                            ) -> (),
                        >;
                } else {
                    interpolation = 1 as ::core::ffi::c_int;
                    vert_band_scale = Some(
                        vp8_vertical_band_2_1_scale_i_c
                            as unsafe fn(
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                ::core::ffi::c_uint,
                            ) -> (),
                    )
                        as Option<unsafe fn(
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                *mut ::core::ffi::c_uchar,
                                ::core::ffi::c_uint,
                                ::core::ffi::c_uint,
                            ) -> (),
                        >;
                }
                source_band_height = 2 as ::core::ffi::c_int;
                dest_band_height = 1 as ::core::ffi::c_int;
            }
            _ => {
                ratio_scalable = 0 as ::core::ffi::c_int;
            }
        }
        if ratio_scalable != 0 {
            if source_height == dest_height {
                k = 0 as ::core::ffi::c_int;
                while k < dest_height as ::core::ffi::c_int {
                    horiz_line_scale.expect("non-null function pointer")(
                        source,
                        source_width,
                        dest,
                        dest_width,
                    );
                    source = source.offset(source_pitch as isize);
                    dest = dest.offset(dest_pitch as isize);
                    k += 1;
                }
                return;
            }
            if interpolation != 0 {
                if source < source_base {
                    source = source_base;
                }
                horiz_line_scale.expect("non-null function pointer")(
                    source,
                    source_width,
                    temp_area,
                    dest_width,
                );
            }
            k = 0 as ::core::ffi::c_int;
            while k < dest_height
                .wrapping_add(dest_band_height as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int
                / dest_band_height
            {
                i = 0 as ::core::ffi::c_int;
                while i < source_band_height {
                    line_src = source.offset((i * source_pitch) as isize);
                    if line_src < source_base {
                        line_src = source_base;
                    }
                    horiz_line_scale.expect("non-null function pointer")(
                        line_src,
                        source_width,
                        temp_area.offset(((i + 1 as ::core::ffi::c_int) * dest_pitch) as isize),
                        dest_width,
                    );
                    i += 1;
                }
                vert_band_scale.expect("non-null function pointer")(
                    temp_area.offset(dest_pitch as isize),
                    dest_pitch as ::core::ffi::c_uint,
                    dest,
                    dest_pitch as ::core::ffi::c_uint,
                    dest_width,
                );
                if interpolation != 0 {
                    memcpy(
                        temp_area as *mut ::core::ffi::c_void,
                        temp_area.offset((source_band_height * dest_pitch) as isize)
                            as *const ::core::ffi::c_void,
                        dest_width as size_t,
                    );
                }
                source = source.offset(
                    (source_band_height as ::core::ffi::c_ulong)
                        .wrapping_mul(source_pitch as ::core::ffi::c_ulong)
                        as isize,
                );
                dest = dest.offset(
                    (dest_band_height as ::core::ffi::c_ulong)
                        .wrapping_mul(dest_pitch as ::core::ffi::c_ulong)
                        as isize,
                );
                k += 1;
            }
            return;
        }
        if hscale == 2 as ::core::ffi::c_uint && hratio == 1 as ::core::ffi::c_uint {
            Scale1Dh = Some(
                scale1d_2t1_ps
                    as unsafe fn(
                        *const ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_int,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                    ) -> (),
            ) as Scale1D;
        }
        if vscale == 2 as ::core::ffi::c_uint && vratio == 1 as ::core::ffi::c_uint {
            if interlaced != 0 {
                Scale1Dv = Some(
                    scale1d_2t1_ps
                        as unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_int,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_int,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                ) as Scale1D;
            } else {
                Scale1Dv = Some(
                    scale1d_2t1_i
                        as unsafe fn(
                            *const ::core::ffi::c_uchar,
                            ::core::ffi::c_int,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                            *mut ::core::ffi::c_uchar,
                            ::core::ffi::c_int,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                ) as Scale1D;
            }
        }
        if source_height == dest_height {
            k = 0 as ::core::ffi::c_int;
            while k < dest_height as ::core::ffi::c_int {
                Scale1Dh.expect("non-null function pointer")(
                    source,
                    1 as ::core::ffi::c_int,
                    hscale,
                    source_width.wrapping_add(1 as ::core::ffi::c_uint),
                    dest,
                    1 as ::core::ffi::c_int,
                    hratio,
                    dest_width,
                );
                source = source.offset(source_pitch as isize);
                dest = dest.offset(dest_pitch as isize);
                k += 1;
            }
            return;
        }
        if dest_height > source_height {
            dest_band_height = temp_area_height as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            source_band_height = (dest_band_height as ::core::ffi::c_uint)
                .wrapping_mul(source_height)
                .wrapping_div(dest_height) as ::core::ffi::c_int;
        } else {
            source_band_height = temp_area_height as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            dest_band_height = (source_band_height as ::core::ffi::c_uint)
                .wrapping_mul(vratio)
                .wrapping_div(vscale) as ::core::ffi::c_int;
        }
        Scale1Dh.expect("non-null function pointer")(
            source,
            1 as ::core::ffi::c_int,
            hscale,
            source_width.wrapping_add(1 as ::core::ffi::c_uint),
            temp_area,
            1 as ::core::ffi::c_int,
            hratio,
            dest_width,
        );
        bands = dest_height
            .wrapping_add(dest_band_height as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_div(dest_band_height as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        k = 0 as ::core::ffi::c_int;
        while k < bands {
            i = 1 as ::core::ffi::c_int;
            while i < source_band_height + 1 as ::core::ffi::c_int {
                if k * source_band_height + i < source_height as ::core::ffi::c_int {
                    Scale1Dh.expect("non-null function pointer")(
                        source.offset((i * source_pitch) as isize),
                        1 as ::core::ffi::c_int,
                        hscale,
                        source_width.wrapping_add(1 as ::core::ffi::c_uint),
                        temp_area.offset((i * dest_pitch) as isize),
                        1 as ::core::ffi::c_int,
                        hratio,
                        dest_width,
                    );
                } else {
                    memcpy(
                        temp_area.offset((i * dest_pitch) as isize) as *mut ::core::ffi::c_void,
                        temp_area.offset(((i - 1 as ::core::ffi::c_int) * dest_pitch) as isize)
                            as *const ::core::ffi::c_void,
                        dest_pitch as size_t,
                    );
                }
                i += 1;
            }
            j = 0 as ::core::ffi::c_int;
            while j < dest_width as ::core::ffi::c_int {
                Scale1Dv.expect("non-null function pointer")(
                    temp_area.offset(j as isize) as *mut ::core::ffi::c_uchar,
                    dest_pitch,
                    vscale,
                    (source_band_height + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    dest.offset(j as isize) as *mut ::core::ffi::c_uchar,
                    dest_pitch,
                    vratio,
                    dest_band_height as ::core::ffi::c_uint,
                );
                j += 1;
            }
            memcpy(
                temp_area as *mut ::core::ffi::c_void,
                temp_area.offset((source_band_height * dest_pitch) as isize)
                    as *const ::core::ffi::c_void,
                dest_pitch as size_t,
            );
            source = source.offset((source_band_height * source_pitch) as isize);
            dest = dest.offset((dest_band_height * dest_pitch) as isize);
            k += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_scale_frame(
    mut src: *mut YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
    mut temp_area: *mut ::core::ffi::c_uchar,
    mut temp_height: ::core::ffi::c_uchar,
    mut hscale: ::core::ffi::c_uint,
    mut hratio: ::core::ffi::c_uint,
    mut vscale: ::core::ffi::c_uint,
    mut vratio: ::core::ffi::c_uint,
    mut interlaced: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut dw: ::core::ffi::c_int = hscale
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_add(((*src).y_width as ::core::ffi::c_uint).wrapping_mul(hratio))
            .wrapping_div(hscale) as ::core::ffi::c_int;
        let mut dh: ::core::ffi::c_int = vscale
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_add(((*src).y_height as ::core::ffi::c_uint).wrapping_mul(vratio))
            .wrapping_div(vscale) as ::core::ffi::c_int;
        Scale2D(
            (*src).y_buffer as *mut ::core::ffi::c_uchar,
            (*src).y_stride,
            (*src).y_width as ::core::ffi::c_uint,
            (*src).y_height as ::core::ffi::c_uint,
            (*dst).y_buffer as *mut ::core::ffi::c_uchar,
            (*dst).y_stride,
            dw as ::core::ffi::c_uint,
            dh as ::core::ffi::c_uint,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if dw < (*dst).y_width {
            i = 0 as ::core::ffi::c_int;
            while i < dh {
                memset(
                    (*dst)
                        .y_buffer
                        .offset((i * (*dst).y_stride) as isize)
                        .offset(dw as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                    *(*dst)
                        .y_buffer
                        .offset((i * (*dst).y_stride + dw - 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                    ((*dst).y_width - dw + 1 as ::core::ffi::c_int) as size_t,
                );
                i += 1;
            }
        }
        if dh < (*dst).y_height {
            i = dh - 1 as ::core::ffi::c_int;
            while i < (*dst).y_height {
                memcpy(
                    (*dst).y_buffer.offset((i * (*dst).y_stride) as isize)
                        as *mut ::core::ffi::c_void,
                    (*dst)
                        .y_buffer
                        .offset(((dh - 2 as ::core::ffi::c_int) * (*dst).y_stride) as isize)
                        as *const ::core::ffi::c_void,
                    ((*dst).y_width + 1 as ::core::ffi::c_int) as size_t,
                );
                i += 1;
            }
        }
        Scale2D(
            (*src).u_buffer as *mut ::core::ffi::c_uchar,
            (*src).uv_stride,
            (*src).uv_width as ::core::ffi::c_uint,
            (*src).uv_height as ::core::ffi::c_uint,
            (*dst).u_buffer as *mut ::core::ffi::c_uchar,
            (*dst).uv_stride,
            (dw / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            (dh / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if (dw / 2 as ::core::ffi::c_int) < (*dst).uv_width {
            i = 0 as ::core::ffi::c_int;
            while i < (*dst).uv_height {
                memset(
                    (*dst)
                        .u_buffer
                        .offset((i * (*dst).uv_stride) as isize)
                        .offset((dw / 2 as ::core::ffi::c_int) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                    *(*dst).u_buffer.offset(
                        (i * (*dst).uv_stride + dw / 2 as ::core::ffi::c_int
                            - 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int,
                    ((*dst).uv_width - dw / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as size_t,
                );
                i += 1;
            }
        }
        if (dh / 2 as ::core::ffi::c_int) < (*dst).uv_height {
            i = dh / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            while i < (*dst).y_height / 2 as ::core::ffi::c_int {
                memcpy(
                    (*dst).u_buffer.offset((i * (*dst).uv_stride) as isize)
                        as *mut ::core::ffi::c_void,
                    (*dst).u_buffer.offset(
                        ((dh / 2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                            * (*dst).uv_stride) as isize,
                    ) as *const ::core::ffi::c_void,
                    (*dst).uv_width as size_t,
                );
                i += 1;
            }
        }
        Scale2D(
            (*src).v_buffer as *mut ::core::ffi::c_uchar,
            (*src).uv_stride,
            (*src).uv_width as ::core::ffi::c_uint,
            (*src).uv_height as ::core::ffi::c_uint,
            (*dst).v_buffer as *mut ::core::ffi::c_uchar,
            (*dst).uv_stride,
            (dw / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            (dh / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if (dw / 2 as ::core::ffi::c_int) < (*dst).uv_width {
            i = 0 as ::core::ffi::c_int;
            while i < (*dst).uv_height {
                memset(
                    (*dst)
                        .v_buffer
                        .offset((i * (*dst).uv_stride) as isize)
                        .offset((dw / 2 as ::core::ffi::c_int) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                    *(*dst).v_buffer.offset(
                        (i * (*dst).uv_stride + dw / 2 as ::core::ffi::c_int
                            - 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int,
                    ((*dst).uv_width - dw / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as size_t,
                );
                i += 1;
            }
        }
        if (dh / 2 as ::core::ffi::c_int) < (*dst).uv_height {
            i = dh / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            while i < (*dst).y_height / 2 as ::core::ffi::c_int {
                memcpy(
                    (*dst).v_buffer.offset((i * (*dst).uv_stride) as isize)
                        as *mut ::core::ffi::c_void,
                    (*dst).v_buffer.offset(
                        ((dh / 2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                            * (*dst).uv_stride) as isize,
                    ) as *const ::core::ffi::c_void,
                    (*dst).uv_width as size_t,
                );
                i += 1;
            }
        }
    }
}
