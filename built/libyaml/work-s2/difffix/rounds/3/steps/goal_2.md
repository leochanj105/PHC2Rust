# Goal 2: Fix bridge_yaml_parser_set_composer_error_context stub

## Function
`bridge_yaml_parser_set_composer_error_context`

## C source
`/home/leochanj/Desktop/libyaml/src/loader.c` — wraps `yaml_parser_set_composer_error_context` (line 154)

## Rust source
`/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/src/test_bridge.rs` — stub at line 1200

## What's wrong
The Rust bridge stub is a no-op that returns 0 without calling the actual implementation.
The C bridge calls `yaml_parser_set_composer_error_context` which sets `parser->error = YAML_COMPOSER_ERROR` (5),
plus `parser->context`, `parser->context_mark`, `parser->problem`, and `parser->problem_mark`.
The Rust stub ignores all parameters and does nothing, so `parser.error` stays 0.

The real Rust implementation already exists as `yaml_parser_loader_set_composer_error_context` in `lib.rs:5100`.

**C output:** `composer_error_ctx_type: 5`
**Rust output:** `composer_error_ctx_type: 0`

## What needs to change
Replace the stub in `test_bridge.rs` (line 1200-1208) with a real implementation that calls
`yaml_parser_loader_set_composer_error_context(parser, context, context_mark, problem, problem_mark)`
and returns its result.

The function signature:
```rust
pub unsafe extern "C" fn bridge_yaml_parser_set_composer_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    yaml_parser_loader_set_composer_error_context(parser, context, context_mark, problem, problem_mark) as c_int
}
```

## Success Criteria
- `composer_error_ctx_type` test outputs `5` (matching C output)
- The bridge function delegates to `yaml_parser_loader_set_composer_error_context`
- All 573 tests pass with zero diff
