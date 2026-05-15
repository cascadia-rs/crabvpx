# Contributing to CrabVPX

Welcome to the CrabVPX project! This document outlines how to set up your development environment and use our tooling.

## Getting Started

### 1. Prerequisites

- **Rust**: Ensure you have the latest stable Rust installed.
- **Python 3**: Required for orchestration and analysis scripts.
- **C Build Tools**: You need `make` and a C compiler (like `gcc` or `clang`) to build the C Oracle.

### 2. Initial Setup

We use `just` as our primary command runner. You can bootstrap your environment by running:

```bash
make bootstrap
```

This will install `just` via `cargo`.

## Development Workflow

We use a unified interface for our common tasks. Once `just` is installed, you can use the following commands:

- **`just compare`**: Runs differential testing. This verifies the Rust implementation against the C Oracle using official VP8 test vectors.
- **`just bench`**: Runs statistical performance benchmarks and generates a distribution analysis.
- **`just analyze`**: Runs a complexity analysis to track technical debt (unsafe blocks, `static mut`, etc.).
- **`just clean`**: Cleans build artifacts for both Rust and C components.

### Legacy Support

If you prefer not to use `just`, or are in an environment where it isn't available, the `Makefile` in the root directory provides a backwards-compatible wrapper for these commands (e.g., `make compare`).

## Project Structure

- `src/`: The transpiled Rust codebase.
- `libvpx/`: The original C source code (Oracle).
- `harness/`: The Rust-based differential testing infrastructure.
- `scripts/`: Python-based orchestration and analysis engines.
- `docs/`: In-depth project analysis and learning documents.
