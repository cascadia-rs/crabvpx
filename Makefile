# CrabVPX Makefile (Just Wrapper)
# Modern projects prefer 'just', so we forward commands to it.

.PHONY: help harness bench analyze clean

help:
	@echo "CrabVPX Command Runner"
	@echo "----------------------"
	@echo "harness : Run the test harness"
	@echo "bench   : Run performance benchmarks"
	@echo "analyze : Run complexity analysis"
	@echo "clean   : Clean build artifacts"
	@echo ""
	@echo "Note: This project uses 'just' for orchestration."
	@echo "If you have 'just' installed, you can run 'just <command>' directly."

harness:
	@./scripts/run_harness.py

bench:
	@./scripts/benchmark.py

analyze:
	@./scripts/analyze_complexity.py --src-dir src

clean:
	rm -rf out/
	cd harness && cargo clean
	cd libvpx && make clean
