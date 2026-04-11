#!/usr/bin/env python3
"""Build a function name → source file path mapping.

Output format (one per line):
  funcname → /absolute/path/to/source.c

Used by the difffix report so the AI can look up where any function is
defined without reading all 190 source files.

Usage: build_func_map.py [--libm PATH]
"""

import os
import re
import argparse

def extract_functions(filepath):
    """Extract function names from a C source file."""
    funcs = []
    with open(filepath) as f:
        content = f.read()
    pattern = r'^((?:static\s+)?[a-zA-Z_][\w\s*]*?\s+\*?[a-zA-Z_]\w*\s*\([^)]*\))\s*\{'
    for m in re.finditer(pattern, content, re.MULTILINE):
        sig = m.group(1).strip()
        if any(sig.startswith(k) for k in ('if', 'for', 'while', 'switch', 'return',
                                             'else', 'do', 'typedef', 'struct', 'union')):
            continue
        # Extract just the function name
        name_part = sig.split('(')[0]
        name_part = name_part.replace('*', ' ')
        name = name_part.split()[-1]
        funcs.append(name)
    return funcs


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--libm", default=os.environ.get("LIBMCS"))
    args = parser.parse_args()

    src_dirs = ["mathd", "mathf", "common", "complexd", "complexf"]

    for src_dir in src_dirs:
        full_dir = os.path.join(args.libm, src_dir)
        if not os.path.isdir(full_dir):
            continue
        for root, dirs, files in os.walk(full_dir):
            for fname in sorted(files):
                if not fname.endswith('.c') or fname == 'fenv.c':
                    continue
                path = os.path.join(root, fname)
                for func in extract_functions(path):
                    print(f"{func} -> {path}")


if __name__ == "__main__":
    main()
