# Experiment Roadmap

## What this experiment does

Transpiles libmcs (C math library, ~190 source files) to Rust, then compares
4 test generation strategies for finding and fixing transpilation bugs:

| Scenario | Description | Testgen method |
|----------|-------------|----------------|
| S1 | Naive one-shot | "generate tests" — no guidance |
| S2 | Explicit one-shot | All functions listed, internals via bridge wrappers |
| S3 | Multi-round + function coverage | Coverage feedback loop, up to 5 rounds |
| S4 | Multi-round + function + branch coverage | LLVM branch coverage feedback, up to 5 rounds |

Plus a **baseline** (raw transpiled Rust, no fixes) judged for comparison.

## API key

Both `claude` and `codex` CLIs read from the environment:

```bash
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
```

## Scripts

```
newexp/
├── common.sh               # Shared helpers, constants, env check
├── run_experiment.sh        # Runs all phases in order
│
├── 01_transpile.sh          # Phase 0: shared C→Rust transpilation
├── 02a_testgen_s1.sh        # Phase 1a: S1 naive one-shot testgen
├── 02b_testgen_s2.sh        # Phase 1b: S2 explicit all-functions testgen
├── 02c_testgen_s3.sh        # Phase 1c: S3 function coverage loop
├── 02d_testgen_s4.sh        # Phase 1d: S4 function + branch coverage loop
├── 03_diffgen.sh            # Phase 2: generate differential tests
├── 04_difffix.sh            # Phase 3: diff-test-fix loop
├── 05_judge.sh              # Phase 4: judger evaluation
│
├── run_difftest.sh          # Separate-binary diff test runner
├── judger_wrapper.sh        # Bridges judger_v2 to harness contract
├── claude-haiku             # Wrapper: claude --model haiku
├── claude-opus              # Wrapper: claude --model opus
├── specialize.md            # Libmcs context (comparison rule, etc.)
│
├── prompts/
│   ├── transpile.md         # C→Rust (used by 01)
│   ├── s1_testgen.md        # S1: naive "generate tests" (used by 02a)
│   ├── s2_testgen.md        # S2: all functions + internals + bridges (used by 02b)
│   ├── testgen.md           # S3/S4: coverage-guided generation (used by 02c, 02d)
│   ├── strategy.md          # S3/S4: coverage gap analysis (used by 02c, 02d)
│   ├── difftest.md          # Differential test generation (used by 03)
│   ├── analyze.md           # Failure analysis → fix goals (used by 04)
│   └── fixer.md             # Surgical Rust fixes (used by 04)
│
├── scripts/
│   ├── extract_signatures.py   # Pre-extract function sigs (~200 lines, ~1.2K tokens)
│   ├── extract_context.py      # Full context: headers + sigs (~2K lines)
│   └── make_difffix_context.py # Per-function divergence context for fix loop
│
├── scenarios/
│   ├── s1_naive/config_overrides.sh
│   ├── s2_explicit/config_overrides.sh
│   ├── s3_function/config_overrides.sh
│   └── s4_branch/config_overrides.sh
│
└── results/                 # Judger reports (created at runtime)
```

## Running

### Full experiment

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cd /home/leochanj/Desktop/libmcs/newexp
./run_experiment.sh -v
```

### Phase by phase

```bash
./01_transpile.sh -v              # Shared C→Rust (sonnet)

./02a_testgen_s1.sh -v            # S1: naive one-shot (haiku)
./02b_testgen_s2.sh -v            # S2: explicit all-functions (haiku)
./02c_testgen_s3.sh -v            # S3: function coverage loop (haiku)
./02d_testgen_s4.sh -v            # S4: func+branch coverage loop (haiku)

./03_diffgen.sh -v                # Diff tests for all scenarios (haiku)
./03_diffgen.sh -v --scenario s2  #   or one scenario

./04_difffix.sh -v                # Fix loops for all scenarios (sonnet)
./04_difffix.sh -v --scenario s3  #   or one scenario

./05_judge.sh -v                  # Judge all (no AI, just build + run)
```

## What each phase does

### 01_transpile.sh (model: sonnet)

- Reads all ~190 C source files from libm/
- Produces `rust-baseline/` (Cargo.toml + src/)
- Skips if `rust-baseline/Cargo.toml` exists

### 02a_testgen_s1.sh (model: haiku)

- Single AI call with `prompts/s1_testgen.md`
- Naive prompt: "generate tests for this library"
- No function list, no coverage feedback
- Output: `work-s1/testgen/test_suite.c`

### 02b_testgen_s2.sh (model: haiku)

- Single AI call with `prompts/s2_testgen.md`
- Explicit: cover ALL functions (public + internal via bridge wrappers)
- Pre-extracted function signatures fed in prompt (~200 lines)
- Output: `work-s2/testgen/test_suite.c` (+ optional `test_bridge.c`)

### 02c_testgen_s3.sh (model: haiku)

- Harness coverage-guided loop, up to 5 rounds
- Each round: extract functions → check coverage → strategy → generate
- `COVERAGE_MODES=function` — tracks which functions are called
- Uses `prompts/testgen.md` + `prompts/strategy.md`
- Output: `work-s3/testgen/test_suite.c`

### 02d_testgen_s4.sh (model: haiku)

- Same loop as S3 plus LLVM branch coverage feedback
- `COVERAGE_MODES=function,branch`
- Extracts per-branch coverage data to guide test generation
- Output: `work-s4/testgen/test_suite.c`

### 03_diffgen.sh (model: haiku)

- Copies `rust-baseline/` → `rust-s{1,2,3,4}/`
- For each: generates `difftest_suite.c` that prints results in %a format
- Same file compiled against C lib and Rust lib separately, outputs diffed
- Includes compile-fix loop
- Uses `prompts/difftest.md`

### 04_difffix.sh (model: sonnet)

- For each scenario: iterative fix loop via progress harness
  1. Run diff tests → collect divergences
  2. `make_difffix_context.py` extracts only diverging functions' source
  3. Analyze failures → goal files (uses `prompts/analyze.md`)
  4. Fix Rust per goal (uses `prompts/fixer.md`)
  5. Re-test → repeat until pass or stall
- Modifies `rust-s{N}/` in place

### 05_judge.sh (no AI)

- Runs `judger_v2` on all 5 Rust codebases (baseline + s1-s4)
- ~16M test cases: glibc vectors + core-math worst cases
- Bitwise comparison, zero tolerance
- Prints summary table

## Token optimization

| Phase | Without optimization | With optimization |
|-------|---------------------|-------------------|
| Testgen (S2-S4) | AI reads all 190 files (~170K tokens) | Pre-extracted signatures fed in prompt (~1.2K tokens) |
| Difffix analysis | AI reads all C + Rust (~340K tokens) | `make_difffix_context.py` feeds only diverging functions (~2-5K) |
| Difffix fixer | AI reads all source (~340K) | Gets one function pair (~1-2K) |

## Output (at runtime)

```
newexp/
├── rust-baseline/              # Raw transpilation (never modified)
├── rust-s1/ .. rust-s4/        # Fixed by each scenario
├── work-transpile/             # Transpilation logs
├── work-s1/ .. work-s4/        # Per-scenario artifacts
│   ├── testgen/                #   test_suite.c, logs
│   ├── diffgen/                #   difftest_suite.c, logs
│   ├── difffix/                #   rounds/, checkpoints/, fix_journal.md
│   └── prompts/                #   Expanded prompts
├── work-signatures.md          # Pre-extracted function signatures
└── results/
    ├── judger_baseline.txt
    └── judger_s{1,2,3,4}.txt
```

## Prerequisites

```
clang-21          # C compiler (LLVM coverage instrumentation)
cargo             # Rust toolchain
claude            # Claude CLI (~/.local/bin/claude)
jq                # JSON processor
python3           # For preprocessing scripts
```

## Troubleshooting

**Check logs**: Each AI call writes to `*_output` in the work directory.

**Re-run a phase**: Delete the artifact it checks:
- 01: `rm -rf rust-baseline/`
- 02a-d: `rm work-sN/testgen/test_suite.c`
- 03: `rm work-sN/diffgen/difftest_suite.c`
- 04: `rm work-sN/difffix/.transpile_done` + step markers

**Rate limits**: Harness wraps claude with 5-min retry, 4-hr max.

**Context too large**: If transpile hits "Prompt is too long", switch to Opus:
edit `01_transpile.sh` to use `claude-opus` instead of default.
