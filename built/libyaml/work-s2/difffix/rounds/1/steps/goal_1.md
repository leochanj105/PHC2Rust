# Goal 1: Fix scanner — missing block-level token emission

## Function
`yaml_parser_fetch_next_token` (and related block-token fetch functions in the scanner)

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/scanner.c`
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/lib.rs` (line ~6060, `yaml_parser_fetch_next_token`)

## What's Wrong
The Rust scanner does not emit block-level tokens: `BLOCK_MAPPING_START` (type 8), `KEY`, `VALUE`, `BLOCK_SEQUENCE_START`, `BLOCK_END`. When scanning YAML like `"key: val\nfoo: bar"`, the C scanner produces `STREAM_START, BLOCK_MAPPING_START, KEY, SCALAR, VALUE, SCALAR, ...` but the Rust scanner skips directly to `SCALAR` (type 21).

Evidence:
- `token_1_type`: C=8 (BLOCK_MAPPING_START), Rust=21 (SCALAR)
- Rust also emits spurious `token_1_scalar_len: 3` and `token_1_scalar_style: 1`
- Tokens 4-7 and `scan_token_count` are missing (scanner crashes before reaching them)

This is the **root cause** of ~300 missing tests: without correct tokens, the parser produces wrong events, the loader builds wrong documents, and all parse-dependent tests fail (mapping, sequence, flow_seq, flow_map, anchor_alias, double_quoted, single_quoted, literal_block, folded_block, explicit_doc, version_dir, tag_dir, nested, multi_doc, empty_doc, null_value, int_scalar, tagged_scalar, parse_error, custom_read_handler, roundtrip, and full load tests).

## What Needs to Change
Compare the Rust `yaml_parser_fetch_next_token` (lib.rs:6060) with the C `yaml_parser_fetch_next_token` in scanner.c. The Rust version is likely missing the block-level token logic:
1. `yaml_parser_unroll_indent` — emitting BLOCK_END tokens when indent decreases
2. `yaml_parser_roll_indent` — emitting BLOCK_MAPPING_START / BLOCK_SEQUENCE_START when indent increases
3. Fetch functions for KEY, VALUE, block-entry tokens
4. Simple key handling that triggers implicit block collection starts

The C scanner uses an indent stack (`parser->indents`) and simple key tracking (`parser->simple_keys`) to decide when to emit block tokens. The Rust version must replicate this logic exactly.

## Success Criteria
- `token_1_type: 8` (BLOCK_MAPPING_START) — matches C
- All 8 tokens produced for `"key: val\nfoo: bar"` input
- `scan_token_count: 8` — matches C
- No FAULT on parser_scan
- All parser-dependent test groups produce output (mapping, sequence, flow_seq, flow_map, etc.)
