#!/usr/bin/env bash
set -euo pipefail

# run_libfuzzer.sh — Build and run libFuzzer on libyaml locally.
#
# Replicates what oss-fuzz does: builds instrumented libyaml, compiles
# 9 fuzzer binaries, runs each with seeds + dictionary.
#
# Usage:
#   ./run_libfuzzer.sh [--time-per-fuzzer=SEC] [--outdir=DIR]
#
# Defaults:
#   --time-per-fuzzer=420   (7 min each × 9 = ~63 min total)
#   --outdir=./fuzz-run
#
# Outputs:
#   OUTDIR/bin/              — compiled fuzzer binaries
#   OUTDIR/corpus/<fuzzer>/  — discovered inputs per fuzzer
#   OUTDIR/crashes/<fuzzer>/ — crash inputs per fuzzer
#   OUTDIR/merged/           — all corpus files deduped by MD5
#   OUTDIR/log/<fuzzer>.log  — fuzzer stdout/stderr
#   OUTDIR/summary.txt       — counts and timing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORPUS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Paths
LIBYAML_SRC="${LIBYAML_SRC:-/home/leochanj/Desktop/libyaml}"
FUZZER_SRC="$CORPUS_DIR/fuzzers"
SEEDS="$CORPUS_DIR/seeds"
DICT="$SCRIPT_DIR/yaml.dict"
CC="${CC:-clang-21}"

# Parse args
TIME_PER_FUZZER=420
OUTDIR="$CORPUS_DIR/fuzz-run"

for arg in "$@"; do
    case "$arg" in
        --time-per-fuzzer=*) TIME_PER_FUZZER="${arg#*=}" ;;
        --outdir=*) OUTDIR="${arg#*=}" ;;
        --help) echo "Usage: $0 [--time-per-fuzzer=SEC] [--outdir=DIR]"; exit 0 ;;
        *) echo "Unknown arg: $arg"; exit 1 ;;
    esac
done

FUZZERS=(
    libyaml_parser_fuzzer
    libyaml_scanner_fuzzer
    libyaml_loader_fuzzer
    libyaml_emitter_fuzzer
    libyaml_dumper_fuzzer
    libyaml_deconstructor_fuzzer
    libyaml_deconstructor_alt_fuzzer
    libyaml_reformatter_fuzzer
    libyaml_reformatter_alt_fuzzer
)

echo "============================================"
echo "libFuzzer run for libyaml"
echo "============================================"
echo "LIBYAML_SRC:     $LIBYAML_SRC"
echo "FUZZER_SRC:      $FUZZER_SRC"
echo "SEEDS:           $SEEDS"
echo "DICT:            $DICT"
echo "TIME_PER_FUZZER: ${TIME_PER_FUZZER}s"
echo "OUTDIR:          $OUTDIR"
echo "CC:              $CC"
echo "FUZZERS:         ${#FUZZERS[@]}"
echo ""

# Validate
[ -d "$LIBYAML_SRC/src" ] || { echo "ERROR: $LIBYAML_SRC/src not found"; exit 1; }
[ -d "$FUZZER_SRC" ]      || { echo "ERROR: $FUZZER_SRC not found"; exit 1; }
[ -d "$SEEDS" ]           || { echo "ERROR: $SEEDS not found"; exit 1; }
[ -f "$DICT" ]            || { echo "ERROR: $DICT not found"; exit 1; }

# Fresh output
mkdir -p "$OUTDIR"/{bin,lib,corpus,crashes,log,merged}

# ── Step 1: Build instrumented libyaml ──
echo "--- Step 1: Building instrumented libyaml.a ---"
CFLAGS_FUZZ="-fsanitize=fuzzer-no-link,address -O1 -fno-omit-frame-pointer -g"

for f in "$LIBYAML_SRC"/src/*.c; do
    bn=$(basename "$f" .c)
    $CC -DHAVE_CONFIG_H=1 \
        -I"$LIBYAML_SRC/include" -I"$LIBYAML_SRC/src" \
        $CFLAGS_FUZZ \
        -c "$f" -o "$OUTDIR/lib/${bn}.o"
done
ar rcs "$OUTDIR/lib/libyaml_fuzz.a" "$OUTDIR/lib"/*.o
rm -f "$OUTDIR/lib"/*.o
echo "  libyaml_fuzz.a built ($(stat -c %s "$OUTDIR/lib/libyaml_fuzz.a") bytes)"

# ── Step 2: Compile 9 fuzzer binaries ──
echo ""
echo "--- Step 2: Compiling ${#FUZZERS[@]} fuzzer binaries ---"
LINK_FLAGS="-fsanitize=fuzzer,address -O1 -fno-omit-frame-pointer -g"
compiled=0
for fz in "${FUZZERS[@]}"; do
    src="$FUZZER_SRC/${fz}.c"
    if [ ! -f "$src" ]; then
        echo "  SKIP $fz (no source at $src)"
        continue
    fi
    if $CC -DHAVE_CONFIG_H=1 \
        -I"$LIBYAML_SRC/include" -I"$LIBYAML_SRC/src" -I"$FUZZER_SRC" \
        $LINK_FLAGS \
        "$src" "$OUTDIR/lib/libyaml_fuzz.a" \
        -lm -lstdc++ \
        -o "$OUTDIR/bin/$fz" 2>"$OUTDIR/log/${fz}_compile.log"; then
        echo "  $fz: OK"
        compiled=$((compiled + 1))
    else
        echo "  $fz: FAIL (see $OUTDIR/log/${fz}_compile.log)"
    fi
done
echo "  $compiled/${#FUZZERS[@]} compiled"

if [ "$compiled" -eq 0 ]; then
    echo "ERROR: No fuzzers compiled"
    exit 1
fi

# ── Step 3: Run each fuzzer ──
echo ""
echo "--- Step 3: Running fuzzers (${TIME_PER_FUZZER}s each) ---"
total_start=$(date +%s)

for fz in "${FUZZERS[@]}"; do
    bin="$OUTDIR/bin/$fz"
    [ -x "$bin" ] || continue

    corpus_dir="$OUTDIR/corpus/$fz"
    crash_dir="$OUTDIR/crashes/$fz"
    log_file="$OUTDIR/log/${fz}.log"
    mkdir -p "$corpus_dir" "$crash_dir"

    echo ""
    echo "  Running $fz for ${TIME_PER_FUZZER}s..."
    fz_start=$(date +%s)

    # Run libFuzzer
    # -artifact_prefix: where to save crash/timeout inputs
    # -max_total_time: time limit
    # -dict: yaml dictionary
    # First arg is corpus dir (read + write), second is seed dir (read only)
    "$bin" \
        "$corpus_dir" \
        "$SEEDS" \
        -dict="$DICT" \
        -max_total_time="$TIME_PER_FUZZER" \
        -artifact_prefix="$crash_dir/" \
        -print_final_stats=1 \
        > "$log_file" 2>&1 || true

    fz_end=$(date +%s)
    fz_elapsed=$((fz_end - fz_start))

    n_corpus=$(find "$corpus_dir" -type f | wc -l)
    n_crashes=$(find "$crash_dir" -type f | wc -l)

    # Extract final coverage from log
    final_cov=$(grep -o 'cov: [0-9]*' "$log_file" | tail -1 | awk '{print $2}')
    total_runs=$(grep -o 'stat::number_of_executed_units: [0-9]*' "$log_file" | awk '{print $2}')

    echo "  Done: ${fz_elapsed}s, corpus=$n_corpus, crashes=$n_crashes, cov=${final_cov:-?}, runs=${total_runs:-?}"
done

total_end=$(date +%s)
total_elapsed=$((total_end - total_start))

# ── Step 4: Merge corpora ──
echo ""
echo "--- Step 4: Merging corpora (MD5 dedup) ---"
for fz in "${FUZZERS[@]}"; do
    corpus_dir="$OUTDIR/corpus/$fz"
    [ -d "$corpus_dir" ] || continue
    find "$corpus_dir" -type f | while read -r f; do
        h=$(md5sum "$f" | awk '{print $1}')
        target="$OUTDIR/merged/${h:0:12}"
        [ -e "$target" ] || cp "$f" "$target"
    done
done
n_merged=$(ls "$OUTDIR/merged" | wc -l)
merged_size=$(du -sh "$OUTDIR/merged" | awk '{print $1}')
echo "  Merged: $n_merged unique files ($merged_size)"

# ── Summary ──
echo ""
echo "============================================"
echo "SUMMARY"
echo "============================================"
echo "Total time: ${total_elapsed}s ($(( total_elapsed / 60 ))m)"
echo "Merged corpus: $n_merged files ($merged_size)"

# Per-fuzzer summary
{
    echo "fuzzer	corpus	crashes	time"
    for fz in "${FUZZERS[@]}"; do
        n_corpus=$(find "$OUTDIR/corpus/$fz" -type f 2>/dev/null | wc -l)
        n_crashes=$(find "$OUTDIR/crashes/$fz" -type f 2>/dev/null | wc -l)
        echo "$fz	$n_corpus	$n_crashes	${TIME_PER_FUZZER}s"
    done
} | tee "$OUTDIR/summary.txt"

echo ""
echo "Output: $OUTDIR"
