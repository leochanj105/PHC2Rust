# Goal 5: Fix yaml_parser_load root_type mismatch

## Function(s)
`yaml_parser_load`, `yaml_parser_load_document`, `yaml_parser_load_nodes`, `yaml_parser_load_scalar`, `yaml_parser_load_sequence`

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/loader.c`
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs`

## What's Wrong
test_parser_load shows:
- C:    `root_type: 3` (YAML_SEQUENCE_NODE)
- Rust: `root_type: 1` (YAML_SCALAR_NODE)

The test feeds `[a, b, c]` (a YAML sequence) to the parser/loader. The C code correctly identifies the root node as a sequence (type 3), but the Rust code identifies it as a scalar (type 1).

This is likely caused by one of:
1. The parser producing wrong events (SCALAR instead of SEQUENCE_START) due to scanner bugs from Goal 2/4, causing the loader to create a scalar node instead of a sequence node.
2. The `yaml_parser_load_nodes` function incorrectly dispatching to `yaml_parser_load_scalar` instead of `yaml_parser_load_sequence`.
3. Node type constants being mapped differently between C and Rust.

Note: This test also crashes with double-free after printing `root_type: 1` (Goal 1).

## What Needs to Change
1. Fix Goals 1, 2, and 4 first — the scanner/parser bugs likely cause the loader to receive wrong events.
2. If root_type is still wrong after fixing scanner/parser, compare:
   - Rust `yaml_parser_load_nodes` with C `yaml_parser_load_nodes` (loader.c) — check the event type dispatch
   - Node type enum values: verify `YAML_SCALAR_NODE = 1`, `YAML_SEQUENCE_NODE = 2`, `YAML_MAPPING_NODE = 3` match C's `yaml.h` definitions
   - The `yaml_parser_load_sequence` function — verify it creates nodes with the correct type

## Success Criteria
test_parser_load output matches C exactly:
```
  root_type: 3
  node1_ok: 1
```
No crash (SIGABRT resolved by Goal 1).
