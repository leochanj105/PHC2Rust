#!/usr/bin/env bash
set -euo pipefail

# 02_testgen.sh — unified test generation for all scenarios.
#
# Replaces upstream's 02a-02f per-scenario scripts with a single dispatcher.
# Usage: ./02_testgen.sh <scenario> [-v]
#   e.g. ./02_testgen.sh s1_naive
#        ./02_testgen.sh s5_branch -v

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

SCENARIO="${1:?Scenario name required (e.g., s1_naive, s4_function)}"
shift || true
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SHORT=$(short_name "$SCENARIO")
WORK_DIR="${EXP_DIR}/work-${SHORT}"
TESTGEN_DIR="${WORK_DIR}/testgen"

# Dispatch on scenario: determine MODE, PROMPT_FILE, PREV_SHORT
case "$SCENARIO" in
    s1_naive|s2_explicit|s3_edgecase)
        MODE="oneshot"
        PROMPT_FILE="${SHORT}_testgen.md"
        PREV_SHORT=""
        ;;
    s4_function)
        MODE="loop"
        PREV_SHORT=""
        ;;
    s5_branch)
        MODE="loop"
        PREV_SHORT="s4"
        ;;
    s6_branch_extended)
        MODE="loop"
        PREV_SHORT="s5"
        ;;
    *)
        echo "Unknown scenario: $SCENARIO" >&2
        echo "Valid scenarios: ${SCENARIOS[*]}" >&2
        exit 1
        ;;
esac

log "TESTGEN ${SHORT}: ${SCENARIO}"

# One-shot scripts skip if test_suite.c already exists
if [ "$MODE" = "oneshot" ] && [ -f "${TESTGEN_DIR}/test_suite.c" ]; then
    echo "test_suite.c already exists — skipping."
    echo "To re-run: rm ${TESTGEN_DIR}/test_suite.c"
    exit 0
fi

setup_scenario_workdir "$SCENARIO"

# Predecessor: copy test_suite.c from prior scenario as starting point
if [ -n "$PREV_SHORT" ]; then
    PREV_TEST="${EXP_DIR}/work-${PREV_SHORT}/testgen/test_suite.c"
    if [ ! -f "$PREV_TEST" ]; then
        echo "ERROR: Predecessor (${PREV_SHORT}) test_suite.c not found at ${PREV_TEST}" >&2
        echo "Run ./02_testgen.sh ${PREV_SHORT}_* first." >&2
        exit 1
    fi
    if [ ! -f "${TESTGEN_DIR}/test_suite.c" ]; then
        echo "Copying ${PREV_SHORT}'s test_suite.c as starting point..."
        cp "$PREV_TEST" "${TESTGEN_DIR}/test_suite.c"
        prev_tests=$(grep -c 'printf(' "$PREV_TEST" 2>/dev/null || echo '?')
        echo "  Starting with ${prev_tests} test prints from ${PREV_SHORT}"
    fi
fi

[ -d "${TESTGEN_DIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_DIR}/.claude"

source "${EXP_DIR}/scripts/ai_runner.sh"
export_scenario_env "$SCENARIO"

cd "$TESTGEN_DIR"

if [ "$MODE" = "oneshot" ]; then
    PROMPT_CTX="Working directory: ${TESTGEN_DIR}

File map (use these absolute paths — do NOT guess relative paths):
  C source tree = ${TEST_CASE_DIR}/
  C include dir = ${C_INCLUDE_DIRS}
Output: write the complete test_suite.c to ${TESTGEN_DIR}/test_suite.c

"
    echo "Running one-shot testgen..."
    echo "Prompt: prompts/${PROMPT_FILE}"
    run_codegen "${PROMPT_CTX}Follow instruction in ${EXP_DIR}/prompts/${PROMPT_FILE}." \
        "${TESTGEN_DIR}/testgen_output" "$VERBOSE"
else
    # Loop mode: source scenario config and run loop runner
    SCENARIO_DIR="${EXP_DIR}/scenarios/${SCENARIO}"
    source "${SCENARIO_DIR}/config_overrides.sh"
    export COVERAGE_MODES MAX_ROUNDS STALL_LIMIT
    echo "Running testgen loop..."
    echo "  COVERAGE_MODES=${COVERAGE_MODES}"
    echo "  MAX_ROUNDS=${MAX_ROUNDS}"
    echo "  STALL_LIMIT=${STALL_LIMIT}"
    "${EXP_DIR}/run_testgen_loop.sh" ${VERBOSE}
fi

[ -f "${TESTGEN_DIR}/test_suite.c" ] || {
    echo "ERROR: test_suite.c not generated" >&2
    exit 1
}

# Count test_* function definitions (more reliable than grepping printf,
# which is inflated by helper functions and loops).
n_tests=$(grep -cE '^[[:space:]]*(static[[:space:]]+)?void[[:space:]]+test_\w+[[:space:]]*\(' \
    "${TESTGEN_DIR}/test_suite.c" 2>/dev/null || echo '?')
echo "${SHORT} testgen complete: ${n_tests} test functions"
