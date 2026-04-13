# Compact Divergences — Round 1

## Summary
- 212 tests failed (191 passed)
- 12 test functions crash with SIGABRT (double free / memory corruption)
- 184 tests missing (output lost due to crashes)
- 28 tests produce wrong output

## Root Cause Analysis

### RC-1: Pointer arithmetic bug in token queue element count (CRITICAL)

**Location**: `lib.rs:6037-6038` and `lib.rs:6213-6214`

Two places compute token queue length using raw address subtraction:
```rust
(*parser).tokens.tail as usize - (*parser).tokens.head as usize
```
This gives **byte offset**, not **element count**. The C original uses:
```c
parser->tokens.tail - parser->tokens.head
```
which gives element count via C pointer arithmetic.

**Impact**: `yaml_parser_save_simple_key` stores an inflated `token_number`.
When `yaml_parser_fetch_value` later computes the insertion index for
BLOCK_MAPPING_START tokens, it gets a hugely wrong index. `queue_insert`
then does a `memmove` with a wrapped-around count, corrupting heap memory.

This single bug causes:
1. Block mappings not detected → parser emits SCALAR instead of MAPPING_START
2. Memory corruption from bogus queue_insert → double free on cleanup → SIGABRT
3. All 12 FAULT tests crash after encountering their first block mapping
4. All MISSING output is from tests that run after crashes
5. MISMATCH results from parser treating mapping keys as standalone scalars

### Affected Tests

| Category | Count | Tests |
|----------|-------|-------|
| FAULT (crash) | 12 | parse_mapping, parse_complex, parse_anchors, parse_tags, parse_directive, parse_multidoc, parse_error_handling, scan_tokens, load_document, roundtrip, unicode_scalars, canonical_emitter |
| MISSING | 184 | Output lost after crashes (event:, scalar, mapping, sequence, token_type, etc.) |
| MISMATCH | 28 | caught_error (wrong error msg), event: (shifted values), token_type (missing BLOCK_MAPPING_START=8, shows SCALAR=21) |

### Divergence Details

1. **FAULT tests**: All crash with "free(): double free detected in tcache 2" or "corrupted unsorted chunks" after the parser encounters a block mapping key followed by `:`.

2. **token_type mismatch**: C emits token_type=8 (BLOCK_MAPPING_START_TOKEN), Rust skips it entirely. C emits token_type=16 (KEY) then 17 (VALUE); Rust shifts by one position.

3. **caught_error mismatch**: C reports "did not find expected ',' or ']'" (correct flow context error), Rust reports "did not find expected \<document start\>" (because the mapping wasn't recognized, parser expected document-level content).

4. **event:: mismatches**: Parser treats mapping key as document scalar, shifting all subsequent event values.
