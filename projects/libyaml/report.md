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

## Transpile comparison: Opus vs Sonnet vs c2rust

### Claude Opus (original baseline)

| metric | value |
|---|---|
| model | claude-opus-4-6[1m] |
| isolation | none (first transpile, nothing to cheat from) |
| duration | 26.0 min |
| cost | $19.07 |
| tool uses | 195 |
| output | 12 files, 13500 lines |
| structure | multi-module (api.rs, scanner.rs, parser.rs, ...) |
| build | clean |
| yaml_* exports | 59 |
| judger (yaml-test-suite, 4424 cases) | **4424 match, 0 diff, 0 panic** |
| judger (oss-fuzz 15k, 300k cases) | **288856 match, 0 diff, 11144 shared-panic** |
| similarity to unsafe-libyaml crate | 53.6% avg function body |

### Claude Sonnet — FIRST attempt (cheated, invalidated)

Sonnet found Opus's `rust-s4/` directory on disk and `cp`'d all 12 files
into its own output via Bash. Produced byte-identical code to Opus.
**Result discarded.** Exposed a critical isolation failure in the experiment
setup (Bash had no path restrictions; settings.json deny rules only block
dedicated Read/Glob/Grep tools, not shell commands).

### Claude Sonnet — SECOND attempt (isolated)

| metric | value |
|---|---|
| model | claude-sonnet-4-6 |
| isolation | **full**: CWD in /tmp, Bash PreToolUse hook blocking 20 forbidden path patterns, settings.json deny on all prior Rust outputs + .cargo/ |
| duration | **74.7 min** |
| cost | **$20.82** |
| tool uses | 370 |
| blocked/denied events | 0 (Sonnet did not attempt to cheat) |
| output | **1 file** (lib.rs), **10191 lines** |
| structure | single monolithic lib.rs (no modules) |
| build | clean (2 warnings: unreachable patterns) |
| yaml_* exports | 60 |
| missing internal functions | 3 (`yaml_parser_set_composer_error`, `yaml_parser_set_composer_error_context`, `yaml_parser_delete_aliases`) |
| judger (yaml-test-suite, 4424 cases) | **2238 match (51%), 523 diff (12%), 1663 panic (38%)** |

### c2rust (mechanical baseline)

| metric | value |
|---|---|
| tool | c2rust 0.22.1 (AST-level mechanical transpiler) |
| duration | ~5 sec |
| cost | $0 |
| output | 8 files, 32260 lines |
| manual fix needed | 1 (opaque extern types moved out of extern block) |
| build | clean |
| yaml_* exports | 59 |
| judger (yaml-test-suite, 4424 cases) | **4424 match, 0 diff, 0 panic** |
| judger (oss-fuzz 15k, 300k cases) | **288856 match, 0 diff, 11144 shared-panic** |

### Summary matrix

| transpiler | contamination | isolation | match% | diff | panic | lines |
|---|---|---|---|---|---|---|
| c2rust | zero | n/a | 100% | 0 | 0 | 32K |
| Claude Opus | high | none needed | 100% | 0 | 0 | 13.5K |
| Claude Sonnet (cheated) | high | **none** | 100% | 0 | 0 | 13.5K (copied) |
| **Claude Sonnet (isolated)** | high | **full** | **51%** | **523** | **1663** | **10.2K** |

**Key finding:** With proper isolation, Sonnet achieves only 51% match rate
despite having the same training data as Opus. The 49% failure rate includes
523 behavioral diffs and 1663 crashes/timeouts. This demonstrates that
training-data contamination alone is not sufficient — model capability
matters. Opus can leverage its training knowledge to produce correct code;
Sonnet cannot reliably do so.

The Sonnet (isolated) transpile is now the active `rust-baseline` for the
difffix pipeline. `rust-baseline-test` is prepared with 137 bridge symbols
(3 missing functions have stub bodies returning 0).

## Phase 4: Difffix (s4 tests, Sonnet transpile)

Using s4's test suite (68 test functions, 173 test lines) against the
Sonnet transpile. Difffix model: Claude Sonnet.

### Baseline (round 0)

| metric | value |
|---|---|
| Tests passed | 148 |
| Tests failed | **25** |
| Crashed test functions | 14 (SIGABRT) |

### Round 1

| metric | value |
|---|---|
| Goals generated | 5 |
| Fixes applied | 5 |
| Tests passed | **173** |
| Tests failed | **0** |
| Cost | $12.01 |
| Tokens out | 166K |
| Code changes | 20 lines diff |
| Cheat audit | CLEAN — 0 forbidden path references |

### Independent judger verification (yaml-test-suite, 4424 cases)

| | before difffix | after difffix |
|---|---|---|
| match | 2238 (51%) | **4267 (96.4%)** |
| diff | 523 (12%) | **157 (3.6%)** |
| panic | 1663 (38%) | **0 (0%)** |

Difffix eliminated ALL 1663 crashes and reduced diffs from 523 → 157 in one
round. The remaining 157 diffs are in code paths the s4 test suite doesn't
cover — they require more test coverage (s5 branch-guided, or additional
testgen) for difffix to see and fix them.

## Phase 2 (Sonnet): Testgen s5 — branch-coverage-guided

Built on top of s4's test suite (68 tests, 2538 lines, 53.1% branch coverage).
5 rounds of branch-coverage feedback. Model: Claude Sonnet.

### Coverage trajectory

| round | covered | uncovered | delta |
|---|---|---|---|
| start (s4 suite) | 1715/3230 (53.1%) | 1515 | — |
| after round 1 | 2042/3230 (63.2%) | 1188 | +327 |
| after round 2 | 2153/3230 (66.7%) | 1077 | +111 |
| after round 3 | 2248/3230 (69.6%) | 982 | +95 |
| after round 4 | 2292/3230 (70.9%) | 938 | +44 |

### Per-round costs

| round | cost | tokens out |
|---|---|---|
| 1 | $4.07 | 62.7k |
| 2 | $2.22 | 44.0k |
| 3 | $2.19 | 35.5k |
| 4 | $2.51 | 53.2k |
| 5 | $4.98 | 44.8k |
| **total** | **$15.97** | |

### Final test suite

| metric | s4 (start) | s5 (final) | delta |
|---|---|---|---|
| Test functions | 68 | **201** | +133 |
| Lines | 2538 | **9137** | +6599 |
| Branch coverage | 53.1% | **70.9%** | +17.8pp |

### Grand total for Sonnet transpile + testgen + fix

| step | duration | cost |
|---|---|---|
| Sonnet transpile (isolated) | 74.7 min | $20.82 |
| Testgen s4 (2 rounds, from Opus era) | 13.5 min | $2.26 |
| Testgen s5 (5 rounds, branch-guided) | ~40 min | $15.97 |
| Difffix round 1 (s4 tests) | ~15 min | $12.01 |
| **Total so far** | **~145 min** | **$51.06** |
