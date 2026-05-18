#!/bin/bash

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

# Count the total number of 'unsafe' keywords in the Rust source code.
# This serves as a proxy metric for how much of the codebase needs refactoring.
UNSAFE_COUNT=$(grep -ro "\bunsafe\b" ../src/ | wc -l | tr -d ' ')
BASELINE=1343

# Calculate progress
REMOVED=$((BASELINE - UNSAFE_COUNT))
if [ $REMOVED -lt 0 ]; then REMOVED=0; fi
PERCENT=$(awk "BEGIN { printf \"%.2f\", ($REMOVED / $BASELINE) * 100 }")

TEXT="Unsafe blocks remaining: $UNSAFE_COUNT / $BASELINE ($PERCENT% removed)"
BAR_WIDTH=${#TEXT}

# Draw progress bar
FILLED=$(( (REMOVED * BAR_WIDTH) / BASELINE ))
EMPTY=$((BAR_WIDTH - FILLED))
BAR_FILLED=$(printf "%${FILLED}s" "" | tr ' ' '█')
BAR_EMPTY=$(printf "%${EMPTY}s" "" | tr ' ' '░')

echo ""
echo "🦀 CrabVPX Unsafe Tracker"
echo "╰─ $TEXT"
echo "  [${BAR_FILLED}${BAR_EMPTY}]"
echo ""
