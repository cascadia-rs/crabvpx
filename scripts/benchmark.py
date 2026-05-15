#!/usr/bin/env python3
"""
benchmark.py

A performance comparison tool for CrabVPX. It executes the test harness for both
the C Oracle and the Rust implementation, parses their suite-level performance
metrics, and calculates the speed regression or improvement with statistical
distribution analysis.

Clean code standards: methods < 20 lines, idiomatic Python, PEP8 compliant.
"""

import argparse
import math
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple


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


def calculate_stats(times: List[float]) -> Tuple[float, float]:
    """Calculates average and standard deviation (sigma) for a list of times."""
    if not times:
        return 0.0, 0.0
    avg = sum(times) / len(times)
    variance = sum((t - avg) ** 2 for t in times) / len(times)
    sigma = math.sqrt(variance)
    return avg, sigma


def run_compare(root_dir: Path, runs: int) -> str:
    """Executes the compare.py script and captures its output."""
    print(f"Running performance benchmark ({runs} runs)...")
    script_path = root_dir / "scripts" / "compare.py"
    try:
        proc = subprocess.run(
            [sys.executable, str(script_path), "-b", "--runs", str(runs)],
            capture_output=True,
            text=True,
            check=True
        )
        return proc.stdout
    except subprocess.CalledProcessError as err:
        print(f"Error: Comparison failed with exit code {err.returncode}")
        print(err.stderr)
        sys.exit(1)


def extract_metrics(output: str) -> Dict[str, Dict]:
    """Extracts performance metrics and raw times from harness output."""
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
                results.setdefault(current_decoder, {}).update(metrics)
        elif "RAW_ITERATION_TIMES_MS:" in line and current_decoder:
            times = [float(x) for x in line.split(": ")[1].split(",")]
            avg, sigma = calculate_stats(times)
            results.setdefault(current_decoder, {}).update({"times": times, "sigma": sigma})

    return results


def generate_ascii_graph(oracle_times: List[float], rust_times: List[float]) -> str:
    """Generates a text-based distribution graph comparing both decoders."""
    if not oracle_times or not rust_times:
        return ""

    all_times = oracle_times + rust_times
    min_t, max_t = min(all_times), max(all_times)
    buckets = 40
    width = (max_t - min_t) / buckets if max_t > min_t else 1.0

    def get_hist(times):
        hist = [0] * buckets
        for t in times:
            idx = min(int((t - min_t) / width), buckets - 1)
            hist[idx] += 1
        return hist

    o_hist, r_hist = get_hist(oracle_times), get_hist(rust_times)
    max_h = max(max(o_hist), max(r_hist))

    graph = ["Suite Performance Distribution (X: time, O: Oracle, R: Rust)"]
    for i in range(buckets):
        t_label = min_t + i * width
        o_bar = "O" * int(o_hist[i] / max_h * 20) if max_h > 0 else ""
        r_bar = "R" * int(r_hist[i] / max_h * 20) if max_h > 0 else ""
        graph.append(f" {t_label:>6.1f} ms | {o_bar:<20} {r_bar}")
    return "\n".join(graph)


def print_row(label: str, oracle_val: float, rust_val: float, unit: str = "ms"):
    """Prints a single row of the comparison table."""
    diff = ((rust_val - oracle_val) / oracle_val) * 100
    color = "\033[32m" if diff <= 0 else "\033[31m"
    reset = "\033[0m"

    print(f" {label:<15} | {oracle_val:>7.2f} {unit} | {rust_val:>7.2f} {unit} | "
          f"{color}{diff:>+8.2f} %{reset}")


def write_markdown_report(out_file: Path, results: Dict[str, Dict], graph: str):
    """Writes a detailed benchmark report to a Markdown file."""
    oracle, rust = results["oracle"], results["rust"]
    
    report = [
        "# Performance Benchmark Report\n",
        "| Metric | C Oracle | CrabVPX | Diff (%) |",
        "|---|---|---|---|",
        f"| Suite Average | {oracle['avg']:.2f} ms | {rust['avg']:.2f} ms | {((rust['avg']-oracle['avg'])/oracle['avg'])*100:+.2f}% |",
        f"| Suite Sigma (σ) | {oracle['sigma']:.2f} ms | {rust['sigma']:.2f} ms | {((rust['sigma']-oracle['sigma'])/oracle['sigma'])*100:+.2f}% |",
        f"| Suite Min | {oracle['min']:.2f} ms | {rust['min']:.2f} ms | {((rust['min']-oracle['min'])/oracle['min'])*100:+.2f}% |",
        f"| Suite Max | {oracle['max']:.2f} ms | {rust['max']:.2f} ms | {((rust['max']-oracle['max'])/oracle['max'])*100:+.2f}% |",
        f"| Per Frame | {oracle['ms_per_frame']:.2f} ms | {rust['ms_per_frame']:.2f} ms | {((rust['ms_per_frame']-oracle['ms_per_frame'])/oracle['ms_per_frame'])*100:+.2f}% |\n",
        "```text",
        graph,
        "```"
    ]
    
    out_file.parent.mkdir(parents=True, exist_ok=True)
    out_file.write_text("\n".join(report))
    print(f"\nDetailed report written to: {out_file}")


def display_results(results: Dict[str, Dict]):
    """Formats and displays the performance comparison table."""
    oracle, rust = results["oracle"], results["rust"]

    print("\n CrabVPX Performance Analysis vs. C Oracle")
    print("-" * 55)
    print(f" {'Metric':<15} | {'C Oracle':<10} | {'CrabVPX':<10} | {'Diff (%)':<10}")
    print("-" * 55)

    print_row("Suite Average", oracle["avg"], rust["avg"])
    print_row("Suite Sigma (σ)", oracle["sigma"], rust["sigma"])
    print_row("Suite Min", oracle["min"], rust["min"])
    print_row("Suite Max", oracle["max"], rust["max"])
    print_row("Per Frame", oracle["ms_per_frame"], rust["ms_per_frame"])

    print("-" * 55)
    print(" Note: Positive % indicates Rust regression (slower).")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Analyze performance distribution.")
    parser.add_argument("-r", "--runs", type=int, default=50, help="Number of runs.")
    args = parser.parse_args()

    root_dir = Path(__file__).resolve().parent.parent
    output = run_compare(root_dir, args.runs)
    results = extract_metrics(output)
    
    if "oracle" not in results or "rust" not in results:
        print("Error: Failed to parse required performance metrics.")
        sys.exit(1)
        
    display_results(results)
    
    graph = generate_ascii_graph(results["oracle"]["times"], results["rust"]["times"])
    report_file = root_dir / "out" / "benchmark_results.md"
    write_markdown_report(report_file, results, graph)


if __name__ == "__main__":
    main()
