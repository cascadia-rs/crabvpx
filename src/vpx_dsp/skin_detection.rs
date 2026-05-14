pub const MODEL_MODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut skin_mean: [[::core::ffi::c_int; 2]; 5] = [
    [7463 as ::core::ffi::c_int, 9614 as ::core::ffi::c_int],
    [6400 as ::core::ffi::c_int, 10240 as ::core::ffi::c_int],
    [7040 as ::core::ffi::c_int, 10240 as ::core::ffi::c_int],
    [8320 as ::core::ffi::c_int, 9280 as ::core::ffi::c_int],
    [6800 as ::core::ffi::c_int, 9614 as ::core::ffi::c_int],
];
static mut skin_inv_cov: [::core::ffi::c_int; 4] = [
    4107 as ::core::ffi::c_int,
    1663 as ::core::ffi::c_int,
    1663 as ::core::ffi::c_int,
    2157 as ::core::ffi::c_int,
];
static mut skin_threshold: [::core::ffi::c_int; 6] = [
    1570636 as ::core::ffi::c_int,
    1400000 as ::core::ffi::c_int,
    800000 as ::core::ffi::c_int,
    800000 as ::core::ffi::c_int,
    800000 as ::core::ffi::c_int,
    800000 as ::core::ffi::c_int,
];
static mut y_low: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
static mut y_high: ::core::ffi::c_int = 220 as ::core::ffi::c_int;
unsafe extern "C" fn vpx_evaluate_skin_color_difference(
    cb: ::core::ffi::c_int,
    cr: ::core::ffi::c_int,
    idx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let cb_q6: ::core::ffi::c_int = cb << 6 as ::core::ffi::c_int;
    let cr_q6: ::core::ffi::c_int = cr << 6 as ::core::ffi::c_int;
    let cb_diff_q12: ::core::ffi::c_int = (cb_q6
        - skin_mean[idx as usize][0 as ::core::ffi::c_int as usize])
        * (cb_q6 - skin_mean[idx as usize][0 as ::core::ffi::c_int as usize]);
    let cbcr_diff_q12: ::core::ffi::c_int = (cb_q6
        - skin_mean[idx as usize][0 as ::core::ffi::c_int as usize])
        * (cr_q6 - skin_mean[idx as usize][1 as ::core::ffi::c_int as usize]);
    let cr_diff_q12: ::core::ffi::c_int = (cr_q6
        - skin_mean[idx as usize][1 as ::core::ffi::c_int as usize])
        * (cr_q6 - skin_mean[idx as usize][1 as ::core::ffi::c_int as usize]);
    let cb_diff_q2: ::core::ffi::c_int = cb_diff_q12
        + ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
        >> 10 as ::core::ffi::c_int;
    let cbcr_diff_q2: ::core::ffi::c_int = cbcr_diff_q12
        + ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
        >> 10 as ::core::ffi::c_int;
    let cr_diff_q2: ::core::ffi::c_int = cr_diff_q12
        + ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
        >> 10 as ::core::ffi::c_int;
    let skin_diff: ::core::ffi::c_int = skin_inv_cov[0 as ::core::ffi::c_int as usize] * cb_diff_q2
        + skin_inv_cov[1 as ::core::ffi::c_int as usize] * cbcr_diff_q2
        + skin_inv_cov[2 as ::core::ffi::c_int as usize] * cbcr_diff_q2
        + skin_inv_cov[3 as ::core::ffi::c_int as usize] * cr_diff_q2;
    return skin_diff;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_skin_pixel(
    y: ::core::ffi::c_int,
    cb: ::core::ffi::c_int,
    cr: ::core::ffi::c_int,
    mut motion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if y < y_low || y > y_high {
        return 0 as ::core::ffi::c_int;
    } else if MODEL_MODE == 0 as ::core::ffi::c_int {
        return (vpx_evaluate_skin_color_difference(cb, cr, 0 as ::core::ffi::c_int)
            < skin_threshold[0 as ::core::ffi::c_int as usize])
            as ::core::ffi::c_int;
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if cb == 128 as ::core::ffi::c_int && cr == 128 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if cb > 150 as ::core::ffi::c_int && cr < 110 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        while i < 5 as ::core::ffi::c_int {
            let mut skin_color_diff: ::core::ffi::c_int =
                vpx_evaluate_skin_color_difference(cb, cr, i);
            if skin_color_diff < skin_threshold[(i + 1 as ::core::ffi::c_int) as usize] {
                if y < 60 as ::core::ffi::c_int
                    && skin_color_diff
                        > 3 as ::core::ffi::c_int
                            * (skin_threshold[(i + 1 as ::core::ffi::c_int) as usize]
                                >> 2 as ::core::ffi::c_int)
                {
                    return 0 as ::core::ffi::c_int;
                } else if motion == 0 as ::core::ffi::c_int
                    && skin_color_diff
                        > skin_threshold[(i + 1 as ::core::ffi::c_int) as usize]
                            >> 1 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                } else {
                    return 1 as ::core::ffi::c_int;
                }
            }
            if skin_color_diff
                > skin_threshold[(i + 1 as ::core::ffi::c_int) as usize] << 3 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    };
}
