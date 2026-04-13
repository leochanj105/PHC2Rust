# Goal 3: Fix test bridge stubs — composer error and delete_aliases

## Function
`bridge_yaml_parser_set_composer_error`, `bridge_yaml_parser_set_composer_error_context`, `bridge_yaml_parser_delete_aliases`

## Source Files
- C source: `/home/leochanj/Desktop/libyaml/src/parser.c` (composer error), `/home/leochanj/Desktop/libyaml/src/loader.c` (delete_aliases)
- Rust source: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/lib.rs`
  - `yaml_parser_loader_set_composer_error` at line 5089
  - `yaml_parser_loader_set_composer_error_context` at line 5100
- Rust test bridge (stubs): `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/test_bridge.rs` lines 1186-1214

## What's Wrong
Three functions in the test bridge are stubs that do nothing instead of calling the real Rust implementations:

1. `bridge_yaml_parser_set_composer_error` — returns 0 without calling `crate::yaml_parser_loader_set_composer_error`. Result: `composer_error_type` is 0 (NO_ERROR) instead of 5 (COMPOSER_ERROR).

2. `bridge_yaml_parser_set_composer_error_context` — returns 0 without calling `crate::yaml_parser_loader_set_composer_error_context`. Result: `composer_error_ctx_type` is 0 instead of 5.

3. `bridge_yaml_parser_delete_aliases` — empty body, does nothing. The real implementation likely exists in the Rust code but the bridge doesn't call it. This contributes to the `bridge_delete_aliases` FAULT (Goal 2).

## What Needs to Change
Replace the three stubs in test_bridge.rs (lines 1190-1214) with real bridge calls:

1. `bridge_yaml_parser_set_composer_error` should call `crate::yaml_parser_loader_set_composer_error(parser, problem, problem_mark)`
2. `bridge_yaml_parser_set_composer_error_context` should call `crate::yaml_parser_loader_set_composer_error_context(parser, context, context_mark, problem, problem_mark)`
3. `bridge_yaml_parser_delete_aliases` should call the real Rust delete_aliases function (find/implement `yaml_parser_delete_aliases` in lib.rs)

Note: The function names differ between the bridge (yaml_parser_set_composer_error) and the implementation (yaml_parser_loader_set_composer_error). Verify the correct mapping by checking the C source.

## Success Criteria
- `composer_error_type: 5` — matches C output
- `composer_error_ctx_type: 5` — matches C output
- `delete_aliases_ok: 1` still produced, and `bridge_delete_aliases` does not FAULT (combined with Goal 2 fix)
