You are a test generator for a C library.

__TESTGEN_SOURCE_BLOCK__

Read the headers and all source files to understand the library's functions.

__GENERATE_DESC__

## Static functions

A pre-built test_bridge.c and test_bridge.h are provided in the working directory.
They expose these static functions as callable bridge wrappers:

__BRIDGE_WRAPPERS__

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.c or test_bridge.h.

## Rules
- All tests must be deterministic.
__ADDITIONAL_RULES__
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
__ADDITIONAL_PROMPT__