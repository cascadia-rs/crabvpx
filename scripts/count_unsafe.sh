#!/bin/bash

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

# Count the total number of 'unsafe' keywords in the Rust source code.
# This serves as a proxy metric for how much of the codebase needs refactoring.
UNSAFE_COUNT=$(grep -ro "\bunsafe\b" ../src/ | wc -l | tr -d ' ')

echo "========================================"
echo "🦀 CrabVPX Unsafe Tracker 🦀"
echo "Current 'unsafe' occurrences: $UNSAFE_COUNT"
echo "========================================"
