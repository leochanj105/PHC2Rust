#!/usr/bin/env bash
set -euo pipefail

# bake.sh — combine framework + project values → concrete tree under built/<project>/
#
# Usage:
#   ./bake.sh <project>
#
# Reads:
#   projects/<project>/manifest.txt   — one path per line (globs expanded against framework/)
#   projects/<project>/values         — section-delimited placeholder map
#
# Writes:
#   built/<project>/<path>             — substituted copy of framework/<path>
#
# Substitution model:
#   - values file uses ==__NAME__== section markers, terminated by ==END==
#   - in framework files, any line consisting EXACTLY of __NAME__ is replaced
#     by the section's content (which may be multiple lines).
#   - lines matching "@include <path>" are replaced by the content of
#     framework/<path> (with substitution applied recursively).
#   - non-placeholder lines pass through verbatim.
#   - manifest lines may contain shell globs (*,?) which are expanded against
#     framework/ at bake time; adding a new file that matches is sufficient to
#     include it.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT="${1:-}"

[ -n "$PROJECT" ] || { echo "Usage: $0 <project>" >&2; exit 1; }

PROJECT_DIR="${ROOT}/projects/${PROJECT}"
MANIFEST="${PROJECT_DIR}/manifest.txt"
VALUES="${PROJECT_DIR}/values"
FRAMEWORK="${ROOT}/framework"
BUILT="${ROOT}/built/${PROJECT}"

[ -f "$MANIFEST" ] || { echo "Error: manifest not found: $MANIFEST" >&2; exit 1; }
[ -d "$FRAMEWORK" ] || { echo "Error: framework not found: $FRAMEWORK" >&2; exit 1; }

# Values file is optional — if absent, all files copied verbatim.
if [ ! -f "$VALUES" ]; then
    VALUES=/dev/null
fi

# Ensure build dir exists. Do NOT wipe — built/<project>/ may also contain
# runtime artifacts (work-*, rust-*, etc.) that the user has produced. Bake
# overwrites files in place; orphans (e.g. files removed from manifest) are
# left behind for the user to clean manually if desired.
mkdir -p "$BUILT"

substitute() {
    local src="$1" dst="$2"
    awk -v values_file="$VALUES" -v framework="$FRAMEWORK" '
    BEGIN {
        # Load the values file into a map.
        current = ""
        while ((getline line < values_file) > 0) {
            if (line ~ /^==__[A-Z_]+__==$/) {
                current = substr(line, 3, length(line) - 4)
                buf[current] = ""
                continue
            }
            if (line == "==END==") {
                current = ""
                continue
            }
            if (current != "") {
                buf[current] = buf[current] line "\n"
            }
        }
        close(values_file)
    }
    function expand_file(path,    line, inc) {
        while ((getline line < path) > 0) {
            if (line ~ /^@include /) {
                inc = framework "/" substr(line, 10)
                expand_file(inc)
            } else if (line in buf) {
                printf "%s", buf[line]
            } else {
                print line
            }
        }
        close(path)
    }
    {
        if ($0 ~ /^@include /) {
            expand_file(framework "/" substr($0, 10))
        } else if ($0 in buf) {
            printf "%s", buf[$0]
        } else {
            print
        }
    }
    ' "$src" > "$dst"
}

bake_one() {
    local path="$1"
    local src="${FRAMEWORK}/${path}"
    local dst="${BUILT}/${path}"
    if [ ! -f "$src" ]; then
        echo "Error: framework file missing: $src" >&2
        exit 1
    fi
    mkdir -p "$(dirname "$dst")"
    # Remove existing dst (may be read-only from a previous bake)
    rm -f "$dst"
    substitute "$src" "$dst"
    # Preserve file mode from framework
    chmod --reference="$src" "$dst"
    count=$((count + 1))
}

count=0
while IFS= read -r path; do
    [[ -z "$path" || "$path" =~ ^# ]] && continue
    if [[ "$path" == *'*'* || "$path" == *'?'* ]]; then
        # Glob — expand against framework/
        for expanded in "$FRAMEWORK"/$path; do
            [ -f "$expanded" ] || continue
            bake_one "${expanded#$FRAMEWORK/}"
        done
    else
        bake_one "$path"
    fi
done < "$MANIFEST"

echo "Baked ${count} files into ${BUILT}"
