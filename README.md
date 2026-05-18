# CrabVPX

Lifting and shifting `libvpx` (specifically VP8 decoding) to Rust using `c2rust`.

## Project Status

- **Transpilation Complete**: The VP8 decoder has been successfully transpiled to Rust using `c2rust`.
- **Rust 2024**: Codebase upgraded to the Rust 2024 edition and builds cleanly.
- **Cross-Platform**: Compiles and runs on Linux, macOS, and Windows.
- **Differential Testing**: We use a custom test harness to run side-by-side differential tests against the original C `libvpx` to ensure 100% bit-for-bit correctness across hundreds of test frames.
- **CI/CD**: GitHub Actions are set up for cross-platform builds, testing, and strict formatting/linting checks.

## Development

The project uses `just` as a command runner. You can run this command without any arguments to see
the full list of supported commands. The most important commands include:

- **`just build`**: Builds the Rust workspace.
- **`just test`**: Runs the Rust test suite.
- **`just lint`**: Runs `cargo fmt` and `cargo clippy` with strict warnings.
- **`just compare`**: Runs the differential test suite (Rust decoder vs C Oracle).
- **`just bench`**: Runs performance benchmarks.

## Next Steps

- **Incremental Refactoring to Safe Rust APIs**: Begin converting the generated
  `unsafe` C-style bindings into idiomatic and safe Rust wrappers (see
  `docs/refactor_plan.md`).
