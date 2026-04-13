# Compact Divergences — Round 1

## Summary

- **Tests passed:** 287
- **Tests failed:** 206 (19 FAULT + 116 MISSING + 90 MISMATCH)

---

## Root Cause 1: Byte-offset vs element-count bug in scanner queue indexing

**Severity:** Critical — causes 19 crashes + cascading failures  
**Affected functions:** `yaml_parser_save_simple_key`, `yaml_parser_fetch_more_tokens`  
**File:** `rust-s3/src/lib.rs`

In `yaml_parser_save_simple_key` (line 6213–6214), `token_number` is computed as:
```rust
(*parser).tokens_parsed + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```
This subtracts raw pointer addresses (byte offset). The C original uses pointer subtraction (`parser->tokens.tail - parser->tokens.head`) which yields element count. For a `yaml_token_t` struct of ~100+ bytes, the byte offset is wildly too large.

Similarly in `yaml_parser_fetch_more_tokens` (line 6037–6038), the need-more-tokens check adds the same wrong byte offset:
```rust
(*sk).token_number == (*parser).tokens_parsed
    + ((*parser).tokens.tail as usize - (*parser).tokens.head as usize)
```
The C code simply checks `simple_key->token_number == parser->tokens_parsed`.

**Effects:**
1. Simple key token numbers are wrong → KEY tokens inserted at garbage queue indices
2. `queue_insert` uses the wrong index → memmove corrupts the queue → memory corruption
3. Scalar token value pointers become invalid → garbage output (`val=⸮ߒZ`)
4. Double-free on cleanup → SIGABRT (19 FAULT tests)
5. Missing BLOCK_MAPPING_START, KEY, VALUE tokens → wrong token/event/node output

**Tests affected:** All 19 FAULT tests + most MISSING/MISMATCH event/token/node outputs.

---

## Root Cause 2: Bridge stubs instead of real implementations

**Severity:** Medium — causes 3 MISMATCH tests  
**File:** `rust-s3/src/test_bridge.rs` (lines 1186–1214)

Three bridge functions are stubbed (return 0 / do nothing) instead of calling the real Rust implementations:

| Stub | Should call | Effect |
|------|-------------|--------|
| `bridge_yaml_parser_set_composer_error` | `yaml_parser_loader_set_composer_error` (lib.rs:5089) | `composer_error` returns 0 instead of 5 |
| `bridge_yaml_parser_set_composer_error_context` | `yaml_parser_loader_set_composer_error_context` (lib.rs:5100) | `composer_ctx_error` returns 0 instead of 5 |
| `bridge_yaml_parser_delete_aliases` | `loader_delete_aliases` (lib.rs:5115) | Not causing visible mismatch but wrong behavior |

**Tests affected:** composer_error (1 mismatch), composer_ctx_error (1 mismatch).

---

## Root Cause 3: Downstream parser/loader mismatches (depends on Root Cause 1)

**Severity:** High — but likely resolves once Root Cause 1 is fixed  

Once the scanner produces correct tokens, the parser should produce correct events, and the loader should produce correct nodes. The observed mismatches:

- `event 9 style=1` (C) vs `event 9 style=2` (Rust) — mapping style BLOCK vs FLOW
- Scalar values in events/nodes differ
- `bridge_load_node_type=3` (C) vs `=1` (Rust) — mapping vs scalar

These are all downstream of the scanner producing wrong token sequences. Fixing Root Cause 1 should resolve most or all of these. Any remaining mismatches would need separate investigation.

**Tests affected:** 64 event mismatches, 11 node mismatches, 12 token mismatches, 1 bridge_load_node_type mismatch.

---

## Divergence Inventory

| Category | Count | Root Cause |
|----------|-------|------------|
| FAULT (crash) | 19 | RC1: byte-offset bug |
| MISSING event | 93 | RC1: crashes prevent output |
| MISSING token | 3 | RC1: crashes prevent output |
| MISSING node | 19 | RC1: crashes prevent output |
| MISSING max_nest_restored | 1 | RC1: crash prevents output |
| MISMATCH token | 12 | RC1: wrong scanner output |
| MISMATCH event | 64 | RC1/RC3: downstream |
| MISMATCH node | 11 | RC1/RC3: downstream |
| MISMATCH bridge_load_node_type | 1 | RC1/RC3: downstream |
| MISMATCH composer_error | 1 | RC2: stub |
| MISMATCH composer_ctx_error | 1 | RC2: stub |
