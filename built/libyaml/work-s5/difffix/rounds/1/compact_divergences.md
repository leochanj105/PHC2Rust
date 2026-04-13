# Compact Divergences — Round 1

## Summary

- **Tests passed**: 511
- **Tests failed**: 163 (across 122 assertion functions)
- **Test functions crashed**: 50 (49 SIGNAL 6 / SIGABRT, 1 SIGNAL 11 / SIGSEGV)
- **C output lines**: 674 | **Rust output lines**: 559

## Root Causes

### RC-1: Double-free memory corruption (systemic)

49 test functions crash with SIGABRT due to `free(): double free detected in tcache 2`
and `corrupted double-linked list` errors. This is the dominant failure mode. The
double-free affects scanner, parser, and loader test functions uniformly, indicating the
root cause is in shared memory management infrastructure (token ownership, string
lifecycle, or cleanup paths).

**Affected test functions** (SIGNAL 6):
parser_scan, parser_load, set_max_nest_level, bridge_parser_scan_full,
bridge_parser_parse_full, bridge_parser_scan_tokens, bridge_parser_fetch_stream,
bridge_parser_fetch_document_indicators, bridge_parser_fetch_collections,
bridge_parser_scan_directives, bridge_parser_parse_states,
bridge_parser_process_empty_scalar, bridge_parser_flow_sequence_mapping_entries,
bridge_parser_indentless_sequence, parser_utf8_bom, parser_complex_flow,
parser_block_sequence_complex, parser_block_scalar_variants,
parser_plain_scalar_variants, parser_nest_limit, parser_multi_anchors,
parser_utf16_bom, parser_error_paths, loader_prestarted_stream,
loader_stream_end_reuse, loader_explicit_tags, loader_sequence_mapping_tags,
parser_block_mapping_various, parser_scan_complex_yaml, parser_implicit_docs,
parser_utf16be_input, scanner_plain_scalar_multiline,
scanner_complex_anchors_aliases, parser_block_mapping_various2, parser_flow_pairs,
loader_anchor_tag_combinations, parser_node_with_anchor_tag,
parser_stream_end_edge, scanner_complex_yaml_inputs, loader_flow_seq_and_map,
parser_indentless_sequence_entry, parser_explicit_doc_end,
loader_mapping_anchor_alias, scanner_tab_in_block_context,
parser_flow_seq_mapping_pairs, scanner_block_scalar_chomp_strip,
parser_flow_mapping_empty_value, loader_explicit_tag_on_scalar,
parser_anchor_then_tag

### RC-2: Assertion failure in yaml_parser_load_document

`loader_stream_end_reuse` test triggers: `assertion failed: (*event).type_ == YAML_DOCUMENT_START_EVENT` at `lib.rs:5131`. This panic crosses the FFI boundary and aborts. May be a consequence of RC-1 (corrupted event data) or an independent logic bug in the loader's handling of stream-end reuse.

### RC-3: Segfault in parser_multi_docs_scan

`parser_multi_docs_scan` crashes with SIGNAL 11 (SIGSEGV). Likely a null pointer
dereference or use-after-free in the scanner when processing multiple documents.

## Missing Test Assertions (163)

All 163 missing test assertions are downstream consequences of the 50 crashing test
functions. No independently missing functions were identified. When a test function
crashes, all remaining assertions from that function are lost.

## Relevant Source Files

- Rust: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs`
- C scanner: `/home/leochanj/Desktop/libyaml/src/scanner.c`
- C parser: `/home/leochanj/Desktop/libyaml/src/parser.c`
- C loader: `/home/leochanj/Desktop/libyaml/src/loader.c`
- C api: `/home/leochanj/Desktop/libyaml/src/api.c`
