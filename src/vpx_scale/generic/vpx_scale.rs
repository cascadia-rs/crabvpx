use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_horizontal_line_2_1_scale_c(
        source: *const u8,
        source_width: u32,
        dest: *mut u8,
        dest_width: u32,
    );
    fn vp8_horizontal_line_5_3_scale_c(
        source: *const u8,
        source_width: u32,
        dest: *mut u8,
        dest_width: u32,
    );
    fn vp8_horizontal_line_5_4_scale_c(
        source: *const u8,
        source_width: u32,
        dest: *mut u8,
        dest_width: u32,
    );
    fn vp8_vertical_band_2_1_scale_c(
        source: *mut u8,
        src_pitch: u32,
        dest: *mut u8,
        dest_pitch: u32,
        dest_width: u32,
    );
    fn vp8_vertical_band_2_1_scale_i_c(
        source: *mut u8,
        src_pitch: u32,
        dest: *mut u8,
        dest_pitch: u32,
        dest_width: u32,
    );
    fn vp8_vertical_band_5_3_scale_c(
        source: *mut u8,
        src_pitch: u32,
        dest: *mut u8,
        dest_pitch: u32,
        dest_width: u32,
    );
    fn vp8_vertical_band_5_4_scale_c(
        source: *mut u8,
        src_pitch: u32,
        dest: *mut u8,
        dest_pitch: u32,
        dest_width: u32,
    );
}

pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub type Scale1D = Option<unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> ()>;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
unsafe fn scale1d_2t1_i(
    mut source: *const u8,
    mut source_step: i32,
    _source_scale: u32,
    _source_length: u32,
    mut dest: *mut u8,
    mut dest_step: i32,
    _dest_scale: u32,
    mut dest_length: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut temp: u32 = 0;
        let mut source_pitch: i32 = source_step;
        source_step *= 2 as i32;
        *dest.offset(0 as isize) = *source.offset(0 as isize);
        i = dest_step as u32;
        j = source_step as u32;
        while i < dest_length.wrapping_mul(dest_step as u32) {
            temp = 8 as u32;
            temp = temp.wrapping_add(
                (3 as i32 * *source.offset(j.wrapping_sub(source_pitch as u32) as isize) as i32)
                    as u32,
            );
            temp = temp.wrapping_add((10 as i32 * *source.offset(j as isize) as i32) as u32);
            temp = temp.wrapping_add(
                (3 as i32 * *source.offset(j.wrapping_add(source_pitch as u32) as isize) as i32)
                    as u32,
            );
            temp >>= 4 as i32;
            *dest.offset(i as isize) = temp as u8;
            i = i.wrapping_add(dest_step as u32);
            j = j.wrapping_add(source_step as u32);
        }
    }
}
unsafe fn scale1d_2t1_ps(
    mut source: *const u8,
    mut source_step: i32,
    _source_scale: u32,
    _source_length: u32,
    mut dest: *mut u8,
    mut dest_step: i32,
    _dest_scale: u32,
    mut dest_length: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        source_step *= 2 as i32;
        j = 0 as u32;
        i = 0 as u32;
        while i < dest_length.wrapping_mul(dest_step as u32) {
            *dest.offset(i as isize) = *source.offset(j as isize);
            i = i.wrapping_add(dest_step as u32);
            j = j.wrapping_add(source_step as u32);
        }
    }
}
unsafe fn scale1d_c(
    mut source: *const u8,
    mut source_step: i32,
    mut source_scale: u32,
    _source_length: u32,
    mut dest: *mut u8,
    mut dest_step: i32,
    mut dest_scale: u32,
    mut dest_length: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        let mut round_value: u32 = dest_scale.wrapping_div(2 as u32);
        let mut left_modifier: u32 = dest_scale;
        let mut right_modifier: u32 = 0 as u32;
        let mut left_pixel: u8 = *source;
        let mut right_pixel: u8 = *source.offset(source_step as isize);
        i = 0 as u32;
        while i < dest_length.wrapping_mul(dest_step as u32) {
            *dest.offset(i as isize) = left_modifier
                .wrapping_mul(left_pixel as u32)
                .wrapping_add(right_modifier.wrapping_mul(right_pixel as u32))
                .wrapping_add(round_value)
                .wrapping_div(dest_scale) as u8;
            right_modifier = right_modifier.wrapping_add(source_scale);
            while right_modifier > dest_scale {
                right_modifier = right_modifier.wrapping_sub(dest_scale);
                source = source.offset(source_step as isize);
                left_pixel = *source;
                right_pixel = *source.offset(source_step as isize);
            }
            left_modifier = dest_scale.wrapping_sub(right_modifier);
            i = i.wrapping_add(dest_step as u32);
        }
    }
}
unsafe fn scale2_d(
    mut source: *mut u8,
    mut source_pitch: i32,
    mut source_width: u32,
    mut source_height: u32,
    mut dest: *mut u8,
    mut dest_pitch: i32,
    mut dest_width: u32,
    mut dest_height: u32,
    mut temp_area: *mut u8,
    mut temp_area_height: u8,
    mut hscale: u32,
    mut hratio: u32,
    mut vscale: u32,
    mut vratio: u32,
    mut interlaced: u32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut bands: i32 = 0;
        let mut dest_band_height: i32 = 0;
        let mut source_band_height: i32 = 0;
        let mut scale1_dv: Scale1D =
            Some(scale1d_c as unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> ());
        let mut scale1_dh: Scale1D =
            Some(scale1d_c as unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> ());
        let mut horiz_line_scale: Option<unsafe fn(*const u8, u32, *mut u8, u32) -> ()> = None;
        let mut vert_band_scale: Option<unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> ()> = None;
        let mut ratio_scalable: i32 = 1 as i32;
        let mut interpolation: i32 = 0 as i32;
        let mut source_base: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut line_src: *mut u8 = ::core::ptr::null_mut::<u8>();
        source_base = source;
        if source_pitch < 0 as i32 {
            let mut offset: i32 = 0;
            offset = source_height.wrapping_sub(1 as u32) as i32;
            offset *= source_pitch;
            source_base = source_base.offset(offset as isize);
        }
        match hratio.wrapping_mul(10 as u32).wrapping_div(hscale) {
            8 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_5_4_scale_c
                        as unsafe fn(*const u8, u32, *mut u8, u32) -> (),
                )
                    as Option<unsafe fn(*const u8, u32, *mut u8, u32) -> ()>;
            }
            6 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_5_3_scale_c
                        as unsafe fn(*const u8, u32, *mut u8, u32) -> (),
                )
                    as Option<unsafe fn(*const u8, u32, *mut u8, u32) -> ()>;
            }
            5 => {
                horiz_line_scale = Some(
                    vp8_horizontal_line_2_1_scale_c
                        as unsafe fn(*const u8, u32, *mut u8, u32) -> (),
                )
                    as Option<unsafe fn(*const u8, u32, *mut u8, u32) -> ()>;
            }
            _ => {
                ratio_scalable = 0 as i32;
            }
        }
        match vratio.wrapping_mul(10 as u32).wrapping_div(vscale) {
            8 => {
                vert_band_scale = Some(
                    vp8_vertical_band_5_4_scale_c
                        as unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> (),
                )
                    as Option<unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> ()>;
                source_band_height = 5 as i32;
                dest_band_height = 4 as i32;
            }
            6 => {
                vert_band_scale = Some(
                    vp8_vertical_band_5_3_scale_c
                        as unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> (),
                )
                    as Option<unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> ()>;
                source_band_height = 5 as i32;
                dest_band_height = 3 as i32;
            }
            5 => {
                if interlaced != 0 {
                    vert_band_scale = Some(
                        vp8_vertical_band_2_1_scale_c
                            as unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> (),
                    )
                        as Option<unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> ()>;
                } else {
                    interpolation = 1 as i32;
                    vert_band_scale = Some(
                        vp8_vertical_band_2_1_scale_i_c
                            as unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> (),
                    )
                        as Option<unsafe fn(*mut u8, u32, *mut u8, u32, u32) -> ()>;
                }
                source_band_height = 2 as i32;
                dest_band_height = 1 as i32;
            }
            _ => {
                ratio_scalable = 0 as i32;
            }
        }
        if ratio_scalable != 0 {
            if source_height == dest_height {
                k = 0 as i32;
                while k < dest_height as i32 {
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
            k = 0 as i32;
            while k < dest_height
                .wrapping_add(dest_band_height as u32)
                .wrapping_sub(1 as u32) as i32
                / dest_band_height
            {
                i = 0 as i32;
                while i < source_band_height {
                    line_src = source.offset((i * source_pitch) as isize);
                    if line_src < source_base {
                        line_src = source_base;
                    }
                    horiz_line_scale.expect("non-null function pointer")(
                        line_src,
                        source_width,
                        temp_area.offset(((i + 1 as i32) * dest_pitch) as isize),
                        dest_width,
                    );
                    i += 1;
                }
                vert_band_scale.expect("non-null function pointer")(
                    temp_area.offset(dest_pitch as isize),
                    dest_pitch as u32,
                    dest,
                    dest_pitch as u32,
                    dest_width,
                );
                if interpolation != 0 {
                    core::ptr::copy_nonoverlapping(
                        temp_area.offset((source_band_height * dest_pitch) as isize)
                            as *const c_void as *const u8,
                        temp_area as *mut c_void as *mut u8,
                        dest_width as usize,
                    );
                }
                source = source
                    .offset((source_band_height as u64).wrapping_mul(source_pitch as u64) as isize);
                dest =
                    dest.offset((dest_band_height as u64).wrapping_mul(dest_pitch as u64) as isize);
                k += 1;
            }
            return;
        }
        if hscale == 2 as u32 && hratio == 1 as u32 {
            scale1_dh = Some(
                scale1d_2t1_ps as unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> (),
            ) as Scale1D;
        }
        if vscale == 2 as u32 && vratio == 1 as u32 {
            if interlaced != 0 {
                scale1_dv = Some(
                    scale1d_2t1_ps
                        as unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> (),
                ) as Scale1D;
            } else {
                scale1_dv = Some(
                    scale1d_2t1_i
                        as unsafe fn(*const u8, i32, u32, u32, *mut u8, i32, u32, u32) -> (),
                ) as Scale1D;
            }
        }
        if source_height == dest_height {
            k = 0 as i32;
            while k < dest_height as i32 {
                scale1_dh.expect("non-null function pointer")(
                    source,
                    1 as i32,
                    hscale,
                    source_width.wrapping_add(1 as u32),
                    dest,
                    1 as i32,
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
            dest_band_height = temp_area_height as i32 - 1 as i32;
            source_band_height = (dest_band_height as u32)
                .wrapping_mul(source_height)
                .wrapping_div(dest_height) as i32;
        } else {
            source_band_height = temp_area_height as i32 - 1 as i32;
            dest_band_height = (source_band_height as u32)
                .wrapping_mul(vratio)
                .wrapping_div(vscale) as i32;
        }
        scale1_dh.expect("non-null function pointer")(
            source,
            1 as i32,
            hscale,
            source_width.wrapping_add(1 as u32),
            temp_area,
            1 as i32,
            hratio,
            dest_width,
        );
        bands = dest_height
            .wrapping_add(dest_band_height as u32)
            .wrapping_sub(1 as u32)
            .wrapping_div(dest_band_height as u32) as i32;
        k = 0 as i32;
        while k < bands {
            i = 1 as i32;
            while i < source_band_height + 1 as i32 {
                if k * source_band_height + i < source_height as i32 {
                    scale1_dh.expect("non-null function pointer")(
                        source.offset((i * source_pitch) as isize),
                        1 as i32,
                        hscale,
                        source_width.wrapping_add(1 as u32),
                        temp_area.offset((i * dest_pitch) as isize),
                        1 as i32,
                        hratio,
                        dest_width,
                    );
                } else {
                    core::ptr::copy_nonoverlapping(
                        temp_area.offset(((i - 1 as i32) * dest_pitch) as isize) as *const c_void
                            as *const u8,
                        temp_area.offset((i * dest_pitch) as isize) as *mut c_void as *mut u8,
                        dest_pitch as usize,
                    );
                }
                i += 1;
            }
            j = 0 as i32;
            while j < dest_width as i32 {
                scale1_dv.expect("non-null function pointer")(
                    temp_area.offset(j as isize) as *mut u8,
                    dest_pitch,
                    vscale,
                    (source_band_height + 1 as i32) as u32,
                    dest.offset(j as isize) as *mut u8,
                    dest_pitch,
                    vratio,
                    dest_band_height as u32,
                );
                j += 1;
            }
            core::ptr::copy_nonoverlapping(
                temp_area.offset((source_band_height * dest_pitch) as isize) as *const c_void
                    as *const u8,
                temp_area as *mut c_void as *mut u8,
                dest_pitch as usize,
            );
            source = source.offset((source_band_height * source_pitch) as isize);
            dest = dest.offset((dest_band_height * dest_pitch) as isize);
            k += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_scale_frame(
    mut src: *mut Yv12BufferConfig,
    mut dst: *mut Yv12BufferConfig,
    mut temp_area: *mut u8,
    mut temp_height: u8,
    mut hscale: u32,
    mut hratio: u32,
    mut vscale: u32,
    mut vratio: u32,
    mut interlaced: u32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut dw: i32 = hscale
            .wrapping_sub(1 as u32)
            .wrapping_add(((*src).y_width as u32).wrapping_mul(hratio))
            .wrapping_div(hscale) as i32;
        let mut dh: i32 = vscale
            .wrapping_sub(1 as u32)
            .wrapping_add(((*src).y_height as u32).wrapping_mul(vratio))
            .wrapping_div(vscale) as i32;
        scale2_d(
            (*src).y_buffer as *mut u8,
            (*src).y_stride,
            (*src).y_width as u32,
            (*src).y_height as u32,
            (*dst).y_buffer as *mut u8,
            (*dst).y_stride,
            dw as u32,
            dh as u32,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if dw < (*dst).y_width {
            i = 0 as i32;
            while i < dh {
                core::ptr::write_bytes(
                    (*dst)
                        .y_buffer
                        .offset((i * (*dst).y_stride) as isize)
                        .offset(dw as isize)
                        .offset(-(1 as isize)) as *mut c_void as *mut u8,
                    *(*dst)
                        .y_buffer
                        .offset((i * (*dst).y_stride + dw - 2 as i32) as isize)
                        as u8,
                    ((*dst).y_width - dw + 1 as i32) as usize,
                );
                i += 1;
            }
        }
        if dh < (*dst).y_height {
            i = dh - 1 as i32;
            while i < (*dst).y_height {
                core::ptr::copy_nonoverlapping(
                    (*dst)
                        .y_buffer
                        .offset(((dh - 2 as i32) * (*dst).y_stride) as isize)
                        as *const c_void as *const u8,
                    (*dst).y_buffer.offset((i * (*dst).y_stride) as isize) as *mut c_void
                        as *mut u8,
                    ((*dst).y_width + 1 as i32) as usize,
                );
                i += 1;
            }
        }
        scale2_d(
            (*src).u_buffer as *mut u8,
            (*src).uv_stride,
            (*src).uv_width as u32,
            (*src).uv_height as u32,
            (*dst).u_buffer as *mut u8,
            (*dst).uv_stride,
            (dw / 2 as i32) as u32,
            (dh / 2 as i32) as u32,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if (dw / 2 as i32) < (*dst).uv_width {
            i = 0 as i32;
            while i < (*dst).uv_height {
                core::ptr::write_bytes(
                    (*dst)
                        .u_buffer
                        .offset((i * (*dst).uv_stride) as isize)
                        .offset((dw / 2 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void as *mut u8,
                    *(*dst)
                        .u_buffer
                        .offset((i * (*dst).uv_stride + dw / 2 as i32 - 2 as i32) as isize)
                        as u8,
                    ((*dst).uv_width - dw / 2 as i32 + 1 as i32) as usize,
                );
                i += 1;
            }
        }
        if (dh / 2 as i32) < (*dst).uv_height {
            i = dh / 2 as i32 - 1 as i32;
            while i < (*dst).y_height / 2 as i32 {
                core::ptr::copy_nonoverlapping(
                    (*dst)
                        .u_buffer
                        .offset(((dh / 2 as i32 - 2 as i32) * (*dst).uv_stride) as isize)
                        as *const c_void as *const u8,
                    (*dst).u_buffer.offset((i * (*dst).uv_stride) as isize) as *mut c_void
                        as *mut u8,
                    (*dst).uv_width as usize,
                );
                i += 1;
            }
        }
        scale2_d(
            (*src).v_buffer as *mut u8,
            (*src).uv_stride,
            (*src).uv_width as u32,
            (*src).uv_height as u32,
            (*dst).v_buffer as *mut u8,
            (*dst).uv_stride,
            (dw / 2 as i32) as u32,
            (dh / 2 as i32) as u32,
            temp_area,
            temp_height,
            hscale,
            hratio,
            vscale,
            vratio,
            interlaced,
        );
        if (dw / 2 as i32) < (*dst).uv_width {
            i = 0 as i32;
            while i < (*dst).uv_height {
                core::ptr::write_bytes(
                    (*dst)
                        .v_buffer
                        .offset((i * (*dst).uv_stride) as isize)
                        .offset((dw / 2 as i32) as isize)
                        .offset(-(1 as isize)) as *mut c_void as *mut u8,
                    *(*dst)
                        .v_buffer
                        .offset((i * (*dst).uv_stride + dw / 2 as i32 - 2 as i32) as isize)
                        as u8,
                    ((*dst).uv_width - dw / 2 as i32 + 1 as i32) as usize,
                );
                i += 1;
            }
        }
        if (dh / 2 as i32) < (*dst).uv_height {
            i = dh / 2 as i32 - 1 as i32;
            while i < (*dst).y_height / 2 as i32 {
                core::ptr::copy_nonoverlapping(
                    (*dst)
                        .v_buffer
                        .offset(((dh / 2 as i32 - 2 as i32) * (*dst).uv_stride) as isize)
                        as *const c_void as *const u8,
                    (*dst).v_buffer.offset((i * (*dst).uv_stride) as isize) as *mut c_void
                        as *mut u8,
                    (*dst).uv_width as usize,
                );
                i += 1;
            }
        }
    }
}
