#!/bin/bash
# fnx_compare.sh - Compare FNX-on vs FNX-off rendering for a Mermaid diagram
#
# Usage: ./fnx_compare.sh input.mmd
#
# Generates two SVG files: {basename}_fnx_off.svg and {basename}_fnx_on.svg
# for visual comparison of layout differences.

set -euo pipefail

INPUT="${1:-}"
if [[ -z "$INPUT" ]]; then
    echo "Usage: $0 <input.mmd>"
    echo "Compare FNX-enabled vs FNX-disabled rendering"
    exit 1
fi

if [[ ! -f "$INPUT" ]]; then
    echo "Error: File not found: $INPUT"
    exit 1
fi

BASE=$(basename "$INPUT" .mmd)
DIR=$(dirname "$INPUT")
OUT_OFF="${DIR}/${BASE}_fnx_off.svg"
OUT_ON="${DIR}/${BASE}_fnx_on.svg"

echo "Rendering with FNX disabled..."
fm-cli render "$INPUT" --fnx-mode disabled --format svg --output "$OUT_OFF"

echo "Rendering with FNX enabled..."
fm-cli render "$INPUT" --fnx-mode enabled --format svg --output "$OUT_ON"

echo ""
echo "Generated:"
echo "  FNX-off: $OUT_OFF"
echo "  FNX-on:  $OUT_ON"

# Compare file sizes as quick proxy for differences
SIZE_OFF=$(wc -c < "$OUT_OFF")
SIZE_ON=$(wc -c < "$OUT_ON")

if [[ "$SIZE_OFF" -eq "$SIZE_ON" ]]; then
    # Check if identical
    if cmp -s "$OUT_OFF" "$OUT_ON"; then
        echo ""
        echo "Result: Outputs are byte-identical (FNX had no effect)"
    else
        echo ""
        echo "Result: Outputs differ slightly (same size, different content)"
    fi
else
    DIFF=$((SIZE_ON - SIZE_OFF))
    echo ""
    echo "Result: FNX-on is ${DIFF} bytes larger (includes centrality classes)"
fi
