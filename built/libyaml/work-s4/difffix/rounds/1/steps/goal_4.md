# Goal 4: Fix multi-document and complex YAML parsing

## Function(s)
`yaml_parser_parse`, `yaml_parser_state_machine`, `yaml_parser_parse_document_start`, `yaml_parser_parse_document_end`, `yaml_parser_process_directives`, and related parser state machine functions

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/parser.c`
- C source: `/home/leochanj/Desktop/libyaml/src/scanner.c` (document indicator scanning)
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs`

## What's Wrong
Multiple tests show the parser cannot handle multi-document YAML or complex event sequences:

1. **test_multiple_documents**: C produces `doc_count: 3`, Rust produces `load_error: did not find expected <document start>` and `doc_count: 1`. The parser fails after the first document.

2. **test_bridge_parser_parse_full**: C produces 6 scalars + `event_count: 18`, Rust produces `parse_error: did not find expected <document start>` and `event_count: 4`. The parser stops after STREAM_START + first document start events.

3. **test_bridge_parser_parse_states**: C `event_count: 39`, Rust `event_count: 4`. The parser stops very early in a complex document.

4. **test_bridge_parser_process_empty_scalar**: C `empty_scalar_found: 1` + `events: 10`, Rust missing `empty_scalar_found` + `events: 4`.

5. **test_bridge_parser_flow_sequence_mapping_entries**: C `events: 14`, Rust `events: 6`.

6. **test_bridge_parser_indentless_sequence**: C `events: 11`, Rust `events: 4`.

7. **test_flow_sequence_compact_mapping**: C `input2_ok: 0 events: 1`, Rust `input2_ok: 0 events: 4`.

The "did not find expected <document start>" error suggests the parser's document-start state handling is broken. The low event counts (4 in many cases = STREAM_START + DOC_START + first_node + DOC_END or similar) suggest the parser state machine exits too early.

This is likely related to Goal 2 (wrong simple key check causing incomplete token fetching), but may also have independent bugs in the parser state machine.

## What Needs to Change
1. Fix Goal 2 first (simple key comparison), then re-test — many of these failures may resolve.
2. If failures persist, compare the Rust parser state machine functions with C `parser.c`:
   - `yaml_parser_parse_document_start` — check the transition after first document
   - `yaml_parser_parse_document_end` — check if it correctly transitions to next document
   - `yaml_parser_parse_document_content` — check empty scalar handling
   - `yaml_parser_parse_flow_sequence_entry` / `yaml_parser_parse_flow_mapping_key` — check flow entry parsing
   - `yaml_parser_parse_indentless_sequence_entry` — check indentless sequence handling
3. Verify `yaml_parser_process_directives` correctly resets state for subsequent documents.

## Success Criteria
All affected tests produce output matching C line-by-line:
- test_multiple_documents: `doc_count: 3`
- test_bridge_parser_parse_full: 6 scalars + `event_count: 18`
- test_bridge_parser_parse_states: `event_count: 39`
- test_bridge_parser_process_empty_scalar: `empty_scalar_found: 1` + `events: 10`
- test_bridge_parser_flow_sequence_mapping_entries: `events: 14`
- test_bridge_parser_indentless_sequence: `events: 11`
- test_flow_sequence_compact_mapping: `input2_ok: 0 events: 1`
