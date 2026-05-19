#[inline]
fn get_prob(mut num: u32, mut den: u32) -> u8 {
    let p: i32 = (num as u64)
        .wrapping_mul(256 as u64)
        .wrapping_add((den >> 1 as i32) as u64)
        .wrapping_div(den as u64) as i32;
    let clipped_prob: i32 = p | (255 as i32 - p) >> 23 as i32 | (p == 0 as i32) as i32;
    clipped_prob as u8
}
#[inline]
fn weighted_prob(mut prob1: i32, mut prob2: i32, mut factor: i32) -> u8 {
    ((prob1 * (256 as i32 - factor) + prob2 * factor + ((1 as i32) << (8 as i32 - 1 as i32)))
        >> 8 as i32) as u8
}
static mut count_to_update_factor: [i32; 21] = [
    0 as i32, 6 as i32, 12 as i32, 19 as i32, 25 as i32, 32 as i32, 38 as i32, 44 as i32,
    51 as i32, 57 as i32, 64 as i32, 70 as i32, 76 as i32, 83 as i32, 89 as i32, 96 as i32,
    102 as i32, 108 as i32, 115 as i32, 121 as i32, 128 as i32,
];
#[inline]
unsafe fn mode_mv_merge_probs(mut pre_prob: u8, mut ct: *const u32) -> u8 {
    unsafe {
        let den: u32 = (*ct.offset(0 as isize)).wrapping_add(*ct.offset(1 as isize));
        if den == 0 as u32 {
            pre_prob
        } else {
            let count: u32 = if den < 20 as u32 { den } else { 20 as u32 };
            let factor: u32 = count_to_update_factor[count as usize] as u32;
            let prob: u8 = get_prob(*ct.offset(0 as isize), den) as u8;
            weighted_prob(pre_prob as i32, prob as i32, factor as i32)
        }
    }
}
#[unsafe(no_mangle)]
pub static mut vpx_norm: [u8; 256] = [
    0 as u8, 7 as u8, 6 as u8, 6 as u8, 5 as u8, 5 as u8, 5 as u8, 5 as u8, 4 as u8, 4 as u8,
    4 as u8, 4 as u8, 4 as u8, 4 as u8, 4 as u8, 4 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8,
    3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8,
    3 as u8, 3 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
    2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
    2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
    2 as u8, 2 as u8, 2 as u8, 2 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
    1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
];
unsafe fn tree_merge_probs_impl(
    mut i: u32,
    mut tree: *const i8,
    mut pre_probs: *const u8,
    mut counts: *const u32,
    mut probs: *mut u8,
) -> u32 {
    unsafe {
        let l: i32 = *tree.offset(i as isize) as i32;
        let left_count: u32 = if l <= 0 as i32 {
            *counts.offset(-l as isize)
        } else {
            tree_merge_probs_impl(l as u32, tree, pre_probs, counts, probs) as u32
        };
        let r: i32 = *tree.offset(i.wrapping_add(1 as u32) as isize) as i32;
        let right_count: u32 = if r <= 0 as i32 {
            *counts.offset(-r as isize)
        } else {
            tree_merge_probs_impl(r as u32, tree, pre_probs, counts, probs) as u32
        };
        let ct: [u32; 2] = [left_count, right_count];
        *probs.offset((i >> 1 as i32) as isize) = mode_mv_merge_probs(
            *pre_probs.offset((i >> 1 as i32) as isize),
            &raw const ct as *const u32,
        );
        left_count.wrapping_add(right_count)
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tree_merge_probs(
    mut tree: *const i8,
    mut pre_probs: *const u8,
    mut counts: *const u32,
    mut probs: *mut u8,
) {
    unsafe {
        tree_merge_probs_impl(0 as u32, tree, pre_probs, counts, probs);
    }
}
