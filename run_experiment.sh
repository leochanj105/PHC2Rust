#!/usr/bin/env bash
set -euo pipefail

# run_experiment.sh — convenience wrapper that runs all phases in order.
#
# Usage:
#   ./run_experiment.sh [-v]
#
# Or run phases individually:
#   ./01_transpile.sh      [-v]
#   ./02a_testgen_s1.sh    [-v]
#   ./02b_testgen_s2.sh    [-v]
#   ./02c_testgen_s3.sh    [-v]
#   ./02d_testgen_s4.sh    [-v]
#   ./03_diffgen.sh        [-v] [--scenario S]
#   ./04_difffix.sh        [-v] [--scenario S]
#   ./05_judge.sh          [-v] [--scenario S]

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"${DIR}/01_transpile.sh" "$@"
"${DIR}/02a_testgen_s1.sh" "$@"
"${DIR}/02b_testgen_s2.sh" "$@"
"${DIR}/02c_testgen_s3.sh" "$@"
"${DIR}/02d_testgen_s4.sh" "$@"
"${DIR}/02e_testgen_s5.sh" "$@"
"${DIR}/03_diffgen.sh" "$@"
"${DIR}/04_difffix.sh" "$@"
"${DIR}/05_judge.sh" "$@"

echo ""
echo "========================================"
echo "EXPERIMENT COMPLETE"
echo "========================================"
echo "Results in: ${DIR}/results/"
