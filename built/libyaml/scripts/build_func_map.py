#!/usr/bin/env python3
"""Build a function name → source file path mapping.

Output format (one per line):
  funcname -> /absolute/path/to/source.c

Used by the difffix report so the AI can look up where any function is
defined without reading every source file.

Iterates the directories listed in $C_SRC_DIRS (space-separated, exported
by common.sh). Extracts function names from each .c file via regex.
"""

import os
import re
import sys


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
        name_part = sig.split('(')[0]
        name_part = name_part.replace('*', ' ')
        name = name_part.split()[-1]
        funcs.append(name)
    return funcs


def main():
    c_src_dirs = os.environ.get("C_SRC_DIRS", "").split()
    if not c_src_dirs:
        sys.exit("C_SRC_DIRS not set (export from common.sh)")

    excludes = set(os.environ.get("EXCLUDE_C_FILES", "").split("|")) - {""}

    for src_dir in c_src_dirs:
        if not os.path.isdir(src_dir):
            continue
        for root, _dirs, files in os.walk(src_dir):
            for fname in sorted(files):
                if not fname.endswith('.c'):
                    continue
                if fname in excludes:
                    continue
                path = os.path.join(root, fname)
                for func in extract_functions(path):
                    print(f"{func} -> {path}")


if __name__ == "__main__":
    main()
