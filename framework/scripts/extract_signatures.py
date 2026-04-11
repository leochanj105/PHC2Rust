#!/usr/bin/env python3
"""Extract function signatures only — minimal context for testgen/strategy.

Output: a compact list of all function signatures with source locations.

Iterates the directories listed in $C_SRC_DIRS (space-separated, exported by
common.sh). For each .c file under those directories, extracts function
definitions via regex and prints them grouped by file.
"""

import os
import re
import sys


def extract_sigs(filepath):
    sigs = []
    with open(filepath) as f:
        content = f.read()
    pattern = r'^((?:static\s+)?[a-zA-Z_][\w\s*]*?\s+\*?[a-zA-Z_]\w*\s*\([^)]*\))\s*\{'
    for m in re.finditer(pattern, content, re.MULTILINE):
        sig = m.group(1).strip()
        if any(sig.startswith(k) for k in ('if', 'for', 'while', 'switch', 'return',
                                             'else', 'do', 'typedef', 'struct', 'union')):
            continue
        sigs.append(sig)
    return sigs


def main():
    c_src_dirs = os.environ.get("C_SRC_DIRS", "").split()
    if not c_src_dirs:
        sys.exit("C_SRC_DIRS not set (export from common.sh)")

    excludes = set(os.environ.get("EXCLUDE_C_FILES", "").split("|")) - {""}

    for src_dir in c_src_dirs:
        if not os.path.isdir(src_dir):
            continue
        print(f"## {os.path.basename(src_dir.rstrip('/'))}/")
        for root, _dirs, files in os.walk(src_dir):
            for fname in sorted(files):
                if not fname.endswith('.c'):
                    continue
                if fname in excludes:
                    continue
                path = os.path.join(root, fname)
                rel = os.path.relpath(path, src_dir)
                for sig in extract_sigs(path):
                    is_static = "static" in sig.split('(')[0]
                    marker = "[static] " if is_static else ""
                    print(f"- {marker}`{sig}` — {rel}")
        print()


if __name__ == "__main__":
    main()
