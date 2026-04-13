# Goal 1: Fix double-free in scanner token cleanup

## Function(s)
- `yaml_parser_delete` (cleanup of remaining tokens)
- `yaml_token_delete` (individual token destruction)
- Scanner token-producing functions: `yaml_parser_scan_anchor`, `yaml_parser_scan_tag`,
  `yaml_parser_scan_plain_scalar`, `yaml_parser_scan_block_scalar`,
  `yaml_parser_scan_flow_scalar`, `yaml_parser_scan_directive`

## Source Files
- **C sources**: `/home/leochanj/Desktop/libyaml/src/scanner.c`, `/home/leochanj/Desktop/libyaml/src/api.c`
- **Rust source**: `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s5/src/lib.rs`

## What's Wrong
49 test functions crash with SIGNAL 6 (SIGABRT) due to `free(): double free detected
in tcache 2`. The double-free is systemic, affecting scanner, parser, and loader tests.
The root cause is in the memory ownership lifecycle of token string data (anchor, tag,
value, handle, prefix pointers allocated by the scanner).

Potential double-free scenarios:
1. Scanner allocates string data for a token → token is consumed by parser (skip_token
   moves queue head) → string pointer is transferred to event/node → both the original
   token cleanup path AND the event/document cleanup path free the same pointer.
2. Error paths in scanner functions may free string data and then also leave the pointer
   in the token, causing `yaml_token_delete` to free it again during parser cleanup.
3. `yaml_parser_delete` at lib.rs:1869 cleans up remaining tokens in the queue
   (head→tail), but if any token's string data was already freed elsewhere, this causes
   double-free.

## What Needs to Change
1. Audit every scanner function that allocates string data (yaml_malloc/yaml_strdup) to
   ensure the pointer is either:
   - Stored in the token AND not freed on the success path, OR
   - Freed on error AND nulled in the token to prevent double-free
2. Compare each scanner function line-by-line with the C original in scanner.c
3. Verify that `yaml_token_delete` (lib.rs:2725 region) matches the C `yaml_token_delete`
   in api.c for every token type
4. Verify that `yaml_parser_delete` (lib.rs:1869) properly cleans up only tokens that
   still own their string data

## Success Criteria
- All 49 SIGNAL 6 (SIGABRT) test function crashes are eliminated
- No `free(): double free detected` or `corrupted double-linked list` errors in test output
- Tests that were MISSING due to crashes now produce output
- `valgrind` or ASAN shows no double-free or use-after-free errors
