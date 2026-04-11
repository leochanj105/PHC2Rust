# libyaml project notes

## Workflow

```
./01_transpile.sh                       # → built/libyaml/rust-baseline/
./01b_prepare.sh                        # → built/libyaml/rust-baseline-test/
./02_testgen.sh s1_naive                # → built/libyaml/work-s1/testgen/test_suite.c
./02_testgen.sh s2_explicit
... etc
./03_diffgen.sh
./04_difffix.sh
```

All scripts run from `built/libyaml/` (which is also `EXP_DIR`). Outputs go
into the same directory.

## Bridges

libyaml has 137 static functions across 8 source files. Bridges are
**auto-generated** by `scripts/gen_bridges.py` from a coverage-instrumented
binary, in **per-file layout** (`bridge_<src>.c`). Output: 8 `bridge_*.c` files
+ 1 `test_bridge.h`.

This is a different layout from libmcs's hand-written single-file
`test_bridge.{c,h}`. Both are supported by `setup_scenario_workdir()` —
trigger is the existence of `test_bridge.h` at `${EXP_DIR}`.

## "Naive one-shot" is a misnomer

`s1_naive` and `s2_explicit` are labelled "one-shot" but in practice
**Claude self-iterates internally** within the single invocation:

1. Reads source via `cat`/`grep`/`sed`/`ls`
2. Writes test_suite.c
3. Compiles it with `gcc` to syntax-check
4. Runs the binary
5. Iterates if errors

Observed in libyaml s1 run: ~31 Bash calls including `gcc -c`, `gcc -o test_suite_run`,
running the binary, and re-editing test_suite.c.

The "one-shot" / "loop" distinction in `02_testgen.sh` only refers to **whether
the framework's coverage feedback loop runs** (via `run_testgen_loop.sh`). It
does not constrain Claude's internal tool use within a single phase invocation.

This is fine — and arguably better than a true "naive" run — but it's worth
knowing when interpreting the s1 results.

## Permission scoping

`settings.json` (in `__SETTINGS_JSON__` value) explicitly **denies** Read/Glob/Grep
on these libyaml subdirectories that should not leak into testgen:

- `tests/`, `testing/`, `Testing/`     — upstream libyaml test suite
- `examples/`                           — example programs
- `judger/`                             — judge harness setup
- `regression-inputs/`                  — regression test data
- `old/`                                — moved-aside experiment work
- `rust-baseline/`, `rust-branch/`, `rust-function/`  — prior transpilation experiments

**Caveat:** these denies only block the **dedicated tools** (Read/Glob/Grep).
`Bash` is in the allow list with no path restriction, so a future Claude run
could in principle `cat libyaml/tests/foo.c`. The s1_naive run audited (2026-04-11)
did not do so — it only Bash-touched `libyaml/src/` and `libyaml/include/`. But
the deny rules are not airtight.

If full enforcement matters, the next layer would be a Bash hook or
sandboxing the script's view of the filesystem (chroot, bind-mounts).
