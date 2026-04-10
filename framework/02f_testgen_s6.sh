#!/usr/bin/env bash
set -euo pipefail

# 02f_testgen_s6.sh — S6: extended branch coverage, building on S5's test suite.
# Runs 5 more rounds of branch coverage feedback starting from S5's final tests.

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SCENARIO="s6_branch_extended"
SHORT="s6"
SCENARIO_DIR="${EXP_DIR}/scenarios/${SCENARIO}"

log "TESTGEN S6: EXTENDED BRANCH COVERAGE (on top of S5, model: sonnet)"

# Check S5 completed
S5_TEST="${EXP_DIR}/work-s5/testgen/test_suite.c"
if [ ! -f "$S5_TEST" ]; then
    echo "ERROR: S5 test_suite.c not found at ${S5_TEST}" >&2
    echo "Run 02e_testgen_s5.sh first." >&2
    exit 1
fi

setup_scenario_workdir "$SCENARIO"
export_scenario_env "$SCENARIO"

source "${SCENARIO_DIR}/config_overrides.sh"
export COVERAGE_MODES MAX_ROUNDS STALL_LIMIT

[ -d "${TESTGEN_WORKDIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_WORKDIR}/.claude"

# Copy S5's test suite as starting point (if not already started)
if [ ! -f "${TESTGEN_WORKDIR}/test_suite.c" ]; then
    echo "Copying S5's test_suite.c as starting point..."
    cp "$S5_TEST" "${TESTGEN_WORKDIR}/test_suite.c"
    S5_BRIDGE="${EXP_DIR}/work-s5/testgen/test_bridge.c"
    [ -f "$S5_BRIDGE" ] && cp "$S5_BRIDGE" "${TESTGEN_WORKDIR}/test_bridge.c"
    S5_BRIDGEH="${EXP_DIR}/work-s5/testgen/test_bridge.h"
    [ -f "$S5_BRIDGEH" ] && cp "$S5_BRIDGEH" "${TESTGEN_WORKDIR}/test_bridge.h"
    s5_tests=$(grep -c 'printf(' "$S5_TEST" 2>/dev/null || echo '?')
    echo "  Starting with ${s5_tests} test prints from S5"
fi

echo "Running extended branch coverage loop..."
echo "  COVERAGE_MODES=${COVERAGE_MODES}"
echo "  MAX_ROUNDS=${MAX_ROUNDS}"
echo "  STALL_LIMIT=${STALL_LIMIT}"

"${EXP_DIR}/run_testgen_loop.sh" ${VERBOSE}

echo ""
echo "S6 testgen complete (S5 + extended branch refinement)."
[ -f "${TESTGEN_WORKDIR}/test_suite.c" ] && \
    echo "Tests: $(grep -c 'printf(' "${TESTGEN_WORKDIR}/test_suite.c" 2>/dev/null || echo '?')"
