# Goal 4: Verify and fix remaining event/node/token mismatches after scanner fix

## Functions
- `yaml_parser_parse` and sub-functions (parser)
- `yaml_parser_load` and sub-functions (loader)
- Scanner output (tokens)

## C source file
- `/home/leochanj/Desktop/libyaml/src/scanner.c`
- `/home/leochanj/Desktop/libyaml/src/parser.c`
- `/home/leochanj/Desktop/libyaml/src/loader.c`

## Rust source file
`/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/lib.rs`

## What's wrong
After fixing Goals 1–2 (scanner byte-offset bug), re-run the differential tests. The following mismatches should be checked:

1. **Token mismatches (12):** Scalar values, lengths, and styles differ. Most are caused by the corrupted queue indexing (Goal 1). After the fix, verify tokens match.

2. **Event mismatches (64):** Mapping style (`style=1` block vs `style=2` flow), scalar values differ. These are downstream of the scanner producing wrong tokens. After scanner fix, verify events match.

3. **Node mismatches (11):** Node types wrong (`type=3` mapping vs `type=1` scalar), values differ. Downstream of parser events. After scanner + parser fix, verify nodes match.

4. **bridge_load_node_type (1):** Returns `type=1` instead of `type=3`. Downstream of loader getting wrong events.

5. **MISSING outputs (116):** These are from tests that crashed. After the crash fix, these tests should produce output. Verify the output matches C.

## What needs to change
1. Apply Goals 1 and 2 first.
2. Re-run the differential tests.
3. If any mismatches remain, investigate the specific parser/loader function producing wrong output by comparing the Rust code against the C original.

## Success Criteria
- All 19 previously-crashing tests produce complete output without SIGABRT.
- Token output matches C for all scanner tests (scan_basic, scan_flow, etc.).
- Event output matches C for all parser tests (parse_basic, parse_mapping, parse_nested, etc.).
- Node output matches C for all loader tests (load_basic, load_mapping, load_nested).
- bridge_load_node_type outputs `3` (matching C).
- The 116 previously-MISSING test outputs are now present and match C.
