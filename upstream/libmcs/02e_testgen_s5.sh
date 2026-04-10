#!/usr/bin/env bash
set -euo pipefail

# 02e_testgen_s5.sh — S5: branch coverage feedback, building on S4's test suite.
#
# Copies S4's final test_suite.c as starting point, then runs a coverage-guided
# loop targeting uncovered BRANCHES only (not functions — S4 already covered those).
# COVERAGE_MODES=branch, MAX_ROUNDS=5
#
# Must run AFTER 02d_testgen_s4.sh.
#
# Usage: ./02e_testgen_s5.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SCENARIO="s5_branch"
SHORT="s5"
SCENARIO_DIR="${EXP_DIR}/scenarios/${SCENARIO}"

log "TESTGEN S5: BRANCH COVERAGE (on top of S4, model: sonnet)"

# ── Check S4 completed ──
S4_TEST="${EXP_DIR}/work-s4/testgen/test_suite.c"
if [ ! -f "$S4_TEST" ]; then
    echo "ERROR: S4 test_suite.c not found at ${S4_TEST}" >&2
    echo "Run 02d_testgen_s4.sh first." >&2
    exit 1
fi

setup_scenario_workdir "$SCENARIO"
export_scenario_env "$SCENARIO"

source "${SCENARIO_DIR}/config_overrides.sh"
export COVERAGE_MODES MAX_ROUNDS STALL_LIMIT

[ -d "${TESTGEN_WORKDIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_WORKDIR}/.claude"

# ── Copy S4's test suite as starting point (if not already started) ──
if [ ! -f "${TESTGEN_WORKDIR}/test_suite.c" ]; then
    echo "Copying S4's test_suite.c as starting point..."
    cp "$S4_TEST" "${TESTGEN_WORKDIR}/test_suite.c"
    # Also copy bridge if S4 produced one
    S4_BRIDGE="${EXP_DIR}/work-s4/testgen/test_bridge.c"
    [ -f "$S4_BRIDGE" ] && cp "$S4_BRIDGE" "${TESTGEN_WORKDIR}/test_bridge.c"
    s4_tests=$(grep -c 'printf(' "$S4_TEST" 2>/dev/null || echo '?')
    echo "  Starting with ${s4_tests} test prints from S4"
fi

echo "Running branch coverage loop..."
echo "  COVERAGE_MODES=${COVERAGE_MODES}"
echo "  MAX_ROUNDS=${MAX_ROUNDS}"
echo "  STALL_LIMIT=${STALL_LIMIT}"

"${EXP_DIR}/run_testgen_loop.sh" ${VERBOSE}

echo ""
echo "S5 testgen complete (S4 + branch refinement)."
[ -f "${TESTGEN_WORKDIR}/test_suite.c" ] && \
    echo "Tests: $(grep -c 'printf(' "${TESTGEN_WORKDIR}/test_suite.c" 2>/dev/null || echo '?')"
