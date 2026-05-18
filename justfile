# CrabVPX Command Runner

# Default: list all commands
default:
    @just --list

# libvpx configuration flags for VP8-focused decoding
libvpx_config_flags := "--enable-shared --disable-static --enable-vp8 --disable-vp8-encoder --disable-vp9 --enable-multithread --enable-postproc --enable-pic --enable-runtime-cpu-detect"

# Configure and build the C Oracle (libvpx) for VP8 decoding
configure:
    git submodule update --init --recursive
    cd libvpx && ./configure {{libvpx_config_flags}}
    cd libvpx && make clean
    cd libvpx && make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)

# Run all tests (including differential vector testing)
test *args:
    cd harness && cargo test --release -- --nocapture {{args}}

# Alias for backwards compatibility
compare *args:
    @just test {{args}}

# Run performance benchmarks with statistical distribution
bench *args:
    cd harness && cargo run --release --bin benchmark -- --dir ../test_data --benchmark {{args}}

# Run complexity and technical debt analysis
analyze *args:
    cd tools/complexity_analyzer && cargo run --release -- {{args}}

# Clean output and build artifacts
clean:
    rm -rf out/
    cd harness && cargo clean
    cd libvpx && make clean

# Build the Rust workspace
build:
    cargo build --workspace

# Format the Rust workspace
fmt:
    cargo fmt --all

# Run linting and formatting checks
lint:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings

# Run tests in the Rust workspace
test-all:
    cargo test --workspace
