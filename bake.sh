#!/usr/bin/env bash
set -euo pipefail

# bake.sh — combine framework + project values → concrete tree under built/<project>/
#
# Usage:
#   ./bake.sh <project>
#
# Reads:
#   projects/<project>/manifest.txt   — one path per line, relative to framework/
#   projects/<project>/values         — section-delimited placeholder map
#
# Writes:
#   built/<project>/<path>             — substituted copy of framework/<path>
#
# Substitution model:
#   - values file uses ==__NAME__== section markers, terminated by ==END==
#   - in framework files, any line consisting EXACTLY of __NAME__ is replaced
#     by the section's content (which may be multiple lines).
#   - non-placeholder lines pass through verbatim.
#   - if no placeholders match, the file is copied byte-identically.

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

# Fresh build dir
rm -rf "$BUILT"
mkdir -p "$BUILT"

substitute() {
    local src="$1" dst="$2"
    awk -v values_file="$VALUES" '
    BEGIN {
        # First load the values file into a map.
        current = ""
        while ((getline line < values_file) > 0) {
            if (line ~ /^==__[A-Z_]+__==$/) {
                # strip leading "==" and trailing "=="
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
    {
        if ($0 in buf) {
            # Substitute multi-line value, no extra newline (value already has trailing \n)
            printf "%s", buf[$0]
        } else {
            print
        }
    }
    ' "$src" > "$dst"
}

count=0
while IFS= read -r path; do
    [[ -z "$path" || "$path" =~ ^# ]] && continue
    src="${FRAMEWORK}/${path}"
    dst="${BUILT}/${path}"
    if [ ! -f "$src" ]; then
        echo "Error: framework file missing: $src" >&2
        exit 1
    fi
    mkdir -p "$(dirname "$dst")"
    substitute "$src" "$dst"
    # Preserve file mode from framework
    chmod --reference="$src" "$dst"
    count=$((count + 1))
done < "$MANIFEST"

echo "Baked ${count} files into ${BUILT}"
