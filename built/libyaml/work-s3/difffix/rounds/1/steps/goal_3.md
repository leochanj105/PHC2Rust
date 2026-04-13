# Goal 3: Fix bridge stubs for composer error functions

## Functions
- `bridge_yaml_parser_set_composer_error`
- `bridge_yaml_parser_set_composer_error_context`
- `bridge_yaml_parser_delete_aliases`

## C source file
`/home/leochanj/Desktop/libyaml/src/loader.c` (composer error functions)

## Rust source file
`/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/test_bridge.rs` (lines 1186–1214)

## What's wrong
Three bridge functions are stubbed (return 0 / do nothing) instead of calling the real Rust implementations that exist in `lib.rs`:

1. `bridge_yaml_parser_set_composer_error` (test_bridge.rs:1191) — returns 0 instead of calling `crate::yaml_parser_loader_set_composer_error` (lib.rs:5089)
2. `bridge_yaml_parser_set_composer_error_context` (test_bridge.rs:1200) — returns 0 instead of calling `crate::yaml_parser_loader_set_composer_error_context` (lib.rs:5100)
3. `bridge_yaml_parser_delete_aliases` (test_bridge.rs:1211) — does nothing instead of calling `crate::loader_delete_aliases` (lib.rs:5115)

This causes:
- `composer_error` test: C outputs `composer_error=5` (YAML_COMPOSER_ERROR), Rust outputs `composer_error=0`
- `composer_ctx_error` test: C outputs `composer_ctx_error=5`, Rust outputs `composer_ctx_error=0`

## What needs to change
Replace the stubs with calls to the real implementations:

```rust
#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_composer_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_loader_set_composer_error(parser, problem, problem_mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_composer_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_loader_set_composer_error_context(parser, context, context_mark, problem, problem_mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_delete_aliases(
    parser: *mut yaml_parser_t,
) {
    crate::loader_delete_aliases(parser)
}
```

Note: The real functions (`yaml_parser_loader_set_composer_error`, etc.) are currently declared `unsafe fn` (not `pub`). They may need to be made `pub(crate)` for the bridge to call them.

## Success Criteria
- `composer_error` test outputs `composer_error=5` (matching C).
- `composer_ctx_error` test outputs `composer_ctx_error=5` (matching C).
- `bridge_yaml_parser_delete_aliases` calls the real cleanup function.
- The stub comment block at the bottom of test_bridge.rs is removed.
