#!/usr/bin/env bash
# config_overrides.sh — S1: naive one-shot test generation

: "${RUST_DIR:=${EXP_DIR}/rust-s1}"
: "${WORK_DIR:=${EXP_DIR}/work-s1}"
: "${COVERAGE_MODES:=function}"
: "${MAX_ROUNDS:=1}"
: "${STALL_LIMIT:=1}"
