#!/usr/bin/env python3
"""Extract function signatures only — minimal context for testgen/strategy.

Output: a compact list of all function signatures with source locations.
~200 lines, ~1.2K tokens.
"""

import os
import re

LIBM = "/home/leochanj/Desktop/libmcs/libm"
SRC_DIRS = ["mathd", "mathf", "common", "complexd", "complexf"]


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
    for src_dir in SRC_DIRS:
        full_dir = os.path.join(LIBM, src_dir)
        if not os.path.isdir(full_dir):
            continue
        print(f"## {src_dir}/")
        for fname in sorted(os.listdir(full_dir)):
            if not fname.endswith('.c'):
                continue
            path = os.path.join(full_dir, fname)
            for sig in extract_sigs(path):
                is_static = "static" in sig.split('(')[0]
                marker = "[static] " if is_static else ""
                print(f"- {marker}`{sig}` — {fname}")

        internal = os.path.join(full_dir, "internal")
        if os.path.isdir(internal):
            for fname in sorted(os.listdir(internal)):
                if not fname.endswith('.c'):
                    continue
                path = os.path.join(internal, fname)
                for sig in extract_sigs(path):
                    print(f"- [static] `{sig}` — internal/{fname}")
        print()


if __name__ == "__main__":
    main()
