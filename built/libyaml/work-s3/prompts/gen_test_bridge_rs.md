You are writing a Rust-side test bridge file for a C library that has been
transpiled to Rust.

## Context

- **C bridge header** (authoritative list of 137 bridge functions to produce):
  `__BRIDGE_H_PATH__`
- **Transpiled Rust library** (where the actual implementations live):
  `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/`
- **Output file** — write the complete result to:
  `__OUTPUT_PATH__`

Each `bridge_X` in the C header corresponds to a C `static` function `X` that
lives in one of the library's source files (e.g. `parser.c`, `scanner.c`,
`emitter.c`). In the transpiled Rust, the same function `X` lives in the
corresponding module (e.g. `crate::parser::X`, `crate::scanner::X`).

## Your task

Produce `test_bridge.rs` containing a `#[no_mangle] pub extern "C"` wrapper
for every `bridge_X` declaration in the C header, where each wrapper simply
calls the matching Rust function in the transpiled crate. Template:

```rust
#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_state_machine(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    crate::parser::yaml_parser_state_machine(parser, event)
}
```

## Rules

1. **Use the types and module paths that actually exist in `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/`.**
   Do not guess. Read the relevant `.rs` files to confirm each function's
   signature and module path before writing its wrapper. Types must match the
   Rust function exactly (e.g. if the Rust fn takes `*mut yaml_parser_t`,
   the bridge must also take `*mut yaml_parser_t`, not `*mut libc::c_void`).

2. **All 137 wrappers must be present.** Read `__BRIDGE_H_PATH__` to get the
   full list. Missing any is a failure.

3. **Do NOT modify any file other than the output.** You are not allowed to
   edit `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/src/*.rs`. Visibility promotion (making private `fn`
   into `pub(crate) fn`) is handled by a separate tool that runs later —
   do not worry about whether the target functions are currently reachable.

4. **Verify the file compiles.** Since you cannot edit `/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/`, do
   the verification in a scratch copy:
   - `cp -r /home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3 /tmp/bridge_verify`
   - Copy your draft `test_bridge.rs` into `/tmp/bridge_verify/src/`
   - Add `mod test_bridge;` to `/tmp/bridge_verify/src/lib.rs`
   - For each private `fn X(` in `/tmp/bridge_verify/src/<module>.rs` that
     your bridge calls, change it to `pub(crate) fn X(` in the scratch copy
     only. (This simulates what `promote_visibility.py` will do later.)
   - Run `cargo build --release --lib --manifest-path /tmp/bridge_verify/Cargo.toml`
   - Fix any compile errors by editing the scratch copy of `test_bridge.rs`
     until the build succeeds.
   - When it builds cleanly, write the final `test_bridge.rs` to
     `__OUTPUT_PATH__` and remove `/tmp/bridge_verify`.

5. **No `extern "C"` links back to the C library.** Every call must resolve
   to a Rust path via `crate::module::X(...)`.

6. **Preserve C calling conventions.** Every wrapper must be marked
   `#[no_mangle] pub extern "C"`. Use `unsafe` when the wrapper takes raw
   pointers.

## Style

- Group wrappers by source file with a comment header like
  `// ── parser.c ──`, matching the structure of `test_bridge.h`.
- Include `#![allow(warnings)]` at the top to suppress unused-parameter and
  similar warnings.
- Do not add tests, documentation beyond one brief top-of-file comment,
  or any code unrelated to the 137 bridge wrappers.
