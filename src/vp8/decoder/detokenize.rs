//! DCT coefficient token decode — port of `vp8/decoder/detokenize.c`.
//!
//! Reads the quantized DCT coefficients for a macroblock from the boolean
//! decoder via the coefficient token tree (the per-frame hot path). `GetCoeffs`
//! walks one block; `vp8_decode_mb_tokens` drives all 24/25 blocks.

pub type vpx_color_space = u32;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub use crate::vp8::common::types::*;
pub type ProbaArray = *const [[u8; 11]; 3];
pub const CHAR_BIT: i32 = 8_i32;
pub const VP8_BD_VALUE_SIZE: i32 = ::core::mem::size_of::<VP8_BD_VALUE>() as i32 * CHAR_BIT;
/// `vp8_reset_mb_tokens_context` — vp8/decoder/detokenize.c:18. Zeroes the
/// above/left nonzero-coefficient entropy contexts for a skipped macroblock.
pub fn vp8_reset_mb_tokens_context(
    a_ctx: &mut ENTROPY_CONTEXT_PLANES,
    l_ctx: &mut ENTROPY_CONTEXT_PLANES,
    is_4x4: bool,
) {
    a_ctx.y1 = [0; 4];
    a_ctx.u = [0; 2];
    a_ctx.v = [0; 2];
    l_ctx.y1 = [0; 4];
    l_ctx.u = [0; 2];
    l_ctx.v = [0; 2];
    if !is_4x4 {
        a_ctx.y2 = 0;
        l_ctx.y2 = 0;
    }
}
static kBands: [u8; 17] = [0, 1, 2, 3, 6, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 0];
static kCat3: [u8; 3] = [173, 148, 140];
static kCat4: [u8; 4] = [176, 155, 140, 135];
static kCat5: [u8; 5] = [180, 157, 141, 134, 130];
static kCat6: [u8; 11] = [254, 254, 243, 230, 196, 177, 153, 140, 133, 130, 129];
static kCat3456: [&[u8]; 4] = [&kCat3, &kCat4, &kCat5, &kCat6];
static kZigzag: [u8; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];
fn GetSigned(br: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder, value_to_sign: i32) -> i32 {
    if br.read_bool(128_i32) != 0 {
        -value_to_sign
    } else {
        value_to_sign
    }
}
/// Decode one block's coefficient run from the boolean decoder via the token
/// tree (the inner loop of `vp8_decode_mb_tokens`). Returns the count of
/// coefficients written (the block's eob).
fn GetCoeffs(
    br: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder,
    prob: &[[[vp8_prob; 11]; 3]; 8],
    ctx: i32,
    mut n: i32,
    out: &mut [i16],
) -> i32 {
    let mut p: &[vp8_prob; 11] = &prob[n as usize][ctx as usize];
    if br.read_bool(p[0] as i32) == 0 {
        return 0;
    }
    loop {
        n += 1;
        if br.read_bool(p[1] as i32) == 0 {
            p = &prob[kBands[n as usize] as usize][0];
        } else {
            let mut v: i32 = 0;
            let mut j: i32 = 0;
            if br.read_bool(p[2] as i32) == 0 {
                p = &prob[kBands[n as usize] as usize][1];
                v = 1;
            } else {
                if br.read_bool(p[3] as i32) == 0 {
                    if br.read_bool(p[4] as i32) == 0 {
                        v = 2;
                    } else {
                        v = 3 + br.read_bool(p[5] as i32);
                    }
                } else if br.read_bool(p[6] as i32) == 0 {
                    if br.read_bool(p[7] as i32) == 0 {
                        v = 5 + br.read_bool(159);
                    } else {
                        v = 7 + 2 * br.read_bool(165);
                        v += br.read_bool(145);
                    }
                } else {
                    let bit1 = br.read_bool(p[8] as i32);
                    let bit0 = br.read_bool(p[(9 + bit1) as usize] as i32);
                    let cat = 2 * bit1 + bit0;
                    v = 0;
                    let tab = kCat3456[cat as usize];
                    for &prob_val in tab {
                        v = 2 * v + br.read_bool(prob_val as i32);
                    }
                    v += 3 + (8 << cat);
                }
                p = &prob[kBands[n as usize] as usize][2];
            }
            j = kZigzag[(n - 1) as usize] as i32;
            out[j as usize] = GetSigned(br, v) as i16;
            if n == 16 || br.read_bool(p[0] as i32) == 0 {
                return n;
            }
        }
        if n == 16 {
            return 16;
        }
    }
}
/// `vp8_decode_mb_tokens` — vp8/decoder/detokenize.c:142. Decodes all blocks of
/// a macroblock (Y2 if present, 16 Y, 8 UV), updating entropy contexts and the
/// per-block `eobs`. Returns the total nonzero-coefficient count.
pub fn vp8_decode_mb_tokens(
    mut safe_decoder: &mut crate::vp8::decoder::dboolhuff::SafeBoolDecoder,
    fc_ref: &FRAME_CONTEXT,
    qcoeff: &mut [i16; 400],
    eobs: &mut [::core::ffi::c_char; 25],
    a_ctx: &mut ENTROPY_CONTEXT_PLANES,
    l_ctx: &mut ENTROPY_CONTEXT_PLANES,
    is_4x4: bool,
) -> i32 {
    let mut eobtotal: i32 = 0;

    let mut skip_dc: i32 = 0;
    if !is_4x4 {
        let coef_probs = &fc_ref.coef_probs[1];
        let ctx = a_ctx.y2 as i32 + l_ctx.y2 as i32;
        let nonzeros = GetCoeffs(
            safe_decoder,
            coef_probs,
            ctx,
            0,
            &mut qcoeff[24 * 16..25 * 16],
        );
        let nonzero_bool = (nonzeros > 0) as i32 as ENTROPY_CONTEXT;
        l_ctx.y2 = nonzero_bool;
        a_ctx.y2 = nonzero_bool;
        eobs[24] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros - 16;
        skip_dc = 1;
    } else {
        skip_dc = 0;
    }

    let coef_probs_y1 = if !is_4x4 {
        &fc_ref.coef_probs[0]
    } else {
        &fc_ref.coef_probs[3]
    };

    for i in 0..16 {
        let col = i & 3;
        let row = i >> 2;
        let ctx = a_ctx.y1[col] as i32 + l_ctx.y1[row] as i32;
        let nonzeros = GetCoeffs(
            safe_decoder,
            coef_probs_y1,
            ctx,
            skip_dc,
            &mut qcoeff[(i * 16)..((i + 1) * 16)],
        );
        let nonzero_bool = (nonzeros > 0) as i32 as ENTROPY_CONTEXT;
        l_ctx.y1[row] = nonzero_bool;
        a_ctx.y1[col] = nonzero_bool;
        let eob_val = nonzeros + skip_dc;
        eobs[i] = eob_val as ::core::ffi::c_char;
        eobtotal += eob_val;
    }

    let coef_probs_uv = &fc_ref.coef_probs[2];

    for i in 16..20 {
        let col = i & 1;
        let row = (i & 3) >> 1;
        let ctx = a_ctx.u[col] as i32 + l_ctx.u[row] as i32;
        let nonzeros = GetCoeffs(
            safe_decoder,
            coef_probs_uv,
            ctx,
            0,
            &mut qcoeff[(i * 16)..((i + 1) * 16)],
        );
        let nonzero_bool = (nonzeros > 0) as i32 as ENTROPY_CONTEXT;
        l_ctx.u[row] = nonzero_bool;
        a_ctx.u[col] = nonzero_bool;
        eobs[i] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros;
    }

    for i in 20..24 {
        let col = i & 1;
        let row = (i & 3) >> 1;
        let ctx = a_ctx.v[col] as i32 + l_ctx.v[row] as i32;
        let nonzeros = GetCoeffs(
            safe_decoder,
            coef_probs_uv,
            ctx,
            0,
            &mut qcoeff[(i * 16)..((i + 1) * 16)],
        );
        let nonzero_bool = (nonzeros > 0) as i32 as ENTROPY_CONTEXT;
        l_ctx.v[row] = nonzero_bool;
        a_ctx.v[col] = nonzero_bool;
        eobs[i] = nonzeros as ::core::ffi::c_char;
        eobtotal += nonzeros;
    }

    eobtotal
}
