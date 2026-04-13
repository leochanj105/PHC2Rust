# Goal 1: Fix byte-offset vs element-count bug in `yaml_parser_save_simple_key`

## Function
`yaml_parser_save_simple_key`

## C source file
`/home/leochanj/Desktop/libyaml/src/scanner.c` (line 1118)

## Rust source file
`/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/lib.rs` (line 6205)

## What's wrong
The `token_number` field is computed using raw pointer address subtraction (`as usize`) instead of element-count pointer subtraction. At line 6213–6214:

```rust
token_number: (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize),
```

In C (scanner.c:1118–1119), this is pointer subtraction which yields element count:
```c
simple_key.token_number =
    parser->tokens_parsed + (parser->tokens.tail - parser->tokens.head);
```

For a `yaml_token_t` struct (~104 bytes), the Rust version produces a value ~104x too large. This corrupts the queue insert index, causing memory corruption, garbage scalar values, and double-free crashes (SIGABRT) in 19 tests.

## What needs to change
Replace the raw address subtraction with proper element-count pointer arithmetic:

```rust
token_number: (*parser).tokens_parsed
    + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize,
```

## Success Criteria
- The `token_number` uses `offset_from()` to compute element count, not byte offset.
- `scan_basic` test no longer crashes and produces the correct token sequence: `STREAM_START, BLOCK_MAPPING_START, KEY, SCALAR "key", VALUE, SCALAR "value", BLOCK_END, STREAM_END`.
- All 19 FAULT tests no longer crash with SIGABRT.
- No "double free" errors in stderr.
