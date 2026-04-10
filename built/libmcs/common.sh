#!/usr/bin/env bash
# common.sh — shared constants and helpers for all phase scripts.
# Source this from every phase script.

COMMON_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export EXP_DIR="$COMMON_DIR"
export HARNESS_DIR="/home/leochanj/Desktop/progress_harness"
export LIBMCS="/home/leochanj/Desktop/libmcs/libm"
export C_SRC_DIRS="${LIBMCS}/mathd ${LIBMCS}/mathf ${LIBMCS}/common ${LIBMCS}/complexd ${LIBMCS}/complexf"
export C_INCLUDE_DIRS="${LIBMCS}/include"

SCENARIOS=(s1_naive s2_explicit s3_edgecase s4_function s5_branch s6_branch_extended)

# ── API key ──
# If ANTHROPIC_API_KEY is set, claude CLI uses it (API billing).
# If not set, claude CLI falls back to your logged-in account (Pro/Team plan).
if [ -n "${ANTHROPIC_API_KEY:-}" ]; then
    echo "  Using API key: ${ANTHROPIC_API_KEY:0:12}..."
else
    echo "  Using logged-in Claude account (no API key set)"
fi

export CODE_GEN_CMD="${CODE_GEN_CMD:-claude}"
export ANALYSIS_CMD="${ANALYSIS_CMD:-claude}"

# ── Ensure CLAUDE.md + settings.json exist at HARNESS_DIR ──
# Claude CLI loads .claude/CLAUDE.md as system prompt and .claude/settings.json
# for permissions. We use a lightweight CLAUDE.md (just instructions, no source
# embedding) because the full source (~22K lines) exceeds Sonnet's 200K context.
# The AI reads source files from disk instead.
_ensure_claude_setup() {
    local claude_dir="${HARNESS_DIR}/.claude"

    # Skip if already set up (avoids race condition with parallel scripts)
    if [ -f "${claude_dir}/CLAUDE.md" ] && [ -f "${claude_dir}/settings.json" ]; then
        return
    fi

    mkdir -p "$claude_dir"

    # Lightweight CLAUDE.md — instructions only, no embedded source
    cat > "${claude_dir}/CLAUDE.md" <<'CLAUDEEOF'
# Project Context

## Library
Libmcs — a C math library (libm implementation). IEEE 754 compliant.

## Comparison rule
All outputs must be bitwise exact. Use %a hex float format. Zero tolerance.

## Source layout
- /home/leochanj/Desktop/libmcs/libm/mathd/       — double-precision functions
- /home/leochanj/Desktop/libmcs/libm/mathf/       — float-precision functions
- /home/leochanj/Desktop/libmcs/libm/complexd/    — complex double functions
- /home/leochanj/Desktop/libmcs/libm/complexf/    — complex float functions
- /home/leochanj/Desktop/libmcs/libm/common/      — shared utilities
- /home/leochanj/Desktop/libmcs/libm/include/     — headers (math.h, complex.h, fenv.h)
Each directory has an internal/ subdirectory with helper functions.
CLAUDEEOF

    # Settings.json — allow access to libmcs source and experiment directories.
    # Block ALL of testing/ (judger + other held-out tests) via both tool deny
    # rules AND Bash deny. Bash deny prevents cat/grep bypass of Read deny.
    cat > "${claude_dir}/settings.json" <<SETTINGSEOF
{
  "permissions": {
    "allow": [
      "Read(./**)", "Write(./**)", "Edit(./**)", "Glob(./**)", "Grep(./**)",
      "Bash",
      "Read(//tmp/**)", "Write(//tmp/**)", "Edit(//tmp/**)",
      "Read(//home/leochanj/Desktop/libmcs/libm/**)",
      "Glob(//home/leochanj/Desktop/libmcs/libm/**)",
      "Grep(//home/leochanj/Desktop/libmcs/libm/**)",
      "Read(//home/leochanj/Desktop/libmcs/newexp/**)",
      "Write(//home/leochanj/Desktop/libmcs/newexp/**)",
      "Edit(//home/leochanj/Desktop/libmcs/newexp/**)",
      "Glob(//home/leochanj/Desktop/libmcs/newexp/**)",
      "Grep(//home/leochanj/Desktop/libmcs/newexp/**)"
    ],
    "deny": [
      "WebFetch", "WebSearch",
      "Read(//home/leochanj/Desktop/libmcs/testing/**)",
      "Glob(//home/leochanj/Desktop/libmcs/testing/**)",
      "Grep(//home/leochanj/Desktop/libmcs/testing/**)"
    ]
  }
}
SETTINGSEOF

    echo "  Generated CLAUDE.md (lightweight) + settings.json"
}
# Defer actual call until after function definitions

# ── Parse common flags ──
VERBOSE=""
ONLY_SCENARIO="all"
_common_parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            -v) VERBOSE="-v" ;;
            --scenario) ONLY_SCENARIO="$2"; shift ;;
            *) echo "Unknown option: $1"; exit 1 ;;
        esac
        shift
    done
}

should_run_scenario() {
    [ "$ONLY_SCENARIO" = "all" ] || [ "$ONLY_SCENARIO" = "$1" ]
}

short_name() {
    # s1_naive -> s1
    echo "${1%%_*}"
}

log() {
    echo ""
    echo "========================================"
    echo "$@"
    echo "========================================"
}

# ── Expand __RUST_DIR__ in a prompt file ──
expand_prompt() {
    local src="$1" dest="$2" rust_dir="$3"
    sed -e "s|__RUST_DIR__|${rust_dir}|g" "$src" > "$dest"
}

# Run the deferred setup now
_ensure_claude_setup

# ── Pre-extract function signatures (shared across testgen scripts) ──
export SIGS_FILE="${EXP_DIR}/work-signatures.md"
if [ ! -f "$SIGS_FILE" ]; then
    echo "Extracting function signatures..."
    python3 "${EXP_DIR}/scripts/extract_signatures.py" > "$SIGS_FILE"
    echo "  $(wc -l < "$SIGS_FILE") lines -> ${SIGS_FILE}"
fi

export FUNC_MAP="${EXP_DIR}/work-func-map.txt"
if [ ! -f "$FUNC_MAP" ]; then
    echo "Building function -> file map..."
    python3 "${EXP_DIR}/scripts/build_func_map.py" > "$FUNC_MAP"
    echo "  $(wc -l < "$FUNC_MAP") entries -> ${FUNC_MAP}"
fi

# ── Set up a scenario's workdir with expanded prompts ──
setup_scenario_workdir() {
    local scenario="$1"
    local short
    short=$(short_name "$scenario")
    local work_dir="${EXP_DIR}/work-${short}"
    local rust_dir="${EXP_DIR}/rust-${short}"

    mkdir -p "${work_dir}/prompts" "${work_dir}/testgen" "${work_dir}/diffgen" "${work_dir}/difffix"

    # Expand all prompts with scenario-specific RUST_DIR
    # Copy ALL .md files from prompts/ (includes s1-s5 testgen variants)
    for prompt_file in "${EXP_DIR}/prompts/"*.md; do
        [ -f "$prompt_file" ] || continue
        local prompt
        prompt=$(basename "$prompt_file" .md)
        local src="${EXP_DIR}/prompts/${prompt}.md"
        [ -f "$src" ] || continue
        expand_prompt "$src" "${work_dir}/prompts/${prompt}.md" "$rust_dir"
    done

    # Copy lightweight .claude/ from harness for prompt caching + permissions
    if [ ! -d "${work_dir}/.claude" ]; then
        cp -r "${HARNESS_DIR}/.claude" "${work_dir}/.claude"
    fi

    # Copy canonical test bridge to testgen dir
    local testgen="${work_dir}/testgen"
    if [ -f "${EXP_DIR}/test_bridge.c" ]; then
        cp "${EXP_DIR}/test_bridge.c" "${testgen}/test_bridge.c"
        cp "${EXP_DIR}/test_bridge.h" "${testgen}/test_bridge.h"
    fi
}

# ── Copy baseline Rust to scenario directory ──
copy_rust_for_scenario() {
    local scenario="$1"
    local short
    short=$(short_name "$scenario")
    local rust_dest="${EXP_DIR}/rust-${short}"

    if [ -d "$rust_dest" ] && [ -f "${rust_dest}/Cargo.toml" ]; then
        echo "  ${rust_dest} already exists — skipping copy."
        return
    fi

    local rust_src="${EXP_DIR}/rust-baseline-test"
    if [ ! -d "$rust_src" ]; then
        rust_src="${EXP_DIR}/rust-baseline"
        echo "  WARNING: rust-baseline-test not found, using rust-baseline (no test bridge)"
    fi
    echo "  Copying $(basename "$rust_src") -> rust-${short}..."
    cp -r "$rust_src" "$rust_dest"
}

# ── Export scenario env vars (used by harness scripts) ──
export_scenario_env() {
    local scenario="$1"
    local short
    short=$(short_name "$scenario")
    local scenario_dir="${EXP_DIR}/scenarios/${scenario}"
    local work_dir="${EXP_DIR}/work-${short}"

    export PROJECT_DIR="$scenario_dir"
    export TEST_CASE_DIR="$LIBMCS"
    export RUST_DIR="${EXP_DIR}/rust-${short}"
    export WORK_DIR="$work_dir"
    export TESTGEN_WORKDIR="${work_dir}/testgen"
    export DIFFGEN_WORKDIR="${work_dir}/diffgen"
    export DIFFFIX_WORKDIR="${work_dir}/difffix"
    export EXPANDED_PROMPTS_DIR="${work_dir}/prompts"
    export C_SRC_DIRS C_INCLUDE_DIRS
    export CONFIGS_FILE=""
}
