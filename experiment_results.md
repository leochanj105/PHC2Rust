# Experiment Results

## Phase 1: Transpilation

| Phase | Wall Clock | Cost |
|-------|-----------|------|
| C → Rust transpilation | 1.7h | $26.82 |

Model: claude-sonnet-4-6

## Phase 2: Test Generation (S1–S5)

### Token Usage

| Scenario | Strategy | Rounds | Output Tokens | Cache Read | Cache Create | Total Tokens | Cost | Wall Clock |
|----------|----------|--------|--------------|------------|-------------|-------------|------|-----------|
| S1 | Naive one-shot | 1 | 43,721 | 177,042 | 52,319 | 273K | $0.91 | 6.8m |
| S2 | Explicit boundary | 1 | 45,422 | 842,404 | 69,745 | 958K | $1.20 | 8.7m |
| S3 | Edge case | 1 | 62,827 | 339,510 | 76,706 | 479K | $1.33 | 10.2m |
| S4 (old, with long double) | Function coverage feedback | 2 | 57,712 | 2,430,391 | 103,993 | 2.6M | $1.99 | 22.4m |
| S4 (new, corrected) | Function coverage feedback | 1 (R2 stopped early) | 40,942 | 439,934 | 44,243 | 525K | $0.91 | 6.8m |
| S5 (old, with long double) | Branch coverage feedback (on S4) | 5 | 435,255 | 15,945,081 | 807,094 | 17.2M | $26.84 | 2.8h |
| S5 (new) | Branch coverage feedback (on S4) | 5 | 315,504 | 15,915,718 | 911,806 | $14.19 | 65m |
| S6 | Extended branch (on S5) | 5 | 304,813 | 22,027,637 | 1,260,609 | $17.24 | 89m |

#### S4 (new) Per-Round Testgen Detail

| Round | Prints | Func Covered | Active Time | Output Tokens | Cache Read | Cache Create | Cost |
|-------|--------|-------------|------------|--------------|-----------|-------------|------|
| R1 | 371 | 186/186 (100%) | 6.8m | 40,942 | 439,934 | 44,243 | $0.91 |
| R2 | — | 186/186 (100%) | — | — | — | — | — |

R2 measured 0 uncovered functions and stopped early. S4 achieved 100% function
coverage in a single round.

#### S5 (new) Per-Round Testgen Detail

| Round | Prints | Cov Conditions | OUR Cov % | Active Time | Output Tokens | Cache Read | Cache Create | Cost |
|-------|--------|---------------|-----------|------------|--------------|-----------|-------------|------|
| S4 base | 371 | 1306/3268 | 40.0% | — | — | — | — | — |
| R1 | 1191 | 1306/3268 | 40.0% | 4.0m | 103,772 | 1,902,419 | 122,682 | $2.69 |
| R2 | 1272 | 2246/3268 | 68.7% | 11.2m | 36,684 | 3,395,848 | 146,361 | $2.25 |
| R3 | 1479 | 2286/3268 | 70.0% | 11.4m | 40,890 | 2,983,183 | 171,993 | $2.30 |
| R4 | 1837 | 2457/3268 | 75.2% | 17.4m | 63,444 | 2,620,303 | 179,805 | $2.64 |
| R5 | 2039 | 2607/3268 | 79.8% | 20.8m | 70,714 | 5,013,965 | 290,965 | $4.31 |
| **Total** | | | | **64.8m** | **315,504** | | | **$14.19** |

Note: R1 coverage shows 40.0% because the uncovered snapshot is measured BEFORE
test generation. R1's improvement shows in R2's measurement (40.0% → 68.7%).

#### S6 Per-Round Testgen Detail

| Round | Prints | Cov Conditions | OUR Cov % | Active Time | Output Tokens | Cache Read | Cache Create | Cost |
|-------|--------|---------------|-----------|------------|--------------|-----------|-------------|------|
| S5 base | 2039 | 2607/3268 | 79.8% | — | — | — | — | — |
| R1 | 2097 | 2646/3268 | 81.0% | 10.2m | 31,000 | 2,674,529 | 194,844 | $2.19 |
| R2 | 2238 | 2662/3268 | 81.5% | 19.8m | 67,468 | 4,263,268 | 306,356 | $3.63 |
| R3 | 2448 | 2694/3268 | 82.4% | 14.3m | 42,476 | 3,640,406 | 235,125 | $2.88 |
| R4 | 2746 | 2765/3268 | 84.6% | 28.8m | 100,222 | 5,624,625 | 238,800 | $4.60 |
| R5 | 2775 | 2797/3268 | 85.6% | 15.7m | 63,647 | 5,824,809 | 285,484 | $3.94 |
| **Total** | | | | **88.8m** | **304,813** | | | **$17.24** |

Note: Coverage % above is measured during the testgen loop (before each round's
generation). Final measured coverage with `branch_coverage.py`: **89.0% OUR**
(2909/3268). The difference is because the final round's tests improve coverage
beyond what was measured at the start of that round.

### Test Counts and Coverage

186 functions are actually compiled in the C library (178 public + 8 static).
Long double wrapper functions (`acosl`, `sinl`, etc.) are behind a disabled
`#ifdef __LIBMCS_LONG_DOUBLE_IS_64BITS` and excluded from the function list.

Two branch coverage metrics:
- **OUR**: from `branch_coverage.py` using `llvm-cov export` branch entries,
  deduplicated. Each branch = 2 conditions (true/false). Covered = count > 0.
  Total: 1634 branches = 3268 conditions.
- **REPORT**: from `llvm-cov report` "Branches" column filtered to libm/ files.
  Total: 3226. Uses LLVM's internal counting (different from export entries).

| Scenario | Test Cases | Func Cov | OUR Branch Cov | OUR (cov/total) | REPORT Branch Cov | REPORT (cov/total) |
|----------|-----------|----------|---------------|-----------------|-------------------|-------------------|
| S1 | 785 | 170/186 (91%) | 54.8% | 1790/3268 | 59.2% | 1911/3226 |
| S2 | 458 | 186/186 (100%) | 43.4% | 1418/3268 | 47.0% | 1516/3226 |
| S3 | 1190 | 174/186 (93%) | 66.1% | 2161/3268 | 71.4% | 2302/3226 |
| S4 | 371 | 186/186 (100%) | 41.6% | 1359/3268 | 45.3% | 1461/3226 |
| S5 | 2039 | 186/186 (100%) | 84.0% | 2744/3268 | 90.5% | 2921/3226 |
| S6 | 2775 | 186/186 (100%) | 89.0% | 2909/3268 | 96.0% | 3098/3226 |

Test case = one library function call with one input. S2 count excludes 1 fenv
crash (fault). S6 builds on S5 with 5 additional branch coverage rounds.

All measured with clang-21, llvm-cov-21, `-O0 -fno-builtin`, all .o linked directly.

Notes:
- S2's test_fenv crashes (segfault). Coverage measured with fenv test skipped.
- S4 has 100% function coverage but only 41.6% branch condition coverage.
- S5 builds on S4 with 5 rounds of branch coverage feedback (40.0% → 84.0%).
- S3's edge case prompt achieves 66.1% without any coverage feedback — better
  than S4 (41.6%) which uses function coverage feedback only.
- OUR metric is stricter than REPORT (different denominator and counting rules).
  Both show the same relative ordering: S5 > S3 > S1 > S2 > S4.

## Phase 3: Diff-Fix

### S3 Results (with test isolation)

Fixer mode: separate analyze + fix. Default mode (rollback + failed attempt
feedback on regression). Tests run independently via fork wrapper — crashes
and timeouts in one test don't affect others.

Results stored in: `work-s3/difffix/`, `rust-s3/`

S3 has 1190 test cases (each = one library function call with one input).

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 52 | 1138 | 95.6% | — | — | — |
| R1 | 52 | 37 | 1153 | 96.9% | 5 | 42.4m | $7.03 |
| R2 | 37 | 7 | 1183 | 99.4% | 5 | 11.1m | $2.11 |
| R3 | 7 | 2 | 1188 | 99.8% | 5 | 10.7m | $1.79 |
| R4 | 2 | 0 | 1190 | 100.0% | 1 | 1.6m | $0.44 |
| **Total** | | | | | **16** | **65.8m** | **$11.38** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 166 | 153,214 | 10,661,132 | 334,649 | $7.03 |
| R2 | 72 | 36,466 | 2,437,294 | 145,470 | $2.11 |
| R3 | 131 | 37,677 | 1,685,650 | 148,831 | $1.79 |
| R4 | 17 | 5,175 | 387,141 | 40,758 | $0.44 |
| **Total** | **386** | **232,532** | **15,171,217** | **669,708** | **$11.38** |

#### Remaining Failures: 0

#### Previous S3 Results (without test isolation, for reference)

Old results stored in: `work-s3/difffix-noreact/`, `rust-s3-noreact/`
- 5 rounds, ended at 2 failures (cacos/cacosf NaN sign), $9.91
- Round 4 regressed (7→8), rolled back; round 5 with feedback got 7→2
- Without test isolation, ctans timeout killed subsequent tests, giving
  the fixer less accurate feedback

### S4 Results (with test isolation)

Fixer mode: separate analyze + fix. No regressions occurred.

Results stored in: `work-s4/difffix/`, `rust-s4/`

S4 test suite: 371 printf calls, produces 371 output lines.

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 7 | 364 | 98.1% | — | — | — |
| R1 | 7 | 2 | 369 | 99.5% | 5 | 5.5m | $1.48 |
| R2 | 2 | 0 | 371 | 100.0% | 2 | 1.9m | $0.57 |
| **Total** | | | | | **7** | **7.4m** | **$2.04** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 19,460 | 15,615 | 895,920 | 100,947 | $1.48 |
| R2 | 81 | 5,063 | 348,596 | 53,046 | $0.57 |
| **Total** | **19,541** | **20,678** | **1,244,516** | **153,993** | **$2.04** |

#### Remaining Failures: 0

### S5 Results (with test isolation)

Fixer mode: separate analyze + fix. No regressions occurred.

Results stored in: `work-s5/difffix/`, `rust-s5/`

S5 has 2039 test cases.

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 44 | 1995 | 97.8% | — | — | — |
| R1 | 44 | 23 | 2016 | 98.9% | 5 | 22.8m | $3.84 |
| R2 | 23 | 3 | 2036 | 99.9% | 5 | 12.3m | $1.74 |
| R3 | 3 | 0 | 2039 | 100.0% | 2 | 1.8m | $0.54 |
| **Total** | | | | | **12** | **36.9m** | **$6.12** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 88 | 86,133 | 5,006,907 | 219,527 | $3.84 |
| R2 | 117 | 44,516 | 1,177,111 | 132,709 | $1.74 |
| R3 | 18 | 6,209 | 303,999 | 53,369 | $0.54 |
| **Total** | **223** | **136,858** | **6,488,017** | **405,605** | **$6.12** |

#### Remaining Failures: 0

### S2 Results (with test isolation)

Fixer mode: separate analyze + fix. No regressions occurred.

Results stored in: `work-s2/difffix/`, `rust-s2/`

S2 has 458 test cases + 1 fenv FAULT (crash) + 44 section headers in output.
Previous counts (496, 502) were wrong — section headers containing '=' were
miscounted as test cases. Fixed in compare_outputs.py.

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 16 | 442 | 96.5% | — | — | — |
| R1 | 16 | 5 | 453 | 98.9% | 5 | 8.3m | $1.44 |
| R2 | 5 | 0 | 458 | 100.0% | 4 | 21.4m | $3.38 |
| **Total** | | | | | **9** | **29.7m** | **$4.82** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 101 | 23,945 | 861,128 | 93,949 | $1.44 |
| R2 | 164 | 72,731 | 4,049,481 | 196,164 | $3.38 |
| **Total** | **265** | **96,676** | **4,910,609** | **290,113** | **$4.82** |

#### Remaining Failures: 0

### S1 Results (with test isolation)

Fixer mode: separate analyze + fix. No regressions occurred.

Results stored in: `work-s1/difffix/`, `rust-s1/`

S1 has 785 test cases.

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 15 | 770 | 98.1% | — | — | — |
| R1 | 15 | 5 | 780 | 99.4% | 5 | 7.3m | $1.92 |
| R2 | 5 | 0 | 785 | 100.0% | 3 | 3.0m | $0.72 |
| **Total** | | | | | **8** | **10.3m** | **$2.65** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 55 | 59,765 | 1,175,006 | 124,280 | $1.92 |
| R2 | 27 | 9,540 | 513,536 | 54,881 | $0.72 |
| **Total** | **82** | **69,305** | **1,688,542** | **179,161** | **$2.65** |

#### Remaining Failures: 0

### S6 Results (with test isolation)

Fixer mode: separate analyze + fix. No regressions occurred.

Results stored in: `work-s6/difffix/`, `rust-s6/`

S6 has 2775 test cases.

#### Per-Round Progression

| Round | Prev Fails | Fails | Passed | Pass Rate | Goals | Active Time | Cost |
|-------|-----------|-------|--------|-----------|-------|------------|------|
| Baseline | — | 68 | 2707 | 97.5% | — | — | — |
| R1 | 68 | 45 | 2730 | 98.4% | 5 | 6.5m | $1.35 |
| R2 | 45 | 35 | 2740 | 98.7% | 5 | 10.9m | $1.57 |
| R3 | 35 | 3 | 2772 | 99.9% | 5 | 21.2m | $3.30 |
| R4 | 3 | 1 | 2774 | 99.96% | 3 | 5.6m | $1.59 |
| R5 | 1 | 0 | 2775 | 100.0% | 1 | 5.4m | $1.35 |
| **Total** | | | | | **19** | **49.6m** | **$9.16** |

#### Per-Round Token Usage

| Round | Input | Output | Cache Read | Cache Create | Cost |
|-------|-------|--------|-----------|-------------|------|
| R1 | 43 | 21,437 | 755,561 | 123,766 | $1.35 |
| R2 | 104 | 42,110 | 717,604 | 135,524 | $1.57 |
| R3 | 81 | 79,176 | 3,346,605 | 195,663 | $3.30 |
| R4 | 43 | 19,449 | 1,356,355 | 87,782 | $1.59 |
| R5 | 31 | 19,858 | 972,355 | 66,276 | $1.35 |
| **Total** | **302** | **182,030** | **7,148,480** | **609,011** | **$9.16** |

#### Per-Round Fixes (audit summary)

| Round | Files touched | Notable fixes |
|-------|---------------|---------------|
| R1 | mathd.rs, mathf.rs, complexd.rs | acosh, cosf, fpclassifyf, cprojd — easy semantic fixes |
| R2 | mathd.rs, mathf.rs, complexf.rs | log10, ilogb/f, modf, partial pow/log1pf |
| R3 | mathd.rs, mathf.rs | pow (20 mismatches) + log1pf (9) — biggest single jump |
| R4 | mathd.rs, mathf.rs | log1pf 1-ULP precision fix; rem_pio2 NaN-sign (`x-x` → explicit qNaN for `inf`); remainder switched `%` → `fmodd()` |
| R5 | mathd.rs | remainder denormal: `while i > 0` (u32) → `while (i as i32) > 0` (signed) so subnormal exponent loop terminates correctly |

#### Remaining Failures: 0

## Difffix Cross-Scenario Summary

Test cases = individual function calls (one call, one input, one output comparison).

| Scenario | Test Cases | Baseline Fails | Final Fails | Rounds | Cost |
|----------|-----------|---------------|-------------|--------|------|
| S3 | 1190 | 52 | 0 | 4 | $11.38 |
| S4 | 371 | 7 | 0 | 2 | $2.04 |
| S1 | 785 | 15 | 0 | 2 | $2.65 |
| S2 | 458 | 16 | 0 | 2 | $4.82 |
| S5 | 2039 | 44 | 0 | 3 | $6.12 |
| S6 | 2775 | 68 | 0 | 5 | $9.16 |

Test case = one library function call with one input. Section headers and FAULT
lines are excluded. S2 has 1 fenv FAULT (crash).

## Configuration

- All AI calls: claude-sonnet-4-6
- Rust: opt-level = 0 (no LLVM optimization, prevents false infinite loops)
- C: clang-21, no optimization flags
- Rust test binary: linked without C fallback library
- Rust bridge: C-ABI wrappers for internal/static functions (test_bridge.rs)
- C bridge: test_bridge.c (wraps static functions for C binary)
- Timeout: 30s per test binary execution
- Difffix default behavior: rollback on regression + feed failed attempt's diff/failures
- REACT_MODE=0 (default): rollback + failed attempt feedback on regression
- REACT_MODE=1: additionally accumulate full ReAct history (all rounds' goals, diffs, results)

## File Layout

- `newexp/experiment_results.md` — this file
- `newexp/rust-baseline/` — pure transpilation output
- `newexp/rust-baseline-test/` — transpilation + test bridge (ready for difftest)
- `newexp/test_bridge.rs` — shared Rust bridge (test infrastructure)
- `newexp/test_bridge.c`, `test_bridge.h` — shared C bridge
- `newexp/work-s{1..5}/testgen/` — test generation results
- `newexp/work-s3/difffix-noreact/` — S3 difffix results (rollback+feedback mode)
- `newexp/work-s4/difffix/` — S4 difffix results (default mode)
- `newexp/rust-s4/` — S4 final Rust code (0 failures)
- `newexp/rust-s4-prefixbackup/` — S4 pre-difffix Rust code
- `newexp/work-s3/difffix-plain/` — S3 difffix round 5 plain mode
- `newexp/rust-s3-noreact/` — S3 final Rust code (rollback+feedback mode)
- `newexp/rust-s3-plain/` — S3 final Rust code (plain round 5)
