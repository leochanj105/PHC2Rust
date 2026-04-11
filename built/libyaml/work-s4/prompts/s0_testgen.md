You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

Generate a test_suite.c that exercises the YAML parser and emitter,
printing events, scalar values, and emitted output as plain text so that
C and Rust outputs can be diffed line-by-line.

## Static functions
Static functions will be expoed via bridge wrappers. They should also be considered
when generating tests.

## Rules
- All tests must be deterministic.
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
