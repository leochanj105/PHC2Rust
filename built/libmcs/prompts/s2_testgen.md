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

## Static functions

A pre-built test_bridge.h is provided in the working directory, declaring
`bridge_*` wrapper functions for the library's static C functions.

    bridge___rem_pio2_internal(double *x, double *y, int e0, int nx)
    bridge___tan(double x, double y, int iy)
    bridge___sin_pi(double x)
    bridge___rem_pio2f_internal(float *x, float *y, int e0, int nx)
    bridge___tanf(float x, float y, int iy)
    bridge___ctans(double _Complex z)
    bridge___ctansf(float _Complex z)
    bridge___sin_pif(float x)

To use them, add `#include "test_bridge.h"` at the top of test_suite.c.
Do NOT generate or modify test_bridge.h or any bridge_* implementation files.

## Rules
- All tests must be deterministic.
- Print all results in %a hex float format.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
