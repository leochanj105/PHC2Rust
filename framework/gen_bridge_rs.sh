#!/usr/bin/env bash
set -euo pipefail

# gen_bridge_rs.sh — one-shot AI call to produce test_bridge.rs.
#
# Prerequisites:
#   - ./01_transpile.sh must have run (rust-baseline/ exists)
#   - test_bridge.h must exist at $EXP_DIR (produced by gen_bridges.py)
#
# Output:
#   $EXP_DIR/test_bridge.rs  — a Rust file that, after being copied into
#     rust-baseline-test/src/ and having `mod test_bridge;` added, builds
#     cleanly with cargo. Contains #[no_mangle] pub extern "C" wrappers
#     for every bridge_* function declared in test_bridge.h.
#
# After this finishes successfully, run ./01b_prepare.sh normally to
# produce rust-baseline-test/.
#
# Usage: ./gen_bridge_rs.sh [-v]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

export CODE_GEN_CMD="${EXP_DIR}/claude-sonnet"
export PATH="${EXP_DIR}:${PATH}"

log "GENERATE test_bridge.rs"

RUST_BASELINE="${EXP_DIR}/rust-baseline"
BRIDGE_H="${EXP_DIR}/test_bridge.h"
OUTPUT="${EXP_DIR}/test_bridge.rs"

[ -d "$RUST_BASELINE" ] || {
    echo "ERROR: $RUST_BASELINE not found. Run ./01_transpile.sh first." >&2
    exit 1
}
[ -f "$BRIDGE_H" ] || {
    echo "ERROR: $BRIDGE_H not found. Run gen_bridges.py first." >&2
    exit 1
}
if [ -f "$OUTPUT" ]; then
    echo "test_bridge.rs already exists at $OUTPUT — skipping."
    echo "  To re-run: rm $OUTPUT"
    exit 0
fi

GEN_WORK="${EXP_DIR}/work-gen-bridge-rs"
mkdir -p "$GEN_WORK/prompts"

# Expand prompt placeholders
sed -e "s|__RUST_DIR__|${RUST_BASELINE}|g" \
    -e "s|__BRIDGE_H_PATH__|${BRIDGE_H}|g" \
    -e "s|__OUTPUT_PATH__|${OUTPUT}|g" \
    "${EXP_DIR}/prompts/gen_test_bridge_rs.md" \
    > "${GEN_WORK}/prompts/gen_test_bridge_rs.md"

# Copy lightweight .claude/ for permissions (refresh to current project)
rm -rf "${GEN_WORK}/.claude"
cp -r "${HARNESS_DIR}/.claude" "${GEN_WORK}/.claude"

source "${EXP_DIR}/scripts/ai_runner.sh"

cd "$GEN_WORK"
echo "Invoking ${CODE_GEN_CMD}..."
echo "  Bridge header: ${BRIDGE_H}"
echo "  Rust baseline: ${RUST_BASELINE}"
echo "  Output file:   ${OUTPUT}"

run_codegen \
    "Follow instruction in ${GEN_WORK}/prompts/gen_test_bridge_rs.md." \
    "${GEN_WORK}/gen_output" \
    "$VERBOSE"

[ -f "$OUTPUT" ] || {
    echo "ERROR: test_bridge.rs was not produced at $OUTPUT" >&2
    echo "Check: ${GEN_WORK}/gen_output" >&2
    exit 1
}

# Sanity: count wrappers
n_wrappers=$(grep -c '#\[no_mangle\]' "$OUTPUT" 2>/dev/null || echo 0)
n_decl=$(grep -cE '^\s*\w.*bridge_\w+\s*\(' "$BRIDGE_H" 2>/dev/null || echo 0)
echo ""
echo "test_bridge.rs: $(wc -l < "$OUTPUT") lines, ${n_wrappers} #[no_mangle] wrappers (header declares ${n_decl})"
