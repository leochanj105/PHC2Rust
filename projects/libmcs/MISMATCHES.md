# Intentional divergences from upstream/libmcs/

The bake produces output that intentionally differs from `upstream/libmcs/` in
the following ways. These are deliberate design improvements, not bugs.

## Testgen scripts: 6 → 1

**Upstream:** `02a_testgen_s1.sh`, `02b_testgen_s2.sh`, `02c_testgen_s3.sh`,
`02d_testgen_s4.sh`, `02e_testgen_s5.sh`, `02f_testgen_s6.sh` — six near-duplicate
scripts, one per scenario.

**Baked:** a single `02_testgen.sh` that takes the scenario name as an argument
and dispatches via a `case` statement. Determines `MODE` (oneshot/loop),
`PROMPT_FILE`, and `PREV_SHORT` from the scenario name.

**Effect on verify:**
- `MISSING`: 02a, 02b, 02c, 02d, 02e, 02f (6 files)
- `EXTRA`: 02_testgen.sh (1 file)

**Usage change:** instead of `./02a_testgen_s1.sh`, run `./02_testgen.sh s1_naive`.

## Behavioral changes (vs the 6 upstream scripts)

1. **Bridge artifact dropped.** s2/s3 upstream PROMPT_CTX has the contradictory
   line `If you create test_bridge.c, write it to ${TESTGEN_DIR}/test_bridge.c`
   even though the testgen prompt explicitly forbids generating bridges and
   `setup_scenario_workdir` already copies the canonical bridge in. Removed.

2. **Bridge propagation in s5/s6 dropped.** Upstream s5 copies `test_bridge.c`
   from s4's workdir (and s6 copies both `test_bridge.c` and `test_bridge.h`
   from s5). The bridge is canonical and already provided per-scenario by
   `setup_scenario_workdir`, so propagating it from a predecessor is redundant.
   Only `test_suite.c` legitimately propagates from predecessor → successor.

3. **Log format simplified.** Upstream has bespoke human-readable log lines like
   `log "TESTGEN S1: NAIVE ONE-SHOT (model: sonnet)"`. Baked uses
   `log "TESTGEN ${SHORT}: ${SCENARIO}"` (e.g. `TESTGEN s1: s1_naive`).
   Same for the "done" message.

4. **s3 run echo simplified.** Upstream s3 prints `Running one-shot testgen with
   edge case guidance...`; baked prints the same `Running one-shot testgen...`
   as s1/s2.

5. **`${LIBMCS}` → `${TEST_CASE_DIR}` in PROMPT_CTX.** Generic naming so the
   same script template would work for non-libmcs projects.

## `difftest.md` dropped (orphan in upstream)

Upstream has `prompts/difftest.md` but no script ever references it.
`03_diffgen.sh` is documented as not needing AI generation:

> With separate-binary testing, test_suite.c IS the difftest — no generation
> needed. This script just copies test_suite.c to the diffgen dir...

Test files are mechanically wrapped via `scripts/wrap_tests_independent.py`,
not generated via Claude. The "compile twice and diff stdout" pattern is
general (works for libyaml too), so this prompt would be unused for any
project, not just libmcs.

**Effect on verify:** `MISSING`: `prompts/difftest.md`

## `analyze_and_fix.md`: fixed `__C_SRC__/` artifact

Upstream `analyze_and_fix.md` contains a literal `__C_SRC__/` placeholder
that nothing ever substitutes (the runtime `expand_prompt` function in
`common.sh` only substitutes `__RUST_DIR__`, not `__C_SRC__`). The result
is that the AI sees an unsubstituted placeholder string in the prompt at
runtime — clearly a bug.

Baked version uses `__CODE_LAYOUT__` (a bake-time placeholder) which
expands to the real `/home/leochanj/Desktop/libmcs/libm/` path.

**Effect on verify:** `MISMATCH`: `prompts/analyze_and_fix.md`

## Pile A: absorbed + 3 cross-project fixes

Pile A was supposed to be 5 verbatim files. 3 actually were:
`04_difffix.sh`, `scripts/compare_outputs.py`, `scripts/wrap_tests_independent.py`.

The other 2 plus one pile-B file had libmcs-specific bits that only surfaced
on a thorough re-scan (path-contains filters and hardcoded subdirectory
lists — not the same as the `fenv.c` pattern that pile B fixed).

### `scripts/branch_coverage.py` — filter prefix parameterized

Upstream hardcodes `filter_prefix='libm/'` in `load_branches()` and checks
the literal string `'libm/'` in 3 places. For any project whose file paths
don't contain `libm/`, the function drops every branch and returns empty.

Baked version reads `SOURCE_PATH_MARKER` from env (defaults to empty =
no filter). libmcs sets `SOURCE_PATH_MARKER=libm/` in its `__LIB_EXPORTS__`.
Behavior is preserved for libmcs; libyaml sets its own marker (or empty).

### `scripts/make_difffix_context.py` — hardcoded subdir list replaced

Upstream hardcodes `for src_dir in ["mathd", "mathf", "common", "complexd",
"complexf"]` when searching for a function's C source file. libmcs-only.

Baked version reads `$C_SRC_DIRS` from env (already exported by
`common.sh`) and walks whatever directories the project specifies. The
`--libm` argparse flag is dropped entirely (superseded by the env var).

### `run_difffix_loop.sh` — `--libm` flag dropped

Drops the now-unused `--libm "${TEST_CASE_DIR}"` argument from its
`make_difffix_context.py` invocation. 1-line edit.

### Supporting value: `SOURCE_PATH_MARKER`

New export in `projects/libmcs/values` `__LIB_EXPORTS__` block:
`export SOURCE_PATH_MARKER="libm/"`. Also `echo`ed at startup alongside
`EXCLUDE_C_FILES` for visibility.

**Effect on verify:**
- `MATCH`: `04_difffix.sh`, `scripts/compare_outputs.py`, `scripts/wrap_tests_independent.py`
- `MISMATCH`: `scripts/branch_coverage.py`, `run_difffix_loop.sh` (new), `common.sh` (now has `SOURCE_PATH_MARKER`), `scripts/make_difffix_context.py` (already mismatched from pile B; more extensive edit now)

## Pile B: exclusion lists made explicit via `EXCLUDE_C_FILES`

Upstream libmcs hardcodes `grep -v fenv.c` or a 12-file `_EXCL=` list across
several scripts to skip source files that either fail to compile under LLVM
coverage instrumentation (`fenv.c`) or contain no real code (the 11 macro
placeholder files like `isfinite.c`).

**Problem:** buried in a grep pipeline, this is a silent coverage hole.
`fenv.c`'s 10 real functions (`feclearexcept`, `fesetround`, ...) are invisible
to the s4/s5 coverage feedback loop — the AI is never told they're uncovered.

**Fix (behavior preserved for libmcs, but visibility added):**

1. **New env var `EXCLUDE_C_FILES`** exported from `common.sh` via the
   project-specific `__LIB_EXPORTS__` placeholder. libmcs sets it to `fenv.c`;
   future projects set it to empty (or whatever) explicitly.
2. **Loud at startup:** `common.sh` echoes `EXCLUDE_C_FILES: ${EXCLUDE_C_FILES:-(none)}`
   on every invocation so the gap is visible in the log.
3. **Scripts updated:** the hardcoded `grep -v fenv.c` or `_EXCL="..."` is
   replaced with `grep -v "${EXCLUDE_C_FILES:-^$}"` (matches nothing if unset).
   - `03_diffgen.sh`, `run_difftest.sh`, `run_testgen_loop.sh`
   - `scripts/build_and_cover.sh`, `scripts/extract_functions.sh`
4. **The 11 empty macro files (`isfinite.c`, etc.) are dropped from the
   exclusion list** — they have no code, nothing to exclude.
5. **Python scripts:** hardcoded `/home/leochanj/Desktop/libmcs/libm` defaults
   removed from `build_func_map.py`, `extract_signatures.py`,
   `make_difffix_context.py`. They now read `LIBMCS` from env (exported by
   `common.sh`) and fail fast if unset.

**Effect on verify:**
- `MISMATCH`: 8 pile-B files (the edits above) + `common.sh` (already listed)
- `MATCH`: `scripts/run_all_configs.sh` (verbatim, no edit needed)

## Intentionally dropped: orphan scripts (pile C)

These scripts exist in `upstream/libmcs/scripts/` but are never called from
any active phase script. They were either previously wired into a workflow
that no longer exists, or they're alternative implementations that were
never adopted. All transitively dead.

| file | reason |
|---|---|
| `scripts/extract_functions_llvmcov.sh` | alternative to `extract_functions.sh`, never referenced |
| `scripts/summarize_coverage.sh` | no callers |
| `scripts/extract_branches.sh` | only called by `summarize_coverage.sh` (dead) |
| `scripts/measure_coverage.sh` | only called by `extract_branches.sh` (dead) |
| `scripts/get_uncovered.sh` | only called by `summarize_coverage.sh` (dead) |
| `scripts/extract_context.py` | no callers |

**Effect on verify:** 6 `MISSING` entries, all intentional.

## Out of scope: irrelevant files (pile D)

Files in `upstream/libmcs/` that are not framework material and will not
be baked. They're either runtime build artifacts, library-specific inputs
handled externally, or project-specific content that doesn't belong in a
cross-project framework.

| file | why |
|---|---|
| `05_judge.sh` | runs libmcs-specific `judger_v2` (glibc + core-math float evaluator); nothing analogous for libyaml. Explicitly skipped. |
| `test_bridge.c` / `test_bridge.h` / `test_bridge.rs` | external inputs, placed at `${EXP_DIR}/` before running `01b_prepare.sh` |
| `branch_total.txt` | runtime build artifact |
| `work-branches.json` / `work-branches.md` / `work-func-map.txt` / `work-functions.md` | runtime build artifacts (produced by coverage tooling) |
| `scripts/__pycache__/*.pyc` | compiled Python bytecode |
| `README.md` | project-specific documentation |

**Effect on verify:** 13 `MISSING` entries, all out of scope.

## New prepare phase: `01b_prepare.sh` + `promote_visibility.py`

**Upstream** has `scripts/prepare_rust_for_test.sh` — a 94-line script with
14 hardcoded `sed` commands that promote specific function names
(`tan_kern`, `sin_pi`, `tanf_kern`, etc.) from private to `pub(crate)` in
the transpiled Rust sources. The list is hand-maintained and libmcs-specific.

**Baked** introduces a new phase:
- `01b_prepare.sh` (new, between `01_transpile.sh` and `02_testgen.sh`).
- `scripts/promote_visibility.py` (new) — parses `test_bridge.rs` to find
  all `crate::module::fn_name(` references, then promotes the corresponding
  functions in `rust-baseline-test/src/`. **Data-driven** — no hardcoded
  list, works for any library whose `test_bridge.rs` follows the
  `crate::module::fn(...)` pattern.
- Upstream's `scripts/prepare_rust_for_test.sh` is dropped (no longer baked).

**`test_bridge.{c,h,rs}` remain external inputs** — placed at `${EXP_DIR}/`
before running `01b_prepare.sh`. The framework does not generate them;
that is handled by a separate utility (e.g. libyaml's `cov/gen_bridges.py`)
out of scope for the bake.

**Effect on verify:**
- `EXTRA`: `01b_prepare.sh`, `scripts/promote_visibility.py`
- `MISSING`: `scripts/prepare_rust_for_test.sh` (intentionally dropped)

## Scenario configs collapsed

**Upstream:** Each `scenarios/sN_*/config_overrides.sh` is ~25 lines and
duplicates project-wide settings (TEST_CASE_DIR, C_SRC_DIRS, C_INCLUDE_DIRS,
JUDGER_DIR, JUDGER_SCRIPT, DIFFTEST_SCRIPT, _EXP) across all 6 files.

**Baked:**
- Project-wide vars (`JUDGER_DIR`, `JUDGER_SCRIPT`, `DIFFTEST_SCRIPT`) moved
  into `common.sh` (single source of truth, swappable per project via the
  `__LIB_EXPORTS__` placeholder).
- Each `scenarios/sN_*/config_overrides.sh` shrinks to ~5 lines: just the
  scenario-specific `RUST_DIR`, `WORK_DIR`, `COVERAGE_MODES`, `MAX_ROUNDS`,
  `STALL_LIMIT`. Uses `${EXP_DIR}` (already exported by `common.sh`) instead
  of redefining `_EXP`.

**Effect on verify:**
- `MISMATCH`: `common.sh` (+3 export lines)
- `MISMATCH`: all 6 `scenarios/sN_*/config_overrides.sh` (much shorter)
