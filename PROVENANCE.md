# Provenance

crabvpx is a Rust port of the **VP8 decoder** from
[libvpx](https://github.com/webmproject/libvpx), the WebM Project's reference
VP8/VP9 codec library.

- The initial Rust was produced by running [`c2rust`](https://github.com/immunant/c2rust)
  over libvpx's VP8 decoder (configured for decode-only), then progressively
  rewritten into safe, idiomatic Rust. Decode output is verified bit-exact
  against libvpx across the WebM VP8 conformance vectors.
- Original C code: copyright the WebM Project authors, 3-Clause BSD
  ([`LICENSE`](LICENSE)) plus the additional patent grant ([`PATENTS`](PATENTS)).
  Those terms are retained here, as required.
- The Rust port and subsequent changes are copyright Jordan Bayles, under the
  same 3-Clause BSD license.

## Test vectors

`test_data/` contains VP8 conformance bitstreams (`*.ivf`) and their reference
per-frame MD5 digests (`*.ivf.md5`) from the WebM Project's
[libvpx test data](https://chromium.googlesource.com/webm/libvpx-test-data),
redistributed under the same terms as libvpx. They are used only to verify the
decoder against the reference.
