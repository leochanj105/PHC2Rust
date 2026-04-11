You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

TODO: describe what the test generator should produce for libyaml.
TODO: e.g. "Generate a test_suite.c that exercises the YAML parser and emitter,
TODO: printing events, scalar values, and emitted output as plain text so that
TODO: C and Rust outputs can be diffed line-by-line."

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

A pre-built test_bridge.c and test_bridge.h are provided in the working directory.
They expose these static functions as callable bridge wrappers:

TODO: run framework/scripts/gen_bridges.py against libyaml/src/ with a
TODO: coverage-instrumented binary to get the authoritative list, then paste
TODO: the bridge function signatures here (one per line, indented).

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.c or test_bridge.h.

## Rules
- Do NOT remove or modify existing test cases.
- All tests must be deterministic.
TODO: any project-specific testgen rules (libmcs had: "- Print all results in %a hex float format.")
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
