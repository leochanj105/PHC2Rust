#!/usr/bin/env bash
# config_overrides.sh — S6: extended branch coverage (5 more rounds on S5)

_EXP="/home/leochanj/Desktop/libmcs/newexp"

: "${TEST_CASE_DIR:=/home/leochanj/Desktop/libmcs/libm}"
: "${RUST_DIR:=${_EXP}/rust-s6}"
: "${WORK_DIR:=${_EXP}/work-s6}"

: "${C_SRC_DIRS:=${TEST_CASE_DIR}/mathd ${TEST_CASE_DIR}/mathf ${TEST_CASE_DIR}/common ${TEST_CASE_DIR}/complexd ${TEST_CASE_DIR}/complexf}"
: "${C_INCLUDE_DIRS:=${TEST_CASE_DIR}/include}"

: "${COVERAGE_MODES:=branch}"
: "${MAX_ROUNDS:=5}"
: "${STALL_LIMIT:=2}"
