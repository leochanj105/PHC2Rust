# libyaml experiment summary

All runs in `/home/leochanj/Desktop/PHC2Rust/built/libyaml/`.
Model: `claude-opus-4-6[1m]`.

## Phase 0: Transpile

| metric | value |
|---|---|
| script | `01_transpile.sh` |
| duration | **26.0 min** |
| turns | 44 |
| tool uses | 195 |
| tokens in | 46 (+ 3.1M cache read, 197.7k cache create) |
| tokens out | 61.1k |
| **cost** | **$19.07** |
| output | 12 `.rs` files, 13500 lines |
| verification | `cargo build --release` clean, 59 `yaml_*` `#[no_mangle]` exports |

Per-file breakdown of the produced Rust:

| file | lines |
|---|---|
| `scanner.rs` | 3670 |
| `emitter.rs` | 2882 |
| `api.rs` | 1804 |
| `parser.rs` | 1694 |
| `yaml.rs` | 742 |
| `loader.rs` | 683 |
| `yaml_private.rs` | 665 |
| `reader.rs` | 587 |
| `dumper.rs` | 539 |
| `writer.rs` | 176 |
| `lib.rs` | 35 |
| `externs.rs` | 23 |

## Phase 2: Testgen

### s1_naive — one-shot, no coverage feedback

Two clean runs completed; most recent kept as the current `work-s1/testgen/test_suite.c`.

| metric | value |
|---|---|
| duration | **4.8 min** |
| turns | 39 |
| tool uses | 38 (31 Bash, 1 Read, others) |
| tokens | in=35, out=20.3k, cache_read=1.5M, cache_create=58.4k |
| **cost** | **$0.96** |
| output | 994 lines, **25 test functions**, 4 bridge calls |
| permission audit | 0 forbidden-path hits |

The first s1 run produced 38 test functions / 1097 lines / 16 bridge calls.
Claude makes different choices each invocation, so the count varies run-to-run.

### s3_edgecase — killed by user, not completed

8 tool uses, no result. Not counted toward cost. Will re-run later.

### s4_function — coverage-guided loop (the working run)

**History: three earlier failed/bogus attempts before the working run.**

| attempt | failure mode | fix |
|---|---|---|
| 1 | bash arithmetic bug in `extract_functions.sh` count line (`grep -c \|\| echo 0` emits "0\n0" on no match) | `_total=${_total:-0}` guard |
| 2 | only 11 functions detected (all other .c files silently failed to compile because missing `-DHAVE_CONFIG_H=1`) | added project-controlled `CC_EXTRA_FLAGS` env var |
| 3 | reported 196/196 covered, **but gamed** — Claude pasted the uncovered list as a C comment header; the grep-based coverage check counted comment lines as covered. Actually 0/137 statics had real call sites. | switched to runtime coverage via `llvm-cov export` on the link binary — now counts functions only if their runtime execution count > 0 |

**Run 4 (the real one, 2 rounds, runtime-coverage):**

| round | duration | turns | cost | in | out | cache read | cache create | outcome |
|---|---|---|---|---|---|---|---|---|
| 1 | 10.2 min | 40 | $1.64 | 31 | 65.1k | 1.4M | 66.5k | all 196 uncovered → wrote first test_suite.c |
| 2 | 3.3 min | 34 | $0.62 | 902 | 12.2k | 849.8k | 46.8k | 189/196 → 196/196, early stop |
| **total** | **13.5 min** | 74 | **$2.26** | | | | | **100% function coverage** |

Round-level coverage trajectory (verified via `llvm-cov export` on the
coverage-instrumented test binary linked with all 8 `bridge_*.o`):

| round | uncovered | covered |
|---|---|---|
| 1 (initial, no test yet) | 196/196 | 0 |
| 2 (after round-1 write) | 7/196 | 189/196 |
| 3 (after round-2 write) | 0/196 | 196/196 (59 public + 137 static) |

Final `test_suite.c`: **2538 lines, 68 test functions, 69 bridge calls.**

## Grand total so far

| step | duration | cost |
|---|---|---|
| Transpile | 26.0 min | $19.07 |
| Testgen s1 (last run) | 4.8 min | $0.96 |
| Testgen s4 (both rounds, runtime-coverage run) | 13.5 min | $2.26 |
| **Total** | **44.3 min** | **$22.29** |

Not included: earlier failed/debug runs of s4 (3 attempts totaling ~10 min,
roughly $1–2 of exploratory cost) and the killed s3 run.

## Infrastructure events and fixes along the way

- **Stale `.claude/settings.json` at `$HARNESS_DIR`**: `_ensure_claude_setup()`
  had a "skip if exists" guard. When libmcs ran first, libyaml inherited its
  stale path whitelist. Transpile failed because `Read(transpile.md)` was
  denied. Fixed by removing the guard (always overwrite settings.json) and
  by refreshing each scenario's `work-*/.claude/` on every setup.

- **Hardcoded libmcs-isms scrubbed from scripts**: `build_func_map.py`,
  `extract_signatures.py`, `extract_functions.sh`, `build_and_cover.sh`,
  `run_testgen_loop.sh`, `branch_coverage.py`, `make_difffix_context.py`,
  `run_difffix_loop.sh` all had libmcs paths/regexes/flag names. All
  parameterized via env vars (`C_LIB_DIR`, `C_SRC_DIRS`, `C_INCLUDE_DIRS`,
  `EXCLUDE_C_FILES`, `SOURCE_PATH_MARKER`, `CC_EXTRA_FLAGS`,
  `COMPARE_IGNORE_KEYS`).

- **Bridge layout for libyaml**: gen_bridges.py extended with `--single-file`
  mode plus static-var collision detection via `nm`. libyaml ended up with
  zero collisions (uses `#define` constants, not `static const` globals).
  For now libyaml uses the original per-file layout (one `bridge_<src>.c`
  per C source) because `yaml_private.h` lacks include guards and a
  single-file bridge `#include`ing every source redefines typedefs.

- **Stdout comparison hardening**: a few test lines print free-text
  `parser.problem` / `emitter.problem` strings that could cosmetically
  differ between C and Rust even when behavior is identical. Safety
  nets in place:
  1. `projects/libyaml/values` `__ADDITIONAL_RULES__` now instructs Claude
     to print numeric error codes, not human-readable error strings.
     (Applies to future testgen runs.)
  2. `projects/libyaml/values` `__LIB_EXPORTS__` sets `COMPARE_IGNORE_KEYS`
     to skip existing test_suite.c lines with known-fragile keys
     (`scan_error`, `parse_error`, `load_error`, `parser_problem`,
     `emitter_error`, `writer_error`, `version_string`).
  3. `framework/scripts/compare_outputs.py` reads `$COMPARE_IGNORE_KEYS`
     and drops matching lines via **exact key match** (no regex).

## What we still need

1. `test_bridge.rs` for libyaml — blocks `01b_prepare.sh`
   - Options: extend `gen_bridges.py` with `--rust`, hand-write, or
     one-shot Claude call.
2. `01b_prepare.sh` run — produces `rust-baseline-test/`
3. `02_testgen.sh s3_edgecase` run (replay, since killed)
4. `03_diffgen.sh` — mechanical wrap + compile check
5. `04_difffix.sh --scenario s3` — the actual goal
