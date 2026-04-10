#!/usr/bin/env bash
set -euo pipefail

# 02a_testgen_s1.sh — S1: naive one-shot test generation.
#
# Single AI call with minimal prompt: "generate tests for this library."
# No function list, no coverage feedback.
#
# Usage: ./02a_testgen_s1.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SCENARIO="s1_naive"
SHORT="s1"
WORK_DIR="${EXP_DIR}/work-${SHORT}"
TESTGEN_DIR="${WORK_DIR}/testgen"

log "TESTGEN S1: NAIVE ONE-SHOT (model: sonnet)"

if [ -f "${TESTGEN_DIR}/test_suite.c" ]; then
    echo "test_suite.c already exists — skipping."
    echo "To re-run: rm ${TESTGEN_DIR}/test_suite.c"
    exit 0
fi

setup_scenario_workdir "$SCENARIO"

# .claude for permissions
[ -d "${TESTGEN_DIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_DIR}/.claude"

source "${HARNESS_DIR}/scripts/ai_runner.sh"
export_scenario_env "$SCENARIO"

cd "$TESTGEN_DIR"

PROMPT_CTX="Working directory: ${TESTGEN_DIR}

File map (use these absolute paths — do NOT guess relative paths):
  C source tree = ${LIBMCS}/
  C include dir = ${C_INCLUDE_DIRS}
Output: write the complete test_suite.c to ${TESTGEN_DIR}/test_suite.c

"

echo "Running one-shot testgen..."
echo "Prompt: prompts/s1_testgen.md"

run_codegen "${PROMPT_CTX}Follow instruction in ${EXP_DIR}/prompts/s1_testgen.md." \
    "${TESTGEN_DIR}/testgen_output" "$VERBOSE"

[ -f "${TESTGEN_DIR}/test_suite.c" ] || {
    echo "ERROR: test_suite.c not generated" >&2
    exit 1
}

n_tests=$(grep -c 'printf(' "${TESTGEN_DIR}/test_suite.c" 2>/dev/null || echo '?')
echo "Done: ~${n_tests} test prints"
