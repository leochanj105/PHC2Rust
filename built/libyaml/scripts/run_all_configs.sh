#!/usr/bin/env bash
set -euo pipefail

# run_all_configs.sh — runs build_and_cover.sh for every config.
# Single config ("default"), no multi-config.
#
# Usage: run_all_configs.sh -w WORKDIR [-j MAX_JOBS]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

WORKDIR=""
MAX_JOBS=1

while getopts "w:j:" opt; do
    case $opt in
        w) WORKDIR="$OPTARG" ;;
        j) MAX_JOBS="$OPTARG" ;;
        *) echo "Usage: $0 -w WORKDIR [-j MAX_JOBS]"; exit 1 ;;
    esac
done

[ -n "$WORKDIR" ] || { echo "Usage: $0 -w WORKDIR [-j MAX_JOBS]"; exit 1; }

# Single config (no multi-config support)
CONFIGS=("default")
echo "(using configs: ${CONFIGS[*]})"

# Clean stale feedback from previous round
rm -rf "${WORKDIR}/feedback"
mkdir -p "${WORKDIR}/feedback"

for config in "${CONFIGS[@]}"; do
    echo "=== $config ==="
    "${SCRIPT_DIR}/build_and_cover.sh" -w "$WORKDIR" "$config"
done

echo ""
echo "All configs done."
