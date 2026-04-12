#!/usr/bin/env bash
# judger_oss_fuzz.sh — differential testing of C vs Rust libyaml on a
# deterministic subset of the oss-fuzz corpus, with C-lib coverage measurement.
#
# Drivers:
#   13 existing judger drivers (from libyaml/judger/test-functions/)
#    7 oss-fuzz fuzzer drivers wrapped as CLIs (from /tmp/oss-fuzz-corpus/fuzzers/)
#
# Coverage: uses LLVM_PROFILE_FILE=%m.profraw so each binary appends to a
# single profraw across all its runs (online merge). Produces exactly
# ~20 profraw files regardless of how many test cases run.
#
# Usage: ./judger_oss_fuzz.sh [N]     N = subset size (default 20000)
# Runs ~N × 20 cases. At ~0.014s/case on tiny inputs + some slower cases,
# expect roughly 1-3 hours of wall-clock time.
#
# Outputs (under /tmp/judge-oss-fuzz/):
#   subset.txt                   — the N chosen input files
#   A/results.jsonl              — C stdout/stderr/exit per case
#   B/results.jsonl              — Rust stdout/stderr/exit per case
#   report.tsv                   — verdict per case
#   summary.txt                  — counts + coverage percentages
#   coverage/judger.profdata     — merged coverage data
#   coverage/export.json         — llvm-cov export (big)

set -euo pipefail

N="${1:-20000}"
CORPUS=/tmp/oss-fuzz-corpus/_merged
FUZZER_DIR=/tmp/oss-fuzz-corpus/fuzzers
OUTDIR=/tmp/judge-oss-fuzz
LIBYAML_ROOT=/home/leochanj/Desktop/libyaml
C_SRC_DIR="${LIBYAML_ROOT}/src"
C_INC_DIR="${LIBYAML_ROOT}/include"
RUST_LIB=/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-baseline-test/target/release/libyaml.a
JUDGER_DIR="${LIBYAML_ROOT}/judger"
CC=clang-21

[ -d "$CORPUS" ]     || { echo "ERROR: $CORPUS not found" >&2; exit 1; }
[ -d "$FUZZER_DIR" ] || { echo "ERROR: $FUZZER_DIR not found" >&2; exit 1; }

# Fresh output dir
rm -rf "$OUTDIR"
mkdir -p "$OUTDIR"/{A/bin,B/bin,coverage,prof,tmp_lib}

# ── Step 0: ensure Rust lib is current (cargo incremental is near-instant) ──
echo "Ensuring rust-baseline-test is built..."
(
    cd "$(dirname "$RUST_LIB")/../.."
    cargo build --release --lib 2>&1 | tail -3
)
[ -f "$RUST_LIB" ] || { echo "ERROR: $RUST_LIB not found after build" >&2; exit 1; }

# ── Step 1: deterministic subset selection (sorted filename, take first N) ──
echo "Selecting $N inputs from $(ls $CORPUS | wc -l) corpus files..."
ls "$CORPUS" | sort > "$OUTDIR/subset_full.txt"
head -n "$N" "$OUTDIR/subset_full.txt" > "$OUTDIR/subset.txt"
actual_n=$(wc -l < "$OUTDIR/subset.txt")
echo "  chose: $actual_n files"

# ── Step 2: build coverage-instrumented libyaml.a for the C side ──
echo "Building coverage-instrumented libyaml_cov.a..."
(
    cd "$OUTDIR/tmp_lib"
    for f in "$C_SRC_DIR"/*.c; do
        bn=$(basename "$f" .c)
        $CC -DHAVE_CONFIG_H=1 \
            -I"$C_INC_DIR" -I"$C_SRC_DIR" \
            -fprofile-instr-generate -fcoverage-mapping -O0 -fno-builtin -g \
            -c "$f" -o "${bn}.o" 2>/dev/null || true
    done
    ar rcs libyaml_cov.a ./*.o
    rm -f ./*.o
)
C_LIB_COV="$OUTDIR/tmp_lib/libyaml_cov.a"

# ── Step 3: compile 13 judger drivers against both libs ──
echo "Compiling 13 judger drivers..."
python3 << PYEOF
import sys
from pathlib import Path
sys.path.insert(0, '$JUDGER_DIR')
from run import compile_test_functions

out = Path('$OUTDIR')
cov_flags = ['-fprofile-instr-generate', '-fcoverage-mapping', '-O0', '-g', '-DHAVE_CONFIG_H=1']
plain_flags = ['-DHAVE_CONFIG_H=1']

a = compile_test_functions(
    bindir=out/'A'/'bin',
    lib_path=Path('$C_LIB_COV'),
    include_dir=Path('$C_INC_DIR'),
    src_dir=Path('$C_SRC_DIR'),
    cc='$CC',
    cflags=cov_flags,
)
b = compile_test_functions(
    bindir=out/'B'/'bin',
    lib_path=Path('$RUST_LIB'),
    include_dir=Path('$C_INC_DIR'),
    src_dir=Path('$C_SRC_DIR'),
    cc='$CC',
    cflags=plain_flags,
)
print(f'  A (C, coverage): {len(a)}  B (Rust): {len(b)}')
PYEOF

# ── Step 4: compile 7 oss-fuzz fuzzer drivers (wrapped as CLIs) ──
echo "Compiling 7 oss-fuzz fuzzer drivers..."
_fz_list="libyaml_parser_fuzzer libyaml_emitter_fuzzer libyaml_dumper_fuzzer libyaml_loader_fuzzer libyaml_deconstructor_fuzzer libyaml_deconstructor_alt_fuzzer libyaml_reformatter_alt_fuzzer"
for fz in $_fz_list; do
    src="${FUZZER_DIR}/${fz}.c"
    [ -f "$src" ] || { echo "  WARN: missing $src, skipping"; continue; }
    # A side: against coverage-instrumented libyaml_cov.a
    $CC -DHAVE_CONFIG_H=1 \
        -I"$C_INC_DIR" -I"$C_SRC_DIR" -I"$FUZZER_DIR" \
        -fprofile-instr-generate -fcoverage-mapping -O0 -g \
        "${FUZZER_DIR}/fuzzer_main.c" "$src" \
        "$C_LIB_COV" -lm \
        -o "$OUTDIR/A/bin/$fz" 2>"$OUTDIR/A/bin/${fz}.err" \
        && printf "  %-40s A:OK " "$fz" || printf "  %-40s A:FAIL " "$fz"
    # B side: against Rust libyaml.a
    $CC -DHAVE_CONFIG_H=1 \
        -I"$C_INC_DIR" -I"$C_SRC_DIR" -I"$FUZZER_DIR" \
        -O0 \
        "${FUZZER_DIR}/fuzzer_main.c" "$src" \
        "$RUST_LIB" -lm -lpthread -ldl \
        -o "$OUTDIR/B/bin/$fz" 2>"$OUTDIR/B/bin/${fz}.err" \
        && echo "B:OK" || echo "B:FAIL"
done

# ── Step 5: run all subset × all drivers, against both A and B ──
echo "Running $actual_n inputs × all drivers against both C and Rust..."
python3 << PYEOF
import sys, subprocess, os, json, time
from pathlib import Path

outdir = Path('$OUTDIR')
corpus = Path('$CORPUS')
prof_dir = outdir/'prof'

# Enumerate drivers from both bindirs (take the intersection — only drivers
# that built successfully on BOTH sides are usable for differential testing)
drivers_A = {p.name for p in (outdir/'A'/'bin').iterdir()
             if p.is_file() and os.access(p, os.X_OK) and p.name != 'compile.log'}
drivers_B = {p.name for p in (outdir/'B'/'bin').iterdir()
             if p.is_file() and os.access(p, os.X_OK) and p.name != 'compile.log'}
drivers = sorted(drivers_A & drivers_B)
print(f'  drivers: {len(drivers)} ({drivers})')

# Read subset
inputs = [(corpus / line.strip()) for line in open(outdir/'subset.txt') if line.strip()]
print(f'  inputs: {len(inputs)}')
print(f'  total cases: {len(inputs) * len(drivers)}')

def normalize(text, bindir, corpus):
    text = text.replace(str(bindir) + '/', '<BIN>/')
    text = text.replace(str(bindir),       '<BIN>')
    text = text.replace(str(corpus) + '/', '<INPUT>/')
    text = text.replace(str(corpus),       '<INPUT>')
    return text

def run_binary(bin_path, input_path, bindir, enable_coverage):
    env = os.environ.copy()
    if enable_coverage:
        # %m expands to the module signature — one file per binary
        env['LLVM_PROFILE_FILE'] = str(prof_dir / '%m.profraw')
    try:
        # 20s timeout: accommodates coverage-instrumented C runs on large
        # (>500KB) inputs, which are slower than the uninstrumented Rust runs.
        result = subprocess.run(
            [str(bin_path), str(input_path)],
            capture_output=True, timeout=20, env=env
        )
        out  = result.stdout.decode('utf-8', errors='replace')
        err  = result.stderr.decode('utf-8', errors='replace')
        code = result.returncode
    except subprocess.TimeoutExpired as e:
        out  = (e.stdout or b'').decode('utf-8', errors='replace')
        err  = (e.stderr or b'').decode('utf-8', errors='replace')
        code = 124
    return {'exit_code': code,
            'stdout': normalize(out, bindir, corpus),
            'stderr': normalize(err, bindir, corpus)}

bindir_A = outdir/'A'/'bin'
bindir_B = outdir/'B'/'bin'

fA = open(outdir/'A'/'results.jsonl', 'w')
fB = open(outdir/'B'/'results.jsonl', 'w')

total = 0
t_start = time.time()
for fi, inp in enumerate(inputs):
    for driver in drivers:
        a = run_binary(bindir_A/driver, inp, bindir_A, enable_coverage=True)
        b = run_binary(bindir_B/driver, inp, bindir_B, enable_coverage=False)
        fA.write(json.dumps({'function': driver, 'input': inp.name, **a}) + '\n')
        fB.write(json.dumps({'function': driver, 'input': inp.name, **b}) + '\n')
        total += 1
    if (fi+1) % 500 == 0:
        dt = time.time() - t_start
        rate = total / dt
        remaining = (len(inputs) - fi - 1) * len(drivers)
        eta = remaining / rate if rate else 0
        print(f'    {fi+1}/{len(inputs)} inputs, {total} cases, {rate:.0f} cases/s, eta {eta/60:.1f} min')

fA.close(); fB.close()
dt = time.time() - t_start
print(f'  done: {total} cases in {dt/60:.1f} min ({total/dt:.0f} cases/s)')
PYEOF

# ── Step 6: compare A vs B ──
echo "Comparing A vs B..."
python3 << PYEOF
import json
from pathlib import Path
outdir = Path('$OUTDIR')

def load(path):
    d = {}
    for line in open(path):
        r = json.loads(line)
        d[(r['function'], r['input'])] = r
    return d

A = load(outdir/'A'/'results.jsonl')
B = load(outdir/'B'/'results.jsonl')

match = diff = panic = 0
diffs = []
def is_abnormal(code):
    # subprocess.run returns negative exit for signal kills (e.g. -11 = SIGSEGV);
    # 124 is timeout (our own convention, see run_binary).
    return code < 0 or code == 124 or code >= 128
for key in sorted(A.keys() & B.keys()):
    a, b = A[key], B[key]
    if is_abnormal(a['exit_code']) or is_abnormal(b['exit_code']):
        panic += 1
        diffs.append(('panic', key[0], key[1]))
    elif (a['exit_code'], a['stdout'], a['stderr']) == (b['exit_code'], b['stdout'], b['stderr']):
        match += 1
    else:
        diff += 1
        diffs.append(('diff', key[0], key[1]))

with open(outdir/'report.tsv', 'w') as f:
    f.write('verdict\tfunction\tinput\n')
    for v, fn, inp in diffs:
        f.write(f'{v}\t{fn}\t{inp}\n')

print(f'  match: {match}')
print(f'  diff:  {diff}')
print(f'  panic: {panic}')
PYEOF

# ── Step 7: merge coverage + summarize ──
echo "Merging coverage..."
ls "$OUTDIR/prof" | head
llvm-profdata-21 merge -sparse "$OUTDIR/prof"/*.profraw -o "$OUTDIR/coverage/judger.profdata" 2>&1 | tail -3

OBJS=""
for b in "$OUTDIR/A/bin"/*; do
    [ -x "$b" ] && [ "$(basename "$b")" != "compile.log" ] && \
        [ "${b##*.}" != "err" ] && OBJS="$OBJS -object $b"
done
llvm-cov-21 export $OBJS -instr-profile="$OUTDIR/coverage/judger.profdata" \
    2>/dev/null > "$OUTDIR/coverage/export.json"

python3 << PYEOF
import json
from pathlib import Path
outdir = Path('$OUTDIR')

d = json.load(open(outdir/'coverage'/'export.json'))

# Function coverage (dedupe by (src, name), OR of count>0 across all binaries)
seen = {}
for fn in d['data'][0]['functions']:
    fs = fn.get('filenames', [])
    if not fs or '/libyaml/src/' not in fs[0]:
        continue
    src = fs[0].rsplit('/', 1)[-1]
    name = fn['name'].rsplit(':', 1)[-1]
    key = (src, name)
    count = fn.get('count', 0)
    if key not in seen or seen[key] < count:
        seen[key] = count

by_file = {}
uncov_list = {}
for (src, name), count in seen.items():
    by_file.setdefault(src, {'total': 0, 'covered': 0})
    by_file[src]['total'] += 1
    if count > 0:
        by_file[src]['covered'] += 1
    else:
        uncov_list.setdefault(src, []).append(name)

# Branch coverage (dedupe by (src, line, col), OR of true/false)
branches = {}
for finfo in d['data'][0]['files']:
    fp = finfo['filename']
    if '/libyaml/src/' not in fp:
        continue
    src = fp.rsplit('/', 1)[-1]
    for br in finfo.get('branches', []):
        key = (src, br[0], br[1])
        t, f = br[4], br[5]
        if key not in branches:
            branches[key] = [t, f]
        else:
            branches[key][0] = max(branches[key][0], t)
            branches[key][1] = max(branches[key][1], f)

per_br = {}
for (src, line, col), (t, f) in branches.items():
    per_br.setdefault(src, {'total': 0, 'cov': 0})
    per_br[src]['total'] += 2
    per_br[src]['cov'] += (1 if t > 0 else 0) + (1 if f > 0 else 0)

lines = []
lines.append('=== oss-fuzz subset × all drivers summary ===')
subset_n = sum(1 for _ in open(outdir/'subset.txt'))
lines.append(f'subset: {subset_n} inputs')
lines.append('')
lines.append('FUNCTION coverage (libyaml/src/):')
tt = tc = 0
for src in sorted(by_file):
    v = by_file[src]
    pct = 100*v['covered']/v['total']
    lines.append(f"  {src:22s} {v['covered']:3d}/{v['total']:3d}  ({pct:5.1f}%)")
    tt += v['total']; tc += v['covered']
lines.append(f"  {'TOTAL':22s} {tc:3d}/{tt:3d}  ({100*tc/tt:5.1f}%)")
lines.append('')
lines.append('uncovered functions:')
for src in sorted(uncov_list):
    for name in sorted(uncov_list[src]):
        lines.append(f'  {src}: {name}')
lines.append('')
lines.append('BRANCH coverage:')
bt = bc = 0
for src in sorted(per_br):
    v = per_br[src]
    pct = 100*v['cov']/v['total']
    lines.append(f"  {src:22s} {v['cov']:5d}/{v['total']:5d}  ({pct:5.1f}%)")
    bt += v['total']; bc += v['cov']
lines.append(f"  {'TOTAL':22s} {bc:5d}/{bt:5d}  ({100*bc/bt:5.1f}%)")

text = '\n'.join(lines)
print()
print(text)
with open(outdir/'summary.txt', 'w') as f:
    f.write(text + '\n')
PYEOF

echo ""
echo "Outputs in: $OUTDIR"
