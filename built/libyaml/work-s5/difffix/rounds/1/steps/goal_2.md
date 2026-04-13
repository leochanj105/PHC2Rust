# Goal 2: Fix assertion failure in yaml_parser_load_document

## Function(s)
- `yaml_parser_load_document` (lib.rs:5127)
- `yaml_parser_load` (lib.rs:5022)

## Source Files
- **C source**: `/home/leochanj/Desktop/libyaml/src/loader.c` (lines 86-210)
- **Rust source**: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs` (lines 5022-5161)

## What's Wrong
The `loader_stream_end_reuse` test triggers an assertion failure at lib.rs:5131:
```
assertion failed: (*event).type_ == YAML_DOCUMENT_START_EVENT
```
The test name suggests it's testing reuse of a parser after stream end. The assertion
fires because `yaml_parser_parse` returns an event that is NOT `YAML_DOCUMENT_START_EVENT`
when `yaml_parser_load` calls `yaml_parser_load_document`.

In the C code (`yaml_parser_load`, loader.c:86-132):
1. After `stream_end_produced` check, it calls `yaml_parser_parse(&event)`
2. If `event.type == YAML_STREAM_END_EVENT`, it returns 1 (success, empty document)
3. Otherwise it proceeds to `yaml_parser_load_document` which asserts DOCUMENT_START

The Rust code (lib.rs:5022-5079) follows the same logic. The issue may be:
1. Memory corruption from double-free (Goal 1) corrupts event type
2. Or: `yaml_parser_parse` returns a zeroed event (type 0) when `stream_end_produced`
   is set and `error != YAML_NO_ERROR` or `state == YAML_PARSE_END_STATE` — the early
   return at lib.rs:3812-3817 returns 1 (success) with a zeroed event. The caller then
   passes this zeroed event to `yaml_parser_load_document` which asserts DOCUMENT_START.

In the C code, `yaml_parser_parse` (parser.c) has the same early return, but the C
`yaml_parser_load` checks `event.type == YAML_STREAM_END_EVENT` and returns 1 for that
case. A zeroed event (type 0 = YAML_NO_EVENT) falls through to `yaml_parser_load_document`.

## What Needs to Change
1. First fix Goal 1 (double-free) — memory corruption may be the root cause
2. If the assertion still fails after fixing Goal 1, investigate the parser state
   machine behavior when a parser is reused after stream end
3. Compare the `yaml_parser_parse` early-return behavior: when `state ==
   YAML_PARSE_END_STATE`, the C and Rust both return success with a zeroed event. The
   Rust `yaml_parser_load` should handle this case (zeroed event = not STREAM_END and
   not DOCUMENT_START) by returning success with an empty document, matching C behavior.
4. Verify the `stream_end_produced` flag is correctly set and checked

## Success Criteria
- `loader_stream_end_reuse` test passes without assertion failure
- All loader tests (`loader_prestarted_stream`, `loader_explicit_tags`,
  `loader_sequence_mapping_tags`, `loader_anchor_tag_combinations`,
  `loader_flow_seq_and_map`, `loader_mapping_anchor_alias`,
  `loader_explicit_tag_on_scalar`) produce correct output
- The `second_load_ok`, `second_root_null`, `stream_end_consumed` assertions pass
