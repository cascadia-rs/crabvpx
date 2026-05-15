#!/usr/bin/env python3
"""
analyze_complexity.py

This script serves as both a complexity analyzer for existing c2rust transpiled
Rust code, and an automated pipeline to simulate a full port of libvpx to Rust.

Usage:
  # Analyze an existing directory (like our current src/)
  python3 scripts/analyze_complexity.py --src-dir src/

  # Run the full pipeline (clone, configure, c2rust, analyze) in out/
  python3 scripts/analyze_complexity.py --full-pipeline
"""

import argparse
import logging
import multiprocessing
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple

logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")


def run_cmd(cmd: List[str], cwd: Path, env: Dict[str, str] = None) -> None:
    """Runs a shell command and checks for success."""
    logging.info("Running command: %s", ' '.join(cmd))
    subprocess.run(cmd, cwd=cwd, env=env, check=True)


def clone_libvpx(out_dir: Path) -> Path:
    """Clones the libvpx repository into the specified directory."""
    libvpx_path = out_dir / "libvpx_full"
    if libvpx_path.exists():
        logging.info("Directory %s already exists, skipping clone.", libvpx_path)
        return libvpx_path

    logging.info("Cloning libvpx into %s...", libvpx_path)
    cmd = ["git", "clone", "https://chromium.googlesource.com/webm/libvpx", str(libvpx_path)]
    run_cmd(cmd, cwd=out_dir)
    return libvpx_path


def configure_libvpx(libvpx_path: Path) -> None:
    """Configures libvpx with a wide set of features to maximize code coverage."""
    logging.info("Configuring libvpx with multiple features enabled...")
    config_cmd = [
        "./configure",
        "--enable-vp8",
        "--enable-vp9",
        "--enable-vp9-highbitdepth",
        "--enable-postproc",
        "--enable-multithread",
        "--enable-experimental",
        "--enable-spatial-svc",
    ]
    run_cmd(config_cmd, cwd=libvpx_path)


def generate_compile_commands(libvpx_path: Path) -> None:
    """Uses 'bear' to generate compile_commands.json during the build."""
    logging.info("Building libvpx and generating compile_commands.json using bear...")
    if not shutil.which("bear"):
        logging.error("The 'bear' tool is required but was not found.")
        sys.exit(1)

    num_cores = str(multiprocessing.cpu_count())
    run_cmd(["bear", "--", "make", "-j", num_cores], cwd=libvpx_path)

    if not (libvpx_path / "compile_commands.json").exists():
        logging.error("Failed to generate compile_commands.json")
        sys.exit(1)


def run_c2rust(libvpx_path: Path) -> None:
    """Runs c2rust transpile on the generated compile_commands.json."""
    logging.info("Transpiling C code to Rust using c2rust...")
    if not shutil.which("c2rust"):
        logging.error("The 'c2rust' tool is required but was not found.")
        sys.exit(1)

    cmd = ["c2rust", "transpile", "compile_commands.json", "--emit-build-files"]
    run_cmd(cmd, cwd=libvpx_path)


def count_unsafe(file_path: Path) -> Tuple[int, int]:
    """
    Counts the number of 'unsafe {' blocks and 'unsafe fn' declarations.
    Returns (unsafe_blocks_count, unsafe_functions_count).
    """
    blocks = 0
    fns = 0
    try:
        content = file_path.read_text(encoding='utf-8', errors='replace')
        blocks = len(re.findall(r"unsafe\s*\{", content))
        fns = len(re.findall(r"unsafe\s+(?:extern\s+\"[^\"]*\"\s+)?fn\s+", content))
    except OSError as e:
        logging.warning("Failed to read %s: %s", file_path, e)
    return blocks, fns


def gather_file_stats(src_dir: Path) -> List[Tuple[str, int, int]]:
    """Walks the directory to gather unsafe stats for all .rs files."""
    file_stats = []
    for path in src_dir.rglob("*.rs"):
        if "target" in path.parts or path.name in ("build.rs", "lib.rs"):
            continue

        blocks, fns = count_unsafe(path)
        if blocks > 0 or fns > 0:
            try:
                rel_path = str(path.relative_to(src_dir))
            except ValueError:
                rel_path = str(path)
            file_stats.append((rel_path, blocks, fns))
    return file_stats


def _add_top_files(report: List[str], title: str, stats: List[Tuple[str, int, int]], index: int):
    """Appends top 10 files table to the report based on the given index."""
    report.extend([title, "| File | Count |", "|---|---|"])
    sorted_stats = sorted(stats, key=lambda x: x[index], reverse=True)
    for stat in sorted_stats[:10]:
        report.append(f"| `{stat[0]}` | {stat[index]} |")
    report.append("")


def format_markdown_report(src_dir: Path, stats: List[Tuple[str, int, int]]) -> str:
    """Formats the gathered statistics into a Markdown report."""
    total_blocks = sum(s[1] for s in stats)
    total_fns = sum(s[2] for s in stats)

    report = [
        "# Unsafe Usage Analysis Report\n",
        "This report provides a programmatic analysis of the generated codebase,",
        "quantifying the scale of unsafety and technical debt required.\n",
        f"- **Target Directory:** `{src_dir.name}`",
        f"- **Total Unsafe Blocks (`unsafe {{ ... }}`):** {total_blocks}",
        f"- **Total Unsafe Functions (`unsafe fn`):** {total_fns}\n",
    ]

    _add_top_files(report, "## Top 10 Files (by Unsafe Blocks)", stats, 1)
    _add_top_files(report, "## Top 10 Files (by Unsafe Functions)", stats, 2)

    report.extend([
        "## Conclusion",
        "Files with high unsafety contain dense pointer arithmetic, manual memory,",
        "or intrinsic calls. They represent the highest risk for manual refactoring."
    ])
    return "\n".join(report)


def analyze_complexity(src_dir: Path, out_file: Path) -> None:
    """Orchestrates gathering stats and writing the Markdown report."""
    logging.info("Analyzing Rust files in %s for unsafe usage...", src_dir)
    stats = gather_file_stats(src_dir)

    logging.info("Generating Markdown report...")
    report_content = format_markdown_report(src_dir, stats)

    out_file.parent.mkdir(parents=True, exist_ok=True)
    out_file.write_text(report_content)
    logging.info("Report written successfully to %s", out_file)


def run_pipeline(out_dir: Path) -> None:
    """Executes the full pipeline simulation."""
    logging.info("Starting full libvpx transpilation pipeline simulation...")
    out_dir.mkdir(parents=True, exist_ok=True)

    libvpx_path = clone_libvpx(out_dir)
    configure_libvpx(libvpx_path)
    generate_compile_commands(libvpx_path)
    run_c2rust(libvpx_path)

    report_file = out_dir / "full_libvpx_unsafe_analysis.md"
    analyze_complexity(libvpx_path, report_file)
    logging.info("Pipeline complete.")


def parse_args() -> argparse.Namespace:
    """Parses command-line arguments."""
    parser = argparse.ArgumentParser(description="Analyze c2rust transpilation complexity.")
    parser.add_argument("--src-dir", type=Path, help="Directory to analyze (e.g. 'src/')")
    parser.add_argument("--full-pipeline", action="store_true", help="Run full pipeline")
    parser.add_argument("--out-dir", type=Path, default=Path("out"), help="Output directory")

    args = parser.parse_args()
    if not args.src_dir and not args.full_pipeline:
        parser.error("You must specify either --src-dir or --full-pipeline.")
    return args


def main():
    """Main entry point for the script."""
    args = parse_args()
    out_dir = args.out_dir.absolute()

    if args.full_pipeline:
        run_pipeline(out_dir)
    elif args.src_dir:
        src_dir = args.src_dir.absolute()
        if not src_dir.exists() or not src_dir.is_dir():
            logging.error("Source directory %s does not exist.", src_dir)
            sys.exit(1)

        report_file = out_dir / "unsafe_analysis.md"
        analyze_complexity(src_dir, report_file)


if __name__ == "__main__":
    main()
