# Goal 1: Verify round 1 pointer arithmetic fix resolves all crashes

## Function
`yaml_parser_fetch_more_tokens` and `yaml_parser_save_simple_key`

## C source file
/home/leochanj/Desktop/libyaml/src/scanner.c

## Rust source file
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs

## What's wrong
**All 50 FAULT tests (SIGNAL 6 abort / SIGNAL 11 segfault) and all 162 MISSING test outputs** trace to a single root cause: byte-offset pointer arithmetic instead of element-count pointer subtraction at lines 6038 and 6214.

The round 1 fix replaced:
```rust
(*parser).tokens.tail as usize - (*parser).tokens.head as usize
```
with:
```rust
(*parser).tokens.tail.offset_from((*parser).tokens.head) as usize
```

This fix is already applied in the current source. However, the tests were never re-run after the fix, so the diff report still shows the pre-fix failures.

### Affected tests (50 crashes)
parser_scan, parser_load, set_max_nest_level, bridge_parser_scan_full, bridge_parser_parse_full, bridge_parser_scan_tokens, bridge_parser_fetch_stream, bridge_parser_fetch_document_indicators, bridge_parser_fetch_collections, bridge_parser_scan_directives, bridge_parser_parse_states, bridge_parser_process_empty_scalar, bridge_parser_flow_sequence_mapping_entries, bridge_parser_indentless_sequence, parser_utf8_bom, parser_complex_flow, parser_block_sequence_complex, parser_block_scalar_variants, parser_plain_scalar_variants, parser_nest_limit, parser_multi_anchors, parser_utf16_bom, parser_error_paths, loader_prestarted_stream, loader_stream_end_reuse, loader_explicit_tags, loader_sequence_mapping_tags, parser_block_mapping_various, parser_scan_complex_yaml, parser_implicit_docs, parser_utf16be_input, scanner_plain_scalar_multiline, scanner_complex_anchors_aliases, parser_block_mapping_various2, parser_flow_pairs, loader_anchor_tag_combinations, parser_node_with_anchor_tag, parser_stream_end_edge, scanner_complex_yaml_inputs, loader_flow_seq_and_map, parser_indentless_sequence_entry, parser_explicit_doc_end, loader_mapping_anchor_alias, scanner_tab_in_block_context, parser_flow_seq_mapping_pairs, parser_multi_docs_scan, scanner_block_scalar_chomp_strip, parser_flow_mapping_empty_value, loader_explicit_tag_on_scalar, parser_anchor_then_tag

### What needs to change
No additional code changes needed. The fix is already applied. Tests must be re-run to verify the fix is effective and to reveal any remaining divergences that were masked by the crashes.

### Analysis notes
- No other `as usize - ... as usize` pointer subtraction bugs were found on non-byte types
- Line 8303 (`nodes.top as usize - nodes.start as usize`) correctly divides by `size_of::<yaml_node_t>()` on line 8304
- All 512 previously passing tests had exact output match (no MISMATCH entries in diff report)
- The 162 MISSING tests (121 functions) are all downstream of the 50 crashes

## Success Criteria
- All 50 previously crashing test functions run without SIGNAL 6 or SIGNAL 11
- The 162 previously MISSING test results now produce output
- Rust test output matches C test output line-by-line for all affected tests
- No new crashes, double-free errors, or corruption messages in stderr
- If new mismatches appear (previously masked by crashes), they will be addressed in subsequent goals
