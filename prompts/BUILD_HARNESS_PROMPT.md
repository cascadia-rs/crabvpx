# Prompt for New Gemini CLI Instance: Building the VP8 Test Harness

*Instructions for the user: You can copy and paste the text below into a new Gemini CLI session (or save it to a file and pipe it in) to bootstrap the development of the test harness.*

***

**System/Role:**
You are an expert Rust systems engineer, specializing in FFI (`bindgen`, `cc`), video codecs, and CLI tool development.

**Context:**
We are in the process of porting `libvpx` (specifically VP8 decoding) to Rust. The project is currently called `crabvpx`.
Right now, the raw transpiled Rust code from `c2rust` does **not** compile, and we are working on fixing those errors.

**Immediate Goal:**
Before we can test the Rust code, we need to build the "Oracle" component of our Differential Test Harness.
I need you to build a standalone Rust CLI tool that tests the original, pristine C version of `libvpx` against its VP8 test vectors (`.ivf` files).

**Requirements for the CLI Harness:**

1.  **Project Setup:**
    *   Initialize a new Rust binary project (e.g., `harness`).
    *   Set up a `build.rs` script using the `cc` crate to compile the necessary upstream C files from the `libvpx/` directory into a static library (`libvpx.a`). *Assume the user has already run `./configure` in `libvpx` to generate necessary headers like `vpx_config.h`.*
    *   Use `bindgen` in `build.rs` to generate safe Rust FFI bindings for `vpx/vpx_decoder.h` and the VP8 decoder interface (`vpx/vp8dx.h`). Put these in a `libvpx-sys` module.

2.  **IVF Parser:**
    *   Write a simple Rust parser for the `.ivf` (Indeo Video Format) file container.
    *   It should be able to read the IVF file header and extract sequential encoded video frames (payload size and payload data).

3.  **Decoding Loop (C Oracle):**
    *   Write the Rust code that uses the generated `bindgen` FFI to:
        1. Initialize the `vpx_codec_ctx_t` decoder using `vpx_codec_vp8_dx()`.
        2. Iterate over the frames extracted from an `.ivf` file.
        3. Pass each compressed frame to `vpx_codec_decode`.
        4. Retrieve the uncompressed YUV frame using `vpx_codec_get_frame`.
        5. Destroy the decoder context when finished.

4.  **CLI, Progress Tracking, & Performance:**
    *   Use `clap` to take a directory path containing `.ivf` files as an argument.
    *   Use the `indicatif` crate to display a rich, live progress bar as the tool crunches through the test vectors.
    *   Implement lightweight performance tracking by default (e.g., simple timing of the decode calls) to catch severe regressions early.
    *   Add an optional `--extensive-perf` flag for rigorous statistical benchmarking (using `criterion` or similar) to verify the decoder's speed against the original C baseline.
    *   The CLI should output a final report: "X out of Y vectors decoded successfully by the C Oracle in Z ms/frame."

5.  **Architecture for the Future:**
    *   Structure the code so that an `Interface` or `Trait` defines a `VideoDecoder`.
    *   Implement this trait for `LibVpxOracleDecoder`.
    *   Leave a stub/placeholder implementation for `CrabVpxDecoder` (which we will fill in later once the Rust code compiles).
    *   The main testing loop should be designed to eventually run *both* implementations and `assert_eq!` the output YUV buffers. For now, it should just run the C version and verify it doesn't return errors.

**Please provide the implementation step-by-step:**
1.  The `Cargo.toml` dependencies.
2.  The `build.rs` script for `cc` and `bindgen`.
3.  The IVF parser module.
4.  The Decoder Trait and FFI wrapper.
5.  The main CLI application with `indicatif`.

Do not attempt to fix or run the `crabvpx` transpiled code; focus entirely on wrapping the C code and building the testing infrastructure.
