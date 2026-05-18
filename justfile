# CrabVPX Command Runner

# Default: list all commands
default:
    @just --list

# libvpx configuration flags for VP8-focused decoding
libvpx_config_flags := "--enable-vp8 --disable-vp8-encoder --disable-vp9 --enable-multithread --enable-postproc --enable-pic --enable-runtime-cpu-detect"

# Configure and build the C Oracle (libvpx) for VP8 decoding
configure:
    git submodule update --init --recursive
    cd libvpx && ./configure {{libvpx_config_flags}}
    cd libvpx && make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu) vpx

# Run differential testing (Oracle vs Rust)
compare *args:
    python ./scripts/compare.py {{args}}

# Run performance benchmarks with statistical distribution
bench *args:
    python ./scripts/benchmark.py {{args}}

# Run complexity and technical debt analysis
analyze *args:
    python ./scripts/analyze_complexity.py {{args}}

# Clean output and build artifacts
clean:
    rm -rf out/
    cd harness && cargo clean
    cd libvpx && make clean

# Build the Rust workspace
build:
    cargo build --workspace

# Run tests in the Rust workspace
test:
    cargo test --workspace
