#!/usr/bin/env bash
set -euo pipefail

# extract_functions.sh — extract all C function names from the compiled library.
# Uses nm on compiled .o files to get actually-compiled symbols only.
# Static functions are detected by compiling with -fno-inline and checking
# which symbols from source don't appear in nm output.
#
# Output: one function name per line. [static] prefix for internal functions.
# Run once; output is reused across all scenarios/rounds.
#
# Usage: extract_functions.sh <output_file>

OUTPUT="${1:?Usage: extract_functions.sh <output_file>}"

: "${C_LIB_DIR:?C_LIB_DIR not set (should be exported by common.sh)}"
: "${C_SRC_DIRS:?C_SRC_DIRS not set (should be exported by common.sh)}"
: "${C_INCLUDE_DIRS:?C_INCLUDE_DIRS not set (should be exported by common.sh)}"
CC="${CC:-clang-21}"

if [ -f "$OUTPUT" ]; then
    echo "Functions already extracted: ${OUTPUT} ($(wc -l < "$OUTPUT") lines)"
    exit 0
fi

echo "Extracting function list from compiled objects..."

BDIR=$(mktemp -d)
trap 'rm -rf "$BDIR"' EXIT

INC_FLAGS=""
for d in $C_INCLUDE_DIRS; do INC_FLAGS="$INC_FLAGS -I$d"; done

# Compile all .c files with coverage instrumentation, then link a minimal
# binary so llvm-cov can report every function it saw (public + static).
# This is more reliable than regex-parsing source for static declarations.
for d in $C_SRC_DIRS; do
    [ -d "$d" ] || continue
    find "$d" -name '*.c' -type f 2>/dev/null | grep -v "${EXCLUDE_C_FILES:-^$}" | sort | while read -r f; do
        bn=$(basename "$f" .c)
        dn=$(basename "$d")
        $CC $INC_FLAGS ${CC_EXTRA_FLAGS:-} -fprofile-instr-generate -fcoverage-mapping \
            -c "$f" -o "${BDIR}/${dn}_${bn}.o" 2>/dev/null || true
    done
done

# Link a trivial binary so llvm-cov has something to export from
echo 'int main(void){return 0;}' > "${BDIR}/m.c"
$CC $INC_FLAGS ${CC_EXTRA_FLAGS:-} -fprofile-instr-generate -fcoverage-mapping \
    "${BDIR}/m.c" "${BDIR}"/*.o \
    -Wl,--allow-multiple-definition -o "${BDIR}/cov_bin" 2>/dev/null || true

# Authoritative function list via llvm-cov export -empty-profile.
# The `name` field looks like:
#   "yaml_parser_initialize"        — public (no ':' separator)
#   "scanner.c:yaml_parser_scan"    — static (file-scoped)
LLVM_COV="${LLVM_COV:-llvm-cov-21}"
"${LLVM_COV}" export "${BDIR}/cov_bin" -empty-profile 2>/dev/null | python3 -c "
import json, sys
d = json.load(sys.stdin)
public, static = set(), set()
for f in d['data'][0].get('functions', []):
    name = f['name']
    if ':' in name:
        static.add(name.rsplit(':', 1)[-1])
    elif name != 'main':  # exclude the stub main() used to link cov_bin
        public.add(name)
with open('${BDIR}/public.txt', 'w') as fp:
    for n in sorted(public):
        fp.write(n + '\n')
with open('${BDIR}/static.txt', 'w') as fp:
    for n in sorted(static):
        fp.write(n + '\n')
" || {
    # Fallback: if llvm-cov failed, at least emit public symbols via nm
    nm "${BDIR}"/*.o 2>/dev/null | grep " T " | awk '{print $3}' | sort -u > "${BDIR}/public.txt"
    : > "${BDIR}/static.txt"
}

# Generate output
{
    echo "# C functions in the target library (compiled symbols)"
    echo "# [static] prefix = internal linkage (needs bridge for testing)"
    echo "# Generated from: nm on compiled .o files"
    echo ""
    while read -r func; do
        echo "$func"
    done < "${BDIR}/public.txt"
    while read -r func; do
        echo "[static] $func"
    done < "${BDIR}/static.txt"
} | sort -t' ' -k2 > "$OUTPUT"

# grep -c outputs "0" when no matches AND exits 1; `|| true` keeps the "0"
# without adding a second one (which would break arithmetic).
_total=$(grep -c -v '^#\|^$' "$OUTPUT" || true)
_static=$(grep -c '^\[static\]' "$OUTPUT" || true)
_total=${_total:-0}
_static=${_static:-0}
_public=$(( _total - _static ))
echo "  ${_total} functions (${_public} public, ${_static} static) -> ${OUTPUT}"
