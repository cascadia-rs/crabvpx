pub type Vp8Prob = u8;
pub type Vp8TreeIndex = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp8TokenStruct {
    pub value: i32,
    pub len: i32,
}
pub type Vp8Token = Vp8TokenStruct;
pub const vp8_prob_half: Vp8Prob = 128 as Vp8Prob;
unsafe fn tree2tok(
    p: *mut Vp8TokenStruct,
    mut t: *const Vp8TreeIndex,
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
            let j: Vp8TreeIndex = *t.offset(fresh0 as isize);
            if j as i32 <= 0 as i32 {
                (*p.offset(-(j as i32) as isize)).value = v;
                (*p.offset(-(j as i32) as isize)).len = L;
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
pub unsafe fn vp8_tokens_from_tree(mut p: *mut Vp8TokenStruct, mut t: *const Vp8TreeIndex) {
    unsafe {
        tree2tok(p, t, 0 as i32, 0 as i32, 0 as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_tokens_from_tree_offset(
    mut p: *mut Vp8TokenStruct,
    mut t: *const Vp8TreeIndex,
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
    mut tok: *const Vp8Token,
    mut tree: *const Vp8TreeIndex,
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
            let mut L: i32 = (*tok.offset(t as isize)).len;
            let enc: i32 = (*tok.offset(t as isize)).value;
            let ct: u32 = *num_events.offset(t as isize);
            let mut i: Vp8TreeIndex = 0 as Vp8TreeIndex;
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
    mut tok: *const Vp8Token,
    mut tree: *const Vp8TreeIndex,
    mut probs: *mut Vp8Prob,
    mut branch_ct: *mut [u32; 2],
    mut num_events: *const u32,
    mut pfactor: u32,
    mut round: i32,
) {
    unsafe {
        let tree_len: i32 = n - 1 as i32;
        let mut t: i32 = 0 as i32;
        branch_counts(n, tok, tree, branch_ct, num_events);
        loop {
            let c: *const u32 = &raw mut *branch_ct.offset(t as isize) as *mut u32;
            let tot: u32 = (*c.offset(0 as isize)).wrapping_add(*c.offset(1 as isize));
            if tot != 0 {
                let p: u32 = ((*c.offset(0 as isize) as u64)
                    .wrapping_mul(pfactor as u64)
                    .wrapping_add(
                        (if round != 0 {
                            tot >> 1 as i32
                        } else {
                            0 as u32
                        }) as u64,
                    ) as u32)
                    .wrapping_div(tot);
                *probs.offset(t as isize) = (if p < 256 as u32 {
                    if p != 0 { p } else { 1 as u32 }
                } else {
                    255 as u32
                }) as Vp8Prob;
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
