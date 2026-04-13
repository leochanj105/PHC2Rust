# Goal 1: Verify round 1 fix resolves all 212 test failures

## Function
`yaml_parser_save_simple_key` and `yaml_parser_scan_next_token` (token queue element count)

## C Source
/home/leochanj/Desktop/libyaml/src/scanner.c

## Rust Source
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s1/src/lib.rs

## What's Wrong
Round 1 identified a single root cause: pointer arithmetic at lib.rs:6037-6038 and lib.rs:6213-6214 computed byte offsets instead of element counts for the token queue length. This caused:
- 12 FAULT tests (SIGABRT from heap corruption via out-of-bounds queue_insert)
- 184 MISSING tests (output lost after crashes)
- 28 MISMATCH tests (block mappings not detected, shifted event values, wrong error messages)

The fix has been applied — both sites now use `offset_from()`:
```rust
(*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

## What Needs to Change
No code change needed. Re-run differential tests to verify all 212 failures are resolved. If any failures persist, they represent secondary bugs masked by the original crash.

## Success Criteria
- All 12 FAULT tests pass without SIGABRT (parse_mapping, parse_complex, parse_anchors, parse_tags, parse_directive, parse_multidoc, parse_error_handling, scan_tokens, load_document, roundtrip, unicode_scalars, canonical_emitter)
- All 184 MISSING tests produce output matching C
- All 28 MISMATCH tests now match C output:
  - event:: values align with C (mapping keys/values in correct positions)
  - token_type: emits 8 (BLOCK_MAPPING_START) where C does
  - caught_error: reports correct error message matching C
- Zero "double free" or "corrupted" messages in stderr
- Total passed tests: 403 (all tests pass)
