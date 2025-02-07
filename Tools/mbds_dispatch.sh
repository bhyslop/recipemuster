#!/bin/bash

# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Dispatch shell script

set -euo pipefail

# Set verbose mode with MBDS_VERBOSE=1 or MBDS_VERBOSE=2 before command
zMBD_VERBOSE=${MBDS_VERBOSE:-0}
zMBD_SHOW() { test "$zMBD_VERBOSE" != "1" || echo "dispatch: $1"; }
test               "$zMBD_VERBOSE" != "2" || set -x

zMBD_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
zMBD_SHOW "Changed to repository root"

zMBD_SHOW "Source variables file and validate"
zMBD_VARIABLES=./mbv.variables.sh
source ${zMBD_VARIABLES}
: ${zMBD_VARIABLES:?}        && zMBD_SHOW "Variables file: ${zMBD_VARIABLES}"
: ${MBV_STATION_FILE:?}     && zMBD_SHOW "Station file:   ${MBV_STATION_FILE}"
: ${MBV_LOG_DIR:?}          && zMBD_SHOW "Log directory:  ${MBV_LOG_DIR}"
: ${MBV_LOG_LAST:?}         && zMBD_SHOW "Latest log:     ${MBV_LOG_LAST}"
: ${MBV_LOG_EXT:?}          && zMBD_SHOW "Log extension:  ${MBV_LOG_EXT}"
: ${MBV_MAKEFILE:?}         && zMBD_SHOW "Makefile:       ${MBV_MAKEFILE}"

zMBD_SHOW "Source station file and validate"
source $MBV_STATION_FILE
: ${MBDS_MAX_JOBS:?}         && zMBD_SHOW "Max jobs:       ${MBDS_MAX_JOBS}"

MBDS_NOW_STAMP=$(date +'%Y%m%d-%H%M%Sp%N')
zMBD_SHOW "Generated timestamp: $MBDS_NOW_STAMP"

zMBDS_JP_ARG=$1
zMBDS_OM_ARG=$2
zMBDS_TARGET=$3
shift 3

zMBD_SHOW "Validating job profile"
case "$zMBDS_JP_ARG" in
  jp_single) zMBDS_JOB_PROFILE=1                ;;
  jp_full)   zMBDS_JOB_PROFILE=$MBDS_MAX_JOBS   ;;
  *) zMBD_SHOW "Invalid job profile: $zMBDS_JP_ARG"; exit 1 ;;
esac

zMBD_SHOW "Validating output mode"
case "$zMBDS_OM_ARG" in
  om_line)   zMBDS_OUTPUT_MODE="-Oline"     ;;
  om_target) zMBDS_OUTPUT_MODE="-Orecurse"  ;;
  *) zMBD_SHOW "Invalid output mode: $zMBDS_OM_ARG"; exit 1 ;;
esac

zMBD_SHOW "Extract tokens from tabtarget so make can use them in all places"
zMBD_SHOW "tabtarget tokenizing: $zMBDS_TARGET"
IFS='.' read -ra MBDS_TOKENS <<< "$zMBDS_TARGET"
zMBD_SHOW "Split tokens: ${MBDS_TOKENS[*]}"

MBDS_TOKEN_PARAMS=()
for i in "${!MBDS_TOKENS[@]}"; do
    [[ -z "${MBDS_TOKENS[$i]}" ]] || MBDS_TOKEN_PARAMS+=("MBDM_PARAMETER_$i=${MBDS_TOKENS[$i]}")
done
zMBD_SHOW "Token parameters: ${MBDS_TOKEN_PARAMS[*]}"

zMBDS_LOG_LAST=$MBV_LOG_DIR/$MBV_LOG_LAST.$MBV_LOG_EXT
zMBDS_LOG_SAME=$MBV_LOG_DIR/same-$zMBDS_TARGET.$MBV_LOG_EXT
zMBDS_LOG_HIST=$MBV_LOG_DIR/hist-$MBDS_NOW_STAMP-$zMBDS_TARGET.$MBV_LOG_EXT

zMBD_SHOW "Log paths:"
zMBD_SHOW "  DIR:   $MBV_LOG_DIR"
zMBD_SHOW "  LAST:  $zMBDS_LOG_LAST"
zMBD_SHOW "  SAME:  $zMBDS_LOG_SAME"

echo "Historical log: $zMBDS_LOG_HIST"

zMBD_SHOW "Assure log directory exists..."
mkdir -p "$MBV_LOG_DIR"

cmd_parts=(
    "make -f $MBV_MAKEFILE"
    "$zMBDS_OUTPUT_MODE -j $zMBDS_JOB_PROFILE"
    "$zMBDS_TARGET"
    "MBDM_NOW_STAMP=$MBDS_NOW_STAMP"
    "MBDM_JOB_PROFILE=$zMBDS_JOB_PROFILE"
    "${MBDS_TOKEN_PARAMS[*]}"
    "$@"
)

MBDS_MAKE_CMD="${cmd_parts[*]}"

echo "Executing: $MBDS_MAKE_CMD"      | tee    "$zMBDS_LOG_LAST" "$zMBDS_LOG_SAME" "$zMBDS_LOG_HIST"
eval            "$MBDS_MAKE_CMD" 2>&1 | tee -a "$zMBDS_LOG_LAST" "$zMBDS_LOG_SAME" "$zMBDS_LOG_HIST"

MBDS_EXIT_STATUS="${PIPESTATUS[0]}"
zMBD_SHOW "Make completed with status: $MBDS_EXIT_STATUS"

exit "$MBDS_EXIT_STATUS"
