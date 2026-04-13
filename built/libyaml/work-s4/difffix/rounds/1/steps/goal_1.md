# Goal 1: Fix double-free crash in yaml_token_delete

## Function
`yaml_token_delete` (and callers: `yaml_parser_delete`, `yaml_parser_scan`, scanner fetch functions)

## C source
- `/home/leochanj/Desktop/libyaml/src/api.c` (yaml_token_delete at line 584, yaml_parser_delete at line 220)
- `/home/leochanj/Desktop/libyaml/src/scanner.c` (fetch functions that enqueue tokens)

## Rust source
- `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs`
  - `yaml_token_delete` at line 2323
  - `yaml_parser_delete` at line 1869
  - `queue_dequeue` at line 1058
  - `queue_dequeue_token` at line 5997
  - `queue_enqueue` at line 1034

## What's wrong
All 14 FAULT tests crash with SIGNAL 6 (abort) due to "double free detected in tcache 2" inside `yaml_token_delete`. The crash occurs in two contexts:
1. During the test's scan/parse loop when calling `yaml_token_delete` on a dequeued token (e.g., `parser_scan`, `bridge_parser_scan_full`)
2. During `yaml_parser_delete` cleanup which iterates remaining tokens and calls `yaml_token_delete` on each (e.g., `parser_load`, `set_max_nest_level`, `bridge_parser_parse_full`)

The root cause is that token string pointers (e.g., `data.scalar.value`, `data.tag.handle`, `data.tag.suffix`, `data.anchor.value`, `data.alias.value`) are freed twice. This happens because somewhere in the scanner/queue pipeline, a token with allocated string pointers gets bitwise-copied (via the `Copy` trait) without proper ownership transfer, leaving two copies with the same pointer that both get freed.

Key areas to investigate:
- `queue_dequeue` / `queue_dequeue_token` copies the token via `Copy` but doesn't zero the original slot in the queue — if queue compaction (`yaml_queue_extend` memmove) or other operations reference the stale data, double-frees can occur
- `queue_enqueue` copies the token value into the queue — check if the local variable in fetch functions still holds live pointers that could be freed
- Compare with C's DEQUEUE/ENQUEUE macros to find semantic differences

## Affected tests (14 FAULT + 25 MISSING outputs)
- parser_scan, parser_load, set_max_nest_level
- bridge_parser_scan_full, bridge_parser_parse_full, bridge_parser_scan_tokens
- bridge_parser_fetch_stream, bridge_parser_fetch_document_indicators
- bridge_parser_fetch_collections, bridge_parser_scan_directives
- bridge_parser_parse_states, bridge_parser_process_empty_scalar
- bridge_parser_flow_sequence_mapping_entries, bridge_parser_indentless_sequence

## What needs to change
Find and fix the ownership bug in the token queue operations. The most likely fix is to zero out the token data in the queue slot after dequeuing (so the stale copy can't be double-freed), OR fix a bug where tokens are enqueued multiple times, OR fix how queue compaction handles token data. Compare the Rust queue operations byte-for-byte with the C macro equivalents.

## Success Criteria
- All 14 FAULT tests stop crashing (no SIGNAL 6, no double-free)
- The 25 MISSING test output lines appear in Rust output
- `yaml_token_delete` can be called on any dequeued token without abort
- `yaml_parser_delete` cleanup completes without abort
