# Compact Divergences — Round 1

## Summary
- Tests passed: 249 / 573
- Tests failed: 324
- FAULTs (crashes): 7 test functions (all SIGABRT — double free)
- Root causes: 3 independent bugs

---

## Divergence 1: Scanner missing block-level tokens (ROOT CAUSE)

**Scope:** 300+ missing tests across parser_scan, parser_parse, parser_load, custom_read_handler, roundtrip

The scanner does not emit block-level tokens: BLOCK_MAPPING_START (8), KEY, VALUE, BLOCK_SEQUENCE_START, BLOCK_END. For input `"key: val\nfoo: bar"`, the C scanner produces:

```
STREAM_START, BLOCK_MAPPING_START(8), KEY, SCALAR, VALUE, SCALAR, KEY, SCALAR, VALUE, SCALAR, BLOCK_END, STREAM_END
```

The Rust scanner produces:

```
STREAM_START, SCALAR(21), ... then crashes
```

**Evidence:**
- `token_1_type`: C=8 (BLOCK_MAPPING_START), Rust=21 (SCALAR)
- Rust emits `token_1_scalar_len: 3` and `token_1_scalar_style: 1` (treating key as bare scalar)
- Missing: token_4 through token_7, scan_token_count
- Cascading: parser gets wrong tokens → mapping_event_2 is 6 (SCALAR) instead of 9 (MAP_START)
- Cascading: loader builds wrong tree → load_doc_0_root_type is 1 (SCALAR) instead of 3 (MAPPING)
- Cascading: all parse-dependent test groups missing (sequence, flow_seq, flow_map, anchor_alias, double_quoted, single_quoted, literal_block, folded_block, explicit_doc, version_dir, tag_dir, nested, multi_doc, empty_doc, null_value, int_scalar, tagged_scalar, parse_error)
- Cascading: custom_read_handler and roundtrip produce partial/wrong events

**Affected C functions:** `yaml_parser_fetch_next_token` in scanner.c

---

## Divergence 2: Double-free in parser/scanner cleanup (7 FAULTs)

**Scope:** parser_scan, parser_parse, parser_load, max_nest_level, bridge_delete_aliases, custom_read_handler, roundtrip

All 7 test functions crash with SIGABRT after partial execution. stderr shows: `free(): double free detected in tcache 2` (7 occurrences).

This is a memory management bug in cleanup/delete functions. Even tests that produce correct output up to a point (e.g., max_nest_level, bridge_delete_aliases) crash during cleanup.

**Evidence:**
- 7 FAULT lines with SIGNAL 6
- r_stderr.txt: 7× "free(): double free detected in tcache 2"
- bridge_delete_aliases outputs `delete_aliases_ok: 1` then immediately crashes
- max_nest_level outputs `max_nest_error: 4` then crashes

**Affected C functions:** `yaml_parser_delete`, `yaml_event_delete`, `yaml_document_delete`, `yaml_token_delete` in api.c; possibly scanner/parser cleanup paths in scanner.c/parser.c

---

## Divergence 3: Composer error type not set

**Scope:** 2 tests (composer_error_type, composer_error_ctx_type)

The composer error-setting function does not set the error type correctly.

**Evidence:**
- `composer_error_type`: C=5 (YAML_COMPOSER_ERROR), Rust=0 (YAML_NO_ERROR)
- `composer_error_ctx_type`: C=5 (YAML_COMPOSER_ERROR), Rust=0 (YAML_NO_ERROR)

**Affected C function:** `yaml_parser_set_composer_error` / `yaml_parser_set_composer_error_context` in parser.c (or api.c)
