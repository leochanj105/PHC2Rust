#!/usr/bin/env bash
# config_overrides.sh — S6: extended branch coverage (5 more rounds on S5)

: "${RUST_DIR:=${EXP_DIR}/rust-s6}"
: "${WORK_DIR:=${EXP_DIR}/work-s6}"
: "${COVERAGE_MODES:=branch}"
: "${MAX_ROUNDS:=5}"
: "${STALL_LIMIT:=2}"
