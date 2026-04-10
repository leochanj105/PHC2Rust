#!/usr/bin/env bash
set -euo pipefail

# 05_judge.sh — Phase 4: judger evaluation.
#
# Runs judger_v2 (glibc + core-math worst-case tests) on all 5 Rust codebases:
# baseline (raw transpilation) + s1 through s4 (fixed).
#
# Usage: ./05_judge.sh [-v] [--scenario s1|s2|s3|s4|all]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

log "PHASE 4: JUDGER EVALUATION"

mkdir -p "${EXP_DIR}/results"

run_judger() {
    local label="$1"
    local rust_dir="$2"
    local report="${EXP_DIR}/results/judger_${label}.txt"

    echo ""
    echo "--- Judging: ${label} ---"
    echo "  Rust:   ${rust_dir}"
    echo "  Report: ${report}"

    if [ ! -f "${rust_dir}/Cargo.toml" ]; then
        echo "  SKIP: no Cargo.toml in ${rust_dir}"
        echo "SKIP: no Cargo.toml" > "$report"
        return
    fi

    export RUST_DIR="$rust_dir"
    export TEST_CASE_DIR="$LIBMCS"
    export C_SRC_DIRS C_INCLUDE_DIRS

    local rc=0
    bash "${EXP_DIR}/judger_wrapper.sh" "$report" || rc=$?

    if [ "$rc" -eq 0 ]; then
        echo "  RESULT: BITWISE IDENTICAL"
    else
        echo "  RESULT: DIVERGENCES DETECTED"
    fi
}

# Always judge baseline
run_judger "baseline" "${EXP_DIR}/rust-baseline"

# Judge each scenario
for scenario in "${SCENARIOS[@]}"; do
    short=$(short_name "$scenario")
    should_run_scenario "$short" || continue
    run_judger "$short" "${EXP_DIR}/rust-${short}"
done

# Summary table
echo ""
log "RESULTS SUMMARY"
printf "%-12s  %-20s  %s\n" "Scenario" "Description" "Divergences"
printf "%-12s  %-20s  %s\n" "--------" "-----------" "-----------"

for label in baseline s1 s2 s3 s4 s5; do
    report="${EXP_DIR}/results/judger_${label}.txt"
    [ -f "$report" ] || continue
    case "$label" in
        baseline) desc="raw transpilation" ;;
        s1) desc="naive one-shot" ;;
        s2) desc="explicit all-funcs" ;;
        s3) desc="all-funcs + edge cases" ;;
        s4) desc="func coverage loop" ;;
        s5) desc="func+branch loop" ;;
    esac
    divs="N/A"
    if grep -q "BITWISE IDENTICAL" "$report" 2>/dev/null; then
        divs="0"
    elif grep -qP '\d+ BITWISE DIVERGENCES' "$report" 2>/dev/null; then
        divs=$(grep -oP '\d+(?= BITWISE DIVERGENCES)' "$report")
    elif grep -q "SKIP" "$report" 2>/dev/null; then
        divs="SKIP"
    fi
    printf "%-12s  %-20s  %s\n" "$label" "$desc" "$divs"
done
