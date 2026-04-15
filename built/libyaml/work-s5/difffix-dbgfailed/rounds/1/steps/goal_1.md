# Goal 1: Fix token queue pointer arithmetic (byte offset vs element count)

## Function
`yaml_parser_fetch_more_tokens` and `yaml_parser_save_simple_key`

## C source file
/home/leochanj/Desktop/libyaml/src/scanner.c

## Rust source file
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs

## What's wrong
**Crash (SIGNAL 6 — double free / memory corruption) in all 50 scanner, parser, and loader test functions.**

Root cause: Two locations compute token queue length using raw byte offset instead of element count.

In C, `parser->tokens.tail - parser->tokens.head` is pointer subtraction on `yaml_token_t*`, which automatically yields the **element count**.

In the Rust translation, `(*parser).tokens.tail as usize - (*parser).tokens.head as usize` casts both pointers to `usize` first, yielding the **byte offset** — which is `element_count * size_of::<yaml_token_t>()` (many times too large).

This corrupted value is stored in `simple_key.token_number` and later used as an index in `queue_insert_token`, causing out-of-bounds writes that corrupt heap metadata and token pointer fields, leading to double-free crashes.

### Affected lines

1. **lib.rs:6038** in `yaml_parser_fetch_more_tokens`:
   ```rust
   (*sk).token_number == (*parser).tokens_parsed
       + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
   ```

2. **lib.rs:6214** in `yaml_parser_save_simple_key`:
   ```rust
   token_number: (*parser).tokens_parsed
       + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize),
   ```

### Fix
Replace both instances with proper pointer element-count arithmetic:
```rust
(*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

## Impact
This single bug causes ALL 50 FAULT tests (SIGNAL 6/11 crashes) and consequently all 162 MISSING test results (121 functions). Fixing it should resolve all crashes and allow the missing tests to produce output.

## Success Criteria
- All 50 previously crashing test functions (parser_scan, parser_load, set_max_nest_level, bridge_parser_scan_full, bridge_parser_parse_full, bridge_parser_scan_tokens, etc.) no longer crash with SIGNAL 6 or SIGNAL 11.
- The 162 previously MISSING test results now produce output.
- The Rust test output matches the C test output for all affected tests.
- No new crashes or double-free errors in stderr.
