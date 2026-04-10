#!/usr/bin/env bash
set -euo pipefail

# 02d_testgen_s4.sh — S4: multi-round with function coverage feedback.
#
# Coverage-guided loop (no strategy step):
#   extract functions → check coverage → generate tests → repeat
# COVERAGE_MODES=function, MAX_ROUNDS=5
#
# Usage: ./02d_testgen_s4.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SCENARIO="s4_function"
SHORT="s4"
SCENARIO_DIR="${EXP_DIR}/scenarios/${SCENARIO}"

log "TESTGEN S4: FUNCTION COVERAGE LOOP (model: sonnet)"

setup_scenario_workdir "$SCENARIO"
export_scenario_env "$SCENARIO"

source "${SCENARIO_DIR}/config_overrides.sh"
export COVERAGE_MODES MAX_ROUNDS STALL_LIMIT

[ -d "${TESTGEN_WORKDIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_WORKDIR}/.claude"

echo "Running testgen loop..."
echo "  COVERAGE_MODES=${COVERAGE_MODES}"
echo "  MAX_ROUNDS=${MAX_ROUNDS}"
echo "  STALL_LIMIT=${STALL_LIMIT}"

"${EXP_DIR}/run_testgen_loop.sh" ${VERBOSE}

echo ""
echo "S4 testgen complete."
[ -f "${TESTGEN_WORKDIR}/test_suite.c" ] && \
    echo "Tests: $(grep -c 'printf(' "${TESTGEN_WORKDIR}/test_suite.c" 2>/dev/null || echo '?')"
