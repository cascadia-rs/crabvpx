#!/bin/bash
# analyze_complexity.sh
# Analyzes the complexity of the c2rust transpiled files by counting unsafe usage.

SRC_DIR=${1:-"src"}

TOTAL_BLOCKS=$(grep -r "unsafe {" "$SRC_DIR" | wc -l | tr -d ' ')
TOTAL_FNS=$(grep -r -E "unsafe.*fn" "$SRC_DIR" | wc -l | tr -d ' ')

echo "### Unsafe Usage Analysis"
echo "A programmatic analysis of the \`c2rust\` generated codebase reveals the following scale of unsafety:"
echo ""
echo "- **Total Unsafe Blocks (\`unsafe { ... }\`):** $TOTAL_BLOCKS"
echo "- **Total Unsafe Functions (\`unsafe fn\`):** $TOTAL_FNS"
echo ""
echo "#### Top 10 Most Complex Files (by Unsafe Blocks)"
echo "| File | Unsafe Blocks |"
echo "|---|---|"
grep -rc "unsafe {" "$SRC_DIR" | awk -F: '$2 > 0 {print $2, $1}' | sort -nr | head -n 10 | while read -r count file; do
    echo "| \`$file\` | $count |"
done
echo ""
echo "#### Top 10 Most Complex Files (by Unsafe Functions)"
echo "| File | Unsafe Functions |"
echo "|---|---|"
grep -rc -E "unsafe.*fn" "$SRC_DIR" | awk -F: '$2 > 0 {print $2, $1}' | sort -nr | head -n 10 | while read -r count file; do
    echo "| \`$file\` | $count |"
done
echo ""
echo "These files represent the highest risk and effort areas for manual refactoring to safe Rust."
