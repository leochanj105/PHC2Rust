
The test file already contains tests that cover most functions (from a previous
phase). Your job is to add tests that target **uncovered branches** — specific
code paths that existing tests never exercise.

## Coverage feedback

This is a coverage-guided test generation round. You will receive **branch
coverage feedback**: a list of branch conditions that existing tests do not
yet exercise.

Read these files in the working directory:

- test_suite.c    — current test file; ADD to it, do not remove existing tests

- uncovered.md    — branch conditions not yet exercised.
  Format: "file.c, Branch (line:col): True/False"
  Each branch has two sides (True and False). An uncovered side means no test
  has made execution take that path yet.
  Example:
    sind.c, Branch (45:12): False    — the else/false side at line 45 was never taken
    expd.c, Branch (120:5): True     — the if/true side at line 120 was never taken

- crash_summary.md — if non-empty, fix crashing tests FIRST

For each uncovered branch in uncovered.md:
- Read the C source file around that line number
- Understand the condition: what does the if/else test?
- Determine what input values would make execution take the uncovered path
- Write a test that calls the function with those specific inputs
