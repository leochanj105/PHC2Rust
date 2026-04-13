# Goal 2: Fix flow_level off-by-one in increase/decrease flow level

## Function
`yaml_parser_increase_flow_level`, `yaml_parser_decrease_flow_level`

## C source
- `/home/leochanj/Desktop/libyaml/src/scanner.c` (search for `increase_flow_level` and `decrease_flow_level`)

## Rust source
- `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s4/src/lib.rs` (search for `yaml_parser_increase_flow_level` and `yaml_parser_decrease_flow_level`)

## What's wrong
MISMATCH in `test_bridge_parser_flow_levels`:
- C output: `increase_flow_ok: 1 level: 1` / `decrease_flow_ok: 1 level: 0`
- Rust output: `increase_flow_ok: 1 level: 2` / `decrease_flow_ok: 1 level: 1`

The Rust flow_level is consistently 1 higher than C. The test initializes a parser (flow_level starts at 0), calls increase (should go to 1), then calls decrease (should go back to 0). Rust reports 2 and 1 respectively, suggesting either:
- The parser initializes flow_level to 1 instead of 0, or
- `increase_flow_level` increments before the check instead of after, or
- Some other off-by-one in the flow_level logic

## What needs to change
Compare the Rust `yaml_parser_increase_flow_level` and `yaml_parser_decrease_flow_level` with the C versions in scanner.c. Fix the off-by-one so flow_level values match C exactly.

## Success Criteria
- `increase_flow_ok: 1 level: 1` (matches C)
- `decrease_flow_ok: 1 level: 0` (matches C)
