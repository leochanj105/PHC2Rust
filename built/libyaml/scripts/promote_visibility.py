#!/usr/bin/env python3
"""promote_visibility.py — promote Rust functions referenced by test_bridge.rs to pub(crate).

Parses test_bridge.rs to find all `crate::module::fn_name(` references, then
edits the corresponding source files (module.rs) to make each referenced
function `pub(crate)`.

Handles these private function forms (what Claude's transpilation produces):
    fn X(...)
    unsafe fn X(...)
    extern "C" fn X(...)
    unsafe extern "C" fn X(...)

In each case a `pub(crate) ` prefix is prepended. Already-public functions
(declared with `pub ` anywhere before `fn`) are left alone.

Implementation uses exact line-prefix string matching (no regex) over a
finite list of known declaration forms. Each source line is inspected
once; any line whose leading tokens exactly match one of the forms gets
the prefix added. Everything else passes through verbatim.

Usage:
    promote_visibility.py <test_bridge.rs> <src_dir>
"""
import sys
from pathlib import Path


# The finite set of private-function declaration prefixes we recognize.
# Order matters only for documentation — the loop tries them longest-first
# to avoid `fn X` accidentally matching part of `unsafe fn X`.
DECL_PREFIXES = (
    'unsafe extern "C" fn',
    'extern "C" fn',
    'unsafe fn',
    'fn',
)


def extract_refs(bridge_text):
    """Find every `crate::<module>::<fn>` reference in the bridge source.

    Returns {module: set(fn_names)}. Uses exact substring scanning, not
    regex. We only care about identifiers immediately following `crate::`.
    """
    out = {}
    prefix = 'crate::'
    i = 0
    while True:
        i = bridge_text.find(prefix, i)
        if i == -1:
            break
        j = i + len(prefix)
        mod_start = j
        while j < len(bridge_text) and (bridge_text[j].isalnum() or bridge_text[j] == '_'):
            j += 1
        module = bridge_text[mod_start:j]
        # Require `::` after module
        if bridge_text[j:j+2] != '::':
            i = j
            continue
        j += 2
        fn_start = j
        while j < len(bridge_text) and (bridge_text[j].isalnum() or bridge_text[j] == '_'):
            j += 1
        fn = bridge_text[fn_start:j]
        # Require `(` following (possibly with whitespace) to be a call
        k = j
        while k < len(bridge_text) and bridge_text[k] in ' \t':
            k += 1
        if k < len(bridge_text) and bridge_text[k] == '(':
            if module and fn:
                out.setdefault(module, set()).add(fn)
        i = j
    return out


def promote_file(src_file: Path, want: set) -> dict:
    """Promote functions in `want` to pub(crate) inside src_file.

    Scans line-by-line. For each line, strips leading whitespace to get the
    'stripped' start. If the stripped start begins with `pub`, the line is
    already public and is left alone. Otherwise, for each known declaration
    prefix, check if the line starts with `<prefix> <wanted_name>` where the
    next char is `(` or `<` (for generics). If so, prepend `pub(crate) `
    after the original indent.

    Returns counts dict.
    """
    text = src_file.read_text()
    lines = text.splitlines(keepends=True)
    counts = {'promoted': 0, 'already_public': 0}
    seen_wanted = set()  # names we found a declaration for in this file

    for idx, line in enumerate(lines):
        # Preserve leading whitespace; look at the rest.
        leading = line[:len(line) - len(line.lstrip(' \t'))]
        rest = line[len(leading):]

        if rest.startswith('pub '):
            # Already public (or pub(crate), pub(super), etc. — any pub prefix)
            continue
        if rest.startswith('pub('):
            continue

        for prefix in DECL_PREFIXES:
            if not rest.startswith(prefix + ' '):
                continue
            # After the prefix+space, the next identifier is the function name.
            after = rest[len(prefix) + 1:]
            # Find end of identifier
            k = 0
            while k < len(after) and (after[k].isalnum() or after[k] == '_'):
                k += 1
            name = after[:k]
            if not name or name not in want:
                continue
            # Next non-space char must be '(' (or '<' for generics)
            m = k
            while m < len(after) and after[m] in ' \t':
                m += 1
            if m >= len(after) or after[m] not in '(<':
                continue
            # Match — prepend pub(crate)
            lines[idx] = leading + 'pub(crate) ' + rest
            counts['promoted'] += 1
            seen_wanted.add(name)
            break  # handled this line

    new_text = ''.join(lines)
    if new_text != text:
        src_file.write_text(new_text)

    # Count functions that were in `want` but found as already-public.
    # For each remaining wanted name, scan lines for a `pub` declaration.
    remaining = want - seen_wanted
    for name in remaining:
        for line in lines:
            stripped = line.lstrip(' \t')
            if not stripped.startswith('pub'):
                continue
            # Strip leading `pub(...)?` and optional space
            s = stripped[3:]  # past "pub"
            if s.startswith('('):
                end = s.find(')')
                if end == -1:
                    continue
                s = s[end + 1:]
            s = s.lstrip(' \t')
            for prefix in DECL_PREFIXES:
                if not s.startswith(prefix + ' '):
                    continue
                after = s[len(prefix) + 1:]
                k = 0
                while k < len(after) and (after[k].isalnum() or after[k] == '_'):
                    k += 1
                if after[:k] == name:
                    counts['already_public'] += 1
                    seen_wanted.add(name)
                    break
            if name in seen_wanted:
                break

    counts['not_found'] = len(want - seen_wanted)
    return counts


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

    by_module = extract_refs(bridge_path.read_text())
    # Filter out the doc-comment template `crate::module::function`
    by_module.pop('module', None)

    totals = {'promoted': 0, 'already_public': 0, 'not_found': 0}
    for module in sorted(by_module):
        src_file = src_dir / f"{module}.rs"
        if not src_file.is_file():
            print(f"  skip: {src_file} not found "
                  f"({sorted(by_module[module])})")
            continue
        counts = promote_file(src_file, by_module[module])
        for k in totals:
            totals[k] += counts[k]

    print(f"Done: {totals['promoted']} promoted, "
          f"{totals['already_public']} already public, "
          f"{totals['not_found']} not found")


if __name__ == "__main__":
    main()
