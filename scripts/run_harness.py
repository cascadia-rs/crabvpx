#!/usr/bin/env python3
"""
run_harness.py

The primary orchestration script for CrabVPX testing. It handles:
1. Downloading the required VP8 test vectors if they are missing.
2. Running the test harness for both the C Oracle and CrabVPX decoders.

Usage:
  ./scripts/run_harness.py [-b/--benchmark]
"""

import argparse
import subprocess
from pathlib import Path
from typing import List

# VP8 Comprehensive Test Vectors from WebM Project
VP8_VECTORS = [
    "vp80-00-comprehensive-001.ivf", "vp80-00-comprehensive-002.ivf",
    "vp80-00-comprehensive-003.ivf", "vp80-00-comprehensive-004.ivf",
    "vp80-00-comprehensive-005.ivf", "vp80-00-comprehensive-006.ivf",
    "vp80-00-comprehensive-007.ivf", "vp80-00-comprehensive-008.ivf",
    "vp80-00-comprehensive-009.ivf", "vp80-00-comprehensive-010.ivf",
    "vp80-00-comprehensive-011.ivf", "vp80-00-comprehensive-012.ivf",
    "vp80-00-comprehensive-013.ivf", "vp80-00-comprehensive-014.ivf",
    "vp80-00-comprehensive-015.ivf", "vp80-00-comprehensive-016.ivf",
    "vp80-00-comprehensive-017.ivf", "vp80-00-comprehensive-018.ivf",
    "vp80-01-intra-1400.ivf", "vp80-01-intra-1411.ivf",
    "vp80-01-intra-1416.ivf", "vp80-01-intra-1417.ivf",
    "vp80-02-inter-1402.ivf", "vp80-02-inter-1412.ivf",
    "vp80-02-inter-1418.ivf", "vp80-02-inter-1424.ivf",
    "vp80-03-segmentation-01.ivf", "vp80-03-segmentation-02.ivf",
    "vp80-03-segmentation-03.ivf", "vp80-03-segmentation-04.ivf",
    "vp80-03-segmentation-1401.ivf", "vp80-03-segmentation-1403.ivf",
    "vp80-03-segmentation-1407.ivf", "vp80-03-segmentation-1408.ivf",
    "vp80-03-segmentation-1409.ivf"
]

BASE_URL = "https://storage.googleapis.com/downloads.webmproject.org/test_data/libvpx"


def download_vectors(test_data_dir: Path):
    """Ensures all required VP8 test vectors are present in the test_data_dir."""
    test_data_dir.mkdir(parents=True, exist_ok=True)
    print(f"Checking for VP8 test vectors in {test_data_dir}...", flush=True)

    for vector in VP8_VECTORS:
        target = test_data_dir / vector
        if not target.exists():
            print(f"Downloading {vector}...", flush=True)
            url = f"{BASE_URL}/{vector}"
            subprocess.run(["curl", "-S", "-s", "-L", "-o", str(target), url], check=True)


def run_cargo(harness_dir: Path, args: List[str], features: str = None):
    """Runs a cargo command within the harness directory."""
    cmd = ["cargo", "run", "--release"]
    if features:
        cmd.extend(["--no-default-features", "--features", features])
    cmd.append("--")
    cmd.extend(args)

    subprocess.run(cmd, cwd=harness_dir, check=True)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Run the CrabVPX test harness.")
    parser.add_argument("-b", "--benchmark", action="store_true", help="Run in benchmark mode.")
    args, unknown = parser.parse_known_args()

    # Get root of repo correctly relative to this script
    root_dir = Path(__file__).resolve().parent.parent
    harness_dir = root_dir / "harness"
    test_data_dir = root_dir / "libvpx-test-data"

    download_vectors(test_data_dir)

    harness_args = ["--dir", str(test_data_dir)]
    if args.benchmark:
        harness_args.append("--benchmark")
    harness_args.extend(unknown)

    print("\n--- Testing C Oracle Decoder ---", flush=True)
    run_cargo(harness_dir, harness_args)

    print("\n--- Testing CrabVPX Rust Decoder ---", flush=True)
    run_cargo(harness_dir, harness_args, features="rust")

    print("", flush=True)
    tracker_script = root_dir / "scripts" / "count_unsafe.sh"
    if tracker_script.exists():
        subprocess.run([str(tracker_script)], check=True)


if __name__ == "__main__":
    main()
