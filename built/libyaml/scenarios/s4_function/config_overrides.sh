#!/usr/bin/env bash
# config_overrides.sh — S4: multi-round with function coverage feedback

: "${RUST_DIR:=${EXP_DIR}/rust-s4}"
: "${WORK_DIR:=${EXP_DIR}/work-s4}"
: "${COVERAGE_MODES:=function}"
: "${MAX_ROUNDS:=5}"
: "${STALL_LIMIT:=2}"
