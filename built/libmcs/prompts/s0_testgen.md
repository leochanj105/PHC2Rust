You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libmcs/libm/.
Headers are in /home/leochanj/Desktop/libmcs/libm/include/.

Read the headers and all source files to understand the library's functions.

Generate a test file called test_suite.c that tests this math library.
Each test function should call a library function with specific inputs and
print the result in %a hex float format, e.g.:

    printf("sin %a = %a\n", 0.5, sin(0.5));

## Static functions
Static functions will be expoed via bridge wrappers. They should also be considered
when generating tests.

## Rules
- All tests must be deterministic.
- Print all results in %a hex float format.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
