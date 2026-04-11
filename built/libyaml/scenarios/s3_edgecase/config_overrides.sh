#!/usr/bin/env bash
# config_overrides.sh — S3: explicit all-functions + edge case guidance, one-shot

: "${RUST_DIR:=${EXP_DIR}/rust-s3}"
: "${WORK_DIR:=${EXP_DIR}/work-s3}"
: "${COVERAGE_MODES:=function}"
: "${MAX_ROUNDS:=1}"
: "${STALL_LIMIT:=1}"
