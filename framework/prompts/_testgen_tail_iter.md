
## Static functions

A pre-built test_bridge.h is provided in the working directory, declaring
`bridge_*` wrapper functions for the library's static C functions.

__BRIDGE_WRAPPERS__

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.h or any bridge_* implementation files.

## Rules
- Do NOT remove or modify existing test cases.
- All tests must be deterministic.
__ADDITIONAL_RULES__
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
__ADDITIONAL_PROMPT__