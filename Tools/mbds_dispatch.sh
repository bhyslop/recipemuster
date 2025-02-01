#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
set -euo pipefail

# Support verbose output via environment variable
MBDS_VERBOSE=${MBDSENV_VERBOSE:-0}
function verbose_echo() {
    if [[ "$MBDS_VERBOSE" == "1" ]]; then
        echo "dispatch: $1"
    fi
}

verbose_echo "Starting dispatch script"

# Get to repository root directory from tabtarget location
MBDS_SCRIPT_DIR="$(dirname "$0")"
cd "$MBDS_SCRIPT_DIR/.."
verbose_echo "Changed to repository root"

# Source variables file
source ./usi-variables.sh
verbose_echo "Sourced variables file"

# Generate timestamp for this run
MBDS_NOW=$(date +'%Y%m%d-%H%M%Sp%N')
verbose_echo "Generated timestamp: $MBDS_NOW"

# First argument is tabtarget name without tt/ prefix or .sh suffix
MBDS_TABTARGET_BASENAME=$1
shift
verbose_echo "Processing tabtarget: $MBDS_TABTARGET_BASENAME"

# Split tabtarget name by '.' and store tokens in array
IFS='.' read -ra MBDS_TOKENS <<< "$MBDS_TABTARGET_BASENAME"
verbose_echo "Split tokens: ${MBDS_TOKENS[*]}"

# Extract any additional make arguments
MBDS_MAKE_ARGS=()
while (( $# )); do
    case $1 in
        *) MBDS_MAKE_ARGS+=("$1")
        ;;
    esac
    shift
done
verbose_echo "Collected make arguments: ${MBDS_MAKE_ARGS[*]}"

# Construct array of token parameters
MBDS_TOKEN_PARAMS=()
for i in "${!MBDS_TOKENS[@]}"; do
    if [[ -n "${MBDS_TOKENS[$i]}" ]]; then
        MBDS_TOKEN_PARAMS+=("RBC_PARAMETER_$i=${MBDS_TOKENS[$i]}")
    fi
done
verbose_echo "Constructed token parameters: ${MBDS_TOKEN_PARAMS[*]}"

# Determine thread profile and output synchronization
if [[ "${MBDS_TOKENS[0]:-}" == "s" ]]; then
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_SINGLE:-1}
    MBDS_OUTPUT_SYNC="-Oline"
    verbose_echo "Single-threaded mode selected"
else
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_MAX:-$(nproc)}
    MBDS_OUTPUT_SYNC="-Orecurse"
    verbose_echo "Multi-threaded mode selected with $MBDS_MAKE_JOBS jobs"
fi

# Set up logging paths
MBDS_LOG_DIR=$USIV_LOG_ABSDIR
MBDS_LOG_LAST=$USIV_LOG_LAST
MBDS_LOG_SAME=$MBDS_LOG_DIR/same-$MBDS_TABTARGET_BASENAME.$USIV_LOG_EXTENSION
MBDS_LOG_HIST=$MBDS_LOG_DIR/hist-$MBDS_NOW-$MBDS_TABTARGET_BASENAME.$USIV_LOG_EXTENSION

verbose_echo "Set up log paths:"
verbose_echo "  DIR:  $MBDS_LOG_DIR"
verbose_echo "  LAST: $MBDS_LOG_LAST"
verbose_echo "  SAME: $MBDS_LOG_SAME"

# Display historical log path that will be created
echo "Historical log will be written to: $MBDS_LOG_HIST"

# Ensure log directory exists
mkdir -p "$MBDS_LOG_DIR"
verbose_echo "Ensured log directory exists"

# Pass timestamp to make using variable name from variables file
MBDS_TIMESTAMP_VAR="${USIV_MAKE_TIMESTAMP_VAR:-MAKE_TIMESTAMP}"
verbose_echo "Using timestamp variable: $MBDS_TIMESTAMP_VAR"

# Construct and execute make command with logging
verbose_echo "Executing make command..."
make -f "$USIV_MAKEFILE"                  \
    $MBDS_OUTPUT_SYNC -j "$MBDS_MAKE_JOBS" \
    "$MBDS_TABTARGET_BASENAME"            \
    "${MBDS_TOKEN_PARAMS[@]}"             \
    "${MBDS_MAKE_ARGS[@]}"                \
    "$MBDS_TIMESTAMP_VAR=$MBDS_NOW"       \
    2>&1                                  \
    | tee "$MBDS_LOG_LAST"                \
    | tee "$MBDS_LOG_SAME"                \
    | tee "$MBDS_LOG_HIST"

MBDS_EXIT_STATUS="${PIPESTATUS[0]}"
verbose_echo "Make completed with status: $MBDS_EXIT_STATUS"

exit "$MBDS_EXIT_STATUS"

