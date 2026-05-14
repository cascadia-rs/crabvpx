pub type vp8_prob = ::core::ffi::c_uchar;
pub type vp8_tree_index = ::core::ffi::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: ::core::ffi::c_int,
    pub Len: ::core::ffi::c_int,
}
pub type vp8_token = vp8_token_struct;
pub type uint64_t = u64;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
unsafe extern "C" fn tree2tok(
    p: *mut vp8_token_struct,
    mut t: *const vp8_tree_index,
    mut i: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
    mut L: ::core::ffi::c_int,
) {
    v += v;
    L += 1;
    loop {
        let fresh0 = i;
        i = i + 1;
        let j: vp8_tree_index = *t.offset(fresh0 as isize);
        if j as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            (*p.offset(-(j as ::core::ffi::c_int) as isize)).value = v;
            (*p.offset(-(j as ::core::ffi::c_int) as isize)).Len = L;
        } else {
            tree2tok(p, t, j as ::core::ffi::c_int, v, L);
        }
        v += 1;
        if !(v & 1 as ::core::ffi::c_int != 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_tokens_from_tree(
    mut p: *mut vp8_token_struct,
    mut t: *const vp8_tree_index,
) {
    tree2tok(
        p,
        t,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn vp8_tokens_from_tree_offset(
    mut p: *mut vp8_token_struct,
    mut t: *const vp8_tree_index,
    mut offset: ::core::ffi::c_int,
) {
    tree2tok(
        p.offset(-(offset as isize)),
        t,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn branch_counts(
    mut n: ::core::ffi::c_int,
    mut tok: *const vp8_token,
    mut tree: *const vp8_tree_index,
    mut branch_ct: *mut [::core::ffi::c_uint; 2],
    mut num_events: *const ::core::ffi::c_uint,
) {
    let tree_len: ::core::ffi::c_int = n - 1 as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let ref mut fresh1 = (*branch_ct.offset(t as isize))[1 as ::core::ffi::c_int as usize];
        *fresh1 = 0 as ::core::ffi::c_uint;
        (*branch_ct.offset(t as isize))[0 as ::core::ffi::c_int as usize] = *fresh1;
        t += 1;
        if !(t < tree_len) {
            break;
        }
    }
    t = 0 as ::core::ffi::c_int;
    loop {
        let mut L: ::core::ffi::c_int = (*tok.offset(t as isize)).Len;
        let enc: ::core::ffi::c_int = (*tok.offset(t as isize)).value;
        let ct: ::core::ffi::c_uint = *num_events.offset(t as isize);
        let mut i: vp8_tree_index = 0 as vp8_tree_index;
        loop {
            L -= 1;
            let b: ::core::ffi::c_int = enc >> L & 1 as ::core::ffi::c_int;
            let j: ::core::ffi::c_int = i as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
            let ref mut fresh2 = (*branch_ct.offset(j as isize))[b as usize];
            *fresh2 = (*fresh2).wrapping_add(ct);
            i = *tree.offset((i as ::core::ffi::c_int + b) as isize);
            if !(i as ::core::ffi::c_int > 0 as ::core::ffi::c_int) {
                break;
            }
        }
        t += 1;
        if !(t < n) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn vp8_tree_probs_from_distribution(
    mut n: ::core::ffi::c_int,
    mut tok: *const vp8_token,
    mut tree: *const vp8_tree_index,
    mut probs: *mut vp8_prob,
    mut branch_ct: *mut [::core::ffi::c_uint; 2],
    mut num_events: *const ::core::ffi::c_uint,
    mut Pfactor: ::core::ffi::c_uint,
    mut Round: ::core::ffi::c_int,
) {
    let tree_len: ::core::ffi::c_int = n - 1 as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    branch_counts(n, tok, tree, branch_ct, num_events);
    loop {
        let c: *const ::core::ffi::c_uint =
            &raw mut *branch_ct.offset(t as isize) as *mut ::core::ffi::c_uint;
        let tot: ::core::ffi::c_uint = (*c.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(*c.offset(1 as ::core::ffi::c_int as isize));
        if tot != 0 {
            let p: ::core::ffi::c_uint = ((*c.offset(0 as ::core::ffi::c_int as isize) as uint64_t)
                .wrapping_mul(Pfactor as uint64_t)
                .wrapping_add(
                    (if Round != 0 {
                        tot >> 1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_uint
                    }) as uint64_t,
                ) as ::core::ffi::c_uint)
                .wrapping_div(tot);
            *probs.offset(t as isize) = (if p < 256 as ::core::ffi::c_uint {
                if p != 0 {
                    p
                } else {
                    1 as ::core::ffi::c_uint
                }
            } else {
                255 as ::core::ffi::c_uint
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
