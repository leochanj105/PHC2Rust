#!/usr/bin/env bash
set -euo pipefail

# 04_difffix.sh — Phase 3: diff-test-fix loop.
#
# For each scenario: iteratively runs differential tests, generates compact
# per-function context, analyzes failures, fixes Rust code, re-tests.
#
# Usage: ./04_difffix.sh [-v] [--scenario s1|s2|s3|s4|s5|all]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

log "PHASE 3: DIFF-TEST-FIX LOOP (model: sonnet)"

run_difffix() {
    local scenario="$1"
    local short
    short=$(short_name "$scenario")
    local scenario_dir="${EXP_DIR}/scenarios/${scenario}"
    local work_dir="${EXP_DIR}/work-${short}"
    local diffgen_dir="${work_dir}/diffgen"
    local difffix_dir="${work_dir}/difffix"
    local rust_dir="${EXP_DIR}/rust-${short}"

    [ -f "${diffgen_dir}/difftest_suite.c" ] || {
        echo "ERROR: no difftest_suite.c for ${scenario}. Run 03_diffgen.sh first." >&2
        return 1
    }

    setup_scenario_workdir "$scenario"
    export_scenario_env "$scenario"
    source "${scenario_dir}/config_overrides.sh"
    # Difffix uses its own round settings (default: 5 rounds, 2 stall limit)
    export MAX_ROUNDS="${DIFFFIX_MAX_ROUNDS:-5}"
    export STALL_LIMIT="${DIFFFIX_STALL_LIMIT:-2}"
    export MAX_GOALS="${MAX_GOALS:-5}"
    export REACT_MODE="${REACT_MODE:-0}"

    mkdir -p "${difffix_dir}"

    echo "  Running difffix loop..."
    echo "  RUST_DIR:    ${rust_dir}"
    echo "  MAX_ROUNDS:  ${MAX_ROUNDS}"
    echo "  MAX_GOALS:   ${MAX_GOALS}"

    "${EXP_DIR}/run_difffix_loop.sh" ${VERBOSE} 2>&1 | sed 's/^/    /'
}

for scenario in "${SCENARIOS[@]}"; do
    short=$(short_name "$scenario")
    should_run_scenario "$short" || continue

    echo ""
    echo "--- Difffix: ${scenario} ---"
    run_difffix "$scenario"
done

echo ""
echo "Diff-fix loop complete."
