#!/bin/bash
# This script is designed to be sourced, not executed directly

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
# Makefile Bash Utilities
#
# This script provides utility functions for the build system.
# It can be sourced by other scripts to provide common functionality.
#
# Usage: source mbd.utils.sh [verbose_level]
#
# Functions:
# - mbd_show: Display messages based on verbosity level
# - mbd_setup: Source and validate configuration files
# - mbd_process_args: Process command-line arguments
# - mbd_curate_same: Format logs for the 'same' log file
# - mbd_curate_hist: Format logs for the historical log file
# - mbd_generate_checksum: Generate and log checksums

# Import validation utilities
source "$(dirname "$0")/crgv.validate.sh"

# Initialize verbosity level
mbd_verbose=${1:-${MBD_VERBOSE:-0}}

# Utility function to display messages based on verbosity
mbd_show() {
    test "$mbd_verbose" != "1" || echo "MBDSHOW: $*"
}

# Enable trace mode if verbose level is 2
if [[ "$mbd_verbose" == "2" ]]; then
    set -x
fi

# Source configuration files and validate settings
mbd_setup() {
    mbd_show "Sourcing variables files and validating"

    # Source main variables file
    local variables_file=${1:-"./mbv.variables.sh"}
    source "$variables_file"

    # Validate essential variables
    crgv_string "$variables_file" MBV_STATION_FILE        1 256
    crgv_string "$variables_file" MBV_LOG_LAST            1 256
    crgv_string "$variables_file" MBV_LOG_EXT             1 32
    crgv_string "$variables_file" MBV_CONSOLE_MAKEFILE    1 256
    crgv_string "$variables_file" MBV_TABTARGET_DIR       1 256
    crgv_string "$variables_file" MBV_TABTARGET_DELIMITER 1 8
    crgv_string "$variables_file" MBV_TEMP_ROOT_DIR       1 256

    # Source station file
    mbd_show "Sourcing station file"
    source "$MBV_STATION_FILE"

    # Validate station variables
    crgv_string  "$MBV_STATION_FILE" MBS_LOG_DIR  1 256
    crgv_decimal "$MBV_STATION_FILE" MBS_MAX_JOBS 1 128

    # Generate timestamp
    MBD_NOW_STAMP=$(date +'%Y%m%d-%H%M%S')-$$-$((RANDOM % 1000))
    mbd_show "Generated timestamp: $MBD_NOW_STAMP"

    # Setup temporary directory
    MBD_TEMP_DIR="$MBV_TEMP_ROOT_DIR/temp-$MBD_NOW_STAMP"
    mkdir -p "$MBD_TEMP_DIR"

    # Validate temporary directory
    if [[ ! -d "$MBD_TEMP_DIR" ]]; then
        echo "ERROR: Failed to create temporary directory: $MBD_TEMP_DIR" >&2
        return 1
    fi

    if [[ -n "$(find "$MBD_TEMP_DIR" -mindepth 1 -print -quit 2>/dev/null)" ]]; then
        echo "ERROR: Temporary directory is not empty: $MBD_TEMP_DIR" >&2
        return 1
    fi

    # Get Git context
    MBD_GIT_CONTEXT=$(git describe --always --dirty --tags --long 2>/dev/null || echo "git-unavailable")

    return 0
}

# Process command-line arguments and tabtarget
mbd_process_args() {
    local jp_arg=$1
    local om_arg=$2
    local target=$3
    shift 3

    # Validate job profile
    mbd_show "Validating job profile" "args"
    case "$jp_arg" in
        jp_single)    MBD_JOB_PROFILE=1             ;;
        jp_bounded)   MBD_JOB_PROFILE=$MBS_MAX_JOBS ;;
        jp_unbounded) MBD_JOB_PROFILE=""            ;;
        *)
            echo "ERROR: Invalid job profile: $jp_arg" >&2
            return 1
            ;;
    esac

    # Validate output mode
    mbd_show "Validating output mode" "args"
    case "$om_arg" in
        om_line)   MBD_OUTPUT_MODE="-Oline"    ;;
        om_target) MBD_OUTPUT_MODE="-Orecurse" ;;
        *)
            echo "ERROR: Invalid output mode: $om_arg" >&2
            return 1
            ;;
    esac

    # Extract tokens from tabtarget
    mbd_show "Tokenizing tabtarget: $target" "args"
    IFS="$MBV_TABTARGET_DELIMITER" read -ra tokens <<< "$target"
    mbd_show "Split tokens: ${tokens[*]}" "args"

    # Process token parameters
    MBD_TOKEN_PARAMS=()
    for i in "${!tokens[@]}"; do
        [[ -z "${tokens[$i]}" ]] || MBD_TOKEN_PARAMS+=("MBD_PARAMETER_$i=${tokens[$i]}")
    done

    # Create tag for log files
    local tag=${tokens[0]}-${tokens[2]}

    # Setup log paths
    MBD_LOG_LAST=$MBS_LOG_DIR/$MBV_LOG_LAST.$MBV_LOG_EXT
    MBD_LOG_SAME=$MBS_LOG_DIR/same-${tag}.$MBV_LOG_EXT
    MBD_LOG_HIST=$MBS_LOG_DIR/hist-${tag}-$MBD_NOW_STAMP.$MBV_LOG_EXT

    # Prepare log directories
    mkdir -p "$MBS_LOG_DIR"

    # Maybe unnecessary
    > "$MBD_LOG_LAST"
    > "$MBD_LOG_SAME"
    > "$MBD_LOG_HIST"

    # Store target and extra arguments
    MBD_TARGET=$target
    MBD_EXTRA_ARGS=("$@")
    MBD_REMAINDER="$*"

    return 0
}

# Function to curate logs for the 'same' log file (normalized output)
mbd_curate_same() {
    # Convert to unix line endings, strip colors, normalize temp dir, remove VOLATILE lines
    sed -e 's/\r/\n/g' \
        -e '/^$/d' \
        -e 's/\x1b[\[][0-9;]*[a-zA-Z]//g' \
        -e 's/\x1b[(][A-Z]//g' \
        -e "s|$MBD_TEMP_DIR|MBD_EPHEMERAL_DIR|g" \
        -e '/VOLATILE/d'
}

# Function to curate logs for the historical log file (with timestamps)
mbd_curate_hist() {
    while read -r line; do
        printf "[%s] %s\n" "$(date +"%Y-%m-%d %H:%M:%S")" "$line"
    done
}

# Generate and log checksum for a file
mbd_generate_checksum() {
    local file=$1
    local output_file=$2

    # Try multiple checksum commands (platform-dependent)
    local checksum=$(sha256sum            "$file" 2>/dev/null ||
                     openssl dgst -sha256 "$file" 2>/dev/null ||
                     echo "checksum-unavailable")

    echo "Same log checksum: $checksum" >> "$output_file"
    return 0
}

# Generate the make command arguments
mbd_gen_make_cmd() {
    local makefile=${1:-$MBV_CONSOLE_MAKEFILE}

    # Build base command
    local cmd=("make" "-f" "$makefile" "$MBD_OUTPUT_MODE")

    # Add debug flags if verbose is set
    if [[ "$mbd_verbose" == "1" ]]; then
        cmd+=("--debug=v")
    elif [[ "$mbd_verbose" == "2" ]]; then
        cmd+=("--debug=v" "--trace")
    fi

    # Add job profile if not empty
    if [[       -n "$MBD_JOB_PROFILE" ]]; then
        cmd+=("-j" "$MBD_JOB_PROFILE")
    fi

    # Add target
    cmd+=("$MBD_TARGET")

    # Add essential variables
    cmd+=("MBD_NOW_STAMP=$MBD_NOW_STAMP"
          "MBD_JOB_PROFILE=$MBD_JOB_PROFILE"
          "MBD_TEMP_DIR=$MBD_TEMP_DIR")

    # Add token parameters
    for param in "${MBD_TOKEN_PARAMS[@]}"; do
        cmd+=("$param")
    done

    # Pass *all* of your extra patterns/args via a single variable,
    # so make won't interpret them as goals.
    cmd+=("MBD_CLI_ARGS='$MBD_REMAINDER'")

    # Return the command
    echo "${cmd[@]}"
}

# Initial message
mbd_show "MBD utilities loaded with verbose level $mbd_verbose" "init"

