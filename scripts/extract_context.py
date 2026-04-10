#!/usr/bin/env python3
"""Extract a compact context file from the C source tree.

Produces a single file containing:
1. All macro definitions from tools.h / internal headers (bit manipulation, etc.)
2. All function signatures with their source file path
3. Type definitions and constants

This avoids the AI reading 190+ files just to understand the library structure.
~2-5K tokens instead of ~170K tokens.
"""

import os
import re
import sys

LIBM = "/home/leochanj/Desktop/libmcs/libm"
SRC_DIRS = ["mathd", "mathf", "common", "complexd", "complexf"]
INCLUDE_DIR = os.path.join(LIBM, "include")


def extract_macros(filepath):
    """Extract #define macros from a header file."""
    macros = []
    with open(filepath) as f:
        for line in f:
            if line.startswith("#define "):
                macros.append(line.rstrip())
    return macros


def extract_function_sigs(filepath):
    """Extract function signatures from a .c file."""
    sigs = []
    with open(filepath) as f:
        content = f.read()

    # Match function definitions (simplified)
    pattern = r'^((?:static\s+)?[a-zA-Z_][\w\s*]*?\s+\*?[a-zA-Z_]\w*\s*\([^)]*\))\s*\{'
    for m in re.finditer(pattern, content, re.MULTILINE):
        sig = m.group(1).strip()
        # Skip preprocessor, control flow
        if any(sig.startswith(k) for k in ('if', 'for', 'while', 'switch', 'return', 'else', 'do', 'typedef')):
            continue
        sigs.append(sig)
    return sigs


def main():
    out = sys.stdout

    # 1. Headers — full content (small, ~960 lines total)
    out.write("# Headers\n\n")
    for hfile in sorted(os.listdir(INCLUDE_DIR)):
        if not hfile.endswith('.h'):
            continue
        path = os.path.join(INCLUDE_DIR, hfile)
        out.write(f"## {hfile}\n```c\n")
        with open(path) as f:
            out.write(f.read())
        out.write("```\n\n")

    # 2. Internal headers — macro/type definitions
    out.write("# Internal Headers\n\n")
    for src_dir in SRC_DIRS:
        internal_dir = os.path.join(LIBM, src_dir, "internal")
        if not os.path.isdir(internal_dir):
            continue
        for fname in sorted(os.listdir(internal_dir)):
            if not fname.endswith('.h'):
                continue
            path = os.path.join(internal_dir, fname)
            out.write(f"## {src_dir}/internal/{fname}\n```c\n")
            with open(path) as f:
                out.write(f.read())
            out.write("```\n\n")

    # 3. Function signatures by module
    out.write("# Function Signatures\n\n")
    for src_dir in SRC_DIRS:
        full_dir = os.path.join(LIBM, src_dir)
        if not os.path.isdir(full_dir):
            continue
        out.write(f"## {src_dir}/\n\n")
        for fname in sorted(os.listdir(full_dir)):
            if not fname.endswith('.c'):
                continue
            path = os.path.join(full_dir, fname)
            sigs = extract_function_sigs(path)
            for sig in sigs:
                is_static = "static" in sig.split('(')[0]
                marker = "[static] " if is_static else ""
                out.write(f"- {marker}`{sig}` — {src_dir}/{fname}\n")
        # Internal .c files
        internal_dir = os.path.join(full_dir, "internal")
        if os.path.isdir(internal_dir):
            for fname in sorted(os.listdir(internal_dir)):
                if not fname.endswith('.c'):
                    continue
                path = os.path.join(internal_dir, fname)
                sigs = extract_function_sigs(path)
                for sig in sigs:
                    out.write(f"- [static] `{sig}` — {src_dir}/internal/{fname}\n")
        out.write("\n")


if __name__ == "__main__":
    main()
