//! Tree-coder probability helpers — port of `vp8/common/treecoder.c`.
//!
//! Builds token-tree probabilities/counts (the bool-coded tree machinery shared
//! by the entropy models).

pub use crate::vp8::common::types::vp8_prob;
pub type vp8_tree_index = i8;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: i32,
    pub Len: i32,
}
pub type vp8_token = vp8_token_struct;
pub const vp8_prob_half: vp8_prob = 128_i32 as vp8_prob;
