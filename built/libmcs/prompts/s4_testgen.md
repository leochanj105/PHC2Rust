You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libmcs/libm/.
Headers are in /home/leochanj/Desktop/libmcs/libm/include/.

Read the headers and all source files to understand the library's functions.

Generate a test file called test_suite.c that tests this math library.
Each test function should call a library function with specific inputs and
print the result in %a hex float format, e.g.:

    printf("sin %a = %a\n", 0.5, sin(0.5));

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

    bridge___rem_pio2_internal(double *x, double *y, int e0, int nx)
    bridge___tan(double x, double y, int iy)
    bridge___sin_pi(double x)
    bridge___rem_pio2f_internal(float *x, float *y, int e0, int nx)
    bridge___tanf(float x, float y, int iy)
    bridge___ctans(double _Complex z)
    bridge___ctansf(float _Complex z)
    bridge___sin_pif(float x)

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.c or test_bridge.h.

## Rules
- Do NOT remove or modify existing test cases.
- All tests must be deterministic.
- Print all results in %a hex float format.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
