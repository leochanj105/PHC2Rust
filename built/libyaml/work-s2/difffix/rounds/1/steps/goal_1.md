# Goal 1: Fix token queue length computation (byte count vs element count)

## Function
`yaml_parser_save_simple_key` (line 6205) and `yaml_parser_fetch_more_tokens` (line 6026)

## C source file
/home/leochanj/Desktop/libyaml/src/scanner.c

## Rust source file
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/lib.rs

## What's wrong
**Crash (double-free / abort) — affects 7 test functions: parser_scan, parser_parse, parser_load, max_nest_level, bridge_delete_aliases, custom_read_handler, roundtrip. Causes 324 missing tests.**

Two lines compute the number of tokens in the queue by casting typed pointers to `usize` before subtracting, which gives a **byte offset** instead of an **element count**:

- **Line 6214**: `token_number: (*parser).tokens_parsed + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)`
- **Line 6038**: `(*sk).token_number == (*parser).tokens_parsed + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)`

In C (scanner.c line 1119): `parser->tokens_parsed + (parser->tokens.tail - parser->tokens.head)` uses native pointer arithmetic which returns element count.

Since `yaml_token_t` is much larger than 1 byte, the Rust code produces a `token_number` that is `sizeof(yaml_token_t)` times too large. Later, at line 6656:
```rust
let idx = (*simple_key).token_number - (*parser).tokens_parsed;
```
This subtraction underflows (in release: wrapping to a huge value; in debug: panic "attempt to subtract with overflow"), producing a garbage index for `queue_insert_token`, which corrupts the token queue and leads to double-free on cleanup.

## What needs to change
Replace the byte-difference computation with proper pointer offset (element count) at both sites:

**Line 6214** — change:
```rust
token_number: (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize),
```
to:
```rust
token_number: (*parser).tokens_parsed
    + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize,
```

**Line 6038** — change:
```rust
&& (*sk).token_number == (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```
to:
```rust
&& (*sk).token_number == (*parser).tokens_parsed
    + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

## Success Criteria
- All 7 previously crashing test functions (parser_scan, parser_parse, parser_load, max_nest_level, bridge_delete_aliases, custom_read_handler, roundtrip) run without crash/abort.
- All 324 previously missing tests produce output.
- The Rust output matches the C output line-by-line for these tests.
- No "double free" or "attempt to subtract with overflow" errors.
