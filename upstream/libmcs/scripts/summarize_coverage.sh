#!/usr/bin/env bash
set -euo pipefail

# summarize_coverage.sh — compute branch coverage from feedback files.
# Standalone version (no harness config.sh dependency).
#
# Reads feedback files from WORKDIR/feedback/ (produced by build_and_cover.sh)
# and branches.md (produced by extract_branches.sh).
#
# Output: WORKDIR/uncovered.md — uncovered branch conditions.
#
# Usage: summarize_coverage.sh <workdir>

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[ $# -eq 1 ] || { echo "Usage: $0 <workdir>"; exit 1; }
WORKDIR="$1"

BRANCHES_FILE="${WORKDIR}/branches.md"
UNCOVERED_FILE="${WORKDIR}/uncovered.md"
FEEDBACK_DIR="${WORKDIR}/feedback"
GET_UNCOVERED="${SCRIPT_DIR}/get_uncovered.sh"

[ -f "$BRANCHES_FILE" ] || { echo "Error: branches.md not found in $WORKDIR"; exit 1; }
[ -f "$GET_UNCOVERED" ] || { echo "Error: get_uncovered.sh not found in $SCRIPT_DIR"; exit 1; }

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Build all possible conditions from branches.md: each branch × {True, False}
grep -v "^#" "$BRANCHES_FILE" | grep -v "^[[:space:]]*$" \
    | awk '{print $0 ": True"; print $0 ": False"}' \
    > "${TMPDIR}/all_conditions.txt"
TOTAL=$(wc -l < "${TMPDIR}/all_conditions.txt")

if [ "$TOTAL" -eq 0 ]; then
    echo "PANIC: branches.md has 0 entries — nothing to track" >&2
    exit 1
fi

COVERED_CONDITIONS="${TMPDIR}/covered_conditions.txt"
touch "$COVERED_CONDITIONS"

FEEDBACK_COUNT=0
mkdir -p "$FEEDBACK_DIR"
for FEEDBACK in "${FEEDBACK_DIR}"/*_feedback; do
    [ -f "$FEEDBACK" ] || continue

    # Skip error/no-coverage feedback files
    if head -1 "$FEEDBACK" | grep -qE "^(COMPILE_ERROR|NO_COVERAGE)"; then
        echo "  Skipping $(basename "$FEEDBACK"): $(head -1 "$FEEDBACK")"
        continue
    fi

    FEEDBACK_COUNT=$((FEEDBACK_COUNT + 1))
    echo "Reading: $(basename "$FEEDBACK")"

    UNCOV_TMP="${TMPDIR}/uncov_tmp.txt"
    ALL_IN_FEEDBACK="${TMPDIR}/all_in_feedback.txt"

    # Get uncovered conditions for this feedback
    "${GET_UNCOVERED}" "$FEEDBACK" "$UNCOV_TMP" > /dev/null

    # Get ALL conditions present in this feedback (both sides of every branch)
    awk '
        /^\/[^ ]+\.c:$/ {
            n = split($0, a, "/")
            fname = substr(a[n], 1, length(a[n]) - 1)
            next
        }
        /Branch \([0-9]+:[0-9]+\):/ {
            match($0, /Branch \([0-9]+:[0-9]+\)/)
            b = fname ", " substr($0, RSTART, RLENGTH)
            print b ": True"
            print b ": False"
        }
    ' "$FEEDBACK" > "$ALL_IN_FEEDBACK"

    # Covered in this feedback = all conditions present MINUS uncovered ones
    set +e
    grep -Fxvf "$UNCOV_TMP" "$ALL_IN_FEEDBACK" >> "$COVERED_CONDITIONS"
    _rc=$?
    set -e
    [ "$_rc" -gt 1 ] && { echo "Error: grep failed on $(basename "$FEEDBACK")"; exit 1; }
done

if [ "$FEEDBACK_COUNT" -eq 0 ]; then
    echo ""
    echo "PANIC: no usable feedback files found under $FEEDBACK_DIR" >&2
    echo "  All feedback files were COMPILE_ERROR or NO_COVERAGE." >&2
    exit 1
fi

# Global covered = deduplicated union of per-feedback covered conditions
sort -u "$COVERED_CONDITIONS" > "${TMPDIR}/covered_unique.txt"

# Global uncovered = all_conditions not in covered
set +e
grep -Fxvf "${TMPDIR}/covered_unique.txt" "${TMPDIR}/all_conditions.txt" \
    > "${TMPDIR}/uncovered.txt"
_rc=$?
set -e
[ "$_rc" -gt 1 ] && { echo "Error: grep failed computing uncovered conditions"; exit 1; }
[ "$_rc" -eq 1 ] && : > "${TMPDIR}/uncovered.txt"

COVERED=$(wc -l < "${TMPDIR}/covered_unique.txt")
UNCOVERED=$(wc -l < "${TMPDIR}/uncovered.txt")

# Sanity: if 0 covered out of many total, something is broken
if [ "$COVERED" -eq 0 ] && [ "$TOTAL" -gt 10 ]; then
    echo "PANIC: 0 branches covered out of ${TOTAL} total." >&2
    echo "  Feedback files may not contain branch data." >&2
    echo "  Check LLVM tool versions match: CC=${CC:-clang-21}, LLVM_COV=${LLVM_COV:-llvm-cov-21}" >&2
    exit 1
fi

# Write uncovered.md
{
    echo "# Uncovered branch conditions"
    echo "# Total: ${TOTAL} | Covered: ${COVERED} | Uncovered: ${UNCOVERED}"
    echo ""
    cat "${TMPDIR}/uncovered.txt"
} > "$UNCOVERED_FILE"

# Coverage rate
if [ "$TOTAL" -gt 0 ]; then
    RATE=$(awk "BEGIN { printf \"%.1f\", ($COVERED / $TOTAL) * 100 }")
else
    RATE="N/A"
fi

echo ""
echo "=== Branch Coverage Summary ==="
echo "Total conditions   : $TOTAL"
echo "Covered            : $COVERED"
echo "Uncovered          : $UNCOVERED"
echo "Coverage rate      : ${RATE}%"
echo "Written to: $UNCOVERED_FILE"
