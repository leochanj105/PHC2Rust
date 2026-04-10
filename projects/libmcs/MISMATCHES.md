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

## `analyze_and_fix.md`: fixed `__C_SRC__/` artifact

Upstream `analyze_and_fix.md` contains a literal `__C_SRC__/` placeholder
that nothing ever substitutes (the runtime `expand_prompt` function in
`common.sh` only substitutes `__RUST_DIR__`, not `__C_SRC__`). The result
is that the AI sees an unsubstituted placeholder string in the prompt at
runtime — clearly a bug.

Baked version uses `__CODE_LAYOUT__` (a bake-time placeholder) which
expands to the real `/home/leochanj/Desktop/libmcs/libm/` path.

**Effect on verify:** `MISMATCH`: `prompts/analyze_and_fix.md`

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
