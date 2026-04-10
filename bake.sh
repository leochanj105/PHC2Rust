#!/usr/bin/env bash
set -euo pipefail

# bake.sh — combine framework + project descriptor → concrete tree under built/<project>/
#
# Usage:
#   ./bake.sh <project>
#
# Reads:
#   projects/<project>/manifest.txt   — one path per line, relative to framework/
#
# Writes:
#   built/<project>/<path>             — copied from framework/<path>
#
# This is the minimal version: pure copy, no placeholder substitution yet.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT="${1:-}"

[ -n "$PROJECT" ] || { echo "Usage: $0 <project>" >&2; exit 1; }

PROJECT_DIR="${ROOT}/projects/${PROJECT}"
MANIFEST="${PROJECT_DIR}/manifest.txt"
FRAMEWORK="${ROOT}/framework"
BUILT="${ROOT}/built/${PROJECT}"

[ -f "$MANIFEST" ] || { echo "Error: manifest not found: $MANIFEST" >&2; exit 1; }
[ -d "$FRAMEWORK" ] || { echo "Error: framework not found: $FRAMEWORK" >&2; exit 1; }

# Fresh build dir
rm -rf "$BUILT"
mkdir -p "$BUILT"

count=0
while IFS= read -r path; do
    # skip blank lines and comments
    [[ -z "$path" || "$path" =~ ^# ]] && continue
    src="${FRAMEWORK}/${path}"
    dst="${BUILT}/${path}"
    if [ ! -f "$src" ]; then
        echo "Error: framework file missing: $src" >&2
        exit 1
    fi
    mkdir -p "$(dirname "$dst")"
    cp -p "$src" "$dst"
    count=$((count + 1))
done < "$MANIFEST"

echo "Baked ${count} files into ${BUILT}"
