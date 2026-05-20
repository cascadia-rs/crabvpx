pub use crate::vp8::common::types::vp8_prob;
pub type vp8_tree_index = ::core::ffi::c_schar;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: ::core::ffi::c_int,
    pub Len: ::core::ffi::c_int,
}
pub type vp8_token = vp8_token_struct;
pub type uint64_t = u64;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;

// Safe recursive helper to traverse the tree and populate the token table.
fn tree2tok_safe(
    p: &mut [vp8_token],
    t: &[vp8_tree_index],
    mut i: i32,
    mut v: i32,
    mut L: i32,
    offset: i32,
) {
    v += v;
    L += 1;
    loop {
        let fresh0 = i;
        i = i + 1;
        let j = t[fresh0 as usize] as i32;
        if j <= 0 {
            let idx = -j - offset;
            p[idx as usize].value = v;
            p[idx as usize].Len = L;
        } else {
            tree2tok_safe(p, t, j, v, L, offset);
        }
        v += 1;
        if (v & 1) == 0 {
            break;
        }
    }
}

fn detect_tree_slice(t: *const vp8_tree_index) -> Option<&'static [vp8_tree_index]> {
    if t.is_null() {
        return None;
    }
    
    use crate::vp8::common::entropy::{
        vp8_coef_tree, cat1, cat2, cat3, cat4, cat5, cat6,
    };
    use crate::vp8::common::entropymode::{
        vp8_bmode_tree, vp8_ymode_tree, vp8_kf_ymode_tree,
        vp8_uv_mode_tree, vp8_mbsplit_tree, vp8_mv_ref_tree,
        vp8_sub_mv_ref_tree,
    };

    if t == vp8_coef_tree.as_ptr() {
        return Some(&vp8_coef_tree);
    }
    if t == vp8_bmode_tree.as_ptr() {
        return Some(&vp8_bmode_tree);
    }
    if t == vp8_ymode_tree.as_ptr() {
        return Some(&vp8_ymode_tree);
    }
    if t == vp8_kf_ymode_tree.as_ptr() {
        return Some(&vp8_kf_ymode_tree);
    }
    if t == vp8_uv_mode_tree.as_ptr() {
        return Some(&vp8_uv_mode_tree);
    }
    if t == vp8_mbsplit_tree.as_ptr() {
        return Some(&vp8_mbsplit_tree);
    }
    if t == vp8_mv_ref_tree.as_ptr() {
        return Some(&vp8_mv_ref_tree);
    }
    if t == vp8_sub_mv_ref_tree.as_ptr() {
        return Some(&vp8_sub_mv_ref_tree);
    }
    if t == cat1.as_ptr() {
        return Some(&cat1);
    }
    if t == cat2.as_ptr() {
        return Some(&cat2);
    }
    if t == cat3.as_ptr() {
        return Some(&cat3);
    }
    if t == cat4.as_ptr() {
        return Some(&cat4);
    }
    if t == cat5.as_ptr() {
        return Some(&cat5);
    }
    if t == cat6.as_ptr() {
        return Some(&cat6);
    }

    None
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tokens_from_tree(
    p: *mut vp8_token_struct,
    t: *const vp8_tree_index,
) {
    if p.is_null() || t.is_null() {
        return;
    }
    let t_slice = detect_tree_slice(t).expect("Unexpected VP8 tree pointer");
    let mut max_token = 0;
    for &val in t_slice {
        let j = val as i32;
        if j <= 0 {
            max_token = std::cmp::max(max_token, -j);
        }
    }
    let p_slice = unsafe { core::slice::from_raw_parts_mut(p, (max_token + 1) as usize) };
    tree2tok_safe(p_slice, t_slice, 0, 0, 0, 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tokens_from_tree_offset(
    p: *mut vp8_token_struct,
    t: *const vp8_tree_index,
    offset: ::core::ffi::c_int,
) {
    if p.is_null() || t.is_null() {
        return;
    }
    let t_slice = detect_tree_slice(t).expect("Unexpected VP8 tree pointer");
    let mut max_token = 0;
    for &val in t_slice {
        let j = val as i32;
        if j <= 0 {
            max_token = std::cmp::max(max_token, -j);
        }
    }
    let limit = (max_token - offset + 1) as usize;
    let p_slice = unsafe { core::slice::from_raw_parts_mut(p, limit) };
    tree2tok_safe(p_slice, t_slice, 0, 0, 0, offset);
}

fn branch_counts_safe(
    n: i32,
    tok: &[vp8_token],
    tree: &[vp8_tree_index],
    branch_ct: &mut [[u32; 2]],
    num_events: &[u32],
) {
    let tree_len = n - 1;
    for t in 0..tree_len as usize {
        branch_ct[t][0] = 0;
        branch_ct[t][1] = 0;
    }
    for t in 0..n as usize {
        let mut L = tok[t].Len;
        let enc = tok[t].value;
        let ct = num_events[t];
        let mut i = 0 as vp8_tree_index;
        loop {
            L -= 1;
            let b = (enc >> L) & 1;
            let j = (i as i32) >> 1;
            branch_ct[j as usize][b as usize] = branch_ct[j as usize][b as usize].wrapping_add(ct);
            i = tree[(i as i32 + b) as usize];
            if i <= 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tree_probs_from_distribution(
    n: ::core::ffi::c_int,
    tok: *const vp8_token,
    tree: *const vp8_tree_index,
    probs: *mut vp8_prob,
    branch_ct: *mut [::core::ffi::c_uint; 2],
    num_events: *const ::core::ffi::c_uint,
    Pfactor: ::core::ffi::c_uint,
    Round: ::core::ffi::c_int,
) {
    if tok.is_null() || tree.is_null() || probs.is_null() || branch_ct.is_null() || num_events.is_null() {
        return;
    }
    let tree_slice = detect_tree_slice(tree).expect("Unexpected VP8 tree pointer");
    unsafe {
        let tok_slice = core::slice::from_raw_parts(tok, n as usize);
        let probs_slice = core::slice::from_raw_parts_mut(probs, (n - 1) as usize);
        let branch_ct_slice = core::slice::from_raw_parts_mut(branch_ct as *mut [u32; 2], (n - 1) as usize);
        let num_events_slice = core::slice::from_raw_parts(num_events, n as usize);
        
        branch_counts_safe(n, tok_slice, tree_slice, branch_ct_slice, num_events_slice);
        
        let tree_len = n - 1;
        for t in 0..tree_len as usize {
            let c = &branch_ct_slice[t];
            let tot = c[0].wrapping_add(c[1]);
            if tot != 0 {
                let p = ((c[0] as u64)
                    .wrapping_mul(Pfactor as u64)
                    .wrapping_add(
                        (if Round != 0 {
                            tot >> 1
                        } else {
                            0
                        }) as u64,
                    ) as u32)
                    .wrapping_div(tot);
                probs_slice[t] = (if p < 256 {
                    if p != 0 {
                        p
                    } else {
                        1
                    }
                } else {
                    255
                }) as vp8_prob;
            } else {
                probs_slice[t] = vp8_prob_half;
            }
        }
    }
}
