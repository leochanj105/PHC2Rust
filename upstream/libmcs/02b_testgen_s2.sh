#!/usr/bin/env bash
set -euo pipefail

# 02b_testgen_s2.sh — S2: explicit all-functions one-shot test generation.
#
# Like S1 but prompt instructs to cover ALL functions including internal/static.
# No function list provided — the AI must discover them by reading source.
#
# Usage: ./02b_testgen_s2.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

SCENARIO="s2_explicit"
SHORT="s2"
WORK_DIR="${EXP_DIR}/work-${SHORT}"
TESTGEN_DIR="${WORK_DIR}/testgen"

log "TESTGEN S2: EXPLICIT ALL-FUNCTIONS (model: sonnet)"

if [ -f "${TESTGEN_DIR}/test_suite.c" ]; then
    echo "test_suite.c already exists — skipping."
    echo "To re-run: rm ${TESTGEN_DIR}/test_suite.c"
    exit 0
fi

setup_scenario_workdir "$SCENARIO"

[ -d "${TESTGEN_DIR}/.claude" ] || cp -r "${HARNESS_DIR}/.claude" "${TESTGEN_DIR}/.claude"

source "${HARNESS_DIR}/scripts/ai_runner.sh"
export_scenario_env "$SCENARIO"

cd "$TESTGEN_DIR"

PROMPT_CTX="Working directory: ${TESTGEN_DIR}

File map (use these absolute paths — do NOT guess relative paths):
  C source tree = ${LIBMCS}/
  C include dir = ${C_INCLUDE_DIRS}
Output: write the complete test_suite.c to ${TESTGEN_DIR}/test_suite.c
If you create test_bridge.c, write it to ${TESTGEN_DIR}/test_bridge.c

"

echo "Running one-shot testgen..."
echo "Prompt: prompts/s2_testgen.md"

run_codegen "${PROMPT_CTX}Follow instruction in ${EXP_DIR}/prompts/s2_testgen.md." \
    "${TESTGEN_DIR}/testgen_output" "$VERBOSE"

[ -f "${TESTGEN_DIR}/test_suite.c" ] || {
    echo "ERROR: test_suite.c not generated" >&2
    exit 1
}

n_tests=$(grep -c 'printf(' "${TESTGEN_DIR}/test_suite.c" 2>/dev/null || echo '?')
echo "Done: ~${n_tests} test prints"
