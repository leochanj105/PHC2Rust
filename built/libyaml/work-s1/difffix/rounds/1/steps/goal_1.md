# Goal 1: Fix token queue element count pointer arithmetic

## Function
`yaml_parser_save_simple_key` and `yaml_parser_scan_next_token` (need_more_tokens check)

## C Source
/home/leochanj/Desktop/libyaml/src/scanner.c

## Rust Source
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s1/src/lib.rs

## What's Wrong
Two locations compute the number of tokens in the queue using raw address subtraction instead of typed pointer offset:

**Line 6037-6038** (need_more_tokens check in `yaml_parser_scan_next_token`):
```rust
&& (*sk).token_number == (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```

**Line 6213-6214** (in `yaml_parser_save_simple_key`):
```rust
token_number: (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize),
```

In C, `parser->tokens.tail - parser->tokens.head` performs pointer arithmetic that divides by `sizeof(yaml_token_t)`, yielding element count. The Rust code casts pointers to `usize` first, producing a **byte offset** instead — `sizeof(yaml_token_t)` times too large.

This causes `yaml_parser_fetch_value` to compute a wildly wrong insertion index for BLOCK_MAPPING_START tokens, leading to:
- Block mappings never detected by the parser
- `queue_insert` called with out-of-bounds index → `memmove` with wrapped count → heap corruption
- Double-free crashes (SIGABRT) during cleanup

## What Needs to Change
Replace both occurrences of:
```rust
(*parser).tokens.tail as usize - (*parser).tokens.head as usize
```
with:
```rust
(*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

This correctly computes the element count, matching the C pointer arithmetic semantics.

## Success Criteria
- All 12 FAULT tests (parse_mapping, parse_complex, parse_anchors, parse_tags, parse_directive, parse_multidoc, parse_error_handling, scan_tokens, load_document, roundtrip, unicode_scalars, canonical_emitter) no longer crash with SIGABRT
- Block mappings are correctly detected: parser emits MAPPING_START events where C does
- The 184 MISSING tests produce output
- The token_type mismatch (C=8/BLOCK_MAPPING_START vs Rust=21/SCALAR) is resolved
- The caught_error mismatch resolves (correct error context with mapping recognized)
- No "double free" or "corrupted" messages in stderr
- Zero stderr output from the Rust test binary (matching C behavior)
