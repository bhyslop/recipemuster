#!/bin/bash

set -euo pipefail

zMBDS_VERBOSE=${MBDS_VERBOSE:-0}
zMBDS_SHOW() { test "$zMBDS_VERBOSE" != "1" || echo "dispatch: $1"; }

zMBDS_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
zMBDS_SHOW "Changed to repository root"

zMBDS_SHOW "Source variables file and station file"
source ./mbdv-variables.shmk
source  $MBDV_STATION_FILE

MBDS_NOW=$(date +'%Y%m%d-%H%M%Sp%N')
zMBDS_SHOW "Generated timestamp: $MBDS_NOW"

zMBDS_BASENAME=$1
shift
zMBDS_SHOW "Processing tabtarget: $zMBDS_BASENAME"

IFS='.' read -ra MBDS_TOKENS <<< "$zMBDS_BASENAME"
zMBDS_SHOW "Split tokens: ${MBDS_TOKENS[*]}"

zMBDS_SHOW "Make arguments: $*"

MBDS_TOKEN_PARAMS=()
for i in "${!MBDS_TOKENS[@]}"; do
    [[ -z "${MBDS_TOKENS[$i]}" ]] || MBDS_TOKEN_PARAMS+=("RBC_PARAMETER_$i=${MBDS_TOKENS[$i]}")
done
zMBDS_SHOW "Token parameters: ${MBDS_TOKEN_PARAMS[*]}"

if [[ "${MBDS_TOKENS[0]:-}" == "s" ]]; then
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_SINGLE:-1}
    MBDS_OUTPUT_SYNC="-Oline"
    zMBDS_SHOW "Single-threaded mode selected"
else
    MBDS_MAKE_JOBS=${SSISTATIONMK_MAKE_JOBS_MAX:-$(nproc)}
    MBDS_OUTPUT_SYNC="-Orecurse"
    zMBDS_SHOW "Multi-threaded mode with $MBDS_MAKE_JOBS jobs"
fi

zMBDS_LOG_DIR=$MBDV_LOG_DIR
zMBDS_LOG_LAST=$MBDV_LOG_LAST
zMBDS_LOG_SAME=$zMBDS_LOG_DIR/same-$zMBDS_BASENAME.$MBDV_LOG_EXT
zMBDS_LOG_HIST=$zMBDS_LOG_DIR/hist-$MBDS_NOW-$zMBDS_BASENAME.$MBDV_LOG_EXT

zMBDS_SHOW "Log paths:"
zMBDS_SHOW "  DIR:  $zMBDS_LOG_DIR"
zMBDS_SHOW "  LAST: $zMBDS_LOG_LAST"
zMBDS_SHOW "  SAME: $zMBDS_LOG_SAME"

echo "Historical log will be written to: $zMBDS_LOG_HIST"

zMBDS_SHOW "Creating log directory"
mkdir -p "$zMBDS_LOG_DIR"

MBDS_TIMESTAMP_VAR="${USIV_MAKE_TIMESTAMP_VAR:-MAKE_TIMESTAMP}"
zMBDS_SHOW "Using timestamp variable: $MBDS_TIMESTAMP_VAR"

zMBDS_SHOW "Executing make command..."
make -f "$MBDV_MAKEFILE"                         \
    $MBDS_OUTPUT_SYNC -j "$MBDS_MAKE_JOBS"       \
    "$zMBDS_BASENAME"                   \
    "${MBDS_TOKEN_PARAMS[@]}"                    \
    "$@"                                         \
    "$MBDS_TIMESTAMP_VAR=$MBDS_NOW"              \
    2>&1                                         \
    | tee "$zMBDS_LOG_LAST"                      \
    | tee "$zMBDS_LOG_SAME"                      \
    | tee "$zMBDS_LOG_HIST"

MBDS_EXIT_STATUS="${PIPESTATUS[0]}"
zMBDS_SHOW "Make completed with status: $MBDS_EXIT_STATUS"

exit "$MBDS_EXIT_STATUS"

