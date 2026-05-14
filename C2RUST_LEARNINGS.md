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
