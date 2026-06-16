#!/bin/bash -eu
# OSS-Fuzz build script for crabvpx.
# Copied into the OSS-Fuzz project dir as projects/crabvpx/build.sh.
# OSS-Fuzz invokes this inside its build container after cloning the repo to $SRC.

cd "$SRC/crabvpx"

# cargo-fuzz is preinstalled in the OSS-Fuzz Rust base image.
cargo fuzz build -O

FUZZ_RELEASE="fuzz/target/x86_64-unknown-linux-gnu/release"

for target in decode_frame decode_ivf; do
  cp "$FUZZ_RELEASE/$target" "$OUT/"
done

# Seed corpus for the container-format target: the project's valid VP8 vectors.
if ls test_data/*.ivf >/dev/null 2>&1; then
  zip -j "$OUT/decode_ivf_seed_corpus.zip" test_data/*.ivf
fi
