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

# Set verbose mode with MBD_VERBOSE=1 or MBD_VERBOSE=2 before command
zMBD_VERBOSE=${MBD_VERBOSE:-0}
zMBD_SHOW() { test "$zMBD_VERBOSE" != "1" || echo "dispatch: $1"; }
test               "$zMBD_VERBOSE" != "2" || set -x

zMBD_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
zMBD_SHOW "Changed to repository root, cwd for all ops dispatched"

zMBD_SHOW "Source variables file and validate"
zMBD_VARIABLES=./mbv.variables.sh
source ${zMBD_VARIABLES}
: ${zMBD_VARIABLES:?}       && zMBD_SHOW "Variables file: ${zMBD_VARIABLES}"
: ${MBV_STATION_FILE:?}     && zMBD_SHOW "Station file:   ${MBV_STATION_FILE}"
: ${MBV_LOG_DIR:?}          && zMBD_SHOW "Log directory:  ${MBV_LOG_DIR}"
: ${MBV_LOG_LAST:?}         && zMBD_SHOW "Latest log:     ${MBV_LOG_LAST}"
: ${MBV_LOG_EXT:?}          && zMBD_SHOW "Log extension:  ${MBV_LOG_EXT}"
: ${MBV_MAKEFILE:?}         && zMBD_SHOW "Makefile:       ${MBV_MAKEFILE}"

zMBD_SHOW "Source station file and validate"
source $MBV_STATION_FILE
: ${MBS_MAX_JOBS:?}         && zMBD_SHOW "Max jobs:       ${MBS_MAX_JOBS}"

zMBD_NOW_STAMP=$(date +'%Y%m%d-%H%M%Sp%N')
zMBD_SHOW "Generated timestamp: $zMBD_NOW_STAMP"

zMBD_JP_ARG=$1
zMBD_OM_ARG=$2
zMBD_TARGET=$3
shift 3

zMBD_SHOW "Validating job profile"
case "$zMBD_JP_ARG" in
  jp_single) zMBD_JOB_PROFILE=1               ;;
  jp_full)   zMBD_JOB_PROFILE=$MBS_MAX_JOBS   ;;
  *) zMBD_SHOW "Invalid job profile: $zMBD_JP_ARG"; exit 1 ;;
esac

zMBD_SHOW "Validating output mode"
case "$zMBD_OM_ARG" in
  om_line)   zMBD_OUTPUT_MODE="-Oline"     ;;
  om_target) zMBD_OUTPUT_MODE="-Orecurse"  ;;
  *) zMBD_SHOW "Invalid output mode: $zMBD_OM_ARG"; exit 1 ;;
esac

zMBD_SHOW "Extract tokens from tabtarget so make can use them in all places"
zMBD_SHOW "tabtarget tokenizing: $zMBD_TARGET"
IFS='.' read -ra zMBD_TOKENS <<< "$zMBD_TARGET"
zMBD_SHOW "Split tokens: ${zMBD_TOKENS[*]}"

zMBD_TOKEN_PARAMS=()
for i in "${!zMBD_TOKENS[@]}"; do
    [[ -z "${zMBD_TOKENS[$i]}" ]] || zMBD_TOKEN_PARAMS+=("MBDM_PARAMETER_$i=${zMBD_TOKENS[$i]}")
done
zMBD_SHOW "Token parameters: ${zMBD_TOKEN_PARAMS[*]}"

zMBD_LOG_LAST=$MBV_LOG_DIR/$MBV_LOG_LAST.$MBV_LOG_EXT
zMBD_LOG_SAME=$MBV_LOG_DIR/same-$zMBD_TARGET.$MBV_LOG_EXT
zMBD_LOG_HIST=$MBV_LOG_DIR/hist-$zMBD_NOW_STAMP-$zMBD_TARGET.$MBV_LOG_EXT

zMBD_SHOW "Log paths:"
zMBD_SHOW "  DIR:   $MBV_LOG_DIR"
zMBD_SHOW "  LAST:  $zMBD_LOG_LAST"
zMBD_SHOW "  SAME:  $zMBD_LOG_SAME"

echo "Historical log: $zMBD_LOG_HIST"

zMBD_SHOW "Assure log directory exists..."
mkdir -p "$MBV_LOG_DIR"

cmd_parts=(
    "make -f $MBV_MAKEFILE"
    "$zMBD_OUTPUT_MODE -j $zMBD_JOB_PROFILE"
    "$zMBD_TARGET"
    "MBDM_NOW_STAMP=$zMBD_NOW_STAMP"
    "MBDM_JOB_PROFILE=$zMBD_JOB_PROFILE"
    "${zMBD_TOKEN_PARAMS[*]}"
    "$@"
)

zMBD_MAKE_CMD="${cmd_parts[*]}"

echo "Executing: $zMBD_MAKE_CMD"      | tee    "$zMBD_LOG_LAST" "$zMBD_LOG_SAME" "$zMBD_LOG_HIST"
eval            "$zMBD_MAKE_CMD" 2>&1 | tee -a "$zMBD_LOG_LAST" "$zMBD_LOG_SAME" "$zMBD_LOG_HIST"

zMBD_EXIT_STATUS="${PIPESTATUS[0]}"
zMBD_SHOW "Make completed with status: $zMBD_EXIT_STATUS"

exit "$zMBD_EXIT_STATUS"
