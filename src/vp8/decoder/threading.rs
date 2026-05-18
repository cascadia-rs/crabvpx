use crate::vp8::decoder::detokenize::vp8_reset_mb_tokens_context;

unsafe extern "C" {
    fn vp8_dc_only_idct_add_neon(
        input_dc: ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_neon(
        input: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dest: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_uv_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst_u: *mut ::core::ffi::c_uchar,
        dst_v: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequant_idct_add_y_block_neon(
        q: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dst: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
        eobs: *mut ::core::ffi::c_char,
    );
    fn vp8_dequantize_b_neon(_: *mut blockd, DQC: *mut ::core::ffi::c_short);
    fn vp8_loop_filter_bh_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bv_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbh_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbv_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        u_ptr: *mut ::core::ffi::c_uchar,
        v_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bhs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_bvs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_mbhs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_loop_filter_mbvs_neon(
        y_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
        blimit: *const ::core::ffi::c_uchar,
    );
    fn vp8_short_inv_walsh4x4_neon(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn vp8_short_inv_walsh4x4_1_c(
        input: *mut ::core::ffi::c_short,
        mb_dqcoeff: *mut ::core::ffi::c_short,
    );
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vpx_internal_error(
        info: *mut vpx_internal_error_info,
        error: vpx_codec_err_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn vp8_loop_filter_frame_init(
        cm: *mut VP8Common,
        mbd: *mut macroblockd,
        default_filt_lvl: ::core::ffi::c_int,
    );
    fn vp8_setup_block_dptrs(x: *mut MACROBLOCKD);
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
    fn vp8_mb_init_dequantizer(pbi: &mut VP8D_COMP, xd: &mut MACROBLOCKD);
    fn vpx_memalign(align: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
    fn vp8_extend_mb_row(
        ybf: *mut YV12_BUFFER_CONFIG,
        YPtr: *mut ::core::ffi::c_uchar,
        UPtr: *mut ::core::ffi::c_uchar,
        VPtr: *mut ::core::ffi::c_uchar,
    );
    fn vp8_decode_mb_tokens(_: *mut VP8D_COMP, _: *mut MACROBLOCKD) -> ::core::ffi::c_int;
    fn vp8_intra4x4_predict(
        above: *mut ::core::ffi::c_uchar,
        yleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        b_mode: B_PREDICTION_MODE,
        dst: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
        top_left: ::core::ffi::c_uchar,
    );
    fn vp8_build_inter_predictors_mb(xd: *mut MACROBLOCKD);
    fn vp8_build_intra_predictors_mby_s(
        x: *mut MACROBLOCKD,
        yabove_row: *mut ::core::ffi::c_uchar,
        yleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        ypred_ptr: *mut ::core::ffi::c_uchar,
        y_stride: ::core::ffi::c_int,
    );
    fn vp8_build_intra_predictors_mbuv_s(
        x: *mut MACROBLOCKD,
        uabove_row: *mut ::core::ffi::c_uchar,
        vabove_row: *mut ::core::ffi::c_uchar,
        uleft: *mut ::core::ffi::c_uchar,
        vleft: *mut ::core::ffi::c_uchar,
        left_stride: ::core::ffi::c_int,
        upred_ptr: *mut ::core::ffi::c_uchar,
        vpred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
    );
    fn vp8_setup_intra_recon_top_line(ybf: *mut YV12_BUFFER_CONFIG);
}
static mut mach_task_self_: mach_port_t = 0;
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
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info {
    pub mblim: *const ::core::ffi::c_uchar,
    pub blim: *const ::core::ffi::c_uchar,
    pub lim: *const ::core::ffi::c_uchar,
    pub hev_thr: *const ::core::ffi::c_uchar,
}
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
pub struct _opaque_pthread_attr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type MV_REFERENCE_FRAME = ::core::ffi::c_uint;
pub const MAX_REF_FRAMES: MV_REFERENCE_FRAME = 4;
pub const ALTREF_FRAME: MV_REFERENCE_FRAME = 3;
pub const GOLDEN_FRAME: MV_REFERENCE_FRAME = 2;
pub const LAST_FRAME: MV_REFERENCE_FRAME = 1;
pub const INTRA_FRAME: MV_REFERENCE_FRAME = 0;
pub type kern_return_t = ::core::ffi::c_int;
pub type task_t = mach_port_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const THREAD_EXIT_SUCCESS: *mut ::core::ffi::c_void = NULL;
pub const CHAR_BIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8BORDERINPIXELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const VP8_BD_VALUE_SIZE: ::core::ffi::c_int =
    ::core::mem::size_of::<VP8_BD_VALUE>() as ::core::ffi::c_int * CHAR_BIT;
pub const VP8_LOTS_OF_BITS: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
#[inline]
fn vp8dx_bool_error(br: &BOOL_DECODER) -> ::core::ffi::c_int {
    if br.count > VP8_BD_VALUE_SIZE && br.count < VP8_LOTS_OF_BITS {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const SYNC_POLICY_FIFO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vpx_atomic_init(
    mut atomic: *mut vpx_atomic_int,
    mut value: ::core::ffi::c_int,
) { unsafe {
    ::core::ptr::write_volatile(&mut (*atomic).value as *mut ::core::ffi::c_int, value);
}}
#[inline]
unsafe extern "C" fn vpx_atomic_store_release(
    mut atomic: *mut vpx_atomic_int,
    mut value: ::core::ffi::c_int,
) { unsafe {
    (*(&raw mut (*atomic).value as *const core::sync::atomic::AtomicI32)).store(value, core::sync::atomic::Ordering::Release);
}}
#[inline]
unsafe extern "C" fn vpx_atomic_load_acquire(
    mut atomic: *const vpx_atomic_int,
) -> ::core::ffi::c_int { unsafe {
    return (*(&raw const (*atomic).value as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::Acquire);
}}
#[inline]
unsafe extern "C" fn vp8_atomic_spin_wait(
    mut mb_col: ::core::ffi::c_int,
    mut last_row_current_mb_col: *const vpx_atomic_int,
    nsync: ::core::ffi::c_int,
) { unsafe {
    while mb_col > vpx_atomic_load_acquire(last_row_current_mb_col) - nsync {}
}}
#[inline]
unsafe extern "C" fn intra_prediction_down_copy(
    mut xd: *mut MACROBLOCKD,
    mut above_right_src: *mut ::core::ffi::c_uchar,
) { unsafe {
    let mut dst_stride: ::core::ffi::c_int = (*xd).dst.y_stride;
    let mut above_right_dst: *mut ::core::ffi::c_uchar = (*xd)
        .dst
        .y_buffer
        .offset(-(dst_stride as isize))
        .offset(16 as ::core::ffi::c_int as isize);
    let mut src_ptr: *mut ::core::ffi::c_uint = above_right_src as *mut ::core::ffi::c_uint;
    let mut dst_ptr0: *mut ::core::ffi::c_uint = above_right_dst
        .offset((4 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr1: *mut ::core::ffi::c_uint = above_right_dst
        .offset((8 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    let mut dst_ptr2: *mut ::core::ffi::c_uint = above_right_dst
        .offset((12 as ::core::ffi::c_int * dst_stride) as isize)
        as *mut ::core::ffi::c_uint;
    *dst_ptr0 = *src_ptr;
    *dst_ptr1 = *src_ptr;
    *dst_ptr2 = *src_ptr;
}}
#[inline]
unsafe extern "C" fn setup_intra_recon_left(
    mut y_buffer: *mut ::core::ffi::c_uchar,
    mut u_buffer: *mut ::core::ffi::c_uchar,
    mut v_buffer: *mut ::core::ffi::c_uchar,
    mut y_stride: ::core::ffi::c_int,
    mut uv_stride: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *y_buffer.offset((y_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *u_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *v_buffer.offset((uv_stride * i) as isize) =
            129 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        i += 1;
    }
}}
unsafe extern "C" fn setup_decoding_thread_data(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut mbrd: *mut MB_ROW_DEC,
    mut count: ::core::ffi::c_int,
) { unsafe {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let mut mbd: *mut MACROBLOCKD = &raw mut (*mbrd.offset(i as isize)).mbd;
        (*mbd).subpixel_predict = (*xd).subpixel_predict;
        (*mbd).subpixel_predict8x4 = (*xd).subpixel_predict8x4;
        (*mbd).subpixel_predict8x8 = (*xd).subpixel_predict8x8;
        (*mbd).subpixel_predict16x16 = (*xd).subpixel_predict16x16;
        (*mbd).frame_type = (*pc).frame_type;
        (*mbd).pre = (*xd).pre;
        (*mbd).dst = (*xd).dst;
        (*mbd).segmentation_enabled = (*xd).segmentation_enabled;
        (*mbd).mb_segment_abs_delta = (*xd).mb_segment_abs_delta;
        memcpy(
            &raw mut (*mbd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                as *mut ::core::ffi::c_void,
            &raw mut (*xd).segment_feature_data as *mut [::core::ffi::c_schar; 4]
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[::core::ffi::c_schar; 4]; 2]>() as size_t,
        );
        memcpy(
            &raw mut (*mbd).ref_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            &raw mut (*xd).ref_lf_deltas as *mut ::core::ffi::c_schar as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        memcpy(
            &raw mut (*mbd).mode_lf_deltas as *mut ::core::ffi::c_schar as *mut ::core::ffi::c_void,
            &raw mut (*xd).mode_lf_deltas as *mut ::core::ffi::c_schar
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_schar; 4]>() as size_t,
        );
        (*mbd).mode_ref_lf_delta_enabled = (*xd).mode_ref_lf_delta_enabled;
        (*mbd).mode_ref_lf_delta_update = (*xd).mode_ref_lf_delta_update;
        (*mbd).current_bc_idx = 0;
        memcpy(
            &raw mut (*mbd).dequant_y1_dc as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
            &raw mut (*xd).dequant_y1_dc as *mut ::core::ffi::c_short as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_short; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*mbd).dequant_y1 as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
            &raw mut (*xd).dequant_y1 as *mut ::core::ffi::c_short as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_short; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*mbd).dequant_y2 as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
            &raw mut (*xd).dequant_y2 as *mut ::core::ffi::c_short as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_short; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*mbd).dequant_uv as *mut ::core::ffi::c_short as *mut ::core::ffi::c_void,
            &raw mut (*xd).dequant_uv as *mut ::core::ffi::c_short as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_short; 16]>() as size_t,
        );
        (*mbd).fullpixel_mask = !(0 as ::core::ffi::c_int);
        if (*pc).full_pixel != 0 {
            (*mbd).fullpixel_mask = !(7 as ::core::ffi::c_int);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pc).mb_rows {
        vpx_atomic_store_release(
            (*pbi).mt_current_mb_col.offset(i as isize) as *mut vpx_atomic_int,
            -(1 as ::core::ffi::c_int),
        );
        i += 1;
    }
}}
unsafe extern "C" fn mt_decode_macroblock(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut mb_idx: ::core::ffi::c_uint,
) { unsafe {
    let mut mode: MB_PREDICTION_MODE = DC_PRED;
    let mut i: ::core::ffi::c_int = 0;
    if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
        vp8_reset_mb_tokens_context(&mut *xd);
    } else if vp8dx_bool_error(&(*pbi).mbc[(*xd).current_bc_idx]) == 0 {
        let mut eobtotal: ::core::ffi::c_int = 0;
        eobtotal = vp8_decode_mb_tokens(pbi, xd);
        (*(*xd).mode_info_context).mbmi.mb_skip_coeff =
            (eobtotal == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
    }
    mode = (*(*xd).mode_info_context).mbmi.mode as MB_PREDICTION_MODE;
    if (*xd).segmentation_enabled != 0 {
        vp8_mb_init_dequantizer(&mut *pbi, &mut *xd);
    }
    if (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
        == INTRA_FRAME as ::core::ffi::c_int
    {
        vp8_build_intra_predictors_mbuv_s(
            xd,
            (*xd).recon_above[1 as ::core::ffi::c_int as usize],
            (*xd).recon_above[2 as ::core::ffi::c_int as usize],
            (*xd).recon_left[1 as ::core::ffi::c_int as usize],
            (*xd).recon_left[2 as ::core::ffi::c_int as usize],
            (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize],
            (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.uv_stride,
        );
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            vp8_build_intra_predictors_mby_s(
                xd,
                (*xd).recon_above[0 as ::core::ffi::c_int as usize],
                (*xd).recon_left[0 as ::core::ffi::c_int as usize],
                (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize],
                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                (*xd).dst.y_stride,
            );
        } else {
            let mut DQC: *mut ::core::ffi::c_short =
                &raw mut (*xd).dequant_y1 as *mut ::core::ffi::c_short;
            let mut dst_stride: ::core::ffi::c_int = (*xd).dst.y_stride;
            if (*(*xd).mode_info_context).mbmi.mb_skip_coeff != 0 {
                memset(
                    &raw mut (*xd).eobs as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    25 as size_t,
                );
            }
            intra_prediction_down_copy(
                xd,
                (*xd).recon_above[0 as ::core::ffi::c_int as usize]
                    .offset(16 as ::core::ffi::c_int as isize),
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut b: *mut BLOCKD =
                    (&raw mut (*xd).block as *mut BLOCKD).offset(i as isize) as *mut BLOCKD;
                let mut dst: *mut ::core::ffi::c_uchar =
                    (*xd).dst.y_buffer.offset((*b).offset as isize);
                let mut b_mode: B_PREDICTION_MODE =
                    (*(*xd).mode_info_context).bmi[i as usize].as_mode;
                let mut Above: *mut ::core::ffi::c_uchar =
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                let mut yleft: *mut ::core::ffi::c_uchar =
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                let mut left_stride: ::core::ffi::c_int = 0;
                let mut top_left: ::core::ffi::c_uchar = 0;
                if i < 4 as ::core::ffi::c_int && (*pbi).common.filter_level != 0 {
                    Above = (*xd).recon_above[0 as ::core::ffi::c_int as usize]
                        .offset((*b).offset as isize);
                } else {
                    Above = dst.offset(-(dst_stride as isize));
                }
                if i % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pbi).common.filter_level != 0
                {
                    yleft = (*xd).recon_left[0 as ::core::ffi::c_int as usize].offset(i as isize);
                    left_stride = 1 as ::core::ffi::c_int;
                } else {
                    yleft = dst.offset(-(1 as ::core::ffi::c_int as isize));
                    left_stride = dst_stride;
                }
                if (i == 4 as ::core::ffi::c_int
                    || i == 8 as ::core::ffi::c_int
                    || i == 12 as ::core::ffi::c_int)
                    && (*pbi).common.filter_level != 0
                {
                    top_left = *(*xd).recon_left[0 as ::core::ffi::c_int as usize]
                        .offset(i as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize));
                } else {
                    top_left = *Above.offset(-(1 as ::core::ffi::c_int) as isize);
                }
                vp8_intra4x4_predict(Above, yleft, left_stride, b_mode, dst, dst_stride, top_left);
                if (*xd).eobs[i as usize] != 0 {
                    if (*xd).eobs[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        vp8_dequant_idct_add_neon((*b).qcoeff, DQC, dst, dst_stride);
                    } else {
                        vp8_dc_only_idct_add_neon(
                            (*(*b).qcoeff.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                * *DQC.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)
                                as ::core::ffi::c_short,
                            dst,
                            dst_stride,
                            dst,
                            dst_stride,
                        );
                        memset(
                            (*b).qcoeff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_short>() as size_t,
                            ),
                        );
                    }
                }
                i += 1;
            }
        }
    } else {
        vp8_build_inter_predictors_mb(xd);
    }
    if (*(*xd).mode_info_context).mbmi.mb_skip_coeff == 0 {
        if mode as ::core::ffi::c_uint != B_PRED as ::core::ffi::c_int as ::core::ffi::c_uint {
            let mut DQC_0: *mut ::core::ffi::c_short =
                &raw mut (*xd).dequant_y1 as *mut ::core::ffi::c_short;
            if mode as ::core::ffi::c_uint != SPLITMV as ::core::ffi::c_int as ::core::ffi::c_uint {
                let mut b_0: *mut BLOCKD = (&raw mut (*xd).block as *mut BLOCKD)
                    .offset(24 as ::core::ffi::c_int as isize)
                    as *mut BLOCKD;
                if (*xd).eobs[24 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                {
                    vp8_dequantize_b_neon(
                        b_0 as *mut blockd,
                        &raw mut (*xd).dequant_y2 as *mut ::core::ffi::c_short,
                    );
                    vp8_short_inv_walsh4x4_neon(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (16 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                } else {
                    *(*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize) = (*(*b_0)
                        .qcoeff
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        * (*xd).dequant_y2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        as ::core::ffi::c_short;
                    vp8_short_inv_walsh4x4_1_c(
                        (*b_0).dqcoeff.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_short,
                        &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                    );
                    memset(
                        (*b_0).qcoeff as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                DQC_0 = &raw mut (*xd).dequant_y1_dc as *mut ::core::ffi::c_short;
            }
            vp8_dequant_idct_add_y_block_neon(
                &raw mut (*xd).qcoeff as *mut ::core::ffi::c_short,
                DQC_0,
                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                (*xd).dst.y_stride,
                &raw mut (*xd).eobs as *mut ::core::ffi::c_char,
            );
        }
        vp8_dequant_idct_add_uv_block_neon(
            (&raw mut (*xd).qcoeff as *mut ::core::ffi::c_short)
                .offset((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            &raw mut (*xd).dequant_uv as *mut ::core::ffi::c_short,
            (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
            (*xd).dst.uv_stride,
            (&raw mut (*xd).eobs as *mut ::core::ffi::c_char)
                .offset(16 as ::core::ffi::c_int as isize),
        );
    }
}}
unsafe extern "C" fn mt_decode_mb_rows(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
    mut start_mb_row: ::core::ffi::c_int,
) { unsafe {
    let mut last_row_current_mb_col: *const vpx_atomic_int = ::core::ptr::null::<vpx_atomic_int>();
    let mut current_mb_col: *mut vpx_atomic_int = ::core::ptr::null_mut::<vpx_atomic_int>();
    let mut mb_row: ::core::ffi::c_int = 0;
    let mut pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let nsync: ::core::ffi::c_int = (*pbi).sync_range;
    let first_row_no_sync_above: vpx_atomic_int = vpx_atomic_int {
        value: (*pc).mb_cols + nsync,
    };
    let mut num_part: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << (*pbi).common.multi_token_partition as ::core::ffi::c_uint;
    let mut last_mb_row: ::core::ffi::c_int = start_mb_row;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    let mut yv12_fb_lst: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[LAST_FRAME as ::core::ffi::c_int as usize];
    let mut recon_y_stride: ::core::ffi::c_int = (*yv12_fb_new).y_stride;
    let mut recon_uv_stride: ::core::ffi::c_int = (*yv12_fb_new).uv_stride;
    let mut ref_buffer: [[*mut ::core::ffi::c_uchar; 3]; 4] =
        [[::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3]; 4];
    let mut dst_buffer: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut i: ::core::ffi::c_int = 0;
    let mut ref_fb_corrupted: [::core::ffi::c_int; 4] = [0; 4];
    ref_fb_corrupted[INTRA_FRAME as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < MAX_REF_FRAMES as ::core::ffi::c_int {
        let mut this_fb: *mut YV12_BUFFER_CONFIG = (*pbi).dec_fb_ref[i as usize];
        ref_buffer[i as usize][0 as ::core::ffi::c_int as usize] =
            (*this_fb).y_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][1 as ::core::ffi::c_int as usize] =
            (*this_fb).u_buffer as *mut ::core::ffi::c_uchar;
        ref_buffer[i as usize][2 as ::core::ffi::c_int as usize] =
            (*this_fb).v_buffer as *mut ::core::ffi::c_uchar;
        ref_fb_corrupted[i as usize] = (*this_fb).corrupted;
        i += 1;
    }
    dst_buffer[0 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).y_buffer as *mut ::core::ffi::c_uchar;
    dst_buffer[1 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).u_buffer as *mut ::core::ffi::c_uchar;
    dst_buffer[2 as ::core::ffi::c_int as usize] =
        (*yv12_fb_new).v_buffer as *mut ::core::ffi::c_uchar;
    (*xd).up_available = (start_mb_row != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*xd).mode_info_context = (*pc)
        .mi
        .offset(((*pc).mode_info_stride * start_mb_row) as isize);
    (*xd).mode_info_stride = (*pc).mode_info_stride;
    mb_row = start_mb_row;
    while mb_row < (*pc).mb_rows {
        let mut recon_yoffset: ::core::ffi::c_int = 0;
        let mut recon_uvoffset: ::core::ffi::c_int = 0;
        let mut mb_col: ::core::ffi::c_int = 0;
        let mut filter_level: ::core::ffi::c_int = 0;
        let mut lfi_n: *mut loop_filter_info_n = &raw mut (*pc).lf_info;
        last_mb_row = mb_row;
        (*xd).current_bc_idx = (mb_row % num_part) as usize;
        if mb_row > 0 as ::core::ffi::c_int {
            last_row_current_mb_col = (*pbi)
                .mt_current_mb_col
                .offset((mb_row - 1 as ::core::ffi::c_int) as isize)
                as *mut vpx_atomic_int;
        } else {
            last_row_current_mb_col = &raw const first_row_no_sync_above;
        }
        current_mb_col = (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut vpx_atomic_int;
        recon_yoffset = mb_row * recon_y_stride * 16 as ::core::ffi::c_int;
        recon_uvoffset = mb_row * recon_uv_stride * 8 as ::core::ffi::c_int;
        (*xd).above_context = (*pc).above_context;
        memset(
            (*xd).left_context as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t,
        );
        (*xd).left_available = 0 as ::core::ffi::c_int;
        (*xd).mb_to_top_edge = -((mb_row * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
        (*xd).mb_to_bottom_edge = (((*pc).mb_rows - 1 as ::core::ffi::c_int - mb_row)
            * 16 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int;
        if (*pbi).common.filter_level != 0 {
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] =
                (*(*pbi).mt_yabove_row.offset(mb_row as isize))
                    .offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
                    .offset(32 as ::core::ffi::c_int as isize);
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] =
                (*(*pbi).mt_uabove_row.offset(mb_row as isize))
                    .offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                    .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] =
                (*(*pbi).mt_vabove_row.offset(mb_row as isize))
                    .offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                    .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_left[0 as ::core::ffi::c_int as usize] =
                *(*pbi).mt_yleft_col.offset(mb_row as isize);
            (*xd).recon_left[1 as ::core::ffi::c_int as usize] =
                *(*pbi).mt_uleft_col.offset(mb_row as isize);
            (*xd).recon_left[2 as ::core::ffi::c_int as usize] =
                *(*pbi).mt_vleft_col.offset(mb_row as isize);
            (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
            (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        } else {
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] =
                dst_buffer[0 as ::core::ffi::c_int as usize].offset(recon_yoffset as isize);
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] =
                dst_buffer[1 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] =
                dst_buffer[2 as ::core::ffi::c_int as usize].offset(recon_uvoffset as isize);
            (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [0 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize));
            (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [1 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize));
            (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [2 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize));
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [0 as ::core::ffi::c_int as usize]
                .offset(-((*xd).dst.y_stride as isize));
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [1 as ::core::ffi::c_int as usize]
                .offset(-((*xd).dst.uv_stride as isize));
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [2 as ::core::ffi::c_int as usize]
                .offset(-((*xd).dst.uv_stride as isize));
            (*xd).recon_left_stride[0 as ::core::ffi::c_int as usize] = (*xd).dst.y_stride;
            (*xd).recon_left_stride[1 as ::core::ffi::c_int as usize] = (*xd).dst.uv_stride;
            setup_intra_recon_left(
                (*xd).recon_left[0 as ::core::ffi::c_int as usize],
                (*xd).recon_left[1 as ::core::ffi::c_int as usize],
                (*xd).recon_left[2 as ::core::ffi::c_int as usize],
                (*xd).dst.y_stride,
                (*xd).dst.uv_stride,
            );
        }
        mb_col = 0 as ::core::ffi::c_int;
        while mb_col < (*pc).mb_cols {
            if (mb_col - 1 as ::core::ffi::c_int) % nsync == 0 as ::core::ffi::c_int {
                vpx_atomic_store_release(current_mb_col, mb_col - 1 as ::core::ffi::c_int);
            }
            if mb_row != 0 && mb_col & nsync - 1 as ::core::ffi::c_int == 0 {
                vp8_atomic_spin_wait(mb_col, last_row_current_mb_col, nsync);
            }
            (*xd).mb_to_left_edge =
                -((mb_col * 16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
            (*xd).mb_to_right_edge = (((*pc).mb_cols - 1 as ::core::ffi::c_int - mb_col)
                * 16 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int;
            (*xd).dst.y_buffer = dst_buffer[0 as ::core::ffi::c_int as usize]
                .offset(recon_yoffset as isize) as *mut uint8_t;
            (*xd).dst.u_buffer = dst_buffer[1 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            (*xd).dst.v_buffer = dst_buffer[2 as ::core::ffi::c_int as usize]
                .offset(recon_uvoffset as isize) as *mut uint8_t;
            (*xd).corrupted |= ref_fb_corrupted[(*(*xd).mode_info_context).mbmi.ref_frame as usize];
            if (*xd).corrupted != 0 {
                while mb_row < (*pc).mb_rows {
                    current_mb_col =
                        (*pbi).mt_current_mb_col.offset(mb_row as isize) as *mut vpx_atomic_int;
                    vpx_atomic_store_release(current_mb_col, (*pc).mb_cols + nsync);
                    mb_row = (mb_row as ::core::ffi::c_uint).wrapping_add(
                        (*pbi)
                            .decoding_thread_count
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                }
                vpx_internal_error(
                    &raw mut (*xd).error_info,
                    VPX_CODEC_CORRUPT_FRAME,
                    b"Corrupted reference frame\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int
                >= LAST_FRAME as ::core::ffi::c_int
            {
                let ref_0: MV_REFERENCE_FRAME =
                    (*(*xd).mode_info_context).mbmi.ref_frame as MV_REFERENCE_FRAME;
                (*xd).pre.y_buffer = ref_buffer[ref_0 as usize][0 as ::core::ffi::c_int as usize]
                    .offset(recon_yoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.u_buffer = ref_buffer[ref_0 as usize][1 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
                (*xd).pre.v_buffer = ref_buffer[ref_0 as usize][2 as ::core::ffi::c_int as usize]
                    .offset(recon_uvoffset as isize)
                    as *mut uint8_t;
            } else {
                (*xd).pre.y_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.u_buffer = ::core::ptr::null_mut::<uint8_t>();
                (*xd).pre.v_buffer = ::core::ptr::null_mut::<uint8_t>();
            }
            mt_decode_macroblock(pbi, xd, 0 as ::core::ffi::c_uint);
            (*xd).left_available = 1 as ::core::ffi::c_int;
            (*xd).corrupted |= vp8dx_bool_error(&(*pbi).mbc[(*xd).current_bc_idx]);
            (*xd).recon_above[0 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [0 as ::core::ffi::c_int as usize]
                .offset(16 as ::core::ffi::c_int as isize);
            (*xd).recon_above[1 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [1 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            (*xd).recon_above[2 as ::core::ffi::c_int as usize] = (*xd).recon_above
                [2 as ::core::ffi::c_int as usize]
                .offset(8 as ::core::ffi::c_int as isize);
            if (*pbi).common.filter_level == 0 {
                (*xd).recon_left[0 as ::core::ffi::c_int as usize] = (*xd).recon_left
                    [0 as ::core::ffi::c_int as usize]
                    .offset(16 as ::core::ffi::c_int as isize);
                (*xd).recon_left[1 as ::core::ffi::c_int as usize] = (*xd).recon_left
                    [1 as ::core::ffi::c_int as usize]
                    .offset(8 as ::core::ffi::c_int as isize);
                (*xd).recon_left[2 as ::core::ffi::c_int as usize] = (*xd).recon_left
                    [2 as ::core::ffi::c_int as usize]
                    .offset(8 as ::core::ffi::c_int as isize);
            }
            if (*pbi).common.filter_level != 0 {
                let mut skip_lf: ::core::ffi::c_int = ((*(*xd).mode_info_context).mbmi.mode
                    as ::core::ffi::c_int
                    != B_PRED as ::core::ffi::c_int
                    && (*(*xd).mode_info_context).mbmi.mode as ::core::ffi::c_int
                        != SPLITMV as ::core::ffi::c_int
                    && (*(*xd).mode_info_context).mbmi.mb_skip_coeff as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                let mode_index: ::core::ffi::c_int = (*lfi_n).mode_lf_lut
                    [(*(*xd).mode_info_context).mbmi.mode as usize]
                    as ::core::ffi::c_int;
                let seg: ::core::ffi::c_int =
                    (*(*xd).mode_info_context).mbmi.segment_id as ::core::ffi::c_int;
                let ref_frame: ::core::ffi::c_int =
                    (*(*xd).mode_info_context).mbmi.ref_frame as ::core::ffi::c_int;
                filter_level = (*lfi_n).lvl[seg as usize][ref_frame as usize][mode_index as usize]
                    as ::core::ffi::c_int;
                if mb_row != (*pc).mb_rows - 1 as ::core::ffi::c_int {
                    memcpy(
                        (*(*pbi)
                            .mt_yabove_row
                            .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                        .offset(32 as ::core::ffi::c_int as isize)
                        .offset((mb_col * 16 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_void,
                        (*xd)
                            .dst
                            .y_buffer
                            .offset((15 as ::core::ffi::c_int * recon_y_stride) as isize)
                            as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    memcpy(
                        (*(*pbi)
                            .mt_uabove_row
                            .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                        .offset(16 as ::core::ffi::c_int as isize)
                        .offset((mb_col * 8 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_void,
                        (*xd)
                            .dst
                            .u_buffer
                            .offset((7 as ::core::ffi::c_int * recon_uv_stride) as isize)
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        (*(*pbi)
                            .mt_vabove_row
                            .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                        .offset(16 as ::core::ffi::c_int as isize)
                        .offset((mb_col * 8 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_void,
                        (*xd)
                            .dst
                            .v_buffer
                            .offset((7 as ::core::ffi::c_int * recon_uv_stride) as isize)
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                }
                if mb_col != (*pc).mb_cols - 1 as ::core::ffi::c_int {
                    let mut next: *mut MODE_INFO = (*xd)
                        .mode_info_context
                        .offset(1 as ::core::ffi::c_int as isize);
                    if (*next).mbmi.ref_frame as ::core::ffi::c_int
                        == INTRA_FRAME as ::core::ffi::c_int
                    {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            *(*(*pbi).mt_yleft_col.offset(mb_row as isize)).offset(i as isize) =
                                *(*xd).dst.y_buffer.offset(
                                    (i * recon_y_stride + 15 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_uchar;
                            i += 1;
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < 8 as ::core::ffi::c_int {
                            *(*(*pbi).mt_uleft_col.offset(mb_row as isize)).offset(i as isize) =
                                *(*xd).dst.u_buffer.offset(
                                    (i * recon_uv_stride + 7 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_uchar;
                            *(*(*pbi).mt_vleft_col.offset(mb_row as isize)).offset(i as isize) =
                                *(*xd).dst.v_buffer.offset(
                                    (i * recon_uv_stride + 7 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_uchar;
                            i += 1;
                        }
                    }
                }
                if filter_level != 0 {
                    if (*pc).filter_type as ::core::ffi::c_uint
                        == NORMAL_LOOPFILTER as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let mut lfi: loop_filter_info = loop_filter_info {
                            mblim: ::core::ptr::null::<::core::ffi::c_uchar>(),
                            blim: ::core::ptr::null::<::core::ffi::c_uchar>(),
                            lim: ::core::ptr::null::<::core::ffi::c_uchar>(),
                            hev_thr: ::core::ptr::null::<::core::ffi::c_uchar>(),
                        };
                        let mut frame_type: FRAME_TYPE = (*pc).frame_type;
                        let hev_index: ::core::ffi::c_int = (*lfi_n).hev_thr_lut
                            [frame_type as usize][filter_level as usize]
                            as ::core::ffi::c_int;
                        lfi.mblim = &raw mut *(&raw mut (*lfi_n).mblim
                            as *mut [::core::ffi::c_uchar; 1])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.blim = &raw mut *(&raw mut (*lfi_n).blim
                            as *mut [::core::ffi::c_uchar; 1])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.lim = &raw mut *(&raw mut (*lfi_n).lim
                            as *mut [::core::ffi::c_uchar; 1])
                            .offset(filter_level as isize)
                            as *mut ::core::ffi::c_uchar;
                        lfi.hev_thr = &raw mut *(&raw mut (*lfi_n).hev_thr
                            as *mut [::core::ffi::c_uchar; 1])
                            .offset(hev_index as isize)
                            as *mut ::core::ffi::c_uchar;
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbv_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                recon_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bv_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                recon_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbh_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                recon_uv_stride,
                                &raw mut lfi,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bh_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.u_buffer as *mut ::core::ffi::c_uchar,
                                (*xd).dst.v_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                recon_uv_stride,
                                &raw mut lfi,
                            );
                        }
                    } else {
                        if mb_col > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbvs_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim
                                    as *mut [::core::ffi::c_uchar; 1])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bvs_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if mb_row > 0 as ::core::ffi::c_int {
                            vp8_loop_filter_mbhs_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                &raw mut *(&raw mut (*lfi_n).mblim
                                    as *mut [::core::ffi::c_uchar; 1])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                        if skip_lf == 0 {
                            vp8_loop_filter_bhs_neon(
                                (*xd).dst.y_buffer as *mut ::core::ffi::c_uchar,
                                recon_y_stride,
                                &raw mut *(&raw mut (*lfi_n).blim as *mut [::core::ffi::c_uchar; 1])
                                    .offset(filter_level as isize)
                                    as *mut ::core::ffi::c_uchar,
                            );
                        }
                    }
                }
            }
            recon_yoffset += 16 as ::core::ffi::c_int;
            recon_uvoffset += 8 as ::core::ffi::c_int;
            (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
            (*xd).above_context = (*xd).above_context.offset(1);
            mb_col += 1;
        }
        if (*pbi).common.filter_level != 0 {
            if mb_row != (*pc).mb_rows - 1 as ::core::ffi::c_int {
                let mut lasty: ::core::ffi::c_int = (*yv12_fb_lst).y_width + VP8BORDERINPIXELS;
                let mut lastuv: ::core::ffi::c_int = ((*yv12_fb_lst).y_width
                    >> 1 as ::core::ffi::c_int)
                    + (VP8BORDERINPIXELS >> 1 as ::core::ffi::c_int);
                i = 0 as ::core::ffi::c_int;
                while i < 4 as ::core::ffi::c_int {
                    *(*(*pbi)
                        .mt_yabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lasty + i) as isize) = *(*(*pbi)
                        .mt_yabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lasty - 1 as ::core::ffi::c_int) as isize);
                    *(*(*pbi)
                        .mt_uabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lastuv + i) as isize) = *(*(*pbi)
                        .mt_uabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lastuv - 1 as ::core::ffi::c_int) as isize);
                    *(*(*pbi)
                        .mt_vabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lastuv + i) as isize) = *(*(*pbi)
                        .mt_vabove_row
                        .offset((mb_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((lastuv - 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
            }
        } else {
            vp8_extend_mb_row(
                yv12_fb_new,
                (*xd).dst.y_buffer.offset(16 as ::core::ffi::c_int as isize),
                (*xd).dst.u_buffer.offset(8 as ::core::ffi::c_int as isize),
                (*xd).dst.v_buffer.offset(8 as ::core::ffi::c_int as isize),
            );
        }
        vpx_atomic_store_release(current_mb_col, mb_col + nsync);
        (*xd).mode_info_context = (*xd).mode_info_context.offset(1);
        (*xd).up_available = 1 as ::core::ffi::c_int;
        (*xd).mode_info_context = (*xd).mode_info_context.offset(
            ((*xd).mode_info_stride as ::core::ffi::c_uint)
                .wrapping_mul((*pbi).decoding_thread_count) as isize,
        );
        mb_row = (mb_row as ::core::ffi::c_uint).wrapping_add(
            (*pbi)
                .decoding_thread_count
                .wrapping_add(1 as ::core::ffi::c_uint),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if last_mb_row + (*pbi).decoding_thread_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        >= (*pc).mb_rows
    {
        crate::thread_shim::vp8_semaphore_signal((*pbi).h_event_end_decoding);
    }
}}
unsafe extern "C" fn thread_decoding_proc(
    mut p_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void { unsafe {
    let mut ithread: ::core::ffi::c_int = (*(p_data as *mut DECODETHREAD_DATA)).ithread;
    let mut pbi: *mut VP8D_COMP = (*(p_data as *mut DECODETHREAD_DATA)).ptr1 as *mut VP8D_COMP;
    let mut mbrd: *mut MB_ROW_DEC = (*(p_data as *mut DECODETHREAD_DATA)).ptr2 as *mut MB_ROW_DEC;
    let mut mb_row_left_context: ENTROPY_CONTEXT_PLANES = ENTROPY_CONTEXT_PLANES {
        y1: [0; 4],
        u: [0; 2],
        v: [0; 2],
        y2: 0,
    };
    while !(vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) == 0 as ::core::ffi::c_int)
    {
        if !(crate::thread_shim::vp8_semaphore_wait(*(*pbi).h_event_start_decoding.offset(ithread as isize))
            == 0 as ::core::ffi::c_int)
        {
            continue;
        }
        if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) == 0 as ::core::ffi::c_int {
            break;
        }
        let mut xd: *mut MACROBLOCKD = &raw mut (*mbrd).mbd;
        (*xd).left_context = &raw mut mb_row_left_context;
        if setjmp(&raw mut (*xd).error_info.jmp as *mut ::core::ffi::c_int) != 0 {
            (*xd).error_info.setjmp = 0 as ::core::ffi::c_int;
            crate::thread_shim::vp8_semaphore_signal((*pbi).h_event_end_decoding);
        } else {
            (*xd).error_info.setjmp = 1 as ::core::ffi::c_int;
            mt_decode_mb_rows(pbi, xd, ithread + 1 as ::core::ffi::c_int);
            (*xd).error_info.setjmp = 0 as ::core::ffi::c_int;
        }
    }
    return THREAD_EXIT_SUCCESS;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_decoder_create_threads(mut pbi: *mut VP8D_COMP) { unsafe {
    let mut core_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ithread: ::core::ffi::c_uint = 0;
    vpx_atomic_init(&raw mut (*pbi).b_multithreaded_rd, 0 as ::core::ffi::c_int);
    (*pbi).allocated_decoding_thread_count = 0 as ::core::ffi::c_int;
    core_count = if (*pbi).max_threads > 8 as ::core::ffi::c_int {
        8 as ::core::ffi::c_int
    } else {
        (*pbi).max_threads
    };
    if core_count > (*pbi).common.processor_core_count {
        core_count = (*pbi).common.processor_core_count;
    }
    if core_count > 1 as ::core::ffi::c_int {
        vpx_atomic_init(&raw mut (*pbi).b_multithreaded_rd, 1 as ::core::ffi::c_int);
        (*pbi).decoding_thread_count =
            (core_count - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        let count = (*pbi).decoding_thread_count as usize;
        (*pbi).h_decoding_thread = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_void>(); count].into_boxed_slice()) as *mut pthread_t;
        if (*pbi).h_decoding_thread.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->h_decoding_thread)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*pbi).h_event_start_decoding = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_void>(); count].into_boxed_slice()) as *mut semaphore_t;
        if (*pbi).h_event_start_decoding.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->h_event_start_decoding)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*pbi).mb_row_di = match crate::vpx_mem::vpx_mem::AlignedBox::new(32, core::mem::size_of::<MB_ROW_DEC>() * count) {
            Some(b) => b.into_raw() as *mut MB_ROW_DEC,
            None => core::ptr::null_mut(),
        };
        if (*pbi).mb_row_di.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mb_row_di)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        memset(
            (*pbi).mb_row_di as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*pbi).decoding_thread_count as size_t)
                .wrapping_mul(::core::mem::size_of::<MB_ROW_DEC>() as size_t),
        );
        (*pbi).de_thread_data = Box::into_raw(vec![DECODETHREAD_DATA { ithread: 0, ptr1: core::ptr::null_mut(), ptr2: core::ptr::null_mut() }; count].into_boxed_slice()) as *mut DECODETHREAD_DATA;
        if (*pbi).de_thread_data.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->de_thread_data)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if crate::thread_shim::vp8_semaphore_create(
            mach_task_self_ as task_t,
            &raw mut (*pbi).h_event_end_decoding,
            SYNC_POLICY_FIFO,
            0 as ::core::ffi::c_int,
        ) != 0
        {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to initialize semaphore\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ithread = 0 as ::core::ffi::c_uint;
        while ithread < (*pbi).decoding_thread_count {
            if crate::thread_shim::vp8_semaphore_create(
                mach_task_self_ as task_t,
                (*pbi).h_event_start_decoding.offset(ithread as isize) as *mut semaphore_t,
                SYNC_POLICY_FIFO,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                break;
            }
            vp8_setup_block_dptrs(&raw mut (*(*pbi).mb_row_di.offset(ithread as isize)).mbd);
            (*(*pbi).de_thread_data.offset(ithread as isize)).ithread =
                ithread as ::core::ffi::c_int;
            let ref mut fresh6 = (*(*pbi).de_thread_data.offset(ithread as isize)).ptr1;
            *fresh6 = pbi as *mut ::core::ffi::c_void;
            let ref mut fresh7 = (*(*pbi).de_thread_data.offset(ithread as isize)).ptr2;
            *fresh7 = (*pbi).mb_row_di.offset(ithread as isize) as *mut MB_ROW_DEC
                as *mut ::core::ffi::c_void;
            if crate::thread_shim::vp8_pthread_create(
                (*pbi).h_decoding_thread.offset(ithread as isize) as *mut pthread_t,
                ::core::ptr::null::<pthread_attr_t>() as *const ::core::ffi::c_void,
                Some(
                    thread_decoding_proc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
                (*pbi).de_thread_data.offset(ithread as isize) as *mut DECODETHREAD_DATA
                    as *mut ::core::ffi::c_void,
            ) != 0
            {
                crate::thread_shim::vp8_semaphore_destroy(
                    mach_task_self_ as task_t,
                    *(*pbi).h_event_start_decoding.offset(ithread as isize),
                );
                break;
            } else {
                ithread = ithread.wrapping_add(1);
            }
        }
        (*pbi).allocated_decoding_thread_count = ithread as ::core::ffi::c_int;
        if (*pbi).allocated_decoding_thread_count
            != (*pbi).decoding_thread_count as ::core::ffi::c_int
        {
            if (*pbi).allocated_decoding_thread_count == 0 as ::core::ffi::c_int {
                crate::thread_shim::vp8_semaphore_destroy(mach_task_self_ as task_t, (*pbi).h_event_end_decoding);
            }
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to create threads\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8mt_de_alloc_temp_buffers(
    mut pbi: *mut VP8D_COMP,
    mut mb_rows: ::core::ffi::c_int,
) { unsafe {
    let mb_rows_usize = mb_rows as usize;
    if !(*pbi).mt_current_mb_col.is_null() {
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_current_mb_col, mb_rows_usize));
        (*pbi).mt_current_mb_col = ::core::ptr::null_mut::<vpx_atomic_int>();
    }
    if !(*pbi).mt_yabove_row.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_yabove_row.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_yabove_row.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_yabove_row, mb_rows_usize));
        (*pbi).mt_yabove_row = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
    if !(*pbi).mt_uabove_row.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_uabove_row.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_uabove_row.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_uabove_row, mb_rows_usize));
        (*pbi).mt_uabove_row = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
    if !(*pbi).mt_vabove_row.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_vabove_row.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_vabove_row.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_vabove_row, mb_rows_usize));
        (*pbi).mt_vabove_row = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
    if !(*pbi).mt_yleft_col.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_yleft_col.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_yleft_col.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_yleft_col, mb_rows_usize));
        (*pbi).mt_yleft_col = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
    if !(*pbi).mt_uleft_col.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_uleft_col.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_uleft_col.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_uleft_col, mb_rows_usize));
        (*pbi).mt_uleft_col = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
    if !(*pbi).mt_vleft_col.is_null() {
        for i in 0..mb_rows_usize {
            let ptr = *(*pbi).mt_vleft_col.add(i);
            if !ptr.is_null() {
                let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw(ptr);
                *(*pbi).mt_vleft_col.add(i) = core::ptr::null_mut();
            }
        }
        let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).mt_vleft_col, mb_rows_usize));
        (*pbi).mt_vleft_col = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8mt_alloc_temp_buffers(
    mut pbi: *mut VP8D_COMP,
    mut width: ::core::ffi::c_int,
    mut prev_mb_rows: ::core::ffi::c_int,
) { unsafe {
    let pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut uv_width: ::core::ffi::c_int = 0;
    if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0 {
        vp8mt_de_alloc_temp_buffers(pbi, prev_mb_rows);
        if width & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            width += 16 as ::core::ffi::c_int - (width & 0xf as ::core::ffi::c_int);
        }
        if width < 640 as ::core::ffi::c_int {
            (*pbi).sync_range = 1 as ::core::ffi::c_int;
        } else if width <= 1280 as ::core::ffi::c_int {
            (*pbi).sync_range = 8 as ::core::ffi::c_int;
        } else if width <= 2560 as ::core::ffi::c_int {
            (*pbi).sync_range = 16 as ::core::ffi::c_int;
        } else {
            (*pbi).sync_range = 32 as ::core::ffi::c_int;
        }
        uv_width = width >> 1 as ::core::ffi::c_int;
        let mb_rows_usize = (*pc).mb_rows as usize;
        
        (*pbi).mt_current_mb_col = Box::into_raw(vec![vpx_atomic_int { value: 0 }; mb_rows_usize].into_boxed_slice()) as *mut vpx_atomic_int;
        if (*pbi).mt_current_mb_col.is_null() {
            vpx_internal_error(
                &raw mut (*pc).error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->mt_current_mb_col\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            vpx_atomic_init(
                (*pbi).mt_current_mb_col.add(i),
                0 as ::core::ffi::c_int,
            );
        }
        
        (*pbi).mt_yabove_row = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_yabove_row.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_yabove_row)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = (width + ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(16, size) {
                Some(b) => b.into_raw(),
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_yabove_row.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_yabove_row[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            memset(
                ptr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                size,
            );
        }
        
        (*pbi).mt_uabove_row = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_uabove_row.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_uabove_row)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(16, size) {
                Some(b) => b.into_raw(),
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_uabove_row.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_uabove_row[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            memset(
                ptr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                size,
            );
        }
        
        (*pbi).mt_vabove_row = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_vabove_row.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_vabove_row)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = (uv_width + 32 as ::core::ffi::c_int) as usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(16, size) {
                Some(b) => b.into_raw(),
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_vabove_row.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_vabove_row[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            memset(
                ptr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                size,
            );
        }
        
        (*pbi).mt_yleft_col = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_yleft_col.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_yleft_col)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = 16usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(32, size) {
                Some(b) => {
                    core::ptr::write_bytes(b.as_ptr(), 0, size);
                    b.into_raw()
                }
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_yleft_col.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_yleft_col[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        
        (*pbi).mt_uleft_col = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_uleft_col.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_uleft_col)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = 8usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(32, size) {
                Some(b) => {
                    core::ptr::write_bytes(b.as_ptr(), 0, size);
                    b.into_raw()
                }
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_uleft_col.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_uleft_col[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        
        (*pbi).mt_vleft_col = Box::into_raw(vec![core::ptr::null_mut::<core::ffi::c_uchar>(); mb_rows_usize].into_boxed_slice()) as *mut *mut ::core::ffi::c_uchar;
        if (*pbi).mt_vleft_col.is_null() {
            vpx_internal_error(
                &raw mut (*pbi).common.error,
                VPX_CODEC_MEM_ERROR,
                b"Failed to allocate (pbi->mt_vleft_col)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        for i in 0..mb_rows_usize {
            let size = 8usize;
            let ptr = match crate::vpx_mem::vpx_mem::AlignedBox::new(32, size) {
                Some(b) => {
                    core::ptr::write_bytes(b.as_ptr(), 0, size);
                    b.into_raw()
                }
                None => core::ptr::null_mut(),
            };
            *(*pbi).mt_vleft_col.add(i) = ptr;
            if ptr.is_null() {
                vpx_internal_error(
                    &raw mut (*pc).error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate pbi->mt_vleft_col[i]\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_decoder_remove_threads(mut pbi: *mut VP8D_COMP) { unsafe {
    if vpx_atomic_load_acquire(&raw mut (*pbi).b_multithreaded_rd) != 0 {
        let mut i: ::core::ffi::c_int = 0;
        vpx_atomic_store_release(&raw mut (*pbi).b_multithreaded_rd, 0 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < (*pbi).allocated_decoding_thread_count {
            crate::thread_shim::vp8_semaphore_signal(*(*pbi).h_event_start_decoding.offset(i as isize));
            crate::thread_shim::vp8_pthread_join(
                *(*pbi).h_decoding_thread.offset(i as isize) as pthread_t,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pbi).allocated_decoding_thread_count {
            crate::thread_shim::vp8_semaphore_destroy(
                mach_task_self_ as task_t,
                *(*pbi).h_event_start_decoding.offset(i as isize),
            );
            i += 1;
        }
        if (*pbi).allocated_decoding_thread_count != 0 {
            crate::thread_shim::vp8_semaphore_destroy(mach_task_self_ as task_t, (*pbi).h_event_end_decoding);
        }
        let count = (*pbi).decoding_thread_count as usize;
        if !(*pbi).h_decoding_thread.is_null() {
            let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).h_decoding_thread, count));
            (*pbi).h_decoding_thread = ::core::ptr::null_mut::<pthread_t>();
        }
        if !(*pbi).h_event_start_decoding.is_null() {
            let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).h_event_start_decoding, count));
            (*pbi).h_event_start_decoding = ::core::ptr::null_mut::<semaphore_t>();
        }
        if !(*pbi).mb_row_di.is_null() {
            let _ = crate::vpx_mem::vpx_mem::AlignedBox::from_raw((*pbi).mb_row_di as *mut u8);
            (*pbi).mb_row_di = ::core::ptr::null_mut::<MB_ROW_DEC>();
        }
        if !(*pbi).de_thread_data.is_null() {
            let _ = Box::from_raw(core::ptr::slice_from_raw_parts_mut((*pbi).de_thread_data, count));
            (*pbi).de_thread_data = ::core::ptr::null_mut::<DECODETHREAD_DATA>();
        }
        vp8mt_de_alloc_temp_buffers(pbi, (*pbi).common.mb_rows);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8mt_decode_mb_rows(
    mut pbi: *mut VP8D_COMP,
    mut xd: *mut MACROBLOCKD,
) -> ::core::ffi::c_int { unsafe {
    let mut pc: *mut VP8_COMMON = &raw mut (*pbi).common;
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut filter_level: ::core::ffi::c_int = (*pc).filter_level;
    let mut yv12_fb_new: *mut YV12_BUFFER_CONFIG =
        (*pbi).dec_fb_ref[INTRA_FRAME as ::core::ffi::c_int as usize];
    if filter_level != 0 {
        memset(
            (*(*pbi)
                .mt_yabove_row
                .offset(0 as ::core::ffi::c_int as isize))
            .offset(VP8BORDERINPIXELS as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            ((*yv12_fb_new).y_width + 5 as ::core::ffi::c_int) as size_t,
        );
        memset(
            (*(*pbi)
                .mt_uabove_row
                .offset(0 as ::core::ffi::c_int as isize))
            .offset((VP8BORDERINPIXELS >> 1 as ::core::ffi::c_int) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (((*yv12_fb_new).y_width >> 1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int)
                as size_t,
        );
        memset(
            (*(*pbi)
                .mt_vabove_row
                .offset(0 as ::core::ffi::c_int as isize))
            .offset((VP8BORDERINPIXELS >> 1 as ::core::ffi::c_int) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (((*yv12_fb_new).y_width >> 1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int)
                as size_t,
        );
        j = 1 as ::core::ffi::c_int;
        while j < (*pc).mb_rows {
            memset(
                (*(*pbi).mt_yabove_row.offset(j as isize))
                    .offset(VP8BORDERINPIXELS as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                1 as size_t,
            );
            memset(
                (*(*pbi).mt_uabove_row.offset(j as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as ::core::ffi::c_int) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                1 as size_t,
            );
            memset(
                (*(*pbi).mt_vabove_row.offset(j as isize))
                    .offset((VP8BORDERINPIXELS >> 1 as ::core::ffi::c_int) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                1 as size_t,
            );
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < (*pc).mb_rows {
            memset(
                *(*pbi).mt_yleft_col.offset(j as isize) as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                16 as size_t,
            );
            memset(
                *(*pbi).mt_uleft_col.offset(j as isize) as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                8 as size_t,
            );
            memset(
                *(*pbi).mt_vleft_col.offset(j as isize) as *mut ::core::ffi::c_void,
                129 as ::core::ffi::c_int as ::core::ffi::c_uchar as ::core::ffi::c_int,
                8 as size_t,
            );
            j += 1;
        }
        vp8_loop_filter_frame_init(pc as *mut VP8Common, &raw mut (*pbi).mb, filter_level);
    } else {
        vp8_setup_intra_recon_top_line(yv12_fb_new);
    }
    setup_decoding_thread_data(
        pbi,
        xd,
        (*pbi).mb_row_di,
        (*pbi).decoding_thread_count as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < (*pbi).decoding_thread_count {
        crate::thread_shim::vp8_semaphore_signal(*(*pbi).h_event_start_decoding.offset(i as isize));
        i = i.wrapping_add(1);
    }
    if setjmp(&raw mut (*xd).error_info.jmp as *mut ::core::ffi::c_int) != 0 {
        (*xd).error_info.setjmp = 0 as ::core::ffi::c_int;
        (*xd).corrupted = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_uint;
        while i < (*pbi).decoding_thread_count {
            crate::thread_shim::vp8_semaphore_wait((*pbi).h_event_end_decoding);
            i = i.wrapping_add(1);
        }
        return -(1 as ::core::ffi::c_int);
    }
    (*xd).error_info.setjmp = 1 as ::core::ffi::c_int;
    mt_decode_mb_rows(pbi, xd, 0 as ::core::ffi::c_int);
    (*xd).error_info.setjmp = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_uint;
    while i
        < (*pbi)
            .decoding_thread_count
            .wrapping_add(1 as ::core::ffi::c_uint)
    {
        crate::thread_shim::vp8_semaphore_wait((*pbi).h_event_end_decoding);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
