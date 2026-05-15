# CrabVPX Makefile (Just Wrapper)
# Modern projects prefer 'just', so we forward commands to it.

.PHONY: help compare bench analyze clean bootstrap configure

help:
	@echo "CrabVPX Command Runner"
	@echo "----------------------"
	@echo "bootstrap: Install required development tools (just)"
	@echo "configure: Setup and build the C Oracle (libvpx)"
	@echo "compare : Run differential testing"
	@echo "bench   : Run performance benchmarks"
	@echo "analyze : Run complexity analysis"
	@echo "clean   : Clean build artifacts"
	@echo ""
	@echo "Note: This project uses 'just' for orchestration."
	@echo "If you have 'just' installed, you can run 'just <command>' directly."

bootstrap:
	@echo "Installing 'just' via cargo..."
	cargo install just

configure:
	git submodule update --init --recursive
	cd libvpx && ./configure --enable-vp8 --enable-vp9 --enable-multithread --enable-postproc --enable-pic --enable-runtime-cpu-detect
	cd libvpx && make -j$$(nproc 2>/dev/null || sysctl -n hw.ncpu)

compare:
	@./scripts/compare.py

bench:
	@./scripts/benchmark.py

analyze:
	@./scripts/analyze_complexity.py --src-dir src

clean:
	rm -rf out/
	cd harness && cargo clean
	cd libvpx && make clean
