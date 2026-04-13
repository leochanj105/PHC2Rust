# Goal 2: Fix double-free in parser/scanner/document cleanup

## Function
`yaml_parser_delete`, `yaml_token_delete`, `yaml_event_delete`, `yaml_document_delete`, and scanner/parser internal cleanup paths

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/api.c` (delete functions), `/home/leochanj/Desktop/libyaml/src/scanner.c`, `/home/leochanj/Desktop/libyaml/src/parser.c`
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/lib.rs`
  - `yaml_parser_delete` at line 1869
  - `yaml_token_delete` at line 2323
  - `yaml_event_delete` at line 2725
  - `yaml_document_delete` at line 2879

## What's Wrong
All 7 test functions that use the parser crash with SIGABRT during cleanup. stderr shows `free(): double free detected in tcache 2` for each crash. The affected test functions are:
- `parser_scan` — crashes after scanning tokens
- `parser_parse` — crashes after parsing events
- `parser_load` — crashes after loading document
- `max_nest_level` — crashes after nesting test
- `bridge_delete_aliases` — crashes immediately after delete_aliases call
- `custom_read_handler` — crashes after custom read test
- `roundtrip` — crashes after roundtrip test

The double-free indicates that cleanup code frees memory that was already freed, or frees stack/static memory. Common causes in C-to-Rust transpilation:
1. `yaml_parser_delete` freeing buffers that were already freed by token/event delete
2. String pointers shared between tokens/events being freed twice
3. Missing NULL-after-free guards (C code sets pointer to NULL after free)

## What Needs to Change
Compare each Rust delete function against its C counterpart in api.c:
1. `yaml_parser_delete` — must NULL out all pointers after freeing, must check for NULL before freeing
2. `yaml_token_delete` — must not double-free scalar/tag/anchor strings
3. `yaml_event_delete` — must not double-free event data strings
4. `yaml_document_delete` — must not double-free node data

Also check that the scanner's internal token queue cleanup doesn't free tokens that were already consumed and freed elsewhere.

## Success Criteria
- No FAULT lines in output (all 7 crashes eliminated)
- No "double free" messages in stderr
- `parser_scan` completes and produces `scan_token_count`
- `parser_parse` completes and produces all event groups through `parse_error_type`
- `parser_load` completes and produces full `load_doc_*` results
- `max_nest_level`, `bridge_delete_aliases`, `custom_read_handler`, `roundtrip` complete without crash
