You are generating a differential test file that compares the original C library
against its transpiled Rust version.

The Rust library exports functions with the SAME names as the C library (no _rs
suffix). Differential testing works by compiling the same test file against the
C library and the Rust library separately, running both, and diffing the output.

- Original C source: /home/leochanj/Desktop/libmcs/libm/
- Transpiled Rust source: __RUST_DIR__/src/

Read test_suite.c — the existing test file.

Generate difftest_suite.c which:

1. For every test call in test_suite.c, create a corresponding call that prints
   the function name, inputs, and result in %a hex float format:

   ```c
   printf("sin %a = %a\n", 0.5, sin(0.5));
   printf("atan2 %a %a = %a\n", y, x, atan2(y, x));
   printf("csin %a %a = %a %a\n", re, im, creal(csin(z)), cimag(csin(z)));
   printf("modf %a = %a %a\n", x, modf(x, &ipart), ipart);
   ```

2. Must compile cleanly with both the C library and the Rust library.

3. All output must use %a format for floats/doubles. This enables bitwise
   comparison by diffing the two outputs.

4. Prints a count line at the end: printf("Total: %d tests\n", count);

5. If test_suite.c uses `bridge_` functions (from test_bridge.c) to call static
   C functions, include those calls too. Declare bridge functions as extern.
   The Rust library does not export static functions, so bridge tests will only
   run in the C build — that is expected.

Write the complete difftest_suite.c file.
