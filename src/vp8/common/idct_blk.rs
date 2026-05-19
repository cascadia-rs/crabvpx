use std::ffi::c_void;
unsafe extern "Rust" {
    fn vp8_dc_only_idct_add_c(
        input_dc: i16,
        pred_ptr: *mut u8,
        pred_stride: i32,
        dst_ptr: *mut u8,
        dst_stride: i32,
    );
    fn vp8_dequant_idct_add_c(input: *mut i16, dq: *mut i16, dest: *mut u8, stride: i32);
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequant_idct_add_y_block_c(
    mut q: *mut i16,
    mut dq: *mut i16,
    mut dst: *mut u8,
    mut stride: i32,
    mut eobs: *mut i8,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0 as i32;
        while i < 4 as i32 {
            j = 0 as i32;
            while j < 4 as i32 {
                let fresh2 = eobs;
                eobs = eobs.offset(1);
                if *fresh2 as i32 > 1 as i32 {
                    vp8_dequant_idct_add_c(q, dq, dst, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as isize) as i32 * *dq.offset(0 as isize) as i32) as i16,
                        dst,
                        stride,
                        dst,
                        stride,
                    );
                    core::ptr::write_bytes(
                        q as *mut c_void as *mut u8,
                        0 as u8,
                        (2 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
                    );
                }
                q = q.offset(16 as isize);
                dst = dst.offset(4 as isize);
                j += 1;
            }
            dst = dst.offset((4 as i32 * stride - 16 as i32) as isize);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dequant_idct_add_uv_block_c(
    mut q: *mut i16,
    mut dq: *mut i16,
    mut dst_u: *mut u8,
    mut dst_v: *mut u8,
    mut stride: i32,
    mut eobs: *mut i8,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0 as i32;
        while i < 2 as i32 {
            j = 0 as i32;
            while j < 2 as i32 {
                let fresh0 = eobs;
                eobs = eobs.offset(1);
                if *fresh0 as i32 > 1 as i32 {
                    vp8_dequant_idct_add_c(q, dq, dst_u, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as isize) as i32 * *dq.offset(0 as isize) as i32) as i16,
                        dst_u,
                        stride,
                        dst_u,
                        stride,
                    );
                    core::ptr::write_bytes(
                        q as *mut c_void as *mut u8,
                        0 as u8,
                        (2 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
                    );
                }
                q = q.offset(16 as isize);
                dst_u = dst_u.offset(4 as isize);
                j += 1;
            }
            dst_u = dst_u.offset((4 as i32 * stride - 8 as i32) as isize);
            i += 1;
        }
        i = 0 as i32;
        while i < 2 as i32 {
            j = 0 as i32;
            while j < 2 as i32 {
                let fresh1 = eobs;
                eobs = eobs.offset(1);
                if *fresh1 as i32 > 1 as i32 {
                    vp8_dequant_idct_add_c(q, dq, dst_v, stride);
                } else {
                    vp8_dc_only_idct_add_c(
                        (*q.offset(0 as isize) as i32 * *dq.offset(0 as isize) as i32) as i16,
                        dst_v,
                        stride,
                        dst_v,
                        stride,
                    );
                    core::ptr::write_bytes(
                        q as *mut c_void as *mut u8,
                        0 as u8,
                        (2 as usize).wrapping_mul(::core::mem::size_of::<i16>() as usize),
                    );
                }
                q = q.offset(16 as isize);
                dst_v = dst_v.offset(4 as isize);
                j += 1;
            }
            dst_v = dst_v.offset((4 as i32 * stride - 8 as i32) as isize);
            i += 1;
        }
    }
}
