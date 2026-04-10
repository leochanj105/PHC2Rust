#!/usr/bin/env bash
set -euo pipefail

# verify.sh — diff built/<project>/ against upstream/libmcs/ by sha256.
#
# Usage:
#   ./verify.sh <project> [--partial]
#
# Reports four buckets:
#   MATCH        — same file, same hash
#   MISMATCH     — same path exists in both, different hash
#   MISSING      — in upstream but not in built (we haven't baked it yet)
#   EXTRA        — in built but not in upstream (bake produced something extra — bug)
#
# Exit 0 only if MISMATCH and EXTRA are both empty AND MISSING is empty.
# (Use --partial to allow MISSING during incremental work.)

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT="${1:-}"
PARTIAL=0
shift || true
for arg in "$@"; do
    case "$arg" in
        --partial) PARTIAL=1 ;;
    esac
done

[ -n "$PROJECT" ] || { echo "Usage: $0 <project> [--partial]" >&2; exit 1; }

UPSTREAM="${ROOT}/upstream/libmcs"
BUILT="${ROOT}/built/${PROJECT}"

[ -d "$UPSTREAM" ] || { echo "Error: upstream not found: $UPSTREAM" >&2; exit 1; }
[ -d "$BUILT" ] || { echo "Error: built not found: $BUILT (run ./bake.sh $PROJECT first)" >&2; exit 1; }

declare -A up_hash bt_hash

while IFS= read -r f; do
    rel="${f#$UPSTREAM/}"
    h=$(sha256sum "$f" | awk '{print $1}')
    up_hash["$rel"]="$h"
done < <(find "$UPSTREAM" -type f | sort)

while IFS= read -r f; do
    rel="${f#$BUILT/}"
    h=$(sha256sum "$f" | awk '{print $1}')
    bt_hash["$rel"]="$h"
done < <(find "$BUILT" -type f | sort)

match=()
mismatch=()
missing=()
extra=()

for rel in "${!up_hash[@]}"; do
    if [ -z "${bt_hash[$rel]+_}" ]; then
        missing+=("$rel")
    elif [ "${up_hash[$rel]}" = "${bt_hash[$rel]}" ]; then
        match+=("$rel")
    else
        mismatch+=("$rel")
    fi
done

for rel in "${!bt_hash[@]}"; do
    if [ -z "${up_hash[$rel]+_}" ]; then
        extra+=("$rel")
    fi
done

# Sort each bucket (handles empty arrays cleanly)
sort_bucket() {
    local -n arr=$1
    if [ "${#arr[@]}" -gt 0 ]; then
        IFS=$'\n' read -r -d '' -a arr < <(printf '%s\n' "${arr[@]}" | sort && printf '\0') || true
    fi
}
sort_bucket match
sort_bucket mismatch
sort_bucket missing
sort_bucket extra

echo "============================================================"
echo "Verify: built/${PROJECT}/  vs  upstream/libmcs/"
echo "============================================================"
echo "  MATCH:    ${#match[@]}"
echo "  MISMATCH: ${#mismatch[@]}"
echo "  MISSING:  ${#missing[@]}"
echo "  EXTRA:    ${#extra[@]}"
echo ""

if [ "${#match[@]}" -gt 0 ]; then
    echo "--- MATCH (${#match[@]}) ---"
    for f in "${match[@]}"; do echo "  ✓ $f"; done
    echo ""
fi

if [ "${#mismatch[@]}" -gt 0 ]; then
    echo "--- MISMATCH (${#mismatch[@]}) ---"
    for f in "${mismatch[@]}"; do
        echo "  ✗ $f"
        echo "    upstream: ${up_hash[$f]}"
        echo "    built:    ${bt_hash[$f]}"
    done
    echo ""
fi

if [ "${#extra[@]}" -gt 0 ]; then
    echo "--- EXTRA (${#extra[@]}) ---"
    for f in "${extra[@]}"; do echo "  + $f"; done
    echo ""
fi

if [ "${#missing[@]}" -gt 0 ]; then
    if [ "$PARTIAL" -eq 1 ]; then
        echo "--- MISSING (${#missing[@]}, ignored due to --partial) ---"
    else
        echo "--- MISSING (${#missing[@]}) ---"
    fi
    # Truncate at 20 entries when there are many
    n=0
    for f in "${missing[@]}"; do
        echo "  - $f"
        n=$((n+1))
        if [ "$n" -ge 20 ] && [ "${#missing[@]}" -gt 20 ]; then
            echo "  ... and $((${#missing[@]} - n)) more"
            break
        fi
    done
    echo ""
fi

if [ "${#mismatch[@]}" -gt 0 ] || [ "${#extra[@]}" -gt 0 ]; then
    exit 1
fi
if [ "${#missing[@]}" -gt 0 ] && [ "$PARTIAL" -eq 0 ]; then
    exit 1
fi
exit 0
