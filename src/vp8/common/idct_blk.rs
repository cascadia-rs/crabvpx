unsafe extern "Rust" {
    fn vp8_dc_only_idct_add_c(
        input_dc: ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn vp8_dequant_idct_add_c(
        input: *mut ::core::ffi::c_short,
        dq: *mut ::core::ffi::c_short,
        dest: *mut ::core::ffi::c_uchar,
        stride: ::core::ffi::c_int,
    );
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequant_idct_add_y_block_c(
    mut q: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dst: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
    mut eobs: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < 4 as ::core::ffi::c_int {
                let fresh2 = eobs;
                eobs = eobs.offset(1);
                if *fresh2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    vp8_dequant_idct_add_c(q, dq, dst, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            * *dq.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_short,
                        dst,
                        stride,
                        dst,
                        stride,
                    );
                    memset(
                        q as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                q = q.offset(16 as ::core::ffi::c_int as isize);
                dst = dst.offset(4 as ::core::ffi::c_int as isize);
                j += 1;
            }
            dst =
                dst.offset((4 as ::core::ffi::c_int * stride - 16 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequant_idct_add_uv_block_c(
    mut q: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dst_u: *mut ::core::ffi::c_uchar,
    mut dst_v: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
    mut eobs: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < 2 as ::core::ffi::c_int {
                let fresh0 = eobs;
                eobs = eobs.offset(1);
                if *fresh0 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    vp8_dequant_idct_add_c(q, dq, dst_u, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            * *dq.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_short,
                        dst_u,
                        stride,
                        dst_u,
                        stride,
                    );
                    memset(
                        q as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                q = q.offset(16 as ::core::ffi::c_int as isize);
                dst_u = dst_u.offset(4 as ::core::ffi::c_int as isize);
                j += 1;
            }
            dst_u =
                dst_u.offset((4 as ::core::ffi::c_int * stride - 8 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < 2 as ::core::ffi::c_int {
                let fresh1 = eobs;
                eobs = eobs.offset(1);
                if *fresh1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    vp8_dequant_idct_add_c(q, dq, dst_v, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            * *dq.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_short,
                        dst_v,
                        stride,
                        dst_v,
                        stride,
                    );
                    memset(
                        q as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_short>() as size_t),
                    );
                }
                q = q.offset(16 as ::core::ffi::c_int as isize);
                dst_v = dst_v.offset(4 as ::core::ffi::c_int as isize);
                j += 1;
            }
            dst_v =
                dst_v.offset((4 as ::core::ffi::c_int * stride - 8 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    }
}
