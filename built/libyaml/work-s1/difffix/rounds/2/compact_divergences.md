# Compact Divergences — Round 2

## Summary
- 212 tests failed in round 1 (191 passed)
- Root cause from round 1 (RC-1: pointer arithmetic bug) has been fixed
- All 212 failures were attributed to a single root cause
- Re-test needed to verify fix and identify any remaining secondary issues

## Round 1 Fix Applied

### RC-1 (FIXED): Pointer arithmetic bug in token queue element count

**Location**: `lib.rs:6037-6038` and `lib.rs:6213-6214`

Both locations now correctly use `offset_from()` to compute element count:
```rust
(*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

This replaced the incorrect raw address subtraction that produced byte offsets instead of element counts, which caused:
- 12 FAULT tests (SIGABRT from heap corruption via bad queue_insert index)
- 184 MISSING tests (output lost after crashes)
- 28 MISMATCH tests (block mappings not detected, shifted parse events)

## Expected Resolution

All 212 failures should be resolved by the RC-1 fix since every category traced back to the inflated `token_number` in `yaml_parser_save_simple_key`:

| Category | Count | Expected After Fix |
|----------|-------|--------------------|
| FAULT (crash) | 12 | Resolved — no more out-of-bounds queue_insert |
| MISSING | 184 | Resolved — no more crashes truncating output |
| MISMATCH (event:) | 26 | Resolved — block mappings detected correctly |
| MISMATCH (caught_error) | 1 | Resolved — correct error context with mapping recognized |
| MISMATCH (token_type) | 1 | Resolved — BLOCK_MAPPING_START token emitted |

## Potential Secondary Issues

If re-test reveals new failures after the fix, likely candidates:
1. Other pointer arithmetic sites (search for `as usize -` patterns on pointer fields)
2. Emitter-side bugs only visible once parser produces correct output (roundtrip, canonical_emitter tests)
3. Unicode handling differences (unicode_scalars test)
