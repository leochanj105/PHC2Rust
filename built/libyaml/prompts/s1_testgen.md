You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

TODO: describe what the test generator should produce for libyaml.
TODO: e.g. "Generate a test_suite.c that exercises the YAML parser and emitter,
TODO: printing events, scalar values, and emitted output as plain text so that
TODO: C and Rust outputs can be diffed line-by-line."

## Static functions

A pre-built test_bridge.c and test_bridge.h are provided in the working directory.
They expose these static functions as callable bridge wrappers:

TODO: run framework/scripts/gen_bridges.py against libyaml/src/ with a
TODO: coverage-instrumented binary to get the authoritative list, then paste
TODO: the bridge function signatures here (one per line, indented).

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.c or test_bridge.h.

## Rules
- All tests must be deterministic.
TODO: any project-specific testgen rules (libmcs had: "- Print all results in %a hex float format.")
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
