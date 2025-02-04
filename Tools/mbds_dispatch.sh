#!/bin/bash

set -euo pipefail

zMBDS_VERBOSE=${MBDS_VERBOSE:-0}
MBDS_SHOW() { test "$zMBDS_VERBOSE" != "1" || echo "dispatch: $1"; }

MBDS_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
MBDS_SHOW "Changed to repository root"

MBDS_SHOW "Source variables file and station file"
source ./mbds-variables.shmk
source $MBDS_STATION_FILE

MBDS_NOW=$(date +'%Y%m%d-%H%M%Sp%N')
MBDS_SHOW "Generated timestamp: $MBDS_NOW"

MBDS_TABTARGET_BASENAME=$1
shift
MBDS_SHOW "Processing tabtarget: $MBDS_TABTARGET_BASENAME"

IFS='.' read -ra MBDS_TOKENS <<< "$MBDS_TABTARGET_BASENAME"
MBDS_SHOW "Split tokens: ${MBDS_TOKENS[*]}"

MBDS_SHOW "Make arguments: $*"

MBDS_TOKEN_PARAMS=()
for i in "${!MBDS_TOKENS[@]}"; do
    [[ -z "${MBDS_TOKENS[$i]}" ]] || MBDS_TOKEN_PARAMS+=("RBC_PARAMETER_$i=${MBDS_TOKENS[$i]}")
done
MBDS_SHOW "Token parameters: ${MBDS_TOKEN_PARAMS[*]}"

if [[ "${MBDS_TOKENS[0]:-}" == "s" ]]; then
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_SINGLE:-1}
    MBDS_OUTPUT_SYNC="-Oline"
    MBDS_SHOW "Single-threaded mode selected"
else
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_MAX:-$(nproc)}
    MBDS_OUTPUT_SYNC="-Orecurse"
    MBDS_SHOW "Multi-threaded mode with $MBDS_MAKE_JOBS jobs"
fi

MBDS_LOG_DIR=$USIV_LOG_ABSDIR
MBDS_LOG_LAST=$USIV_LOG_LAST
MBDS_LOG_SAME=$MBDS_LOG_DIR/same-$MBDS_TABTARGET_BASENAME.$USIV_LOG_EXTENSION
MBDS_LOG_HIST=$MBDS_LOG_DIR/hist-$MBDS_NOW-$MBDS_TABTARGET_BASENAME.$USIV_LOG_EXTENSION

MBDS_SHOW "Log paths:"
MBDS_SHOW "  DIR:  $MBDS_LOG_DIR"
MBDS_SHOW "  LAST: $MBDS_LOG_LAST"
MBDS_SHOW "  SAME: $MBDS_LOG_SAME"

echo "Historical log will be written to: $MBDS_LOG_HIST"

MBDS_SHOW "Creating log directory"
mkdir -p "$MBDS_LOG_DIR"

MBDS_TIMESTAMP_VAR="${USIV_MAKE_TIMESTAMP_VAR:-MAKE_TIMESTAMP}"
MBDS_SHOW "Using timestamp variable: $MBDS_TIMESTAMP_VAR"

MBDS_SHOW "Executing make command..."
make -f "$USIV_MAKEFILE"                         \
    $MBDS_OUTPUT_SYNC -j "$MBDS_MAKE_JOBS"       \
    "$MBDS_TABTARGET_BASENAME"                   \
    "${MBDS_TOKEN_PARAMS[@]}"                    \
    "$@"                                         \
    "$MBDS_TIMESTAMP_VAR=$MBDS_NOW"              \
    2>&1                                         \
    | tee "$MBDS_LOG_LAST"                       \
    | tee "$MBDS_LOG_SAME"                       \
    | tee "$MBDS_LOG_HIST"

MBDS_EXIT_STATUS="${PIPESTATUS[0]}"
MBDS_SHOW "Make completed with status: $MBDS_EXIT_STATUS"

exit "$MBDS_EXIT_STATUS"
