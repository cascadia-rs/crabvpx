//! SIMD kernels behind a single `Simd` trait so each kernel's logic is written
//! once (in `kernels`) and shared across instruction sets; per-ISA backends
//! only provide the primitive ops. See `docs/simd-architecture.md`.
//!
//! Today only the NEON backend exists (aarch64); SSE slots in by adding another
//! `impl Simd`. The scalar functions in `loopfilter_filters`/`filter` remain the
//! bit-exact reference every backend is gated against.

// `kernels` and the `Simd` trait exist only to serve the per-ISA backends, all
// of which are aarch64-only today, so they are dead on every other target.
#[cfg(target_arch = "aarch64")]
pub(crate) mod kernels;
#[cfg(target_arch = "aarch64")]
pub(crate) mod neon;

/// Primitive SIMD ops over an 8-lane `u8` vector (`U8`) and an 8-lane `i16`
/// vector (`I16`). Methods are `#[inline(always)]` in impls so the generic
/// kernels monomorphize into straight-line vector code per ISA.
///
/// All arithmetic is safe (intrinsics are safe inside `#[target_feature]` on
/// Rust >= 1.87, and NEON is aarch64 baseline); only `load_u8`/`store_u8` are
/// `unsafe` (raw pointer access).
#[cfg(target_arch = "aarch64")]
pub(crate) trait Simd {
    type U8: Copy;
    type I16: Copy;

    /// # Safety
    /// `p` must be valid for an 8-byte read.
    unsafe fn load_u8(p: *const u8) -> Self::U8;
    /// # Safety
    /// `p` must be valid for an 8-byte write.
    unsafe fn store_u8(p: *mut u8, v: Self::U8);

    // --- u8 lane ops ---
    fn splat_u8(x: u8) -> Self::U8;
    fn abd_u8(a: Self::U8, b: Self::U8) -> Self::U8; // |a - b|
    fn cgt_u8(a: Self::U8, b: Self::U8) -> Self::U8; // 0xFF where a > b
    fn or_u8(a: Self::U8, b: Self::U8) -> Self::U8;
    fn not_u8(a: Self::U8) -> Self::U8;

    // --- conversions ---
    fn widen_u8(a: Self::U8) -> Self::I16; // zero-extend (values 0..=255)
    fn widen_mask(a: Self::U8) -> Self::I16; // sign-extend a 0x00/0xFF byte mask
    fn narrow_mask(a: Self::I16) -> Self::U8; // 0x0000/0xFFFF i16 mask -> 0x00/0xFF
    fn to_signed_i16(a: Self::U8) -> Self::I16; // (a ^ 0x80) as i8, widened
    fn from_signed_i16(a: Self::I16) -> Self::U8; // narrow to i8, ^ 0x80

    // --- i16 lane ops ---
    fn splat_i16(x: i16) -> Self::I16;
    fn add_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn sub_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn mul_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn and_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn cgt_i16(a: Self::I16, b: Self::I16) -> Self::I16; // 0xFFFF where a > b
    fn min_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn max_i16(a: Self::I16, b: Self::I16) -> Self::I16;
    fn shl_i16<const N: i32>(a: Self::I16) -> Self::I16;
    fn shr_i16<const N: i32>(a: Self::I16) -> Self::I16; // arithmetic

    /// 8x8 byte transpose (rows -> columns). `out[j]` = column `j` of input.
    fn transpose8x8(rows: [Self::U8; 8]) -> [Self::U8; 8];
}
