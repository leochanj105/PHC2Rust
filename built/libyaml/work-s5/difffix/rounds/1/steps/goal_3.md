# Goal 3: Fix segfault in parser_multi_docs_scan

## Function(s)
- Scanner/parser functions exercised by multi-document scanning
- `yaml_parser_scan_to_next_token`, `yaml_parser_fetch_next_token`
- `yaml_parser_parse_document_start`, `yaml_parser_parse_document_end`

## Source Files
- **C sources**: `/home/leochanj/Desktop/libyaml/src/scanner.c`, `/home/leochanj/Desktop/libyaml/src/parser.c`
- **Rust source**: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs`

## What's Wrong
`parser_multi_docs_scan` crashes with SIGNAL 11 (SIGSEGV), indicating a null pointer
dereference or access to freed/invalid memory. This is the only test that crashes with
SIGSEGV rather than SIGABRT, suggesting a distinct issue from the double-free (Goal 1).

The test exercises scanning through multiple YAML documents in a single stream (multiple
`---` / `...` separators). The segfault likely occurs when:
1. The scanner resets state between documents but leaves a dangling pointer
2. A null check is missing where the C code has an implicit NULL guard via macro
3. The token queue is accessed after the parser reaches an error state
4. `parser_peek_token` returns null and the caller dereferences without checking

## What Needs to Change
1. First fix Goal 1 (double-free) — the SIGSEGV may be a consequence of memory
   corruption from double-free
2. If the SIGSEGV persists after Goal 1, reproduce with a multi-document YAML input
   and trace the exact crash location
3. Audit `yaml_parser_parse_document_end` (state transition back to
   YAML_PARSE_DOCUMENT_START_STATE) and ensure the scanner state is properly reset
4. Check all callers of `parser_peek_token` for null-pointer guards matching the C code
5. Verify `yaml_parser_process_directives` handles repeated calls correctly (called
   once per document)

## Success Criteria
- `parser_multi_docs_scan` completes without SIGSEGV
- All multi-document related assertions pass: `multi_doc_count`, `mixed_doc_count`,
  `doc_count`
- The `expl_doc1_events`, `expl_doc2_events` assertions produce correct output
