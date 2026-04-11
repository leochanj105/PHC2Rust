# Project Context

## Library
Libmcs — a C math library (libm implementation). IEEE 754 compliant.

## Comparison rule
All outputs must be bitwise exact. Use %a hex float format. Zero tolerance.

## Source layout
- /home/leochanj/Desktop/libmcs/libm/mathd/       — double-precision functions
- /home/leochanj/Desktop/libmcs/libm/mathf/       — float-precision functions
- /home/leochanj/Desktop/libmcs/libm/complexd/    — complex double functions
- /home/leochanj/Desktop/libmcs/libm/complexf/    — complex float functions
- /home/leochanj/Desktop/libmcs/libm/common/      — shared utilities
- /home/leochanj/Desktop/libmcs/libm/include/     — headers (math.h, complex.h, fenv.h)
Each directory has an internal/ subdirectory with helper functions.
