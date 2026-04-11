You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

Generate a test_suite.c that exercises the YAML parser and emitter,
printing events, scalar values, and emitted output as plain text so that
C and Rust outputs can be diffed line-by-line.

Cover ALL functions in this library, including internal static functions
that are not declared in headers.

For each function, include multiple inputs to cover edge cases:
NaN, +Inf, -Inf, +0.0, -0.0, denormals, very large values, very small values,
and boundary values.

## Static functions

A pre-built test_bridge.h is provided in the working directory, declaring
`bridge_*` wrapper functions for the library's static C functions.

    (See test_bridge.h for the full list — too many to inline here.)

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.h or any bridge_* implementation files.

## Rules
- All tests must be deterministic.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
