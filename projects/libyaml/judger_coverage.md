# libyaml differential + coverage results

## Run B: oss-fuzz corpus × 20 drivers, N=15000 (scaled)

**Setup:** 15,000 deterministic inputs (sorted filename order) from merged
oss-fuzz corpus (7 fuzzer corpora, deduplicated to 65,614 unique files) ×
20 drivers (13 judger drivers + 7 oss-fuzz fuzzer harnesses wrapped as CLIs).

**Runtime:** 39.4 min wall, 127 cases/s. No AI calls.

**Verdict distribution (300,000 total cases):**

| category | count | % |
|---|---|---|
| match (C == Rust, same exit, same stdout, same stderr) | 288,856 | 96.29% |
| both crash, same signal | 11,141 | 3.71% |
| both crash, different signal (same bug, glibc memory-check race) | 3 | 0.001% |
| **only C crashed, Rust ran** | **0** | **0%** |
| **only Rust crashed, C ran** | **0** | **0%** |
| **real behavioral diff** | **0** | **0%** |

All panics are in drivers that `assert()` on malformed input
(`run-emitter-test-suite: 10,379`, `run-dumper`: 388, `run-emitter`: 374,
`libyaml_dumper_fuzzer`: 3). Both sides fail identically.

**Coverage of libyaml/src/ (run B, aggregated):**

| file | function | branch |
|---|---|---|
| api.c | 46/53 (86.8%) | 331/434 (76.3%) |
| dumper.c | 11/11 (100%) | 85/96 (88.5%) |
| emitter.c | 47/47 (100%) | 720/886 (81.3%) |
| loader.c | 14/14 (100%) | 123/158 (77.8%) |
| parser.c | 23/24 (95.8%) | 369/412 (89.6%) |
| reader.c | 4/4 (100%) | 158/170 (92.9%) |
| scanner.c | 41/41 (100%) | 571/704 (81.1%) |
| writer.c | 2/2 (100%) | 33/40 (82.5%) |
| yaml_private.h | — | 119/132 (90.2%) |
| **TOTAL** | **188/196 (95.9%)** | **2509/3032 (82.8%)** |

**Remaining 8 uncovered functions** (all configuration setters no fuzzer
calls, plus one file-write handler the fuzzers bypass):

- api.c: `yaml_emitter_set_break`, `yaml_emitter_set_encoding`,
  `yaml_emitter_set_indent`, `yaml_emitter_set_width`,
  `yaml_parser_set_encoding`, `yaml_parser_set_input`, `yaml_file_write_handler`
- parser.c: `yaml_set_max_nest_level`

## Delta over yaml-test-suite baseline

| metric | yaml-test-suite | oss-fuzz 15k | delta |
|---|---|---|---|
| Cases | 4,424 | 300,000 | ×68 |
| Function coverage | 184/196 (93.9%) | 188/196 (95.9%) | +4 fns, +2.0pp |
| Branch coverage | 2156/3032 (71.1%) | 2509/3032 (82.8%) | **+353 branches, +11.7pp** |
| Real Rust/C divergences | 0 | 0 | — |

---

## Run A: judger yaml-test-suite baseline (historical)

Coverage of `libyaml/src/*.c` when exercised by the full judger test corpus
(13 test drivers × yaml-test-suite, 4424 cases total).

**Measurement method:**
1. Compile all `libyaml/src/*.c` with `-fprofile-instr-generate -fcoverage-mapping`
2. Compile all 13 judger test drivers against the instrumented static lib
3. Run all 4424 test cases, one subprocess each, unique `LLVM_PROFILE_FILE` per case
4. Merge 4424 `.profraw` → `judger.profdata` via `llvm-profdata-21 merge -sparse`
5. `llvm-cov-21 export -object <each driver binary> -instr-profile=judger.profdata`
6. Aggregate/dedupe function and branch hits across all drivers

## Summary

| file | function cov | branch cov |
|---|---|---|
| `api.c` | 46/53 (86.8%) | 305/434 (70.3%) |
| `dumper.c` | 11/11 (100%) | 72/96 (75.0%) |
| `emitter.c` | 46/47 (97.9%) | 636/886 (71.8%) |
| `loader.c` | 13/14 (92.9%) | 114/158 (72.2%) |
| `parser.c` | 22/24 (91.7%) | 326/412 (79.1%) |
| `reader.c` | 4/4 (100%) | 145/170 (85.3%) |
| `scanner.c` | 41/41 (100%) | 461/704 (65.5%) |
| `writer.c` | 1/2 (50%) | 10/40 (25.0%) |
| `yaml_private.h` (inlines) | — | 87/132 (65.9%) |
| **TOTAL** | **184/196 (93.9%)** | **2156/3032 (71.1%)** |

## 12 uncovered functions

Configuration setters the judger drivers never call (they use defaults):

- `api.c`: `yaml_emitter_set_break`, `yaml_emitter_set_encoding`,
  `yaml_emitter_set_indent`, `yaml_emitter_set_output`,
  `yaml_emitter_set_width`, `yaml_parser_set_encoding`,
  `yaml_parser_set_input`
- `emitter.c`: `yaml_emitter_write_bom`
- `loader.c`: `yaml_parser_set_composer_error`
- `parser.c`: `yaml_maximum_level_reached`, `yaml_set_max_nest_level`
- `writer.c`: `yaml_emitter_set_writer_error`

## Interpretation

- **93.9% function coverage** is high, but the 12 uncovered functions include
  7 configuration setters. None of the drivers change encoding, indent,
  line break, width, output handler, or stream input — they all use defaults.
- **71.1% branch coverage** leaves ~29% of conditional paths untested.
  Error paths, rarely-triggered configurations, and OOM handling account
  for most of the gap.
- **`writer.c`** is notably weak (25% branch): only the common path through
  one function is exercised, error handling is mostly dead.

## What a 4424/4424 judger pass actually proves

- Claude's transpilation is functionally equivalent to C libyaml for
  everything the judger exercises.
- Approximately **184 of 196** public-and-static functions behave identically
  under the default configuration.
- Approximately **71%** of library branches produce bitwise-identical output.
- The **remaining ~29% of branches** are unverified — any transpile bug
  there is invisible to this judger.

Two probes confirmed this gap was real (see history in `report.md`):
- `return 1 → return 100` in `yaml_emitter_initialize` success path —
  not caught (callers treat any nonzero as success).
- `return 0 → return 100` in `yaml_emitter_initialize` OOM failure path —
  not caught (OOM never occurs under the test corpus).

## Command to reproduce

```bash
mkdir -p /tmp/cov-judger && cd /tmp/cov-judger

# 1. instrumented static lib
for f in /home/leochanj/Desktop/libyaml/src/*.c; do
  clang-21 -DHAVE_CONFIG_H=1 \
    -I/home/leochanj/Desktop/libyaml/include \
    -I/home/leochanj/Desktop/libyaml/src \
    -fprofile-instr-generate -fcoverage-mapping -O0 -fno-builtin -g \
    -c "$f" -o "$(basename "$f" .c).o"
done
ar rcs libyaml_cov.a *.o && rm *.o

# 2. compile judger drivers against it
python3 -c "
import sys; sys.path.insert(0, '/home/leochanj/Desktop/libyaml/judger')
from run import compile_test_functions
from pathlib import Path
compile_test_functions(
    bindir=Path('/tmp/cov-judger/bin'),
    lib_path=Path('/tmp/cov-judger/libyaml_cov.a'),
    include_dir=Path('/home/leochanj/Desktop/libyaml/include'),
    src_dir=Path('/home/leochanj/Desktop/libyaml/src'),
    cc='clang-21',
    cflags=['-fprofile-instr-generate', '-fcoverage-mapping', '-O0', '-g', '-DHAVE_CONFIG_H=1'],
)"

# 3. run all cases with unique profraw per case  (~4424 runs, ~1 min)
mkdir -p prof
python3 << 'PY'
import sys, subprocess, os
from pathlib import Path
sys.path.insert(0, '/home/leochanj/Desktop/libyaml/judger')
from run import enumerate_test_cases
for i, c in enumerate(enumerate_test_cases()):
    bp = Path('/tmp/cov-judger/bin') / c.function
    if not bp.exists(): continue
    env = os.environ.copy()
    env['LLVM_PROFILE_FILE'] = f'/tmp/cov-judger/prof/case_{i:05d}.profraw'
    if c.input_mode in ('arg','event'):
        cmd, stdin_data = [str(bp), c.input_path], None
    elif c.input_mode == 'stdin':
        cmd, stdin_data = [str(bp)], Path(c.input_path).read_bytes()
    else:
        cmd, stdin_data = [str(bp)], None
    try:
        subprocess.run(cmd, input=stdin_data, capture_output=True, timeout=10, env=env)
    except subprocess.TimeoutExpired: pass
PY

# 4. merge + export
llvm-profdata-21 merge -sparse prof/*.profraw -o judger.profdata
OBJS=""; for b in bin/*; do [ -x "$b" ] && OBJS="$OBJS -object $b"; done
llvm-cov-21 export $OBJS -instr-profile=judger.profdata > export_all.json
```
