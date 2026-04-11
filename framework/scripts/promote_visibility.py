#!/usr/bin/env python3
"""promote_visibility.py — promote Rust functions referenced by test_bridge.rs to pub(crate).

Parses test_bridge.rs to find all `crate::module::fn_name(` references, then
edits the corresponding source files (module.rs) to change
`^fn fn_name(` → `pub(crate) fn fn_name(`.

Already-public functions (those declared as `pub fn ...` or `pub(crate) fn ...`)
are left alone — the regex only matches a bare `fn` at start of line.

Usage:
    promote_visibility.py <test_bridge.rs> <src_dir>

Exits 0 on success. Does NOT fail if a referenced function isn't found in
source — that case is silent (it may already be public).
"""
import re
import sys
from pathlib import Path


def main():
    if len(sys.argv) != 3:
        print("Usage: promote_visibility.py <test_bridge.rs> <src_dir>",
              file=sys.stderr)
        sys.exit(1)

    bridge_path = Path(sys.argv[1])
    src_dir = Path(sys.argv[2])

    if not bridge_path.is_file():
        print(f"ERROR: bridge file not found: {bridge_path}", file=sys.stderr)
        sys.exit(1)
    if not src_dir.is_dir():
        print(f"ERROR: src dir not found: {src_dir}", file=sys.stderr)
        sys.exit(1)

    bridge_text = bridge_path.read_text()

    # Collect `crate::module::fn_name(` references (bridge wrappers delegate here).
    ref_re = re.compile(r'crate::(\w+)::(\w+)\s*\(')
    by_module: dict[str, set[str]] = {}
    for module, fn in ref_re.findall(bridge_text):
        by_module.setdefault(module, set()).add(fn)

    promoted = 0
    skipped = 0
    for module in sorted(by_module):
        src_file = src_dir / f"{module}.rs"
        if not src_file.is_file():
            print(f"  skip: {src_file} not found "
                  f"({sorted(by_module[module])})")
            continue
        text = src_file.read_text()
        original = text
        for fn in sorted(by_module[module]):
            new = re.sub(
                rf'(?m)^fn {re.escape(fn)}\(',
                f'pub(crate) fn {fn}(',
                text,
            )
            if new != text:
                promoted += 1
                print(f"  promoted: {module}::{fn}")
                text = new
            else:
                skipped += 1
        if text != original:
            src_file.write_text(text)

    print(f"Done: {promoted} promoted, {skipped} already-public or not-found")


if __name__ == "__main__":
    main()
