#!/usr/bin/env bash
set -euo pipefail

# 01_transpile.sh — Phase 0: shared C→Rust transpilation.
#
# Produces rust-baseline/ used as starting point for all scenarios.
#
# Usage: ./01_transpile.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

log "PHASE 0: TRANSPILE (shared baseline)"

RUST_BASELINE="${EXP_DIR}/rust-baseline"
if [ -f "${RUST_BASELINE}/Cargo.toml" ]; then
    echo "Baseline Rust already exists — skipping."
    echo "  ${RUST_BASELINE}/Cargo.toml"
    echo "  To re-run: rm -rf ${RUST_BASELINE}"
    exit 0
fi

TRANS_WORK="${EXP_DIR}/work-transpile"
mkdir -p "$TRANS_WORK/prompts" "$RUST_BASELINE"

# Expand transpile prompt
expand_prompt "${EXP_DIR}/prompts/transpile.md" \
    "${TRANS_WORK}/prompts/transpile.md" "$RUST_BASELINE"

# Source AI runner
source "${HARNESS_DIR}/scripts/ai_runner.sh"

export TEST_CASE_DIR="$LIBMCS"
export RUST_DIR="$RUST_BASELINE"
export WORK_DIR="$TRANS_WORK"

cd "$HARNESS_DIR"
echo "Transpiling C -> Rust (using ${CODE_GEN_CMD})..."
echo "  Prompt:  ${TRANS_WORK}/prompts/transpile.md"
echo "  Output:  ${RUST_BASELINE}/"

run_codegen "Follow instruction in ${TRANS_WORK}/prompts/transpile.md." \
    "${TRANS_WORK}/transpile_output" "$VERBOSE"

[ -f "${RUST_BASELINE}/Cargo.toml" ] || {
    echo "ERROR: Transpilation did not produce Cargo.toml" >&2
    echo "Check: ${TRANS_WORK}/transpile_output" >&2
    exit 1
}

echo ""
echo "Transpilation complete: ${RUST_BASELINE}"
