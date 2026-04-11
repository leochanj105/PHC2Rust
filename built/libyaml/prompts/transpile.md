You are transpiling a C library to Rust.

- Original C source: /home/leochanj/Desktop/libyaml/src/
- Output Rust project: __RUST_DIR__/

Read ALL .c and .h files in the C source tree.

Produce a complete, equivalent Rust library that:

1. Reproduces the exact behavior of the C code for all inputs.
2. Exposes every non-static C function as `#[no_mangle] pub extern "C"` using
   the **same name** as the C function. Example:
   ```rust
   #[no_mangle]
pub extern "C" fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32 {
       // ... Rust implementation ...
   }
   ```
3. Builds with `cargo build` (produce Cargo.toml, src/lib.rs, etc.).

## What "transpile" means

Re-implement every C function body in Rust with equivalent logic. Loops,
conditionals, arithmetic, pointer arithmetic, array indexing — all translated.

The following are NOT acceptable and will be rejected:
- Calling back into the original C library via `extern "C"` or `#[link(...)]`
- FFI bindings (e.g. bindgen output)
- `unimplemented!()`, `todo!()`, or stubs

The Rust library must be entirely self-contained. It must not reference or link
against the original C source at all.

## Rules
- Do NOT fix bugs in the C code — reproduce its behavior exactly.
- Preserve exact order of operations, error checks, and validation.
- If the C code uses global/static state, reproduce it faithfully in Rust.

## Cargo.toml

```toml
[lib]
crate-type = ["staticlib", "cdylib"]
```

Both crate types are required. Do NOT add `#![no_std]` — staticlib requires std.
