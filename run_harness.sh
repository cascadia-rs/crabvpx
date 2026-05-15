#!/bin/bash
set -e

# Define directories
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HARNESS_DIR="$DIR/harness"
TEST_DATA_DIR="$DIR/libvpx-test-data"
BASE_URL="https://storage.googleapis.com/downloads.webmproject.org/test_data/libvpx"

# Why not `make testdata`?
# 1. `make testdata` requires `libvpx` to be configured with `--enable-unit-tests`.
# 2. `make testdata` downloads hundreds of megabytes of vectors for VP9, encoders, etc.
# Since we are currently focusing solely on the VP8 decoder port, we manually fetch 
# only the comprehensive VP8 decoder test vectors to keep things lightweight and fast.

# VP8 Comprehensive Test Vectors
VP8_VECTORS=(
    "vp80-00-comprehensive-001.ivf"
    "vp80-00-comprehensive-002.ivf"
    "vp80-00-comprehensive-003.ivf"
    "vp80-00-comprehensive-004.ivf"
    "vp80-00-comprehensive-005.ivf"
    "vp80-00-comprehensive-006.ivf"
    "vp80-00-comprehensive-007.ivf"
    "vp80-00-comprehensive-008.ivf"
    "vp80-00-comprehensive-009.ivf"
    "vp80-00-comprehensive-010.ivf"
    "vp80-00-comprehensive-011.ivf"
    "vp80-00-comprehensive-012.ivf"
    "vp80-00-comprehensive-013.ivf"
    "vp80-00-comprehensive-014.ivf"
    "vp80-00-comprehensive-015.ivf"
    "vp80-00-comprehensive-016.ivf"
    "vp80-00-comprehensive-017.ivf"
    "vp80-00-comprehensive-018.ivf"
    "vp80-01-intra-1400.ivf"
    "vp80-01-intra-1411.ivf"
    "vp80-01-intra-1416.ivf"
    "vp80-01-intra-1417.ivf"
    "vp80-02-inter-1402.ivf"
    "vp80-02-inter-1412.ivf"
    "vp80-02-inter-1418.ivf"
    "vp80-02-inter-1424.ivf"
    "vp80-03-segmentation-01.ivf"
    "vp80-03-segmentation-02.ivf"
    "vp80-03-segmentation-03.ivf"
    "vp80-03-segmentation-04.ivf"
    "vp80-03-segmentation-1401.ivf"
    "vp80-03-segmentation-1403.ivf"
    "vp80-03-segmentation-1407.ivf"
    "vp80-03-segmentation-1408.ivf"
    "vp80-03-segmentation-1409.ivf"
)

mkdir -p "$TEST_DATA_DIR"

echo "Checking for VP8 test vectors in $TEST_DATA_DIR..."
for vector in "${VP8_VECTORS[@]}"; do
    if [ ! -f "$TEST_DATA_DIR/$vector" ]; then
        echo "Downloading $vector..."
        curl -S -s --retry 3 -L -o "$TEST_DATA_DIR/$vector" "$BASE_URL/$vector"
    fi
done

echo ""
echo "--- Testing C Oracle Decoder ---"
cd "$HARNESS_DIR"
cargo run --release -- --dir "$TEST_DATA_DIR"

echo ""
echo "--- Testing CrabVPX Rust Decoder ---"
cargo run --release --no-default-features --features rust -- --dir "$TEST_DATA_DIR"

echo ""
# Run the unsafe tracker
"$DIR/scripts/count_unsafe.sh"

