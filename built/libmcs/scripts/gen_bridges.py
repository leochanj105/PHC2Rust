"""Generate test bridge wrappers for static C functions (per-source-file layout).

For each .c file in <src_dir>, produces a bridge_<name>.c file under <out_dir>
that:
  1. #includes the original .c source (so it has access to static functions)
  2. Defines bridge_<funcname>(...) wrappers calling each static function

The bridges are linked into the test binary so test code can call
bridge_<funcname>(...) instead of the unreachable static <funcname>(...).

Linker --allow-multiple-definition is required because each bridge .c file
re-#includes a source .c file, redefining all of its non-static symbols.

Requires a coverage-instrumented binary to derive the authoritative static
function list via `llvm-cov export -empty-profile`.

NOTE: this generator produces per-file C bridges only. It does NOT emit a
single-file test_bridge.c/.h, nor a test_bridge.rs. Extending to those
layouts is a separate piece of work.

Usage:
    python3 gen_bridges.py <src_dir> <out_dir> --binary <cov_bin> [-I <inc_dir>...]
"""
import argparse
import os
import re
import subprocess
import sys


# Locate the start of any function definition: '<ret> <name>('.
# We deliberately don't anchor on `static` because clang -ast-print sometimes
# strips the storage class from the definition when it was declared earlier.
# The caller filters by an externally-supplied static-name set.
# `\*?` allows pointer return types with no space (e.g. 'yaml_char_t *foo').
_FUNC_START_RE = re.compile(
    r"^(?:static\s+)?(?P<ret>[\w\s\*]+?[\w\*])\s*(?P<name>\w+)\s*\(",
    re.MULTILINE,
)


def _extract_balanced_args(text, start_idx):
    """Given text and the index of the opening '(', return (args_str, end_idx)
    where end_idx is the index just past the matching ')'."""
    assert text[start_idx] == "("
    depth = 0
    i = start_idx
    while i < len(text):
        c = text[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return text[start_idx + 1:i], i + 1
        i += 1
    return None, len(text)


def get_ast_print(c_file, include_dirs, src_dir):
    """Run clang -Xclang -ast-print and return the printed source."""
    cmd = [
        "clang-21", "-Xclang", "-ast-print", "-fsyntax-only",
        "-DHAVE_CONFIG_H=1",
    ]
    for d in include_dirs:
        cmd += ["-I", d]
    cmd += ["-I", src_dir, c_file]
    return subprocess.check_output(cmd, stderr=subprocess.DEVNULL, text=True)


def parse_function_defs(ast_text, name_filter=None):
    """Return list of (name, return_type, arg_decls, arg_names) for function
    DEFINITIONS (not declarations). A definition has '{' after ')'.

    If name_filter is given (a set), only functions whose name is in the set
    are returned. Otherwise all definitions are returned.
    """
    funcs = []
    seen = set()
    for m in _FUNC_START_RE.finditer(ast_text):
        ret = m.group("ret").strip()
        name = m.group("name").strip()
        if name_filter is not None and name not in name_filter:
            continue
        paren_idx = m.end() - 1
        args, end_idx = _extract_balanced_args(ast_text, paren_idx)
        if args is None:
            continue
        rest = ast_text[end_idx:end_idx + 200].lstrip()
        if not rest.startswith("{"):
            continue  # forward declaration only
        if name in seen:
            continue
        seen.add(name)

        # Normalize pointer return types: 'yaml_char_t *' -> the '*' may have
        # been captured at the end of `ret` (e.g. 'yaml_char_t' or 'yaml_char_t *').
        ret = ret.strip()
        args = args.strip()
        if args == "" or args == "void":
            arg_decls = "void"
            arg_names = []
        else:
            arg_decls = args
            arg_names = []
            for param in _split_params(args):
                arg_names.append(_extract_param_name(param))
        funcs.append((name, ret, arg_decls, arg_names))
    return funcs


def get_static_function_names(binary, llvm_cov="llvm-cov-21"):
    """Use llvm-cov export -empty-profile on a coverage-instrumented binary
    to get the authoritative list of static functions, grouped by source file."""
    import json
    out = subprocess.check_output(
        [llvm_cov, "export", binary, "-empty-profile"],
        stderr=subprocess.DEVNULL,
    )
    data = json.loads(out)
    by_file = {}
    for f in data["data"][0]["functions"]:
        name = f["name"]
        if ":" not in name:
            continue  # not a static function
        bare = name.split(":", 1)[-1]
        src = f["filenames"][0].rsplit("/", 1)[-1]
        by_file.setdefault(src, set()).add(bare)
    return by_file


def _extract_param_name(param):
    """Extract the parameter name, ignoring __attribute__((...)) suffixes."""
    # Strip trailing attributes
    cleaned = re.sub(r"__attribute__\s*\(\([^)]*\)\)", "", param).strip()
    # Strip array brackets
    cleaned = re.sub(r"\[[^\]]*\]\s*$", "", cleaned).strip()
    # Take the last word — that's the parameter name
    m = re.search(r"(\w+)\s*$", cleaned)
    return m.group(1) if m else ""


def _split_params(args):
    """Split parameter list on commas, respecting nested parens (for function pointers)."""
    out = []
    depth = 0
    cur = []
    for ch in args:
        if ch == "(":
            depth += 1
            cur.append(ch)
        elif ch == ")":
            depth -= 1
            cur.append(ch)
        elif ch == "," and depth == 0:
            out.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    if cur:
        out.append("".join(cur))
    return out


def _wrapper_lines(funcs, prefix="    "):
    """Return a list of lines defining bridge_* wrapper functions."""
    out = []
    for name, ret, arg_decls, arg_names in funcs:
        call_args = ", ".join(arg_names) if arg_names else ""
        ret_clean = ret.strip()
        is_void = ret_clean == "void"
        out.append(f"{ret_clean} bridge_{name}({arg_decls}) {{")
        if is_void:
            out.append(f"{prefix}{name}({call_args});")
        else:
            out.append(f"{prefix}return {name}({call_args});")
        out.append("}")
        out.append("")
    return out


def _wrapper_decl(funcs):
    """Return a list of bridge_* function declarations (for test_bridge.h)."""
    out = []
    for name, ret, arg_decls, _ in funcs:
        out.append(f"{ret.strip()} bridge_{name}({arg_decls});")
    return out


def emit_bridge(c_file, out_path, funcs):
    """Per-file mode: write a bridge .c file that #includes c_file and exports bridge_* wrappers."""
    abs_src = os.path.abspath(c_file)
    lines = []
    lines.append("/* AUTO-GENERATED by gen_bridges.py — do not edit. */")
    lines.append(f'#include "{abs_src}"')
    lines.append("")
    lines.extend(_wrapper_lines(funcs))
    with open(out_path, "w") as f:
        f.write("\n".join(lines))


def emit_single_file_bridge(out_dir, groups):
    """Single-file mode: write test_bridge.c and test_bridge.h.

    `groups` is a list of (c_file_abs, funcs) tuples, one per source file
    that contributes bridges.

    Assumes no cross-file name collisions (the collision-detection step
    would warn about any). For libraries that use #define-style constants
    rather than `static const` globals (like libyaml), this is the case.
    """
    # ---- test_bridge.c ----
    c_lines = [
        "/* AUTO-GENERATED by gen_bridges.py --single-file — do not edit. */",
        "",
    ]
    all_funcs = []
    for abs_src, funcs in groups:
        basename = os.path.basename(abs_src)
        c_lines.append(f"/* ===== {basename} ===== */")
        c_lines.append(f'#include "{abs_src}"')
        c_lines.append("")
        c_lines.extend(_wrapper_lines(funcs))
        all_funcs.extend(funcs)

    with open(os.path.join(out_dir, "test_bridge.c"), "w") as f:
        f.write("\n".join(c_lines))

    # ---- test_bridge.h ----
    h_lines = [
        "/* AUTO-GENERATED by gen_bridges.py --single-file — do not edit. */",
        "#ifndef TEST_BRIDGE_H",
        "#define TEST_BRIDGE_H",
        "",
        "#include <yaml.h>",
        "",
    ]
    h_lines.extend(_wrapper_decl(all_funcs))
    h_lines.append("")
    h_lines.append("#endif /* TEST_BRIDGE_H */")

    with open(os.path.join(out_dir, "test_bridge.h"), "w") as f:
        f.write("\n".join(h_lines))


def detect_static_var_collisions(obj_dir):
    """Use `nm` on each .o file in obj_dir to find static variable name
    collisions across files. Returns a dict {var_name: [files]} for any
    name that appears in more than one file.

    Compiler-generated local labels (dot-prefixed, e.g. `.L.str.N`) are
    excluded — they're not real C identifiers and don't collide when
    .c files are #included into a single bridge file.
    """
    import subprocess
    data_types = {"d", "r", "b", "s"}
    all_vars = {}
    for f in sorted(os.listdir(obj_dir)):
        if not f.endswith(".o"):
            continue
        out = subprocess.check_output(["nm", os.path.join(obj_dir, f)], text=True)
        for line in out.splitlines():
            parts = line.split()
            if len(parts) == 3:
                _, ty, name = parts
            elif len(parts) == 2:
                ty, name = parts
            else:
                continue
            if name.startswith("."):
                continue  # skip compiler-generated labels
            if ty in data_types:
                all_vars.setdefault(name, []).append(f)
    return {v: files for v, files in all_vars.items() if len(files) > 1}


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("src_dir", help="Path to the C library src/ directory")
    p.add_argument("out_dir", help="Output directory for bridge_*.c files (per-file mode) or test_bridge.{c,h} (single-file mode)")
    p.add_argument("--binary", required=True,
                   help="Coverage-instrumented binary used to get the authoritative "
                        "static function list (e.g. built/libyaml/work-prepare/cov_bin)")
    p.add_argument("--include", "-I", action="append", default=[],
                   help="Additional include directories")
    p.add_argument("--llvm-cov", default="llvm-cov-21")
    p.add_argument("--single-file", action="store_true",
                   help="Emit a single test_bridge.c + test_bridge.h instead of one bridge file per source.")
    p.add_argument("--obj-dir",
                   help="(single-file mode) Directory containing .o files, used to detect "
                        "static variable name collisions via nm. If omitted, collision "
                        "detection is skipped.")
    args = p.parse_args(argv)

    include_dirs = list(args.include)
    lib_root = os.path.dirname(os.path.abspath(args.src_dir))
    for d in (
        os.path.join(lib_root, "include"),
        os.path.join(lib_root, "build", "include"),
    ):
        if os.path.isdir(d):
            include_dirs.append(d)

    os.makedirs(args.out_dir, exist_ok=True)

    static_by_file = get_static_function_names(args.binary, args.llvm_cov)
    total_expected = sum(len(s) for s in static_by_file.values())
    print(f"Static functions to bridge (from {args.binary}): {total_expected}")
    print()

    # Optional: warn about static-variable name collisions (single-file mode only)
    if args.single_file and args.obj_dir:
        collisions = detect_static_var_collisions(args.obj_dir)
        if collisions:
            print(f"WARNING: {len(collisions)} static variable name collisions across files:")
            for v, files in sorted(collisions.items()):
                print(f"  {v}: {files}")
            print("Single-file bridge may fail to compile. Consider per-file mode or")
            print("hand-editing renames.")
            print()
        else:
            print("No static variable name collisions — single-file bridge is safe.")
            print()

    # Collect wrapper groups: (abs_src_path, [(name, ret, arg_decls, arg_names), ...])
    groups = []
    total = 0
    for fname in sorted(os.listdir(args.src_dir)):
        if not fname.endswith(".c"):
            continue
        wanted = static_by_file.get(fname, set())
        if not wanted:
            continue
        c_file = os.path.join(args.src_dir, fname)
        try:
            ast = get_ast_print(c_file, include_dirs, args.src_dir)
        except subprocess.CalledProcessError as e:
            print(f"  WARNING: clang -ast-print failed on {fname}: {e}", file=sys.stderr)
            continue
        funcs = parse_function_defs(ast, name_filter=wanted)
        found_names = {f[0] for f in funcs}
        missing = wanted - found_names
        groups.append((os.path.abspath(c_file), funcs))
        msg = f"  {fname}: {len(funcs)}/{len(wanted)} wrappers"
        if missing:
            msg += f"  (MISSING: {', '.join(sorted(missing))})"
        print(msg)
        total += len(funcs)

    if args.single_file:
        emit_single_file_bridge(args.out_dir, groups)
        print()
        print(f"Total bridge wrappers generated: {total}/{total_expected}")
        print(f"Output: {args.out_dir}/test_bridge.c  + {args.out_dir}/test_bridge.h")
    else:
        for abs_src, funcs in groups:
            fname = os.path.basename(abs_src)
            out_path = os.path.join(args.out_dir, f"bridge_{fname}")
            emit_bridge(abs_src, out_path, funcs)
        print()
        print(f"Total bridge wrappers generated: {total}/{total_expected}")
        print(f"Output: {args.out_dir}/bridge_*.c")


if __name__ == "__main__":
    main()
