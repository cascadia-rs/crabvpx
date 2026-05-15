# CrabVPX Makefile (Just Wrapper)
# Modern projects prefer 'just', so we forward commands to it.

.PHONY: help compare bench analyze clean bootstrap

help:
	@echo "CrabVPX Command Runner"
	@echo "----------------------"
	@echo "bootstrap: Install required development tools (just)"
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
