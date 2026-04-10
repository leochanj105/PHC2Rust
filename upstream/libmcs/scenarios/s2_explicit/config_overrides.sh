#!/usr/bin/env bash
# config_overrides.sh — S2: explicit all-functions one-shot test generation

_EXP="/home/leochanj/Desktop/libmcs/newexp"

: "${TEST_CASE_DIR:=/home/leochanj/Desktop/libmcs/libm}"
: "${RUST_DIR:=${_EXP}/rust-s2}"
: "${WORK_DIR:=${_EXP}/work-s2}"

: "${C_SRC_DIRS:=${TEST_CASE_DIR}/mathd ${TEST_CASE_DIR}/mathf ${TEST_CASE_DIR}/common ${TEST_CASE_DIR}/complexd ${TEST_CASE_DIR}/complexf}"
: "${C_INCLUDE_DIRS:=${TEST_CASE_DIR}/include}"

: "${CONFIGS_FILE:=}"
: "${COVERAGE_MODES:=function}"
: "${MAX_ROUNDS:=1}"
: "${STALL_LIMIT:=1}"

: "${DIFFTEST_SCRIPT:=${_EXP}/run_difftest.sh}"

: "${JUDGER_DIR:=/home/leochanj/Desktop/libmcs/testing}"
: "${JUDGER_SCRIPT:=${_EXP}/judger_wrapper.sh}"
