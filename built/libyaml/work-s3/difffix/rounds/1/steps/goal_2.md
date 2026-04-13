# Goal 2: Fix byte-offset bug in `yaml_parser_fetch_more_tokens` need-more-tokens check

## Function
`yaml_parser_fetch_more_tokens`

## C source file
`/home/leochanj/Desktop/libyaml/src/scanner.c` (line 833)

## Rust source file
`/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/lib.rs` (line 6026)

## What's wrong
The need-more-tokens check at line 6037–6038 adds a byte offset instead of element count:

```rust
if (*sk).token_number == (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```

The C original (scanner.c:833) simply checks:
```c
if (simple_key->possible
        && simple_key->token_number == parser->tokens_parsed) {
```

The C code does NOT add `(tail - head)` here — it only compares against `tokens_parsed`. The Rust code adds the extra (incorrectly computed) term, which breaks the need-more-tokens logic. This causes the scanner to stop fetching tokens too early or too late.

## What needs to change
Remove the extra byte-offset term to match the C code:

```rust
if (*sk).token_number == (*parser).tokens_parsed
```

## Success Criteria
- The `yaml_parser_fetch_more_tokens` function checks `token_number == tokens_parsed` without adding queue length.
- The scanner correctly fetches additional tokens when a simple key occupies the head position.
- Combined with Goal 1, all simple key and block mapping token emission works correctly.
