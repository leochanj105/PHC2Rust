# Goal 3: Fix flow level off-by-one

## Function(s)
`yaml_parser_increase_flow_level`, `yaml_parser_decrease_flow_level`, and/or parser initialization

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/scanner.c` (yaml_parser_increase_flow_level, line 1162; yaml_parser_decrease_flow_level, line 1188)
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs` (yaml_parser_increase_flow_level, line 6251; yaml_parser_decrease_flow_level, line 6283)

## What's Wrong
test_bridge_parser_flow_levels shows:
- C:    `increase_flow_ok: 1 level: 1` / `decrease_flow_ok: 1 level: 0`
- Rust: `increase_flow_ok: 1 level: 2` / `decrease_flow_ok: 1 level: 1`

After one call to `yaml_parser_increase_flow_level`, the flow_level should be 1, but Rust reports 2. After one decrease, it should be 0, but Rust reports 1. This means either:
1. `parser->flow_level` starts at 1 instead of 0 after initialization, OR
2. The test setup calls some function that bumps flow_level before the explicit increase/decrease test, OR
3. `yaml_parser_increase_flow_level` increments twice, OR
4. Some other initialization path sets flow_level to a non-zero value.

The `yaml_parser_increase_flow_level` and `yaml_parser_decrease_flow_level` functions themselves look correct (matching C logic). The issue is likely in parser initialization or the test's parser setup triggering an extra flow level increment.

## What Needs to Change
Investigate what sets `parser->flow_level` before the test's explicit increase/decrease calls. Check:
1. `yaml_parser_initialize` — should memset(0) the parser, so flow_level starts at 0
2. Any function called during parser setup in the test (e.g., `yaml_parser_set_input_string` might trigger a fetch that calls `yaml_parser_fetch_flow_collection_start`)
3. The `bridge_yaml_parser_increase_flow_level` test harness — check what happens between parser init and the flow level call

Fix whichever path incorrectly sets flow_level to a non-zero initial value.

## Success Criteria
test_bridge_parser_flow_levels output matches C exactly:
```
  increase_flow_ok: 1 level: 1
  decrease_flow_ok: 1 level: 0
```
