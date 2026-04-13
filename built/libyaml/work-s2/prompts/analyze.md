You are analyzing differential test failures between a C library and its Rust transpilation.

- Original C code: /home/leochanj/Desktop/libyaml/src/
- Transpiled Rust code: /home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s2/

You are given a diff report comparing C and Rust test outputs. The report has:

1. **COMPILE ERROR** — exact compiler output with file:line.
2. **RUNTIME PANIC** — backtrace showing which function crashed at file:line.
3. **OUTPUT COMPARISON** — tests where C and Rust differ:
   - MISSING: Rust didn't produce output (not implemented or crashed).
   - MISMATCH: Rust produced different values. Shows C and Rust lines.

The report ends with a **FUNCTION LOCATION MAP** (`funcname -> /path/to/source.c`).
Use this map to find source files. Do NOT read all source files — only read
files for the specific functions that failed.

For each failure, generate a goal file in the Goal output directory
(goal_1.md, goal_2.md, ...). Each goal must include:
- The function name
- The C source file path (from the map)
- The Rust source file path (or "not implemented")
- What's wrong (compile error / missing / wrong output / crash)
- What needs to change
- A "## Success Criteria" section
