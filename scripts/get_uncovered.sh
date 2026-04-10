#!/usr/bin/env bash
set -euo pipefail

# Usage: get_uncovered.sh <feedback_file> <dest_file>
#
# Reads an llvm-cov show feedback file and outputs all uncovered branch
# conditions (True or False side with count = 0), one per line:
#   xxx.c, Branch (120:5): True
#   xxx.c, Branch (120:5): False

usage() {
    echo "Usage: $0 <feedback_file> <dest_file>"
    exit 1
}

[ $# -ne 2 ] && usage

FEEDBACK="$1"
DEST="$2"

[ -f "$FEEDBACK" ] || { echo "Error: feedback file not found: $FEEDBACK"; exit 1; }

awk '
    # File header: /full/path/to/file.c:
    /^\/[^ ]+\.c:$/ {
        n = split($0, a, "/")
        fname = substr(a[n], 1, length(a[n]) - 1)
        next
    }
    # Branch line: "  |  Branch (line:col): [True: X, False: Y]"
    /Branch \([0-9]+:[0-9]+\):/ {
        match($0, /Branch \([0-9]+:[0-9]+\)/)
        branch = fname ", " substr($0, RSTART, RLENGTH)

        # Extract True count (between "True: " and ",")
        split($0, t, "True: ")
        split(t[2], tv, ",")
        true_val = tv[1]
        gsub(/[[:space:]]/, "", true_val)

        # Extract False count (between "False: " and "]")
        split($0, f, "False: ")
        split(f[2], fv, "]")
        false_val = fv[1]
        gsub(/[[:space:]]/, "", false_val)

        # Guard: if extraction failed, skip this line
        if (true_val == "" || false_val == "") next

        if (true_val == "0")  print branch ": True"
        if (false_val == "0") print branch ": False"
    }
' "$FEEDBACK" > "$DEST"

echo "Written to: $DEST ($(wc -l < "$DEST") uncovered conditions)"
