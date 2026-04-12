# Compact Divergences — Round 1

## Summary
- 148 tests pass, 25 tests fail
- 14 tests crash with SIGNAL 6 (SIGABRT) — "free(): double free detected in tcache 2"
- 13 missing test assertion functions across failing tests

## Divergence Groups

### D1: Double-free crash in parser cleanup (14 tests)
All SIGABRT crashes occur during `yaml_parser_delete` after scanning/parsing.
Affected tests: parser_scan, parser_load, set_max_nest_level, bridge_parser_scan_full, bridge_parser_parse_full, bridge_parser_scan_tokens, bridge_parser_fetch_stream, bridge_parser_fetch_document_indicators, bridge_parser_fetch_collections, bridge_parser_scan_directives, bridge_parser_parse_states, bridge_parser_process_empty_scalar, bridge_parser_flow_sequence_mapping_entries, bridge_parser_indentless_sequence
Stderr: "free(): double free detected in tcache 2" (repeated 14 times)

### D2: Wrong simple key comparison in yaml_parser_fetch_more_tokens
Rust line 6037-6038: `(*sk).token_number == (*parser).tokens_parsed + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)`
C line 832-833: `simple_key->token_number == parser->tokens_parsed`
The Rust adds `(tail as usize - head as usize)` which is NOT in the C. This is a byte-offset not element-count, and the addition itself is wrong.

### D3: Flow level off-by-one (test_bridge_parser_flow_levels)
- C:    `increase_flow_ok: 1 level: 1` / `decrease_flow_ok: 1 level: 0`
- Rust: `increase_flow_ok: 1 level: 2` / `decrease_flow_ok: 1 level: 1`
The parser's flow_level starts at 1 instead of 0, or the test setup initializes differently.

### D4: Multi-document / complex YAML parsing fails
- test_multiple_documents: C `doc_count: 3`, Rust `load_error: did not find expected <document start>` + `doc_count: 1`
- test_bridge_parser_parse_full: C produces 6 scalars + `event_count: 18`, Rust produces `parse_error` + `event_count: 4`

### D5: Parser event counts wrong (before crash)
- test_bridge_parser_parse_states: Rust `event_count: 4` vs C `event_count: 39`
- test_bridge_parser_process_empty_scalar: Rust missing `empty_scalar_found: 1`, `events: 4` vs C `events: 10`
- test_bridge_parser_flow_sequence_mapping_entries: Rust `events: 6` vs C `events: 14`
- test_bridge_parser_indentless_sequence: Rust `events: 4` vs C `events: 11`
- test_flow_sequence_compact_mapping: Rust `input2_ok: 0 events: 4` vs C `input2_ok: 0 events: 1`

### D6: root_type mismatch in yaml_parser_load
- test_parser_load: C `root_type: 3` (YAML_SEQUENCE_NODE), Rust `root_type: 1` (YAML_SCALAR_NODE)

## Root Cause Analysis
D1 (double-free) is likely caused by scanner/parser corrupting internal state, leading to double-free during cleanup. D2 (wrong simple key comparison) may contribute to incorrect scanning which cascades into D4 and D5. D3 (flow level) is likely an initialization issue. D6 (root_type) may be a loader node-type mapping bug or a consequence of D2/D4 where the parser can't see past the first scalar.
