
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
