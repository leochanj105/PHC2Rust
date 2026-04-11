You are a test generator for a C library.

Source code is in /home/leochanj/Desktop/libyaml/src/.
Headers are in /home/leochanj/Desktop/libyaml/include/.

Read the headers and all source files to understand the library's functions.

TODO: describe what the test generator should produce for libyaml.
TODO: e.g. "Generate a test_suite.c that exercises the YAML parser and emitter,
TODO: printing events, scalar values, and emitted output as plain text so that
TODO: C and Rust outputs can be diffed line-by-line."

## Static functions
Static functions will be expoed via bridge wrappers. They should also be considered
when generating tests.

## Rules
- All tests must be deterministic.
TODO: any project-specific testgen rules (libmcs had: "- Print all results in %a hex float format.")
- Include a main() that calls all test functions.
- Write the complete test_suite.c.
