#!/usr/bin/env bash
# config_overrides.sh — S5: multi-round with function + branch coverage feedback

_EXP="/home/leochanj/Desktop/libmcs/newexp"

: "${TEST_CASE_DIR:=/home/leochanj/Desktop/libmcs/libm}"
: "${RUST_DIR:=${_EXP}/rust-s5}"
: "${WORK_DIR:=${_EXP}/work-s5}"

: "${C_SRC_DIRS:=${TEST_CASE_DIR}/mathd ${TEST_CASE_DIR}/mathf ${TEST_CASE_DIR}/common ${TEST_CASE_DIR}/complexd ${TEST_CASE_DIR}/complexf}"
: "${C_INCLUDE_DIRS:=${TEST_CASE_DIR}/include}"

: "${CONFIGS_FILE:=}"
: "${COVERAGE_MODES:=branch}"
: "${MAX_ROUNDS:=5}"
: "${STALL_LIMIT:=2}"

: "${DIFFTEST_SCRIPT:=${_EXP}/run_difftest.sh}"

: "${JUDGER_DIR:=/home/leochanj/Desktop/libmcs/testing}"
: "${JUDGER_SCRIPT:=${_EXP}/judger_wrapper.sh}"
