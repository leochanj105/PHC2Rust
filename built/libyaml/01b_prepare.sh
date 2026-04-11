#!/usr/bin/env bash
set -euo pipefail

# 01b_prepare.sh — prepare transpiled Rust for differential testing.
#
# Phase between transpile (01) and testgen (02). Consumes rust-baseline/ and
# externally-provided test_bridge.{c,h,rs}, produces rust-baseline-test/ —
# a testable Rust crate that exposes internal (formerly static) functions
# via the test bridge.
#
# Steps:
#   1. Copy rust-baseline → rust-baseline-test (fresh)
#   2. Promote the internal functions referenced by test_bridge.rs to pub(crate)
#      (data-driven from the bridge file itself; no hardcoded list)
#   3. Copy test_bridge.rs into src/
#   4. Append `mod test_bridge;` to lib.rs (idempotent)
#   5. Verify `cargo build --release --lib` succeeds
#
# Usage: ./01b_prepare.sh

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

BASELINE="${EXP_DIR}/rust-baseline"
OUTPUT="${EXP_DIR}/rust-baseline-test"
BRIDGE_SRC="${EXP_DIR}/test_bridge.rs"

log "PHASE 0.5: PREPARE RUST FOR TESTING"

[ -d "$BASELINE" ] || {
    echo "ERROR: rust-baseline not found at $BASELINE" >&2
    echo "  Run ./01_transpile.sh first." >&2
    exit 1
}
[ -f "$BRIDGE_SRC" ] || {
    echo "ERROR: test_bridge.rs not found at $BRIDGE_SRC" >&2
    echo "  Place the externally-generated bridge file there before running prepare." >&2
    exit 1
}

echo "  Source:  $BASELINE"
echo "  Output:  $OUTPUT"
echo "  Bridge:  $BRIDGE_SRC"

# ── Step 1: Fresh copy ──
rm -rf "$OUTPUT"
cp -r "$BASELINE" "$OUTPUT"
rm -rf "$OUTPUT/target"

SRC="${OUTPUT}/src"

# ── Step 2: Data-driven visibility promotion ──
echo "  Promoting function visibility..."
python3 "${EXP_DIR}/scripts/promote_visibility.py" "$BRIDGE_SRC" "$SRC"

# ── Step 3: Copy bridge into src/ ──
cp "$BRIDGE_SRC" "${SRC}/test_bridge.rs"

# ── Step 4: Inject mod declaration (idempotent) ──
if ! grep -q '^mod test_bridge' "${SRC}/lib.rs"; then
    echo '' >> "${SRC}/lib.rs"
    echo 'mod test_bridge;' >> "${SRC}/lib.rs"
fi

# ── Step 5: Build verification ──
echo "  Building..."
if cargo build --release --lib --manifest-path "$OUTPUT/Cargo.toml" 2>"${OUTPUT}/build_err.txt"; then
    echo "  Build OK"
    rm -f "${OUTPUT}/build_err.txt"
else
    echo "  BUILD FAILED:" >&2
    cat "${OUTPUT}/build_err.txt" >&2
    exit 1
fi

echo ""
echo "Prepare complete: $OUTPUT"
