# CrabVPX Command Runner

# Default: list all commands
default:
    @just --list

# Configure and build the C Oracle (libvpx)
configure:
    git submodule update --init --recursive
    cd libvpx && ./configure --enable-vp8 --enable-vp9 --enable-multithread --enable-postproc --enable-pic --enable-runtime-cpu-detect
    cd libvpx && make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)

# Run differential testing (Oracle vs Rust)
compare *args:
    ./scripts/compare.py {{args}}

# Run performance benchmarks with statistical distribution
bench *args:
    ./scripts/benchmark.py {{args}}

# Run complexity and technical debt analysis
analyze *args:
    ./scripts/analyze_complexity.py {{args}}

# Clean output and build artifacts
clean:
    rm -rf out/
    cd harness && cargo clean
    cd libvpx && make clean
