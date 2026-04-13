#!/usr/bin/env python3
"""Compare C and Rust test outputs line by line.

Each line is one test case: "funcname input... = result..."
Compares corresponding lines between C and Rust output.

Produces a structured report:
- MISSING: lines in C output but not in Rust (crash/timeout/not implemented)
- MISMATCH: same line position, different result
- FAULT: lines starting with "FAULT" (crash/timeout detected by fork wrapper)

Environment:
  COMPARE_IGNORE_KEYS: space-separated list of line keys to exclude entirely.
      For a line "foo: 42", the key is "foo"; for "sin 0.5 = 0x1.8p-1",
      the key is "sin". Matching is exact on the first whitespace-delimited
      token of the key portion (before ':' or '='). Used to skip lines whose
      value is cosmetic free text (e.g. library error messages) that may
      differ between C and Rust without indicating a real bug.

Usage: compare_outputs.py <c_output> <rust_output> [-o report_file]
"""

import os
import sys
import argparse


def _ignore_keys():
    """Parse $COMPARE_IGNORE_KEYS into a set of exact-match keys."""
    return set(os.environ.get("COMPARE_IGNORE_KEYS", "").split())


def _line_key(line):
    """Extract the key from a test line.

    Lines look like either "key: value" or "key value = result".
    The key is the first whitespace-delimited token of the left side.
    """
    left = line.split('=', 1)[0]
    left = left.split(':', 1)[0]
    toks = left.split()
    return toks[0] if toks else ""


def parse_output(filepath, ignored=None):
    """Parse test output into list of lines.
    Skips FAULT lines (handled separately), blank lines, and lines whose
    key is in `ignored` (set)."""
    if ignored is None:
        ignored = _ignore_keys()
    test_lines = []
    fault_lines = []
    with open(filepath, errors='replace') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            if line.startswith("FAULT "):
                fault_lines.append(line)
                continue
            if line.startswith("==="):
                continue
            # Skip lines that aren't test output (no = or : separator)
            if '=' not in line and ': ' not in line:
                continue
            if ignored and _line_key(line) in ignored:
                continue
            test_lines.append(line)
    return test_lines, fault_lines


def func_name(line):
    """Extract function name from a test line like 'sin 0x1p-1 = 0x1p-1'."""
    key = line.split('=')[0].strip()
    return key.split()[0] if key.split() else key


def compare(c_file, r_file):
    c_lines, c_faults = parse_output(c_file)
    r_lines, r_faults = parse_output(r_file)

    missing = []    # test cases in C but not in Rust
    mismatch = []   # test cases in both but different result

    # Build Rust lookup: key -> list of lines (preserves duplicates)
    r_by_key = {}
    for line in r_lines:
        key = line.split('=')[0].strip()
        if key not in r_by_key:
            r_by_key[key] = []
        r_by_key[key].append(line)

    # For each C test case, find matching Rust result
    r_used = {}  # track which Rust lines we've matched
    for c_line in c_lines:
        key = c_line.split('=')[0].strip()
        if key not in r_by_key or len(r_by_key[key]) == 0:
            missing.append(c_line)
        else:
            # Pop first matching Rust line for this key
            r_line = r_by_key[key].pop(0)
            if c_line != r_line:
                mismatch.append((c_line, r_line))

    return missing, mismatch, r_faults, len(c_lines), len(r_lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("c_output")
    parser.add_argument("rust_output")
    parser.add_argument("-o", "--output", default=None)
    args = parser.parse_args()

    missing, mismatch, r_faults, c_count, r_count = compare(
        args.c_output, args.rust_output)

    lines = []

    lines.append(f"C output: {c_count} test lines")
    lines.append(f"Rust output: {r_count} test lines")
    lines.append("")

    # Faults (crashes/timeouts caught by fork wrapper)
    if r_faults:
        lines.append(f"FAULT ({len(r_faults)} test functions crashed/timed out):")
        for fl in r_faults:
            lines.append(f"  {fl}")
        lines.append("")

    # Group missing by function
    if missing:
        missing_funcs = {}
        for c_line in missing:
            fn = func_name(c_line)
            if fn not in missing_funcs:
                missing_funcs[fn] = []
            missing_funcs[fn].append(c_line)

        lines.append(f"MISSING ({len(missing)} tests, {len(missing_funcs)} functions):")
        lines.append("Functions not in Rust output (not implemented or crashed):")
        for fn in sorted(missing_funcs):
            lines.append(f"  {fn}: {len(missing_funcs[fn])} tests")
        lines.append("")

    # Group mismatch by function
    if mismatch:
        mismatch_funcs = {}
        for c_line, r_line in mismatch:
            fn = func_name(c_line)
            if fn not in mismatch_funcs:
                mismatch_funcs[fn] = []
            mismatch_funcs[fn].append((c_line, r_line))

        lines.append(f"MISMATCH ({len(mismatch)} tests, {len(mismatch_funcs)} functions):")
        lines.append("C and Rust produce different results:")
        for fn in sorted(mismatch_funcs):
            pairs = mismatch_funcs[fn]
            lines.append(f"  {fn}: {len(pairs)} mismatches")
            for c_line, r_line in pairs[:3]:
                lines.append(f"    C:    {c_line}")
                lines.append(f"    Rust: {r_line}")
            if len(pairs) > 3:
                lines.append(f"    ... ({len(pairs) - 3} more)")
        lines.append("")

    # Summary
    total_failures = len(missing) + len(mismatch)
    lines.append("SUMMARY")
    lines.append(f"Tests passed:     {c_count - total_failures}")
    lines.append(f"Tests failed:     {total_failures}")
    if missing:
        missing_funcs_list = sorted(set(func_name(l) for l in missing))
        lines.append(f"Missing functions: {', '.join(missing_funcs_list)}")
    if mismatch:
        mismatch_funcs_list = sorted(set(func_name(c) for c, _ in mismatch))
        lines.append(f"Mismatched functions: {', '.join(mismatch_funcs_list)}")

    output = '\n'.join(lines)
    if args.output:
        with open(args.output, 'w') as f:
            f.write(output + '\n')
    print(output)


if __name__ == "__main__":
    main()
