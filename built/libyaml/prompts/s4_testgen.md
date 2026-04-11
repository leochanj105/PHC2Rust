You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

Generate a test_suite.c that exercises the YAML parser and emitter,
printing events, scalar values, and emitted output as plain text so that
C and Rust outputs can be diffed line-by-line.

Cover ALL functions in this library, including internal static functions
that are not declared in headers.

## Coverage feedback

This is a coverage-guided test generation round. You will receive **function
coverage feedback**: a list of functions that existing tests do not yet call.

Read these files in the working directory:

- test_suite.c             — current test file; ADD to it, do not remove existing tests
- uncovered_functions.md   — functions not yet called by any test.
  Format: one function per line. [static] prefix means internal linkage.

- crash_summary.md         — if non-empty, fix crashing tests FIRST

For each function in uncovered_functions.md, write a test that calls it.
Read the corresponding .c file if you need to understand what inputs are valid.

## Static functions

A pre-built test_bridge.h is provided in the working directory, declaring
`bridge_*` wrapper functions for the library's static C functions.

    (See test_bridge.h for the full list — too many to inline here.)

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.h or any bridge_* implementation files.

## Rules
- Do NOT remove or modify existing test cases.
- All tests must be deterministic.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
