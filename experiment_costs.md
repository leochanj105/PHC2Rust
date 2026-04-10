# Experiment Cost & Timing Summary

## Overview

| Phase | Scenario | Wall Clock | Output Tokens | Total Tokens | Cost |
|-------|----------|-----------|--------------|-------------|------|
| **Transpile** | — | 1.7h | 38,778 | 2.2M | $26.82 |
| **Testgen** | S1 | 6.8m | 43,721 | 273K | $0.91 |
| | S2 | 8.7m | 45,422 | 958K | $1.20 |
| | S3 | 10.2m | 62,827 | 479K | $1.33 |
| | S4 | 22.4m | 115,424 | 5.1M | $3.98 |
| | S5 (5 rounds) | 2.8h | 509,355 | 23.6M | $26.84 |
| **Difffix** | S1 (3 rounds) | 33.9m | 115,000 | 4.6M | $5.42 |

## Token Breakdown

| Phase | Scenario | Input | Output | Cache Read | Cache Create | Cost |
|-------|----------|-------|--------|-----------|-------------|------|
| Transpile | — | 3,461 | 38,778 | 1,898,646 | 285,892 | $26.82 |
| Testgen | S1 | 10 | 43,721 | 177,042 | 52,319 | $0.91 |
| Testgen | S2 | 21 | 45,422 | 842,404 | 69,745 | $1.20 |
| Testgen | S3 | 14 | 62,827 | 339,510 | 76,706 | $1.33 |
| Testgen | S4 | 5,076 | 115,424 | 4,860,782 | 207,986 | $3.98 |
| Testgen | S5 | 2,642 | 509,355 | 22,132,905 | 1,001,641 | $26.84 |
| Difffix | S1 | 158 | 115,000 | 4,145,849 | 360,209 | $5.42 |

## Totals

| | Wall Clock | Cost |
|---|-----------|------|
| Transpile | 1.7h | $26.82 |
| Testgen (S1–S5) | ~3.2h | $34.26 |
| Difffix (S1 only) | 34m | $5.42 |
| **Grand Total** | ~5.4h | **$66.50** |

## Notes

- All testgen/difffix use claude-sonnet-4-6. Transpilation used claude-sonnet-4-6.
- S5 rounds 1–3 also used claude-haiku-4-5 for file-reading subagents.
- S4 ran 2 testgen rounds (function coverage feedback).
- S5 ran 5 testgen rounds (branch coverage feedback), building on S4's test suite.
- S1 difffix achieved 100% test pass (779/779) in 3 rounds.
- Wall clock includes AI API time + local compile/test time.
- S5 R4 hit rate limits ($3.57 mostly retry delays, minimal output).
