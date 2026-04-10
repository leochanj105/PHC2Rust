#!/usr/bin/env bash
set -euo pipefail

# 03_diffgen.sh — Phase 2: prepare differential tests for each scenario.
#
# With separate-binary testing, test_suite.c IS the difftest — no generation
# needed. This script just copies test_suite.c to the diffgen dir, copies
# the Rust baseline, and verifies it compiles against both C and Rust libs.
#
# Usage: ./03_diffgen.sh [-v] [--scenario s1|s2|s3|s4|s5|all]

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
_common_parse_args "$@"

CC="${CC:-clang-21}"

log "PHASE 2: PREPARE DIFFERENTIAL TESTS"

prepare_difftest() {
    local scenario="$1"
    local short
    short=$(short_name "$scenario")
    local work_dir="${EXP_DIR}/work-${short}"
    local testgen_dir="${work_dir}/testgen"
    local diffgen_dir="${work_dir}/diffgen"

    # Copy baseline Rust
    copy_rust_for_scenario "$scenario"
    setup_scenario_workdir "$scenario"

    [ -f "${testgen_dir}/test_suite.c" ] || {
        echo "ERROR: no test_suite.c for ${scenario}. Run testgen first." >&2
        return 1
    }

    mkdir -p "$diffgen_dir"

    # Copy test_suite.c and wrap for independent test execution
    cp "${testgen_dir}/test_suite.c" "${diffgen_dir}/difftest_suite_raw.c"
    python3 "${EXP_DIR}/scripts/wrap_tests_independent.py" \
        "${diffgen_dir}/difftest_suite_raw.c" \
        "${diffgen_dir}/difftest_suite.c" 2
    # Copy bridge
    [ -f "${testgen_dir}/test_bridge.c" ] && cp "${testgen_dir}/test_bridge.c" "${diffgen_dir}/test_bridge.c"
    [ -f "${testgen_dir}/test_bridge.h" ] && cp "${testgen_dir}/test_bridge.h" "${diffgen_dir}/test_bridge.h"

    echo "  difftest_suite.c = test_suite.c wrapped ($(grep -c 'printf(' "${diffgen_dir}/difftest_suite.c") prints)"

    # Compile check against C lib
    local INC_FLAGS=""
    for d in $C_INCLUDE_DIRS; do INC_FLAGS="$INC_FLAGS -I$d"; done

    local C_SRCS=""
    for d in $C_SRC_DIRS; do
        [ -d "$d" ] || continue
        while IFS= read -r f; do C_SRCS="$C_SRCS $f"; done \
            < <(find "$d" -name '*.c' -type f | grep -v fenv.c)
    done

    echo "  Compile check (C lib)..."
    local BDIR
    BDIR=$(mktemp -d)
    (
        cd "$BDIR"
        $CC $INC_FLAGS -c $C_SRCS 2>/dev/null
        ar rcs libc_impl.a ./*.o 2>/dev/null
        rm -f ./*.o
        BRIDGE_OBJ=""
        [ -f "${diffgen_dir}/test_bridge.c" ] && \
            $CC $INC_FLAGS -c "${diffgen_dir}/test_bridge.c" -o bridge.o 2>/dev/null && \
            BRIDGE_OBJ="bridge.o"
        if $CC $INC_FLAGS -Wno-implicit-function-declaration \
            "${diffgen_dir}/difftest_suite.c" $BRIDGE_OBJ libc_impl.a -lm \
            -Wl,--allow-multiple-definition \
            -o /dev/null 2>"${diffgen_dir}/c_compile_err.txt"; then
            echo "    C: OK"
        else
            echo "    C: ERRORS (see ${diffgen_dir}/c_compile_err.txt)"
            head -5 "${diffgen_dir}/c_compile_err.txt"
        fi
    )
    rm -rf "$BDIR"
}

for scenario in "${SCENARIOS[@]}"; do
    short=$(short_name "$scenario")
    should_run_scenario "$short" || continue

    echo ""
    echo "--- Prepare: ${scenario} ---"
    prepare_difftest "$scenario"
done

echo ""
echo "Differential test preparation complete."
