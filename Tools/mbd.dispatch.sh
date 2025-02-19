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

##########################################
# Makefile Bash Dispatch Shell Script
#
# Notes:
#  - Set verbose mode with MBD_VERBOSE=1 or MBD_VERBOSE=2 before command
#  - Leading 'z' variable prefix indicates it should not be referenced
#    by name outside of this file.
#
# Commentary:
#    In days where all builds were local, the dispatch script served
#  a critical role in scrubbing the environment variables down to the
#  bare minimum needed before handing control to make.  This reduced
#  the chances of 'works in my environment but not in yours.'  At the
#  time of this writing, this is embracing a new model where environments
#  are set by containers instead.  Podman under windows with cygwin has
#  some exotic environment variable entanglements so little effort is
#  done to scrub out the variables.

set -euo pipefail

zMBD_VERBOSE=${MBD_VERBOSE:-0}
zMBD_SHOW() { test "$zMBD_VERBOSE" != "1" || echo "dispatch: $1"; }
test               "$zMBD_VERBOSE" != "2" || set -x

zMBD_SHOW "Starting dispatch script"

cd "$(dirname "$0")/.."
zMBD_SHOW "Changed to repository root, cwd for all commands executed"

zMBD_SHOW "Source variables file and validate"
zMBD_VARIABLES=./mbv.variables.sh
source ${zMBD_VARIABLES}
: ${zMBD_VARIABLES:?}          && zMBD_SHOW "Variables file:      ${zMBD_VARIABLES}"
: ${MBV_STATION_FILE:?}        && zMBD_SHOW "Station file:        ${MBV_STATION_FILE}"
: ${MBV_LOG_LAST:?}            && zMBD_SHOW "Latest log:          ${MBV_LOG_LAST}"
: ${MBV_LOG_EXT:?}             && zMBD_SHOW "Log extension:       ${MBV_LOG_EXT}"
: ${MBV_CONSOLE_MAKEFILE:?}    && zMBD_SHOW "Console Makefile:    ${MBV_CONSOLE_MAKEFILE}"
: ${MBV_TABTARGET_DIR:?}       && zMBD_SHOW "Tabtarget Dir:       ${MBV_TABTARGET_DIR}"
: ${MBV_TABTARGET_DELIMITER:?} && zMBD_SHOW "Tabtarget Delimiter: ${MBV_TABTARGET_DELIMITER}"
: ${MBV_TEMP_ROOT_DIR:?}       && zMBD_SHOW "Temp root directory: ${MBV_TEMP_ROOT_DIR}"

zMBD_SHOW "Source select station file vars and validate"
source $MBV_STATION_FILE
: ${MBS_LOG_DIR:?}          && zMBD_SHOW "Log directory:  ${MBS_LOG_DIR}"
: ${MBS_MAX_JOBS:?}         && zMBD_SHOW "Max jobs:       ${MBS_MAX_JOBS}"

zMBD_NOW_STAMP=$(date +'%Y%m%d-%H%M%Sp%N')
zMBD_SHOW "Generated timestamp: $zMBD_NOW_STAMP"

zMBD_SHOW "Setting up temporary directory"
zMBD_TEMP_DIR="$MBV_TEMP_ROOT_DIR/temp-$zMBD_NOW_STAMP"
mkdir -p        "$zMBD_TEMP_DIR"
test  -d        "$zMBD_TEMP_DIR"                            || ( zMBD_SHOW "Failed mkdir temp: $zMBD_TEMP_DIR" && exit 1)
test -z "$(find "$zMBD_TEMP_DIR" -mindepth 1 -print -quit)" || ( zMBD_SHOW "temp dir nonempty: $zMBD_TEMP_DIR" && exit 1)

zMBD_JP_ARG=$1
zMBD_OM_ARG=$2
zMBD_TARGET=$3
shift 3

zMBD_SHOW "Validating job profile"
case "$zMBD_JP_ARG" in
  jp_single)    zMBD_JOB_PROFILE=1               ;;
  jp_bounded)   zMBD_JOB_PROFILE=$MBS_MAX_JOBS   ;;
  jp_unbounded) zMBD_JOB_PROFILE=""              ;;
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
IFS="$MBV_TABTARGET_DELIMITER" read -ra zMBD_TOKENS <<< "$zMBD_TARGET"
zMBD_SHOW "Split tokens: ${zMBD_TOKENS[*]}"

zMBD_TOKEN_PARAMS=()
for i in "${!zMBD_TOKENS[@]}"; do
    [[ -z "${zMBD_TOKENS[$i]}" ]] || zMBD_TOKEN_PARAMS+=("MBDM_PARAMETER_$i=${zMBD_TOKENS[$i]}")
done
zMBD_SHOW "Token parameters: ${zMBD_TOKEN_PARAMS[*]}"

zMBD_TAG=${zMBD_TOKENS[0]}-${zMBD_TOKENS[2]}
zMBD_SHOW "Presume second token descriptive so using logfile infix: ${zMBD_TAG}"

zMBD_LOG_LAST=$MBS_LOG_DIR/$MBV_LOG_LAST.$MBV_LOG_EXT
zMBD_LOG_SAME=$MBS_LOG_DIR/same-${zMBD_TAG}.$MBV_LOG_EXT
zMBD_LOG_HIST=$MBS_LOG_DIR/hist-${zMBD_TAG}-$zMBD_NOW_STAMP.$MBV_LOG_EXT

zMBD_SHOW "Log paths:"
zMBD_SHOW "  DIR:   $MBS_LOG_DIR"
zMBD_SHOW "  LAST:  $zMBD_LOG_LAST"
zMBD_SHOW "  SAME:  $zMBD_LOG_SAME"

echo "Historical log: $zMBD_LOG_HIST"

zMBD_SHOW "Assure log directory exists and logs prepared..."
mkdir -p "$MBS_LOG_DIR"
> "$zMBD_LOG_LAST"
> "$zMBD_LOG_SAME"
> "$zMBD_LOG_HIST"

zMBD_GIT_CONTEXT=$(git describe --always --dirty --tags --long 2>/dev/null || echo "git-unavailable")
echo "Git context: $zMBD_GIT_CONTEXT" >> "$zMBD_LOG_HIST"

cmd_parts=(
    "make -f $MBV_CONSOLE_MAKEFILE"
    "$zMBD_OUTPUT_MODE -j $zMBD_JOB_PROFILE"
    "$zMBD_TARGET"
    "MBD_NOW_STAMP=$zMBD_NOW_STAMP"
    "MBD_JOB_PROFILE=$zMBD_JOB_PROFILE"
    "MBD_DISPATCH_TEMP_DIR=$zMBD_TEMP_DIR"
    "${zMBD_TOKEN_PARAMS[*]}"
    "$@"
)

zMBD_MAKE_CMD="${cmd_parts[*]}"

zMBD_TIMESTAMP() {
    while read -r line; do 
        printf "[%s] %s\n" "$(date +"%Y-%m-%d %H:%M:%S")" "$line"
    done
}

zMBD_SHOW "eval: $zMBD_MAKE_CMD"

echo "command: $zMBD_MAKE_CMD" >> "$zMBD_LOG_LAST" >> "$zMBD_LOG_SAME" >> "$zMBD_LOG_HIST"

set +e
zMBD_STATUS_TMP="$zMBD_TEMP_DIR/status-$$"
{ 
    eval "$zMBD_MAKE_CMD" 2>&1
    echo $? > "$zMBD_STATUS_TMP"
    zMBD_SHOW "Make completed with local status: $(cat $zMBD_STATUS_TMP)"
} | tee -a "$zMBD_LOG_LAST" "$zMBD_LOG_SAME" >(zMBD_TIMESTAMP >> "$zMBD_LOG_HIST")
zMBD_EXIT_STATUS=$(cat "$zMBD_STATUS_TMP")
rm "$zMBD_STATUS_TMP"
set -e

zMBD_SHOW "Generate checksum after all logging is complete, regardless of eval status"
echo "Same log checksum: $(sha256sum            "$zMBD_LOG_SAME" 2>/dev/null || 
                           openssl dgst -sha256 "$zMBD_LOG_SAME" 2>/dev/null || 
                           echo "checksum-unavailable")" >> "$zMBD_LOG_HIST" || true

zMBD_SHOW "Make completed with status: $zMBD_EXIT_STATUS"

exit "$zMBD_EXIT_STATUS"

