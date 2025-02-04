#!/bin/bash

set -euo pipefail
test "$zMBDS_VERBOSE" != "1" || set -x

zMBDS_VERBOSE=${MBDS_VERBOSE:-0}
zMBDS_SHOW() { test "$zMBDS_VERBOSE" != "1" || echo "dispatch: $1"; }

zMBDS_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
zMBDS_SHOW "Changed to repository root"

zMBDS_SHOW "Source variables file and validate"
source ./mbdv-variables.shmk
: ${MBDV_STATION_FILE:?}     && zMBDS_SHOW "Station file:  ${MBDV_STATION_FILE}"
: ${MBDV_LOG_DIR:?}          && zMBDS_SHOW "Log directory: ${MBDV_LOG_DIR}"
: ${MBDV_LOG_LAST:?}         && zMBDS_SHOW "Latest log:    ${MBDV_LOG_LAST}"
: ${MBDV_LOG_EXT:?}          && zMBDS_SHOW "Log extension: ${MBDV_LOG_EXT}"
: ${MBDV_MAKEFILE:?}         && zMBDS_SHOW "Makefile:      ${MBDV_MAKEFILE}"

zMBDS_SHOW "Source station file and validate"
source  $MBDV_STATION_FILE

MBDS_NOW=$(date +'%Y%m%d-%H%M%Sp%N')
zMBDS_SHOW "Generated timestamp: $MBDS_NOW"

zMBDS_JP_ARG=$1
zMBDS_OM_ARG=$2
zMBDS_BASENAME=$3
shift 3

zMBDS_SHOW "Validating job profile"
case "$zMBDS_JP_ARG" in
  jp_single) zMBDS_MAKE_JP=1        ;;
  jp_full)   zMBDS_MAKE_JP=$(nproc) ;;
  *) zMBDS_SHOW "Invalid job profile: $zMBDS_JP_ARG"; exit 1 ;;
esac

zMBDS_SHOW "Validating output mode"
case "$zMBDS_OM_ARG" in
  om_line)   zMBDS_OUTPUT_SYNC="-Oline"     ;;
  om_target) zMBDS_OUTPUT_SYNC="-Orecurse"  ;;
  *) zMBDS_SHOW "Invalid output mode: $zMBDS_OM_ARG"; exit 1 ;;
esac

zMBDS_SHOW "tabtarget tokenizing: $zMBDS_BASENAME"
IFS='.' read -ra MBDS_TOKENS <<< "$zMBDS_BASENAME"
zMBDS_SHOW "Split tokens: ${MBDS_TOKENS[*]}"

zMBDS_SHOW "Make arguments: $*"

MBDS_TOKEN_PARAMS=()
for i in "${!MBDS_TOKENS[@]}"; do
    [[ -z "${MBDS_TOKENS[$i]}" ]] || MBDS_TOKEN_PARAMS+=("RBC_PARAMETER_$i=${MBDS_TOKENS[$i]}")
done
zMBDS_SHOW "Token parameters: ${MBDS_TOKEN_PARAMS[*]}"

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
    $zMBDS_OUTPUT_SYNC -j "$zMBDS_MAKE_JP"       \
    "$zMBDS_BASENAME"                            \
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

