# libyaml transpile audit

Manual code audit of Claude's C→Rust transpilation, conducted 2026-04-11.

## Context

- C source: `/home/leochanj/Desktop/libyaml/src/` (9 .c files, ~5000 lines)
- Rust transpile: `PHC2Rust/built/libyaml/rust-baseline/src/` (12 .rs files, 13500 lines)
- Reference Rust port: `unsafe-libyaml` crate v0.2.11 (by David Tolnay, on crates.io)
- Average function body similarity to unsafe-libyaml: **53.6%** (not a copy)

## Finding 1: 39 missing assert checks

C libyaml has 99 `assert()` calls across its source files. Claude's Rust
transpilation has only 83 active `assert!()` calls.

- **13 asserts commented out** (exist as `/* assert(...) */` but don't execute)
- **26 asserts completely absent** (no Rust equivalent at all)

### Where they are

**writer.rs (3 commented):** null-emitter, null-write-handler, null-encoding
checks at the top of `yaml_emitter_flush`. If a caller passes an
uninitialized emitter, C aborts at the assert; Rust proceeds to null
dereference (UB).

**dumper.rs (8 commented):** preconditions on `yaml_emitter_open`,
`yaml_emitter_close`, `yaml_emitter_dump` — checks that emitter is not
null, is/isn't opened, document is not null. Plus `assert(0)` for an
"impossible" branch in `yaml_emitter_dump_node`.

**emitter.rs (2 commented):** `assert(1)` in "invalid state" and
"impossible" branches in the emitter state machine.

**api.c → api.rs (14 missing):** precondition checks including:
- `assert(!parser->read_handler)` before setting input (3 call sites)
- `assert(!emitter->write_handler)` before setting output (3 call sites)
- `assert(!parser->encoding)` / `assert(!emitter->encoding)` (2)
- `assert(tag_directive->handle)` / `assert(tag_directive->prefix)` (4)
- `assert(document->nodes.start[N].type == YAML_*_NODE)` (2)

**loader.c → loader.rs (6 missing):** state invariants:
- `assert(event.type == YAML_DOCUMENT_START_EVENT)` (1)
- `assert(event.type == YAML_STREAM_START_EVENT)` (1)
- Node-type assertions in load_sequence/load_mapping (4)

**reader.c → reader.rs (1 missing):** `assert(parser->read_handler)`

### Impact

These asserts fire on **API misuse** (caller bugs), not on input data.
Correctly-written drivers never violate these preconditions, which is why
300,000 differential test cases showed 0 diffs. But if the Rust library is
used by buggy application code, C would abort with a clear assert message
while Rust would silently enter undefined behavior.

### Detectability

Not detectable by differential testing (requires API-misuse drivers).
Detectable by: code audit, static analysis, or custom test drivers that
deliberately violate preconditions.

## Finding 2: UTF-16 surrogate encoding bug (shared with C)

In `yaml_emitter_flush` (writer.c:119 / writer.rs:149), the low-surrogate
byte computation for UTF-16 output is wrong:

```c
// C (and Rust, faithfully translated):
emitter->raw_buffer.last[high+2] = 0xDC + ((value >> 8) & 0xFF);

// Correct:
emitter->raw_buffer.last[high+2] = 0xDC + ((value & 0x3FF) >> 8);
```

For U+1F600 (😀): C/Rust produce low surrogate 0xD200, correct is 0xDE00.

This is a **pre-existing bug in upstream libyaml**, not a transpile error.
Claude faithfully reproduced it. Affects all codepoints above U+103FF in
UTF-16 output mode. Never caught because no test input uses UTF-16 encoding.

## Finding 3: Training-data contamination evidence

| metric | value |
|---|---|
| Average function body similarity to unsafe-libyaml | 53.6% |
| Functions >80% similar | 26/200 (13%) |
| Functions <50% similar | 83/200 (42%) |
| Shared function names | 190/213 |
| Claude-only function names | 65 |

Conclusion: Claude did NOT copy unsafe-libyaml, but was clearly informed by
having seen it during training. The code structure diverges (different macro
signatures, different module layout, different error handling) but the
function names and overall approach are highly correlated.

**libyaml is not a valid uncontaminated benchmark** for measuring AI
transpilation capability. Results should be reported with this caveat.

## Finding 4: Functional equivalence — exhaustive search found 0 diffs

| test approach | cases | diffs |
|---|---|---|
| yaml-test-suite × 13 drivers | 4,424 | 0 |
| oss-fuzz corpus × 20 drivers | 300,000 | 0 |
| Targeted: 5 config setters (previously uncovered) | 1 | 0 |
| Targeted: `yaml_parser_set_encoding` | 1 | 0 |
| Targeted: `yaml_set_max_nest_level` | 1 | 0 |
| Targeted: UTF-16 LE encoding | 1 | 0 |
| Targeted: UTF-16 BE + emoji (U+1F600) | 1 | 0 |
| Targeted: UTF-16 BE + unicode flag + emoji | 1 | 0 |
| Manual code audit (~15 functions) | — | 0 functional diffs |

**The UTF-16 surrogate pair bug (Finding 2) is dead code:** the emitter
always escapes non-BMP characters as `\U0001XXXX` before they reach the
UTF-8→UTF-16 conversion in `yaml_emitter_flush`. So the buggy surrogate
encoding branch never executes in either C or Rust.

**Remaining untested paths (narrow):**
- Pure OOM paths (require malloc-failure injection — not possible via input)
- Some deep error-recovery branches in scanner/parser
- The dead UTF-16 surrogate code

**Assessment:** At 300k+ differential test cases + 8 targeted tests on
previously-uncovered paths + manual audit, the transpile is functionally
equivalent on every reachable code path we could exercise. The most likely
explanation is training-data contamination (see Finding 3) — Claude "knows"
what correct libyaml behavior looks like from having seen both the C source
and the unsafe-libyaml Rust crate during training.
