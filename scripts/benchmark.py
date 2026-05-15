#!/usr/bin/env python3
"""
benchmark.py

A performance comparison tool for CrabVPX. It executes the test harness for both
the C Oracle and the Rust implementation, parses their suite-level performance
metrics, and calculates the speed regression or improvement.

Clean code standards: methods < 20 lines, idiomatic Python, PEP8 compliant.
"""

import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, Optional


def to_ms(val_str: str, unit: str) -> float:
    """Converts a duration string and unit into a float representing milliseconds."""
    val = float(val_str)
    if unit == 's':
        return val * 1000.0
    if unit == 'µs':
        return val / 1000.0
    return val


def parse_perf_line(line: str) -> Optional[Dict[str, float]]:
    """Parses a suite performance line into a dictionary of metrics."""
    pattern = (
        r"avg\s+([0-9.]+)(ms|µs|s),\s*"
        r"min\s+([0-9.]+)(ms|µs|s),\s*"
        r"max\s+([0-9.]+)(ms|µs|s),\s*"
        r"([0-9.]+)\s*ms/frame"
    )
    match = re.search(pattern, line)
    if not match:
        return None

    return {
        "avg": to_ms(match.group(1), match.group(2)),
        "min": to_ms(match.group(3), match.group(4)),
        "max": to_ms(match.group(5), match.group(6)),
        "ms_per_frame": float(match.group(7))
    }


def run_harness(root_dir: Path) -> str:
    """Executes the run_harness.py script and captures its output."""
    print("Running performance benchmark...")
    script_path = root_dir / "scripts" / "run_harness.py"
    try:
        proc = subprocess.run(
            [sys.executable, str(script_path), "-b"],
            capture_output=True,
            text=True,
            check=True
        )
        return proc.stdout
    except subprocess.CalledProcessError as err:
        print(f"Error: Harness failed with exit code {err.returncode}")
        print(err.stderr)
        sys.exit(1)


def extract_metrics(output: str) -> Dict[str, Dict[str, float]]:
    """Extracts performance metrics for both Oracle and Rust from harness output."""
    results = {}
    current_decoder = None

    for line in output.splitlines():
        if "Testing C Oracle Decoder" in line:
            current_decoder = "oracle"
        elif "Testing CrabVPX Rust Decoder" in line:
            current_decoder = "rust"
        elif "OVERALL_SUITE_PERF:" in line and current_decoder:
            metrics = parse_perf_line(line)
            if metrics:
                results[current_decoder] = metrics

    if "oracle" not in results or "rust" not in results:
        print("Error: Failed to parse required performance metrics.")
        sys.exit(1)

    return results


def print_row(label: str, oracle_val: float, rust_val: float):
    """Prints a single row of the comparison table."""
    diff = ((rust_val - oracle_val) / oracle_val) * 100
    color = "\033[32m" if diff <= 0 else "\033[31m"
    reset = "\033[0m"

    print(f" {label:<15} | {oracle_val:>7.2f} ms | {rust_val:>7.2f} ms | "
          f"{color}{diff:>+8.2f} %{reset}")


def display_results(results: Dict[str, Dict[str, float]]):
    """Formats and displays the performance comparison table."""
    oracle = results["oracle"]
    rust = results["rust"]

    print("\n CrabVPX Performance Analysis vs. C Oracle")
    print("-" * 52)
    print(f" {'Metric':<15} | {'C Oracle':<10} | {'CrabVPX':<10} | {'Diff (%)':<10}")
    print("-" * 52)

    print_row("Suite Average", oracle["avg"], rust["avg"])
    print_row("Suite Min", oracle["min"], rust["min"])
    print_row("Suite Max", oracle["max"], rust["max"])
    print_row("Per Frame", oracle["ms_per_frame"], rust["ms_per_frame"])

    print("-" * 52)
    print(" Note: Positive % indicates Rust regression (slower).")


def main():
    """Main entry point."""
    root_dir = Path(__file__).resolve().parent.parent
    output = run_harness(root_dir)
    results = extract_metrics(output)
    display_results(results)


if __name__ == "__main__":
    main()
