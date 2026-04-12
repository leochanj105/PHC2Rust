# Goal 1: Fix double-free crash in parser/scanner cleanup

## Function(s)
`yaml_parser_delete` and internal scanner/parser functions that free token/buffer memory

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/api.c` (yaml_parser_delete, line 220)
- C source: `/home/leochanj/Desktop/libyaml/src/scanner.c` (scanner internals)
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs` (yaml_parser_delete at line 1869)

## What's Wrong
14 tests crash with SIGABRT (signal 6) due to "free(): double free detected in tcache 2". The crash happens during `yaml_parser_delete` cleanup after scanning/parsing operations. Memory that was already freed by a scanner/parser function is freed again by the cleanup path.

Affected tests: parser_scan, parser_load, set_max_nest_level, bridge_parser_scan_full, bridge_parser_parse_full, bridge_parser_scan_tokens, bridge_parser_fetch_stream, bridge_parser_fetch_document_indicators, bridge_parser_fetch_collections, bridge_parser_scan_directives, bridge_parser_parse_states, bridge_parser_process_empty_scalar, bridge_parser_flow_sequence_mapping_entries, bridge_parser_indentless_sequence

## What Needs to Change
1. Audit all scanner/parser functions that call `yaml_free` on token data, buffer data, or string data. Ensure that after freeing, the corresponding pointer is set to null so that `yaml_parser_delete` does not free it again.
2. Compare the Rust `yaml_token_delete` (line 2323), `buffer_del` (line 841), `string_del` (line 870), `stack_del` (line 936), and `queue_del` (line 1017) with their C macro counterparts (BUFFER_DEL, STACK_DEL, QUEUE_DEL, etc.) to ensure identical cleanup semantics.
3. Check if any scanner function (e.g., `yaml_parser_scan_plain_scalar`, `yaml_parser_scan_tag`, etc.) frees a string/token but leaves the pointer non-null in the token queue.
4. Verify that the Rust `yaml_parser_initialize` → `yaml_parser_delete` cycle is idempotent (no crash when parser was initialized but not used).

## Success Criteria
All 14 FAULT tests complete without SIGABRT. The `r_stderr.txt` contains no "double free" messages. Each test that previously crashed now produces output matching the C output or reveals a separate logic bug (addressed in other goals).
