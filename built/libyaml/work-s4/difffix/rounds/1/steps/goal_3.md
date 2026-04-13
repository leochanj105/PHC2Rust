# Goal 3: Fix multi-document loading and parser event count mismatches

## Function
`yaml_parser_load`, `yaml_parser_parse`, and related parser state machine functions

## C source
- `/home/leochanj/Desktop/libyaml/src/loader.c` (yaml_parser_load)
- `/home/leochanj/Desktop/libyaml/src/parser.c` (yaml_parser_parse and state functions)

## Rust source
- `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs` (search for `yaml_parser_load`, `yaml_parser_parse`)

## What's wrong
Several non-crashing tests produce wrong output values. These are independent of the double-free crash (each test creates a fresh parser):

### 3a. test_multiple_documents — doc_count mismatch
- C output: `doc_count: 3`
- Rust output: `load_error: did not find expected <document start>` / `doc_count: 1`
- The Rust parser fails after loading the first document with a "did not find expected <document start>" error, preventing the second and third documents from loading.

### 3b. test_bridge_parser_parse_full — event_count mismatch (partially crashes)
- C output: 7 scalar lines + `event_count: 18`
- Rust output: 1 scalar line + `parse_error: did not find expected <document start>` + `event_count: 4`
- Parser fails early with the same "did not find expected <document start>" error.

### 3c. test_bridge_parser_parse_states — event_count mismatch (then crashes)
- C output: `event_count: 39`
- Rust output: `event_count: 4`

### 3d. test_bridge_parser_process_empty_scalar — events mismatch (then crashes)
- C output: `empty_scalar_found: 1` / `events: 10`
- Rust output: `events: 4` (no empty_scalar_found line)

### 3e. test_bridge_parser_flow_sequence_mapping_entries — events mismatch (then crashes)
- C output: `events: 14`
- Rust output: `events: 6`

### 3f. test_bridge_parser_indentless_sequence — events mismatch (then crashes)
- C output: `events: 11`
- Rust output: `events: 4`

### 3g. test_flow_sequence_compact_mapping — input2 event count
- C output: `input2_ok: 0 events: 1`
- Rust output: `input2_ok: 0 events: 4`

NOTE: Some of these (3b-3f) are from tests that also crash (Goal 1). After fixing the double-free, re-test to see if event counts correct themselves. The "did not find expected <document start>" error appearing in both crashing and non-crashing tests (3a, 3b) suggests a separate parser bug in document boundary detection that may be independent of the double-free.

## What needs to change
1. Fix the parser's document boundary detection — the "did not find expected <document start>" error indicates the parser fails to recognize document start markers (`---`) after the first document ends. Compare `yaml_parser_parse_document_start` and related state functions with C.
2. After Goal 1 is fixed, re-test the crashing tests (3b-3f) to see which event count mismatches persist.
3. For input2 event count (3g), compare how the parser handles flow sequence compact mappings.

## Success Criteria
- `test_multiple_documents` outputs `doc_count: 3` (matches C)
- `test_bridge_parser_parse_full` outputs `event_count: 18` with all 7 scalar lines
- All event counts match C output line-for-line
- `input2_ok: 0 events: 1` in test_flow_sequence_compact_mapping
