# Test vectors

VP8 conformance bitstreams (`*.ivf`) and their reference per-frame MD5 digests
(`*.ivf.md5`) from the WebM Project's
[libvpx test data](https://chromium.googlesource.com/webm/libvpx-test-data),
redistributed under the same terms as libvpx (see ../LICENSE and ../PATENTS).

`tests/differential_md5.rs` decodes each vector and checks every frame's MD5
against the reference digest — a bit-exact differential test against libvpx that
needs no libvpx build at test time.
