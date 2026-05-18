# c2rust Learnings & Caveats

This document captures the caveats, quirks, and required manual interventions discovered while using `c2rust` to port `libvpx` to Rust.

## 1. Rust Editions

`c2rust` generates code that relies on pre-2024 Rust behaviors, specifically regarding `unsafe` code.

- **Extern Blocks:** In Rust 2024, `extern "C" { ... }` blocks must be declared as `unsafe extern "C"`. `c2rust` generates the older syntax.
- **Unsafe Operations in Unsafe Fns:** Rust 2024 enforces `unsafe_op_in_unsafe_fn`, requiring explicit `unsafe { ... }` blocks around raw pointer dereferences even inside an `unsafe fn`. `c2rust` assumes the older behavior where everything inside an `unsafe fn` is implicitly allowed.
- **Fix:** Keep the Cargo project on `edition = "2021"` initially. Once the project compiles, use `cargo fix --edition` to automatically upgrade the syntax.

## 2. Atomics and Intrinsics

`c2rust` maps C compiler builtins for atomics directly to `core::intrinsics`.

- `core::intrinsics` is unstable and meant strictly for compiler internals.
- **Fix:** Manually replace calls to `core::intrinsics::atomic_load_acquire` and `atomic_store_release` with standard Rust atomics using `core::sync::atomic::AtomicI32` and the `&raw` operator.

**Example C2Rust Output:**

```rust
return ::core::intrinsics::atomic_load_acquire(&raw const (*atomic).value);
```

**Corrected Rust:**

```rust
return (*(&raw const (*atomic).value as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::Acquire);
```

The `&raw const` (and `&raw mut`) syntax is necessary when casting raw pointers in transpiled C code to prevent the compiler from making strict aliasing or alignment assumptions that a normal reference (`&`) would imply, avoiding Undefined Behavior.

## 3. C-Variadics and VaList

When dealing with C-style variadic functions (`...`), `c2rust` uses the unstable `#![feature(c_variadic)]` and maps arguments to `core::ffi::VaList`.

- `c2rust` generates code that attempts to extract arguments using a generic `.arg::<T>()` method (e.g., `args.arg::<*mut ::core::ffi::c_int>()`).
- In recent Rust nightlies, the API for `VaList` has changed. The `.arg::<T>()` method no longer exists directly on the struct.
- **Fix:** Replace `.arg::<T>()` with `.next_arg::<T>()`. Additionally, when passing a `VaList` to another function, `c2rust` may generate `.as_va_list()`, which also no longer exists. Instead, just pass the `VaList` (or a clone of it) directly.

## 4. Platform-Specific SIMD Imports (ARM Neon)

When transpiling code that heavily uses platform-specific intrinsics (like ARM Neon), `c2rust` might fail to include the necessary architecture imports in every file.

- **Fix:** Manually add the appropriate architecture use statements to the top of the affected transpiled files. For ARM64 Neon, this means adding:

```rust
use std::arch::aarch64::*;
```

Note: Use `std::arch` instead of `core::arch` to avoid missing feature gates like `stdarch_aarch64` that might arise if trying to import directly from `core`.

## 5. SIMD Translation Strategy (`std::simd` vs. FFI vs. Intrinsics)
While it might seem tempting to rewrite the hardware-specific C intrinsics or Assembly using Rust's portable `std::simd`, it is generally not recommended for a "Lift and Shift" migration of a heavily optimized codec like `libvpx`:
- **Performance:** Video codecs rely on complex data shuffles, swizzles, and saturating math operations that map perfectly to specific hardware instructions (e.g., ARM Neon `vtbl` or x86 `pshufb`). `std::simd` is hardware-agnostic and may struggle to compile these operations down to the same optimal instructions, potentially causing significant performance regressions.
- **Stability:** The `std::simd` API is currently nightly-only and unstable.

**How `c2rust` Handles Optimizations:**
1. **C Intrinsics (`_neon.c`, `_sse2.c`):** `c2rust` struggles heavily with complex macros and intrinsic headers (like `<arm_neon.h>`). While it generates the `.rs` files, it often **silently drops** the actual function implementations, resulting in empty `.rs` files and missing symbols during linking. **Do not rely on the transpiled `_neon.rs` files.**
2. **Raw Assembly (`.asm` / `.S`):** `c2rust` *cannot* translate raw assembly files.

**Strategy:** Because `c2rust` fails to transpile the C-intrinsics and raw assembly, we **must** keep the original `_neon.c` and `.asm` files in the project. We compile them natively using the `cc` crate in `build.rs` and link to them from the transpiled Rust code via FFI (`extern "C"`). This preserves the critical hardware acceleration without forcing a manual rewrite of thousands of lines of SIMD code.

## 6. C-Flavored Rust and Binary Compatibility

One of the most important findings is the "philosophy" of the code `c2rust` produces. While it is 100% valid Rust code compiled by `rustc`, it is semantically **"C-flavored Rust."**

### Key Characteristics

- **Binary Compatibility:** By default, every function is marked `pub unsafe extern "C" fn` and often includes `#[no_mangle]`. This ensures the compiled Rust library maintains the exact same ABI as the original C library, allowing it to be a drop-in replacement.
- **C Semantics in Rust Syntax:** The code "thinks" like C. It uses raw pointers (`*mut T`) instead of references, manual memory management (`malloc`/`free`) via FFI, and relies heavily on `static mut` for global state.
- **The "Lift and Shift" Goal:** The objective of `c2rust` is **mechanical equivalence**, not idiomatic safety. It produces a codebase that passes pristine C tests (as verified by our Differential Test Harness) but requires significant manual "Rustification" to become safe, idiomatic code.

### The Path to Idiomatic Rust

The transition from "C-flavored Rust" to "Safe Rust" is an incremental process of:

1. Replacing raw pointers with safe references (`&` and `&mut`) and slices (`&[u8]`).
2. Moving `static mut` global state into instance-based structs or thread-safe containers.
3. Removing `extern "C"` and `#[no_mangle]` from internal functions that no longer need to be callable from C.
4. Introducing `Option` and `Result` to replace C-style error codes and null-pointer checks.
