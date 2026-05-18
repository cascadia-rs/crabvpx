pub type vp8_prob = u8;
pub type vp8_tree_index = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: i32,
    pub Len: i32,
}
pub type vp8_token = vp8_token_struct;
pub type uint64_t = u64;
pub const vp8_prob_half: vp8_prob = 128 as vp8_prob;
unsafe fn tree2tok(
    p: *mut vp8_token_struct,
    mut t: *const vp8_tree_index,
    mut i: i32,
    mut v: i32,
    mut L: i32,
) {
    unsafe {
        v += v;
        L += 1;
        loop {
            let fresh0 = i;
            i += 1;
            let j: vp8_tree_index = *t.offset(fresh0 as isize);
            if j as i32 <= 0 as i32 {
                (*p.offset(-(j as i32) as isize)).value = v;
                (*p.offset(-(j as i32) as isize)).Len = L;
            } else {
                tree2tok(p, t, j as i32, v, L);
            }
            v += 1;
            if !(v & 1 as i32 != 0) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_tokens_from_tree(mut p: *mut vp8_token_struct, mut t: *const vp8_tree_index) {
    unsafe {
        tree2tok(p, t, 0 as i32, 0 as i32, 0 as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_tokens_from_tree_offset(
    mut p: *mut vp8_token_struct,
    mut t: *const vp8_tree_index,
    mut offset: i32,
) {
    unsafe {
        tree2tok(
            p.offset(-(offset as isize)),
            t,
            0 as i32,
            0 as i32,
            0 as i32,
        );
    }
}
unsafe fn branch_counts(
    mut n: i32,
    mut tok: *const vp8_token,
    mut tree: *const vp8_tree_index,
    mut branch_ct: *mut [u32; 2],
    mut num_events: *const u32,
) {
    unsafe {
        let tree_len: i32 = n - 1 as i32;
        let mut t: i32 = 0 as i32;
        loop {
            let fresh1 = &mut (*branch_ct.offset(t as isize))[1 as usize];
            *fresh1 = 0 as u32;
            (*branch_ct.offset(t as isize))[0 as usize] = *fresh1;
            t += 1;
            if !(t < tree_len) {
                break;
            }
        }
        t = 0 as i32;
        loop {
            let mut L: i32 = (*tok.offset(t as isize)).Len;
            let enc: i32 = (*tok.offset(t as isize)).value;
            let ct: u32 = *num_events.offset(t as isize);
            let mut i: vp8_tree_index = 0 as vp8_tree_index;
            loop {
                L -= 1;
                let b: i32 = enc >> L & 1 as i32;
                let j: i32 = i as i32 >> 1 as i32;
                let fresh2 = &mut (*branch_ct.offset(j as isize))[b as usize];
                *fresh2 = (*fresh2).wrapping_add(ct);
                i = *tree.offset((i as i32 + b) as isize);
                if !(i as i32 > 0 as i32) {
                    break;
                }
            }
            t += 1;
            if !(t < n) {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_tree_probs_from_distribution(
    mut n: i32,
    mut tok: *const vp8_token,
    mut tree: *const vp8_tree_index,
    mut probs: *mut vp8_prob,
    mut branch_ct: *mut [u32; 2],
    mut num_events: *const u32,
    mut Pfactor: u32,
    mut Round: i32,
) {
    unsafe {
        let tree_len: i32 = n - 1 as i32;
        let mut t: i32 = 0 as i32;
        branch_counts(n, tok, tree, branch_ct, num_events);
        loop {
            let c: *const u32 = &raw mut *branch_ct.offset(t as isize) as *mut u32;
            let tot: u32 = (*c.offset(0 as isize)).wrapping_add(*c.offset(1 as isize));
            if tot != 0 {
                let p: u32 = ((*c.offset(0 as isize) as uint64_t)
                    .wrapping_mul(Pfactor as uint64_t)
                    .wrapping_add(
                        (if Round != 0 {
                            tot >> 1 as i32
                        } else {
                            0 as u32
                        }) as uint64_t,
                    ) as u32)
                    .wrapping_div(tot);
                *probs.offset(t as isize) = (if p < 256 as u32 {
                    if p != 0 { p } else { 1 as u32 }
                } else {
                    255 as u32
                }) as vp8_prob;
            } else {
                *probs.offset(t as isize) = vp8_prob_half;
            }
            t += 1;
            if !(t < tree_len) {
                break;
            }
        }
    }
}
