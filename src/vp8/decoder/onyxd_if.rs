use crate::vp8::decoder::decodeframe::vp8cx_init_de_quantizer;
use crate::vp8::common::mbpitch::vp8_setup_block_dptrs;

unsafe extern "C" {
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn vp8_loop_filter_init(cm: *mut VP8Common);
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
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
    fn vp8_decode_frame(pbi: *mut VP8D_COMP) -> ::core::ffi::c_int;
    fn vpx_memalign(align: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
    fn vp8_decoder_remove_threads(pbi: *mut VP8D_COMP);
    fn vp8_decoder_create_threads(pbi: *mut VP8D_COMP);
    fn vp8_init_intra_predictors();
    fn vpx_dsp_rtcd();
}
pub use crate::vp8::common::alloccommon::{vp8_create_common, vp8_remove_common};
pub use crate::vp8::common::types::*;
pub use crate::vpx_scale::generic::yv12extend::vp8_yv12_copy_frame_c;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: C2RustUnnamed = 4;
pub const ALTREF_FRAME: C2RustUnnamed = 3;
pub const GOLDEN_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_ppflags_t {
    pub post_proc_flag: ::core::ffi::c_int,
    pub deblocking_level: ::core::ffi::c_int,
    pub noise_level: ::core::ffi::c_int,
    pub display_ref_frame_flag: ::core::ffi::c_int,
    pub display_mb_modes_flag: ::core::ffi::c_int,
    pub display_b_modes_flag: ::core::ffi::c_int,
    pub display_mv_flag: ::core::ffi::c_int,
}
pub type vpx_ref_frame_type = ::core::ffi::c_uint;
pub const VP8_ALTR_FRAME: vpx_ref_frame_type = 4;
pub const VP8_GOLD_FRAME: vpx_ref_frame_type = 2;
pub const VP8_LAST_FRAME: vpx_ref_frame_type = 1;
pub type pthread_once_t = __darwin_pthread_once_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const _PTHREAD_ONCE_SIG_init: ::core::ffi::c_int = 0x30b1bcba as ::core::ffi::c_int;
pub const NUM_YV12_BUFFERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn once(mut func: Option<unsafe extern "C" fn() -> ()>) { unsafe {
    static mut lock: pthread_once_t = _opaque_pthread_once_t {
        __sig: _PTHREAD_ONCE_SIG_init as ::core::ffi::c_long,
        __opaque: [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };
    pthread_once(&raw mut lock, func as Option<unsafe extern "C" fn() -> ()>);
}}
unsafe extern "C" fn initialize_dec() { unsafe {
    static mut init_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if init_done == 0 {
        vpx_dsp_rtcd();
        vp8_init_intra_predictors();
        ::core::ptr::write_volatile(
            &raw mut init_done as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}}
unsafe extern "C" fn remove_decompressor(mut pbi: *mut VP8D_COMP) { unsafe {
    vp8_remove_common(&mut (*pbi).common);
    if !pbi.is_null() {
        let _ = Box::from_raw(pbi);
    }
}}
unsafe extern "C" fn create_decompressor(mut oxcf: *mut VP8D_CONFIG) -> *mut VP8D_COMP { unsafe {
    let mut pbi: *mut VP8D_COMP = match Box::<VP8D_COMP>::try_new_zeroed() {
        Ok(b) => Box::into_raw(b.assume_init()),
        Err(_) => return ::core::ptr::null_mut::<VP8D_COMP>(),
    };
    if setjmp(&raw mut (*pbi).common.error.jmp as *mut ::core::ffi::c_int) != 0 {
        (*pbi).common.error.setjmp = 0 as ::core::ffi::c_int;
        remove_decompressor(pbi);
        return ::core::ptr::null_mut::<VP8D_COMP>();
    }
    (*pbi).common.error.setjmp = 1 as ::core::ffi::c_int;
    vp8_create_common(&mut (*pbi).common);
    (*pbi).common.current_video_frame = 0 as ::core::ffi::c_uint;
    (*pbi).ready_for_new_data = 1 as ::core::ffi::c_int;
    vp8cx_init_de_quantizer(&mut *pbi);
    vp8_loop_filter_init(&raw mut (*pbi).common);
    (*pbi).common.error.setjmp = 0 as ::core::ffi::c_int;
    (*pbi).ec_enabled = 0 as ::core::ffi::c_int;
    (*pbi).ec_active = 0 as ::core::ffi::c_int;
    (*pbi).decoded_key_frame = 0 as ::core::ffi::c_int;
    (*pbi).independent_partitions = 0 as ::core::ffi::c_int;
    vp8_setup_block_dptrs(&mut (*pbi).mb);
    once(Some(initialize_dec as unsafe extern "C" fn() -> ()));
    return pbi as *mut VP8D_COMP;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut ref_fb_idx: ::core::ffi::c_int = 0;
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).lst_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).gld_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_idx = (*cm).alt_fb_idx;
    } else {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Invalid reference frame\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*pbi).common.error.error_code;
    }
    if (*cm).yv12_fb[ref_fb_idx as usize].y_height != (*sd).y_height
        || (*cm).yv12_fb[ref_fb_idx as usize].y_width != (*sd).y_width
        || (*cm).yv12_fb[ref_fb_idx as usize].uv_height != (*sd).uv_height
        || (*cm).yv12_fb[ref_fb_idx as usize].uv_width != (*sd).uv_width
    {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        vp8_yv12_copy_frame_c(
            &(*cm).yv12_fb[ref_fb_idx as usize],
            &mut *sd,
        );
    }
    return (*pbi).common.error.error_code;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_set_reference(
    mut pbi: *mut VP8D_COMP,
    mut ref_frame_flag: vpx_ref_frame_type,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> vpx_codec_err_t { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut ref_fb_ptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut free_fb: ::core::ffi::c_int = 0;
    if ref_frame_flag as ::core::ffi::c_uint
        == VP8_LAST_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).lst_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_GOLD_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).gld_fb_idx;
    } else if ref_frame_flag as ::core::ffi::c_uint
        == VP8_ALTR_FRAME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ref_fb_ptr = &raw mut (*cm).alt_fb_idx;
    } else {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Invalid reference frame\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*pbi).common.error.error_code;
    }
    if (*cm).yv12_fb[*ref_fb_ptr as usize].y_height != (*sd).y_height
        || (*cm).yv12_fb[*ref_fb_ptr as usize].y_width != (*sd).y_width
        || (*cm).yv12_fb[*ref_fb_ptr as usize].uv_height != (*sd).uv_height
        || (*cm).yv12_fb[*ref_fb_ptr as usize].uv_width != (*sd).uv_width
    {
        vpx_internal_error(
            &raw mut (*pbi).common.error,
            VPX_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        free_fb = get_free_fb(&mut *cm);
        (*cm).fb_idx_ref_cnt[free_fb as usize] -= 1;
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            ref_fb_ptr,
            free_fb,
        );
        vp8_yv12_copy_frame_c(
            &*sd,
            &mut (*cm).yv12_fb[*ref_fb_ptr as usize],
        );
    }
    return (*pbi).common.error.error_code;
}}
fn get_free_fb(cm: &mut VP8_COMMON) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_YV12_BUFFERS {
        if cm.fb_idx_ref_cnt[i as usize] == 0 as ::core::ffi::c_int {
            break;
        }
        i += 1;
    }
    cm.fb_idx_ref_cnt[i as usize] = 1 as ::core::ffi::c_int;
    return i;
}
unsafe extern "C" fn ref_cnt_fb(
    mut buf: *mut ::core::ffi::c_int,
    mut idx: *mut ::core::ffi::c_int,
    mut new_idx: ::core::ffi::c_int,
) { unsafe {
    if *buf.offset(*idx as isize) > 0 as ::core::ffi::c_int {
        let ref mut fresh0 = *buf.offset(*idx as isize);
        *fresh0 -= 1;
    }
    *idx = new_idx;
    let ref mut fresh1 = *buf.offset(new_idx as isize);
    *fresh1 += 1;
}}
unsafe extern "C" fn swap_frame_buffers(mut cm: *mut VP8_COMMON) -> ::core::ffi::c_int { unsafe {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cm).copy_buffer_to_arf != 0 {
        let mut new_fb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*cm).copy_buffer_to_arf == 1 as ::core::ffi::c_int {
            new_fb = (*cm).lst_fb_idx;
        } else if (*cm).copy_buffer_to_arf == 2 as ::core::ffi::c_int {
            new_fb = (*cm).gld_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).alt_fb_idx,
            new_fb,
        );
    }
    if (*cm).copy_buffer_to_gf != 0 {
        let mut new_fb_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*cm).copy_buffer_to_gf == 1 as ::core::ffi::c_int {
            new_fb_0 = (*cm).lst_fb_idx;
        } else if (*cm).copy_buffer_to_gf == 2 as ::core::ffi::c_int {
            new_fb_0 = (*cm).alt_fb_idx;
        } else {
            err = -(1 as ::core::ffi::c_int);
        }
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).gld_fb_idx,
            new_fb_0,
        );
    }
    if (*cm).refresh_golden_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).gld_fb_idx,
            (*cm).new_fb_idx,
        );
    }
    if (*cm).refresh_alt_ref_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).alt_fb_idx,
            (*cm).new_fb_idx,
        );
    }
    if (*cm).refresh_last_frame != 0 {
        ref_cnt_fb(
            &raw mut (*cm).fb_idx_ref_cnt as *mut ::core::ffi::c_int,
            &raw mut (*cm).lst_fb_idx,
            (*cm).new_fb_idx,
        );
        (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
            .offset((*cm).lst_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    } else {
        (*cm).frame_to_show = (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG)
            .offset((*cm).new_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    }
    (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
    return err;
}}
unsafe extern "C" fn check_fragments_for_errors(mut pbi: *mut VP8D_COMP) -> ::core::ffi::c_int { unsafe {
    if (*pbi).ec_active == 0
        && (*pbi).fragments.count <= 1 as ::core::ffi::c_uint
        && (*pbi).fragments.sizes[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_uint
    {
        let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
        if (*cm).fb_idx_ref_cnt[(*cm).lst_fb_idx as usize] > 1 as ::core::ffi::c_int {
            let prev_idx: ::core::ffi::c_int = (*cm).lst_fb_idx;
            (*cm).fb_idx_ref_cnt[prev_idx as usize] -= 1;
            (*cm).lst_fb_idx = get_free_fb(&mut *cm);
            let src_ptr = &raw const (*cm).yv12_fb[prev_idx as usize];
            let dst_ptr = &raw mut (*cm).yv12_fb[(*cm).lst_fb_idx as usize];
            vp8_yv12_copy_frame_c(&*src_ptr, &mut *dst_ptr);
        }
        (*cm).yv12_fb[(*cm).lst_fb_idx as usize].corrupted = 1 as ::core::ffi::c_int;
        (*cm).show_frame = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_receive_compressed_data(
    mut pbi: *mut VP8D_COMP,
) -> ::core::ffi::c_int { unsafe {
    let mut cm: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut retcode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    (*pbi).common.error.error_code = VPX_CODEC_OK;
    retcode = check_fragments_for_errors(pbi);
    if retcode <= 0 as ::core::ffi::c_int {
        return retcode;
    }
    (*cm).new_fb_idx = get_free_fb(&mut *cm);
    (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).new_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[LAST_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).lst_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[GOLDEN_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).gld_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    (*pbi).dec_fb_ref[ALTREF_FRAME as ::core::ffi::c_int as usize] =
        (&raw mut (*cm).yv12_fb as *mut YV12_BUFFER_CONFIG).offset((*cm).alt_fb_idx as isize)
            as *mut YV12_BUFFER_CONFIG;
    retcode = vp8_decode_frame(pbi);
    if retcode < 0 as ::core::ffi::c_int {
        if (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] > 0 as ::core::ffi::c_int {
            (*cm).fb_idx_ref_cnt[(*cm).new_fb_idx as usize] -= 1;
        }
        (*pbi).common.error.error_code = VPX_CODEC_ERROR;
        if (*pbi).mb.error_info.error_code as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            (*pbi).common.error.error_code = (*pbi).mb.error_info.error_code;
            memcpy(
                &raw mut (*pbi).common.error.detail as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw mut (*pbi).mb.error_info.detail as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            );
        }
    } else if swap_frame_buffers(cm) != 0 {
        (*pbi).common.error.error_code = VPX_CODEC_ERROR;
    } else {
        if (*cm).show_frame != 0 {
            (*cm).current_video_frame = (*cm).current_video_frame.wrapping_add(1);
            (*cm).show_frame_mi = (*cm).mi;
        }
        (*pbi).ready_for_new_data = 0 as ::core::ffi::c_int;
    }
    return retcode;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_raw_frame(
    mut pbi: *mut VP8D_COMP,
    mut sd: *mut YV12_BUFFER_CONFIG,
    mut flags: *mut vp8_ppflags_t,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*pbi).ready_for_new_data == 1 as ::core::ffi::c_int {
        return ret;
    }
    if (*pbi).common.show_frame == 0 as ::core::ffi::c_int {
        return ret;
    }
    (*pbi).ready_for_new_data = 1 as ::core::ffi::c_int;
    if !(*pbi).common.frame_to_show.is_null() {
        *sd = *(*pbi).common.frame_to_show;
        (*sd).y_width = (*pbi).common.Width;
        (*sd).y_height = (*pbi).common.Height;
        (*sd).uv_height = (*pbi).common.Height / 2 as ::core::ffi::c_int;
        ret = 0 as ::core::ffi::c_int;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_references_buffer(
    mut oci: *mut VP8_COMMON,
    mut ref_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut mi: *const MODE_INFO = (*oci).mi;
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut mb_col: ::core::ffi::c_int = 0;
    mb_row = 0 as ::core::ffi::c_int;
    while mb_row < (*oci).mb_rows {
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*oci).mb_cols {
            if (*mi).mbmi.ref_frame as ::core::ffi::c_int == ref_frame {
                return 1 as ::core::ffi::c_int;
            }
            mb_col += 1;
            mi = mi.offset(1);
        }
        mi = mi.offset(1);
        mb_row += 1;
    }
    return 0 as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_create_decoder_instances(
    mut fb: *mut frame_buffers,
    mut oxcf: *mut VP8D_CONFIG,
) -> ::core::ffi::c_int { unsafe {
    (*fb).pbi[0 as ::core::ffi::c_int as usize] = create_decompressor(oxcf);
    if (*fb).pbi[0 as ::core::ffi::c_int as usize].is_null() {
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    if setjmp(
        &raw mut (**(&raw mut (*fb).pbi as *mut *mut VP8D_COMP)
            .offset(0 as ::core::ffi::c_int as isize))
        .common
        .error
        .jmp as *mut ::core::ffi::c_int,
    ) != 0
    {
        (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
            .common
            .error
            .setjmp = 0 as ::core::ffi::c_int;
        vp8_remove_decoder_instances(fb);
        memset(
            &raw mut (*fb).pbi as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut VP8D_COMP; 32]>() as size_t,
        );
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
        .common
        .error
        .setjmp = 1 as ::core::ffi::c_int;
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize]).max_threads = (*oxcf).max_threads;
    vp8_decoder_create_threads((*fb).pbi[0 as ::core::ffi::c_int as usize]);
    (*(*fb).pbi[0 as ::core::ffi::c_int as usize])
        .common
        .error
        .setjmp = 0 as ::core::ffi::c_int;
    return VPX_CODEC_OK as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_remove_decoder_instances(
    mut fb: *mut frame_buffers,
) -> ::core::ffi::c_int { unsafe {
    let mut pbi: *mut VP8D_COMP = (*fb).pbi[0 as ::core::ffi::c_int as usize];
    if pbi.is_null() {
        return VPX_CODEC_ERROR as ::core::ffi::c_int;
    }
    vp8_decoder_remove_threads(pbi);
    remove_decompressor(pbi);
    (*fb).pbi[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<VP8D_COMP>();
    return VPX_CODEC_OK as ::core::ffi::c_int;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8dx_get_quantizer(mut pbi: *const VP8D_COMP) -> ::core::ffi::c_int { unsafe {
    return (*pbi).common.base_qindex;
}}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
