pub const MODEL_MODE: i32 = 1 as i32;
static mut skin_mean: [[i32; 2]; 5] = [
    [7463 as i32, 9614 as i32],
    [6400 as i32, 10240 as i32],
    [7040 as i32, 10240 as i32],
    [8320 as i32, 9280 as i32],
    [6800 as i32, 9614 as i32],
];
static mut skin_inv_cov: [i32; 4] = [4107 as i32, 1663 as i32, 1663 as i32, 2157 as i32];
static mut skin_threshold: [i32; 6] = [
    1570636 as i32,
    1400000 as i32,
    800000 as i32,
    800000 as i32,
    800000 as i32,
    800000 as i32,
];
static mut y_low: i32 = 40 as i32;
static mut y_high: i32 = 220 as i32;
unsafe fn vpx_evaluate_skin_color_difference(cb: i32, cr: i32, idx: i32) -> i32 {
    unsafe {
        let cb_q6: i32 = cb << 6 as i32;
        let cr_q6: i32 = cr << 6 as i32;
        let cb_diff_q12: i32 = (cb_q6 - skin_mean[idx as usize][0 as usize])
            * (cb_q6 - skin_mean[idx as usize][0 as usize]);
        let cbcr_diff_q12: i32 = (cb_q6 - skin_mean[idx as usize][0 as usize])
            * (cr_q6 - skin_mean[idx as usize][1 as usize]);
        let cr_diff_q12: i32 = (cr_q6 - skin_mean[idx as usize][1 as usize])
            * (cr_q6 - skin_mean[idx as usize][1 as usize]);
        let cb_diff_q2: i32 = (cb_diff_q12 + ((1 as i32) << 9 as i32)) >> 10 as i32;
        let cbcr_diff_q2: i32 = (cbcr_diff_q12 + ((1 as i32) << 9 as i32)) >> 10 as i32;
        let cr_diff_q2: i32 = (cr_diff_q12 + ((1 as i32) << 9 as i32)) >> 10 as i32;
        let skin_diff: i32 = skin_inv_cov[0 as usize] * cb_diff_q2
            + skin_inv_cov[1 as usize] * cbcr_diff_q2
            + skin_inv_cov[2 as usize] * cbcr_diff_q2
            + skin_inv_cov[3 as usize] * cr_diff_q2;
        skin_diff
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_skin_pixel(y: i32, cb: i32, cr: i32, mut motion: i32) -> i32 {
    unsafe {
        if y < y_low || y > y_high {
            0 as i32
        } else if MODEL_MODE == 0 as i32 {
            (vpx_evaluate_skin_color_difference(cb, cr, 0 as i32) < skin_threshold[0 as usize])
                as i32
        } else {
            let mut i: i32 = 0 as i32;
            if cb == 128 as i32 && cr == 128 as i32 {
                return 0 as i32;
            }
            if cb > 150 as i32 && cr < 110 as i32 {
                return 0 as i32;
            }
            while i < 5 as i32 {
                let mut skin_color_diff: i32 = vpx_evaluate_skin_color_difference(cb, cr, i);
                if skin_color_diff < skin_threshold[(i + 1 as i32) as usize] {
                    if y < 60 as i32
                        && skin_color_diff
                            > 3 as i32 * (skin_threshold[(i + 1 as i32) as usize] >> 2 as i32)
                    {
                        return 0 as i32;
                    } else if motion == 0 as i32
                        && skin_color_diff > skin_threshold[(i + 1 as i32) as usize] >> 1 as i32
                    {
                        return 0 as i32;
                    } else {
                        return 1 as i32;
                    }
                }
                if skin_color_diff > skin_threshold[(i + 1 as i32) as usize] << 3 as i32 {
                    return 0 as i32;
                }
                i += 1;
            }
            0 as i32
        }
    }
}
