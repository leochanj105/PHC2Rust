# Goal 1: Fix fetch_more_tokens simple key check (wrong condition causes queue corruption and double-free)

## Function
`yaml_parser_fetch_more_tokens` (line ~6037 in lib.rs)

## C source file
/home/leochanj/Desktop/libyaml/src/scanner.c (line 833)

## Rust source file
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/lib.rs (line 6037-6038)

## What's wrong
**Crash (double-free / abort) — affects ALL 7 crashing test functions: parser_scan, parser_parse, parser_load, max_nest_level, bridge_delete_aliases, custom_read_handler, roundtrip. Causes all 324 missing tests.**

The simple key check in `yaml_parser_fetch_more_tokens` has an incorrect condition. In C (scanner.c:833):

```c
simple_key->token_number == parser->tokens_parsed
```

But in Rust (lib.rs:6037-6038):

```rust
(*sk).token_number == (*parser).tokens_parsed
    + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

The Rust code adds `queue_length` (tail - head) to `tokens_parsed`, which the C code does NOT do. The C condition checks whether the simple key's token is at the head of the queue (i.e., `token_number == tokens_parsed`). The Rust condition incorrectly checks whether `token_number` equals `tokens_parsed + queue_length` (the tail position).

**Root cause chain:**
1. The wrong condition prevents `fetch_more_tokens` from fetching required tokens when a simple key resolution is pending.
2. When `yaml_parser_fetch_value` later runs, `(*simple_key).token_number - (*parser).tokens_parsed` underflows (the simple key's token was already consumed because the scanner didn't know to stop).
3. In release mode, the underflow wraps to a huge `usize` value, which is used as an index in `queue_insert_token`, corrupting the queue memory layout.
4. When tokens with corrupted/aliased pointers are later freed by `yaml_token_delete`, the double-free crash occurs.

**Evidence:** In debug mode (with overflow checks), the crash is: `attempt to subtract with overflow` at lib.rs:6656 (`yaml_parser_fetch_value`), confirming the underflow.

## What needs to change
At line 6037-6038, remove the `+ queue_length` term:

**Before:**
```rust
&& (*sk).token_number == (*parser).tokens_parsed
    + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

**After:**
```rust
&& (*sk).token_number == (*parser).tokens_parsed
```

This matches the C code at scanner.c:833.

## Success Criteria
- All 7 previously crashing test functions (parser_scan, parser_parse, parser_load, max_nest_level, bridge_delete_aliases, custom_read_handler, roundtrip) run without crash/abort.
- All 324 previously missing tests produce output.
- No "double free", "attempt to subtract with overflow", or ASAN errors.
- The Rust output matches the C output line-by-line for all tests.
