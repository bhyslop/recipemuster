#!/bin/bash
#
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
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Bash Dispatch Utility - Direct bash dispatch without make

set -euo pipefail

# Initialize verbose level
BDU_VERBOSE=${BDU_VERBOSE:-0}

# Source validation utilities
source "$(dirname "$0")/crgv.validate.sh"

# Utility function for verbose output
bdu_show() {
    test "$BDU_VERBOSE" != "1" || echo "BDUSHOW: $*"
}

# Enable trace mode if verbose level is 2
if [[ "$BDU_VERBOSE" == "2" ]]; then
    set -x
fi

# Source configuration and setup environment
bdu_setup() {
    bdu_show "Starting BDU setup"
    
    # Source main variables file
    local variables_file="./bdu.variables.sh"
    source "$variables_file"
    
    # Validate essential variables
    crgv_string "$variables_file" BDU_STATION_FILE        1 256
    crgv_string "$variables_file" BDU_LOG_LAST            1 256
    crgv_string "$variables_file" BDU_LOG_EXT             1 32
    crgv_string "$variables_file" BDU_TABTARGET_DIR       1 256
    crgv_string "$variables_file" BDU_TABTARGET_DELIMITER 1 8
    crgv_string "$variables_file" BDU_TEMP_ROOT_DIR       1 256
    crgv_string "$variables_file" BDU_TOOLS_DIR           1 256
    
    # Source station file
    bdu_show "Sourcing station file: $BDU_STATION_FILE"
    source "$BDU_STATION_FILE"
    
    # Validate station variables
    crgv_string "$BDU_STATION_FILE" BDS_LOG_DIR 1 256
    
    # Generate timestamp
    BDU_NOW_STAMP=$(date +'%Y%m%d-%H%M%S')-$$-$((RANDOM % 1000))
    bdu_show "Generated timestamp: $BDU_NOW_STAMP"
    
    # Setup temporary directory
    BDU_TEMP_DIR="$BDU_TEMP_ROOT_DIR/temp-$BDU_NOW_STAMP"
    mkdir -p "$BDU_TEMP_DIR"
    
    # Validate temporary directory
    if [[ ! -d "$BDU_TEMP_DIR" ]]; then
        echo "ERROR: Failed to create temporary directory: $BDU_TEMP_DIR" >&2
        return 1
    fi
    
    if [[ -n "$(find "$BDU_TEMP_DIR" -mindepth 1 -print -quit 2>/dev/null)" ]]; then
        echo "ERROR: Temporary directory is not empty: $BDU_TEMP_DIR" >&2
        return 1
    fi
    
    # Get Git context
    BDU_GIT_CONTEXT=$(git describe --always --dirty --tags --long 2>/dev/null || echo "git-unavailable")
    bdu_show "Git context: $BDU_GIT_CONTEXT"
    
    # Export for child processes
    export BDU_TEMP_DIR
    export BDU_NOW_STAMP
    
    return 0
}

# Process command-line arguments
bdu_process_args() {
    local target=$1
    shift
    
    bdu_show "Processing target: $target"
    
    # Extract tokens from tabtarget
    IFS="$BDU_TABTARGET_DELIMITER" read -ra tokens <<< "$target"
    bdu_show "Split tokens: ${tokens[*]}"
    
    # Store primary command token
    BDU_COMMAND="${tokens[0]}"
    
    # Create tag for log files
    local tag="${tokens[0]}-${tokens[2]:-unknown}"
    
    # Setup log paths
    BDU_LOG_LAST="$BDS_LOG_DIR/$BDU_LOG_LAST.$BDU_LOG_EXT"
    BDU_LOG_SAME="$BDS_LOG_DIR/same-${tag}.$BDU_LOG_EXT"
    BDU_LOG_HIST="$BDS_LOG_DIR/hist-${tag}-$BDU_NOW_STAMP.$BDU_LOG_EXT"
    
    # Prepare log directories
    mkdir -p "$BDS_LOG_DIR"
    
    # Initialize log files
    > "$BDU_LOG_LAST"
    > "$BDU_LOG_SAME"
    > "$BDU_LOG_HIST"
    
    # Store target and extra arguments
    BDU_TARGET="$target"
    BDU_CLI_ARGS="$*"
    
    return 0
}

# Function to curate logs for the 'same' log file (normalized output)
bdu_curate_same() {
    # Convert to unix line endings, strip colors, normalize temp dir, remove VOLATILE lines
    sed -e 's/\r/\n/g' \
        -e '/^$/d' \
        -e 's/\x1b[\[][0-9;]*[a-zA-Z]//g' \
        -e 's/\x1b[(][A-Z]//g' \
        -e "s|$BDU_TEMP_DIR|BDU_EPHEMERAL_DIR|g" \
        -e '/VOLATILE/d'
}

# Function to curate logs for the historical log file (with timestamps)
bdu_curate_hist() {
    while read -r line; do
        printf "[%s] %s\n" "$(date +"%Y-%m-%d %H:%M:%S")" "$line"
    done
}

# Generate and log checksum for a file
bdu_generate_checksum() {
    local file=$1
    local output_file=$2
    
    # Try multiple checksum commands (platform-dependent)
    local checksum=$(sha256sum            "$file" 2>/dev/null ||
                     openssl dgst -sha256 "$file" 2>/dev/null ||
                     echo "checksum-unavailable")
    
    echo "Same log checksum: $checksum" >> "$output_file"
    return 0
}

# Main execution
main() {
    bdu_show "Starting BDU dispatch"
    
    # Setup environment
    bdu_setup || (echo "ERROR: Environment setup failed" >&2 && exit 1)
    bdu_show "Environment setup complete"
    
    # Process arguments
    bdu_process_args "$@" || (echo "ERROR: Argument processing failed" >&2 && exit 1)
    bdu_show "Arguments processed"
    
    # Build RBK command
    local rbk_cmd="$BDU_TOOLS_DIR/rbk_Coordinator.sh"
    bdu_show "RBK command: $rbk_cmd $BDU_COMMAND $BDU_CLI_ARGS"
    
    # Log command to all log files
    echo "log files: $BDU_LOG_LAST $BDU_LOG_SAME $BDU_LOG_HIST"
    echo "command: $rbk_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_LAST"
    echo "command: $rbk_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_SAME"
    echo "command: $rbk_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_HIST"
    echo "Git context: $BDU_GIT_CONTEXT" >> "$BDU_LOG_HIST"
    
    bdu_show "Executing RBK coordinator"
    
    # Execute RBK with logging
    set +e
    zBDU_STATUS_FILE="$BDU_TEMP_DIR/status-$$"
    {
        "$rbk_cmd" "$BDU_COMMAND" $BDU_CLI_ARGS 2>&1
        echo $? > "$zBDU_STATUS_FILE"
        bdu_show "RBK status: $(cat $zBDU_STATUS_FILE)"
    } | tee -a "$BDU_LOG_LAST" >(bdu_curate_same >> "$BDU_LOG_SAME") \
                               >(bdu_curate_hist >> "$BDU_LOG_HIST")
    zBDU_EXIT_STATUS=$(cat "$zBDU_STATUS_FILE")
    rm                     "$zBDU_STATUS_FILE"
    set -e
    
    # Generate checksum for the log file
    bdu_generate_checksum "$BDU_LOG_SAME" "$BDU_LOG_HIST"
    bdu_show "Checksum generated"
    
    bdu_show "BDU completed with status: $zBDU_EXIT_STATUS"
    
    exit "$zBDU_EXIT_STATUS"
}

# Run main with all arguments
main "$@"

