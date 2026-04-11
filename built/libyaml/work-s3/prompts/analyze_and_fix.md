You are fixing a transpiled Rust math library to match the original C library's behavior.

- Original C code: /home/leochanj/Desktop/libyaml/src/
- Transpiled Rust code: /home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s3/

You are given a diff report comparing C and Rust test outputs. The report has:

1. **COMPILE ERROR** — exact compiler output with file:line.
2. **RUNTIME PANIC / TIMEOUT** — crash or hang info with the last output line.
3. **OUTPUT COMPARISON** — tests where C and Rust differ:
   - MISSING: Rust didn't produce output (not implemented, crashed, or hung).
   - MISMATCH: Rust produced different values. Shows both C and Rust lines.

The report ends with a **FUNCTION LOCATION MAP** (`funcname -> /path/to/source.c`).
Use this map to find source files. Do NOT read all source files — only read
files for the specific functions that failed.

## Your task

Read the diff report. Identify the most impactful failures (prioritize crashes
and hangs over mismatches — a crash blocks all subsequent tests). Then fix the
Rust code directly. Fix up to 5 functions per call.

For each failure:
1. Read the C source file (use the function location map).
2. Read the corresponding Rust source.
3. Fix the Rust code:
   - Missing function: implement it by translating the C code. Add a
     `#[no_mangle] pub extern "C"` export in lib.rs.
   - Wrong output: find the bug and fix it to match the C code.
   - Crash/hang: find the infinite loop or panic and fix it.
   - Compile error: fix the syntax/type error.

## Rules
- Reproduce the C behavior exactly — output must match bitwise.
- Do NOT add `extern "C"` declarations linking back to C code.
- Do NOT add `#![no_std]`. Keep `crate-type = ["staticlib", "cdylib"]`.
- Do NOT modify test_bridge.rs — it is test infrastructure.
