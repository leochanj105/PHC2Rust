# Goal 2: Fix simple key token_number comparison in yaml_parser_fetch_more_tokens

## Function(s)
`yaml_parser_fetch_more_tokens`

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/scanner.c` (yaml_parser_fetch_more_tokens, line 801)
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs` (yaml_parser_fetch_more_tokens, line 6026)

## What's Wrong
The Rust code at line 6037-6038 has an incorrect simple key check:
```rust
(*sk).token_number == (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```

The C code at scanner.c:832-833 is simply:
```c
simple_key->token_number == parser->tokens_parsed
```

Two bugs:
1. The Rust adds `(tail as usize - head as usize)` which does NOT exist in the C original.
2. Even if the addition were intended, `tail as usize - head as usize` computes a byte offset, not an element count (pointer arithmetic in C gives element count, but casting to usize in Rust gives byte offset).

This causes the scanner to incorrectly determine whether more tokens need to be fetched for simple key resolution, leading to incomplete/wrong scanning of YAML with implicit keys.

## What Needs to Change
Remove the `+ ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)` term from the comparison. The corrected line should be:
```rust
(*sk).token_number == (*parser).tokens_parsed
```
matching the C exactly.

## Success Criteria
The simple key check in `yaml_parser_fetch_more_tokens` matches the C logic: `simple_key->token_number == parser->tokens_parsed`. Tests that depend on implicit key scanning (block mappings, complex documents) produce correct token/event counts.
