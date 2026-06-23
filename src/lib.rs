//! # crabvpx
//!
//! A memory-safe, pure-Rust VP8 video decoder. Originally lifted from
//! `libvpx` and progressively rewritten into safe Rust; decode output is
//! bit-exact with `libvpx`. On aarch64, NEON kernels (loop filter and
//! sub-pixel filter, in [`vp8::common::neon`]) bring decode to roughly
//! 1.1–1.5× of libvpx on heavy content and faster on lighter streams; other
//! targets use the auto-vectorized scalar path. All SIMD `unsafe` is
//! confined to the NEON module, each kernel bit-exact-gated against its
//! scalar twin.
//!
//! ## Decoding
//!
//! Use [`api::Vp8Decoder`] via the [`api::Decoder`] trait:
//!
//! ```no_run
//! use crabvpx::api::{Decoder, Plane, Vp8Decoder};
//!
//! let mut dec = Vp8Decoder::new();
//! dec.init().unwrap();
//! // `frame` is one compressed VP8 frame (e.g. an IVF payload).
//! # let frame: &[u8] = &[];
//! dec.decode(frame).unwrap();
//! while let Some(img) = dec.get_frame().unwrap() {
//!     let y = img.plane(Plane::Y).unwrap();
//!     let stride = img.stride(Plane::Y);
//!     let _ = (img.width(), img.height(), y, stride);
//! }
//! ```
//!
//! Multithreaded decoding is opt-in via the `CRABVPX_THREADS` environment
//! variable (default `1`) and is bit-identical to single-threaded.
//!
//! ## Scope
//!
//! VP8 **decoding** only — no encoder, and no VP9/AV1.

// === Deliberate, documented lint exemptions ===
// rustc and clippy are otherwise blocking CI gates; everything below is a
// justified exception. (`unused_mut`, `unused_assignments` and `dead_code` — the
// bulk of the original c2rust noise — were removed, not exempted: the scalar
// fallback twins that are dead only in the aarch64 NEON build carry a targeted
// `#[cfg_attr(target_arch = "aarch64", allow(dead_code))]` instead.)
//
// Naming: this is a faithful port of libvpx's C and the identifiers are kept
// byte-for-byte (`MACROBLOCKD`, `vp8_prob`, `QIndex`, `vp8_default_zig_zag1d`,
// …) so the source-citation doc comments (`// vp8/common/foo.c:NN`) line up with
// the upstream symbol names. Renaming to Rust conventions would sever that
// cross-referencing across the whole tree, so these stay by design:
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// Structural lints that don't fit this libvpx-derived numeric code and are not
// worth churning the whole tree for. Keep this list small.
//   - too_many_arguments: kernel/codec signatures inherited from the C source.
//   - needless_range_loop: verified — every site is a strided / multi-array
//     pixel or SIMD load-store loop (`r[i] = load(base + i*p - 4)`) where the
//     index drives pointer arithmetic; the range form is clearer than the
//     iterator rewrite (clippy --fix declines all of them).
//   - type_complexity: function-pointer dispatch-table types.
//   - module_inception: module layout mirrors libvpx (e.g. vpx_mem::vpx_mem).
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]

pub mod api;
pub mod thread_shim;
pub mod vp8 {
    pub mod common {
        pub mod alloccommon;
        pub mod blockd;
        pub mod dequantize;
        pub mod entropy;
        pub mod entropymode;
        pub mod entropymv;
        pub mod extend;
        pub mod filter;
        pub mod findnearmv;
        pub mod simd;
        pub mod generic {
            pub mod systemdependent;
        } // mod generic
        pub mod idct_blk;
        pub mod idctllm;
        pub mod loopfilter_filters;
        pub mod mbpitch;
        pub mod modecont;
        pub mod quant_common;
        pub mod reconinter;
        pub mod reconintra;
        pub mod reconintra4x4;
        pub mod rtcd;
        pub mod safe_predict;
        pub mod setupintrarecon;
        pub mod swapyv12buffer;
        pub mod treecoder;
        pub mod types;
        pub mod vp8_loopfilter;
    } // mod common
    pub mod decoder {
        pub mod dboolhuff;
        pub mod decodeframe;
        pub mod decodemv;
        pub mod detokenize;
        pub mod onyxd_if;
        pub mod threading;
    } // mod decoder
    pub mod vp8_dx_iface;
} // mod vp8
pub mod vpx {
    pub mod src {
        // pub mod vpx_codec;
        // pub mod vpx_decoder;
        // pub mod vpx_encoder;
        // pub mod vpx_image;
    } // mod src
} // mod vpx
pub mod vpx_config;
pub mod vpx_dsp {
    pub mod arm {
        // pub mod intrapred_neon;
    } // mod arm
    // pub mod bitreader;
    // pub mod bitreader_buffer;
    pub mod intrapred;
    // pub mod prob;
    // pub mod skin_detection;
    pub mod vpx_dsp_rtcd;
} // mod vpx_dsp
pub mod vpx_mem {
    pub mod vpx_mem;
} // mod vpx_mem
pub mod vpx_scale {
    pub mod generic {
        // pub mod gen_scalers;
        // pub mod vpx_scale;
        pub mod yv12config;
        pub mod yv12extend;
    } // mod generic
    pub mod vpx_scale_rtcd;
} // mod vpx_scale
// pub mod vpx_util {
//     pub mod vpx_write_yuv_frame;
// } // mod vpx_util
