#!/usr/bin/env bash
# config_overrides.sh — S2: explicit all-functions one-shot test generation

: "${RUST_DIR:=${EXP_DIR}/rust-s2}"
: "${WORK_DIR:=${EXP_DIR}/work-s2}"
: "${COVERAGE_MODES:=function}"
: "${MAX_ROUNDS:=1}"
: "${STALL_LIMIT:=1}"
