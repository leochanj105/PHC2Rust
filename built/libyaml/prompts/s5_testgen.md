You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

Generate a test_suite.c that exercises the YAML parser and emitter,
printing events, scalar values, and emitted output as plain text so that
C and Rust outputs can be diffed line-by-line.

Cover ALL functions in this library, including internal static functions
that are not declared in headers.

The test file already contains tests that cover most functions (from a previous
phase). Your job is to add tests that target **uncovered branches** — specific
code paths that existing tests never exercise.

## Coverage feedback

This is a coverage-guided test generation round. You will receive **branch
coverage feedback**: a list of branch conditions that existing tests do not
yet exercise.

Read these files in the working directory:

- test_suite.c    — current test file; ADD to it, do not remove existing tests

- uncovered.md    — branch conditions not yet exercised.
  Format: "file.c, Branch (line:col): True/False"
  Each branch has two sides (True and False). An uncovered side means no test
  has made execution take that path yet.
  Example:
    sind.c, Branch (45:12): False    — the else/false side at line 45 was never taken
    expd.c, Branch (120:5): True     — the if/true side at line 120 was never taken

- crash_summary.md — if non-empty, fix crashing tests FIRST

For each uncovered branch in uncovered.md:
- Read the C source file around that line number
- Understand the condition: what does the if/else test?
- Determine what input values would make execution take the uncovered path
- Write a test that calls the function with those specific inputs

## Static functions

A pre-built test_bridge.c and test_bridge.h are provided in the working directory.
They expose these static functions as callable bridge wrappers:

    (See test_bridge.h for the full list — too many to inline here.)

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.c or test_bridge.h.

## Rules
- Do NOT remove or modify existing test cases.
- All tests must be deterministic.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
