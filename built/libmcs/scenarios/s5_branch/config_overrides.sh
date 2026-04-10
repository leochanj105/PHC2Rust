#!/usr/bin/env bash
# config_overrides.sh — S5: multi-round with function + branch coverage feedback

: "${RUST_DIR:=${EXP_DIR}/rust-s5}"
: "${WORK_DIR:=${EXP_DIR}/work-s5}"
: "${COVERAGE_MODES:=branch}"
: "${MAX_ROUNDS:=5}"
: "${STALL_LIMIT:=2}"
