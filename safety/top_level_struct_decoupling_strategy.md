# Top-Level State Struct Decoupling Strategy

## Status: Fully Accomplished (Phases 1-3 Complete)
All three core structural decoupling phases outlined below have been successfully implemented across CrabVPX. Long-lived pointer caching fields (`mode_info_context`, `above_context`, `left_context`) have been fully purged from `MACROBLOCKD`, replaced with absolute tracking indices (`mode_info_idx`, `above_context_idx`), and disjoint root destructuring (`split_mut()`) is actively utilized across single-threaded decoding pipelines.

---

## The Core Challenge: Self-Referential Data Structures & Cross-Referencing Pointers
In the legacy C architecture of CrabVPX (transpiled via `c2rust`), top-level decoder control structs—primarily `VP8D_COMP`, `VP8_COMMON`, and `MACROBLOCKD`—exhibit deep structural entanglement. 

Specifically, `MACROBLOCKD` (the working context for macroblock reconstruction) cached long-lived raw pointers (`*mut MODE_INFO`, `*mut ENTROPY_CONTEXT_PLANES`, and buffer pointers inside `YV12_BUFFER_CONFIG`) that pointed directly into buffers owned by `VP8_COMMON`. Because `VP8D_COMP` owns both `mb` (`MACROBLOCKD`) and `common` (`VP8_COMMON`), any attempt by an agent to replace these raw pointers with standard Rust mutable references (`&mut T`) immediately hit intractable **self-referential struct errors** and **lifetime / borrow checker errors** (`E0499`, `E0502`).

```
Legacy Entangled Memory Layout:
VP8D_COMP
 ├── common: VP8_COMMON (owns Box<[MODE_INFO]>, Box<[ENTROPY_CONTEXT_PLANES]>)
 └── mb: MACROBLOCKD
      ├── mode_info_context: *mut MODE_INFO ─────────────┐ (Points into common)
      └── above_context: *mut ENTROPY_CONTEXT_PLANES ────┘
```

To eliminate the raw pointer helper wrappers (`mode_info_mut()`, `above_context_mut()`) without breaking compilation or borrow checker constraints, agents executed a **structural decoupling strategy**.

---

## The Decoupling Roadmap (Execution Architecture)

### Phase 1: Convert Long-Lived Pointer Cache to Indices & Offsets (Complete)
Never store `&mut` or `&` references inside static, top-level state structs like `MACROBLOCKD`. Instead, replace cached raw pointers with absolute indices (`usize`) or grid coordinates (`stride`, `row`, `col`).

**Legacy Pattern (Unsafe Pointer Storage):**
```rust
pub struct macroblockd {
    pub mode_info_context: *mut MODE_INFO,
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    // ...
}
```

**Achieved Safe Architectural Equivalent (Offset/Index Storage):**
```rust
pub struct macroblockd {
    pub mode_info_idx: usize,
    pub above_context_idx: usize,
    // ...
}
```

---

### Phase 2: Design Transient Context Helpers (Dynamic Reborrowing) (Complete)
Because leaf functions require simultaneous access to both `MACROBLOCKD` buffers and `VP8_COMMON` data, create transient helper structs and context accessors that dynamically index from base slice pointers (`mip_ptr()`, `above_context_ptr()`) only for the duration of a function call.

```rust
impl macroblockd {
    pub fn mode_info(&self, mi_base: *const MODE_INFO) -> &MODE_INFO {
        unsafe { &*mi_base.add(self.mode_info_idx) }
    }
    pub fn mode_info_mut(&mut self, mi_base: *mut MODE_INFO) -> &mut MODE_INFO {
        unsafe { &mut *mi_base.add(self.mode_info_idx) }
    }
}
```

---

### Phase 3: Implement Disjoint Destructuring at the Root (`VP8D_COMP`) (Complete)
When operating at the `VP8D_COMP` root, passing `&mut pbi` to subroutines causes massive reborrowing lockups. Implement explicit destructuring methods on `VP8D_COMP` to split the state into disjoint mutable borrows.

```rust
impl VP8D_COMP {
    /// Safely splits the root structure into disjoint mutable components
    pub fn split_mut(&mut self) -> (&mut MACROBLOCKD, &mut VP8_COMMON, &mut [vp8_reader; 9]) {
        (&mut self.mb, &mut self.common, &mut self.mbc)
    }
}
```

---

## Critical Rules of Engagement for Agents

1. **One Field at a Time:** Do not attempt to refactor `MACROBLOCKD` or `VP8_COMMON` entirely in a single turn. Select exactly one raw pointer field, convert its usage across the active call path, verify it builds, and commit.
2. **Never Box References:** Do not put references in smart pointers (`Box<&mut T>`) or resort to interior mutability hacks (`RefCell`, `UnsafeCell`) to bypass structural borrow conflicts.
3. **Continuous Verification:** After modifying struct fields or FFI signatures in `src/vp8/common/types.rs`, immediately execute `./build.sh` to confirm compilation and `./scripts/compare.py` to guarantee bit-exact decoder alignment.
4. **Leave Hand-Off Breadcrumbs:** Record your progress on entangled field conversion in `safety/HINTS.md` so subsequent agents know exactly which pointer caching fields remain.
